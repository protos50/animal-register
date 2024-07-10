use actix_web::{web, HttpResponse};
use sqlx::PgPool;
use log::{info, error};
use crate::models::department::{Department, SimpleDepartment, NewDepartment};

pub async fn get_departments(pool: web::Data<PgPool>) -> HttpResponse {
    match sqlx::query_as::<_, Department>("
    SELECT
        D.ID_DEPARTMENT,
        P.PROVINCE_NAME AS PROVINCE,
        D.DEPARTMENT_NAME AS DEPARTMENT 
    FROM
        DEPARTMENT D
        JOIN PROVINCE P ON D.ID_PROVINCE = P.ID_PROVINCE;
    ")
        .fetch_all(pool.get_ref())
        .await {
            Ok(rows) => {
                info!("Successfully fetched {} department", rows.len());
                HttpResponse::Ok().json(rows)
            },
            Err(e) => {
                error!("Failed to fetch department: {:?}", e);
                HttpResponse::InternalServerError().finish()
            },  
        }
}

pub async fn create_departments(new_department: web::Json<NewDepartment>, pool: web::Data<PgPool>) -> HttpResponse {
    let new_department: NewDepartment = new_department.into_inner(); // Convertir de web::Json a Family
    
    // Ejecutar la inserción en la base de datos
    let result = sqlx::query!(
        "INSERT INTO DEPARTMENT (ID_PROVINCE, DEPARTMENT_NAME) VALUES ($1, $2) RETURNING ID_DEPARTMENT",
        new_department.id_province, new_department.department_name
    )
    .fetch_one(pool.get_ref())
    .await;
    
    match result {
        Ok(row) => {
            info!("Successfully created locality: {:?}", row);
            HttpResponse::Created().json(row.id_department)
        }
        Err(e) => {
            error!("Failed to create locality: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

pub async fn get_simple_departments(pool: web::Data<PgPool>) -> HttpResponse {
    match sqlx::query_as::<_, SimpleDepartment>("
    SELECT
        ID_DEPARTMENT,
        DEPARTMENT_NAME
    FROM
        DEPARTMENT;
    ")
        .fetch_all(pool.get_ref())
        .await {
            Ok(rows) => {
                info!("Successfully fetched {} department", rows.len());
                HttpResponse::Ok().json(rows)
            },
            Err(e) => {
                error!("Failed to fetch department: {:?}", e);
                HttpResponse::InternalServerError().finish()
            },  
        }
}