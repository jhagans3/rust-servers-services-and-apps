use crate::models::poem::Course;
use crate::state::poem::AppState;
use chrono::Utc;
use poem::web::Data;
use poem_openapi::{param::Path, payload::Json, OpenApi};
use serde_json::{json, Value};
use std::sync::Arc;

pub struct EzCourseApi;

#[OpenApi]
impl EzCourseApi {
    #[oai(path = "/health", method = "get")]
    async fn health_check(&self, app_state: Data<&Arc<AppState>>) -> Json<Value> {
        let health_check_response = &app_state.health_check_response;
        let mut visit_count = app_state.visit_count.lock().unwrap();
        let status = format!("{health_check_response} {visit_count} times");
        *visit_count += 1;

        Json(json!({"status": status}))
    }

    #[oai(path = "/courses", method = "post")]
    async fn new_course(
        &self,
        new_course: Json<Course>,
        app_state: Data<&Arc<AppState>>,
    ) -> Json<Value> {
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

        Json(json!("Poem Added course"))
    }

    #[oai(path = "/courses/:tutor_id", method = "get")]
    pub async fn get_courses_for_tutor(
        &self,
        app_state: Data<&Arc<AppState>>,
        tutor_id: Path<i32>,
    ) -> Json<Value> {
        let filtered_courses = app_state
            .courses
            .lock()
            .unwrap()
            .clone()
            .into_iter()
            .filter(|course| course.tutor_id == tutor_id.0)
            .collect::<Vec<Course>>();

        if filtered_courses.len() > 0 {
            Json(json!(filtered_courses))
        } else {
            Json(json!("No courses found for tutor".to_string()))
        }
    }

    #[oai(path = "/courses/:tutor_id/:course_id", method = "get")]
    pub async fn get_course_detail(
        &self,
        app_state: Data<&Arc<AppState>>,
        tutor_id: Path<i32>,
        course_id: Path<i32>,
    ) -> Json<Value> {
        let selected_course = app_state
            .courses
            .lock()
            .unwrap()
            .clone()
            .into_iter()
            .find(|x| x.tutor_id == tutor_id.0 && x.course_id == Some(course_id.0))
            .ok_or("Course not found");

        if let Ok(course) = selected_course {
            Json(json!(course))
        } else {
            Json(json!("Course not found".to_string()))
        }
    }
}
