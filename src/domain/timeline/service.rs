// Timeline service with business logic

use std::sync::Arc;
use tracing::{debug, info};

use crate::utils::error::DevErpError;
use super::entity::{
    CreateTimeline, Timeline, TimelineFilter, UpdateTimeline,
    CreateMilestone, Milestone, MilestoneFilter, UpdateMilestone,
};
use super::repository::{TimelineRepository, MilestoneRepository};

/// Timeline service containing business logic
///
/// This service layer sits between the CLI/API layer and the repository layer.
/// It handles:
/// - Business rule validation
/// - Cross-entity operations
/// - Timeline and milestone management
/// - Domain logic enforcement
pub struct TimelineService {
    timeline_repository: Arc<dyn TimelineRepository>,
    milestone_repository: Arc<dyn MilestoneRepository>,
}

impl TimelineService {
    /// Create a new timeline service
    ///
    /// # Arguments
    /// * `timeline_repository` - The timeline repository implementation
    /// * `milestone_repository` - The milestone repository implementation
    pub fn new(
        timeline_repository: Arc<dyn TimelineRepository>,
        milestone_repository: Arc<dyn MilestoneRepository>,
    ) -> Self {
        Self {
            timeline_repository,
            milestone_repository,
        }
    }

    // ========== Timeline Operations ==========

    /// Create a new timeline
    ///
    /// # Arguments
    /// * `input` - Timeline creation data
    ///
    /// # Returns
    /// * `Ok(Timeline)` - The created timeline
    /// * `Err(DevErpError)` - Validation or database error
    ///
    /// # Business Rules
    /// - Timeline name must not be empty
    /// - End date must be after or equal to start date
    pub async fn create_timeline(&self, input: CreateTimeline) -> Result<Timeline, DevErpError> {
        debug!("Service: Creating timeline '{}' for project {}", input.name, input.project_id);

        let timeline = self.timeline_repository.create(input).await?;

        info!(timeline_id = %timeline.id, timeline_name = %timeline.name, "Timeline created");

        Ok(timeline)
    }

    /// Get a timeline by ID
    ///
    /// # Arguments
    /// * `id` - The timeline ID
    ///
    /// # Returns
    /// * `Ok(Timeline)` - The timeline if found
    /// * `Err(DevErpError::NotFound)` - If timeline doesn't exist
    pub async fn get_timeline(&self, id: i64) -> Result<Timeline, DevErpError> {
        debug!("Service: Getting timeline by id {}", id);

        self.timeline_repository
            .find_by_id(id)
            .await?
            .ok_or_else(|| DevErpError::NotFound(format!("Timeline with id {} not found", id)))
    }

    /// List timelines with filtering and pagination
    ///
    /// # Arguments
    /// * `filter` - Filter and pagination options
    ///
    /// # Returns
    /// * `Ok(Vec<Timeline>)` - List of timelines matching the filter
    pub async fn list_timelines(&self, filter: TimelineFilter) -> Result<Vec<Timeline>, DevErpError> {
        debug!("Service: Listing timelines with filter: {:?}", filter);

        self.timeline_repository.find_all(filter).await
    }

    /// Get all timelines for a project
    ///
    /// # Arguments
    /// * `project_id` - The project ID
    ///
    /// # Returns
    /// * `Ok(Vec<Timeline>)` - List of timelines for the project
    pub async fn get_timelines_by_project(&self, project_id: i64) -> Result<Vec<Timeline>, DevErpError> {
        debug!("Service: Getting timelines for project {}", project_id);

        self.timeline_repository.find_by_project(project_id).await
    }

    /// Count timelines matching filter criteria
    ///
    /// # Arguments
    /// * `filter` - Filter options
    ///
    /// # Returns
    /// * `Ok(i64)` - Number of matching timelines
    pub async fn count_timelines(&self, filter: TimelineFilter) -> Result<i64, DevErpError> {
        debug!("Service: Counting timelines with filter: {:?}", filter);

        self.timeline_repository.count(filter).await
    }

    /// Update an existing timeline
    ///
    /// # Arguments
    /// * `input` - Timeline update data
    ///
    /// # Returns
    /// * `Ok(Timeline)` - The updated timeline
    /// * `Err(DevErpError)` - Validation or database error
    ///
    /// # Business Rules
    /// - Timeline must exist
    /// - If both start_date and end_date are updated, end_date must be after start_date
    pub async fn update_timeline(&self, input: UpdateTimeline) -> Result<Timeline, DevErpError> {
        debug!("Service: Updating timeline {}", input.id);

        // Additional business logic can be added here
        // For example, check if timeline is already completed and prevent updates

        let timeline = self.timeline_repository.update(input).await?;

        info!(timeline_id = %timeline.id, "Timeline updated");

        Ok(timeline)
    }

    /// Delete a timeline (soft delete)
    ///
    /// # Arguments
    /// * `id` - The timeline ID
    ///
    /// # Returns
    /// * `Ok(())` - Timeline deleted successfully
    /// * `Err(DevErpError::NotFound)` - If timeline doesn't exist
    ///
    /// # Business Rules
    /// - Soft deletes the timeline and all associated milestones
    pub async fn delete_timeline(&self, id: i64) -> Result<(), DevErpError> {
        debug!("Service: Deleting timeline {}", id);

        let deleted = self.timeline_repository.soft_delete(id).await?;

        if !deleted {
            return Err(DevErpError::NotFound(format!("Timeline with id {} not found", id)));
        }

        info!(timeline_id = %id, "Timeline deleted");

        Ok(())
    }

    /// Restore a soft-deleted timeline
    ///
    /// # Arguments
    /// * `id` - The timeline ID
    ///
    /// # Returns
    /// * `Ok(())` - Timeline restored successfully
    /// * `Err(DevErpError::NotFound)` - If timeline doesn't exist or not deleted
    pub async fn restore_timeline(&self, id: i64) -> Result<(), DevErpError> {
        debug!("Service: Restoring timeline {}", id);

        let restored = self.timeline_repository.restore(id).await?;

        if !restored {
            return Err(DevErpError::NotFound(format!("Timeline with id {} not found or not deleted", id)));
        }

        info!(timeline_id = %id, "Timeline restored");

        Ok(())
    }

    // ========== Milestone Operations ==========

    /// Create a new milestone
    ///
    /// # Arguments
    /// * `input` - Milestone creation data
    ///
    /// # Returns
    /// * `Ok(Milestone)` - The created milestone
    /// * `Err(DevErpError)` - Validation or database error
    ///
    /// # Business Rules
    /// - Milestone name must not be empty
    /// - Timeline and project must exist
    /// - Target date should ideally be within the timeline's date range (warning if not)
    pub async fn create_milestone(&self, input: CreateMilestone) -> Result<Milestone, DevErpError> {
        debug!("Service: Creating milestone '{}' for timeline {}", input.name, input.timeline_id);

        // Verify timeline exists
        let timeline = self.get_timeline(input.timeline_id).await?;

        // Optional: Warn if target date is outside timeline range
        if input.target_date < timeline.start_date || input.target_date > timeline.end_date {
            debug!(
                "Warning: Milestone target date {} is outside timeline range {}-{}",
                input.target_date, timeline.start_date, timeline.end_date
            );
        }

        let milestone = self.milestone_repository.create(input).await?;

        info!(milestone_id = %milestone.id, milestone_name = %milestone.name, "Milestone created");

        Ok(milestone)
    }

    /// Get a milestone by ID
    ///
    /// # Arguments
    /// * `id` - The milestone ID
    ///
    /// # Returns
    /// * `Ok(Milestone)` - The milestone if found
    /// * `Err(DevErpError::NotFound)` - If milestone doesn't exist
    pub async fn get_milestone(&self, id: i64) -> Result<Milestone, DevErpError> {
        debug!("Service: Getting milestone by id {}", id);

        self.milestone_repository
            .find_by_id(id)
            .await?
            .ok_or_else(|| DevErpError::NotFound(format!("Milestone with id {} not found", id)))
    }

    /// List milestones with filtering and pagination
    ///
    /// # Arguments
    /// * `filter` - Filter and pagination options
    ///
    /// # Returns
    /// * `Ok(Vec<Milestone>)` - List of milestones matching the filter
    pub async fn list_milestones(&self, filter: MilestoneFilter) -> Result<Vec<Milestone>, DevErpError> {
        debug!("Service: Listing milestones with filter: {:?}", filter);

        self.milestone_repository.find_all(filter).await
    }

    /// Get all milestones for a timeline
    ///
    /// # Arguments
    /// * `timeline_id` - The timeline ID
    ///
    /// # Returns
    /// * `Ok(Vec<Milestone>)` - List of milestones for the timeline
    pub async fn get_milestones_by_timeline(&self, timeline_id: i64) -> Result<Vec<Milestone>, DevErpError> {
        debug!("Service: Getting milestones for timeline {}", timeline_id);

        self.milestone_repository.find_by_timeline(timeline_id).await
    }

    /// Get all milestones for a project
    ///
    /// # Arguments
    /// * `project_id` - The project ID
    ///
    /// # Returns
    /// * `Ok(Vec<Milestone>)` - List of milestones for the project
    pub async fn get_milestones_by_project(&self, project_id: i64) -> Result<Vec<Milestone>, DevErpError> {
        debug!("Service: Getting milestones for project {}", project_id);

        self.milestone_repository.find_by_project(project_id).await
    }

    /// Count milestones matching filter criteria
    ///
    /// # Arguments
    /// * `filter` - Filter options
    ///
    /// # Returns
    /// * `Ok(i64)` - Number of matching milestones
    pub async fn count_milestones(&self, filter: MilestoneFilter) -> Result<i64, DevErpError> {
        debug!("Service: Counting milestones with filter: {:?}", filter);

        self.milestone_repository.count(filter).await
    }

    /// Update an existing milestone
    ///
    /// # Arguments
    /// * `input` - Milestone update data
    ///
    /// # Returns
    /// * `Ok(Milestone)` - The updated milestone
    /// * `Err(DevErpError)` - Validation or database error
    pub async fn update_milestone(&self, input: UpdateMilestone) -> Result<Milestone, DevErpError> {
        debug!("Service: Updating milestone {}", input.id);

        let milestone = self.milestone_repository.update(input).await?;

        info!(milestone_id = %milestone.id, "Milestone updated");

        Ok(milestone)
    }

    /// Delete a milestone (soft delete)
    ///
    /// # Arguments
    /// * `id` - The milestone ID
    ///
    /// # Returns
    /// * `Ok(())` - Milestone deleted successfully
    /// * `Err(DevErpError::NotFound)` - If milestone doesn't exist
    pub async fn delete_milestone(&self, id: i64) -> Result<(), DevErpError> {
        debug!("Service: Deleting milestone {}", id);

        let deleted = self.milestone_repository.soft_delete(id).await?;

        if !deleted {
            return Err(DevErpError::NotFound(format!("Milestone with id {} not found", id)));
        }

        info!(milestone_id = %id, "Milestone deleted");

        Ok(())
    }

    /// Restore a soft-deleted milestone
    ///
    /// # Arguments
    /// * `id` - The milestone ID
    ///
    /// # Returns
    /// * `Ok(())` - Milestone restored successfully
    /// * `Err(DevErpError::NotFound)` - If milestone doesn't exist or not deleted
    pub async fn restore_milestone(&self, id: i64) -> Result<(), DevErpError> {
        debug!("Service: Restoring milestone {}", id);

        let restored = self.milestone_repository.restore(id).await?;

        if !restored {
            return Err(DevErpError::NotFound(format!("Milestone with id {} not found or not deleted", id)));
        }

        info!(milestone_id = %id, "Milestone restored");

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;
    use crate::domain::timeline::entity::{TimelineType, TimelineStatus, MilestoneStatus};
    use chrono::{NaiveDate, Utc};
    use mockall::mock;
    use mockall::predicate::*;

    mock! {
        pub TimelineRepo {}

        #[async_trait]
        impl TimelineRepository for TimelineRepo {
            async fn create(&self, timeline: CreateTimeline) -> Result<Timeline, DevErpError>;
            async fn find_by_id(&self, id: i64) -> Result<Option<Timeline>, DevErpError>;
            async fn find_all(&self, filter: TimelineFilter) -> Result<Vec<Timeline>, DevErpError>;
            async fn find_by_project(&self, project_id: i64) -> Result<Vec<Timeline>, DevErpError>;
            async fn count(&self, filter: TimelineFilter) -> Result<i64, DevErpError>;
            async fn update(&self, timeline: UpdateTimeline) -> Result<Timeline, DevErpError>;
            async fn soft_delete(&self, id: i64) -> Result<bool, DevErpError>;
            async fn delete(&self, id: i64) -> Result<bool, DevErpError>;
            async fn restore(&self, id: i64) -> Result<bool, DevErpError>;
        }
    }

    mock! {
        pub MilestoneRepo {}

        #[async_trait]
        impl MilestoneRepository for MilestoneRepo {
            async fn create(&self, milestone: CreateMilestone) -> Result<Milestone, DevErpError>;
            async fn find_by_id(&self, id: i64) -> Result<Option<Milestone>, DevErpError>;
            async fn find_all(&self, filter: MilestoneFilter) -> Result<Vec<Milestone>, DevErpError>;
            async fn find_by_timeline(&self, timeline_id: i64) -> Result<Vec<Milestone>, DevErpError>;
            async fn find_by_project(&self, project_id: i64) -> Result<Vec<Milestone>, DevErpError>;
            async fn count(&self, filter: MilestoneFilter) -> Result<i64, DevErpError>;
            async fn update(&self, milestone: UpdateMilestone) -> Result<Milestone, DevErpError>;
            async fn soft_delete(&self, id: i64) -> Result<bool, DevErpError>;
            async fn delete(&self, id: i64) -> Result<bool, DevErpError>;
            async fn restore(&self, id: i64) -> Result<bool, DevErpError>;
        }
    }

    fn create_test_timeline(id: i64, project_id: i64, name: &str) -> Timeline {
        Timeline {
            id,
            project_id,
            name: name.to_string(),
            description: Some("Test timeline".to_string()),
            timeline_type: TimelineType::Sprint,
            start_date: NaiveDate::from_ymd_opt(2025, 1, 1).unwrap(),
            end_date: NaiveDate::from_ymd_opt(2025, 1, 14).unwrap(),
            status: TimelineStatus::Active,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            deleted_at: None,
        }
    }

    fn create_test_milestone(id: i64, timeline_id: i64, project_id: i64, name: &str) -> Milestone {
        Milestone {
            id,
            timeline_id,
            project_id,
            name: name.to_string(),
            description: Some("Test milestone".to_string()),
            target_date: NaiveDate::from_ymd_opt(2025, 1, 7).unwrap(),
            actual_date: None,
            status: MilestoneStatus::Pending,
            completion_percentage: 0,
            metadata: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            deleted_at: None,
        }
    }

    #[tokio::test]
    async fn test_create_timeline_success() {
        let mut mock_timeline_repo = MockTimelineRepo::new();
        let mock_milestone_repo = MockMilestoneRepo::new();

        mock_timeline_repo
            .expect_create()
            .times(1)
            .returning(move |_| Ok(create_test_timeline(1, 1, "Sprint 1")));

        let service = TimelineService::new(
            Arc::new(mock_timeline_repo),
            Arc::new(mock_milestone_repo),
        );

        let input = CreateTimeline {
            project_id: 1,
            name: "Sprint 1".to_string(),
            description: Some("Test sprint".to_string()),
            timeline_type: Some(TimelineType::Sprint),
            start_date: NaiveDate::from_ymd_opt(2025, 1, 1).unwrap(),
            end_date: NaiveDate::from_ymd_opt(2025, 1, 14).unwrap(),
            status: Some(TimelineStatus::Active),
        };

        let result = service.create_timeline(input).await;
        assert!(result.is_ok());
        let timeline = result.unwrap();
        assert_eq!(timeline.name, "Sprint 1");
        assert_eq!(timeline.project_id, 1);
    }

    #[tokio::test]
    async fn test_get_timeline_success() {
        let mut mock_timeline_repo = MockTimelineRepo::new();
        let mock_milestone_repo = MockMilestoneRepo::new();

        mock_timeline_repo
            .expect_find_by_id()
            .with(eq(1))
            .times(1)
            .returning(move |_| Ok(Some(create_test_timeline(1, 1, "Sprint 1"))));

        let service = TimelineService::new(
            Arc::new(mock_timeline_repo),
            Arc::new(mock_milestone_repo),
        );

        let result = service.get_timeline(1).await;
        assert!(result.is_ok());
        let timeline = result.unwrap();
        assert_eq!(timeline.name, "Sprint 1");
    }

    #[tokio::test]
    async fn test_get_timeline_not_found() {
        let mut mock_timeline_repo = MockTimelineRepo::new();
        let mock_milestone_repo = MockMilestoneRepo::new();

        mock_timeline_repo
            .expect_find_by_id()
            .with(eq(999))
            .times(1)
            .returning(|_| Ok(None));

        let service = TimelineService::new(
            Arc::new(mock_timeline_repo),
            Arc::new(mock_milestone_repo),
        );

        let result = service.get_timeline(999).await;
        assert!(result.is_err());
        match result.unwrap_err() {
            DevErpError::NotFound(msg) => {
                assert!(msg.contains("not found"));
            }
            _ => panic!("Expected not found error"),
        }
    }

    #[tokio::test]
    async fn test_create_milestone_success() {
        let mut mock_timeline_repo = MockTimelineRepo::new();
        let mut mock_milestone_repo = MockMilestoneRepo::new();

        // Mock timeline lookup for validation
        mock_timeline_repo
            .expect_find_by_id()
            .with(eq(1))
            .times(1)
            .returning(move |_| Ok(Some(create_test_timeline(1, 1, "Sprint 1"))));

        mock_milestone_repo
            .expect_create()
            .times(1)
            .returning(move |_| Ok(create_test_milestone(1, 1, 1, "Feature Complete")));

        let service = TimelineService::new(
            Arc::new(mock_timeline_repo),
            Arc::new(mock_milestone_repo),
        );

        let input = CreateMilestone {
            timeline_id: 1,
            project_id: 1,
            name: "Feature Complete".to_string(),
            description: Some("All features implemented".to_string()),
            target_date: NaiveDate::from_ymd_opt(2025, 1, 7).unwrap(),
            status: Some(MilestoneStatus::Pending),
            completion_percentage: Some(0),
            metadata: None,
        };

        let result = service.create_milestone(input).await;
        assert!(result.is_ok());
        let milestone = result.unwrap();
        assert_eq!(milestone.name, "Feature Complete");
        assert_eq!(milestone.timeline_id, 1);
    }

    #[tokio::test]
    async fn test_create_milestone_timeline_not_found() {
        let mut mock_timeline_repo = MockTimelineRepo::new();
        let mock_milestone_repo = MockMilestoneRepo::new();

        mock_timeline_repo
            .expect_find_by_id()
            .with(eq(999))
            .times(1)
            .returning(|_| Ok(None));

        let service = TimelineService::new(
            Arc::new(mock_timeline_repo),
            Arc::new(mock_milestone_repo),
        );

        let input = CreateMilestone {
            timeline_id: 999,
            project_id: 1,
            name: "Feature Complete".to_string(),
            description: None,
            target_date: NaiveDate::from_ymd_opt(2025, 1, 7).unwrap(),
            status: None,
            completion_percentage: None,
            metadata: None,
        };

        let result = service.create_milestone(input).await;
        assert!(result.is_err());
        match result.unwrap_err() {
            DevErpError::NotFound(msg) => {
                assert!(msg.contains("Timeline"));
            }
            _ => panic!("Expected not found error"),
        }
    }

    #[tokio::test]
    async fn test_delete_timeline_success() {
        let mut mock_timeline_repo = MockTimelineRepo::new();
        let mock_milestone_repo = MockMilestoneRepo::new();

        mock_timeline_repo
            .expect_soft_delete()
            .with(eq(1))
            .times(1)
            .returning(|_| Ok(true));

        let service = TimelineService::new(
            Arc::new(mock_timeline_repo),
            Arc::new(mock_milestone_repo),
        );

        let result = service.delete_timeline(1).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_delete_timeline_not_found() {
        let mut mock_timeline_repo = MockTimelineRepo::new();
        let mock_milestone_repo = MockMilestoneRepo::new();

        mock_timeline_repo
            .expect_soft_delete()
            .with(eq(999))
            .times(1)
            .returning(|_| Ok(false));

        let service = TimelineService::new(
            Arc::new(mock_timeline_repo),
            Arc::new(mock_milestone_repo),
        );

        let result = service.delete_timeline(999).await;
        assert!(result.is_err());
        match result.unwrap_err() {
            DevErpError::NotFound(msg) => {
                assert!(msg.contains("not found"));
            }
            _ => panic!("Expected not found error"),
        }
    }

    #[tokio::test]
    async fn test_get_milestones_by_timeline() {
        let mock_timeline_repo = MockTimelineRepo::new();
        let mut mock_milestone_repo = MockMilestoneRepo::new();

        mock_milestone_repo
            .expect_find_by_timeline()
            .with(eq(1))
            .times(1)
            .returning(|_| {
                Ok(vec![
                    create_test_milestone(1, 1, 1, "M1"),
                    create_test_milestone(2, 1, 1, "M2"),
                ])
            });

        let service = TimelineService::new(
            Arc::new(mock_timeline_repo),
            Arc::new(mock_milestone_repo),
        );

        let result = service.get_milestones_by_timeline(1).await;
        assert!(result.is_ok());
        let milestones = result.unwrap();
        assert_eq!(milestones.len(), 2);
    }
}
