use axum::{routing::get, Json, Router};
use serde_json::{json, Value};

async fn health_check_handler() -> Json<Value> {
    Json(json!({"status": "Hello Axum. EzyTutors is alive and kicking"}))
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/health", get(health_check_handler));

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
