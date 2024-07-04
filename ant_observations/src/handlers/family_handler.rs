use actix_web::{web, HttpResponse};
use sqlx::PgPool;
use log::{info, error};
use crate::models::family::{Family, NewFamily};

pub async fn get_families(pool: web::Data<PgPool>) -> HttpResponse {
    match sqlx::query_as::<_, Family>("
    SELECT
        F.ID_FAMILY,
        F.SCIENTIFIC_NAME AS FAMILY
    FROM
        FAMILY F;
    ")
        .fetch_all(pool.get_ref())
        .await {
            Ok(rows) => {
                info!("Successfully fetched {} families", rows.len());
                HttpResponse::Ok().json(rows)
            },
            Err(e) => {
                error!("Failed to fetch families: {:?}", e);
                HttpResponse::InternalServerError().finish()
            },  
        }
}

// Handler para crear una nueva familia
pub async fn create_family(new_family: web::Json<NewFamily>, pool: web::Data<PgPool>) -> HttpResponse {
    let new_family: NewFamily = new_family.into_inner(); // Convertir de web::Json a Family
    
    // Ejecutar la inserción en la base de datos
    let result = sqlx::query!(
        "INSERT INTO family (scientific_name) VALUES ($1) RETURNING id_family",
        new_family.scientific_name
    )
    .fetch_one(pool.get_ref())
    .await;
    
    match result {
        Ok(row) => {
            let family_response = Family {
                id_family: row.id_family,
                family: new_family.scientific_name,    
            };
            info!("Successfully created family: {:?}", family_response);
            HttpResponse::Created().json(family_response)
        }
        Err(e) => {
            error!("Failed to create family: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
