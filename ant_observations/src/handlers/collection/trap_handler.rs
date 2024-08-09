use crate::models::collection::trap::{NewTrap, Trap};
use actix_web::{web, HttpResponse};
use log::{error, info};
use sqlx::PgPool;

pub async fn get_traps(pool: web::Data<PgPool>) -> HttpResponse {
    match sqlx::query_as::<_, Trap>(
        "
    SELECT
        T.ID_TRAP,
        T.TRAP_NAME 
    FROM
        TRAP T
    ",
    )
    .fetch_all(pool.get_ref())
    .await
    {
        Ok(rows) => {
            info!("Successfully fetched {} trap", rows.len());
            HttpResponse::Ok().json(rows)
        }
        Err(e) => {
            error!("Failed to fetch trap: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

pub async fn create_traps(
    new_trap: web::Json<NewTrap>,
    pool: web::Data<PgPool>,
) -> HttpResponse {
    let new_trap: NewTrap = new_trap.into_inner(); // Convertir de web::Json a Family

    // Ejecutar la inserción en la base de datos
    let result = sqlx::query!(
        "INSERT INTO TRAP (TRAP_NAME) VALUES ($1) RETURNING ID_TRAP",
        new_trap.trap_name
    )
    .fetch_one(pool.get_ref())
    .await;

    match result {
        Ok(row) => {
            info!("Successfully created trap: {:?}", row);
            HttpResponse::Created().json(row.id_trap)
        }
        Err(e) => {
            error!("Failed to create trap: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
