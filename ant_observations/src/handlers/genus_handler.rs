use actix_web::{web, HttpResponse};
use sqlx::PgPool;
use log::{info, error};
use crate::models::genus::{Genus, NewGenus};

pub async fn get_genera(pool: web::Data<PgPool>) -> HttpResponse {
    match sqlx::query_as::<_, Genus>("SELECT id_genus, id_tribe, scientific_name FROM genus")
        .fetch_all(pool.get_ref())
        .await {
            Ok(rows) => {
                info!("Successfully fetched {} genera", rows.len());
                HttpResponse::Ok().json(rows)
            },
            Err(e) => {
                error!("Failed to fetch genera: {:?}", e);
                HttpResponse::InternalServerError().finish()
            },  
        }
}

// Handler para crear una nuevo genero
pub async fn create_genus(new_genus: web::Json<NewGenus>, pool: web::Data<PgPool>) -> HttpResponse {
    let new_genus: NewGenus = new_genus.into_inner(); // Convertir de web::Json a Family
    
    // Ejecutar la inserción en la base de datos
    let result = sqlx::query!(
        "INSERT INTO genus (id_tribe, scientific_name) VALUES ($1, $2) RETURNING id_genus",
        new_genus.id_tribe, new_genus.scientific_name
    )
    .fetch_one(pool.get_ref())
    .await;
    
    match result {
        Ok(row) => {
            let genus_response = Genus {
                id_genus: row.id_genus,
                id_tribe: new_genus.id_tribe,
                scientific_name: new_genus.scientific_name,    
            };
            info!("Successfully created genus: {:?}", genus_response);
            HttpResponse::Created().json(genus_response)
        }
        Err(e) => {
            error!("Failed to create genus: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}