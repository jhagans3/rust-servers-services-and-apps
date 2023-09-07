use crate::models::poem::Course;
use std::sync::Mutex;

pub struct AppState {
    pub health_check_response: String,
    pub visit_count: Mutex<u32>,
    pub courses: Mutex<Vec<Course>>,
}

pub fn app_state() -> AppState {
    AppState {
        health_check_response: "Poem is good. You've already asked me".to_string(),
        visit_count: Mutex::new(0),
        courses: Mutex::new(Vec::new()),
    }
}
