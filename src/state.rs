use sqlx::PgPool;

#[derive(Clone)]
pub struct AppState {
    pub(crate) connection: PgPool,
}

impl AppState {
    pub async fn new(connection: PgPool) -> AppState {
        Self { connection }
    }
}
