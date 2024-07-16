use actix_web::{web, HttpResponse};
use sqlx::PgPool;
use log::{info, error};
use crate::models::geographic::country::{Country, NewCountry};

pub async fn get_countries(pool: web::Data<PgPool>) -> HttpResponse {
    match sqlx::query_as::<_, Country>("
    SELECT
        C.ID_COUNTRY,
        C.COUNTRY_NAME AS COUNTRY 
    FROM
        COUNTRY C
    ")
        .fetch_all(pool.get_ref())
        .await {
            Ok(rows) => {
                info!("Successfully fetched {} country", rows.len());
                HttpResponse::Ok().json(rows)
            },
            Err(e) => {
                error!("Failed to fetch country: {:?}", e);
                HttpResponse::InternalServerError().finish()
            },  
        }
}

pub async fn create_countries(new_country: web::Json<NewCountry>, pool: web::Data<PgPool>) -> HttpResponse {
    let new_country: NewCountry = new_country.into_inner(); // Convertir de web::Json a Family
    
    // Ejecutar la inserción en la base de datos
    let result = sqlx::query!(
        "INSERT INTO COUNTRY (COUNTRY_NAME) VALUES ($1) RETURNING ID_COUNTRY",
        new_country.country_name
    )
    .fetch_one(pool.get_ref())
    .await;
    
    match result {
        Ok(row) => {
            info!("Successfully created country: {:?}", row);
            HttpResponse::Created().json(row.id_country)
        }
        Err(e) => {
            error!("Failed to create country: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}