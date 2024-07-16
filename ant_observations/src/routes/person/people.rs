use actix_web::web;
use crate::handlers::person::person_handler::{create_people, get_people};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.route("/people", web::get().to(get_people));
    cfg.route("/people", web::post().to(create_people));
}