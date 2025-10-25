mod helpers;

use chrono::NaiveDate;
use deverp::domain::project::entity::ProjectStatus;
use deverp::domain::project::service::ProjectService;
use deverp::domain::report::service::ReportService;
use deverp::domain::resource::service::ResourceService;
use deverp::domain::task::entity::TaskStatus;
use deverp::domain::task::service::TaskService;
use deverp::domain::timeline::entity::{CreateMilestone, MilestoneStatus, TimelineStatus};
use deverp::domain::timeline::service::TimelineService;
use deverp::infrastructure::repositories::project_repo::PostgresProjectRepository;
use deverp::infrastructure::repositories::resource_repo::PostgresResourceRepository;
use deverp::infrastructure::repositories::task_repo::{
    PostgresTaskCommentRepository, PostgresTaskDependencyRepository, PostgresTaskRepository,
};
use deverp::infrastructure::repositories::timeline_repo::{
    PostgresMilestoneRepository, PostgresTimelineRepository,
};
use helpers::*;
use std::sync::Arc;

/// Scenario 1: Complete project lifecycle from creation to completion
///
/// This test simulates a full project workflow:
/// 1. Create a project
/// 2. Add tasks
/// 3. Create timeline
/// 4. Add milestones
/// 5. Link resources
/// 6. Update progress
/// 7. Generate reports
/// 8. Complete project
#[tokio::test]
async fn test_complete_project_lifecycle() {
    // Setup test database
    let pool = setup_test_database()
        .await
        .expect("Failed to setup test database");

    // Initialize repositories
    let project_repo = Arc::new(PostgresProjectRepository::new(pool.clone()));
    let task_repo = Arc::new(PostgresTaskRepository::new(pool.clone()));
    let task_dependency_repo = Arc::new(PostgresTaskDependencyRepository::new(pool.clone()));
    let task_comment_repo = Arc::new(PostgresTaskCommentRepository::new(pool.clone()));
    let timeline_repo = Arc::new(PostgresTimelineRepository::new(pool.clone()));
    let milestone_repo = Arc::new(PostgresMilestoneRepository::new(pool.clone()));
    let resource_repo = Arc::new(PostgresResourceRepository::new(pool.clone()));

    // Initialize services
    let project_service = ProjectService::new(project_repo.clone());
    let task_service = TaskService::new(
        task_repo.clone(),
        task_dependency_repo.clone(),
        task_comment_repo.clone(),
    );
    let timeline_service = TimelineService::new(timeline_repo.clone(), milestone_repo.clone());
    let resource_service = ResourceService::new(resource_repo.clone());
    let report_service = ReportService::new(
        project_repo.clone(),
        task_repo.clone(),
        resource_repo.clone(),
        timeline_repo.clone(),
        milestone_repo.clone(),
    );

    // Step 1: Create a project
    println!("Step 1: Creating project...");
    let create_project = create_test_project("E2E Test Project");
    let project = project_service
        .create_project(create_project)
        .await
        .expect("Failed to create project");

    assert_eq!(project.name, "E2E Test Project");
    assert_eq!(project.status, ProjectStatus::Planning);
    assert_eq!(project.progress_percentage, Some(0));

    // Step 2: Add tasks
    println!("Step 2: Adding tasks...");
    let mut tasks = Vec::new();

    let task1 = task_service
        .create_task(create_test_task(project.id, "Design Database Schema"))
        .await
        .expect("Failed to create task 1");
    tasks.push(task1.clone());

    let task2 = task_service
        .create_task(create_test_task(project.id, "Implement API"))
        .await
        .expect("Failed to create task 2");
    tasks.push(task2.clone());

    let task3 = task_service
        .create_task(create_test_task(project.id, "Write Tests"))
        .await
        .expect("Failed to create task 3");
    tasks.push(task3.clone());

    let task4 = task_service
        .create_task(create_test_task(project.id, "Deploy to Production"))
        .await
        .expect("Failed to create task 4");
    tasks.push(task4.clone());

    assert_eq!(tasks.len(), 4);

    // Step 3: Create timeline
    println!("Step 3: Creating timeline...");
    let create_timeline = create_test_timeline(project.id, "Q1 2025 Development");
    let timeline = timeline_service
        .create_timeline(create_timeline)
        .await
        .expect("Failed to create timeline");

    assert_eq!(timeline.name, "Q1 2025 Development");
    assert_eq!(timeline.status, TimelineStatus::Planned);

    // Step 4: Add milestones
    println!("Step 4: Adding milestones...");
    let milestone1 = CreateMilestone {
        timeline_id: timeline.id,
        project_id: project.id,
        name: "Database Design Complete".to_string(),
        description: Some("Database schema finalized".to_string()),
        target_date: NaiveDate::from_ymd_opt(2025, 2, 1).unwrap(),
        status: Some(MilestoneStatus::Pending),
        completion_percentage: Some(0),
        metadata: None,
    };

    let milestone2 = CreateMilestone {
        timeline_id: timeline.id,
        project_id: project.id,
        name: "API Implementation Complete".to_string(),
        description: Some("All API endpoints implemented".to_string()),
        target_date: NaiveDate::from_ymd_opt(2025, 3, 1).unwrap(),
        status: Some(MilestoneStatus::Pending),
        completion_percentage: Some(0),
        metadata: None,
    };

    let m1 = timeline_service
        .create_milestone(milestone1)
        .await
        .expect("Failed to create milestone 1");

    let m2 = timeline_service
        .create_milestone(milestone2)
        .await
        .expect("Failed to create milestone 2");

    assert_eq!(m1.name, "Database Design Complete");
    assert_eq!(m2.name, "API Implementation Complete");

    // Step 5: Link resources
    println!("Step 5: Linking resources...");
    let resource1 = resource_service
        .create_resource(create_test_resource("PostgreSQL"))
        .await
        .expect("Failed to create resource 1");

    let resource2 = resource_service
        .create_resource(create_test_resource("Rust sqlx"))
        .await
        .expect("Failed to create resource 2");

    resource_service
        .link_resource_to_project(deverp::domain::resource::entity::LinkResourceToProject {
            project_id: project.id,
            resource_id: resource1.id,
            usage_notes: Some("Database system".to_string()),
            version_used: Some("14.0".to_string()),
            is_critical: Some(true),
        })
        .await
        .expect("Failed to link resource 1");

    resource_service
        .link_resource_to_project(deverp::domain::resource::entity::LinkResourceToProject {
            project_id: project.id,
            resource_id: resource2.id,
            usage_notes: Some("Database driver".to_string()),
            version_used: Some("0.7.0".to_string()),
            is_critical: Some(false),
        })
        .await
        .expect("Failed to link resource 2");

    // Verify resources are linked
    let project_resources = resource_service
        .get_project_resources(project.id)
        .await
        .expect("Failed to get project resources");

    assert_eq!(project_resources.len(), 2);

    // Step 6: Update progress
    println!("Step 6: Updating progress...");

    // Complete task 1
    task_service
        .update_task(deverp::domain::task::entity::UpdateTask {
            id: tasks[0].id,
            title: None,
            description: None,
            status: Some(TaskStatus::Done),
            priority: None,
            assigned_to: None,
            estimated_hours: None,
            actual_hours: Some(10.0),
            due_date: None,
            task_type: None,
            tags: None,
        })
        .await
        .expect("Failed to update task 1");

    // Complete milestone 1
    timeline_service
        .update_milestone(deverp::domain::timeline::entity::UpdateMilestone {
            id: m1.id,
            name: None,
            description: None,
            target_date: None,
            actual_date: Some(chrono::Utc::now().date_naive()),
            status: Some(deverp::domain::timeline::entity::MilestoneStatus::Completed),
            completion_percentage: Some(100),
            metadata: None,
        })
        .await
        .expect("Failed to complete milestone 1");

    // Update project status to Active
    project_service
        .update_project(deverp::domain::project::entity::UpdateProject {
            id: project.id,
            name: None,
            description: None,
            code: None,
            status: Some(ProjectStatus::Active),
            priority: None,
            start_date: None,
            end_date: None,
            actual_start_date: Some(NaiveDate::from_ymd_opt(2025, 1, 15).unwrap()),
            actual_end_date: None,
            progress_percentage: Some(25),
            repository_url: None,
            repository_branch: None,
            tags: None,
            metadata: None,
        })
        .await
        .expect("Failed to update project");

    // Step 7: Generate reports
    println!("Step 7: Generating reports...");

    let status_report = report_service
        .generate_project_status_report()
        .await
        .expect("Failed to generate status report");

    assert!(status_report.active_projects >= 1);

    let project_summaries = report_service
        .generate_project_summary()
        .await
        .expect("Failed to generate project summary");

    let project_summary = project_summaries
        .iter()
        .find(|s| s.project_name == "E2E Test Project")
        .expect("Project not found in summary");
    assert_eq!(project_summary.total_tasks, 4);
    assert_eq!(project_summary.completed_tasks, 1);

    // Step 8: Complete remaining tasks and project
    println!("Step 8: Completing project...");

    for task in tasks.iter().skip(1) {
        task_service
            .update_task(deverp::domain::task::entity::UpdateTask {
                id: task.id,
                title: None,
                description: None,
                status: Some(TaskStatus::Done),
                priority: None,
                assigned_to: None,
                estimated_hours: None,
                actual_hours: Some(8.0),
                due_date: None,
                task_type: None,
                tags: None,
            })
            .await
            .expect("Failed to complete task");
    }

    // Mark project as completed
    let completed_project = project_service
        .update_project(deverp::domain::project::entity::UpdateProject {
            id: project.id,
            name: None,
            description: None,
            code: None,
            status: Some(ProjectStatus::Completed),
            priority: None,
            start_date: None,
            end_date: None,
            actual_start_date: None,
            actual_end_date: Some(NaiveDate::from_ymd_opt(2025, 3, 15).unwrap()),
            progress_percentage: Some(100),
            repository_url: None,
            repository_branch: None,
            tags: None,
            metadata: None,
        })
        .await
        .expect("Failed to complete project");

    assert_eq!(completed_project.status, ProjectStatus::Completed);
    assert_eq!(completed_project.progress_percentage, Some(100));

    // Verify final state
    let final_summaries = report_service
        .generate_project_summary()
        .await
        .expect("Failed to generate final summary");

    let final_summary = final_summaries
        .iter()
        .find(|s| s.project_id == project.id)
        .expect("Project not found in final summary");
    assert_eq!(final_summary.completed_tasks, 4);
    assert_eq!(final_summary.total_tasks, 4);

    println!("✅ Scenario 1: Project lifecycle test completed successfully!");
}
