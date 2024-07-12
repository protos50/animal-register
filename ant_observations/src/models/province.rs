use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Province {
    pub id_province: i32,
    pub country: String,
    pub province: String
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct SimpleProvince {
    pub id_province: i32,
    pub province_name: String
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct NewProvince {
    pub id_country: i32,
    pub province_name: String
}

