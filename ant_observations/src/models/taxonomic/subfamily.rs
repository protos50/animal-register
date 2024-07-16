use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(FromRow, Serialize, Deserialize, Debug)]
pub struct Subfamily {
    pub id_subfamily: i32,
    pub subfamily: String,
    pub family: String,

}

#[derive(Serialize, Deserialize, Debug, FromRow)]
pub struct NewSubfamily {
    pub id_family: i32,
    pub scientific_name: String,
    
}