use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct PreservationMethod {
    pub id_preservation_method: i32,
    pub method_name: String
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct NewPreservationMethod {
    pub method_name: String
}