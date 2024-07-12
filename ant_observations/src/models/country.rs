use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Country {
    pub id_country: i32,
    pub country: String 
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct NewCountry {
    pub country_name: String
}