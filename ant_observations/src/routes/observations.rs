use actix_web::web;
use crate::handlers::observation_handler::get_observations;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.route("/observations", web::get().to(get_observations));
}