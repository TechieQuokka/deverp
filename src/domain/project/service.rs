// Project service with business logic

use std::sync::Arc;
use tracing::{debug, info};
use uuid::Uuid;

use crate::utils::error::DevErpError;
use super::entity::{CreateProject, Project, ProjectFilter, ProjectStatus, UpdateProject};
use super::repository::ProjectRepository;

/// Project service containing business logic
///
/// This service layer sits between the CLI/API layer and the repository layer.
/// It handles:
/// - Business rule validation
/// - Cross-entity operations
/// - Transaction coordination
/// - Domain logic enforcement
pub struct ProjectService {
    repository: Arc<dyn ProjectRepository>,
}

impl ProjectService {
    /// Create a new project service
    ///
    /// # Arguments
    /// * `repository` - The project repository implementation
    pub fn new(repository: Arc<dyn ProjectRepository>) -> Self {
        Self { repository }
    }

    /// Create a new project
    ///
    /// # Arguments
    /// * `input` - Project creation data
    ///
    /// # Returns
    /// * `Ok(Project)` - The created project
    /// * `Err(DevErpError)` - Validation or database error
    ///
    /// # Business Rules
    /// - Project name must not be empty
    /// - Project code must be unique if provided
    /// - End date must be after start date if both provided
    pub async fn create_project(&self, input: CreateProject) -> Result<Project, DevErpError> {
        debug!("Service: Creating project '{}'", input.name);

        // Validation is handled in entity and repository
        // Additional business logic can be added here

        let project = self.repository.create(input).await?;

        info!(project_id = %project.id, project_name = %project.name, "Project created");

        Ok(project)
    }

    /// Get a project by ID
    ///
    /// # Arguments
    /// * `id` - The project ID
    ///
    /// # Returns
    /// * `Ok(Project)` - The project if found
    /// * `Err(DevErpError::NotFound)` - If project doesn't exist
    pub async fn get_project(&self, id: i64) -> Result<Project, DevErpError> {
        debug!("Service: Getting project by id {}", id);

        self.repository
            .find_by_id(id)
            .await?
            .ok_or_else(|| DevErpError::NotFound(format!("Project with id {} not found", id)))
    }

    /// Get a project by UUID
    ///
    /// # Arguments
    /// * `uuid` - The project UUID
    ///
    /// # Returns
    /// * `Ok(Project)` - The project if found
    /// * `Err(DevErpError::NotFound)` - If project doesn't exist
    pub async fn get_project_by_uuid(&self, uuid: Uuid) -> Result<Project, DevErpError> {
        debug!("Service: Getting project by uuid {}", uuid);

        self.repository
            .find_by_uuid(uuid)
            .await?
            .ok_or_else(|| DevErpError::NotFound(format!("Project with uuid {} not found", uuid)))
    }

    /// Get a project by code
    ///
    /// # Arguments
    /// * `code` - The project code
    ///
    /// # Returns
    /// * `Ok(Project)` - The project if found
    /// * `Err(DevErpError::NotFound)` - If project doesn't exist
    pub async fn get_project_by_code(&self, code: &str) -> Result<Project, DevErpError> {
        debug!("Service: Getting project by code '{}'", code);

        self.repository
            .find_by_code(code)
            .await?
            .ok_or_else(|| DevErpError::NotFound(format!("Project with code '{}' not found", code)))
    }

    /// List projects with optional filtering and pagination
    ///
    /// # Arguments
    /// * `filter` - Filter and pagination options
    ///
    /// # Returns
    /// * `Ok(Vec<Project>)` - List of matching projects
    pub async fn list_projects(&self, filter: ProjectFilter) -> Result<Vec<Project>, DevErpError> {
        debug!("Service: Listing projects with filter: {:?}", filter);

        let projects = self.repository.find_all(filter).await?;

        debug!("Service: Found {} projects", projects.len());

        Ok(projects)
    }

    /// Count projects matching filter criteria
    ///
    /// # Arguments
    /// * `filter` - Filter options
    ///
    /// # Returns
    /// * `Ok(i64)` - Number of matching projects
    pub async fn count_projects(&self, filter: ProjectFilter) -> Result<i64, DevErpError> {
        debug!("Service: Counting projects with filter: {:?}", filter);

        let count = self.repository.count(filter).await?;

        debug!("Service: Project count: {}", count);

        Ok(count)
    }

    /// Update an existing project
    ///
    /// # Arguments
    /// * `input` - Project update data
    ///
    /// # Returns
    /// * `Ok(Project)` - The updated project
    /// * `Err(DevErpError)` - Validation or database error
    ///
    /// # Business Rules
    /// - Project must exist
    /// - If code is changed, new code must be unique
    /// - Progress percentage must be 0-100
    pub async fn update_project(&self, input: UpdateProject) -> Result<Project, DevErpError> {
        debug!("Service: Updating project id {}", input.id);

        // Additional business logic can be added here
        // For example, automatically update actual_start_date when status changes to Active

        let project = self.repository.update(input).await?;

        info!(project_id = %project.id, "Project updated");

        Ok(project)
    }

    /// Update project status
    ///
    /// This is a convenience method for status updates with business logic
    ///
    /// # Arguments
    /// * `id` - Project ID
    /// * `new_status` - New status to set
    ///
    /// # Returns
    /// * `Ok(Project)` - The updated project
    ///
    /// # Business Logic
    /// - When status changes to Active, set actual_start_date if not set
    /// - When status changes to Completed, set actual_end_date if not set
    pub async fn update_status(&self, id: i64, new_status: ProjectStatus) -> Result<Project, DevErpError> {
        debug!("Service: Updating project {} status to {:?}", id, new_status);

        // Get current project
        let current = self.get_project(id).await?;

        // Build update with business logic
        let mut update = UpdateProject {
            id,
            status: Some(new_status.clone()),
            ..Default::default()
        };

        // Apply business rules based on status change
        match new_status {
            ProjectStatus::Active => {
                // Set actual start date if not already set
                if current.actual_start_date.is_none() {
                    update.actual_start_date = Some(chrono::Utc::now().date_naive());
                    info!(project_id = %id, "Setting actual_start_date on status change to Active");
                }
            }
            ProjectStatus::Completed => {
                // Set actual end date if not already set
                if current.actual_end_date.is_none() {
                    update.actual_end_date = Some(chrono::Utc::now().date_naive());
                    info!(project_id = %id, "Setting actual_end_date on status change to Completed");
                }
                // Set progress to 100% if not already
                if current.progress_percentage.unwrap_or(0) < 100 {
                    update.progress_percentage = Some(100);
                    info!(project_id = %id, "Setting progress to 100% on status change to Completed");
                }
            }
            _ => {}
        }

        self.repository.update(update).await
    }

    /// Update project progress
    ///
    /// # Arguments
    /// * `id` - Project ID
    /// * `progress` - Progress percentage (0-100)
    ///
    /// # Returns
    /// * `Ok(Project)` - The updated project
    ///
    /// # Business Logic
    /// - Progress 100% automatically sets status to Completed if not already
    pub async fn update_progress(&self, id: i64, progress: i32) -> Result<Project, DevErpError> {
        if !(0..=100).contains(&progress) {
            return Err(DevErpError::Validation(
                "Progress must be between 0 and 100".to_string()
            ));
        }

        debug!("Service: Updating project {} progress to {}%", id, progress);

        let current = self.get_project(id).await?;

        let mut update = UpdateProject {
            id,
            progress_percentage: Some(progress),
            ..Default::default()
        };

        // If progress is 100% and status is not Completed, update status
        if progress == 100 && current.status != ProjectStatus::Completed {
            update.status = Some(ProjectStatus::Completed);
            update.actual_end_date = Some(chrono::Utc::now().date_naive());
            info!(project_id = %id, "Auto-completing project as progress reached 100%");
        }

        self.repository.update(update).await
    }

    /// Archive a project
    ///
    /// Archives a project by setting its status to Archived.
    /// This is different from soft delete - archived projects remain visible
    /// but marked as inactive.
    ///
    /// # Arguments
    /// * `id` - Project ID
    ///
    /// # Returns
    /// * `Ok(Project)` - The archived project
    pub async fn archive_project(&self, id: i64) -> Result<Project, DevErpError> {
        debug!("Service: Archiving project {}", id);

        let update = UpdateProject {
            id,
            status: Some(ProjectStatus::Archived),
            ..Default::default()
        };

        let project = self.repository.update(update).await?;

        info!(project_id = %id, "Project archived");

        Ok(project)
    }

    /// Delete a project (soft delete)
    ///
    /// # Arguments
    /// * `id` - Project ID
    ///
    /// # Returns
    /// * `Ok(bool)` - true if deleted, false if not found
    pub async fn delete_project(&self, id: i64) -> Result<bool, DevErpError> {
        debug!("Service: Soft deleting project {}", id);

        let deleted = self.repository.soft_delete(id).await?;

        if deleted {
            info!(project_id = %id, "Project soft deleted");
        }

        Ok(deleted)
    }

    /// Permanently delete a project (hard delete)
    ///
    /// **WARNING**: This permanently removes the project from the database.
    /// Use with extreme caution. Prefer soft_delete in most cases.
    ///
    /// # Arguments
    /// * `id` - Project ID
    ///
    /// # Returns
    /// * `Ok(bool)` - true if deleted, false if not found
    pub async fn permanently_delete_project(&self, id: i64) -> Result<bool, DevErpError> {
        debug!("Service: Permanently deleting project {} - THIS IS IRREVERSIBLE", id);

        let deleted = self.repository.delete(id).await?;

        if deleted {
            info!(project_id = %id, "Project permanently deleted");
        }

        Ok(deleted)
    }

    /// Restore a soft-deleted project
    ///
    /// # Arguments
    /// * `id` - Project ID
    ///
    /// # Returns
    /// * `Ok(bool)` - true if restored, false if not found or not deleted
    pub async fn restore_project(&self, id: i64) -> Result<bool, DevErpError> {
        debug!("Service: Restoring project {}", id);

        let restored = self.repository.restore(id).await?;

        if restored {
            info!(project_id = %id, "Project restored");
        }

        Ok(restored)
    }

    /// Get all projects with a specific tag
    ///
    /// # Arguments
    /// * `tag` - The tag to search for
    ///
    /// # Returns
    /// * `Ok(Vec<Project>)` - List of projects with the tag
    pub async fn get_projects_by_tag(&self, tag: &str) -> Result<Vec<Project>, DevErpError> {
        debug!("Service: Getting projects by tag '{}'", tag);

        let projects = self.repository.find_by_tag(tag).await?;

        debug!("Service: Found {} projects with tag '{}'", projects.len(), tag);

        Ok(projects)
    }

    /// Calculate project statistics
    ///
    /// Returns basic statistics about a project
    ///
    /// # Arguments
    /// * `id` - Project ID
    ///
    /// # Returns
    /// * `Ok(ProjectStats)` - Project statistics
    pub async fn get_project_stats(&self, id: i64) -> Result<ProjectStats, DevErpError> {
        let project = self.get_project(id).await?;

        // Calculate duration
        let duration_days = if let (Some(start), Some(end)) = (project.start_date, project.end_date) {
            Some((end - start).num_days())
        } else {
            None
        };

        // Calculate actual duration
        let actual_duration_days = if let (Some(start), Some(end)) =
            (project.actual_start_date, project.actual_end_date) {
            Some((end - start).num_days())
        } else {
            None
        };

        // Check if overdue (planned end date passed but not completed)
        let is_overdue = if let Some(end_date) = project.end_date {
            let today = chrono::Utc::now().date_naive();
            end_date < today && project.status != ProjectStatus::Completed
        } else {
            false
        };

        Ok(ProjectStats {
            project_id: project.id,
            project_name: project.name.clone(),
            status: project.status.clone(),
            progress_percentage: project.progress_percentage.unwrap_or(0),
            duration_days,
            actual_duration_days,
            is_overdue,
            days_overdue: if is_overdue && project.end_date.is_some() {
                let today = chrono::Utc::now().date_naive();
                Some((today - project.end_date.unwrap()).num_days())
            } else {
                None
            },
        })
    }
}

/// Project statistics
#[derive(Debug, Clone)]
pub struct ProjectStats {
    pub project_id: i64,
    pub project_name: String,
    pub status: ProjectStatus,
    pub progress_percentage: i32,
    pub duration_days: Option<i64>,
    pub actual_duration_days: Option<i64>,
    pub is_overdue: bool,
    pub days_overdue: Option<i64>,
}

// Default implementation for UpdateProject to support partial updates
impl Default for UpdateProject {
    fn default() -> Self {
        Self {
            id: 0,
            name: None,
            description: None,
            code: None,
            status: None,
            priority: None,
            start_date: None,
            end_date: None,
            actual_start_date: None,
            actual_end_date: None,
            progress_percentage: None,
            repository_url: None,
            repository_branch: None,
            tags: None,
            metadata: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockall::mock;
    use mockall::predicate::*;
    use chrono::NaiveDate;

    // Mock repository for testing
    mock! {
        pub ProjectRepo {}

        #[async_trait::async_trait]
        impl ProjectRepository for ProjectRepo {
            async fn create(&self, project: CreateProject) -> Result<Project, DevErpError>;
            async fn find_by_id(&self, id: i64) -> Result<Option<Project>, DevErpError>;
            async fn find_by_uuid(&self, uuid: Uuid) -> Result<Option<Project>, DevErpError>;
            async fn find_by_code(&self, code: &str) -> Result<Option<Project>, DevErpError>;
            async fn find_all(&self, filter: ProjectFilter) -> Result<Vec<Project>, DevErpError>;
            async fn count(&self, filter: ProjectFilter) -> Result<i64, DevErpError>;
            async fn update(&self, project: UpdateProject) -> Result<Project, DevErpError>;
            async fn soft_delete(&self, id: i64) -> Result<bool, DevErpError>;
            async fn delete(&self, id: i64) -> Result<bool, DevErpError>;
            async fn restore(&self, id: i64) -> Result<bool, DevErpError>;
            async fn code_exists(&self, code: &str, exclude_id: Option<i64>) -> Result<bool, DevErpError>;
            async fn find_by_tag(&self, tag: &str) -> Result<Vec<Project>, DevErpError>;
        }
    }

    fn create_test_project() -> Project {
        use crate::domain::project::entity::Priority;

        Project {
            id: 1,
            uuid: Uuid::new_v4(),
            name: "Test Project".to_string(),
            description: Some("Test Description".to_string()),
            code: Some("TEST-001".to_string()),
            status: ProjectStatus::Planning,
            priority: Priority::Medium,
            start_date: Some(NaiveDate::from_ymd_opt(2025, 1, 1).unwrap()),
            end_date: Some(NaiveDate::from_ymd_opt(2025, 12, 31).unwrap()),
            actual_start_date: None,
            actual_end_date: None,
            progress_percentage: Some(0),
            repository_url: None,
            repository_branch: Some("main".to_string()),
            tags: Some(vec!["test".to_string()]),
            metadata: None,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
            deleted_at: None,
        }
    }

    #[tokio::test]
    async fn test_get_project() {
        let mut mock_repo = MockProjectRepo::new();
        let test_project = create_test_project();
        let expected_id = test_project.id;

        mock_repo
            .expect_find_by_id()
            .with(eq(expected_id))
            .times(1)
            .returning(move |_| Ok(Some(create_test_project())));

        let service = ProjectService::new(Arc::new(mock_repo));
        let result = service.get_project(expected_id).await;

        assert!(result.is_ok());
        let project = result.unwrap();
        assert_eq!(project.id, expected_id);
        assert_eq!(project.name, "Test Project");
    }

    #[tokio::test]
    async fn test_get_project_not_found() {
        let mut mock_repo = MockProjectRepo::new();

        mock_repo
            .expect_find_by_id()
            .with(eq(999))
            .times(1)
            .returning(|_| Ok(None));

        let service = ProjectService::new(Arc::new(mock_repo));
        let result = service.get_project(999).await;

        assert!(result.is_err());
        match result {
            Err(DevErpError::NotFound(msg)) => {
                assert!(msg.contains("999"));
            }
            _ => panic!("Expected NotFound error"),
        }
    }

    #[tokio::test]
    async fn test_update_status_to_active() {
        let mut mock_repo = MockProjectRepo::new();
        let mut test_project = create_test_project();

        // First call: get current project
        mock_repo
            .expect_find_by_id()
            .times(1)
            .returning(move |_| {
                let mut p = create_test_project();
                p.actual_start_date = None;
                Ok(Some(p))
            });

        // Second call: update project
        test_project.status = ProjectStatus::Active;
        test_project.actual_start_date = Some(chrono::Utc::now().date_naive());

        mock_repo
            .expect_update()
            .times(1)
            .returning(move |_| {
                let mut p = create_test_project();
                p.status = ProjectStatus::Active;
                p.actual_start_date = Some(chrono::Utc::now().date_naive());
                Ok(p)
            });

        let service = ProjectService::new(Arc::new(mock_repo));
        let result = service.update_status(1, ProjectStatus::Active).await;

        assert!(result.is_ok());
        let project = result.unwrap();
        assert_eq!(project.status, ProjectStatus::Active);
        assert!(project.actual_start_date.is_some());
    }
}
