use poem::{listener::TcpListener, Route};
use poem_openapi::{payload::Json, OpenApi, OpenApiService};
use serde_json::{json, Value};

struct Api;

#[OpenApi]
impl Api {
    #[oai(path = "/health", method = "get")]
    async fn index(&self) -> Json<Value> {
        Json(json!({"status": "Hello Poem. EzyTutors is alive and kicking"}))
    }
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let api_service =
        OpenApiService::new(Api, "Hello World", "1.0").server("http://localhost:3000/");
    let ui = api_service.swagger_ui();
    let app = Route::new().nest("/", api_service).nest("/swagger", ui);

    poem::Server::new(TcpListener::bind("127.0.0.1:3000"))
        .run(app)
        .await
}
