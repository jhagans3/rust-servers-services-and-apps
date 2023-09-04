use actix_web::{App, HttpServer};
use dotenv::dotenv;
use sqlx::postgres::PgPool;
use std::env;

use tutor_db::{
    routes::actix::{actix_router, course_routes},
    state::actix::app_state,
};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let pg_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let db_pool = PgPool::connect(&pg_url).await.unwrap();
    let shared_data = app_state(db_pool);

    let app = move || {
        App::new()
            .app_data(shared_data.clone())
            .configure(actix_router)
            .configure(course_routes)
    };

    HttpServer::new(app).bind("127.0.0.1:3000")?.run().await
}
