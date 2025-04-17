mod routes;
mod handlers;
pub mod utils;
use dotenvy::dotenv;
use std::env;
use actix_cors::Cors;
use actix_web::{get, App, HttpServer, Responder};
use routes::auth;
use env_logger;

#[get("/healthz")]
async fn health_check() -> impl Responder {
    "Rorscha server is running!"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    println!("🚀 Rorscha server starting...");

    HttpServer::new(|| {
        App::new()
            .wrap(
                Cors::default()
                .allowed_origin("http://localhost:3000")
                .allowed_methods(vec!["GET", "PUT", "DELETE", "POST", "OPTIONS"])
                .allowed_headers(vec!["Content-Type", "Authorization"])
                .supports_credentials()
            )
            .configure(auth::init_routes)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}