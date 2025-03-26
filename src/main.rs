mod routes;
mod models;
mod jwt;

use actix_web::{web, App, HttpServer};
use sqlx::PgPool;
use dotenv::dotenv;
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()>{
    dotenv().ok();

    let addr = env::var("ADDR").expect("ADDR must be set");
    let port: u16 = match env::var("PORT") {
        Ok(val) => val.parse().expect("Invalid PORT value"),
        Err(_) => panic!("PORT must be set"),
    };
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    println!("connecting database");
    let pool = PgPool::connect(&db_url).await.expect("error connecting database");

    println!("starting on {}:{}", addr, port);
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .configure(routes::configure_routes)
    })
        .bind((addr, port))?
        .run()
        .await
}
