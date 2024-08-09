use actix_web::{web, HttpResponse, Error};
use sqlx::PgPool;
use log::{info, error};
use crate::models::observation::observation::{NewObservation, Observations, PaginationParams};


pub async fn get_observations(pool: web::Data<PgPool>, pagination: web::Query<PaginationParams>) -> HttpResponse {
    let pagination_params = pagination.into_inner();

    let result: Result<Vec<Observations>, sqlx::Error> = sqlx::query_as::<_, Observations>(
        "SELECT * FROM public.get_observations($1, $2)",
    )
    .bind(pagination_params.limit)
    .bind(pagination_params.offset)
    .fetch_all(pool.get_ref())
    .await;

    match result {
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

pub async fn download_observations_csv(
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, Error> {
    let result = sqlx::query_as::<_, Observations>(
        "SELECT * FROM public.get_observations(-1, -1)"
    )
    .fetch_all(pool.get_ref())
    .await;

    match result {
        Ok(rows) => {
            let mut wtr = csv::Writer::from_writer(vec![]);
            for row in rows {
                wtr.serialize(row).unwrap();
            }
            let data = wtr.into_inner().unwrap();
            Ok(HttpResponse::Ok()
                .content_type("text/csv")
                .append_header(("Content-Disposition", "attachment; filename=\"observations.csv\""))
                .body(data))
        }
        Err(e) => {
            error!("Failed to fetch observations: {:?}", e);
            Ok(HttpResponse::InternalServerError().finish())
        }
    }
}