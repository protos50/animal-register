pub mod families;
pub mod species;
pub mod genera;
pub mod subfamilies;
pub mod tribes;
pub mod observations;

use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    // Usar un único scope para ambas configuraciones
    cfg.service(
        web::scope("/api")
            .configure(families::config)
            .configure(genera::config)
            .configure(species::config)
            .configure(subfamilies::config)
            .configure(tribes::config)
            .configure(observations::config)
    );
}
