use actix_web::web;
use crate::handlers::genus_handler::{get_genera, create_genus};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.route("/genera", web::get().to(get_genera));
    cfg.route("/genera", web::post().to(create_genus));
}