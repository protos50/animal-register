use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, FromRow)]
pub struct Family {
    pub id_family: i32,
    pub scientific_name: String,
}
