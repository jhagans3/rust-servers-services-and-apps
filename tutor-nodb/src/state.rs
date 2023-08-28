pub mod actix;
pub mod axum;

use std::sync::Mutex;

use crate::models::Course;

#[derive(Debug)]
pub struct AppState {
    pub health_check_response: String,
    pub visit_count: Mutex<u32>,
    pub courses: Mutex<Vec<Course>>,
}
