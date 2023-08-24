use crate::state::AppState;
use axum::extract::State;
use axum::Json;
use serde_json::{json, Value};
use std::sync::Arc;

pub async fn health_check(state: State<Arc<AppState>>) -> Json<Value> {
    let app_state = state.clone();
    let health_check_response = &app_state.health_check_response;
    let mut visit_count = app_state.visit_count.lock().unwrap();
    let status = format!("{health_check_response} {visit_count} times");
    *visit_count += 1;

    Json(json!({"status": status}))
}
