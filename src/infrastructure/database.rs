// Database connection management

use crate::{config::settings::DatabaseConfig, Result};
use sqlx::{postgres::PgPoolOptions, PgPool};

pub async fn establish_connection(config: &DatabaseConfig) -> Result<PgPool> {
    let pool = PgPoolOptions::new()
        .max_connections(config.max_connections)
        .connect(&config.database_url())
        .await?;

    Ok(pool)
}

pub async fn test_connection(pool: &PgPool) -> Result<()> {
    sqlx::query("SELECT 1").execute(pool).await?;

    Ok(())
}
