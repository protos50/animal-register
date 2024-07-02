use actix_web::{web, HttpResponse};
use sqlx::PgPool;
use log::{info, error};
use crate::models::tribe::{Tribe, NewTribe};

pub async fn get_tribes(pool: web::Data<PgPool>) -> HttpResponse {
    match sqlx::query_as::<_, Tribe>("SELECT id_tribe, id_subfamily, scientific_name FROM tribe")
        .fetch_all(pool.get_ref())
        .await {
            Ok(rows) => {
                info!("Successfully fetched {} tribes", rows.len());
                HttpResponse::Ok().json(rows)
            },
            Err(e) => {
                error!("Failed to fetch tribes: {:?}", e);
                HttpResponse::InternalServerError().finish()
            },  
        }
}

// Handler para crear una nuevo genero
pub async fn create_tribe(new_tribe: web::Json<NewTribe>, pool: web::Data<PgPool>) -> HttpResponse {
    let new_tribe: NewTribe = new_tribe.into_inner(); // Convertir de web::Json a Family
    
    // Ejecutar la inserción en la base de datos
    let result = sqlx::query!(
        "INSERT INTO tribe (id_subfamily, scientific_name) VALUES ($1, $2) RETURNING id_tribe",
        new_tribe.id_subfamily, new_tribe.scientific_name
    )
    .fetch_one(pool.get_ref())
    .await;
    
    match result {
        Ok(row) => {
            let tribe_response = Tribe {
                id_tribe: row.id_tribe,
                id_subfamily: new_tribe.id_subfamily,
                scientific_name: new_tribe.scientific_name,    
            };
            info!("Successfully created tribe: {:?}", tribe_response);
            HttpResponse::Created().json(tribe_response)
        }
        Err(e) => {
            error!("Failed to create tribe: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}