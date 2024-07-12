use actix_web::{web, HttpResponse};
use sqlx::PgPool;
use log::{info, error};
use crate::models::province::{NewProvince, Province, SimpleProvince};

pub async fn get_provinces(pool: web::Data<PgPool>) -> HttpResponse {
    match sqlx::query_as::<_, Province>("
    SELECT
        P.ID_PROVINCE,
        C.COUNTRY_NAME AS COUNTRY,
        P.PROVINCE_NAME AS PROVINCE 
    FROM
        PROVINCE P
        JOIN COUNTRY C ON C.ID_COUNTRY = P.ID_COUNTRY;
    ")
        .fetch_all(pool.get_ref())
        .await {
            Ok(rows) => {
                info!("Successfully fetched {} province", rows.len());
                HttpResponse::Ok().json(rows)
            },
            Err(e) => {
                error!("Failed to fetch province: {:?}", e);
                HttpResponse::InternalServerError().finish()
            },  
        }
}

pub async fn create_provinces(new_province: web::Json<NewProvince>, pool: web::Data<PgPool>) -> HttpResponse {
    let new_province: NewProvince = new_province.into_inner(); // Convertir de web::Json a Family
    
    // Ejecutar la inserción en la base de datos
    let result = sqlx::query!(
        "INSERT INTO PROVINCE (ID_COUNTRY, PROVINCE_NAME) VALUES ($1, $2) RETURNING ID_PROVINCE",
        new_province.id_country, new_province.province_name
    )
    .fetch_one(pool.get_ref())
    .await;
    
    match result {
        Ok(row) => {
            info!("Successfully created locality: {:?}", row);
            HttpResponse::Created().json(row.id_province)
        }
        Err(e) => {
            error!("Failed to create locality: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

pub async fn get_simple_provinces(pool: web::Data<PgPool>) -> HttpResponse {
    match sqlx::query_as::<_, SimpleProvince>("
    SELECT
        ID_PROVINCE,
        PROVINCE_NAME
    FROM
        PROVINCE;
    ")
        .fetch_all(pool.get_ref())
        .await {
            Ok(rows) => {
                info!("Successfully fetched {} province", rows.len());
                HttpResponse::Ok().json(rows)
            },
            Err(e) => {
                error!("Failed to fetch province: {:?}", e);
                HttpResponse::InternalServerError().finish()
            },  
        }
}