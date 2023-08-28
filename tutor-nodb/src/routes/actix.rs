use crate::handlers::actix::{get_course_detail, get_courses_for_tutor, health_check, new_course};
use crate::state::actix::app_state;
use actix_web::web;

pub fn actix_router(cfg: &mut web::ServiceConfig) {
    let app_data = app_state();
    cfg.app_data(app_data).service(health_check);
}

pub fn course_routes(cfg: &mut web::ServiceConfig) {
    let app_data = app_state();
    cfg.app_data(app_data)
        .service(new_course)
        .service(get_courses_for_tutor)
        .service(get_course_detail);
}
