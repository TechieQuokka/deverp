use chrono::{NaiveDate, Utc};
use deverp::domain::project::entity::{CreateProject, ProjectStatus, Priority};
use deverp::domain::task::entity::{CreateTask, TaskStatus, TaskType, TaskPriority};
use deverp::domain::resource::entity::{CreateResource, ResourceType, ResourceStatus};
use deverp::domain::timeline::entity::{CreateTimeline, TimelineType, TimelineStatus};

/// Creates a test project with default values
pub fn create_test_project(name: &str) -> CreateProject {
    CreateProject {
        name: name.to_string(),
        description: Some(format!("Test project: {}", name)),
        code: Some(format!("TEST-{}", name.to_uppercase().replace(" ", "-"))),
        status: Some(ProjectStatus::Planning),
        priority: Some(Priority::Medium),
        start_date: Some(NaiveDate::from_ymd_opt(2025, 1, 1).unwrap()),
        end_date: Some(NaiveDate::from_ymd_opt(2025, 12, 31).unwrap()),
        repository_url: Some("https://github.com/test/repo".to_string()),
        repository_branch: Some("main".to_string()),
        tags: Some(vec!["test".to_string(), "integration".to_string()]),
        metadata: None,
    }
}

/// Creates a test task with default values
#[allow(dead_code)]
pub fn create_test_task(project_id: i64, title: &str) -> CreateTask {
    CreateTask {
        project_id,
        parent_task_id: None,
        title: title.to_string(),
        description: Some(format!("Test task: {}", title)),
        task_number: Some(format!("TASK-{}", title.to_uppercase().replace(" ", "-"))),
        status: Some(TaskStatus::Todo),
        priority: Some(TaskPriority::Medium),
        assigned_to: Some("test_user".to_string()),
        estimated_hours: Some(8.0),
        due_date: Some(Utc::now() + chrono::Duration::days(7)),
        task_type: Some(TaskType::Feature),
        tags: Some(vec!["test".to_string()]),
    }
}

/// Creates a test resource with default values
#[allow(dead_code)]
pub fn create_test_resource(name: &str) -> CreateResource {
    CreateResource {
        name: name.to_string(),
        description: Some(format!("Test resource: {}", name)),
        resource_type: ResourceType::Library,
        version: Some("1.0.0".to_string()),
        url: Some("https://example.com".to_string()),
        documentation_url: Some("https://example.com/docs".to_string()),
        license: Some("MIT".to_string()),
        status: Some(ResourceStatus::Active),
        metadata: None,
        tags: Some(vec!["test".to_string()]),
    }
}

/// Creates a test timeline with default values
#[allow(dead_code)]
pub fn create_test_timeline(project_id: i64, name: &str) -> CreateTimeline {
    CreateTimeline {
        project_id,
        name: name.to_string(),
        description: Some(format!("Test timeline: {}", name)),
        timeline_type: Some(TimelineType::Project),
        start_date: NaiveDate::from_ymd_opt(2025, 1, 1).unwrap(),
        end_date: NaiveDate::from_ymd_opt(2025, 12, 31).unwrap(),
        status: Some(TimelineStatus::Planned),
    }
}
