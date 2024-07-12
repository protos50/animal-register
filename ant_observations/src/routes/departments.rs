use actix_web::web;
use crate::handlers::department_handler::{get_departments, create_departments, get_simple_departments};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.route("/departments", web::get().to(get_departments));
    cfg.route("/departments", web::post().to(create_departments));
    cfg.route("/simple_departments", web::get().to(get_simple_departments));
}