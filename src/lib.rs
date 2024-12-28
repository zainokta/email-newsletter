use axum::{routing::{get, post}, Router};
use tokio::{net::TcpListener, signal};

mod health_check;
mod subscribe;

pub async fn run(listener: TcpListener) {
    let app = Router::new()
        .route("/healthz", get(health_check::health_check))
        .route("/subscriptions", post(subscribe::subscribe));

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}

async fn shutdown_signal() {
    let sigkill = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = sigkill => {},
        _ = terminate => {},
    }
}
