use actix_web::web;
use crate::handlers::person::role_handler::{create_roles, get_roles};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.route("/roles", web::get().to(get_roles));
    cfg.route("/roles", web::post().to(create_roles));
}