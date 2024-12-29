use tokio::net::TcpListener;
use zero2prod::{configuration::get_configuration, run};

#[tokio::main]
async fn main() {
    let config = get_configuration().expect("Failed to read configuration file");

    let listener = TcpListener::bind(format!("127.0.0.1:{}", config.application_port))
        .await
        .unwrap_or_else(|_| panic!("Failed to bind port {}", config.application_port));

    run(listener).await;
}
