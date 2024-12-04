use actix_web::{web, App, HttpServer};
use actix_files::Files;
use actix_cors::Cors;
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::env;
use env_logger::Env;
use log::{info, warn};
use ant_observations::routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init_from_env(Env::default().default_filter_or("debug"));

    let port: u16 = env::var("SERVER_PORT").expect("SERVER_PORT not set").parse().expect("Invalid port number");
    let url: String = env::var("SERVER_URL").expect("SERVER_URL not set").parse().expect("Invalid url number");
    let database_url: String = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let environment: String = env::var("ENVIRONMENT").unwrap_or_else(|_| String::from("development"));
    
    // Configuración de CORS basada en el entorno
    let allowed_origins: Vec<String> = if environment == "production" {
        // En producción, ALLOWED_ORIGINS es obligatorio
        match env::var("ALLOWED_ORIGINS") {
            Ok(origins) => origins
                .split(',')
                .map(|s| s.trim().to_string())
                .collect(),
            Err(_) => {
                panic!("ALLOWED_ORIGINS must be set in production environment");
            }
        }
    } else {
        // En desarrollo, usar valores por defecto
        warn!("Using default CORS settings for development");
        vec![
            String::from("http://localhost:3000"),
            String::from("http://localhost")
        ]
    };

    let pool: sqlx::Pool<sqlx::Postgres> = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to create pool.");

    info!("Starting server in {} mode at http://{}:{}", environment, url, port);
    info!("Allowing CORS for origins: {:?}", allowed_origins);
    
    HttpServer::new(move || {
        // Configuración de CORS que permite múltiples orígenes
        let mut cors = Cors::default()
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
            .allowed_headers(vec!["Authorization", "Content-Type"])
            .expose_headers(vec!["content-disposition"])
            .max_age(3600)
            .supports_credentials(); // Importante para cookies/auth en producción

        // Agregar todos los orígenes permitidos desde la variable de entorno
        for origin in &allowed_origins {
            cors = cors.allowed_origin(origin);
        }

        App::new()
            .wrap(cors)
            .app_data(web::Data::new(pool.clone()))
            .configure(routes::config)
    })
    .bind((url, port))?
    .run()
    .await
}