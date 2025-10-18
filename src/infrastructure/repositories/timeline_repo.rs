// PostgreSQL implementation of TimelineRepository and MilestoneRepository

use async_trait::async_trait;
use sqlx::PgPool;
use tracing::{debug, info};

use crate::domain::timeline::{
    entity::{
        CreateTimeline, Timeline, TimelineFilter, UpdateTimeline,
        CreateMilestone, Milestone, MilestoneFilter, UpdateMilestone,
        TimelineStatus, TimelineType, MilestoneStatus,
    },
    repository::{TimelineRepository, MilestoneRepository},
};
use crate::utils::error::DevErpError;

/// PostgreSQL implementation of the TimelineRepository trait
pub struct PostgresTimelineRepository {
    pool: PgPool,
}

impl PostgresTimelineRepository {
    /// Create a new PostgreSQL timeline repository
    ///
    /// # Arguments
    /// * `pool` - PostgreSQL connection pool
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl TimelineRepository for PostgresTimelineRepository {
    async fn create(&self, timeline: CreateTimeline) -> Result<Timeline, DevErpError> {
        // Validate input
        timeline.validate()
            .map_err(|e| DevErpError::Validation(e))?;

        debug!("Creating timeline: {} for project_id: {}", timeline.name, timeline.project_id);

        let result = sqlx::query_as!(
            Timeline,
            r#"
            INSERT INTO timelines (
                project_id, name, description, timeline_type,
                start_date, end_date, status
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING
                id, project_id, name, description,
                timeline_type as "timeline_type!: TimelineType",
                start_date, end_date,
                status as "status!: TimelineStatus",
                created_at, updated_at, deleted_at
            "#,
            timeline.project_id,
            timeline.name,
            timeline.description,
            timeline.timeline_type.unwrap_or(TimelineType::Project).as_str(),
            timeline.start_date,
            timeline.end_date,
            timeline.status.unwrap_or(TimelineStatus::Planned).as_str()
        )
        .fetch_one(&self.pool)
        .await?;

        info!(timeline_id = %result.id, project_id = %result.project_id, "Timeline created successfully");

        Ok(result)
    }

    async fn find_by_id(&self, id: i64) -> Result<Option<Timeline>, DevErpError> {
        debug!("Finding timeline by id: {}", id);

        let result = sqlx::query_as!(
            Timeline,
            r#"
            SELECT
                id, project_id, name, description,
                timeline_type as "timeline_type!: TimelineType",
                start_date, end_date,
                status as "status!: TimelineStatus",
                created_at, updated_at, deleted_at
            FROM timelines
            WHERE id = $1 AND deleted_at IS NULL
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(result)
    }

    async fn find_all(&self, filter: TimelineFilter) -> Result<Vec<Timeline>, DevErpError> {
        debug!("Finding timelines with filter: {:?}", filter);

        // Fetch all non-deleted timelines
        let mut results = sqlx::query_as!(
            Timeline,
            r#"
            SELECT
                id, project_id, name, description,
                timeline_type as "timeline_type!: TimelineType",
                start_date, end_date,
                status as "status!: TimelineStatus",
                created_at, updated_at, deleted_at
            FROM timelines
            WHERE deleted_at IS NULL
            ORDER BY start_date DESC
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        // Apply filters in memory
        if let Some(project_id) = filter.project_id {
            results.retain(|t| t.project_id == project_id);
        }

        if let Some(ref timeline_type) = filter.timeline_type {
            results.retain(|t| &t.timeline_type == timeline_type);
        }

        if let Some(ref status) = filter.status {
            results.retain(|t| &t.status == status);
        }

        // Apply pagination
        let offset = filter.get_offset() as usize;
        let limit = filter.get_limit() as usize;
        let total = results.len();

        if offset < total {
            let end = (offset + limit).min(total);
            results = results[offset..end].to_vec();
        } else {
            results.clear();
        }

        Ok(results)
    }

    async fn find_by_project(&self, project_id: i64) -> Result<Vec<Timeline>, DevErpError> {
        debug!("Finding timelines for project_id: {}", project_id);

        let results = sqlx::query_as!(
            Timeline,
            r#"
            SELECT
                id, project_id, name, description,
                timeline_type as "timeline_type!: TimelineType",
                start_date, end_date,
                status as "status!: TimelineStatus",
                created_at, updated_at, deleted_at
            FROM timelines
            WHERE project_id = $1 AND deleted_at IS NULL
            ORDER BY start_date DESC
            "#,
            project_id
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(results)
    }

    async fn count(&self, filter: TimelineFilter) -> Result<i64, DevErpError> {
        debug!("Counting timelines with filter: {:?}", filter);

        // For simplicity, reuse find_all and count in memory
        // This is acceptable for a CLI application with limited data
        let timelines = self.find_all(TimelineFilter {
            project_id: filter.project_id,
            timeline_type: filter.timeline_type,
            status: filter.status,
            offset: None,
            limit: None,
        }).await?;

        Ok(timelines.len() as i64)
    }

    async fn update(&self, timeline: UpdateTimeline) -> Result<Timeline, DevErpError> {
        // Validate input
        timeline.validate()
            .map_err(|e| DevErpError::Validation(e))?;

        debug!("Updating timeline id: {}", timeline.id);

        // First, fetch the existing timeline
        let _existing = self.find_by_id(timeline.id).await?
            .ok_or_else(|| DevErpError::NotFound(format!("Timeline with id {} not found", timeline.id)))?;

        // Update fields
        let result = sqlx::query_as!(
            Timeline,
            r#"
            UPDATE timelines
            SET
                name = COALESCE($2, name),
                description = COALESCE($3, description),
                timeline_type = COALESCE($4, timeline_type),
                start_date = COALESCE($5, start_date),
                end_date = COALESCE($6, end_date),
                status = COALESCE($7, status),
                updated_at = NOW()
            WHERE id = $1 AND deleted_at IS NULL
            RETURNING
                id, project_id, name, description,
                timeline_type as "timeline_type!: TimelineType",
                start_date, end_date,
                status as "status!: TimelineStatus",
                created_at, updated_at, deleted_at
            "#,
            timeline.id,
            timeline.name,
            timeline.description,
            timeline.timeline_type.map(|t| t.as_str().to_string()),
            timeline.start_date,
            timeline.end_date,
            timeline.status.map(|s| s.as_str().to_string())
        )
        .fetch_one(&self.pool)
        .await?;

        info!(timeline_id = %result.id, "Timeline updated successfully");

        Ok(result)
    }

    async fn soft_delete(&self, id: i64) -> Result<bool, DevErpError> {
        debug!("Soft deleting timeline id: {}", id);

        let result = sqlx::query!(
            "UPDATE timelines SET deleted_at = NOW() WHERE id = $1 AND deleted_at IS NULL",
            id
        )
        .execute(&self.pool)
        .await?;

        let deleted = result.rows_affected() > 0;

        if deleted {
            info!(timeline_id = %id, "Timeline soft deleted successfully");
        }

        Ok(deleted)
    }

    async fn delete(&self, id: i64) -> Result<bool, DevErpError> {
        debug!("Hard deleting timeline id: {}", id);

        let result = sqlx::query!("DELETE FROM timelines WHERE id = $1", id)
            .execute(&self.pool)
            .await?;

        let deleted = result.rows_affected() > 0;

        if deleted {
            info!(timeline_id = %id, "Timeline hard deleted successfully");
        }

        Ok(deleted)
    }

    async fn restore(&self, id: i64) -> Result<bool, DevErpError> {
        debug!("Restoring timeline id: {}", id);

        let result = sqlx::query!(
            "UPDATE timelines SET deleted_at = NULL WHERE id = $1 AND deleted_at IS NOT NULL",
            id
        )
        .execute(&self.pool)
        .await?;

        let restored = result.rows_affected() > 0;

        if restored {
            info!(timeline_id = %id, "Timeline restored successfully");
        }

        Ok(restored)
    }
}

/// PostgreSQL implementation of the MilestoneRepository trait
pub struct PostgresMilestoneRepository {
    pool: PgPool,
}

impl PostgresMilestoneRepository {
    /// Create a new PostgreSQL milestone repository
    ///
    /// # Arguments
    /// * `pool` - PostgreSQL connection pool
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl MilestoneRepository for PostgresMilestoneRepository {
    async fn create(&self, milestone: CreateMilestone) -> Result<Milestone, DevErpError> {
        // Validate input
        milestone.validate()
            .map_err(|e| DevErpError::Validation(e))?;

        debug!("Creating milestone: {} for timeline_id: {}", milestone.name, milestone.timeline_id);

        let result = sqlx::query_as!(
            Milestone,
            r#"
            INSERT INTO milestones (
                timeline_id, project_id, name, description,
                target_date, status, completion_percentage, metadata
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            RETURNING
                id, timeline_id, project_id, name, description,
                target_date, actual_date,
                status as "status!: MilestoneStatus",
                completion_percentage as "completion_percentage!",
                metadata,
                created_at, updated_at, deleted_at
            "#,
            milestone.timeline_id,
            milestone.project_id,
            milestone.name,
            milestone.description,
            milestone.target_date,
            milestone.status.unwrap_or(MilestoneStatus::Pending).as_str(),
            milestone.completion_percentage.unwrap_or(0),
            milestone.metadata
        )
        .fetch_one(&self.pool)
        .await?;

        info!(milestone_id = %result.id, timeline_id = %result.timeline_id, "Milestone created successfully");

        Ok(result)
    }

    async fn find_by_id(&self, id: i64) -> Result<Option<Milestone>, DevErpError> {
        debug!("Finding milestone by id: {}", id);

        let result = sqlx::query_as!(
            Milestone,
            r#"
            SELECT
                id, timeline_id, project_id, name, description,
                target_date, actual_date,
                status as "status!: MilestoneStatus",
                completion_percentage as "completion_percentage!",
                metadata,
                created_at, updated_at, deleted_at
            FROM milestones
            WHERE id = $1 AND deleted_at IS NULL
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(result)
    }

    async fn find_all(&self, filter: MilestoneFilter) -> Result<Vec<Milestone>, DevErpError> {
        debug!("Finding milestones with filter: {:?}", filter);

        // Fetch all non-deleted milestones
        let mut results = sqlx::query_as!(
            Milestone,
            r#"
            SELECT
                id, timeline_id, project_id, name, description,
                target_date, actual_date,
                status as "status!: MilestoneStatus",
                completion_percentage as "completion_percentage!",
                metadata,
                created_at, updated_at, deleted_at
            FROM milestones
            WHERE deleted_at IS NULL
            ORDER BY target_date ASC
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        // Apply filters in memory
        if let Some(timeline_id) = filter.timeline_id {
            results.retain(|m| m.timeline_id == timeline_id);
        }

        if let Some(project_id) = filter.project_id {
            results.retain(|m| m.project_id == project_id);
        }

        if let Some(ref status) = filter.status {
            results.retain(|m| &m.status == status);
        }

        // Apply pagination
        let offset = filter.get_offset() as usize;
        let limit = filter.get_limit() as usize;
        let total = results.len();

        if offset < total {
            let end = (offset + limit).min(total);
            results = results[offset..end].to_vec();
        } else {
            results.clear();
        }

        Ok(results)
    }

    async fn find_by_timeline(&self, timeline_id: i64) -> Result<Vec<Milestone>, DevErpError> {
        debug!("Finding milestones for timeline_id: {}", timeline_id);

        let results = sqlx::query_as!(
            Milestone,
            r#"
            SELECT
                id, timeline_id, project_id, name, description,
                target_date, actual_date,
                status as "status!: MilestoneStatus",
                completion_percentage as "completion_percentage!",
                metadata,
                created_at, updated_at, deleted_at
            FROM milestones
            WHERE timeline_id = $1 AND deleted_at IS NULL
            ORDER BY target_date ASC
            "#,
            timeline_id
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(results)
    }

    async fn find_by_project(&self, project_id: i64) -> Result<Vec<Milestone>, DevErpError> {
        debug!("Finding milestones for project_id: {}", project_id);

        let results = sqlx::query_as!(
            Milestone,
            r#"
            SELECT
                id, timeline_id, project_id, name, description,
                target_date, actual_date,
                status as "status!: MilestoneStatus",
                completion_percentage as "completion_percentage!",
                metadata,
                created_at, updated_at, deleted_at
            FROM milestones
            WHERE project_id = $1 AND deleted_at IS NULL
            ORDER BY target_date ASC
            "#,
            project_id
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(results)
    }

    async fn count(&self, filter: MilestoneFilter) -> Result<i64, DevErpError> {
        debug!("Counting milestones with filter: {:?}", filter);

        // For simplicity, reuse find_all and count in memory
        let milestones = self.find_all(MilestoneFilter {
            timeline_id: filter.timeline_id,
            project_id: filter.project_id,
            status: filter.status,
            offset: None,
            limit: None,
        }).await?;

        Ok(milestones.len() as i64)
    }

    async fn update(&self, milestone: UpdateMilestone) -> Result<Milestone, DevErpError> {
        // Validate input
        milestone.validate()
            .map_err(|e| DevErpError::Validation(e))?;

        debug!("Updating milestone id: {}", milestone.id);

        // First, fetch the existing milestone
        let _existing = self.find_by_id(milestone.id).await?
            .ok_or_else(|| DevErpError::NotFound(format!("Milestone with id {} not found", milestone.id)))?;

        // Update fields
        let result = sqlx::query_as!(
            Milestone,
            r#"
            UPDATE milestones
            SET
                name = COALESCE($2, name),
                description = COALESCE($3, description),
                target_date = COALESCE($4, target_date),
                actual_date = COALESCE($5, actual_date),
                status = COALESCE($6, status),
                completion_percentage = COALESCE($7, completion_percentage),
                metadata = COALESCE($8, metadata),
                updated_at = NOW()
            WHERE id = $1 AND deleted_at IS NULL
            RETURNING
                id, timeline_id, project_id, name, description,
                target_date, actual_date,
                status as "status!: MilestoneStatus",
                completion_percentage as "completion_percentage!",
                metadata,
                created_at, updated_at, deleted_at
            "#,
            milestone.id,
            milestone.name,
            milestone.description,
            milestone.target_date,
            milestone.actual_date,
            milestone.status.map(|s| s.as_str().to_string()),
            milestone.completion_percentage,
            milestone.metadata
        )
        .fetch_one(&self.pool)
        .await?;

        info!(milestone_id = %result.id, "Milestone updated successfully");

        Ok(result)
    }

    async fn soft_delete(&self, id: i64) -> Result<bool, DevErpError> {
        debug!("Soft deleting milestone id: {}", id);

        let result = sqlx::query!(
            "UPDATE milestones SET deleted_at = NOW() WHERE id = $1 AND deleted_at IS NULL",
            id
        )
        .execute(&self.pool)
        .await?;

        let deleted = result.rows_affected() > 0;

        if deleted {
            info!(milestone_id = %id, "Milestone soft deleted successfully");
        }

        Ok(deleted)
    }

    async fn delete(&self, id: i64) -> Result<bool, DevErpError> {
        debug!("Hard deleting milestone id: {}", id);

        let result = sqlx::query!("DELETE FROM milestones WHERE id = $1", id)
            .execute(&self.pool)
            .await?;

        let deleted = result.rows_affected() > 0;

        if deleted {
            info!(milestone_id = %id, "Milestone hard deleted successfully");
        }

        Ok(deleted)
    }

    async fn restore(&self, id: i64) -> Result<bool, DevErpError> {
        debug!("Restoring milestone id: {}", id);

        let result = sqlx::query!(
            "UPDATE milestones SET deleted_at = NULL WHERE id = $1 AND deleted_at IS NOT NULL",
            id
        )
        .execute(&self.pool)
        .await?;

        let restored = result.rows_affected() > 0;

        if restored {
            info!(milestone_id = %id, "Milestone restored successfully");
        }

        Ok(restored)
    }
}
