use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(FromRow, Serialize, Deserialize, Debug)]
pub struct Species {
    pub id_species: i32,
    pub species: String,
    pub genus: String,

}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct SimpleSpecies {
    pub id_species: i32,
    pub scientific_name: String,
}

#[derive(FromRow, Serialize, Deserialize, Debug)]
pub struct NewSpecies {
    pub id_genus: i32,
    pub scientific_name: String,

}