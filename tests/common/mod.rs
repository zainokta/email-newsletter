use tokio::net::TcpListener;

pub async fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0")
        .await
        .expect("Failed to bind port 8080");

    let port = listener.local_addr().unwrap().port();

    let server = zero2prod::run(listener);

    let _ = tokio::spawn(server);

    format!("http://127.0.0.1:{}", port)
}
