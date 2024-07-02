use actix_web::web;
use crate::handlers::tribe_handler::{get_tribes, create_tribe};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.route("/tribes", web::get().to(get_tribes));
    cfg.route("/tribes", web::post().to(create_tribe));
}