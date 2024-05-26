// src/handlers/generic_handler.rs

use actix_web::{HttpResponse, Responder};

pub async fn favicon() -> impl Responder {
    HttpResponse::NotFound()
}
