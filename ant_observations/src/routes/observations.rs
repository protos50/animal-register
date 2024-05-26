use actix_web::{web, HttpResponse};
use crate::handlers::observations_handler::get_observations;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.route("/observations", web::get().to(get_observations));
}
