use secrecy::{ExposeSecret, SecretBox, SecretString};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub database: DatabaseConfig,
    pub application_port: u16,
}

#[derive(Deserialize, Clone)]
pub struct DatabaseConfig {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: SecretString,
    pub database_name: String,
}

pub fn get_configuration() -> Result<Config, config::ConfigError> {
    let config = config::Config::builder()
        .add_source(config::File::new("config.yaml", config::FileFormat::Yaml))
        .build()
        .expect("Unable to get configuration file");

    config.try_deserialize::<Config>()
}

impl DatabaseConfig {
    pub fn connection_string(&self) -> SecretString {
        SecretBox::from(format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username,
            self.password.expose_secret(),
            self.host,
            self.port,
            self.database_name
        ))
    }
}
