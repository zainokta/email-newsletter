use core::time;

use secrecy::ExposeSecret;
use sqlx::postgres::PgPoolOptions;
use tokio::net::TcpListener;
use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;
use zero2prod::telemetry::{get_subscriber, init_subscriber};

#[tokio::main]
async fn main() {
    let subscriber = get_subscriber("zero2prod".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let config = get_configuration().expect("Failed to read configuration file");

    let pool = PgPoolOptions::new()
        .max_connections(50)
        .min_connections(10)
        .idle_timeout(time::Duration::from_secs(30))
        .max_lifetime(time::Duration::from_secs(60))
        .connect_lazy(config.database.connection_string().expose_secret())
        .expect("Failed to connect to Postgres.");

    let listener = TcpListener::bind(format!(
        "{}:{}",
        config.application.host, config.application.port
    ))
    .await
    .unwrap_or_else(|_| panic!("Failed to bind port {}", config.application.port));

    run(listener, pool).await;
}
