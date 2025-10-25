use std::sync::Arc;
use tracing::{info, warn};
use uuid::Uuid;

use crate::utils::error::DevErpError;

use super::{
    entity::{
        CreateResource, LinkResourceToProject, ProjectResource, Resource, ResourceFilter,
        ResourceUsageStats, UpdateProjectResource, UpdateResource,
    },
    repository::ResourceRepository,
};

/// Service for resource management business logic
pub struct ResourceService {
    repository: Arc<dyn ResourceRepository>,
}

impl ResourceService {
    /// Create a new ResourceService with the given repository
    pub fn new(repository: Arc<dyn ResourceRepository>) -> Self {
        Self { repository }
    }

    /// Create a new resource with validation
    pub async fn create_resource(&self, input: CreateResource) -> Result<Resource, DevErpError> {
        // Validate input
        if input.name.trim().is_empty() {
            return Err(DevErpError::Validation(
                "Resource name cannot be empty".to_string(),
            ));
        }

        if let Some(ref url) = input.url {
            if !url.is_empty() && !Self::is_valid_url(url) {
                return Err(DevErpError::Validation(format!(
                    "Invalid URL format: {}",
                    url
                )));
            }
        }

        if let Some(ref doc_url) = input.documentation_url {
            if !doc_url.is_empty() && !Self::is_valid_url(doc_url) {
                return Err(DevErpError::Validation(format!(
                    "Invalid documentation URL format: {}",
                    doc_url
                )));
            }
        }

        let resource = self.repository.create(input).await?;
        info!(resource_id = %resource.id, resource_name = %resource.name, "Created new resource");

        Ok(resource)
    }

    /// Get resource by ID
    pub async fn get_resource(&self, id: i64) -> Result<Resource, DevErpError> {
        self.repository
            .find_by_id(id)
            .await?
            .ok_or_else(|| DevErpError::NotFound(format!("Resource with id {} not found", id)))
    }

    /// Get resource by UUID
    pub async fn get_resource_by_uuid(&self, uuid: Uuid) -> Result<Resource, DevErpError> {
        self.repository
            .find_by_uuid(uuid)
            .await?
            .ok_or_else(|| DevErpError::NotFound(format!("Resource with uuid {} not found", uuid)))
    }

    /// List resources with optional filtering
    pub async fn list_resources(
        &self,
        filter: ResourceFilter,
    ) -> Result<Vec<Resource>, DevErpError> {
        self.repository.find_all(filter).await
    }

    /// Update a resource
    pub async fn update_resource(&self, input: UpdateResource) -> Result<Resource, DevErpError> {
        // Validate the resource exists
        let _existing = self.get_resource(input.id).await?;

        // Validate input
        if let Some(ref name) = input.name {
            if name.trim().is_empty() {
                return Err(DevErpError::Validation(
                    "Resource name cannot be empty".to_string(),
                ));
            }
        }

        if let Some(ref url) = input.url {
            if !url.is_empty() && !Self::is_valid_url(url) {
                return Err(DevErpError::Validation(format!(
                    "Invalid URL format: {}",
                    url
                )));
            }
        }

        if let Some(ref doc_url) = input.documentation_url {
            if !doc_url.is_empty() && !Self::is_valid_url(doc_url) {
                return Err(DevErpError::Validation(format!(
                    "Invalid documentation URL format: {}",
                    doc_url
                )));
            }
        }

        let resource = self.repository.update(input).await?;
        info!(
            resource_id = %resource.id,
            resource_name = %resource.name,
            "Updated resource"
        );

        Ok(resource)
    }

    /// Delete a resource (soft delete)
    pub async fn delete_resource(&self, id: i64) -> Result<(), DevErpError> {
        // Check if resource exists
        let resource = self.get_resource(id).await?;

        // Check if resource is linked to any active projects
        let projects = self.repository.find_projects_using_resource(id).await?;
        if !projects.is_empty() {
            warn!(
                resource_id = %id,
                project_count = %projects.len(),
                "Deleting resource that is linked to {} projects",
                projects.len()
            );
        }

        let deleted = self.repository.soft_delete(id).await?;
        if !deleted {
            return Err(DevErpError::NotFound(format!(
                "Failed to delete resource with id {}",
                id
            )));
        }

        info!(resource_id = %id, resource_name = %resource.name, "Deleted resource");
        Ok(())
    }

    /// Link a resource to a project
    pub async fn link_resource_to_project(
        &self,
        input: LinkResourceToProject,
    ) -> Result<ProjectResource, DevErpError> {
        // Validate that both project and resource exist
        // Note: In a real implementation, we'd check if the project exists
        // For now, we rely on foreign key constraints in the database

        let link = self.repository.link_to_project(input.clone()).await?;
        info!(
            project_id = %input.project_id,
            resource_id = %input.resource_id,
            "Linked resource to project"
        );

        Ok(link)
    }

    /// Unlink a resource from a project
    pub async fn unlink_resource_from_project(
        &self,
        project_id: i64,
        resource_id: i64,
    ) -> Result<(), DevErpError> {
        let unlinked = self
            .repository
            .unlink_from_project(project_id, resource_id)
            .await?;

        if !unlinked {
            return Err(DevErpError::NotFound(format!(
                "Link between project {} and resource {} not found",
                project_id, resource_id
            )));
        }

        info!(
            project_id = %project_id,
            resource_id = %resource_id,
            "Unlinked resource from project"
        );

        Ok(())
    }

    /// Update project-resource link
    pub async fn update_project_resource(
        &self,
        input: UpdateProjectResource,
    ) -> Result<ProjectResource, DevErpError> {
        let link = self
            .repository
            .update_project_resource(input.clone())
            .await?;
        info!(
            project_id = %input.project_id,
            resource_id = %input.resource_id,
            "Updated project-resource link"
        );

        Ok(link)
    }

    /// Get all resources for a project
    pub async fn get_project_resources(
        &self,
        project_id: i64,
    ) -> Result<Vec<Resource>, DevErpError> {
        self.repository.find_by_project_id(project_id).await
    }

    /// Get resource usage statistics
    pub async fn get_resource_usage(
        &self,
        resource_id: i64,
    ) -> Result<ResourceUsageStats, DevErpError> {
        self.repository.get_usage_stats(resource_id).await
    }

    /// Get usage statistics for all resources
    pub async fn get_all_resource_usage(&self) -> Result<Vec<ResourceUsageStats>, DevErpError> {
        self.repository.get_all_usage_stats().await
    }

    /// Analyze resource utilization
    pub async fn analyze_resource_utilization(
        &self,
    ) -> Result<Vec<ResourceUsageStats>, DevErpError> {
        let stats = self.get_all_resource_usage().await?;

        // Sort by total projects descending
        let mut sorted_stats = stats;
        sorted_stats.sort_by(|a, b| b.total_projects.cmp(&a.total_projects));

        Ok(sorted_stats)
    }

    /// Basic URL validation
    fn is_valid_url(url: &str) -> bool {
        url.starts_with("http://") || url.starts_with("https://") || url.starts_with("file://")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::resource::entity::{ResourceStatus, ResourceType};
    use async_trait::async_trait;
    use chrono::Utc;
    use mockall::mock;
    use mockall::predicate::*;

    mock! {
        pub ResourceRepo {}

        #[async_trait]
        impl ResourceRepository for ResourceRepo {
            async fn create(&self, resource: CreateResource) -> Result<Resource, DevErpError>;
            async fn find_by_id(&self, id: i64) -> Result<Option<Resource>, DevErpError>;
            async fn find_by_uuid(&self, uuid: Uuid) -> Result<Option<Resource>, DevErpError>;
            async fn find_all(&self, filter: ResourceFilter) -> Result<Vec<Resource>, DevErpError>;
            async fn update(&self, resource: UpdateResource) -> Result<Resource, DevErpError>;
            async fn soft_delete(&self, id: i64) -> Result<bool, DevErpError>;
            async fn delete(&self, id: i64) -> Result<bool, DevErpError>;
            async fn link_to_project(&self, link: LinkResourceToProject) -> Result<ProjectResource, DevErpError>;
            async fn unlink_from_project(&self, project_id: i64, resource_id: i64) -> Result<bool, DevErpError>;
            async fn update_project_resource(&self, update: UpdateProjectResource) -> Result<ProjectResource, DevErpError>;
            async fn find_by_project_id(&self, project_id: i64) -> Result<Vec<Resource>, DevErpError>;
            async fn find_projects_using_resource(&self, resource_id: i64) -> Result<Vec<i64>, DevErpError>;
            async fn get_usage_stats(&self, resource_id: i64) -> Result<ResourceUsageStats, DevErpError>;
            async fn get_all_usage_stats(&self) -> Result<Vec<ResourceUsageStats>, DevErpError>;
        }
    }

    fn create_test_resource(id: i64, name: &str) -> Resource {
        Resource {
            id,
            uuid: Uuid::new_v4(),
            name: name.to_string(),
            description: Some("Test resource".to_string()),
            resource_type: ResourceType::Library,
            version: Some("1.0.0".to_string()),
            url: Some("https://example.com".to_string()),
            documentation_url: Some("https://docs.example.com".to_string()),
            license: Some("MIT".to_string()),
            status: Some(ResourceStatus::Active),
            metadata: None,
            tags: Some(vec!["test".to_string()]),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            deleted_at: None,
        }
    }

    #[test]
    fn test_is_valid_url() {
        assert!(ResourceService::is_valid_url("http://example.com"));
        assert!(ResourceService::is_valid_url("https://example.com"));
        assert!(ResourceService::is_valid_url("file:///path/to/file"));
        assert!(!ResourceService::is_valid_url("example.com"));
        assert!(!ResourceService::is_valid_url("ftp://example.com"));
    }

    #[tokio::test]
    async fn test_create_resource_success() {
        let mut mock_repo = MockResourceRepo::new();

        mock_repo
            .expect_create()
            .times(1)
            .returning(move |_| Ok(create_test_resource(1, "Test Library")));

        let service = ResourceService::new(Arc::new(mock_repo));

        let input = CreateResource {
            name: "Test Library".to_string(),
            description: Some("Test resource".to_string()),
            resource_type: ResourceType::Library,
            version: Some("1.0.0".to_string()),
            url: Some("https://example.com".to_string()),
            documentation_url: Some("https://docs.example.com".to_string()),
            license: Some("MIT".to_string()),
            status: Some(ResourceStatus::Active),
            metadata: None,
            tags: Some(vec!["test".to_string()]),
        };

        let result = service.create_resource(input).await;
        assert!(result.is_ok());
        let resource = result.unwrap();
        assert_eq!(resource.name, "Test Library");
    }

    #[tokio::test]
    async fn test_create_resource_validation_empty_name() {
        let mock_repo = MockResourceRepo::new();
        let service = ResourceService::new(Arc::new(mock_repo));

        let input = CreateResource {
            name: "   ".to_string(), // Empty name after trim
            description: Some("Test".to_string()),
            resource_type: ResourceType::Library,
            version: None,
            url: None,
            documentation_url: None,
            license: None,
            status: None,
            metadata: None,
            tags: None,
        };

        let result = service.create_resource(input).await;
        assert!(result.is_err());
        match result.unwrap_err() {
            DevErpError::Validation(msg) => {
                assert!(msg.contains("name cannot be empty"));
            }
            _ => panic!("Expected validation error"),
        }
    }

    #[tokio::test]
    async fn test_create_resource_validation_invalid_url() {
        let mock_repo = MockResourceRepo::new();
        let service = ResourceService::new(Arc::new(mock_repo));

        let input = CreateResource {
            name: "Test Resource".to_string(),
            description: None,
            resource_type: ResourceType::Library,
            version: None,
            url: Some("invalid-url".to_string()),
            documentation_url: None,
            license: None,
            status: None,
            metadata: None,
            tags: None,
        };

        let result = service.create_resource(input).await;
        assert!(result.is_err());
        match result.unwrap_err() {
            DevErpError::Validation(msg) => {
                assert!(msg.contains("Invalid URL format"));
            }
            _ => panic!("Expected validation error"),
        }
    }

    #[tokio::test]
    async fn test_get_resource_success() {
        let mut mock_repo = MockResourceRepo::new();

        mock_repo
            .expect_find_by_id()
            .with(eq(1))
            .times(1)
            .returning(move |_| Ok(Some(create_test_resource(1, "Test Library"))));

        let service = ResourceService::new(Arc::new(mock_repo));

        let result = service.get_resource(1).await;
        assert!(result.is_ok());
        let resource = result.unwrap();
        assert_eq!(resource.name, "Test Library");
    }

    #[tokio::test]
    async fn test_get_resource_not_found() {
        let mut mock_repo = MockResourceRepo::new();

        mock_repo
            .expect_find_by_id()
            .with(eq(999))
            .times(1)
            .returning(|_| Ok(None));

        let service = ResourceService::new(Arc::new(mock_repo));

        let result = service.get_resource(999).await;
        assert!(result.is_err());
        match result.unwrap_err() {
            DevErpError::NotFound(msg) => {
                assert!(msg.contains("not found"));
            }
            _ => panic!("Expected not found error"),
        }
    }

    #[tokio::test]
    async fn test_delete_resource_with_projects() {
        let mut mock_repo = MockResourceRepo::new();

        mock_repo
            .expect_find_by_id()
            .with(eq(1))
            .times(1)
            .returning(move |_| Ok(Some(create_test_resource(1, "Test Library"))));

        mock_repo
            .expect_find_projects_using_resource()
            .with(eq(1))
            .times(1)
            .returning(|_| Ok(vec![1, 2, 3])); // Resource is used by 3 projects

        mock_repo
            .expect_soft_delete()
            .with(eq(1))
            .times(1)
            .returning(|_| Ok(true));

        let service = ResourceService::new(Arc::new(mock_repo));

        let result = service.delete_resource(1).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_link_resource_to_project() {
        let mut mock_repo = MockResourceRepo::new();

        mock_repo
            .expect_link_to_project()
            .times(1)
            .returning(|link| {
                Ok(ProjectResource {
                    project_id: link.project_id,
                    resource_id: link.resource_id,
                    usage_notes: link.usage_notes,
                    version_used: link.version_used,
                    is_critical: Some(link.is_critical.unwrap_or(false)),
                    added_at: Utc::now(),
                    removed_at: None,
                })
            });

        let service = ResourceService::new(Arc::new(mock_repo));

        let input = LinkResourceToProject {
            project_id: 1,
            resource_id: 1,
            usage_notes: Some("Used for testing".to_string()),
            version_used: Some("1.0.0".to_string()),
            is_critical: Some(true),
        };

        let result = service.link_resource_to_project(input).await;
        assert!(result.is_ok());
        let link = result.unwrap();
        assert_eq!(link.project_id, 1);
        assert_eq!(link.resource_id, 1);
        assert_eq!(link.is_critical, Some(true));
    }
}
