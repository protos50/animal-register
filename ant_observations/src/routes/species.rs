use actix_web::web;
use crate::handlers::species_handler::{get_species, create_species};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.route("/species", web::get().to(get_species));
    cfg.route("/species", web::post().to(create_species));
}