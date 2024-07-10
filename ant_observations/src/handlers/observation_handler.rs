use actix_web::{web, HttpResponse};
use sqlx::PgPool;
use log::{info, error};
use crate::models::observation::{NewObservation, Observations};

pub async fn get_observations(pool: web::Data<PgPool>) -> HttpResponse {
    match sqlx::query_as::<_, Observations>("
        SELECT * FROM get_observations()
    ")
    .fetch_all(pool.get_ref())
    .await {
        Ok(rows) => {
            info!("Successfully fetched {} observations", rows.len());
            HttpResponse::Ok().json(rows)
        },
        Err(e) => {
            error!("Failed to fetch observations: {:?}", e);
            HttpResponse::InternalServerError().finish()
        },
    }
}

pub async fn create_observation(new_observation: web::Json<NewObservation>, pool: web::Data<PgPool>) -> HttpResponse {
    let new_observation: NewObservation = new_observation.into_inner(); // Convertir de web::Json a Family
    
    // Ejecutar la inserción en la base de datos
    let result = sqlx::query!(
        "INSERT INTO OBSERVATION (ID_SPECIES, ID_LOCALITY) VALUES ($1, $2) RETURNING ID_OBSERVATION",
        new_observation.id_species, new_observation.id_locality
    )
    .fetch_one(pool.get_ref())
    .await;
    
    match result {
        Ok(row) => {
            info!("Successfully created species: {:?}", row); 
            HttpResponse::Created().json(row.id_observation)
        }
        Err(e) => {
            error!("Failed to create observation: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}