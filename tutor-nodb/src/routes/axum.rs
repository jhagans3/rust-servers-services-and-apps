use ::axum::{routing::get, Router};
use std::sync::Arc;

use crate::handlers::axum::health_check;
use crate::state::axum::app_state;

pub fn axum_router() -> Router {
    let app_state = app_state();

    let shared_data = Arc::new(app_state);
    Router::new()
        .route("/health", get(health_check))
        .with_state(shared_data)
}
