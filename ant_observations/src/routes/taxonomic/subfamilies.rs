use actix_web::web;
use crate::handlers::taxonomic::subfamily_handler::{get_subfamilies, create_subfamily};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.route("/subfamilies", web::get().to(get_subfamilies));
    cfg.route("/subfamilies", web::post().to(create_subfamily));
}