use axum::{
    extract::{Form, State},
    http::StatusCode,
    response::IntoResponse,
};
use chrono::Utc;
use garde::Validate;
use sqlx::PgPool;
use uuid::Uuid;

use crate::state::AppState;

use super::Subscription;

#[tracing::instrument(name = "Adding a new subscriber",
    skip(state, subscription),
    fields(
        subscriber_email = %subscription.email,
        subscriber_name = %subscription.name
    )
)]
pub async fn subscribe(
    State(state): State<AppState>,
    Form(subscription): Form<Subscription>,
) -> impl IntoResponse {
    if let Err(e) = subscription.validate() {
        tracing::error!("Failed to validate subscription: {:?}", e);
        return StatusCode::BAD_REQUEST.into_response();
    }

    match insert_subscriber(&state.connection, subscription).await {
        Ok(_) => StatusCode::OK.into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

#[tracing::instrument(
    name = "Saving new subscriber details in the database",
    skip(subscription, pool)
)]
async fn insert_subscriber(pool: &PgPool, subscription: Subscription) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, name, email, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        subscription.name.to_owned(),
        subscription.email.to_owned(),
        Utc::now(),
    )
    .execute(&pool.to_owned())
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query: {:?}", e);
        e
    })?;

    Ok(())
}
