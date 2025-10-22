mod helpers;

use std::sync::Arc;
use std::time::Instant;
use deverp::domain::project::service::ProjectService;
use deverp::domain::task::service::TaskService;
use deverp::infrastructure::repositories::project_repo::PostgresProjectRepository;
use deverp::infrastructure::repositories::task_repo::{PostgresTaskRepository, PostgresTaskDependencyRepository, PostgresTaskCommentRepository};
use helpers::*;

/// Performance Test: Bulk project creation
///
/// Tests creating 100 projects and measures performance
#[tokio::test]
async fn test_bulk_project_creation() {
    let pool = setup_test_database().await.expect("Failed to setup test database");
    let project_repo = Arc::new(PostgresProjectRepository::new(pool.clone()));
    let project_service = ProjectService::new(project_repo);

    println!("Performance Test: Creating 100 projects...");

    let start = Instant::now();

    for i in 0..100 {
        let project_input = create_test_project(&format!("Bulk Project {}", i));
        project_service.create_project(project_input)
            .await
            .expect(&format!("Failed to create project {}", i));
    }

    let duration = start.elapsed();

    println!("✅ Created 100 projects in {:?}", duration);
    println!("   Average: {:?} per project", duration / 100);

    // Verify all projects were created
    let filter = deverp::domain::project::entity::ProjectFilter {
        status: None,
        priority: None,
        tags: None,
        search: None,
        offset: None,
        limit: None,
    };
    let all_projects = project_service.list_projects(filter)
        .await
        .expect("Failed to list projects");

    assert_eq!(all_projects.len(), 100, "Should have 100 projects");

    // Performance assertion: should complete within reasonable time
    assert!(duration.as_secs() < 30, "Bulk creation should complete within 30 seconds");
}

/// Performance Test: Bulk task creation
///
/// Tests creating 1000 tasks across 10 projects
#[tokio::test]
async fn test_bulk_task_creation() {
    let pool = setup_test_database().await.expect("Failed to setup test database");

    let project_repo = Arc::new(PostgresProjectRepository::new(pool.clone()));
    let task_repo = Arc::new(PostgresTaskRepository::new(pool.clone()));
    let task_dependency_repo = Arc::new(PostgresTaskDependencyRepository::new(pool.clone()));
    let task_comment_repo = Arc::new(PostgresTaskCommentRepository::new(pool.clone()));

    let project_service = ProjectService::new(project_repo);
    let task_service = TaskService::new(task_repo, task_dependency_repo, task_comment_repo);

    println!("Performance Test: Creating 1000 tasks across 10 projects...");

    // First, create 10 projects
    let mut project_ids = Vec::new();
    for i in 0..10 {
        let project = project_service.create_project(create_test_project(&format!("Project {}", i)))
            .await
            .expect("Failed to create project");
        project_ids.push(project.id);
    }

    // Now create 100 tasks per project (1000 total)
    let start = Instant::now();

    let mut task_count = 0;
    for (project_idx, project_id) in project_ids.iter().enumerate() {
        for task_idx in 0..100 {
            let task_input = create_test_task(*project_id, &format!("Task {}-{}", project_idx, task_idx));
            task_service.create_task(task_input)
                .await
                .expect("Failed to create task");
            task_count += 1;
        }
    }

    let duration = start.elapsed();

    println!("✅ Created {} tasks in {:?}", task_count, duration);
    println!("   Average: {:?} per task", duration / task_count);

    assert_eq!(task_count, 1000, "Should have created 1000 tasks");

    // Performance assertion
    assert!(duration.as_secs() < 60, "Bulk task creation should complete within 60 seconds");
}

/// Performance Test: Complex queries with large dataset
#[tokio::test]
async fn test_query_performance_large_dataset() {
    let pool = setup_test_database().await.expect("Failed to setup test database");

    let project_repo = Arc::new(PostgresProjectRepository::new(pool.clone()));
    let task_repo = Arc::new(PostgresTaskRepository::new(pool.clone()));
    let task_dependency_repo = Arc::new(PostgresTaskDependencyRepository::new(pool.clone()));
    let task_comment_repo = Arc::new(PostgresTaskCommentRepository::new(pool.clone()));

    let project_service = ProjectService::new(project_repo);
    let task_service = TaskService::new(task_repo, task_dependency_repo, task_comment_repo);

    println!("Performance Test: Query performance with large dataset...");

    // Create 50 projects with 20 tasks each (1000 tasks total)
    let mut project_ids = Vec::new();
    for i in 0..50 {
        let project = project_service.create_project(create_test_project(&format!("Query Test Project {}", i)))
            .await
            .expect("Failed to create project");
        project_ids.push(project.id);

        for j in 0..20 {
            task_service.create_task(create_test_task(project.id, &format!("Task {}", j)))
                .await
                .expect("Failed to create task");
        }
    }

    println!("Created test dataset: 50 projects, 1000 tasks");

    // Test 1: List all projects
    let start = Instant::now();
    let filter = deverp::domain::project::entity::ProjectFilter {
        status: None,
        priority: None,
        tags: None,
        search: None,
        offset: None,
        limit: None,
    };
    let all_projects = project_service.list_projects(filter)
        .await
        .expect("Failed to list projects");
    let list_duration = start.elapsed();

    println!("✅ Listed {} projects in {:?}", all_projects.len(), list_duration);
    assert!(list_duration.as_millis() < 1000, "Listing projects should take < 1 second");

    // Test 2: Get project details (should include task counts)
    let start = Instant::now();
    for project_id in project_ids.iter().take(10) {
        project_service.get_project(*project_id)
            .await
            .expect("Failed to get project");
    }
    let detail_duration = start.elapsed();

    println!("✅ Retrieved 10 project details in {:?}", detail_duration);
    assert!(detail_duration.as_millis() < 500, "Getting 10 projects should take < 500ms");

    // Test 3: List tasks for a specific project
    let start = Instant::now();
    let task_filter = deverp::domain::task::entity::TaskFilter {
        project_id: Some(project_ids[0]),
        status: None,
        priority: None,
        task_type: None,
        assigned_to: None,
        parent_task_id: None,
        include_deleted: false,
        offset: None,
        limit: None,
    };
    let project_tasks = task_service.list_tasks(task_filter)
        .await
        .expect("Failed to list tasks");
    let task_list_duration = start.elapsed();

    println!("✅ Listed {} tasks for project in {:?}", project_tasks.len(), task_list_duration);
    assert_eq!(project_tasks.len(), 20, "Should have 20 tasks");
    assert!(task_list_duration.as_millis() < 500, "Listing tasks should take < 500ms");
}

/// Performance Test: Memory usage
///
/// Tests memory efficiency when handling large result sets
#[tokio::test]
async fn test_memory_usage() {
    let pool = setup_test_database().await.expect("Failed to setup test database");

    let project_repo = Arc::new(PostgresProjectRepository::new(pool.clone()));
    let project_service = ProjectService::new(project_repo);

    println!("Performance Test: Memory usage with pagination...");

    // Create 200 projects
    for i in 0..200 {
        let project_input = create_test_project(&format!("Memory Test Project {}", i));
        project_service.create_project(project_input)
            .await
            .expect("Failed to create project");
    }

    // Test pagination (should be more memory efficient than loading all at once)
    let start = Instant::now();

    let mut total_loaded = 0;
    for page in 0..4 {
        let offset = page * 50;
        let filter = deverp::domain::project::entity::ProjectFilter {
            status: None,
            priority: None,
            tags: None,
            search: None,
            offset: Some(offset),
            limit: Some(50),
        };
        let projects = project_service.list_projects(filter)
            .await
            .expect("Failed to list projects with pagination");

        total_loaded += projects.len();
        println!("   Page {}: loaded {} projects", page + 1, projects.len());
    }

    let duration = start.elapsed();

    println!("✅ Loaded {} projects using pagination in {:?}", total_loaded, duration);
    assert_eq!(total_loaded, 200, "Should load all 200 projects across pages");
}

/// Performance Test: Concurrent operations
///
/// Tests system behavior under concurrent load
#[tokio::test]
async fn test_concurrent_operations() {
    let pool = setup_test_database().await.expect("Failed to setup test database");

    let project_repo = Arc::new(PostgresProjectRepository::new(pool.clone()));
    let project_service = Arc::new(ProjectService::new(project_repo));

    println!("Performance Test: Concurrent operations...");

    let start = Instant::now();

    // Spawn 10 concurrent tasks, each creating 10 projects
    let mut handles = Vec::new();

    for i in 0..10 {
        let service = project_service.clone();
        let handle = tokio::spawn(async move {
            for j in 0..10 {
                let project_input = create_test_project(&format!("Concurrent-{}-{}", i, j));
                service.create_project(project_input)
                    .await
                    .expect("Failed to create project");
            }
        });
        handles.push(handle);
    }

    // Wait for all tasks to complete
    for handle in handles {
        handle.await.expect("Task panicked");
    }

    let duration = start.elapsed();

    println!("✅ Completed 100 concurrent operations in {:?}", duration);

    // Verify all projects were created
    let filter = deverp::domain::project::entity::ProjectFilter {
        status: None,
        priority: None,
        tags: None,
        search: None,
        offset: None,
        limit: None,
    };
    let all_projects = project_service.list_projects(filter)
        .await
        .expect("Failed to list projects");

    assert_eq!(all_projects.len(), 100, "Should have 100 projects from concurrent operations");
    assert!(duration.as_secs() < 20, "Concurrent operations should complete within 20 seconds");
}

/// Performance Test: Database connection pool efficiency
#[tokio::test]
async fn test_connection_pool_efficiency() {
    let pool = setup_test_database().await.expect("Failed to setup test database");

    let project_repo = Arc::new(PostgresProjectRepository::new(pool.clone()));
    let project_service = Arc::new(ProjectService::new(project_repo));

    println!("Performance Test: Connection pool efficiency...");

    // Create initial projects
    for i in 0..10 {
        project_service.create_project(create_test_project(&format!("Pool Test {}", i)))
            .await
            .expect("Failed to create project");
    }

    // Perform many rapid queries (should reuse connections efficiently)
    let start = Instant::now();

    for _ in 0..100 {
        let filter = deverp::domain::project::entity::ProjectFilter {
            status: None,
            priority: None,
            tags: None,
            search: None,
            offset: None,
            limit: Some(10),
        };
        project_service.list_projects(filter)
            .await
            .expect("Failed to list projects");
    }

    let duration = start.elapsed();

    println!("✅ Performed 100 queries in {:?}", duration);
    println!("   Average: {:?} per query", duration / 100);

    assert!(duration.as_secs() < 5, "100 queries should complete within 5 seconds with connection pooling");
}
