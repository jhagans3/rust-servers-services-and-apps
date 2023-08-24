use actix_web::{App, HttpServer};

use nodb::handlers::actix::health_check;
use nodb::state::actix::app_state;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let app_data = app_state();

    HttpServer::new(move || App::new().app_data(app_data.clone()).service(health_check))
        .bind("127.0.0.1:3000")?
        .run()
        .await
}
