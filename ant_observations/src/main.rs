use actix_web::{web, App, HttpServer, HttpResponse};
use actix_files::Files;
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::env;
use env_logger::Env;
use log::info;
use ant_observations::routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let port: u16 = env::var("SERVER_PORT").expect("SERVER_PORT not set").parse().expect("Invalid port number");
    let url: String = env::var("SERVER_URL").expect("SERVER_URL not set").parse().expect("Invalid url number");
    let database_url: String = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    

    let pool: sqlx::Pool<sqlx::Postgres> = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to create pool.");

    info!("Starting server at http://0.0.0.0:{}", port);
    
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .configure(routes::config)
            // Servir archivos estáticos desde el directorio build de React
            .service(Files::new("/", "../front_application/build").index_file("index.html"))
    })
    .bind((url, port))?
    .run()
    .await
}