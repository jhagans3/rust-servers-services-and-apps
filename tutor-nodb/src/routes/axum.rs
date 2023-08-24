use ::axum::{routing::get, Router};
use std::sync::{Arc, Mutex};

use crate::handlers::axum::health_check;
use crate::state::axum::AppState;

pub fn axum_router() -> Router {
    let app_state = AppState {
        health_check_response: "I'm good. You've already asked me ".to_string(),
        visit_count: Mutex::new(0),
    };

    let shared_data = Arc::new(app_state);
    Router::new()
        .route("/health", get(health_check))
        .with_state(shared_data)
}
