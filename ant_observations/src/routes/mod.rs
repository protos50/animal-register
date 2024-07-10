pub mod families;
pub mod species;
pub mod genera;
pub mod subfamilies;
pub mod tribes;
pub mod observations;
pub mod localities;
pub mod departments;

use actix_web::web;

use crate::models::department;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .configure(families::config)
            .configure(genera::config)
            .configure(species::config)
            .configure(subfamilies::config)
            .configure(tribes::config)
            .configure(observations::config)
            .configure(localities::config)
            .configure(departments::config)
    );
}
