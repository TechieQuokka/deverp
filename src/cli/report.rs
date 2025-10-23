// Report CLI commands

use std::sync::Arc;
use crate::Result;
use crate::utils::formatter::{table_header, table_row, section_header, key_value};
use super::commands::{ReportCommand, OutputFormat};
use crate::config::settings::Settings;
use crate::infrastructure::database;

use crate::domain::report::ReportService;
use crate::infrastructure::repositories::{
    project_repo::PostgresProjectRepository,
    task_repo::PostgresTaskRepository,
    resource_repo::PostgresResourceRepository,
    timeline_repo::{PostgresTimelineRepository, PostgresMilestoneRepository},
};

/// Handle report commands
pub async fn handle(command: ReportCommand, _format: OutputFormat) -> Result<()> {
    // Establish database connection
    let settings = Settings::default();
    let pool = database::establish_connection(&settings.database).await?;

    // Create repositories
    let project_repo = Arc::new(PostgresProjectRepository::new(pool.clone()));
    let task_repo = Arc::new(PostgresTaskRepository::new(pool.clone()));
    let resource_repo = Arc::new(PostgresResourceRepository::new(pool.clone()));
    let timeline_repo = Arc::new(PostgresTimelineRepository::new(pool.clone()));
    let milestone_repo = Arc::new(PostgresMilestoneRepository::new(pool));

    // Create report service
    let service = ReportService::new(project_repo, task_repo, resource_repo, timeline_repo, milestone_repo);

    match command {
        ReportCommand::Status => handle_status(service).await,
        ReportCommand::ProjectSummary => handle_project_summary(service).await,
        ReportCommand::TaskAnalytics => handle_task_analytics(service).await,
        ReportCommand::ResourceUsage => handle_resource_usage(service).await,
        ReportCommand::TimelineProgress => handle_timeline_progress(service).await,
    }
}

/// Handle status report command
async fn handle_status(service: ReportService) -> Result<()> {
    let report = service.generate_project_status_report().await?;

    section_header("PROJECT STATUS REPORT");

    println!();
    println!("Overall Statistics:");
    key_value("Total Projects", &report.total_projects.to_string());
    key_value("Active Projects", &report.active_projects.to_string());
    key_value("Completed Projects", &report.completed_projects.to_string());
    key_value("On Hold Projects", &report.on_hold_projects.to_string());
    key_value("Cancelled Projects", &report.cancelled_projects.to_string());
    key_value("Archived Projects", &report.archived_projects.to_string());
    key_value("Average Progress", &format!("{:.1}%", report.average_progress));
    key_value("Delayed Projects", &report.delayed_projects.to_string());

    println!();
    println!("Projects by Priority:");
    key_value("  Critical", &report.projects_by_priority.critical.to_string());
    key_value("  High", &report.projects_by_priority.high.to_string());
    key_value("  Medium", &report.projects_by_priority.medium.to_string());
    key_value("  Low", &report.projects_by_priority.low.to_string());

    println!();
    key_value("Generated At", &report.generated_at.format("%Y-%m-%d %H:%M:%S UTC").to_string());

    Ok(())
}

/// Handle project summary report command
async fn handle_project_summary(service: ReportService) -> Result<()> {
    let summary = service.generate_project_summary().await?;

    section_header("PROJECT SUMMARY");

    if summary.is_empty() {
        println!("\nNo projects found.");
        return Ok(());
    }

    println!();
    table_header(&[
        "ID",
        "Code",
        "Name",
        "Status",
        "Priority",
        "Progress",
        "Tasks",
        "Completed",
        "Start Date",
        "End Date",
    ]);

    for item in summary {
        table_row(&[
            item.project_id.to_string(),
            item.project_code.unwrap_or_else(|| "-".to_string()),
            item.project_name,
            item.status,
            item.priority,
            format!("{}%", item.progress_percentage),
            item.total_tasks.to_string(),
            format!("{}/{}", item.completed_tasks, item.total_tasks),
            item.start_date.unwrap_or_else(|| "-".to_string()),
            item.end_date.unwrap_or_else(|| "-".to_string()),
        ]);
    }

    Ok(())
}

/// Handle task analytics report command
async fn handle_task_analytics(service: ReportService) -> Result<()> {
    let report = service.generate_task_analytics().await?;

    section_header("TASK ANALYTICS REPORT");

    println!();
    println!("Overall Statistics:");
    key_value("Total Tasks", &report.total_tasks.to_string());
    key_value("Completion Rate", &format!("{:.1}%", report.completion_rate));
    key_value("Overdue Tasks", &report.overdue_tasks.to_string());
    key_value("On-Time Completions", &report.on_time_completion_count.to_string());

    println!();
    println!("Tasks by Status:");
    key_value("  Todo", &report.tasks_by_status.todo.to_string());
    key_value("  In Progress", &report.tasks_by_status.in_progress.to_string());
    key_value("  Blocked", &report.tasks_by_status.blocked.to_string());
    key_value("  Review", &report.tasks_by_status.review.to_string());
    key_value("  Testing", &report.tasks_by_status.testing.to_string());
    key_value("  Done", &report.tasks_by_status.done.to_string());
    key_value("  Cancelled", &report.tasks_by_status.cancelled.to_string());

    println!();
    println!("Tasks by Priority:");
    key_value("  Critical", &report.tasks_by_priority.critical.to_string());
    key_value("  High", &report.tasks_by_priority.high.to_string());
    key_value("  Medium", &report.tasks_by_priority.medium.to_string());
    key_value("  Low", &report.tasks_by_priority.low.to_string());

    println!();
    println!("Time Tracking:");
    key_value("Total Estimated Hours", &format!("{:.1}", report.total_estimated_hours));
    key_value("Total Actual Hours", &format!("{:.1}", report.total_actual_hours));
    key_value("Average Estimated Hours", &format!("{:.1}", report.avg_estimated_hours));
    key_value("Average Actual Hours", &format!("{:.1}", report.avg_actual_hours));
    key_value("Time Variance", &format!("{:.1}%", report.time_variance_percentage));

    println!();
    key_value("Generated At", &report.generated_at.format("%Y-%m-%d %H:%M:%S UTC").to_string());

    Ok(())
}

/// Handle resource usage report command
async fn handle_resource_usage(service: ReportService) -> Result<()> {
    let report = service.generate_resource_usage_report().await?;

    section_header("RESOURCE USAGE REPORT");

    println!();
    println!("Overall Statistics:");
    key_value("Total Resources", &report.total_resources.to_string());
    key_value("Active Resources", &report.active_resources.to_string());
    key_value("Deprecated Resources", &report.deprecated_resources.to_string());
    key_value("Unused Resources", &report.unused_resources.to_string());

    println!();
    println!("Resources by Type:");
    key_value("  Library", &report.resources_by_type.library.to_string());
    key_value("  API", &report.resources_by_type.api.to_string());
    key_value("  Tool", &report.resources_by_type.tool.to_string());
    key_value("  Service", &report.resources_by_type.service.to_string());
    key_value("  Documentation", &report.resources_by_type.documentation.to_string());
    key_value("  Other", &report.resources_by_type.other.to_string());

    if !report.most_used_resources.is_empty() {
        println!();
        section_header("TOP 10 MOST USED RESOURCES");
        println!();
        table_header(&["ID", "Name", "Type", "Projects", "Critical Projects"]);

        for item in &report.most_used_resources {
            table_row(&[
                item.resource_id.to_string(),
                item.resource_name.clone(),
                item.resource_type.clone(),
                item.project_count.to_string(),
                item.critical_project_count.to_string(),
            ]);
        }
    }

    println!();
    key_value("Generated At", &report.generated_at.format("%Y-%m-%d %H:%M:%S UTC").to_string());

    Ok(())
}

/// Handle timeline progress report command
async fn handle_timeline_progress(service: ReportService) -> Result<()> {
    let report = service.generate_timeline_progress_report().await?;

    section_header("TIMELINE PROGRESS REPORT");

    println!();
    println!("Timeline Statistics:");
    key_value("Total Timelines", &report.total_timelines.to_string());
    key_value("Active Timelines", &report.active_timelines.to_string());
    key_value("Completed Timelines", &report.completed_timelines.to_string());

    println!();
    println!("Milestone Statistics:");
    key_value("Total Milestones", &report.total_milestones.to_string());
    key_value("Completed Milestones", &report.completed_milestones.to_string());
    key_value("Missed Milestones", &report.missed_milestones.to_string());
    key_value("Completion Rate", &format!("{:.1}%", report.milestone_completion_rate));
    key_value("On-Time Completion Rate", &format!("{:.1}%", report.on_time_milestone_rate));
    key_value("Upcoming Milestones (30 days)", &report.upcoming_milestones_count.to_string());

    println!();
    key_value("Generated At", &report.generated_at.format("%Y-%m-%d %H:%M:%S UTC").to_string());

    Ok(())
}
