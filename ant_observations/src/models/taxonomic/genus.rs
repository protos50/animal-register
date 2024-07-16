use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Genus {
    pub id_genus: i32,
    pub genus: String,
    pub tribe: String,

}

#[derive(Serialize, Deserialize, FromRow)]
pub struct NewGenus {
    pub id_tribe: i32,
    pub scientific_name: String,

}