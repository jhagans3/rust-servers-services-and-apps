use nodb::routes;

#[tokio::main]
async fn main() {
    let app = routes::axum::axum_router();

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
