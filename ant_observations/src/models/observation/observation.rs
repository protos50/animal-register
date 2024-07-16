use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::NaiveDate;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Observations {
    pub id_observation: i32,
    pub collection_date: NaiveDate,
    pub person_name: String,
    pub person_lastname: String,
    pub trap_name: String,
    pub method_name: String, 
    pub species: String,
    pub genus: String,
    pub tribe: String,
    pub subfamily: String,
    pub family: String,
    pub province: String,
    pub department: String,
    pub locality: String, 
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct PaginationParams {
    pub limit: i32,
    pub offset: i32,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct NewObservation {
    pub id_species: i32,
    pub id_locality: i32,
}