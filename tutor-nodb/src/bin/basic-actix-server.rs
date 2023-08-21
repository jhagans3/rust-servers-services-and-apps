// use actix_web::web::get;
use actix_web::web::Json;
use actix_web::{get, App, HttpServer, Responder};
use serde_json::json;

#[get("/health")]
async fn health_check_handler() -> impl Responder {
    Json(json!({"status": "Hello Actix. EzyTutors is alive and kicking"}))
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    //     HttpServer::new(|| App::new().route("/health", get().to(health_check_handler)))
    HttpServer::new(|| App::new().service(health_check_handler))
        .bind("127.0.0.1:3000")?
        .run()
        .await
}
