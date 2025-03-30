use actix_web::{web, HttpResponse, HttpRequest, Responder, http::header};
use sqlx::PgPool;

use crate::models::User;
use crate::jwt;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/user")
            .route("/name/{username}", web::get().to(get_user_by_username))
            .route("/id/{id}", web::get().to(get_user_by_id))
            .route("/me", web::get().to(get_user_by_token))
    );
}

async fn get_user_by_username(path: web::Path<String>, pool: web::Data<PgPool>) -> impl Responder {
    match User::get_by_username(&pool, &path.into_inner()).await {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string())
    }
}

async fn get_user_by_id(path: web::Path<i32>, pool: web::Data<PgPool>) -> impl Responder {
    match User::get_by_id(&pool, path.into_inner()).await {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string())
    }
}

async fn get_user_by_token(req: HttpRequest, pool: web::Data<PgPool>) -> impl Responder {
    // TODO: auth middleware
    let id = match req.headers().get(header::AUTHORIZATION).expect("failed to do something idk").to_str() {
        Ok(token) => jwt::decode_jwt(token),
        Err(_) => return HttpResponse::BadRequest().body("Invalid token")
    };
    println!("id: {}", id);

    match User::get_by_id(&pool, id.parse().expect("Failed to convert string id to i32")).await {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string())
    }
}