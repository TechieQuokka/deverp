use async_trait::async_trait;

use crate::domain::config::entity::{Configuration, CreateConfiguration, UpdateConfiguration};
use crate::utils::error::DevErpError;

#[async_trait]
pub trait ConfigRepository: Send + Sync {
    async fn create(&self, config: CreateConfiguration) -> Result<Configuration, DevErpError>;

    async fn find_by_key(&self, key: &str) -> Result<Option<Configuration>, DevErpError>;

    async fn find_all(&self) -> Result<Vec<Configuration>, DevErpError>;

    async fn update(&self, config: UpdateConfiguration) -> Result<Configuration, DevErpError>;

    async fn delete(&self, key: &str) -> Result<bool, DevErpError>;

    async fn reset_to_defaults(&self) -> Result<(), DevErpError>;
}
