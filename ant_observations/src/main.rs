mod config;
mod routes;
mod handlers;

use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    let port = env::var("SERVER_PORT").expect("SERVER_PORT not set").parse().expect("Invalid port number");

    HttpServer::new(|| {
        App::new()
            .configure(routes::config)
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await
}