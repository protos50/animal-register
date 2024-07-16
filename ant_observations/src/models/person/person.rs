use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Person {
    pub id_person: i32,
    pub person_name: String,
    pub person_lastname: String
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct NewPerson {
    pub person_name: String,
    pub person_lastname: String
}