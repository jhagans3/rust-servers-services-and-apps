use actix_web::web::{Data, Json, Path};
use actix_web::{get, post, HttpResponse, Responder};
use serde_json::json;

use crate::models::Course;
use crate::state::AppState;
use chrono::Utc;

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
pub async fn new_course(new_course: Json<Course>, app_state: Data<AppState>) -> HttpResponse {
    let course_count_for_user = app_state
        .courses
        .lock()
        .unwrap()
        .clone()
        .into_iter()
        .filter(|course| course.tutor_id == new_course.tutor_id)
        .count() as i32;

    let new_course = Course {
        tutor_id: new_course.tutor_id,
        course_id: Some(course_count_for_user + 1),
        course_name: new_course.course_name.clone(),
        posted_time: Some(Utc::now().naive_utc()),
    };

    app_state.courses.lock().unwrap().push(new_course);

    HttpResponse::Ok().json("Added course")
}

#[get("/courses/{tutor_id}")]
pub async fn get_courses_for_tutor(app_state: Data<AppState>, params: Path<i32>) -> HttpResponse {
    let tutor_id: i32 = params.into_inner();

    let filtered_courses = app_state
        .courses
        .lock()
        .unwrap()
        .clone()
        .into_iter()
        .filter(|course| course.tutor_id == tutor_id)
        .collect::<Vec<Course>>();

    if filtered_courses.len() > 0 {
        HttpResponse::Ok().json(filtered_courses)
    } else {
        HttpResponse::Ok().json("No courses found for tutor".to_string())
    }
}

#[get("/courses/{tutor_id}/{course_id}")]
pub async fn get_course_detail(
    app_state: Data<AppState>,
    params: Path<(i32, i32)>,
) -> HttpResponse {
    let (tutor_id, course_id) = params.into_inner();
    let selected_course = app_state
        .courses
        .lock()
        .unwrap()
        .clone()
        .into_iter()
        .find(|x| x.tutor_id == tutor_id && x.course_id == Some(course_id))
        .ok_or("Course not found");

    if let Ok(course) = selected_course {
        HttpResponse::Ok().json(course)
    } else {
        HttpResponse::Ok().json("Course not found".to_string())
    }
}

/*
#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{http::StatusCode, web};
    use std::sync::Mutex;

    #[tokio::test]
    async fn post_course_test() {
        let course = web::Json(Course {
            tutor_id: 1,
            course_name: "Hello, this is test course".into(),
            course_id: None,
            posted_time: None,
        });
        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            courses: Mutex::new(vec![]),
        });

        let resp = new_course(course, app_state).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }
}
*/
