use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Location {
    pub locality: String,
    pub department: String,
    pub province: String,
    pub country: String
}

