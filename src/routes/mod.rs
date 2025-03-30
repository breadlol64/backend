pub mod index;
pub mod user;
pub mod auth;
pub mod click;

use actix_web::web;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("")
            .configure(index::configure_routes)
            .configure(user::configure_routes)
            .configure(auth::configure_routes)
            .configure(click::configure_routes)
    );
}