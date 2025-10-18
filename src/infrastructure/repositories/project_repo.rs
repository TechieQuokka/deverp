// PostgreSQL implementation of ProjectRepository

use async_trait::async_trait;
use sqlx::{PgPool, Row};
use tracing::{debug, info, warn};
use uuid::Uuid;

use crate::domain::project::{
    entity::{CreateProject, Project, ProjectFilter, UpdateProject},
    repository::ProjectRepository,
};
use crate::utils::error::DevErpError;

/// PostgreSQL implementation of the ProjectRepository trait
pub struct PostgresProjectRepository {
    pool: PgPool,
}

impl PostgresProjectRepository {
    /// Create a new PostgreSQL project repository
    ///
    /// # Arguments
    /// * `pool` - PostgreSQL connection pool
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ProjectRepository for PostgresProjectRepository {
    async fn create(&self, project: CreateProject) -> Result<Project, DevErpError> {
        // Validate input
        project.validate()
            .map_err(|e| DevErpError::Validation(e))?;

        // Check for code uniqueness if code is provided
        if let Some(ref code) = project.code {
            if self.code_exists(code, None).await? {
                return Err(DevErpError::Conflict(format!(
                    "Project code '{}' already exists",
                    code
                )));
            }
        }

        debug!("Creating project: {}", project.name);

        let result = sqlx::query_as!(
            Project,
            r#"
            INSERT INTO projects (
                name, description, code, status, priority,
                start_date, end_date, repository_url, repository_branch,
                tags, metadata
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
            RETURNING
                id, uuid, name, description, code,
                status as "status: _", priority as "priority: _",
                start_date, end_date, actual_start_date, actual_end_date,
                progress_percentage, repository_url, repository_branch,
                tags, metadata,
                created_at, updated_at, deleted_at
            "#,
            project.name,
            project.description,
            project.code,
            project.status.unwrap_or(crate::domain::project::entity::ProjectStatus::Planning).as_str(),
            project.priority.unwrap_or(crate::domain::project::entity::Priority::Medium).as_str(),
            project.start_date,
            project.end_date,
            project.repository_url,
            project.repository_branch.or(Some("main".to_string())),
            project.tags.as_deref(),
            project.metadata
        )
        .fetch_one(&self.pool)
        .await?;

        info!(project_id = %result.id, project_uuid = %result.uuid, "Project created successfully");

        Ok(result)
    }

    async fn find_by_id(&self, id: i64) -> Result<Option<Project>, DevErpError> {
        debug!("Finding project by id: {}", id);

        let result = sqlx::query_as!(
            Project,
            r#"
            SELECT
                id, uuid, name, description, code,
                status as "status: _", priority as "priority: _",
                start_date, end_date, actual_start_date, actual_end_date,
                progress_percentage, repository_url, repository_branch,
                tags, metadata,
                created_at, updated_at, deleted_at
            FROM projects
            WHERE id = $1 AND deleted_at IS NULL
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(result)
    }

    async fn find_by_uuid(&self, uuid: Uuid) -> Result<Option<Project>, DevErpError> {
        debug!("Finding project by uuid: {}", uuid);

        let result = sqlx::query_as!(
            Project,
            r#"
            SELECT
                id, uuid, name, description, code,
                status as "status: _", priority as "priority: _",
                start_date, end_date, actual_start_date, actual_end_date,
                progress_percentage, repository_url, repository_branch,
                tags, metadata,
                created_at, updated_at, deleted_at
            FROM projects
            WHERE uuid = $1 AND deleted_at IS NULL
            "#,
            uuid
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(result)
    }

    async fn find_by_code(&self, code: &str) -> Result<Option<Project>, DevErpError> {
        debug!("Finding project by code: {}", code);

        let result = sqlx::query_as!(
            Project,
            r#"
            SELECT
                id, uuid, name, description, code,
                status as "status: _", priority as "priority: _",
                start_date, end_date, actual_start_date, actual_end_date,
                progress_percentage, repository_url, repository_branch,
                tags, metadata,
                created_at, updated_at, deleted_at
            FROM projects
            WHERE code = $1 AND deleted_at IS NULL
            "#,
            code
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(result)
    }

    async fn find_all(&self, filter: ProjectFilter) -> Result<Vec<Project>, DevErpError> {
        debug!("Finding all projects with filter: {:?}", filter);

        let limit = filter.get_limit();
        let offset = filter.get_offset();

        // Build dynamic query based on filter
        let mut query = String::from(
            r#"
            SELECT
                id, uuid, name, description, code,
                status, priority,
                start_date, end_date, actual_start_date, actual_end_date,
                progress_percentage, repository_url, repository_branch,
                tags, metadata,
                created_at, updated_at, deleted_at
            FROM projects
            WHERE deleted_at IS NULL
            "#
        );

        let mut conditions = Vec::new();
        let mut param_count = 1;

        // Add status filter
        if filter.status.is_some() {
            conditions.push(format!("status = ${}", param_count));
            param_count += 1;
        }

        // Add priority filter
        if filter.priority.is_some() {
            conditions.push(format!("priority = ${}", param_count));
            param_count += 1;
        }

        // Add search filter (searches in name and description)
        if filter.search.is_some() {
            conditions.push(format!(
                "(name ILIKE ${} OR description ILIKE ${})",
                param_count, param_count
            ));
            param_count += 1;
        }

        // Add tag filter
        if let Some(ref tags) = filter.tags {
            if !tags.is_empty() {
                conditions.push(format!("tags && ${}", param_count));
                param_count += 1;
            }
        }

        if !conditions.is_empty() {
            query.push_str(" AND ");
            query.push_str(&conditions.join(" AND "));
        }

        query.push_str(" ORDER BY created_at DESC");
        query.push_str(&format!(" LIMIT ${} OFFSET ${}", param_count, param_count + 1));

        // Execute query with dynamic binding
        let mut query_builder = sqlx::query_as::<_, Project>(&query);

        if let Some(status) = &filter.status {
            query_builder = query_builder.bind(status.as_str());
        }

        if let Some(priority) = &filter.priority {
            query_builder = query_builder.bind(priority.as_str());
        }

        if let Some(ref search) = filter.search {
            let search_pattern = format!("%{}%", search);
            query_builder = query_builder.bind(search_pattern);
        }

        if let Some(ref tags) = filter.tags {
            if !tags.is_empty() {
                query_builder = query_builder.bind(tags);
            }
        }

        query_builder = query_builder.bind(limit).bind(offset);

        let results = query_builder.fetch_all(&self.pool).await?;

        debug!("Found {} projects", results.len());

        Ok(results)
    }

    async fn count(&self, filter: ProjectFilter) -> Result<i64, DevErpError> {
        debug!("Counting projects with filter: {:?}", filter);

        let mut query = String::from(
            "SELECT COUNT(*) as count FROM projects WHERE deleted_at IS NULL"
        );

        let mut conditions = Vec::new();
        let mut param_count = 1;

        if filter.status.is_some() {
            conditions.push(format!("status = ${}", param_count));
            param_count += 1;
        }

        if filter.priority.is_some() {
            conditions.push(format!("priority = ${}", param_count));
            param_count += 1;
        }

        if filter.search.is_some() {
            conditions.push(format!(
                "(name ILIKE ${} OR description ILIKE ${})",
                param_count, param_count
            ));
            param_count += 1;
        }

        if let Some(ref tags) = filter.tags {
            if !tags.is_empty() {
                conditions.push(format!("tags && ${}", param_count));
            }
        }

        if !conditions.is_empty() {
            query.push_str(" AND ");
            query.push_str(&conditions.join(" AND "));
        }

        let mut query_builder = sqlx::query(&query);

        if let Some(status) = &filter.status {
            query_builder = query_builder.bind(status.as_str());
        }

        if let Some(priority) = &filter.priority {
            query_builder = query_builder.bind(priority.as_str());
        }

        if let Some(ref search) = filter.search {
            let search_pattern = format!("%{}%", search);
            query_builder = query_builder.bind(search_pattern);
        }

        if let Some(ref tags) = filter.tags {
            if !tags.is_empty() {
                query_builder = query_builder.bind(tags);
            }
        }

        let row = query_builder.fetch_one(&self.pool).await?;
        let count: i64 = row.try_get("count")?;

        debug!("Project count: {}", count);

        Ok(count)
    }

    async fn update(&self, project: UpdateProject) -> Result<Project, DevErpError> {
        // Validate input
        project.validate()
            .map_err(|e| DevErpError::Validation(e))?;

        // Check if project exists
        let existing = self.find_by_id(project.id).await?
            .ok_or_else(|| DevErpError::NotFound(format!("Project with id {} not found", project.id)))?;

        // Check for code uniqueness if code is being changed
        if let Some(ref new_code) = project.code {
            if existing.code.as_ref() != Some(new_code) {
                if self.code_exists(new_code, Some(project.id)).await? {
                    return Err(DevErpError::Conflict(format!(
                        "Project code '{}' already exists",
                        new_code
                    )));
                }
            }
        }

        debug!("Updating project: {}", project.id);

        // Build dynamic update query
        let result = sqlx::query_as!(
            Project,
            r#"
            UPDATE projects
            SET
                name = COALESCE($2, name),
                description = COALESCE($3, description),
                code = COALESCE($4, code),
                status = COALESCE($5, status),
                priority = COALESCE($6, priority),
                start_date = COALESCE($7, start_date),
                end_date = COALESCE($8, end_date),
                actual_start_date = COALESCE($9, actual_start_date),
                actual_end_date = COALESCE($10, actual_end_date),
                progress_percentage = COALESCE($11, progress_percentage),
                repository_url = COALESCE($12, repository_url),
                repository_branch = COALESCE($13, repository_branch),
                tags = COALESCE($14, tags),
                metadata = COALESCE($15, metadata),
                updated_at = CURRENT_TIMESTAMP
            WHERE id = $1 AND deleted_at IS NULL
            RETURNING
                id, uuid, name, description, code,
                status as "status: _", priority as "priority: _",
                start_date, end_date, actual_start_date, actual_end_date,
                progress_percentage, repository_url, repository_branch,
                tags, metadata,
                created_at, updated_at, deleted_at
            "#,
            project.id,
            project.name,
            project.description,
            project.code,
            project.status.map(|s| s.as_str()),
            project.priority.map(|p| p.as_str()),
            project.start_date,
            project.end_date,
            project.actual_start_date,
            project.actual_end_date,
            project.progress_percentage,
            project.repository_url,
            project.repository_branch,
            project.tags.as_deref(),
            project.metadata
        )
        .fetch_one(&self.pool)
        .await?;

        info!(project_id = %result.id, "Project updated successfully");

        Ok(result)
    }

    async fn soft_delete(&self, id: i64) -> Result<bool, DevErpError> {
        debug!("Soft deleting project: {}", id);

        let result = sqlx::query!(
            "UPDATE projects SET deleted_at = CURRENT_TIMESTAMP WHERE id = $1 AND deleted_at IS NULL",
            id
        )
        .execute(&self.pool)
        .await?;

        let deleted = result.rows_affected() > 0;

        if deleted {
            info!(project_id = %id, "Project soft deleted successfully");
        } else {
            warn!(project_id = %id, "Project not found for soft delete");
        }

        Ok(deleted)
    }

    async fn delete(&self, id: i64) -> Result<bool, DevErpError> {
        warn!(project_id = %id, "Hard deleting project - this is irreversible");

        let result = sqlx::query!(
            "DELETE FROM projects WHERE id = $1",
            id
        )
        .execute(&self.pool)
        .await?;

        let deleted = result.rows_affected() > 0;

        if deleted {
            warn!(project_id = %id, "Project hard deleted successfully");
        } else {
            warn!(project_id = %id, "Project not found for hard delete");
        }

        Ok(deleted)
    }

    async fn restore(&self, id: i64) -> Result<bool, DevErpError> {
        debug!("Restoring project: {}", id);

        let result = sqlx::query!(
            "UPDATE projects SET deleted_at = NULL WHERE id = $1 AND deleted_at IS NOT NULL",
            id
        )
        .execute(&self.pool)
        .await?;

        let restored = result.rows_affected() > 0;

        if restored {
            info!(project_id = %id, "Project restored successfully");
        } else {
            warn!(project_id = %id, "Project not found or not deleted for restore");
        }

        Ok(restored)
    }

    async fn code_exists(&self, code: &str, exclude_id: Option<i64>) -> Result<bool, DevErpError> {
        debug!("Checking if project code exists: {}", code);

        let count: i64 = if let Some(id) = exclude_id {
            sqlx::query_scalar!(
                "SELECT COUNT(*) as count FROM projects WHERE code = $1 AND id != $2 AND deleted_at IS NULL",
                code,
                id
            )
            .fetch_one(&self.pool)
            .await?
            .unwrap_or(0)
        } else {
            sqlx::query_scalar!(
                "SELECT COUNT(*) as count FROM projects WHERE code = $1 AND deleted_at IS NULL",
                code
            )
            .fetch_one(&self.pool)
            .await?
            .unwrap_or(0)
        };

        Ok(count > 0)
    }

    async fn find_by_tag(&self, tag: &str) -> Result<Vec<Project>, DevErpError> {
        debug!("Finding projects by tag: {}", tag);

        let results = sqlx::query_as!(
            Project,
            r#"
            SELECT
                id, uuid, name, description, code,
                status as "status: _", priority as "priority: _",
                start_date, end_date, actual_start_date, actual_end_date,
                progress_percentage, repository_url, repository_branch,
                tags, metadata,
                created_at, updated_at, deleted_at
            FROM projects
            WHERE $1 = ANY(tags) AND deleted_at IS NULL
            ORDER BY created_at DESC
            "#,
            tag
        )
        .fetch_all(&self.pool)
        .await?;

        debug!("Found {} projects with tag '{}'", results.len(), tag);

        Ok(results)
    }
}

#[cfg(test)]
mod tests {
    // Note: Full integration tests should be in tests/ directory
    // using testcontainers for real database testing

    #[test]
    fn test_repository_creation() {
        // This is a simple smoke test to ensure the struct can be created
        // Real tests require a database connection
    }
}
