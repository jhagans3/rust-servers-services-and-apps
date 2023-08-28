use crate::state::AppState;
use std::sync::Mutex;

pub fn app_state() -> AppState {
    AppState {
        health_check_response: "Axum is good. You've already asked me".to_string(),
        visit_count: Mutex::new(0),
        courses: Mutex::new(Vec::new()),
    }
}
