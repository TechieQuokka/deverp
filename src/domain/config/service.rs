use std::sync::Arc;

use sqlx::PgPool;

use crate::domain::config::entity::{ConfigDataType, Configuration, UpdateConfiguration};
use crate::domain::config::repository::ConfigRepository;
use crate::utils::error::DevErpError;

pub struct ConfigService {
    repository: Arc<dyn ConfigRepository>,
    pool: PgPool,
}

impl ConfigService {
    pub fn new(repository: Arc<dyn ConfigRepository>, pool: PgPool) -> Self {
        Self { repository, pool }
    }

    pub async fn get_config(&self, key: &str) -> Result<Configuration, DevErpError> {
        let config = self.repository.find_by_key(key).await?.ok_or_else(|| {
            DevErpError::NotFound(format!("Configuration key '{}' not found", key))
        })?;

        Ok(config)
    }

    pub async fn get_all_configs(&self) -> Result<Vec<Configuration>, DevErpError> {
        self.repository.find_all().await
    }

    pub async fn set_config(
        &self,
        key: &str,
        value: String,
        description: Option<String>,
    ) -> Result<Configuration, DevErpError> {
        // First, get the existing configuration to check its data type
        let existing = self.repository.find_by_key(key).await?.ok_or_else(|| {
            DevErpError::NotFound(format!("Configuration key '{}' not found", key))
        })?;

        // Validate the value based on data type
        self.validate_value(&value, &existing.data_type)?;

        // Update the configuration
        let update = UpdateConfiguration {
            config_key: key.to_string(),
            config_value: value,
            description,
        };

        self.repository.update(update).await
    }

    pub async fn reset_to_defaults(&self) -> Result<(), DevErpError> {
        self.repository.reset_to_defaults().await
    }

    pub async fn test_database_connection(&self) -> Result<String, DevErpError> {
        // Simple query to test database connectivity
        let result = sqlx::query!("SELECT 1 as test")
            .fetch_one(&self.pool)
            .await
            .map_err(DevErpError::Database)?;

        if result.test == Some(1) {
            Ok("Database connection successful".to_string())
        } else {
            Err(DevErpError::Database(sqlx::Error::RowNotFound))
        }
    }

    pub async fn get_database_version(&self) -> Result<String, DevErpError> {
        let result = sqlx::query!("SELECT version() as version")
            .fetch_one(&self.pool)
            .await
            .map_err(DevErpError::Database)?;

        Ok(result.version.unwrap_or_else(|| "Unknown".to_string()))
    }

    pub async fn get_database_stats(&self) -> Result<DatabaseStats, DevErpError> {
        // Get table counts
        let project_count =
            sqlx::query!("SELECT COUNT(*) as count FROM projects WHERE deleted_at IS NULL")
                .fetch_one(&self.pool)
                .await?
                .count
                .unwrap_or(0);

        let task_count =
            sqlx::query!("SELECT COUNT(*) as count FROM tasks WHERE deleted_at IS NULL")
                .fetch_one(&self.pool)
                .await?
                .count
                .unwrap_or(0);

        let resource_count =
            sqlx::query!("SELECT COUNT(*) as count FROM resources WHERE deleted_at IS NULL")
                .fetch_one(&self.pool)
                .await?
                .count
                .unwrap_or(0);

        let timeline_count =
            sqlx::query!("SELECT COUNT(*) as count FROM timelines WHERE deleted_at IS NULL")
                .fetch_one(&self.pool)
                .await?
                .count
                .unwrap_or(0);

        Ok(DatabaseStats {
            project_count,
            task_count,
            resource_count,
            timeline_count,
        })
    }

    fn validate_value(&self, value: &str, data_type: &ConfigDataType) -> Result<(), DevErpError> {
        match data_type {
            ConfigDataType::Integer => {
                value.parse::<i64>().map_err(|_| {
                    DevErpError::Validation(format!("Value '{}' is not a valid integer", value))
                })?;
            }
            ConfigDataType::Boolean => {
                value.parse::<bool>().map_err(|_| {
                    DevErpError::Validation(format!(
                        "Value '{}' is not a valid boolean (use 'true' or 'false')",
                        value
                    ))
                })?;
            }
            ConfigDataType::Json => {
                serde_json::from_str::<serde_json::Value>(value).map_err(|_| {
                    DevErpError::Validation(format!("Value '{}' is not valid JSON", value))
                })?;
            }
            ConfigDataType::String => {
                // String type accepts any value
            }
        }
        Ok(())
    }
}

#[derive(Debug)]
pub struct DatabaseStats {
    pub project_count: i64,
    pub task_count: i64,
    pub resource_count: i64,
    pub timeline_count: i64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::config::entity::CreateConfiguration;
    use mockall::mock;
    use mockall::predicate::*;

    mock! {
        ConfigRepo {}

        #[async_trait::async_trait]
        impl ConfigRepository for ConfigRepo {
            async fn create(&self, config: CreateConfiguration) -> Result<Configuration, DevErpError>;
            async fn find_by_key(&self, key: &str) -> Result<Option<Configuration>, DevErpError>;
            async fn find_all(&self) -> Result<Vec<Configuration>, DevErpError>;
            async fn update(&self, config: UpdateConfiguration) -> Result<Configuration, DevErpError>;
            async fn delete(&self, key: &str) -> Result<bool, DevErpError>;
            async fn reset_to_defaults(&self) -> Result<(), DevErpError>;
        }
    }

    #[tokio::test]
    async fn test_validate_value_integer_valid() {
        let service = create_test_service().await;
        assert!(service
            .validate_value("42", &ConfigDataType::Integer)
            .is_ok());
    }

    #[tokio::test]
    async fn test_validate_value_integer_invalid() {
        let service = create_test_service().await;
        assert!(service
            .validate_value("not a number", &ConfigDataType::Integer)
            .is_err());
    }

    #[tokio::test]
    async fn test_validate_value_boolean_valid() {
        let service = create_test_service().await;
        assert!(service
            .validate_value("true", &ConfigDataType::Boolean)
            .is_ok());
        assert!(service
            .validate_value("false", &ConfigDataType::Boolean)
            .is_ok());
    }

    #[tokio::test]
    async fn test_validate_value_boolean_invalid() {
        let service = create_test_service().await;
        assert!(service
            .validate_value("yes", &ConfigDataType::Boolean)
            .is_err());
    }

    #[tokio::test]
    async fn test_validate_value_json_valid() {
        let service = create_test_service().await;
        assert!(service
            .validate_value(r#"{"key": "value"}"#, &ConfigDataType::Json)
            .is_ok());
    }

    #[tokio::test]
    async fn test_validate_value_json_invalid() {
        let service = create_test_service().await;
        assert!(service
            .validate_value("not json", &ConfigDataType::Json)
            .is_err());
    }

    #[tokio::test]
    async fn test_validate_value_string_always_valid() {
        let service = create_test_service().await;
        assert!(service
            .validate_value("any value", &ConfigDataType::String)
            .is_ok());
    }

    async fn create_test_service() -> ConfigService {
        let mock_repo = MockConfigRepo::new();
        // Create a dummy pool with a test database URL
        // Using connect_lazy means it won't actually connect until first query
        let database_url = std::env::var("DATABASE_URL")
            .unwrap_or_else(|_| "postgres://localhost/test".to_string());
        let pool = PgPool::connect_lazy(&database_url).expect("Failed to create pool");
        ConfigService::new(Arc::new(mock_repo), pool)
    }
}
