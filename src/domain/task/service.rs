use std::sync::Arc;
use tracing::{debug, info, warn};
use uuid::Uuid;

use crate::utils::error::DevErpError;

use super::entity::{
    CreateTask, CreateTaskComment, CreateTaskDependency, Task, TaskComment, TaskDependency,
    TaskFilter, TaskStatus, UpdateTask,
};
use super::repository::{TaskCommentRepository, TaskDependencyRepository, TaskRepository};

/// Task service containing business logic for task management
pub struct TaskService {
    task_repo: Arc<dyn TaskRepository>,
    dependency_repo: Arc<dyn TaskDependencyRepository>,
    comment_repo: Arc<dyn TaskCommentRepository>,
}

impl TaskService {
    /// Create a new TaskService with the given repositories
    pub fn new(
        task_repo: Arc<dyn TaskRepository>,
        dependency_repo: Arc<dyn TaskDependencyRepository>,
        comment_repo: Arc<dyn TaskCommentRepository>,
    ) -> Self {
        Self {
            task_repo,
            dependency_repo,
            comment_repo,
        }
    }

    /// Create a new task
    pub async fn create_task(&self, input: CreateTask) -> Result<Task, DevErpError> {
        debug!("Creating new task: {}", input.title);

        // Validate input
        input.validate().map_err(DevErpError::Validation)?;

        // If parent_task_id is provided, verify it exists
        if let Some(parent_id) = input.parent_task_id {
            let parent = self.task_repo.find_by_id(parent_id).await?;
            if parent.is_none() {
                return Err(DevErpError::NotFound(format!(
                    "Parent task with id {} not found",
                    parent_id
                )));
            }
        }

        let task = self.task_repo.create(input).await?;

        info!(task_id = %task.id, task_uuid = %task.uuid, "Task created successfully");

        Ok(task)
    }

    /// Get a task by its ID
    pub async fn get_task_by_id(&self, id: i64) -> Result<Task, DevErpError> {
        debug!("Fetching task with id: {}", id);

        let task = self
            .task_repo
            .find_by_id(id)
            .await?
            .ok_or_else(|| DevErpError::NotFound(format!("Task with id {} not found", id)))?;

        Ok(task)
    }

    /// Get a task by its UUID
    pub async fn get_task_by_uuid(&self, uuid: Uuid) -> Result<Task, DevErpError> {
        debug!("Fetching task with uuid: {}", uuid);

        let task = self.task_repo.find_by_uuid(uuid).await?.ok_or_else(|| {
            DevErpError::NotFound(format!("Task with uuid {} not found", uuid))
        })?;

        Ok(task)
    }

    /// List tasks with optional filtering
    pub async fn list_tasks(&self, filter: TaskFilter) -> Result<Vec<Task>, DevErpError> {
        debug!("Listing tasks with filter: {:?}", filter);

        let tasks = self.task_repo.find_all(filter).await?;

        info!("Retrieved {} tasks", tasks.len());

        Ok(tasks)
    }

    /// Update a task
    pub async fn update_task(&self, input: UpdateTask) -> Result<Task, DevErpError> {
        debug!("Updating task with id: {}", input.id);

        // Validate input
        input.validate().map_err(DevErpError::Validation)?;

        // Verify task exists
        let existing_task = self.get_task_by_id(input.id).await?;

        // If status is being updated to 'done', set completed_at if not already set
        let updated_task = if let Some(ref new_status) = input.status {
            if *new_status == TaskStatus::Done && existing_task.completed_at.is_none() {
                info!(task_id = %input.id, "Marking task as completed");
            } else if *new_status != TaskStatus::Done && existing_task.completed_at.is_some() {
                warn!(task_id = %input.id, "Changing status from done to {} - completed_at will remain set", new_status);
            }

            self.task_repo.update(input).await?
        } else {
            self.task_repo.update(input).await?
        };

        info!(task_id = %updated_task.id, "Task updated successfully");

        Ok(updated_task)
    }

    /// Change task status with validation
    pub async fn change_task_status(
        &self,
        task_id: i64,
        new_status: TaskStatus,
    ) -> Result<Task, DevErpError> {
        debug!(
            "Changing status of task {} to {}",
            task_id, new_status
        );

        let task = self.get_task_by_id(task_id).await?;

        // Check if status transition is valid
        if !self.is_valid_status_transition(&task.status, &new_status) {
            return Err(DevErpError::Validation(format!(
                "Invalid status transition from {} to {}",
                task.status, new_status
            )));
        }

        // If changing to 'in_progress' and started_at is not set, we could set it
        // If changing to 'done' and completed_at is not set, we could set it
        // But for now, we'll just update the status

        let update = UpdateTask {
            id: task_id,
            status: Some(new_status.clone()),
            title: None,
            description: None,
            priority: None,
            assigned_to: None,
            estimated_hours: None,
            actual_hours: None,
            due_date: None,
            task_type: None,
            tags: None,
        };

        let updated_task = self.task_repo.update(update).await?;

        info!(
            task_id = %task_id,
            old_status = %task.status,
            new_status = %new_status,
            "Task status changed successfully"
        );

        Ok(updated_task)
    }

    /// Validate status transitions
    fn is_valid_status_transition(
        &self,
        from_status: &TaskStatus,
        to_status: &TaskStatus,
    ) -> bool {
        use TaskStatus::*;

        // Define valid transitions
        match (from_status, to_status) {
            // From Todo
            (Todo, InProgress) => true,
            (Todo, Blocked) => true,
            (Todo, Cancelled) => true,

            // From InProgress
            (InProgress, Todo) => true,
            (InProgress, Blocked) => true,
            (InProgress, Review) => true,
            (InProgress, Testing) => true,
            (InProgress, Done) => true,
            (InProgress, Cancelled) => true,

            // From Blocked
            (Blocked, Todo) => true,
            (Blocked, InProgress) => true,
            (Blocked, Cancelled) => true,

            // From Review
            (Review, InProgress) => true,
            (Review, Testing) => true,
            (Review, Done) => true,
            (Review, Cancelled) => true,

            // From Testing
            (Testing, InProgress) => true,
            (Testing, Review) => true,
            (Testing, Done) => true,
            (Testing, Cancelled) => true,

            // From Done
            (Done, InProgress) => true, // Allow reopening

            // From Cancelled
            (Cancelled, Todo) => true, // Allow reactivation

            // Same status is always valid
            _ if from_status == to_status => true,

            // All other transitions are invalid
            _ => false,
        }
    }

    /// Delete a task (soft delete)
    pub async fn delete_task(&self, id: i64) -> Result<(), DevErpError> {
        debug!("Soft deleting task with id: {}", id);

        // Verify task exists
        let _task = self.get_task_by_id(id).await?;

        let deleted = self.task_repo.soft_delete(id).await?;

        if !deleted {
            return Err(DevErpError::NotFound(format!(
                "Task with id {} not found",
                id
            )));
        }

        info!(task_id = %id, "Task soft deleted successfully");

        Ok(())
    }

    /// Count tasks matching a filter
    pub async fn count_tasks(&self, filter: TaskFilter) -> Result<i64, DevErpError> {
        self.task_repo.count(filter).await
    }

    // ===== Task Dependency Management =====

    /// Add a dependency between two tasks
    pub async fn add_task_dependency(
        &self,
        dependency: CreateTaskDependency,
    ) -> Result<TaskDependency, DevErpError> {
        debug!(
            "Adding dependency: task {} depends on task {}",
            dependency.task_id, dependency.depends_on_task_id
        );

        // Validate input
        dependency
            .validate()
            .map_err(DevErpError::Validation)?;

        // Verify both tasks exist
        let _task = self.get_task_by_id(dependency.task_id).await?;
        let _depends_on = self.get_task_by_id(dependency.depends_on_task_id).await?;

        // Check if this would create a circular dependency
        if self
            .dependency_repo
            .would_create_cycle(dependency.task_id, dependency.depends_on_task_id)
            .await?
        {
            return Err(DevErpError::Validation(
                "Adding this dependency would create a circular dependency".to_string(),
            ));
        }

        let task_dep = self.dependency_repo.add_dependency(dependency).await?;

        info!(
            task_id = %task_dep.task_id,
            depends_on = %task_dep.depends_on_task_id,
            "Task dependency added successfully"
        );

        Ok(task_dep)
    }

    /// Remove a dependency between two tasks
    pub async fn remove_task_dependency(
        &self,
        task_id: i64,
        depends_on_task_id: i64,
    ) -> Result<(), DevErpError> {
        debug!(
            "Removing dependency: task {} depends on task {}",
            task_id, depends_on_task_id
        );

        let removed = self
            .dependency_repo
            .remove_dependency(task_id, depends_on_task_id)
            .await?;

        if !removed {
            return Err(DevErpError::NotFound(
                "Dependency not found".to_string(),
            ));
        }

        info!(
            task_id = %task_id,
            depends_on = %depends_on_task_id,
            "Task dependency removed successfully"
        );

        Ok(())
    }

    /// Get all dependencies for a task
    pub async fn get_task_dependencies(&self, task_id: i64) -> Result<Vec<TaskDependency>, DevErpError> {
        debug!("Fetching dependencies for task {}", task_id);

        let dependencies = self.dependency_repo.get_dependencies(task_id).await?;

        info!(
            task_id = %task_id,
            count = dependencies.len(),
            "Retrieved task dependencies"
        );

        Ok(dependencies)
    }

    /// Get all tasks that depend on a specific task
    pub async fn get_task_dependents(&self, task_id: i64) -> Result<Vec<TaskDependency>, DevErpError> {
        debug!("Fetching tasks that depend on task {}", task_id);

        let dependents = self.dependency_repo.get_dependents(task_id).await?;

        info!(
            task_id = %task_id,
            count = dependents.len(),
            "Retrieved dependent tasks"
        );

        Ok(dependents)
    }

    /// Check if adding a dependency would create a cycle
    pub async fn would_create_cycle(
        &self,
        task_id: i64,
        depends_on_task_id: i64,
    ) -> Result<bool, DevErpError> {
        self.dependency_repo
            .would_create_cycle(task_id, depends_on_task_id)
            .await
    }

    // ===== Task Comment Management =====

    /// Add a comment to a task
    pub async fn add_task_comment(
        &self,
        comment: CreateTaskComment,
    ) -> Result<TaskComment, DevErpError> {
        debug!("Adding comment to task {}", comment.task_id);

        // Validate input
        comment.validate().map_err(DevErpError::Validation)?;

        // Verify task exists
        let _task = self.get_task_by_id(comment.task_id).await?;

        let task_comment = self.comment_repo.create(comment).await?;

        info!(
            comment_id = %task_comment.id,
            task_id = %task_comment.task_id,
            "Task comment added successfully"
        );

        Ok(task_comment)
    }

    /// Get all comments for a task
    pub async fn get_task_comments(&self, task_id: i64) -> Result<Vec<TaskComment>, DevErpError> {
        debug!("Fetching comments for task {}", task_id);

        let comments = self.comment_repo.find_by_task_id(task_id).await?;

        info!(
            task_id = %task_id,
            count = comments.len(),
            "Retrieved task comments"
        );

        Ok(comments)
    }

    /// Update a task comment
    pub async fn update_task_comment(
        &self,
        comment_id: i64,
        comment_text: String,
    ) -> Result<TaskComment, DevErpError> {
        debug!("Updating comment {}", comment_id);

        if comment_text.trim().is_empty() {
            return Err(DevErpError::Validation(
                "Comment text cannot be empty".to_string(),
            ));
        }

        let comment = self.comment_repo.update(comment_id, comment_text).await?;

        info!(comment_id = %comment_id, "Task comment updated successfully");

        Ok(comment)
    }

    /// Delete a task comment (soft delete)
    pub async fn delete_task_comment(&self, comment_id: i64) -> Result<(), DevErpError> {
        debug!("Soft deleting comment {}", comment_id);

        let deleted = self.comment_repo.soft_delete(comment_id).await?;

        if !deleted {
            return Err(DevErpError::NotFound(format!(
                "Comment with id {} not found",
                comment_id
            )));
        }

        info!(comment_id = %comment_id, "Task comment soft deleted successfully");

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_status_transitions() {
        let service = TaskService::new(
            Arc::new(MockTaskRepository),
            Arc::new(MockDependencyRepository),
            Arc::new(MockCommentRepository),
        );

        // Valid transitions
        assert!(service.is_valid_status_transition(&TaskStatus::Todo, &TaskStatus::InProgress));
        assert!(service.is_valid_status_transition(
            &TaskStatus::InProgress,
            &TaskStatus::Review
        ));
        assert!(service.is_valid_status_transition(&TaskStatus::Review, &TaskStatus::Done));
        assert!(service.is_valid_status_transition(&TaskStatus::Done, &TaskStatus::InProgress)); // Reopening

        // Invalid transitions
        assert!(!service.is_valid_status_transition(&TaskStatus::Todo, &TaskStatus::Done));
        assert!(!service.is_valid_status_transition(&TaskStatus::Todo, &TaskStatus::Review));
    }

    // Mock implementations for testing
    struct MockTaskRepository;
    #[async_trait::async_trait]
    impl TaskRepository for MockTaskRepository {
        async fn create(&self, _task: CreateTask) -> Result<Task, DevErpError> {
            unimplemented!()
        }
        async fn find_by_id(&self, _id: i64) -> Result<Option<Task>, DevErpError> {
            unimplemented!()
        }
        async fn find_by_uuid(&self, _uuid: Uuid) -> Result<Option<Task>, DevErpError> {
            unimplemented!()
        }
        async fn find_all(&self, _filter: TaskFilter) -> Result<Vec<Task>, DevErpError> {
            unimplemented!()
        }
        async fn update(&self, _task: UpdateTask) -> Result<Task, DevErpError> {
            unimplemented!()
        }
        async fn delete(&self, _id: i64) -> Result<bool, DevErpError> {
            unimplemented!()
        }
        async fn soft_delete(&self, _id: i64) -> Result<bool, DevErpError> {
            unimplemented!()
        }
        async fn count(&self, _filter: TaskFilter) -> Result<i64, DevErpError> {
            unimplemented!()
        }
    }

    struct MockDependencyRepository;
    #[async_trait::async_trait]
    impl TaskDependencyRepository for MockDependencyRepository {
        async fn add_dependency(
            &self,
            _dependency: CreateTaskDependency,
        ) -> Result<TaskDependency, DevErpError> {
            unimplemented!()
        }
        async fn remove_dependency(
            &self,
            _task_id: i64,
            _depends_on_task_id: i64,
        ) -> Result<bool, DevErpError> {
            unimplemented!()
        }
        async fn get_dependencies(&self, _task_id: i64) -> Result<Vec<TaskDependency>, DevErpError> {
            unimplemented!()
        }
        async fn get_dependents(&self, _task_id: i64) -> Result<Vec<TaskDependency>, DevErpError> {
            unimplemented!()
        }
        async fn would_create_cycle(
            &self,
            _task_id: i64,
            _depends_on_task_id: i64,
        ) -> Result<bool, DevErpError> {
            unimplemented!()
        }
        async fn get_dependency_chain(&self, _start_task_id: i64) -> Result<Vec<i64>, DevErpError> {
            unimplemented!()
        }
    }

    struct MockCommentRepository;
    #[async_trait::async_trait]
    impl TaskCommentRepository for MockCommentRepository {
        async fn create(&self, _comment: CreateTaskComment) -> Result<TaskComment, DevErpError> {
            unimplemented!()
        }
        async fn find_by_id(&self, _id: i64) -> Result<Option<TaskComment>, DevErpError> {
            unimplemented!()
        }
        async fn find_by_task_id(&self, _task_id: i64) -> Result<Vec<TaskComment>, DevErpError> {
            unimplemented!()
        }
        async fn update(
            &self,
            _id: i64,
            _comment_text: String,
        ) -> Result<TaskComment, DevErpError> {
            unimplemented!()
        }
        async fn soft_delete(&self, _id: i64) -> Result<bool, DevErpError> {
            unimplemented!()
        }
    }
}
