use actix_web::{web, HttpResponse, Responder};

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/")
            .route("", web::get().to(index))
    );
}

async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello")
}