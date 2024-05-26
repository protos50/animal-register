use actix_web::{HttpResponse};

pub async fn get_observations() -> HttpResponse {
    let data = vec![
        "Observation 1: Details about ant behavior...",
        "Observation 2: Details about ant habitat...",
    ];
    HttpResponse::Ok().json(data)
}
