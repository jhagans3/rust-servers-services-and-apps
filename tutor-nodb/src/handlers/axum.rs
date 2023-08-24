use crate::state::axum::AppState;
use axum::extract::State;
use axum::Json;
use serde_json::{json, Value};
use std::sync::Arc;

pub async fn health_check(state: State<Arc<AppState>>) -> Json<Value> {
    let app_state = state.clone();
    let health_check_response = &app_state.health_check_response;
    let mut visit_count = app_state.visit_count.lock().unwrap();
    let s = format!("{} {} times", health_check_response, visit_count);
    *visit_count += 1;

    Json(json!({"status": s}))
}
