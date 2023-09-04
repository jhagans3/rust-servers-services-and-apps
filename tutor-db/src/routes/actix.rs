use crate::handlers::actix::{
    get_course_detail, get_courses_for_tutor, health_check, post_new_course,
};
use actix_web::web;

pub fn actix_router(cfg: &mut web::ServiceConfig) {
    cfg.service(health_check);
}

pub fn course_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(post_new_course)
        .service(get_courses_for_tutor)
        .service(get_course_detail);
}
