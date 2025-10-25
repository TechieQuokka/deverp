mod helpers;

use deverp::domain::project::service::ProjectService;
use deverp::domain::task::entity::DependencyType;
use deverp::domain::task::service::TaskService;
use deverp::infrastructure::repositories::project_repo::PostgresProjectRepository;
use deverp::infrastructure::repositories::task_repo::PostgresTaskRepository;
use helpers::*;
use std::sync::Arc;

/// Scenario 2: Task dependency management
///
/// This test validates task dependency functionality:
/// 1. Create multiple tasks
/// 2. Add dependencies between tasks
/// 3. Attempt to create circular dependency (should fail)
/// 4. Verify dependency-based workflow
#[tokio::test]
async fn test_task_dependency_management() {
    // Setup test database
    let pool = setup_test_database()
        .await
        .expect("Failed to setup test database");

    // Initialize repositories and services
    let project_repo = Arc::new(PostgresProjectRepository::new(pool.clone()));
    let task_repo = Arc::new(PostgresTaskRepository::new(pool.clone()));
    let dependency_repo = Arc::new(
        deverp::infrastructure::repositories::task_repo::PostgresTaskDependencyRepository::new(
            pool.clone(),
        ),
    );
    let comment_repo = Arc::new(
        deverp::infrastructure::repositories::task_repo::PostgresTaskCommentRepository::new(
            pool.clone(),
        ),
    );

    let project_service = ProjectService::new(project_repo);
    let task_service = TaskService::new(task_repo, dependency_repo, comment_repo);

    // Step 1: Create a project and multiple tasks
    println!("Step 1: Creating project and tasks...");

    let project = project_service
        .create_project(create_test_project("Dependency Test Project"))
        .await
        .expect("Failed to create project");

    // Create tasks with specific dependencies in mind:
    // Task A -> Task B -> Task C -> Task D
    // Task E (independent)

    let task_a = task_service
        .create_task(create_test_task(project.id, "Task A - Foundation"))
        .await
        .expect("Failed to create task A");

    let task_b = task_service
        .create_task(create_test_task(project.id, "Task B - Build on A"))
        .await
        .expect("Failed to create task B");

    let task_c = task_service
        .create_task(create_test_task(project.id, "Task C - Build on B"))
        .await
        .expect("Failed to create task C");

    let task_d = task_service
        .create_task(create_test_task(project.id, "Task D - Build on C"))
        .await
        .expect("Failed to create task D");

    let task_e = task_service
        .create_task(create_test_task(project.id, "Task E - Independent"))
        .await
        .expect("Failed to create task E");

    println!("Created 5 tasks");

    // Step 2: Add dependencies
    println!("Step 2: Adding dependencies...");

    // Task B depends on Task A (finish-to-start)
    task_service
        .add_task_dependency(deverp::domain::task::entity::CreateTaskDependency {
            task_id: task_b.id,
            depends_on_task_id: task_a.id,
            dependency_type: Some(DependencyType::FinishToStart),
        })
        .await
        .expect("Failed to add dependency B -> A");

    // Task C depends on Task B
    task_service
        .add_task_dependency(deverp::domain::task::entity::CreateTaskDependency {
            task_id: task_c.id,
            depends_on_task_id: task_b.id,
            dependency_type: Some(DependencyType::FinishToStart),
        })
        .await
        .expect("Failed to add dependency C -> B");

    // Task D depends on Task C
    task_service
        .add_task_dependency(deverp::domain::task::entity::CreateTaskDependency {
            task_id: task_d.id,
            depends_on_task_id: task_c.id,
            dependency_type: Some(DependencyType::FinishToStart),
        })
        .await
        .expect("Failed to add dependency D -> C");

    println!("Added dependencies: A -> B -> C -> D");

    // Verify dependencies were created
    let task_b_deps = task_service
        .get_task_dependencies(task_b.id)
        .await
        .expect("Failed to get task B dependencies");

    assert_eq!(task_b_deps.len(), 1);
    assert_eq!(task_b_deps[0].depends_on_task_id, task_a.id);

    // Step 3: Attempt circular dependency (should fail)
    println!("Step 3: Testing circular dependency detection...");

    // Try to make Task A depend on Task D (which would create a cycle: A -> B -> C -> D -> A)
    let circular_result = task_service
        .add_task_dependency(deverp::domain::task::entity::CreateTaskDependency {
            task_id: task_a.id,
            depends_on_task_id: task_d.id,
            dependency_type: Some(DependencyType::FinishToStart),
        })
        .await;

    assert!(
        circular_result.is_err(),
        "Circular dependency should have been rejected"
    );

    if let Err(e) = circular_result {
        println!("✓ Circular dependency correctly rejected: {}", e);
    }

    // Step 4: Verify dependency chain
    println!("Step 4: Verifying dependency chain...");

    // Get all dependencies for task D (should include C, B, and A transitively)
    let task_d_deps = task_service
        .get_task_dependencies(task_d.id)
        .await
        .expect("Failed to get task D dependencies");

    assert_eq!(
        task_d_deps.len(),
        1,
        "Task D should have 1 direct dependency"
    );
    assert_eq!(task_d_deps[0].depends_on_task_id, task_c.id);

    // Step 5: Test dependency removal
    println!("Step 5: Testing dependency removal...");

    task_service
        .remove_task_dependency(task_c.id, task_b.id)
        .await
        .expect("Failed to remove dependency C -> B");

    let task_c_deps_after = task_service
        .get_task_dependencies(task_c.id)
        .await
        .expect("Failed to get task C dependencies after removal");

    assert_eq!(
        task_c_deps_after.len(),
        0,
        "Task C should have no dependencies after removal"
    );

    // Step 6: Verify independent task has no dependencies
    println!("Step 6: Verifying independent task...");

    let task_e_deps = task_service
        .get_task_dependencies(task_e.id)
        .await
        .expect("Failed to get task E dependencies");

    assert_eq!(
        task_e_deps.len(),
        0,
        "Independent task should have no dependencies"
    );

    // Step 7: Test complex dependency scenario
    println!("Step 7: Testing complex dependency scenario...");

    // Re-add the removed dependency
    task_service
        .add_task_dependency(deverp::domain::task::entity::CreateTaskDependency {
            task_id: task_c.id,
            depends_on_task_id: task_b.id,
            dependency_type: Some(DependencyType::FinishToStart),
        })
        .await
        .expect("Failed to re-add dependency C -> B");

    // Add Task E as a dependency to Task D (multiple dependencies)
    task_service
        .add_task_dependency(deverp::domain::task::entity::CreateTaskDependency {
            task_id: task_d.id,
            depends_on_task_id: task_e.id,
            dependency_type: Some(DependencyType::FinishToStart),
        })
        .await
        .expect("Failed to add dependency D -> E");

    let task_d_deps_final = task_service
        .get_task_dependencies(task_d.id)
        .await
        .expect("Failed to get task D final dependencies");

    assert_eq!(
        task_d_deps_final.len(),
        2,
        "Task D should have 2 dependencies"
    );

    // Step 8: Test dependency type variations
    println!("Step 8: Testing different dependency types...");

    // Create two more tasks for testing other dependency types
    let task_f = task_service
        .create_task(create_test_task(project.id, "Task F"))
        .await
        .expect("Failed to create task F");

    let task_g = task_service
        .create_task(create_test_task(project.id, "Task G"))
        .await
        .expect("Failed to create task G");

    // Test start-to-start dependency
    task_service
        .add_task_dependency(deverp::domain::task::entity::CreateTaskDependency {
            task_id: task_f.id,
            depends_on_task_id: task_g.id,
            dependency_type: Some(DependencyType::StartToStart),
        })
        .await
        .expect("Failed to add start-to-start dependency");

    let task_f_deps = task_service
        .get_task_dependencies(task_f.id)
        .await
        .expect("Failed to get task F dependencies");

    assert_eq!(task_f_deps[0].dependency_type, DependencyType::StartToStart);

    println!("✅ Scenario 2: Task dependency management test completed successfully!");
}

/// Test that verifies we cannot create a self-dependency
#[tokio::test]
async fn test_self_dependency_rejection() {
    let pool = setup_test_database()
        .await
        .expect("Failed to setup test database");

    let project_repo = Arc::new(PostgresProjectRepository::new(pool.clone()));
    let task_repo = Arc::new(PostgresTaskRepository::new(pool.clone()));
    let dependency_repo = Arc::new(
        deverp::infrastructure::repositories::task_repo::PostgresTaskDependencyRepository::new(
            pool.clone(),
        ),
    );
    let comment_repo = Arc::new(
        deverp::infrastructure::repositories::task_repo::PostgresTaskCommentRepository::new(
            pool.clone(),
        ),
    );

    let project_service = ProjectService::new(project_repo);
    let task_service = TaskService::new(task_repo, dependency_repo, comment_repo);

    let project = project_service
        .create_project(create_test_project("Self Dependency Test"))
        .await
        .expect("Failed to create project");

    let task = task_service
        .create_task(create_test_task(project.id, "Self Task"))
        .await
        .expect("Failed to create task");

    // Attempt to create self-dependency
    let result = task_service
        .add_task_dependency(deverp::domain::task::entity::CreateTaskDependency {
            task_id: task.id,
            depends_on_task_id: task.id,
            dependency_type: Some(DependencyType::FinishToStart),
        })
        .await;

    assert!(result.is_err(), "Self-dependency should be rejected");
    println!("✅ Self-dependency correctly rejected");
}

/// Test complex circular dependency detection
#[tokio::test]
async fn test_complex_circular_dependency() {
    let pool = setup_test_database()
        .await
        .expect("Failed to setup test database");

    let project_repo = Arc::new(PostgresProjectRepository::new(pool.clone()));
    let task_repo = Arc::new(PostgresTaskRepository::new(pool.clone()));
    let dependency_repo = Arc::new(
        deverp::infrastructure::repositories::task_repo::PostgresTaskDependencyRepository::new(
            pool.clone(),
        ),
    );
    let comment_repo = Arc::new(
        deverp::infrastructure::repositories::task_repo::PostgresTaskCommentRepository::new(
            pool.clone(),
        ),
    );

    let project_service = ProjectService::new(project_repo);
    let task_service = TaskService::new(task_repo, dependency_repo, comment_repo);

    let project = project_service
        .create_project(create_test_project("Complex Cycle Test"))
        .await
        .expect("Failed to create project");

    // Create tasks: A -> B -> C -> D -> E
    let tasks: Vec<_> = (0..5)
        .map(|i| {
            let title = format!("Task {}", (b'A' + i as u8) as char);
            create_test_task(project.id, &title)
        })
        .collect();

    let mut task_ids = Vec::new();
    for task_input in tasks {
        let task = task_service
            .create_task(task_input)
            .await
            .expect("Failed to create task");
        task_ids.push(task.id);
    }

    // Create chain: A -> B -> C -> D -> E
    for i in 0..4 {
        task_service
            .add_task_dependency(deverp::domain::task::entity::CreateTaskDependency {
                task_id: task_ids[i + 1],
                depends_on_task_id: task_ids[i],
                dependency_type: Some(DependencyType::FinishToStart),
            })
            .await
            .expect("Failed to add dependency");
    }

    // Try to close the loop: E -> A (should fail)
    let result = task_service
        .add_task_dependency(deverp::domain::task::entity::CreateTaskDependency {
            task_id: task_ids[0],
            depends_on_task_id: task_ids[4],
            dependency_type: Some(DependencyType::FinishToStart),
        })
        .await;

    assert!(
        result.is_err(),
        "Circular dependency should be detected in long chain"
    );
    println!("✅ Complex circular dependency correctly detected");
}
