// Timeline repository trait

use async_trait::async_trait;

use crate::utils::error::DevErpError;
use super::entity::{
    CreateTimeline, Timeline, TimelineFilter, UpdateTimeline,
    CreateMilestone, Milestone, MilestoneFilter, UpdateMilestone,
};

/// Repository trait for timeline data access
///
/// This trait defines the interface for all timeline-related database operations.
#[async_trait]
pub trait TimelineRepository: Send + Sync {
    /// Create a new timeline
    ///
    /// # Arguments
    /// * `timeline` - The timeline creation data
    ///
    /// # Returns
    /// * `Ok(Timeline)` - The created timeline with generated ID
    /// * `Err(DevErpError)` - Database error or validation error
    async fn create(&self, timeline: CreateTimeline) -> Result<Timeline, DevErpError>;

    /// Find a timeline by its internal ID
    ///
    /// # Arguments
    /// * `id` - The internal timeline ID
    ///
    /// # Returns
    /// * `Ok(Some(Timeline))` - Timeline found
    /// * `Ok(None)` - Timeline not found or soft deleted
    /// * `Err(DevErpError)` - Database error
    async fn find_by_id(&self, id: i64) -> Result<Option<Timeline>, DevErpError>;

    /// Find all timelines matching the filter criteria
    ///
    /// # Arguments
    /// * `filter` - Filter and pagination options
    ///
    /// # Returns
    /// * `Ok(Vec<Timeline>)` - List of matching timelines (may be empty)
    /// * `Err(DevErpError)` - Database error
    async fn find_all(&self, filter: TimelineFilter) -> Result<Vec<Timeline>, DevErpError>;

    /// Find all timelines for a specific project
    ///
    /// # Arguments
    /// * `project_id` - The project ID
    ///
    /// # Returns
    /// * `Ok(Vec<Timeline>)` - List of timelines for the project
    /// * `Err(DevErpError)` - Database error
    async fn find_by_project(&self, project_id: i64) -> Result<Vec<Timeline>, DevErpError>;

    /// Count timelines matching the filter criteria
    ///
    /// # Arguments
    /// * `filter` - Filter options (pagination is ignored)
    ///
    /// # Returns
    /// * `Ok(i64)` - Number of matching timelines
    /// * `Err(DevErpError)` - Database error
    async fn count(&self, filter: TimelineFilter) -> Result<i64, DevErpError>;

    /// Update an existing timeline
    ///
    /// # Arguments
    /// * `timeline` - The timeline update data with ID
    ///
    /// # Returns
    /// * `Ok(Timeline)` - The updated timeline
    /// * `Err(DevErpError)` - Database error or validation error
    async fn update(&self, timeline: UpdateTimeline) -> Result<Timeline, DevErpError>;

    /// Soft delete a timeline by setting deleted_at timestamp
    ///
    /// # Arguments
    /// * `id` - The internal timeline ID
    ///
    /// # Returns
    /// * `Ok(bool)` - true if timeline was deleted, false if not found
    /// * `Err(DevErpError)` - Database error
    async fn soft_delete(&self, id: i64) -> Result<bool, DevErpError>;

    /// Hard delete a timeline (permanently removes from database)
    ///
    /// # Arguments
    /// * `id` - The internal timeline ID
    ///
    /// # Returns
    /// * `Ok(bool)` - true if timeline was deleted, false if not found
    /// * `Err(DevErpError)` - Database error
    async fn delete(&self, id: i64) -> Result<bool, DevErpError>;

    /// Restore a soft-deleted timeline
    ///
    /// # Arguments
    /// * `id` - The internal timeline ID
    ///
    /// # Returns
    /// * `Ok(bool)` - true if timeline was restored, false if not found
    /// * `Err(DevErpError)` - Database error
    async fn restore(&self, id: i64) -> Result<bool, DevErpError>;
}

/// Repository trait for milestone data access
///
/// This trait defines the interface for all milestone-related database operations.
#[async_trait]
pub trait MilestoneRepository: Send + Sync {
    /// Create a new milestone
    ///
    /// # Arguments
    /// * `milestone` - The milestone creation data
    ///
    /// # Returns
    /// * `Ok(Milestone)` - The created milestone with generated ID
    /// * `Err(DevErpError)` - Database error or validation error
    async fn create(&self, milestone: CreateMilestone) -> Result<Milestone, DevErpError>;

    /// Find a milestone by its internal ID
    ///
    /// # Arguments
    /// * `id` - The internal milestone ID
    ///
    /// # Returns
    /// * `Ok(Some(Milestone))` - Milestone found
    /// * `Ok(None)` - Milestone not found or soft deleted
    /// * `Err(DevErpError)` - Database error
    async fn find_by_id(&self, id: i64) -> Result<Option<Milestone>, DevErpError>;

    /// Find all milestones matching the filter criteria
    ///
    /// # Arguments
    /// * `filter` - Filter and pagination options
    ///
    /// # Returns
    /// * `Ok(Vec<Milestone>)` - List of matching milestones (may be empty)
    /// * `Err(DevErpError)` - Database error
    async fn find_all(&self, filter: MilestoneFilter) -> Result<Vec<Milestone>, DevErpError>;

    /// Find all milestones for a specific timeline
    ///
    /// # Arguments
    /// * `timeline_id` - The timeline ID
    ///
    /// # Returns
    /// * `Ok(Vec<Milestone>)` - List of milestones for the timeline
    /// * `Err(DevErpError)` - Database error
    async fn find_by_timeline(&self, timeline_id: i64) -> Result<Vec<Milestone>, DevErpError>;

    /// Find all milestones for a specific project
    ///
    /// # Arguments
    /// * `project_id` - The project ID
    ///
    /// # Returns
    /// * `Ok(Vec<Milestone>)` - List of milestones for the project
    /// * `Err(DevErpError)` - Database error
    async fn find_by_project(&self, project_id: i64) -> Result<Vec<Milestone>, DevErpError>;

    /// Count milestones matching the filter criteria
    ///
    /// # Arguments
    /// * `filter` - Filter options (pagination is ignored)
    ///
    /// # Returns
    /// * `Ok(i64)` - Number of matching milestones
    /// * `Err(DevErpError)` - Database error
    async fn count(&self, filter: MilestoneFilter) -> Result<i64, DevErpError>;

    /// Update an existing milestone
    ///
    /// # Arguments
    /// * `milestone` - The milestone update data with ID
    ///
    /// # Returns
    /// * `Ok(Milestone)` - The updated milestone
    /// * `Err(DevErpError)` - Database error or validation error
    async fn update(&self, milestone: UpdateMilestone) -> Result<Milestone, DevErpError>;

    /// Soft delete a milestone by setting deleted_at timestamp
    ///
    /// # Arguments
    /// * `id` - The internal milestone ID
    ///
    /// # Returns
    /// * `Ok(bool)` - true if milestone was deleted, false if not found
    /// * `Err(DevErpError)` - Database error
    async fn soft_delete(&self, id: i64) -> Result<bool, DevErpError>;

    /// Hard delete a milestone (permanently removes from database)
    ///
    /// # Arguments
    /// * `id` - The internal milestone ID
    ///
    /// # Returns
    /// * `Ok(bool)` - true if milestone was deleted, false if not found
    /// * `Err(DevErpError)` - Database error
    async fn delete(&self, id: i64) -> Result<bool, DevErpError>;

    /// Restore a soft-deleted milestone
    ///
    /// # Arguments
    /// * `id` - The internal milestone ID
    ///
    /// # Returns
    /// * `Ok(bool)` - true if milestone was restored, false if not found
    /// * `Err(DevErpError)` - Database error
    async fn restore(&self, id: i64) -> Result<bool, DevErpError>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_timeline_repository_trait_is_object_safe() {
        fn _assert_object_safe(_: &dyn TimelineRepository) {}
    }

    #[test]
    fn test_milestone_repository_trait_is_object_safe() {
        fn _assert_object_safe(_: &dyn MilestoneRepository) {}
    }
}
