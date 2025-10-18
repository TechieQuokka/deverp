// Connection pool management

use sqlx::PgPool;
use crate::Result;

pub struct ConnectionPool {
    pool: PgPool,
}

impl ConnectionPool {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub fn get_pool(&self) -> &PgPool {
        &self.pool
    }

    pub async fn health_check(&self) -> Result<()> {
        sqlx::query("SELECT 1")
            .execute(&self.pool)
            .await?;

        Ok(())
    }
}
