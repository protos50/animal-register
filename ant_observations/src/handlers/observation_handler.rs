use actix_web::{web, HttpResponse};
use sqlx::PgPool;
use log::{info, error};
use crate::models::observation::{NewObservation, Observations};

pub async fn get_observations(pool: web::Data<PgPool>) -> HttpResponse {
    match sqlx::query_as::<_, Observations>("
    SELECT
        O.ID_OBSERVATION,
        E.SCIENTIFIC_NAME AS SPECIES,
        G.SCIENTIFIC_NAME AS GENUS,
        T.SCIENTIFIC_NAME AS TRIBE,
        S.SCIENTIFIC_NAME AS SUBFAMILY,
        F.SCIENTIFIC_NAME AS FAMILY
    FROM
        OBSERVATION O
        JOIN SPECIES E ON O.ID_SPECIES = E.ID_SPECIES
        JOIN GENUS G ON E.ID_GENUS = G.ID_GENUS
        JOIN TRIBE T ON G.ID_TRIBE = T.ID_TRIBE
        JOIN SUBFAMILY S ON T.ID_SUBFAMILY = S.ID_SUBFAMILY
        JOIN FAMILY F ON S.ID_FAMILY = F.ID_FAMILY;
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
        "INSERT INTO observation (id_species) VALUES ($1) RETURNING id_observation",
        new_observation.id_species
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