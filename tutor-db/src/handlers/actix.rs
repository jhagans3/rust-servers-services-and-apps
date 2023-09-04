use actix_web::web::{Data, Json, Path};
use actix_web::{get, post, HttpResponse, Responder};
use serde_json::json;

use crate::db_access::{get_course_details_db, get_courses_for_tutor_db, post_new_course_db};
use crate::models::Course;
use crate::state::AppState;

#[get("/health")]
pub async fn health_check(state: Data<AppState>) -> impl Responder {
    let app_state = state.clone();
    let health_check_response = &app_state.health_check_response;
    let mut visit_count = app_state.visit_count.lock().unwrap();
    let status = format!("{health_check_response} {visit_count} times");
    *visit_count += 1;

    Json(json!({"status": status}))
}

#[post("/courses")]
pub async fn post_new_course(new_course: Json<Course>, app_state: Data<AppState>) -> HttpResponse {
    let course = post_new_course_db(&app_state.db, new_course.into()).await;

    HttpResponse::Ok().json(format!("Added new course: {course:?}"))
}

#[get("/courses/{tutor_id}")]
pub async fn get_courses_for_tutor(app_state: Data<AppState>, params: Path<i32>) -> HttpResponse {
    let tutor_id = params.into_inner();
    let courses = get_courses_for_tutor_db(&app_state.db, tutor_id).await;

    HttpResponse::Ok().json(format!(
        "Got courses: {courses:?} for tutor with id: {tutor_id}"
    ))
}

#[get("/courses/{tutor_id}/{course_id}")]
pub async fn get_course_detail(
    app_state: Data<AppState>,
    params: Path<(i32, i32)>,
) -> HttpResponse {
    let (tutor_id, course_id) = params.into_inner();
    let course = get_course_details_db(&app_state.db, tutor_id, course_id).await;

    HttpResponse::Ok().json(format!("Got course {course:?} using tutor and course id"))
}
