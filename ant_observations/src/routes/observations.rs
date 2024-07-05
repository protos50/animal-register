use actix_web::web;
use crate::handlers::observation_handler::{get_observations, create_observation};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.route("/observations", web::get().to(get_observations));
    cfg.route("/observations", web::post().to(create_observation));
}