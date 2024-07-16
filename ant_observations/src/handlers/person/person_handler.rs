use actix_web::{web, HttpResponse, Error};
use sqlx::PgPool;
use log::{info, error};
use crate::models::person::person::{Person, NewPerson};

pub async fn get_people(pool: web::Data<PgPool>) -> HttpResponse {
    match sqlx::query_as::<_, Person>("
    SELECT
        P.ID_PERSON,
        P.PERSON_NAME,
        P.PERSON_LASTNAME
    FROM
        PERSON P;
    ")
        .fetch_all(pool.get_ref())
        .await {
            Ok(rows) => {
                info!("Successfully fetched {} people", rows.len());
                HttpResponse::Ok().json(rows)
            },
            Err(e) => {
                error!("Failed to fetch people: {:?}", e);
                HttpResponse::InternalServerError().finish()
            },  
        }  
}


pub async fn create_people(new_people: web::Json<NewPerson>, pool: web::Data<PgPool>) -> HttpResponse {
    let new_people: NewPerson = new_people.into_inner(); // Convertir de web::Json a Family
    
    // Ejecutar la inserción en la base de datos
    let result = sqlx::query!(
        "INSERT INTO PERSON (PERSON_NAME, PERSON_LASTNAME) VALUES ($1, $2) RETURNING ID_PERSON",
        new_people.person_name, new_people.person_lastname
    )
    .fetch_one(pool.get_ref())
    .await;
    
    match result {
        Ok(row) => {
            info!("Successfully created person: {:?}", row);
            HttpResponse::Created().json(row.id_person)
        }
        Err(e) => {
            error!("Failed to create person: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
    
}
