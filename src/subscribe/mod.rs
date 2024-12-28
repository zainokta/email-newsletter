use axum::{extract::Form, http::StatusCode, response::IntoResponse};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Subscription {
    name: Option<String>,
    email: Option<String>,
}

pub async fn subscribe(Form(subscription): Form<Subscription>) -> impl IntoResponse {
    if subscription.name.is_none() || subscription.email.is_none() {
        return (StatusCode::BAD_REQUEST, "Invalid data").into_response();
    }

    (StatusCode::OK, "All good").into_response()
}
