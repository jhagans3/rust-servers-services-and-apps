use actix_web::{App, HttpServer};

use nodb::routes::actix::{actix_router, course_routes};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || App::new().configure(actix_router).configure(course_routes))
        .bind("127.0.0.1:3000")?
        .run()
        .await
}
