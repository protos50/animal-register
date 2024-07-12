use actix_web::web;
use crate::handlers::country_handler::{get_countries, create_countries};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.route("/countries", web::get().to(get_countries));
    cfg.route("/countries", web::post().to(create_countries));
}