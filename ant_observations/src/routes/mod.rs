pub mod observations;
pub mod family;

use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/api").configure(observations::config));
    cfg.service(web::scope("/api").configure(family::config));
}
