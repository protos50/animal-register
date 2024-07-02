use actix_web::{web, HttpResponse};
use sqlx::PgPool;
use log::{info, error};
use crate::models::subfamily::{Subfamily, NewSubfamily};

pub async fn get_subfamilies(pool: web::Data<PgPool>) -> HttpResponse {
    match sqlx::query_as::<_, Subfamily>("SELECT id_subfamily, id_family, scientific_name FROM subfamily")
        .fetch_all(pool.get_ref())
        .await {
            Ok(rows) => {
                info!("Successfully fetched {} subfamilies", rows.len());
                HttpResponse::Ok().json(rows)
            },
            Err(e) => {
                error!("Failed to fetch subfamilies: {:?}", e);
                HttpResponse::InternalServerError().finish()
            },  
        }
}

// Handler para crear una nuevo genero
pub async fn create_subfamily(new_subfamily: web::Json<NewSubfamily>, pool: web::Data<PgPool>) -> HttpResponse {
    let new_subfamily: NewSubfamily = new_subfamily.into_inner(); // Convertir de web::Json a Family
    
    // Ejecutar la inserción en la base de datos
    let result = sqlx::query!(
        "INSERT INTO subfamily (id_family, scientific_name) VALUES ($1, $2) RETURNING id_subfamily",
        new_subfamily.id_family, new_subfamily.scientific_name
    )
    .fetch_one(pool.get_ref())
    .await;
    
    match result {
        Ok(row) => {
            let subfamily_response = Subfamily {
                id_subfamily: row.id_subfamily,
                id_family: new_subfamily.id_family,
                scientific_name: new_subfamily.scientific_name,    
            };
            info!("Successfully created subfamily: {:?}", subfamily_response);
            HttpResponse::Created().json(subfamily_response)
        }
        Err(e) => {
            error!("Failed to create subfamily: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}