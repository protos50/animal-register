use crate::models::collection::preservation_method::{NewPreservationMethod, PreservationMethod};
use actix_web::{web, HttpResponse};
use log::{error, info};
use sqlx::PgPool;

pub async fn get_preservation_methods(pool: web::Data<PgPool>) -> HttpResponse {
    match sqlx::query_as::<_, PreservationMethod>(
        "
    SELECT
        PM.ID_PRESERVATION_METHOD,
        PM.METHOD_NAME
    FROM
        PRESERVATION_METHOD PM
    ",
    )
    .fetch_all(pool.get_ref())
    .await
    {
        Ok(rows) => {
            info!("Successfully fetched {} preservation methods", rows.len());
            HttpResponse::Ok().json(rows)
        }
        Err(e) => {
            error!("Failed to fetch preservation: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

pub async fn create_preservation_methods(
    new_preservation_method: web::Json<NewPreservationMethod>,
    pool: web::Data<PgPool>,
) -> HttpResponse {
    let new_preservation_method: NewPreservationMethod = new_preservation_method.into_inner();

    // Ejecutar la inserción en la base de datos
    let result = sqlx::query!(
    "INSERT INTO PRESERVATION_METHOD (METHOD_NAME) VALUES ($1) RETURNING ID_PRESERVATION_METHOD",
    new_preservation_method.method_name
)
    .fetch_one(pool.get_ref())
    .await;

    match result {
        Ok(row) => {
            info!("Successfully created preservation method: {:?}", row);
            HttpResponse::Created().json(row.id_preservation_method)
        }
        Err(e) => {
            error!("Failed to create preservation method: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
