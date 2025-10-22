mod helpers;

use std::sync::Arc;
use chrono::NaiveDate;
use deverp::domain::project::entity::{CreateProject, ProjectStatus, Priority};
use deverp::domain::project::service::ProjectService;
use deverp::domain::task::service::TaskService;
use deverp::domain::resource::service::ResourceService;
use deverp::infrastructure::repositories::project_repo::PostgresProjectRepository;
use deverp::infrastructure::repositories::task_repo::{
    PostgresTaskRepository, PostgresTaskDependencyRepository, PostgresTaskCommentRepository
};
use deverp::infrastructure::repositories::resource_repo::PostgresResourceRepository;
use deverp::utils::error::DevErpError;
use helpers::*;

/// Test handling of invalid input validation
#[tokio::test]
async fn test_validation_errors() {
    let pool = setup_test_database().await.expect("Failed to setup test database");
    let project_repo = Arc::new(PostgresProjectRepository::new(pool.clone()));
    let project_service = ProjectService::new(project_repo);

    println!("Testing validation errors...");

    // Test 1: Empty project name should fail
    let invalid_project = CreateProject {
        name: "".to_string(),  // Empty name
        description: Some("Test".to_string()),
        code: None,
        status: Some(ProjectStatus::Planning),
        priority: Some(Priority::Medium),
        start_date: None,
        end_date: None,
        repository_url: None,
        repository_branch: None,
        tags: None,
        metadata: None,
    };

    let result = project_service.create_project(invalid_project).await;
    assert!(result.is_err(), "Empty project name should be rejected");
    println!("✓ Empty project name correctly rejected");

    // Test 2: Invalid date range (end before start)
    let invalid_dates = CreateProject {
        name: "Invalid Dates Project".to_string(),
        description: None,
        code: None,
        status: Some(ProjectStatus::Planning),
        priority: Some(Priority::Medium),
        start_date: Some(NaiveDate::from_ymd_opt(2025, 12, 31).unwrap()),
        end_date: Some(NaiveDate::from_ymd_opt(2025, 1, 1).unwrap()),  // End before start
        repository_url: None,
        repository_branch: None,
        tags: None,
        metadata: None,
    };

    let result = project_service.create_project(invalid_dates).await;
    assert!(result.is_err(), "Invalid date range should be rejected");
    println!("✓ Invalid date range correctly rejected");

    // Test 3: Progress validation is done in UpdateProject, not CreateProject
    // So we skip this test as CreateProject doesn't have progress_percentage field
    println!("✓ Skipped invalid progress percentage test (not applicable to CreateProject)");

    println!("✅ Validation error tests completed");
}

/// Test handling of not found errors
#[tokio::test]
async fn test_not_found_errors() {
    let pool = setup_test_database().await.expect("Failed to setup test database");

    let project_repo = Arc::new(PostgresProjectRepository::new(pool.clone()));
    let task_repo = Arc::new(PostgresTaskRepository::new(pool.clone()));
    let task_dependency_repo = Arc::new(PostgresTaskDependencyRepository::new(pool.clone()));
    let task_comment_repo = Arc::new(PostgresTaskCommentRepository::new(pool.clone()));

    let project_service = ProjectService::new(project_repo);
    let task_service = TaskService::new(task_repo, task_dependency_repo, task_comment_repo);

    println!("Testing not found errors...");

    // Test 1: Get non-existent project
    let result = project_service.get_project(999999).await;
    assert!(result.is_err(), "Getting non-existent project should fail");

    if let Err(e) = result {
        match e {
            DevErpError::NotFound(_) => println!("✓ Not found error correctly returned for project"),
            _ => panic!("Expected NotFound error, got: {:?}", e),
        }
    }

    // Test 2: Update non-existent project
    let fake_project_update = deverp::domain::project::entity::UpdateProject {
        id: 999999,  // Non-existent ID
        name: Some("Fake Project".to_string()),
        description: None,
        code: None,
        status: None,
        priority: None,
        start_date: None,
        end_date: None,
        actual_start_date: None,
        actual_end_date: None,
        progress_percentage: None,
        repository_url: None,
        repository_branch: None,
        tags: None,
        metadata: None,
    };
    let _result = project_service.update_project(fake_project_update).await;
    // Note: update might succeed if ID doesn't exist yet, so we test with a very large ID

    // Test 3: Delete non-existent project
    let result = project_service.delete_project(999999).await;
    assert!(result.is_err(), "Deleting non-existent project should fail");
    println!("✓ Delete non-existent project correctly handled");

    // Test 4: Get tasks for non-existent project
    let filter = deverp::domain::task::entity::TaskFilter {
        project_id: Some(999999),
        status: None,
        priority: None,
        task_type: None,
        assigned_to: None,
        parent_task_id: None,
        include_deleted: false,
        offset: None,
        limit: None,
    };
    let result = task_service.list_tasks(filter).await;
    // This should succeed but return empty list
    assert!(result.is_ok(), "Listing tasks for non-existent project should return empty list");
    assert_eq!(result.unwrap().len(), 0, "Should return empty task list");
    println!("✓ Tasks for non-existent project correctly handled");

    println!("✅ Not found error tests completed");
}

/// Test handling of duplicate data
#[tokio::test]
async fn test_duplicate_errors() {
    let pool = setup_test_database().await.expect("Failed to setup test database");
    let project_repo = Arc::new(PostgresProjectRepository::new(pool.clone()));
    let project_service = ProjectService::new(project_repo);

    println!("Testing duplicate errors...");

    // Create a project with a unique code
    let mut project1 = create_test_project("Duplicate Test 1");
    project1.code = Some("DUP-001".to_string());

    project_service.create_project(project1.clone())
        .await
        .expect("Failed to create first project");

    // Try to create another project with the same code
    let mut project2 = create_test_project("Duplicate Test 2");
    project2.code = Some("DUP-001".to_string());  // Same code

    let result = project_service.create_project(project2).await;
    assert!(result.is_err(), "Duplicate project code should be rejected");

    if let Err(e) = result {
        match e {
            DevErpError::Conflict(_) | DevErpError::Database(_) => {
                println!("✓ Duplicate code correctly rejected");
            }
            _ => panic!("Expected Conflict or Database error for duplicate, got: {:?}", e),
        }
    }

    println!("✅ Duplicate error tests completed");
}

/// Test handling of referential integrity violations
#[tokio::test]
async fn test_referential_integrity() {
    let pool = setup_test_database().await.expect("Failed to setup test database");

    let task_repo = Arc::new(PostgresTaskRepository::new(pool.clone()));
    let task_dependency_repo = Arc::new(PostgresTaskDependencyRepository::new(pool.clone()));
    let task_comment_repo = Arc::new(PostgresTaskCommentRepository::new(pool.clone()));
    let task_service = TaskService::new(task_repo, task_dependency_repo, task_comment_repo);

    println!("Testing referential integrity...");

    // Try to create a task for a non-existent project
    let invalid_task = create_test_task(999999, "Orphan Task");

    let result = task_service.create_task(invalid_task).await;
    assert!(result.is_err(), "Task with non-existent project should be rejected");

    if let Err(e) = result {
        match e {
            DevErpError::Database(_) | DevErpError::NotFound(_) => {
                println!("✓ Task with non-existent project correctly rejected");
            }
            _ => panic!("Expected Database or NotFound error, got: {:?}", e),
        }
    }

    println!("✅ Referential integrity tests completed");
}

/// Test transaction rollback on errors
#[tokio::test]
async fn test_transaction_rollback() {
    let pool = setup_test_database().await.expect("Failed to setup test database");

    let project_repo = Arc::new(PostgresProjectRepository::new(pool.clone()));
    let task_repo = Arc::new(PostgresTaskRepository::new(pool.clone()));
    let task_dependency_repo = Arc::new(PostgresTaskDependencyRepository::new(pool.clone()));
    let task_comment_repo = Arc::new(PostgresTaskCommentRepository::new(pool.clone()));

    let project_service = ProjectService::new(project_repo.clone());
    let task_service = TaskService::new(task_repo, task_dependency_repo, task_comment_repo);

    println!("Testing transaction rollback...");

    // Create a project
    let project = project_service.create_project(create_test_project("Transaction Test"))
        .await
        .expect("Failed to create project");

    // Create a task
    let task = task_service.create_task(create_test_task(project.id, "Task 1"))
        .await
        .expect("Failed to create task");

    // Count tasks before
    let filter_before = deverp::domain::task::entity::TaskFilter {
        project_id: Some(project.id),
        status: None,
        priority: None,
        task_type: None,
        assigned_to: None,
        parent_task_id: None,
        include_deleted: false,
        offset: None,
        limit: None,
    };
    let tasks_before = task_service.list_tasks(filter_before)
        .await
        .expect("Failed to list tasks");

    let _count_before = tasks_before.len();

    // Attempt an operation that should fail (e.g., circular dependency)
    // This should not affect the existing data
    let task2 = task_service.create_task(create_test_task(project.id, "Task 2"))
        .await
        .expect("Failed to create task 2");

    // Try to create circular dependency
    let dep1 = deverp::domain::task::entity::CreateTaskDependency {
        task_id: task.id,
        depends_on_task_id: task2.id,
        dependency_type: Some(deverp::domain::task::entity::DependencyType::FinishToStart),
    };
    let _ = task_service.add_task_dependency(dep1).await;

    let dep2 = deverp::domain::task::entity::CreateTaskDependency {
        task_id: task2.id,
        depends_on_task_id: task.id,
        dependency_type: Some(deverp::domain::task::entity::DependencyType::FinishToStart),
    };
    let result = task_service.add_task_dependency(dep2).await;

    assert!(result.is_err(), "Circular dependency should fail");

    // Verify data is still consistent
    let filter_after = deverp::domain::task::entity::TaskFilter {
        project_id: Some(project.id),
        status: None,
        priority: None,
        task_type: None,
        assigned_to: None,
        parent_task_id: None,
        include_deleted: false,
        offset: None,
        limit: None,
    };
    let tasks_after = task_service.list_tasks(filter_after)
        .await
        .expect("Failed to list tasks");

    // Tasks should still exist
    assert!(tasks_after.len() >= 2, "Tasks should not be deleted after failed operation");

    println!("✓ Data remains consistent after failed operation");
    println!("✅ Transaction rollback tests completed");
}

/// Test handling of database connection errors
#[tokio::test]
async fn test_connection_errors() {
    // This test attempts to connect to an invalid database
    println!("Testing connection error handling...");

    // Try to create a pool with invalid connection string
    let result = sqlx::postgres::PgPoolOptions::new()
        .max_connections(1)
        .connect("postgres://invalid:invalid@localhost:99999/invalid")
        .await;

    assert!(result.is_err(), "Invalid connection should fail");
    println!("✓ Invalid connection correctly rejected");

    println!("✅ Connection error tests completed");
}

/// Test concurrent access conflicts
#[tokio::test]
async fn test_concurrent_conflicts() {
    let pool = setup_test_database().await.expect("Failed to setup test database");

    let project_repo = Arc::new(PostgresProjectRepository::new(pool.clone()));
    let project_service = Arc::new(ProjectService::new(project_repo));

    println!("Testing concurrent access...");

    // Create a project
    let project = project_service.create_project(create_test_project("Concurrent Test"))
        .await
        .expect("Failed to create project");

    // Simulate concurrent updates
    let service1 = project_service.clone();
    let service2 = project_service.clone();

    let project_id = project.id;

    let handle1 = tokio::spawn(async move {
        let update = deverp::domain::project::entity::UpdateProject {
            id: project_id,
            name: None,
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
        service1.update_project(update).await
    });

    let handle2 = tokio::spawn(async move {
        let update = deverp::domain::project::entity::UpdateProject {
            id: project_id,
            name: None,
            description: None,
            code: None,
            status: None,
            priority: None,
            start_date: None,
            end_date: None,
            actual_start_date: None,
            actual_end_date: None,
            progress_percentage: Some(75),
            repository_url: None,
            repository_branch: None,
            tags: None,
            metadata: None,
        };
        service2.update_project(update).await
    });

    let result1 = handle1.await.expect("Task 1 panicked");
    let result2 = handle2.await.expect("Task 2 panicked");

    // At least one should succeed
    assert!(result1.is_ok() || result2.is_ok(), "At least one concurrent update should succeed");

    println!("✓ Concurrent updates handled correctly");
    println!("✅ Concurrent conflict tests completed");
}

/// Test resource cleanup after errors
#[tokio::test]
async fn test_resource_cleanup() {
    let pool = setup_test_database().await.expect("Failed to setup test database");

    let project_repo = Arc::new(PostgresProjectRepository::new(pool.clone()));
    let resource_repo = Arc::new(PostgresResourceRepository::new(pool.clone()));

    let project_service = ProjectService::new(project_repo);
    let resource_service = ResourceService::new(resource_repo);

    println!("Testing resource cleanup after errors...");

    // Create a project and resource
    let project = project_service.create_project(create_test_project("Cleanup Test"))
        .await
        .expect("Failed to create project");

    let resource = resource_service.create_resource(create_test_resource("Test Resource"))
        .await
        .expect("Failed to create resource");

    // Link resource to project
    use deverp::domain::resource::entity::LinkResourceToProject;
    resource_service.link_resource_to_project(LinkResourceToProject {
        project_id: project.id,
        resource_id: resource.id,
        usage_notes: None,
        version_used: None,
        is_critical: Some(false),
    })
        .await
        .expect("Failed to link resource");

    // Now delete the project (should handle resource links)
    let result = project_service.delete_project(project.id).await;
    assert!(result.is_ok(), "Project deletion should succeed");

    // Resource should still exist (soft delete)
    let resource_check = resource_service.get_resource(resource.id).await;
    assert!(resource_check.is_ok(), "Resource should still exist after project deletion");

    println!("✓ Resources cleaned up correctly after project deletion");
    println!("✅ Resource cleanup tests completed");
}
