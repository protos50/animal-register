use actix_web::web;
use crate::handlers::observation_handler::{create_observation, download_observations_csv, get_observations};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.route("/observations", web::get().to(get_observations));
    cfg.route("/observations", web::post().to(create_observation));
    cfg.route("/download_observations_csv", web::get().to(download_observations_csv));
}