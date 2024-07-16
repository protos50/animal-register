use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(FromRow, Serialize, Deserialize, Debug)]
pub struct Tribe {
    pub id_tribe: i32,
    pub tribe: String,
    pub subfamily: String,

}

#[derive(Serialize, Deserialize, Debug, FromRow)]
pub struct NewTribe {
    pub id_subfamily: i32,
    pub scientific_name: String,
    
}