use actix_web::web;
use crate::handlers::province_handler::{get_provinces, create_provinces, get_simple_provinces};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.route("/provinces", web::get().to(get_provinces));
    cfg.route("/provinces", web::post().to(create_provinces));
    cfg.route("/simple_provinces", web::get().to(get_simple_provinces));
}