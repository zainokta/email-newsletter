use secrecy::{ExposeSecret, SecretBox, SecretString};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub database: DatabaseConfig,
    pub application: ApplicationConfig,
}

#[derive(Deserialize, Clone)]
pub struct DatabaseConfig {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: SecretString,
    pub database_name: String,
}

#[derive(Deserialize, Clone)]
pub struct ApplicationConfig {
    pub port: u16,
    pub host: String,
}

pub fn get_configuration() -> Result<Config, config::ConfigError> {
    let base_path = std::env::current_dir().expect("Failed to determine the current directory");

    let configuration_directory = base_path.join("configuration");

    let environment: Environment = std::env::var("APP_ENVIRONMENT")
        .unwrap_or_else(|_| "local".into())
        .try_into()
        .expect("Failed to parse APP_ENVIRONMENT.");

    let environment_filename = format!("{}.yaml", environment.as_str());

    let config = config::Config::builder()
        .add_source(config::File::from(
            configuration_directory.join("base.yaml"),
        ))
        .add_source(config::File::from(
            configuration_directory.join(environment_filename),
        ))
        .build()?;

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

pub enum Environment {
    Local,
    Production,
}

impl Environment {
    pub fn as_str(&self) -> &'static str {
        match self {
            Environment::Local => "local",
            Environment::Production => "production",
        }
    }
}

impl TryFrom<String> for Environment {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "local" => Ok(Self::Local),
            "production" => Ok(Self::Production),
            other => Err(format!(
                "{} is not a supported environment. \
                Use either `local` or `production`.",
                other
            )),
        }
    }
}
