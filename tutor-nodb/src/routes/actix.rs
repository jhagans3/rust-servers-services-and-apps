use crate::handlers::actix::health_check;
use crate::state::actix::app_state;
use actix_web::web;

pub fn actix_router(cfg: &mut web::ServiceConfig) {
    let app_data = app_state();
    cfg.app_data(app_data).service(health_check);
}
