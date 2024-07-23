use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Role {
    pub id_role: i32,
    pub role_name: String,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct NewRole {
    pub role_name: String,
}
