pub mod observations;

use actix_web::web;
use crate::handlers::generic_handler::favicon;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/api")
    .configure(observations::config))
    .route("/favicon.ico", web::get().to(favicon));
}
