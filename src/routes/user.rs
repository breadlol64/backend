use actix_web::{web, HttpResponse, Responder};
use sqlx::PgPool;

use crate::models::User;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/user")
            .route("/{username}", web::get().to(get_user))
    );
}

async fn get_user(path: web::Path<String>, pool: web::Data<PgPool>) -> impl Responder {
    match User::get_by_username(&pool, &path.into_inner()).await {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string())
    }
}