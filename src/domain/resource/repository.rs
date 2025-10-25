use async_trait::async_trait;
use uuid::Uuid;

use crate::utils::error::DevErpError;

use super::entity::{
    CreateResource, LinkResourceToProject, ProjectResource, Resource, ResourceFilter,
    ResourceUsageStats, UpdateProjectResource, UpdateResource,
};

/// Repository trait for Resource operations
#[async_trait]
pub trait ResourceRepository: Send + Sync {
    /// Create a new resource
    async fn create(&self, resource: CreateResource) -> Result<Resource, DevErpError>;

    /// Find resource by ID
    async fn find_by_id(&self, id: i64) -> Result<Option<Resource>, DevErpError>;

    /// Find resource by UUID
    async fn find_by_uuid(&self, uuid: Uuid) -> Result<Option<Resource>, DevErpError>;

    /// Find all resources matching the filter
    async fn find_all(&self, filter: ResourceFilter) -> Result<Vec<Resource>, DevErpError>;

    /// Update an existing resource
    async fn update(&self, resource: UpdateResource) -> Result<Resource, DevErpError>;

    /// Soft delete a resource
    async fn soft_delete(&self, id: i64) -> Result<bool, DevErpError>;

    /// Hard delete a resource (use with caution)
    async fn delete(&self, id: i64) -> Result<bool, DevErpError>;

    /// Link a resource to a project
    async fn link_to_project(
        &self,
        link: LinkResourceToProject,
    ) -> Result<ProjectResource, DevErpError>;

    /// Unlink a resource from a project (soft delete)
    async fn unlink_from_project(
        &self,
        project_id: i64,
        resource_id: i64,
    ) -> Result<bool, DevErpError>;

    /// Update project-resource link
    async fn update_project_resource(
        &self,
        update: UpdateProjectResource,
    ) -> Result<ProjectResource, DevErpError>;

    /// Find all resources linked to a project
    async fn find_by_project_id(&self, project_id: i64) -> Result<Vec<Resource>, DevErpError>;

    /// Find all projects using a resource
    async fn find_projects_using_resource(&self, resource_id: i64)
        -> Result<Vec<i64>, DevErpError>;

    /// Get resource usage statistics
    async fn get_usage_stats(&self, resource_id: i64) -> Result<ResourceUsageStats, DevErpError>;

    /// Get usage statistics for all resources
    async fn get_all_usage_stats(&self) -> Result<Vec<ResourceUsageStats>, DevErpError>;
}
