use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(FromRow, Serialize, Deserialize, Debug)]
pub struct Species {
    pub id_species: i32,
    pub id_genus: i32,
    pub scientific_name: String,

}

#[derive(FromRow, Serialize, Deserialize, Debug)]
pub struct NewSpecies {
    pub id_genus: i32,
    pub scientific_name: String,

}