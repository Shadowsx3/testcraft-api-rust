use actix_web::web;
use crate::handlers::base_handler::get_scope;

pub fn config_app(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(get_scope())
    );
}