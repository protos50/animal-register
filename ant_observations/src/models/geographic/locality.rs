use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Locality {
    pub id_locality: i32,
    pub department: String,
    pub locality: String,

}

#[derive(Serialize, Deserialize, FromRow)]
pub struct SimpleLocality {
    pub id_locality: i32,
    pub locality_name: String,

}

#[derive(Serialize, Deserialize, FromRow)]
pub struct NewLocality {
    pub id_department: i32,
    pub locality_name: String,

}

/*
CREATE TABLE LOCALITY (
    ID_LOCALITY SERIAL,
	ID_DEPARTMENT INTEGER NOT NULL,
	LOCALITY_NAME TEXT NOT NULL,
	CONSTRAINT FK_LOCALITY_DEPARTMENT FOREIGN KEY (ID_DEPARTMENT) REFERENCES DEPARTMENT (ID_DEPARTMENT),
	CONSTRAINT PK_LOCALITY PRIMARY KEY (ID_LOCALITY)
);
 */