use tokio::net::TcpListener;
use zero2prod::run;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080")
        .await
        .expect("Failed to bind port 8080");

    run(listener).await;
}
