use actix_web::{web, HttpResponse};
use sqlx::PgPool;
use log::{info, error};
use crate::models::person::role::{Role, NewRole};


pub async fn get_roles(pool: web::Data<PgPool>) -> HttpResponse {
    match sqlx::query_as::<_, Role>("
    SELECT
        R.ID_ROLE,
        R.ROLE_NAME
    FROM
        ROLE R
    ")
        .fetch_all(pool.get_ref())
        .await {
            Ok(rows) => {
                info!("Successfully fetched {} roles", rows.len());
                HttpResponse::Ok().json(rows)
            },
            Err(e) => {
                error!("Failed to fetch roles: {:?}", e);
                HttpResponse::InternalServerError().finish()
            },  
        }
}


pub async fn create_roles(new_roles: web::Json<NewRole>, pool: web::Data<PgPool>) -> HttpResponse {
    let new_roles: NewRole = new_roles.into_inner(); // Convertir de web::Json a Family
    
    // Ejecutar la inserción en la base de datos
    let result = sqlx::query!(
        "INSERT INTO ROLE (ROLE_NAME) VALUES ($1) RETURNING ID_ROLE",
        new_roles.role_name
    )
    .fetch_one(pool.get_ref())
    .await;
    
    match result {
        Ok(row) => {
            info!("Successfully created role: {:?}", row);
            HttpResponse::Created().json(row.id_role)
        }
        Err(e) => {
            error!("Failed to create role: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
    
}