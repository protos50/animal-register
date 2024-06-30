use actix_web::{web, HttpResponse};
use crate::handlers::family_handler::get_families;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.route("/families", web::get().to(get_families));
}
