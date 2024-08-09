pub mod observation;
pub mod geographic;
pub mod collection;
pub mod taxonomic;
pub mod person;

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
            .configure(person::people::config)
            .configure(person::roles::config)
            .configure(collection::preservation_method::config)
            .configure(collection::trap::config)

    );
}
