use nodb::routes::poem::app;
use poem::listener::TcpListener;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let app = app();

    poem::Server::new(TcpListener::bind("127.0.0.1:3000"))
        .run(app)
        .await
}
