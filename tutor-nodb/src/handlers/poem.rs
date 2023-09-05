use crate::state::AppState;
use poem::web::Data;
use poem_openapi::{payload::Json, OpenApi};
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
}
