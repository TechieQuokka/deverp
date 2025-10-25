// Project repository trait

use async_trait::async_trait;
use uuid::Uuid;

use super::entity::{CreateProject, Project, ProjectFilter, UpdateProject};
use crate::utils::error::DevErpError;

/// Repository trait for project data access
///
/// This trait defines the interface for all project-related database operations.
/// Implementations should handle database-specific details while maintaining
/// the contract defined here.
#[async_trait]
pub trait ProjectRepository: Send + Sync {
    /// Create a new project
    ///
    /// # Arguments
    /// * `project` - The project creation data
    ///
    /// # Returns
    /// * `Ok(Project)` - The created project with generated ID and UUID
    /// * `Err(DevErpError)` - Database error or validation error
    ///
    /// # Errors
    /// * `DevErpError::Database` - Database connection or query error
    /// * `DevErpError::Conflict` - Project code already exists (if provided)
    /// * `DevErpError::Validation` - Invalid input data
    async fn create(&self, project: CreateProject) -> Result<Project, DevErpError>;

    /// Find a project by its internal ID
    ///
    /// # Arguments
    /// * `id` - The internal project ID
    ///
    /// # Returns
    /// * `Ok(Some(Project))` - Project found
    /// * `Ok(None)` - Project not found or soft deleted
    /// * `Err(DevErpError)` - Database error
    async fn find_by_id(&self, id: i64) -> Result<Option<Project>, DevErpError>;

    /// Find a project by its UUID
    ///
    /// # Arguments
    /// * `uuid` - The project UUID
    ///
    /// # Returns
    /// * `Ok(Some(Project))` - Project found
    /// * `Ok(None)` - Project not found or soft deleted
    /// * `Err(DevErpError)` - Database error
    async fn find_by_uuid(&self, uuid: Uuid) -> Result<Option<Project>, DevErpError>;

    /// Find a project by its code
    ///
    /// # Arguments
    /// * `code` - The project code (unique identifier)
    ///
    /// # Returns
    /// * `Ok(Some(Project))` - Project found
    /// * `Ok(None)` - Project not found or soft deleted
    /// * `Err(DevErpError)` - Database error
    async fn find_by_code(&self, code: &str) -> Result<Option<Project>, DevErpError>;

    /// Find all projects matching the filter criteria
    ///
    /// # Arguments
    /// * `filter` - Filter and pagination options
    ///
    /// # Returns
    /// * `Ok(Vec<Project>)` - List of matching projects (may be empty)
    /// * `Err(DevErpError)` - Database error
    async fn find_all(&self, filter: ProjectFilter) -> Result<Vec<Project>, DevErpError>;

    /// Count projects matching the filter criteria
    ///
    /// # Arguments
    /// * `filter` - Filter options (pagination is ignored)
    ///
    /// # Returns
    /// * `Ok(i64)` - Number of matching projects
    /// * `Err(DevErpError)` - Database error
    async fn count(&self, filter: ProjectFilter) -> Result<i64, DevErpError>;

    /// Update an existing project
    ///
    /// # Arguments
    /// * `project` - The project update data with ID
    ///
    /// # Returns
    /// * `Ok(Project)` - The updated project
    /// * `Err(DevErpError)` - Database error or validation error
    ///
    /// # Errors
    /// * `DevErpError::NotFound` - Project not found
    /// * `DevErpError::Database` - Database connection or query error
    /// * `DevErpError::Conflict` - Project code already exists (if changed)
    async fn update(&self, project: UpdateProject) -> Result<Project, DevErpError>;

    /// Soft delete a project by setting deleted_at timestamp
    ///
    /// # Arguments
    /// * `id` - The internal project ID
    ///
    /// # Returns
    /// * `Ok(bool)` - true if project was deleted, false if not found
    /// * `Err(DevErpError)` - Database error
    async fn soft_delete(&self, id: i64) -> Result<bool, DevErpError>;

    /// Hard delete a project (permanently removes from database)
    ///
    /// **WARNING**: This operation is irreversible. Use with caution.
    /// Typically, soft_delete should be used instead.
    ///
    /// # Arguments
    /// * `id` - The internal project ID
    ///
    /// # Returns
    /// * `Ok(bool)` - true if project was deleted, false if not found
    /// * `Err(DevErpError)` - Database error
    async fn delete(&self, id: i64) -> Result<bool, DevErpError>;

    /// Restore a soft-deleted project
    ///
    /// # Arguments
    /// * `id` - The internal project ID
    ///
    /// # Returns
    /// * `Ok(bool)` - true if project was restored, false if not found or not deleted
    /// * `Err(DevErpError)` - Database error
    async fn restore(&self, id: i64) -> Result<bool, DevErpError>;

    /// Check if a project code already exists
    ///
    /// # Arguments
    /// * `code` - The project code to check
    /// * `exclude_id` - Optional project ID to exclude from check (for updates)
    ///
    /// # Returns
    /// * `Ok(bool)` - true if code exists, false otherwise
    /// * `Err(DevErpError)` - Database error
    async fn code_exists(&self, code: &str, exclude_id: Option<i64>) -> Result<bool, DevErpError>;

    /// Get all projects with a specific tag
    ///
    /// # Arguments
    /// * `tag` - The tag to search for
    ///
    /// # Returns
    /// * `Ok(Vec<Project>)` - List of projects with the tag (may be empty)
    /// * `Err(DevErpError)` - Database error
    async fn find_by_tag(&self, tag: &str) -> Result<Vec<Project>, DevErpError>;
}

#[cfg(test)]
mod tests {
    use super::*;

    // Note: These are just type checking tests
    // Actual repository implementation tests should be in integration tests
    // using a real database (with testcontainers)

    #[test]
    fn test_repository_trait_is_object_safe() {
        // This test ensures the trait is object-safe (can be used as dyn ProjectRepository)
        fn _assert_object_safe(_: &dyn ProjectRepository) {}
    }
}
