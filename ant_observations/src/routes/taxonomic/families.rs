use actix_web::web;
use crate::handlers::taxonomic::family_handler::{create_family, get_families};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.route("/families", web::get().to(get_families));
    cfg.route("/families", web::post().to(create_family));
}
