use async_trait::async_trait;
use uuid::Uuid;

use crate::utils::error::DevErpError;

use super::entity::{
    CreateTask, CreateTaskComment, CreateTaskDependency, Task, TaskComment, TaskDependency,
    TaskFilter, UpdateTask,
};

/// Repository trait for Task operations
#[async_trait]
pub trait TaskRepository: Send + Sync {
    /// Create a new task
    async fn create(&self, task: CreateTask) -> Result<Task, DevErpError>;

    /// Find a task by its internal ID
    async fn find_by_id(&self, id: i64) -> Result<Option<Task>, DevErpError>;

    /// Find a task by its UUID
    async fn find_by_uuid(&self, uuid: Uuid) -> Result<Option<Task>, DevErpError>;

    /// Find all tasks matching the given filter
    async fn find_all(&self, filter: TaskFilter) -> Result<Vec<Task>, DevErpError>;

    /// Update an existing task
    async fn update(&self, task: UpdateTask) -> Result<Task, DevErpError>;

    /// Hard delete a task (use with caution)
    async fn delete(&self, id: i64) -> Result<bool, DevErpError>;

    /// Soft delete a task (sets deleted_at timestamp)
    async fn soft_delete(&self, id: i64) -> Result<bool, DevErpError>;

    /// Count tasks matching the filter
    async fn count(&self, filter: TaskFilter) -> Result<i64, DevErpError>;
}

/// Repository trait for Task Dependency operations
#[async_trait]
pub trait TaskDependencyRepository: Send + Sync {
    /// Add a dependency between two tasks
    async fn add_dependency(
        &self,
        dependency: CreateTaskDependency,
    ) -> Result<TaskDependency, DevErpError>;

    /// Remove a dependency between two tasks
    async fn remove_dependency(
        &self,
        task_id: i64,
        depends_on_task_id: i64,
    ) -> Result<bool, DevErpError>;

    /// Get all dependencies for a specific task
    async fn get_dependencies(&self, task_id: i64) -> Result<Vec<TaskDependency>, DevErpError>;

    /// Get all tasks that depend on a specific task
    async fn get_dependents(&self, task_id: i64) -> Result<Vec<TaskDependency>, DevErpError>;

    /// Check if adding a dependency would create a cycle
    async fn would_create_cycle(
        &self,
        task_id: i64,
        depends_on_task_id: i64,
    ) -> Result<bool, DevErpError>;

    /// Get all task IDs in the dependency path from start_task_id
    async fn get_dependency_chain(&self, start_task_id: i64) -> Result<Vec<i64>, DevErpError>;
}

/// Repository trait for Task Comment operations
#[async_trait]
pub trait TaskCommentRepository: Send + Sync {
    /// Create a new comment on a task
    async fn create(&self, comment: CreateTaskComment) -> Result<TaskComment, DevErpError>;

    /// Find a comment by its ID
    async fn find_by_id(&self, id: i64) -> Result<Option<TaskComment>, DevErpError>;

    /// Find all comments for a specific task
    async fn find_by_task_id(&self, task_id: i64) -> Result<Vec<TaskComment>, DevErpError>;

    /// Update a comment
    async fn update(&self, id: i64, comment_text: String) -> Result<TaskComment, DevErpError>;

    /// Soft delete a comment
    async fn soft_delete(&self, id: i64) -> Result<bool, DevErpError>;
}
