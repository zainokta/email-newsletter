use core::time;
use std::str::FromStr;

use axum::{
    http::{self, HeaderName},
    routing::{get, post},
    Router,
};
use sqlx::PgPool;
use tokio::{net::TcpListener, signal};
use tower_http::{
    catch_panic::CatchPanicLayer,
    cors::{Any, CorsLayer},
    request_id::{MakeRequestUuid, PropagateRequestIdLayer, SetRequestIdLayer},
    timeout::TimeoutLayer,
    trace::{DefaultOnResponse, TraceLayer},
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::health_check::handler as health_check_handler;
use crate::state::AppState;
use crate::subscribe::handler as subscribe_handler;

pub async fn run(listener: TcpListener, connection: PgPool) {
    let shared_connection = AppState::new(connection).await;

    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();

    // TODO: change this to configurable
    let cors = CorsLayer::new()
        .allow_methods(Any)
        .allow_headers(Any)
        .allow_origin(Any);

    let request_id_header_name =
        HeaderName::from_str("x-request-id").expect("Failed to parse request id header name");

    let subscription_router = Router::new().route("/", post(subscribe_handler::subscribe));

    let app = Router::new()
        .route("/healthz", get(health_check_handler::health_check))
        .nest("/subscription", subscription_router)
        .layer(CatchPanicLayer::new())
        .layer(cors)
        .layer(SetRequestIdLayer::new(
            request_id_header_name.to_owned(),
            MakeRequestUuid,
        ))
        .layer(PropagateRequestIdLayer::new(
            request_id_header_name.to_owned(),
        ))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(|request: &http::Request<_>| {
                    let request_id = request
                        .headers()
                        .get("x-request-id")
                        .and_then(|value| value.to_str().ok())
                        .unwrap_or("<unknown>")
                        .to_string();

                        tracing::info_span!("request", request_id = %request_id, method = %request.method(), uri = %request.uri())
                })
                .on_response(DefaultOnResponse::new().include_headers(true)),
        ).layer(TimeoutLayer::new(time::Duration::from_secs(60))) // TODO: change this to configurable
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
