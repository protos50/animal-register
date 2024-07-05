use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Observations {
    pub id_observation: i32,
    pub species: String,
    pub genus: String,
    pub tribe: String,
    pub subfamily: String,
    pub family: String,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct NewObservation {
    pub id_species: i32,
}