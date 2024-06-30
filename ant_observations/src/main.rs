/* 
mod config;
mod routes;
mod handlers;
*/

use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use sqlx::{database, pool, postgres::PgPoolOptions};
use std::env;
use ant_observations::routes;
use ant_observations::config;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    let port = env::var("SERVER_PORT").expect("SERVER_PORT not set").parse().expect("Invalid port number");
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to create pool.");

    println!("Starting server at http://0.0.0.0:{}", port);
    
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .configure(routes::config)
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}