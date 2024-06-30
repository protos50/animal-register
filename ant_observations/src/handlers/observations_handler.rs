use actix_web::{web, HttpResponse};
use sqlx::PgPool;
use crate::models::observation::Observation;


pub async fn get_observations(pool: web::Data<PgPool>) -> HttpResponse {
    match sqlx::query_as::<_, Observation>("SELECT id, detail FROM observations")
        .fetch_all(pool.get_ref())
        .await {
            Ok(rows) => HttpResponse::Ok().json(rows),
            Err(_) => HttpResponse::InternalServerError().finish(),
        }
}
