use actix_web::{web, HttpResponse};
use sqlx::PgPool;
use serde::{Deserialize, Serialize};
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE, AUTHORIZATION};
use std::env;
use serde_urlencoded;

use crate::models::User;
use crate::jwt;

#[derive(Deserialize)]
struct Code {
    code: String,
}

#[derive(Serialize)]
struct TokenReqBody {
    grant_type: String,
    code: String,
    redirect_uri: String,
}

#[derive(Deserialize)]
struct TokenResBody {
    access_token: String
}

#[derive(Deserialize)]
struct UserResBody {
    id: String,
    username: String,
    verified: bool,
    email: String,
    avatar: String
}

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .route("/discord", web::get().to(auth_discord))
    );
}

async fn auth_discord(
    query: web::Query<Code>,
    pool: web::Data<PgPool>
) -> Result<HttpResponse, actix_web::Error> {
    let client_id = env::var("DS_CLIENT_ID").expect("DS_CLIENT_ID must be set");
    let client_secret = env::var("DS_CLIENT_SECRET").expect("DS_CLIENT_SECRET must be set");

    let code = &query.code;
    let client = reqwest::Client::new();

    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/x-www-form-urlencoded"));
    // get token
    let body = TokenReqBody {
        grant_type: "authorization_code".to_string(),
        code: code.to_string(),
        redirect_uri: "http://localhost:3000/callback".to_string(),
    };

    let encoded_body = serde_urlencoded::to_string(&body)
        .map_err(actix_web::error::ErrorInternalServerError)?;
    println!("getting token for code {}", code);
    let token_res = client.post("https://discord.com/api/v10/oauth2/token")
        .headers(headers)
        .body(encoded_body)
        .basic_auth(client_id, Some(client_secret))
        .send()
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    // get user data
    let token: TokenResBody = token_res.json()
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;
    println!("getting user data for token {}", token.access_token);
    let mut user_headers = HeaderMap::new();
    user_headers.insert(
        AUTHORIZATION,
        HeaderValue::from_str(&format!("Bearer {}", token.access_token))
            .map_err(actix_web::error::ErrorInternalServerError)?,
    );

    let user_res = client.get("https://discord.com/api/v10/users/@me")
        .headers(user_headers)
        .send()
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let user_data: UserResBody = user_res.json().await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    // get or create user
    let user = match User::get_by_social_id(&pool, &user_data.id).await {
        Ok(user) => {
            println!("Found user {:?}", user);
            user
        },
        Err(sqlx::Error::RowNotFound) => {
            println!("Not found user {} {}", &user_data.username, &user_data.id);

            match User::create(
                &pool,
                &user_data.username,
                &user_data.email,
                &format!("https://cdn.discordapp.com/avatars/{}/{}.png", &user_data.id, &user_data.avatar),
                &user_data.id
            ).await {
                Ok(user) => {
                    println!("Created user {:?}", user);
                    user
                },
                Err(e) => return Err(actix_web::error::ErrorInternalServerError(e))
            }
        },
        Err(e) => return Err(actix_web::error::ErrorInternalServerError(e))
    };

    let token = jwt::gen_jwt(&user.id.to_string());

    Ok(HttpResponse::Ok().body(format!(r#"{{"token": "{}"}}"#, token)))
}
