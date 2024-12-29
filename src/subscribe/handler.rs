use axum::{
    extract::{Form, State},
    http::StatusCode,
    response::IntoResponse,
};
use chrono::Utc;
use uuid::Uuid;

use crate::state::AppState;

use super::Subscription;

pub async fn subscribe(
    State(state): State<AppState>,
    Form(subscription): Form<Subscription>,
) -> impl IntoResponse {
    if subscription.name.is_none() || subscription.email.is_none() {
        return (StatusCode::BAD_REQUEST, "Missing name or email").into_response();
    }

    match sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, name, email, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        subscription.name.to_owned(),
        subscription.email.to_owned(),
        Utc::now(),
    )
    .execute(&state.connection.to_owned())
    .await
    {
        Ok(_) => (StatusCode::OK, "Subscription successful").into_response(),
        Err(e) => {
            eprintln!("Failed to execute query: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Failed to execute query").into_response()
        }
    }
}
