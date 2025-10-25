use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

/// Main Task entity representing a task in the system
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Task {
    pub id: i64,
    pub uuid: Uuid,
    pub project_id: i64,
    pub parent_task_id: Option<i64>,
    pub title: String,
    pub description: Option<String>,
    pub task_number: Option<String>,
    pub status: TaskStatus,
    pub priority: TaskPriority,
    pub assigned_to: Option<String>,
    pub estimated_hours: Option<f64>,
    pub actual_hours: Option<f64>,
    pub due_date: Option<DateTime<Utc>>,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub task_type: Option<TaskType>,
    pub tags: Option<Vec<String>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

/// Task status enum
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq, Eq)]
#[sqlx(type_name = "VARCHAR", rename_all = "snake_case")]
#[derive(Default)]
pub enum TaskStatus {
    #[default]
    Todo,
    InProgress,
    Blocked,
    Review,
    Testing,
    Done,
    Cancelled,
}

impl std::fmt::Display for TaskStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TaskStatus::Todo => write!(f, "todo"),
            TaskStatus::InProgress => write!(f, "in_progress"),
            TaskStatus::Blocked => write!(f, "blocked"),
            TaskStatus::Review => write!(f, "review"),
            TaskStatus::Testing => write!(f, "testing"),
            TaskStatus::Done => write!(f, "done"),
            TaskStatus::Cancelled => write!(f, "cancelled"),
        }
    }
}

impl std::str::FromStr for TaskStatus {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "todo" => Ok(TaskStatus::Todo),
            "in_progress" => Ok(TaskStatus::InProgress),
            "blocked" => Ok(TaskStatus::Blocked),
            "review" => Ok(TaskStatus::Review),
            "testing" => Ok(TaskStatus::Testing),
            "done" => Ok(TaskStatus::Done),
            "cancelled" => Ok(TaskStatus::Cancelled),
            _ => Err(format!("Invalid task status: {}", s)),
        }
    }
}

/// Task priority enum
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq, Eq)]
#[sqlx(type_name = "VARCHAR", rename_all = "lowercase")]
#[derive(Default)]
pub enum TaskPriority {
    Low,
    #[default]
    Medium,
    High,
    Critical,
}

impl std::fmt::Display for TaskPriority {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TaskPriority::Low => write!(f, "low"),
            TaskPriority::Medium => write!(f, "medium"),
            TaskPriority::High => write!(f, "high"),
            TaskPriority::Critical => write!(f, "critical"),
        }
    }
}

impl std::str::FromStr for TaskPriority {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "low" => Ok(TaskPriority::Low),
            "medium" => Ok(TaskPriority::Medium),
            "high" => Ok(TaskPriority::High),
            "critical" => Ok(TaskPriority::Critical),
            _ => Err(format!("Invalid task priority: {}", s)),
        }
    }
}

/// Task type enum
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq, Eq)]
#[sqlx(type_name = "VARCHAR", rename_all = "lowercase")]
#[derive(Default)]
pub enum TaskType {
    #[default]
    Feature,
    Bug,
    Enhancement,
    Refactor,
    Docs,
    Test,
    Chore,
}

impl std::fmt::Display for TaskType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TaskType::Feature => write!(f, "feature"),
            TaskType::Bug => write!(f, "bug"),
            TaskType::Enhancement => write!(f, "enhancement"),
            TaskType::Refactor => write!(f, "refactor"),
            TaskType::Docs => write!(f, "docs"),
            TaskType::Test => write!(f, "test"),
            TaskType::Chore => write!(f, "chore"),
        }
    }
}

impl std::str::FromStr for TaskType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "feature" => Ok(TaskType::Feature),
            "bug" => Ok(TaskType::Bug),
            "enhancement" => Ok(TaskType::Enhancement),
            "refactor" => Ok(TaskType::Refactor),
            "docs" => Ok(TaskType::Docs),
            "test" => Ok(TaskType::Test),
            "chore" => Ok(TaskType::Chore),
            _ => Err(format!("Invalid task type: {}", s)),
        }
    }
}

/// Task dependency entity
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct TaskDependency {
    pub task_id: i64,
    pub depends_on_task_id: i64,
    pub dependency_type: DependencyType,
    pub created_at: DateTime<Utc>,
}

/// Dependency type enum
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq, Eq)]
#[sqlx(type_name = "VARCHAR", rename_all = "snake_case")]
#[derive(Default)]
pub enum DependencyType {
    #[default]
    FinishToStart,
    StartToStart,
    FinishToFinish,
    StartToFinish,
}

impl std::fmt::Display for DependencyType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DependencyType::FinishToStart => write!(f, "finish_to_start"),
            DependencyType::StartToStart => write!(f, "start_to_start"),
            DependencyType::FinishToFinish => write!(f, "finish_to_finish"),
            DependencyType::StartToFinish => write!(f, "start_to_finish"),
        }
    }
}

impl std::str::FromStr for DependencyType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "finish_to_start" => Ok(DependencyType::FinishToStart),
            "start_to_start" => Ok(DependencyType::StartToStart),
            "finish_to_finish" => Ok(DependencyType::FinishToFinish),
            "start_to_finish" => Ok(DependencyType::StartToFinish),
            _ => Err(format!("Invalid dependency type: {}", s)),
        }
    }
}

/// Task comment entity
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct TaskComment {
    pub id: i64,
    pub task_id: i64,
    pub comment_text: String,
    pub author: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

/// Input structure for creating a new task
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTask {
    pub project_id: i64,
    pub parent_task_id: Option<i64>,
    pub title: String,
    pub description: Option<String>,
    pub task_number: Option<String>,
    pub status: Option<TaskStatus>,
    pub priority: Option<TaskPriority>,
    pub assigned_to: Option<String>,
    pub estimated_hours: Option<f64>,
    pub due_date: Option<DateTime<Utc>>,
    pub task_type: Option<TaskType>,
    pub tags: Option<Vec<String>>,
}

impl CreateTask {
    /// Validate the create task input
    pub fn validate(&self) -> Result<(), String> {
        if self.title.trim().is_empty() {
            return Err("Task title cannot be empty".to_string());
        }

        if self.title.len() > 500 {
            return Err("Task title cannot exceed 500 characters".to_string());
        }

        if let Some(hours) = self.estimated_hours {
            if hours < 0.0 {
                return Err("Estimated hours cannot be negative".to_string());
            }
        }

        Ok(())
    }
}

/// Input structure for updating a task
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateTask {
    pub id: i64,
    pub title: Option<String>,
    pub description: Option<String>,
    pub status: Option<TaskStatus>,
    pub priority: Option<TaskPriority>,
    pub assigned_to: Option<String>,
    pub estimated_hours: Option<f64>,
    pub actual_hours: Option<f64>,
    pub due_date: Option<DateTime<Utc>>,
    pub task_type: Option<TaskType>,
    pub tags: Option<Vec<String>>,
}

impl UpdateTask {
    /// Validate the update task input
    pub fn validate(&self) -> Result<(), String> {
        if let Some(ref title) = self.title {
            if title.trim().is_empty() {
                return Err("Task title cannot be empty".to_string());
            }
            if title.len() > 500 {
                return Err("Task title cannot exceed 500 characters".to_string());
            }
        }

        if let Some(hours) = self.estimated_hours {
            if hours < 0.0 {
                return Err("Estimated hours cannot be negative".to_string());
            }
        }

        if let Some(hours) = self.actual_hours {
            if hours < 0.0 {
                return Err("Actual hours cannot be negative".to_string());
            }
        }

        Ok(())
    }
}

/// Filter for querying tasks
#[derive(Debug, Clone, Default)]
pub struct TaskFilter {
    pub project_id: Option<i64>,
    pub status: Option<TaskStatus>,
    pub priority: Option<TaskPriority>,
    pub task_type: Option<TaskType>,
    pub assigned_to: Option<String>,
    pub parent_task_id: Option<i64>,
    pub include_deleted: bool,
    pub offset: Option<i64>,
    pub limit: Option<i64>,
}

/// Input for creating a task comment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTaskComment {
    pub task_id: i64,
    pub comment_text: String,
    pub author: Option<String>,
}

impl CreateTaskComment {
    /// Validate the create comment input
    pub fn validate(&self) -> Result<(), String> {
        if self.comment_text.trim().is_empty() {
            return Err("Comment text cannot be empty".to_string());
        }

        Ok(())
    }
}

/// Input for adding a task dependency
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTaskDependency {
    pub task_id: i64,
    pub depends_on_task_id: i64,
    pub dependency_type: Option<DependencyType>,
}

impl CreateTaskDependency {
    /// Validate the create dependency input
    pub fn validate(&self) -> Result<(), String> {
        if self.task_id == self.depends_on_task_id {
            return Err("A task cannot depend on itself".to_string());
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task_status_parsing() {
        assert_eq!("todo".parse::<TaskStatus>().unwrap(), TaskStatus::Todo);
        assert_eq!(
            "in_progress".parse::<TaskStatus>().unwrap(),
            TaskStatus::InProgress
        );
        assert_eq!(
            "blocked".parse::<TaskStatus>().unwrap(),
            TaskStatus::Blocked
        );
        assert!("invalid".parse::<TaskStatus>().is_err());
    }

    #[test]
    fn test_task_priority_parsing() {
        assert_eq!("low".parse::<TaskPriority>().unwrap(), TaskPriority::Low);
        assert_eq!(
            "medium".parse::<TaskPriority>().unwrap(),
            TaskPriority::Medium
        );
        assert_eq!("high".parse::<TaskPriority>().unwrap(), TaskPriority::High);
        assert_eq!(
            "critical".parse::<TaskPriority>().unwrap(),
            TaskPriority::Critical
        );
        assert!("invalid".parse::<TaskPriority>().is_err());
    }

    #[test]
    fn test_task_type_parsing() {
        assert_eq!("feature".parse::<TaskType>().unwrap(), TaskType::Feature);
        assert_eq!("bug".parse::<TaskType>().unwrap(), TaskType::Bug);
        assert!("invalid".parse::<TaskType>().is_err());
    }

    #[test]
    fn test_dependency_type_parsing() {
        assert_eq!(
            "finish_to_start".parse::<DependencyType>().unwrap(),
            DependencyType::FinishToStart
        );
        assert_eq!(
            "start_to_start".parse::<DependencyType>().unwrap(),
            DependencyType::StartToStart
        );
        assert!("invalid".parse::<DependencyType>().is_err());
    }

    #[test]
    fn test_create_task_validation() {
        let valid_task = CreateTask {
            project_id: 1,
            parent_task_id: None,
            title: "Valid Task".to_string(),
            description: None,
            task_number: None,
            status: None,
            priority: None,
            assigned_to: None,
            estimated_hours: Some(5.0),
            due_date: None,
            task_type: None,
            tags: None,
        };
        assert!(valid_task.validate().is_ok());

        let empty_title = CreateTask {
            title: "".to_string(),
            ..valid_task.clone()
        };
        assert!(empty_title.validate().is_err());

        let negative_hours = CreateTask {
            estimated_hours: Some(-1.0),
            ..valid_task.clone()
        };
        assert!(negative_hours.validate().is_err());
    }

    #[test]
    fn test_create_dependency_validation() {
        let valid_dep = CreateTaskDependency {
            task_id: 1,
            depends_on_task_id: 2,
            dependency_type: None,
        };
        assert!(valid_dep.validate().is_ok());

        let self_dep = CreateTaskDependency {
            task_id: 1,
            depends_on_task_id: 1,
            dependency_type: None,
        };
        assert!(self_dep.validate().is_err());
    }

    #[test]
    fn test_create_comment_validation() {
        let valid_comment = CreateTaskComment {
            task_id: 1,
            comment_text: "This is a comment".to_string(),
            author: Some("user".to_string()),
        };
        assert!(valid_comment.validate().is_ok());

        let empty_comment = CreateTaskComment {
            task_id: 1,
            comment_text: "".to_string(),
            author: None,
        };
        assert!(empty_comment.validate().is_err());
    }
}
