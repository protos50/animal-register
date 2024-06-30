use actix_web::{get, post, web, HttpResponse, Responder};
use sqlx::PgPool;
use serde::Deserialize;
use crate::models::family::Family;

#[derive(Deserialize)]
struct CreateFamily {
    scientific_name: String,
}

//#[get("/families")]
/* 
pub async fn get_families(pool: web::Data<PgPool>) -> impl Responder {
    let families = sqlx::query_as!(Family, "SELECT id_family, scientific_name FROM family")
        .fetch_all(pool.get_ref())
        .await
        .unwrap();
    HttpResponse::Ok().json(families)
}*/

pub async fn get_families(pool: web::Data<PgPool>) -> HttpResponse {
    match sqlx::query_as::<_, Family>("SELECT id_family, scientific_name FROM family")
        .fetch_all(pool.get_ref())
        .await {
            Ok(rows) => HttpResponse::Ok().json(rows),
            Err(_) => HttpResponse::InternalServerError().finish(),
        }
}
/*
#[post("/families")]
pub async fn create_family(pool: web::Data<PgPool>, item: web::Json<CreateFamily>) -> impl Responder {
    let result = sqlx::query!(
        "INSERT INTO Family (scientific_name) VALUES ($1) RETURNING id_family",
        item.scientific_name
    )
    .fetch_one(pool.get_ref())
    .await;

    match result {
        Ok(record) => HttpResponse::Ok().json(record),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
 */