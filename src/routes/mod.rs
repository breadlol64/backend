pub mod index;
pub mod user;

use actix_web::web;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("")
            .configure(index::configure_routes)
            .configure(user::configure_routes)
    );
}