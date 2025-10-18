// Application settings

use serde::Deserialize;
use config::{Config, ConfigError, File, Environment};

#[derive(Debug, Deserialize, Clone)]
pub struct Settings {
    pub database: DatabaseConfig,
    pub logging: LoggingConfig,
    pub application: ApplicationConfig,
}

#[derive(Debug, Deserialize, Clone)]
pub struct DatabaseConfig {
    pub host: String,
    pub port: u16,
    pub name: String,
    pub user: String,
    pub password: String,
    pub max_connections: u32,
}

impl DatabaseConfig {
    pub fn database_url(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.user, self.password, self.host, self.port, self.name
        )
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct LoggingConfig {
    pub level: String,
    pub file: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ApplicationConfig {
    pub default_project_status: String,
    pub date_format: String,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let config = Config::builder()
            // Start with default config file
            .add_source(File::with_name("config/default").required(false))
            // Override with environment variables (with prefix DEVERP)
            .add_source(Environment::with_prefix("DEVERP").separator("__"))
            .build()?;

        config.try_deserialize()
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            database: DatabaseConfig {
                host: "localhost".to_string(),
                port: 5432,
                name: "deverp".to_string(),
                user: "deverp_user".to_string(),
                password: "2147483647".to_string(),
                max_connections: 5,
            },
            logging: LoggingConfig {
                level: "info".to_string(),
                file: None,
            },
            application: ApplicationConfig {
                default_project_status: "planning".to_string(),
                date_format: "%Y-%m-%d".to_string(),
            },
        }
    }
}
