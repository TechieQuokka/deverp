use async_trait::async_trait;
use sqlx::PgPool;

use crate::domain::config::entity::{
    ConfigDataType, Configuration, CreateConfiguration, UpdateConfiguration,
};
use crate::domain::config::repository::ConfigRepository;
use crate::utils::error::DevErpError;

pub struct PostgresConfigRepository {
    pool: PgPool,
}

impl PostgresConfigRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ConfigRepository for PostgresConfigRepository {
    async fn create(&self, config: CreateConfiguration) -> Result<Configuration, DevErpError> {
        let result = sqlx::query_as!(
            Configuration,
            r#"
            INSERT INTO configurations (config_key, config_value, description, data_type, is_encrypted, is_required)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING
                id,
                config_key,
                config_value,
                description,
                data_type as "data_type!: ConfigDataType",
                is_encrypted as "is_encrypted!: bool",
                is_required as "is_required!: bool",
                created_at,
                updated_at
            "#,
            config.config_key,
            config.config_value,
            config.description,
            config.data_type as ConfigDataType,
            config.is_encrypted,
            config.is_required
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(result)
    }

    async fn find_by_key(&self, key: &str) -> Result<Option<Configuration>, DevErpError> {
        let result = sqlx::query_as!(
            Configuration,
            r#"
            SELECT
                id,
                config_key,
                config_value,
                description,
                data_type as "data_type!: ConfigDataType",
                is_encrypted as "is_encrypted!: bool",
                is_required as "is_required!: bool",
                created_at,
                updated_at
            FROM configurations
            WHERE config_key = $1
            "#,
            key
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(result)
    }

    async fn find_all(&self) -> Result<Vec<Configuration>, DevErpError> {
        let results = sqlx::query_as!(
            Configuration,
            r#"
            SELECT
                id,
                config_key,
                config_value,
                description,
                data_type as "data_type!: ConfigDataType",
                is_encrypted as "is_encrypted!: bool",
                is_required as "is_required!: bool",
                created_at,
                updated_at
            FROM configurations
            ORDER BY config_key ASC
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(results)
    }

    async fn update(&self, config: UpdateConfiguration) -> Result<Configuration, DevErpError> {
        let result = sqlx::query_as!(
            Configuration,
            r#"
            UPDATE configurations
            SET
                config_value = $2,
                description = COALESCE($3, description),
                updated_at = CURRENT_TIMESTAMP
            WHERE config_key = $1
            RETURNING
                id,
                config_key,
                config_value,
                description,
                data_type as "data_type!: ConfigDataType",
                is_encrypted as "is_encrypted!: bool",
                is_required as "is_required!: bool",
                created_at,
                updated_at
            "#,
            config.config_key,
            config.config_value,
            config.description
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(result)
    }

    async fn delete(&self, key: &str) -> Result<bool, DevErpError> {
        let result = sqlx::query!(
            r#"
            DELETE FROM configurations
            WHERE config_key = $1
            "#,
            key
        )
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected() > 0)
    }

    async fn reset_to_defaults(&self) -> Result<(), DevErpError> {
        // Begin transaction
        let mut tx = self.pool.begin().await?;

        // Delete all existing configurations
        sqlx::query!("DELETE FROM configurations")
            .execute(&mut *tx)
            .await?;

        // Insert default configurations
        sqlx::query!(
            r#"
            INSERT INTO configurations (config_key, config_value, description, data_type) VALUES
            ('default_project_status', 'planning', 'Default status for new projects', 'string'),
            ('default_task_status', 'todo', 'Default status for new tasks', 'string'),
            ('date_format', '%Y-%m-%d', 'Default date format', 'string'),
            ('enable_audit_log', 'true', 'Enable audit logging', 'boolean')
            "#
        )
        .execute(&mut *tx)
        .await?;

        // Commit transaction
        tx.commit().await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_config_repository_mock() {
        // This is a placeholder for actual integration tests with testcontainers
        // Real tests would require a PostgreSQL instance
        assert!(true);
    }
}
