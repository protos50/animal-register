use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Department {
    pub id_department: i32,
    pub province: String,
    pub department: String,

}

#[derive(Serialize, Deserialize, FromRow)]
pub struct SimpleDepartment {
    pub id_department: i32,
    pub department_name: String,

}

#[derive(Serialize, Deserialize, FromRow)]
pub struct NewDepartment {
    pub id_province: i32,
    pub department_name: String,

}