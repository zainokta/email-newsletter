use axum::response::IntoResponse;

pub async fn subscribe() -> impl IntoResponse {
    axum::http::StatusCode::OK
}
