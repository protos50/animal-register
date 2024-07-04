use actix_web::{web, HttpResponse};
use sqlx::PgPool;
use log::{info, error};
use crate::models::observation::Observations;
/* 
pub async fn get_species_with_relations(pool: web::Data<PgPool>) -> impl Responder {
    let result = sqlx::query_as!(
        SpeciesWithRelations,
        r#"
        SELECT e.scientific_name AS species, 
               g.scientific_name AS genus, 
               t.scientific_name AS tribe, 
               s.scientific_name AS subfamily, 
               f.scientific_name AS family
        FROM species e
        JOIN genus g ON e.id_genus = g.id_genus
        JOIN tribe t ON g.id_tribe = t.id_tribe
        JOIN subfamily s ON t.id_subfamily = s.id_subfamily
        JOIN family f ON s.id_family = f.id_family;
        "#
    )
    .fetch_all(pool.get_ref())
    .await;

    match result {
        Ok(rows) => HttpResponse::Ok().json(rows),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
*/
pub async fn get_observations(pool: web::Data<PgPool>) -> HttpResponse {
    match sqlx::query_as::<_, Observations>("
    SELECT
        O.ID_OBSERVATION,
        E.SCIENTIFIC_NAME AS SPECIES,
        G.SCIENTIFIC_NAME AS GENUS,
        T.SCIENTIFIC_NAME AS TRIBE,
        S.SCIENTIFIC_NAME AS SUBFAMILY,
        F.SCIENTIFIC_NAME AS FAMILY
    FROM
        OBSERVATION O
        JOIN SPECIES E ON O.ID_SPECIES = E.ID_SPECIES
        JOIN GENUS G ON E.ID_GENUS = G.ID_GENUS
        JOIN TRIBE T ON G.ID_TRIBE = T.ID_TRIBE
        JOIN SUBFAMILY S ON T.ID_SUBFAMILY = S.ID_SUBFAMILY
        JOIN FAMILY F ON S.ID_FAMILY = F.ID_FAMILY;
    ")
        .fetch_all(pool.get_ref())
        .await {
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