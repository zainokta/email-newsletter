use axum::{
    routing::{get, post},
    Router,
};
use sqlx::PgPool;
use tokio::{net::TcpListener, signal};

use crate::health_check::*;
use crate::state::AppState;
use crate::subscribe::*;

pub async fn run(listener: TcpListener, connection: PgPool) {
    let shared_connection = AppState::new(connection).await;

    let app = Router::new()
        .route("/healthz", get(health_check::health_check))
        .route("/subscriptions", post(subscription::subscribe))
        .with_state(shared_connection.clone());

    axum::serve(listener, app.into_make_service())
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
