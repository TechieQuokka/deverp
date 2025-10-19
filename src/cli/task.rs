// Task CLI commands implementation

use std::sync::Arc;
use chrono::{DateTime, NaiveDate, Utc};
use colored::Colorize;
use uuid::Uuid;

use super::commands::{
    TaskCommand, CreateTaskArgs, ListTaskArgs, ShowTaskArgs,
    UpdateTaskArgs, DeleteTaskArgs, AddDependencyArgs,
    RemoveDependencyArgs, AddCommentArgs,
};
use super::output::{PaginatedOutput, section_title, summary_line, confirm, empty_state};
use crate::config::settings::Settings;
use crate::domain::task::{
    entity::{CreateTask, UpdateTask, TaskFilter, TaskStatus, TaskPriority, TaskType, DependencyType, CreateTaskDependency, CreateTaskComment},
    service::TaskService,
};
use crate::infrastructure::{
    database,
    repositories::{
        PostgresTaskRepository,
        PostgresTaskDependencyRepository,
        PostgresTaskCommentRepository,
    },
};
use crate::utils::error::DevErpError;
use crate::Result;

/// Handle task commands
pub async fn handle(command: TaskCommand) -> Result<()> {
    match command {
        TaskCommand::Create(args) => handle_create(args).await,
        TaskCommand::List(args) => handle_list(args).await,
        TaskCommand::Show(args) => handle_show(args).await,
        TaskCommand::Update(args) => handle_update(args).await,
        TaskCommand::Delete(args) => handle_delete(args).await,
        TaskCommand::AddDependency(args) => handle_add_dependency(args).await,
        TaskCommand::RemoveDependency(args) => handle_remove_dependency(args).await,
        TaskCommand::AddComment(args) => handle_add_comment(args).await,
    }
}

/// Create database connection and task service
async fn create_service() -> Result<TaskService> {
    let settings = Settings::default();
    let pool = database::establish_connection(&settings.database).await?;

    let task_repo = Arc::new(PostgresTaskRepository::new(pool.clone()));
    let dependency_repo = Arc::new(PostgresTaskDependencyRepository::new(pool.clone()));
    let comment_repo = Arc::new(PostgresTaskCommentRepository::new(pool));

    Ok(TaskService::new(task_repo, dependency_repo, comment_repo))
}

/// Handle task create command
async fn handle_create(args: CreateTaskArgs) -> Result<()> {
    let service = create_service().await?;

    // Parse status if provided
    let status = if let Some(status_str) = args.status {
        Some(status_str.parse::<TaskStatus>()
            .map_err(|e| DevErpError::Validation(e))?)
    } else {
        None
    };

    // Parse priority if provided
    let priority = if let Some(priority_str) = args.priority {
        Some(priority_str.parse::<TaskPriority>()
            .map_err(|e| DevErpError::Validation(e))?)
    } else {
        None
    };

    // Parse task type if provided
    let task_type = if let Some(type_str) = args.task_type {
        Some(type_str.parse::<TaskType>()
            .map_err(|e| DevErpError::Validation(e))?)
    } else {
        None
    };

    // Parse due date if provided
    let due_date = if let Some(date_str) = args.due_date {
        Some(parse_datetime(&date_str)?)
    } else {
        None
    };

    // Parse tags if provided
    let tags = args.tags.map(|tags_str| {
        tags_str.split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect()
    });

    // Create task input
    let input = CreateTask {
        project_id: args.project_id,
        parent_task_id: args.parent_task_id,
        title: args.title,
        description: args.description,
        task_number: args.task_number,
        status,
        priority,
        assigned_to: args.assigned_to,
        estimated_hours: args.estimated_hours,
        due_date,
        task_type,
        tags,
    };

    // Validate input
    input.validate()
        .map_err(|e| DevErpError::Validation(e))?;

    // Create task
    let task = service.create_task(input).await?;

    // Display success message
    println!("{} Task created successfully!", "✓".green().bold());
    println!();
    summary_line("ID", &task.id.to_string());
    summary_line("UUID", &task.uuid.to_string());
    summary_line("Title", &task.title);
    summary_line("Project ID", &task.project_id.to_string());
    if let Some(ref desc) = task.description {
        summary_line("Description", desc);
    }
    summary_line("Status", &task.status.to_string());
    summary_line("Priority", &task.priority.to_string());
    if let Some(ref task_type) = task.task_type {
        summary_line("Type", &task_type.to_string());
    }
    println!();

    Ok(())
}

/// Handle task list command
async fn handle_list(args: ListTaskArgs) -> Result<()> {
    let service = create_service().await?;

    // Parse status filter if provided
    let status = if let Some(status_str) = args.status {
        Some(status_str.parse::<TaskStatus>()
            .map_err(|e| DevErpError::Validation(e))?)
    } else {
        None
    };

    // Parse priority filter if provided
    let priority = if let Some(priority_str) = args.priority {
        Some(priority_str.parse::<TaskPriority>()
            .map_err(|e| DevErpError::Validation(e))?)
    } else {
        None
    };

    // Parse task type filter if provided
    let task_type = if let Some(type_str) = args.task_type {
        Some(type_str.parse::<TaskType>()
            .map_err(|e| DevErpError::Validation(e))?)
    } else {
        None
    };

    // Build filter
    let filter = TaskFilter {
        project_id: args.project_id,
        status,
        priority,
        task_type,
        assigned_to: args.assigned_to,
        parent_task_id: args.parent_task_id,
        include_deleted: false,
        offset: Some(args.pagination.offset()),
        limit: Some(args.pagination.limit()),
    };

    // Get tasks
    let tasks = service.list_tasks(filter).await?;

    // Display results
    if tasks.is_empty() {
        empty_state("tasks");
        return Ok(());
    }

    section_title(&format!("Tasks ({} found)", tasks.len()));
    println!();

    for task in &tasks {
        // Status color coding
        let status_str = match task.status {
            TaskStatus::Done => task.status.to_string().green(),
            TaskStatus::InProgress => task.status.to_string().cyan(),
            TaskStatus::Blocked => task.status.to_string().red(),
            TaskStatus::Cancelled => task.status.to_string().dimmed(),
            _ => task.status.to_string().yellow(),
        };

        // Priority indicator
        let priority_indicator = match task.priority {
            TaskPriority::Critical => "!!!".red().bold(),
            TaskPriority::High => "!!".yellow(),
            TaskPriority::Medium => "!".cyan(),
            TaskPriority::Low => "-".dimmed(),
        };

        println!("  {} {} {}",
            priority_indicator,
            task.title.bold(),
            status_str
        );
        println!("    ID: {} | UUID: {} | Project: {}",
            task.id.to_string().yellow(),
            task.uuid.to_string().dimmed(),
            task.project_id.to_string().cyan()
        );

        if let Some(ref desc) = task.description {
            let short_desc = if desc.len() > 80 {
                format!("{}...", &desc[..77])
            } else {
                desc.clone()
            };
            println!("    {}", short_desc.dimmed());
        }

        // Additional info
        let mut info_parts = vec![];
        if let Some(ref task_type) = task.task_type {
            info_parts.push(format!("Type: {}", task_type.to_string()));
        }
        if let Some(ref assigned_to) = task.assigned_to {
            info_parts.push(format!("Assigned: {}", assigned_to));
        }
        if let Some(ref due_date) = task.due_date {
            info_parts.push(format!("Due: {}", due_date.format("%Y-%m-%d")));
        }
        if !info_parts.is_empty() {
            println!("    {}", info_parts.join(" | ").dimmed());
        }

        println!();
    }

    // Show pagination info
    let output = PaginatedOutput::new(tasks, args.pagination.page, args.pagination.per_page);
    output.print_metadata();

    Ok(())
}

/// Handle task show command
async fn handle_show(args: ShowTaskArgs) -> Result<()> {
    let service = create_service().await?;

    // Try to parse as UUID first, then as ID
    let task = if let Ok(uuid) = Uuid::parse_str(&args.identifier) {
        service.get_task_by_uuid(uuid).await?
    } else if let Ok(id) = args.identifier.parse::<i64>() {
        service.get_task_by_id(id).await?
    } else {
        return Err(DevErpError::Validation(
            "Invalid identifier. Must be a valid UUID or numeric ID".to_string()
        ).into());
    };

    // Display task details
    section_title(&format!("Task: {}", task.title));
    println!();

    summary_line("ID", &task.id.to_string());
    summary_line("UUID", &task.uuid.to_string());
    summary_line("Title", &task.title);
    summary_line("Project ID", &task.project_id.to_string());

    if let Some(ref desc) = task.description {
        summary_line("Description", desc);
    }

    if let Some(parent_id) = task.parent_task_id {
        summary_line("Parent Task", &parent_id.to_string());
    }

    if let Some(ref task_number) = task.task_number {
        summary_line("Task Number", task_number);
    }

    summary_line("Status", &task.status.to_string());
    summary_line("Priority", &task.priority.to_string());

    if let Some(ref task_type) = task.task_type {
        summary_line("Type", &task_type.to_string());
    }

    if let Some(ref assigned_to) = task.assigned_to {
        summary_line("Assigned To", assigned_to);
    }

    if let Some(estimated_hours) = task.estimated_hours {
        summary_line("Estimated Hours", &format!("{:.2}", estimated_hours));
    }

    if let Some(actual_hours) = task.actual_hours {
        summary_line("Actual Hours", &format!("{:.2}", actual_hours));
    }

    if let Some(due_date) = task.due_date {
        summary_line("Due Date", &due_date.format("%Y-%m-%d %H:%M:%S").to_string());
    }

    if let Some(started_at) = task.started_at {
        summary_line("Started At", &started_at.format("%Y-%m-%d %H:%M:%S").to_string());
    }

    if let Some(completed_at) = task.completed_at {
        summary_line("Completed At", &completed_at.format("%Y-%m-%d %H:%M:%S").to_string());
    }

    if let Some(ref tags) = task.tags {
        if !tags.is_empty() {
            summary_line("Tags", &tags.join(", "));
        }
    }

    println!();
    summary_line("Created", &task.created_at.format("%Y-%m-%d %H:%M:%S").to_string());
    summary_line("Updated", &task.updated_at.format("%Y-%m-%d %H:%M:%S").to_string());

    // Get and display dependencies
    let dependencies = service.get_task_dependencies(task.id).await?;
    if !dependencies.is_empty() {
        println!();
        section_title("Dependencies");
        for dep in dependencies {
            println!("  {} Task {} depends on Task {} ({})",
                "→".cyan(),
                dep.task_id,
                dep.depends_on_task_id,
                dep.dependency_type.to_string().dimmed()
            );
        }
    }

    // Get and display comments
    let comments = service.get_task_comments(task.id).await?;
    if !comments.is_empty() {
        println!();
        section_title("Comments");
        for comment in comments {
            let author = comment.author.as_deref().unwrap_or("Unknown");
            println!("  {} {} - {}",
                "💬".cyan(),
                author.bold(),
                comment.created_at.format("%Y-%m-%d %H:%M:%S").to_string().dimmed()
            );
            println!("    {}", comment.comment_text);
            println!();
        }
    }

    println!();

    Ok(())
}

/// Handle task update command
async fn handle_update(args: UpdateTaskArgs) -> Result<()> {
    let service = create_service().await?;

    // Get the task ID
    let id = if let Ok(uuid) = Uuid::parse_str(&args.identifier) {
        let task = service.get_task_by_uuid(uuid).await?;
        task.id
    } else if let Ok(id) = args.identifier.parse::<i64>() {
        id
    } else {
        return Err(DevErpError::Validation(
            "Invalid identifier. Must be a valid UUID or numeric ID".to_string()
        ).into());
    };

    // Parse status if provided
    let status = if let Some(status_str) = args.status {
        Some(status_str.parse::<TaskStatus>()
            .map_err(|e| DevErpError::Validation(e))?)
    } else {
        None
    };

    // Parse priority if provided
    let priority = if let Some(priority_str) = args.priority {
        Some(priority_str.parse::<TaskPriority>()
            .map_err(|e| DevErpError::Validation(e))?)
    } else {
        None
    };

    // Parse task type if provided
    let task_type = if let Some(type_str) = args.task_type {
        Some(type_str.parse::<TaskType>()
            .map_err(|e| DevErpError::Validation(e))?)
    } else {
        None
    };

    // Parse due date if provided
    let due_date = if let Some(date_str) = args.due_date {
        Some(parse_datetime(&date_str)?)
    } else {
        None
    };

    // Parse tags if provided
    let tags = args.tags.map(|tags_str| {
        tags_str.split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect()
    });

    // Create update input
    let input = UpdateTask {
        id,
        title: args.title,
        description: args.description,
        status,
        priority,
        assigned_to: args.assigned_to,
        estimated_hours: args.estimated_hours,
        actual_hours: args.actual_hours,
        due_date,
        task_type,
        tags,
    };

    // Validate input
    input.validate()
        .map_err(|e| DevErpError::Validation(e))?;

    // Update task
    let task = service.update_task(input).await?;

    // Display success message
    println!("{} Task updated successfully!", "✓".green().bold());
    println!();
    summary_line("ID", &task.id.to_string());
    summary_line("Title", &task.title);
    summary_line("Status", &task.status.to_string());
    summary_line("Priority", &task.priority.to_string());
    println!();

    Ok(())
}

/// Handle task delete command
async fn handle_delete(args: DeleteTaskArgs) -> Result<()> {
    let service = create_service().await?;

    // Get the task
    let task = if let Ok(uuid) = Uuid::parse_str(&args.identifier) {
        service.get_task_by_uuid(uuid).await?
    } else if let Ok(id) = args.identifier.parse::<i64>() {
        service.get_task_by_id(id).await?
    } else {
        return Err(DevErpError::Validation(
            "Invalid identifier. Must be a valid UUID or numeric ID".to_string()
        ).into());
    };

    // Confirm deletion
    if !args.confirm {
        let confirmed = confirm(&format!(
            "Are you sure you want to delete task '{}'? This action cannot be undone.",
            task.title
        ));

        if !confirmed {
            println!("Deletion cancelled.");
            return Ok(());
        }
    }

    // Delete task
    service.delete_task(task.id).await?;

    println!("{} Task '{}' deleted successfully.", "✓".green().bold(), task.title);

    Ok(())
}

/// Handle add dependency command
async fn handle_add_dependency(args: AddDependencyArgs) -> Result<()> {
    let service = create_service().await?;

    // Parse dependency type if provided
    let dependency_type = if let Some(type_str) = args.dependency_type {
        Some(type_str.parse::<DependencyType>()
            .map_err(|e| DevErpError::Validation(e))?)
    } else {
        None
    };

    // Create dependency input
    let input = CreateTaskDependency {
        task_id: args.task_id,
        depends_on_task_id: args.depends_on_task_id,
        dependency_type,
    };

    // Validate input
    input.validate()
        .map_err(|e| DevErpError::Validation(e))?;

    // Add dependency
    let dependency = service.add_task_dependency(input).await?;

    // Display success message
    println!("{} Task dependency added successfully!", "✓".green().bold());
    println!();
    summary_line("Task ID", &dependency.task_id.to_string());
    summary_line("Depends On", &dependency.depends_on_task_id.to_string());
    summary_line("Type", &dependency.dependency_type.to_string());
    println!();

    Ok(())
}

/// Handle remove dependency command
async fn handle_remove_dependency(args: RemoveDependencyArgs) -> Result<()> {
    let service = create_service().await?;

    // Remove dependency
    service.remove_task_dependency(args.task_id, args.depends_on_task_id).await?;

    // Display success message
    println!("{} Task dependency removed successfully!", "✓".green().bold());
    println!();
    summary_line("Task ID", &args.task_id.to_string());
    summary_line("Removed Dependency On", &args.depends_on_task_id.to_string());
    println!();

    Ok(())
}

/// Handle add comment command
async fn handle_add_comment(args: AddCommentArgs) -> Result<()> {
    let service = create_service().await?;

    // Create comment input
    let input = CreateTaskComment {
        task_id: args.task_id,
        comment_text: args.comment,
        author: args.author,
    };

    // Validate input
    input.validate()
        .map_err(|e| DevErpError::Validation(e))?;

    // Add comment
    let comment = service.add_task_comment(input).await?;

    // Display success message
    println!("{} Comment added successfully!", "✓".green().bold());
    println!();
    summary_line("Comment ID", &comment.id.to_string());
    summary_line("Task ID", &comment.task_id.to_string());
    if let Some(ref author) = comment.author {
        summary_line("Author", author);
    }
    println!("  {}", comment.comment_text);
    println!();

    Ok(())
}

/// Parse datetime from string (supports both YYYY-MM-DD and YYYY-MM-DD HH:MM:SS formats)
fn parse_datetime(date_str: &str) -> Result<DateTime<Utc>> {
    // Try parsing as full datetime first
    if let Ok(dt) = DateTime::parse_from_str(date_str, "%Y-%m-%d %H:%M:%S %z") {
        return Ok(dt.with_timezone(&Utc));
    }

    // Try parsing as date only (assume midnight UTC)
    if let Ok(date) = NaiveDate::parse_from_str(date_str, "%Y-%m-%d") {
        let naive_dt = date.and_hms_opt(0, 0, 0)
            .ok_or_else(|| DevErpError::Validation("Invalid time component".to_string()))?;
        return Ok(DateTime::from_naive_utc_and_offset(naive_dt, Utc));
    }

    Err(DevErpError::Validation(
        format!("Invalid date format: {}. Expected YYYY-MM-DD or 'YYYY-MM-DD HH:MM:SS'", date_str)
    ).into())
}
