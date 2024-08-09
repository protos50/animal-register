use actix_web::web;
use crate::handlers::collection::preservation_method_handler::{get_preservation_methods, create_preservation_methods};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.route("/preservation_methods", web::get().to(get_preservation_methods));
    cfg.route("/preservation_methods", web::post().to(create_preservation_methods));
}