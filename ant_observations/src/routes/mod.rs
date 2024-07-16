pub mod observation;
pub mod geographic;
pub mod taxonomic;


use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .configure(taxonomic::families::config)
            .configure(taxonomic::genera::config)
            .configure(taxonomic::species::config)
            .configure(taxonomic::subfamilies::config)
            .configure(taxonomic::tribes::config)
            .configure(observation::observations::config)
            .configure(geographic::localities::config)
            .configure(geographic::departments::config)
            .configure(geographic::provinces::config)
            .configure(geographic::countries::config)
    );
}
