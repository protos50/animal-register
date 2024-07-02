use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Family {
    pub id_family: i32,
    pub scientific_name: String
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct NewFamily {
    pub scientific_name: String
}
