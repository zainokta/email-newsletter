use sqlx::{Connection, Executor, PgConnection, PgPool};
use tokio::net::TcpListener;
use uuid::Uuid;
use zero2prod::configuration::{get_configuration, DatabaseConfig};
use zero2prod::startup::run;

#[warn(dead_code)]
pub struct TestApp {
    pub address: String,
    pub pool: PgPool,
}

pub async fn spawn_app() -> TestApp {
    let listener = TcpListener::bind("127.0.0.1:0")
        .await
        .expect("Failed to bind port 8080");

    let mut config = get_configuration().expect("Failed to read configuration.");
    config.database.database_name = Uuid::new_v4().to_string();

    let connection_pool = configure_database(&config.database).await;

    let port = listener.local_addr().unwrap().port();

    let server = run(listener, connection_pool.to_owned());

    let _ = tokio::spawn(server);

    TestApp {
        address: format!("http://127.0.0.1:{}", port),
        pool: connection_pool,
    }
}

pub async fn configure_database(config: &DatabaseConfig) -> PgPool {
    let maintenance_config = DatabaseConfig {
        database_name: "postgres".to_string(),
        username: "postgres".to_string(),
        password: "password".to_string(),
        ..config.clone()
    };

    let mut connection = PgConnection::connect(&maintenance_config.connection_string())
        .await
        .expect("Failed to connect to Postgres.");

    connection
        .execute(format!(r#"CREATE DATABASE "{}";"#, config.database_name).as_str())
        .await
        .expect("Failed to create database.");

    let connection_pool = PgPool::connect(&config.connection_string())
        .await
        .expect("Failed to connect to Postgres.");

    sqlx::migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("Failed to run migrations.");

    connection_pool
}
