use actix_web::web;
use crate::handlers::collection::trap_handler::{get_traps, create_traps};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.route("/traps", web::get().to(get_traps));
    cfg.route("/traps", web::post().to(create_traps));
}