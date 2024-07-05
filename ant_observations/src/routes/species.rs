use actix_web::web;
use crate::handlers::species_handler::{get_species, create_species, get_simple_species};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.route("/species", web::get().to(get_species));
    cfg.route("/species", web::post().to(create_species));
    cfg.route("/simple_species", web::get().to(get_simple_species));
}