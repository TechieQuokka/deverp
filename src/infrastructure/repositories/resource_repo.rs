use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::resource::{
    entity::{
        CreateResource, LinkResourceToProject, ProjectResource, Resource, ResourceFilter,
        ResourceUsageStats, UpdateProjectResource, UpdateResource,
    },
    repository::ResourceRepository,
};
use crate::utils::error::DevErpError;

/// PostgreSQL implementation of ResourceRepository
pub struct PostgresResourceRepository {
    pool: PgPool,
}

impl PostgresResourceRepository {
    /// Create a new PostgresResourceRepository
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ResourceRepository for PostgresResourceRepository {
    async fn create(&self, resource: CreateResource) -> Result<Resource, DevErpError> {
        let rec = sqlx::query_as!(
            Resource,
            r#"
            INSERT INTO resources (
                name, description, resource_type, version, url,
                documentation_url, license, status, metadata, tags
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, COALESCE($8, 'active'), $9, $10)
            RETURNING
                id, uuid, name, description,
                resource_type as "resource_type: _",
                version, url, documentation_url, license,
                status as "status: _",
                metadata, tags,
                created_at, updated_at, deleted_at
            "#,
            resource.name,
            resource.description,
            resource.resource_type as _,
            resource.version,
            resource.url,
            resource.documentation_url,
            resource.license,
            resource.status.map(|s| s.to_string()),
            resource.metadata,
            resource.tags.as_deref(),
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(rec)
    }

    async fn find_by_id(&self, id: i64) -> Result<Option<Resource>, DevErpError> {
        let resource = sqlx::query_as!(
            Resource,
            r#"
            SELECT
                id, uuid, name, description,
                resource_type as "resource_type: _",
                version, url, documentation_url, license,
                status as "status: _",
                metadata, tags,
                created_at, updated_at, deleted_at
            FROM resources
            WHERE id = $1 AND deleted_at IS NULL
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(resource)
    }

    async fn find_by_uuid(&self, uuid: Uuid) -> Result<Option<Resource>, DevErpError> {
        let resource = sqlx::query_as!(
            Resource,
            r#"
            SELECT
                id, uuid, name, description,
                resource_type as "resource_type: _",
                version, url, documentation_url, license,
                status as "status: _",
                metadata, tags,
                created_at, updated_at, deleted_at
            FROM resources
            WHERE uuid = $1 AND deleted_at IS NULL
            "#,
            uuid
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(resource)
    }

    async fn find_all(&self, filter: ResourceFilter) -> Result<Vec<Resource>, DevErpError> {
        // For simplicity, we'll use a basic query with optional filters
        // In production, consider using a query builder for complex filtering

        let resources = if filter.resource_type.is_none()
            && filter.status.is_none()
            && filter.name_contains.is_none()
            && filter.tags.is_none()
        {
            // No filters, get all resources
            let query = sqlx::query_as!(
                Resource,
                r#"
                SELECT
                    id, uuid, name, description,
                    resource_type as "resource_type: _",
                    version, url, documentation_url, license,
                    status as "status: _",
                    metadata, tags,
                    created_at, updated_at, deleted_at
                FROM resources
                WHERE deleted_at IS NULL
                ORDER BY name ASC
                "#
            );

            query.fetch_all(&self.pool).await?
        } else if let Some(resource_type) = filter.resource_type {
            // Filter by resource type
            sqlx::query_as!(
                Resource,
                r#"
                SELECT
                    id, uuid, name, description,
                    resource_type as "resource_type: _",
                    version, url, documentation_url, license,
                    status as "status: _",
                    metadata, tags,
                    created_at, updated_at, deleted_at
                FROM resources
                WHERE deleted_at IS NULL AND resource_type = $1
                ORDER BY name ASC
                "#,
                resource_type as _
            )
            .fetch_all(&self.pool)
            .await?
        } else if let Some(status) = filter.status {
            // Filter by status
            sqlx::query_as!(
                Resource,
                r#"
                SELECT
                    id, uuid, name, description,
                    resource_type as "resource_type: _",
                    version, url, documentation_url, license,
                    status as "status: _",
                    metadata, tags,
                    created_at, updated_at, deleted_at
                FROM resources
                WHERE deleted_at IS NULL AND status = $1
                ORDER BY name ASC
                "#,
                status as _
            )
            .fetch_all(&self.pool)
            .await?
        } else if let Some(name_contains) = filter.name_contains {
            // Filter by name
            let pattern = format!("%{}%", name_contains);
            sqlx::query_as!(
                Resource,
                r#"
                SELECT
                    id, uuid, name, description,
                    resource_type as "resource_type: _",
                    version, url, documentation_url, license,
                    status as "status: _",
                    metadata, tags,
                    created_at, updated_at, deleted_at
                FROM resources
                WHERE deleted_at IS NULL AND name ILIKE $1
                ORDER BY name ASC
                "#,
                pattern
            )
            .fetch_all(&self.pool)
            .await?
        } else {
            // Default: return all
            sqlx::query_as!(
                Resource,
                r#"
                SELECT
                    id, uuid, name, description,
                    resource_type as "resource_type: _",
                    version, url, documentation_url, license,
                    status as "status: _",
                    metadata, tags,
                    created_at, updated_at, deleted_at
                FROM resources
                WHERE deleted_at IS NULL
                ORDER BY name ASC
                "#
            )
            .fetch_all(&self.pool)
            .await?
        };

        // Apply pagination if specified
        let offset = filter.offset.unwrap_or(0) as usize;
        let limit = filter.limit.map(|l| l as usize);

        let result: Vec<Resource> = resources
            .into_iter()
            .skip(offset)
            .take(limit.unwrap_or(usize::MAX))
            .collect();

        Ok(result)
    }

    async fn update(&self, resource: UpdateResource) -> Result<Resource, DevErpError> {
        let rec = sqlx::query_as!(
            Resource,
            r#"
            UPDATE resources
            SET
                name = COALESCE($2, name),
                description = COALESCE($3, description),
                resource_type = COALESCE($4, resource_type),
                version = COALESCE($5, version),
                url = COALESCE($6, url),
                documentation_url = COALESCE($7, documentation_url),
                license = COALESCE($8, license),
                status = COALESCE($9, status),
                metadata = COALESCE($10, metadata),
                tags = COALESCE($11, tags)
            WHERE id = $1 AND deleted_at IS NULL
            RETURNING
                id, uuid, name, description,
                resource_type as "resource_type: _",
                version, url, documentation_url, license,
                status as "status: _",
                metadata, tags,
                created_at, updated_at, deleted_at
            "#,
            resource.id,
            resource.name,
            resource.description,
            resource.resource_type.map(|rt| rt.to_string()),
            resource.version,
            resource.url,
            resource.documentation_url,
            resource.license,
            resource.status.map(|s| s.to_string()),
            resource.metadata,
            resource.tags.as_deref(),
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(rec)
    }

    async fn soft_delete(&self, id: i64) -> Result<bool, DevErpError> {
        let result = sqlx::query!(
            r#"
            UPDATE resources
            SET deleted_at = NOW()
            WHERE id = $1 AND deleted_at IS NULL
            "#,
            id
        )
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected() > 0)
    }

    async fn delete(&self, id: i64) -> Result<bool, DevErpError> {
        let result = sqlx::query!(
            r#"
            DELETE FROM resources
            WHERE id = $1
            "#,
            id
        )
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected() > 0)
    }

    async fn link_to_project(
        &self,
        link: LinkResourceToProject,
    ) -> Result<ProjectResource, DevErpError> {
        let rec = sqlx::query_as!(
            ProjectResource,
            r#"
            INSERT INTO project_resources (
                project_id, resource_id, usage_notes, version_used, is_critical
            )
            VALUES ($1, $2, $3, $4, COALESCE($5, false))
            RETURNING
                project_id, resource_id, usage_notes, version_used,
                is_critical, added_at, removed_at
            "#,
            link.project_id,
            link.resource_id,
            link.usage_notes,
            link.version_used,
            link.is_critical,
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(rec)
    }

    async fn unlink_from_project(
        &self,
        project_id: i64,
        resource_id: i64,
    ) -> Result<bool, DevErpError> {
        let result = sqlx::query!(
            r#"
            UPDATE project_resources
            SET removed_at = NOW()
            WHERE project_id = $1 AND resource_id = $2 AND removed_at IS NULL
            "#,
            project_id,
            resource_id
        )
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected() > 0)
    }

    async fn update_project_resource(
        &self,
        update: UpdateProjectResource,
    ) -> Result<ProjectResource, DevErpError> {
        let rec = sqlx::query_as!(
            ProjectResource,
            r#"
            UPDATE project_resources
            SET
                usage_notes = COALESCE($3, usage_notes),
                version_used = COALESCE($4, version_used),
                is_critical = COALESCE($5, is_critical)
            WHERE project_id = $1 AND resource_id = $2 AND removed_at IS NULL
            RETURNING
                project_id, resource_id, usage_notes, version_used,
                is_critical, added_at, removed_at
            "#,
            update.project_id,
            update.resource_id,
            update.usage_notes,
            update.version_used,
            update.is_critical,
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(rec)
    }

    async fn find_by_project_id(&self, project_id: i64) -> Result<Vec<Resource>, DevErpError> {
        let resources = sqlx::query_as!(
            Resource,
            r#"
            SELECT
                r.id, r.uuid, r.name, r.description,
                r.resource_type as "resource_type: _",
                r.version, r.url, r.documentation_url, r.license,
                r.status as "status: _",
                r.metadata, r.tags,
                r.created_at, r.updated_at, r.deleted_at
            FROM resources r
            INNER JOIN project_resources pr ON r.id = pr.resource_id
            WHERE pr.project_id = $1
              AND r.deleted_at IS NULL
              AND pr.removed_at IS NULL
            ORDER BY r.name ASC
            "#,
            project_id
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(resources)
    }

    async fn find_projects_using_resource(&self, resource_id: i64) -> Result<Vec<i64>, DevErpError> {
        let project_ids = sqlx::query_scalar!(
            r#"
            SELECT project_id
            FROM project_resources
            WHERE resource_id = $1 AND removed_at IS NULL
            "#,
            resource_id
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(project_ids)
    }

    async fn get_usage_stats(&self, resource_id: i64) -> Result<ResourceUsageStats, DevErpError> {
        let stats = sqlx::query_as!(
            ResourceUsageStats,
            r#"
            SELECT
                r.id as resource_id,
                r.name as resource_name,
                r.resource_type as "resource_type: _",
                COUNT(DISTINCT pr.project_id) as "total_projects!",
                COUNT(DISTINCT CASE WHEN pr.is_critical = true THEN pr.project_id END) as "critical_projects!"
            FROM resources r
            LEFT JOIN project_resources pr ON r.id = pr.resource_id AND pr.removed_at IS NULL
            WHERE r.id = $1 AND r.deleted_at IS NULL
            GROUP BY r.id, r.name, r.resource_type
            "#,
            resource_id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(stats)
    }

    async fn get_all_usage_stats(&self) -> Result<Vec<ResourceUsageStats>, DevErpError> {
        let stats = sqlx::query_as!(
            ResourceUsageStats,
            r#"
            SELECT
                r.id as resource_id,
                r.name as resource_name,
                r.resource_type as "resource_type: _",
                COUNT(DISTINCT pr.project_id) as "total_projects!",
                COUNT(DISTINCT CASE WHEN pr.is_critical = true THEN pr.project_id END) as "critical_projects!"
            FROM resources r
            LEFT JOIN project_resources pr ON r.id = pr.resource_id AND pr.removed_at IS NULL
            WHERE r.deleted_at IS NULL
            GROUP BY r.id, r.name, r.resource_type
            ORDER BY COUNT(DISTINCT pr.project_id) DESC, r.name ASC
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(stats)
    }
}
