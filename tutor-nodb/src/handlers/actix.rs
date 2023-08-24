use actix_web::web::{Data, Json};
use actix_web::{get, Responder};
use serde_json::json;

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
