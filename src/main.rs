use core::time;

use sqlx::postgres::PgPoolOptions;
use tokio::net::TcpListener;
use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;

#[tokio::main]
async fn main() {
    let config = get_configuration().expect("Failed to read configuration file");

    let pool = PgPoolOptions::new()
        .max_connections(50)
        .min_connections(10)
        .idle_timeout(time::Duration::from_secs(30))
        .max_lifetime(time::Duration::from_secs(60))
        .connect(&config.database.connection_string())
        .await
        .expect("Failed to connect to Postgres.");

    let listener = TcpListener::bind(format!("127.0.0.1:{}", config.application_port))
        .await
        .unwrap_or_else(|_| panic!("Failed to bind port {}", config.application_port));

    run(listener, pool).await;
}
