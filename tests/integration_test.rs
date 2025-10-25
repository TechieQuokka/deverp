// Integration tests for DevERP
// These tests require a running PostgreSQL database
// Set DATABASE_URL environment variable or use default test database

mod helpers;

use deverp::domain::project::entity::ProjectFilter;
use deverp::domain::project::service::ProjectService;
use deverp::domain::task::service::TaskService;
use deverp::infrastructure::repositories::project_repo::PostgresProjectRepository;
use deverp::infrastructure::repositories::task_repo::{
    PostgresTaskCommentRepository, PostgresTaskDependencyRepository, PostgresTaskRepository,
};
use helpers::*;
use std::sync::Arc;

/// Basic integration test: Create and retrieve a project
#[tokio::test]
async fn test_create_and_get_project() {
    let pool = setup_test_database()
        .await
        .expect("Failed to setup test database");
    let project_repo = Arc::new(PostgresProjectRepository::new(pool.clone()));
    let project_service = ProjectService::new(project_repo);

    // Create a project
    let create_input = create_test_project("Integration Test Project");
    let created = project_service
        .create_project(create_input)
        .await
        .expect("Failed to create project");

    assert!(!created.name.is_empty());
    assert!(created.id > 0);

    // Retrieve the project
    let retrieved = project_service
        .get_project(created.id)
        .await
        .expect("Failed to get project");

    assert_eq!(retrieved.id, created.id);
    assert_eq!(retrieved.name, created.name);

    println!("✅ Basic integration test passed");
}

/// Test project listing with filters
#[tokio::test]
async fn test_list_projects() {
    let pool = setup_test_database()
        .await
        .expect("Failed to setup test database");
    let project_repo = Arc::new(PostgresProjectRepository::new(pool.clone()));
    let project_service = ProjectService::new(project_repo);

    // Create multiple projects
    for i in 1..=5 {
        let input = create_test_project(&format!("List Test Project {}", i));
        project_service
            .create_project(input)
            .await
            .expect("Failed to create project");
    }

    // List all projects
    let filter = ProjectFilter {
        status: None,
        priority: None,
        search: None,
        tags: None,
        offset: None,
        limit: None,
    };

    let projects = project_service
        .list_projects(filter)
        .await
        .expect("Failed to list projects");

    assert!(projects.len() >= 5, "Should have at least 5 projects");

    println!("✅ Project listing test passed");
}

/// Test task creation and association with project
#[tokio::test]
async fn test_create_task_for_project() {
    let pool = setup_test_database()
        .await
        .expect("Failed to setup test database");

    let project_repo = Arc::new(PostgresProjectRepository::new(pool.clone()));
    let task_repo = Arc::new(PostgresTaskRepository::new(pool.clone()));
    let dependency_repo = Arc::new(PostgresTaskDependencyRepository::new(pool.clone()));
    let comment_repo = Arc::new(PostgresTaskCommentRepository::new(pool.clone()));

    let project_service = ProjectService::new(project_repo);
    let task_service = TaskService::new(task_repo, dependency_repo, comment_repo);

    // Create a project
    let project = project_service
        .create_project(create_test_project("Task Test Project"))
        .await
        .expect("Failed to create project");

    // Create a task for the project
    let task_input = create_test_task(project.id, "Integration Test Task");
    let task = task_service
        .create_task(task_input)
        .await
        .expect("Failed to create task");

    assert_eq!(task.project_id, project.id);
    assert!(!task.title.is_empty());

    println!("✅ Task creation test passed");
}

/// Test basic CRUD operations
#[tokio::test]
async fn test_project_crud() {
    let pool = setup_test_database()
        .await
        .expect("Failed to setup test database");
    let project_repo = Arc::new(PostgresProjectRepository::new(pool.clone()));
    let project_service = ProjectService::new(project_repo);

    // Create
    let create_input = create_test_project("CRUD Test Project");
    let created = project_service
        .create_project(create_input)
        .await
        .expect("Failed to create project");

    // Read
    let read = project_service
        .get_project(created.id)
        .await
        .expect("Failed to read project");

    assert_eq!(read.id, created.id);

    // Update - using the UpdateProject structure
    use deverp::domain::project::entity::UpdateProject;
    let update = UpdateProject {
        id: created.id,
        name: Some("Updated CRUD Project".to_string()),
        description: None,
        code: None,
        status: None,
        priority: None,
        start_date: None,
        end_date: None,
        actual_start_date: None,
        actual_end_date: None,
        progress_percentage: Some(50),
        repository_url: None,
        repository_branch: None,
        tags: None,
        metadata: None,
    };

    let updated = project_service
        .update_project(update)
        .await
        .expect("Failed to update project");

    assert_eq!(updated.name, "Updated CRUD Project");
    assert_eq!(updated.progress_percentage, Some(50));

    // Delete (soft delete)
    let deleted = project_service
        .delete_project(created.id)
        .await
        .expect("Failed to delete project");

    assert!(deleted, "Delete should return true");

    println!("✅ CRUD operations test passed");
}

/// Test database connection and migration
#[tokio::test]
async fn test_database_connection() {
    let result = setup_test_database().await;
    assert!(result.is_ok(), "Database connection should succeed");

    let pool = result.unwrap();

    // Verify we can execute a simple query
    let row: (i32,) = sqlx::query_as("SELECT 1")
        .fetch_one(&pool)
        .await
        .expect("Failed to execute test query");

    assert_eq!(row.0, 1);

    println!("✅ Database connection test passed");
}
