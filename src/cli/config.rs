// Configuration CLI commands

use std::sync::Arc;
use crate::Result;
use super::commands::{ConfigCommand, OutputFormat};
use crate::config::settings::Settings;
use crate::infrastructure::database;

use crate::domain::config::service::ConfigService;
use crate::infrastructure::repositories::config_repo::PostgresConfigRepository;
use crate::utils::formatter::{table_header, table_row};
use crate::utils::error::DevErpError;

/// Handle config commands
pub async fn handle(command: ConfigCommand, _format: OutputFormat) -> Result<()> {
    // Establish database connection
    let settings = Settings::default();
    let pool = database::establish_connection(&settings.database).await?;

    // Create repository and service
    let repo = Arc::new(PostgresConfigRepository::new(pool.clone()));
    let service = ConfigService::new(repo, pool);

    match command {
        ConfigCommand::Show { key } => handle_show(service, key).await,
        ConfigCommand::Set {
            key,
            value,
            description,
        } => handle_set(service, key, value, description).await,
        ConfigCommand::Reset { confirm } => handle_reset(service, confirm).await,
        ConfigCommand::TestDb { verbose } => handle_test_db(service, verbose).await,
    }
}

async fn handle_show(
    service: ConfigService,
    key: Option<String>,
) -> Result<()> {
    match key {
        Some(k) => {
            // Show single configuration
            let config = service.get_config(&k).await?;

            table_header(&["Key", "Value", "Type", "Description"]);
            table_row(&[
                config.config_key,
                config.config_value,
                config.data_type.to_string(),
                config.description.unwrap_or_else(|| "-".to_string()),
            ]);
        }
        None => {
            // Show all configurations
            let configs = service.get_all_configs().await?;

            table_header(&["Key", "Value", "Type", "Required", "Description"]);
            for config in configs {
                table_row(&[
                    config.config_key,
                    config.config_value,
                    config.data_type.to_string(),
                    if config.is_required { "Yes".to_string() } else { "No".to_string() },
                    config.description.unwrap_or_else(|| "-".to_string()),
                ]);
            }
        }
    }

    Ok(())
}

async fn handle_set(
    service: ConfigService,
    key: String,
    value: String,
    description: Option<String>,
) -> Result<()> {
    let config = service.set_config(&key, value, description).await?;

    println!("Configuration updated successfully:");
    println!("  Key: {}", config.config_key);
    println!("  Value: {}", config.config_value);
    println!("  Type: {}", config.data_type);

    Ok(())
}

async fn handle_reset(service: ConfigService, confirm: bool) -> Result<()> {
    if !confirm {
        return Err(DevErpError::Validation(
            "Reset operation requires --confirm flag to prevent accidental data loss".to_string()
        ).into());
    }

    service.reset_to_defaults().await?;
    println!("All configurations have been reset to default values");

    Ok(())
}

async fn handle_test_db(service: ConfigService, verbose: bool) -> Result<()> {
    // Test basic connectivity
    let connection_result = service.test_database_connection().await?;
    println!("✓ {}", connection_result);

    if verbose {
        // Get database version
        let version = service.get_database_version().await?;
        println!("\nDatabase Information:");
        println!("  Version: {}", version);

        // Get database statistics
        let stats = service.get_database_stats().await?;
        println!("\nDatabase Statistics:");
        println!("  Projects: {}", stats.project_count);
        println!("  Tasks: {}", stats.task_count);
        println!("  Resources: {}", stats.resource_count);
        println!("  Timelines: {}", stats.timeline_count);
    }

    Ok(())
}
