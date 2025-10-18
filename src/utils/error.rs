// Error types for DevERP

use thiserror::Error;

#[derive(Error, Debug)]
pub enum DevErpError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Configuration error: {0}")]
    Config(String),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Conflict: {0}")]
    Conflict(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Internal error: {0}")]
    Internal(String),
}

// Implement From for common error types
impl From<config::ConfigError> for DevErpError {
    fn from(err: config::ConfigError) -> Self {
        DevErpError::Config(err.to_string())
    }
}
