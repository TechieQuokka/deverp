use sqlx::{PgPool, postgres::PgPoolOptions};
use deverp::utils::error::DevErpError;

/// Creates a test database pool
///
/// This function expects a DATABASE_URL environment variable to be set
/// for testing purposes. Use a separate test database to avoid conflicts.
pub async fn create_test_pool() -> Result<PgPool, DevErpError> {
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://deverp_user:2147483647@localhost:5432/deverp_test".to_string());

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .map_err(|e| DevErpError::Database(e))?;

    Ok(pool)
}

/// Runs all migrations on the test database
pub async fn run_migrations(pool: &PgPool) -> Result<(), DevErpError> {
    sqlx::migrate!("./migrations")
        .run(pool)
        .await
        .map_err(|e| DevErpError::Config(format!("Migration error: {}", e)))?;

    Ok(())
}

/// Cleans up all data from test tables
///
/// This is useful for ensuring tests start with a clean slate
pub async fn cleanup_database(pool: &PgPool) -> Result<(), DevErpError> {
    // Use sqlx::query instead of query! for TRUNCATE to avoid offline mode issues
    sqlx::query("TRUNCATE TABLE task_comments CASCADE")
        .execute(pool)
        .await?;

    sqlx::query("TRUNCATE TABLE task_dependencies CASCADE")
        .execute(pool)
        .await?;

    sqlx::query("TRUNCATE TABLE tasks CASCADE")
        .execute(pool)
        .await?;

    sqlx::query("TRUNCATE TABLE project_resources CASCADE")
        .execute(pool)
        .await?;

    sqlx::query("TRUNCATE TABLE resources CASCADE")
        .execute(pool)
        .await?;

    sqlx::query("TRUNCATE TABLE milestones CASCADE")
        .execute(pool)
        .await?;

    sqlx::query("TRUNCATE TABLE timelines CASCADE")
        .execute(pool)
        .await?;

    sqlx::query("TRUNCATE TABLE project_tags CASCADE")
        .execute(pool)
        .await?;

    sqlx::query("TRUNCATE TABLE projects CASCADE")
        .execute(pool)
        .await?;

    Ok(())
}

/// Sets up a fresh test database with migrations
pub async fn setup_test_database() -> Result<PgPool, DevErpError> {
    let pool = create_test_pool().await?;
    run_migrations(&pool).await?;
    cleanup_database(&pool).await?;
    Ok(pool)
}
