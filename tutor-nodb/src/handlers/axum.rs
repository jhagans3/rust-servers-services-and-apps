use crate::state::AppState;
use axum::extract::State;
use axum::Json;
use serde_json::{json, Value};
use std::sync::Arc;

#[utoipa::path(
        get,
        path = "/health",
        responses(
            (status = 200, description = "Health found successfully"),
            (status = NOT_FOUND, description = "Health was not found")
        )
    )]
pub async fn health_check(State(app_state): State<Arc<AppState>>) -> Json<Value> {
    let health_check_response = &app_state.health_check_response;
    let mut visit_count = app_state.visit_count.lock().unwrap();
    let status = format!("{health_check_response} {visit_count} times");
    *visit_count += 1;

    Json(json!({"status": status}))
}
