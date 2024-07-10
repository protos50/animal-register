use actix_web::web;
use crate::handlers::locality_handler::{get_localities, create_localities, get_simple_localities};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.route("/localities", web::get().to(get_localities));
    cfg.route("/localities", web::post().to(create_localities));
    cfg.route("simple_localities", web::get().to(get_simple_localities));
}