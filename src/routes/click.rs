use actix_web::{web, HttpResponse, HttpRequest, Responder};
use actix_web::http::header;
use sqlx::PgPool;
use crate::jwt;
use crate::models::User;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("")
            .route("/click", web::patch().to(click))
    );
}

async fn click(req: HttpRequest, pool: web::Data<PgPool>) -> impl Responder {
    // TODO: auth middleware
    let id = match req.headers().get(header::AUTHORIZATION).expect("failed to do something in click route idk").to_str() {
        Ok(token) => jwt::decode_jwt(token).parse().expect("failed to convert token to i32"),
        Err(_) => return HttpResponse::BadRequest().body("Invalid token")
    };

    let user = match User::get_by_id(&pool, id).await {
        Ok(user) => user,
        Err(e) => return HttpResponse::InternalServerError().body(e.to_string())
    };

    let updated_user = user.add_balance(&pool, 1).await.expect("Failed to add balance");

    HttpResponse::Ok().body(format!(r#"{{"balance": {}}}"#, updated_user.balance))
}
