use actix_web::{web, HttpResponse};
use sqlx::PgPool;
use log::{info, error};
use crate::models::geographic::locality::{Locality, SimpleLocality, NewLocality};

pub async fn get_localities(pool: web::Data<PgPool>) -> HttpResponse {
    match sqlx::query_as::<_, Locality>("
    SELECT
        L.ID_LOCALITY,
        D.DEPARTMENT_NAME AS DEPARTMENT,
        L.LOCALITY_NAME AS LOCALITY 
    FROM
        LOCALITY L
        JOIN DEPARTMENT D ON L.ID_DEPARTMENT = D.ID_DEPARTMENT;
    ")
        .fetch_all(pool.get_ref())
        .await {
            Ok(rows) => {
                info!("Successfully fetched {} locality", rows.len());
                HttpResponse::Ok().json(rows)
            },
            Err(e) => {
                error!("Failed to fetch locality: {:?}", e);
                HttpResponse::InternalServerError().finish()
            },  
        }
}

pub async fn create_localities(new_locality: web::Json<NewLocality>, pool: web::Data<PgPool>) -> HttpResponse {
    let new_locality: NewLocality = new_locality.into_inner(); // Convertir de web::Json a Family
    
    // Ejecutar la inserción en la base de datos
    let result = sqlx::query!(
        "INSERT INTO LOCALITY (ID_DEPARTMENT, LOCALITY_NAME) VALUES ($1, $2) RETURNING ID_LOCALITY",
        new_locality.id_department, new_locality.locality_name
    )
    .fetch_one(pool.get_ref())
    .await;
    
    match result {
        Ok(row) => {
            info!("Successfully created locality: {:?}", row);
            HttpResponse::Created().json(row.id_locality)
        }
        Err(e) => {
            error!("Failed to create locality: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

pub async fn get_simple_localities(pool: web::Data<PgPool>) -> HttpResponse {
    match sqlx::query_as::<_, SimpleLocality>("
    SELECT
        ID_LOCALITY,
        LOCALITY_NAME
    FROM
        LOCALITY;
    ")
        .fetch_all(pool.get_ref())
        .await {
            Ok(rows) => {
                info!("Successfully fetched {} locality", rows.len());
                HttpResponse::Ok().json(rows)
            },
            Err(e) => {
                error!("Failed to fetch locality: {:?}", e);
                HttpResponse::InternalServerError().finish()
            },  
        }
}