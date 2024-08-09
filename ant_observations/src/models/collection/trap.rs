use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Trap {
    pub id_trap: i32,
    pub trap_name: String
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct NewTrap {
    pub trap_name: String
}
