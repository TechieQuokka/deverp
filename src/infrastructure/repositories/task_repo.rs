use async_trait::async_trait;
use chrono::Utc;
use sqlx::PgPool;
use std::collections::{HashSet, VecDeque};
use uuid::Uuid;

use crate::domain::task::{
    CreateTask, CreateTaskComment, CreateTaskDependency, Task, TaskComment, TaskCommentRepository,
    TaskDependency, TaskDependencyRepository, TaskFilter, TaskRepository, UpdateTask,
};
use crate::utils::error::DevErpError;

/// PostgreSQL implementation of TaskRepository
pub struct PostgresTaskRepository {
    pool: PgPool,
}

impl PostgresTaskRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl TaskRepository for PostgresTaskRepository {
    async fn create(&self, task: CreateTask) -> Result<Task, DevErpError> {
        task.validate()
            .map_err(|e| DevErpError::Validation(e))?;

        let task = sqlx::query_as!(
            Task,
            r#"
            INSERT INTO tasks (
                project_id, parent_task_id, title, description, task_number,
                status, priority, assigned_to, estimated_hours, due_date, task_type, tags
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
            RETURNING
                id, uuid, project_id, parent_task_id, title, description, task_number,
                status as "status: _", priority as "priority: _", assigned_to,
                estimated_hours, actual_hours, due_date, started_at, completed_at,
                task_type as "task_type: _", tags, created_at, updated_at, deleted_at
            "#,
            task.project_id,
            task.parent_task_id,
            task.title,
            task.description,
            task.task_number,
            task.status.unwrap_or_default().to_string(),
            task.priority.unwrap_or_default().to_string(),
            task.assigned_to,
            task.estimated_hours,
            task.due_date,
            task.task_type.unwrap_or_default().to_string(),
            task.tags.as_deref(),
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(task)
    }

    async fn find_by_id(&self, id: i64) -> Result<Option<Task>, DevErpError> {
        let task = sqlx::query_as!(
            Task,
            r#"
            SELECT
                id, uuid, project_id, parent_task_id, title, description, task_number,
                status as "status: _", priority as "priority: _", assigned_to,
                estimated_hours, actual_hours, due_date, started_at, completed_at,
                task_type as "task_type: _", tags, created_at, updated_at, deleted_at
            FROM tasks
            WHERE id = $1 AND deleted_at IS NULL
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(task)
    }

    async fn find_by_uuid(&self, uuid: Uuid) -> Result<Option<Task>, DevErpError> {
        let task = sqlx::query_as!(
            Task,
            r#"
            SELECT
                id, uuid, project_id, parent_task_id, title, description, task_number,
                status as "status: _", priority as "priority: _", assigned_to,
                estimated_hours, actual_hours, due_date, started_at, completed_at,
                task_type as "task_type: _", tags, created_at, updated_at, deleted_at
            FROM tasks
            WHERE uuid = $1 AND deleted_at IS NULL
            "#,
            uuid
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(task)
    }

    async fn find_all(&self, filter: TaskFilter) -> Result<Vec<Task>, DevErpError> {
        let mut query = String::from(
            r#"
            SELECT
                id, uuid, project_id, parent_task_id, title, description, task_number,
                status, priority, assigned_to, estimated_hours, actual_hours,
                due_date, started_at, completed_at, task_type, tags,
                created_at, updated_at, deleted_at
            FROM tasks
            WHERE 1=1
            "#,
        );

        if !filter.include_deleted {
            query.push_str(" AND deleted_at IS NULL");
        }

        if let Some(project_id) = filter.project_id {
            query.push_str(&format!(" AND project_id = {}", project_id));
        }

        if let Some(ref status) = filter.status {
            query.push_str(&format!(" AND status = '{}'", status));
        }

        if let Some(ref priority) = filter.priority {
            query.push_str(&format!(" AND priority = '{}'", priority));
        }

        if let Some(ref task_type) = filter.task_type {
            query.push_str(&format!(" AND task_type = '{}'", task_type));
        }

        if let Some(ref assigned_to) = filter.assigned_to {
            query.push_str(&format!(" AND assigned_to = '{}'", assigned_to));
        }

        if let Some(parent_task_id) = filter.parent_task_id {
            query.push_str(&format!(" AND parent_task_id = {}", parent_task_id));
        }

        query.push_str(" ORDER BY created_at DESC");

        if let Some(limit) = filter.limit {
            query.push_str(&format!(" LIMIT {}", limit));
        }

        if let Some(offset) = filter.offset {
            query.push_str(&format!(" OFFSET {}", offset));
        }

        let tasks = sqlx::query_as::<_, Task>(&query)
            .fetch_all(&self.pool)
            .await?;

        Ok(tasks)
    }

    async fn update(&self, task: UpdateTask) -> Result<Task, DevErpError> {
        task.validate()
            .map_err(|e| DevErpError::Validation(e))?;

        // Build dynamic update query
        let mut updates = Vec::new();
        let mut args_index = 2; // Start from $2 since $1 is id

        if task.title.is_some() {
            updates.push(format!("title = ${}", args_index));
            args_index += 1;
        }
        if task.description.is_some() {
            updates.push(format!("description = ${}", args_index));
            args_index += 1;
        }
        if task.status.is_some() {
            updates.push(format!("status = ${}", args_index));
            args_index += 1;
        }
        if task.priority.is_some() {
            updates.push(format!("priority = ${}", args_index));
            args_index += 1;
        }
        if task.assigned_to.is_some() {
            updates.push(format!("assigned_to = ${}", args_index));
            args_index += 1;
        }
        if task.estimated_hours.is_some() {
            updates.push(format!("estimated_hours = ${}", args_index));
            args_index += 1;
        }
        if task.actual_hours.is_some() {
            updates.push(format!("actual_hours = ${}", args_index));
            args_index += 1;
        }
        if task.due_date.is_some() {
            updates.push(format!("due_date = ${}", args_index));
            args_index += 1;
        }
        if task.task_type.is_some() {
            updates.push(format!("task_type = ${}", args_index));
            args_index += 1;
        }
        if task.tags.is_some() {
            updates.push(format!("tags = ${}", args_index));
        }

        if updates.is_empty() {
            return Err(DevErpError::Validation(
                "No fields to update".to_string(),
            ));
        }

        let query = format!(
            r#"
            UPDATE tasks
            SET {}
            WHERE id = $1 AND deleted_at IS NULL
            RETURNING
                id, uuid, project_id, parent_task_id, title, description, task_number,
                status, priority, assigned_to, estimated_hours, actual_hours,
                due_date, started_at, completed_at, task_type, tags,
                created_at, updated_at, deleted_at
            "#,
            updates.join(", ")
        );

        let mut query_builder = sqlx::query_as::<_, Task>(&query);
        query_builder = query_builder.bind(task.id);

        if let Some(title) = task.title {
            query_builder = query_builder.bind(title);
        }
        if let Some(description) = task.description {
            query_builder = query_builder.bind(description);
        }
        if let Some(status) = task.status {
            query_builder = query_builder.bind(status.to_string());
        }
        if let Some(priority) = task.priority {
            query_builder = query_builder.bind(priority.to_string());
        }
        if let Some(assigned_to) = task.assigned_to {
            query_builder = query_builder.bind(assigned_to);
        }
        if let Some(estimated_hours) = task.estimated_hours {
            query_builder = query_builder.bind(estimated_hours);
        }
        if let Some(actual_hours) = task.actual_hours {
            query_builder = query_builder.bind(actual_hours);
        }
        if let Some(due_date) = task.due_date {
            query_builder = query_builder.bind(due_date);
        }
        if let Some(task_type) = task.task_type {
            query_builder = query_builder.bind(task_type.to_string());
        }
        if let Some(tags) = task.tags {
            query_builder = query_builder.bind(tags);
        }

        let updated_task = query_builder.fetch_one(&self.pool).await?;

        Ok(updated_task)
    }

    async fn delete(&self, id: i64) -> Result<bool, DevErpError> {
        let result = sqlx::query!(
            r#"
            DELETE FROM tasks WHERE id = $1
            "#,
            id
        )
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected() > 0)
    }

    async fn soft_delete(&self, id: i64) -> Result<bool, DevErpError> {
        let result = sqlx::query!(
            r#"
            UPDATE tasks
            SET deleted_at = $1
            WHERE id = $2 AND deleted_at IS NULL
            "#,
            Utc::now(),
            id
        )
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected() > 0)
    }

    async fn count(&self, filter: TaskFilter) -> Result<i64, DevErpError> {
        let mut query = String::from("SELECT COUNT(*) as count FROM tasks WHERE 1=1");

        if !filter.include_deleted {
            query.push_str(" AND deleted_at IS NULL");
        }

        if let Some(project_id) = filter.project_id {
            query.push_str(&format!(" AND project_id = {}", project_id));
        }

        if let Some(ref status) = filter.status {
            query.push_str(&format!(" AND status = '{}'", status));
        }

        if let Some(ref priority) = filter.priority {
            query.push_str(&format!(" AND priority = '{}'", priority));
        }

        if let Some(ref task_type) = filter.task_type {
            query.push_str(&format!(" AND task_type = '{}'", task_type));
        }

        let result: (i64,) = sqlx::query_as(&query).fetch_one(&self.pool).await?;

        Ok(result.0)
    }
}

/// PostgreSQL implementation of TaskDependencyRepository
pub struct PostgresTaskDependencyRepository {
    pool: PgPool,
}

impl PostgresTaskDependencyRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl TaskDependencyRepository for PostgresTaskDependencyRepository {
    async fn add_dependency(
        &self,
        dependency: CreateTaskDependency,
    ) -> Result<TaskDependency, DevErpError> {
        dependency
            .validate()
            .map_err(|e| DevErpError::Validation(e))?;

        // Check if adding this dependency would create a cycle
        if self
            .would_create_cycle(dependency.task_id, dependency.depends_on_task_id)
            .await?
        {
            return Err(DevErpError::Validation(
                "Adding this dependency would create a circular dependency".to_string(),
            ));
        }

        let task_dep = sqlx::query_as!(
            TaskDependency,
            r#"
            INSERT INTO task_dependencies (task_id, depends_on_task_id, dependency_type)
            VALUES ($1, $2, $3)
            RETURNING
                task_id, depends_on_task_id,
                dependency_type as "dependency_type: _",
                created_at
            "#,
            dependency.task_id,
            dependency.depends_on_task_id,
            dependency.dependency_type.unwrap_or_default().to_string()
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(task_dep)
    }

    async fn remove_dependency(
        &self,
        task_id: i64,
        depends_on_task_id: i64,
    ) -> Result<bool, DevErpError> {
        let result = sqlx::query!(
            r#"
            DELETE FROM task_dependencies
            WHERE task_id = $1 AND depends_on_task_id = $2
            "#,
            task_id,
            depends_on_task_id
        )
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected() > 0)
    }

    async fn get_dependencies(&self, task_id: i64) -> Result<Vec<TaskDependency>, DevErpError> {
        let dependencies = sqlx::query_as!(
            TaskDependency,
            r#"
            SELECT
                task_id, depends_on_task_id,
                dependency_type as "dependency_type: _",
                created_at
            FROM task_dependencies
            WHERE task_id = $1
            "#,
            task_id
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(dependencies)
    }

    async fn get_dependents(&self, task_id: i64) -> Result<Vec<TaskDependency>, DevErpError> {
        let dependents = sqlx::query_as!(
            TaskDependency,
            r#"
            SELECT
                task_id, depends_on_task_id,
                dependency_type as "dependency_type: _",
                created_at
            FROM task_dependencies
            WHERE depends_on_task_id = $1
            "#,
            task_id
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(dependents)
    }

    async fn would_create_cycle(
        &self,
        task_id: i64,
        depends_on_task_id: i64,
    ) -> Result<bool, DevErpError> {
        // Get the complete dependency chain starting from depends_on_task_id
        let chain = self.get_dependency_chain(depends_on_task_id).await?;

        // If task_id is in the chain, adding this dependency would create a cycle
        Ok(chain.contains(&task_id))
    }

    async fn get_dependency_chain(&self, start_task_id: i64) -> Result<Vec<i64>, DevErpError> {
        // Use BFS to traverse the dependency graph
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        let mut chain = Vec::new();

        queue.push_back(start_task_id);
        visited.insert(start_task_id);

        while let Some(current_id) = queue.pop_front() {
            chain.push(current_id);

            // Get all tasks that current_id depends on
            let dependencies = self.get_dependencies(current_id).await?;

            for dep in dependencies {
                if !visited.contains(&dep.depends_on_task_id) {
                    visited.insert(dep.depends_on_task_id);
                    queue.push_back(dep.depends_on_task_id);
                }
            }
        }

        Ok(chain)
    }
}

/// PostgreSQL implementation of TaskCommentRepository
pub struct PostgresTaskCommentRepository {
    pool: PgPool,
}

impl PostgresTaskCommentRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl TaskCommentRepository for PostgresTaskCommentRepository {
    async fn create(&self, comment: CreateTaskComment) -> Result<TaskComment, DevErpError> {
        comment
            .validate()
            .map_err(|e| DevErpError::Validation(e))?;

        let task_comment = sqlx::query_as!(
            TaskComment,
            r#"
            INSERT INTO task_comments (task_id, comment_text, author)
            VALUES ($1, $2, $3)
            RETURNING id, task_id, comment_text, author, created_at, updated_at, deleted_at
            "#,
            comment.task_id,
            comment.comment_text,
            comment.author
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(task_comment)
    }

    async fn find_by_id(&self, id: i64) -> Result<Option<TaskComment>, DevErpError> {
        let comment = sqlx::query_as!(
            TaskComment,
            r#"
            SELECT id, task_id, comment_text, author, created_at, updated_at, deleted_at
            FROM task_comments
            WHERE id = $1 AND deleted_at IS NULL
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(comment)
    }

    async fn find_by_task_id(&self, task_id: i64) -> Result<Vec<TaskComment>, DevErpError> {
        let comments = sqlx::query_as!(
            TaskComment,
            r#"
            SELECT id, task_id, comment_text, author, created_at, updated_at, deleted_at
            FROM task_comments
            WHERE task_id = $1 AND deleted_at IS NULL
            ORDER BY created_at ASC
            "#,
            task_id
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(comments)
    }

    async fn update(&self, id: i64, comment_text: String) -> Result<TaskComment, DevErpError> {
        if comment_text.trim().is_empty() {
            return Err(DevErpError::Validation(
                "Comment text cannot be empty".to_string(),
            ));
        }

        let comment = sqlx::query_as!(
            TaskComment,
            r#"
            UPDATE task_comments
            SET comment_text = $1, updated_at = $2
            WHERE id = $3 AND deleted_at IS NULL
            RETURNING id, task_id, comment_text, author, created_at, updated_at, deleted_at
            "#,
            comment_text,
            Utc::now(),
            id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(comment)
    }

    async fn soft_delete(&self, id: i64) -> Result<bool, DevErpError> {
        let result = sqlx::query!(
            r#"
            UPDATE task_comments
            SET deleted_at = $1
            WHERE id = $2 AND deleted_at IS NULL
            "#,
            Utc::now(),
            id
        )
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected() > 0)
    }
}
