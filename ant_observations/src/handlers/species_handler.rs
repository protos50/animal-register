use actix_web::{web, HttpResponse};
use sqlx::PgPool;
use log::{info, error};
use crate::models::species::{Species, NewSpecies};

pub async fn get_species(pool: web::Data<PgPool>) -> HttpResponse {
    match sqlx::query_as::<_, Species>("SELECT id_species, id_genus, scientific_name FROM species")
        .fetch_all(pool.get_ref())
        .await {
            Ok(rows) => {
                info!("Successfully fetched {} species", rows.len());
                HttpResponse::Ok().json(rows)
            },
            Err(e) => {
                error!("Failed to fetch species: {:?}", e);
                HttpResponse::InternalServerError().finish()
            },  
        }
}

// Handler para crear una nuevo genero
pub async fn create_species(new_species: web::Json<NewSpecies>, pool: web::Data<PgPool>) -> HttpResponse {
    let new_species: NewSpecies = new_species.into_inner(); // Convertir de web::Json a Family
    
    // Ejecutar la inserción en la base de datos
    let result = sqlx::query!(
        "INSERT INTO species (id_genus, scientific_name) VALUES ($1, $2) RETURNING id_species",
        new_species.id_genus, new_species.scientific_name
    )
    .fetch_one(pool.get_ref())
    .await;
    
    match result {
        Ok(row) => {
            let species_response = Species {
                id_species: row.id_species,
                id_genus: new_species.id_genus,
                scientific_name: new_species.scientific_name,    
            };
            info!("Successfully created species: {:?}", species_response);
            HttpResponse::Created().json(species_response)
        }
        Err(e) => {
            error!("Failed to create species: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}