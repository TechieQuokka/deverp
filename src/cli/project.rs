// Project CLI commands implementation

use std::sync::Arc;
use chrono::NaiveDate;
use colored::Colorize;
use uuid::Uuid;

use super::commands::{
    ProjectCommand, CreateProjectArgs, ListProjectArgs, ShowProjectArgs,
    UpdateProjectArgs, DeleteProjectArgs, ArchiveProjectArgs, OutputFormat,
};
use super::output::{PaginatedOutput, section_title, summary_line, confirm, empty_state};
use crate::config::settings::Settings;
use crate::domain::project::{
    entity::{CreateProject, UpdateProject, ProjectFilter, ProjectStatus, Priority},
    service::ProjectService,
};
use crate::infrastructure::{database, repositories::project_repo::PostgresProjectRepository};
use crate::utils::error::DevErpError;
use crate::utils::formatter;
use crate::Result;

/// Handle project commands
pub async fn handle(command: ProjectCommand, format: OutputFormat) -> Result<()> {
    match command {
        ProjectCommand::Create(args) => handle_create(args, format).await,
        ProjectCommand::List(args) => handle_list(args, format).await,
        ProjectCommand::Show(args) => handle_show(args, format).await,
        ProjectCommand::Update(args) => handle_update(args, format).await,
        ProjectCommand::Delete(args) => handle_delete(args, format).await,
        ProjectCommand::Archive(args) => handle_archive(args, format).await,
    }
}

/// Create database connection and project service
async fn create_service() -> Result<ProjectService> {
    let settings = Settings::default();
    let pool = database::establish_connection(&settings.database).await?;
    let repository = Arc::new(PostgresProjectRepository::new(pool));
    Ok(ProjectService::new(repository))
}

/// Handle project create command
async fn handle_create(args: CreateProjectArgs, _format: OutputFormat) -> Result<()> {
    let service = create_service().await?;

    // Parse status if provided
    let status = if let Some(status_str) = args.status {
        Some(status_str.parse::<ProjectStatus>()
            .map_err(|e| DevErpError::Validation(e))?)
    } else {
        None
    };

    // Parse priority if provided
    let priority = if let Some(priority_str) = args.priority {
        Some(priority_str.parse::<Priority>()
            .map_err(|e| DevErpError::Validation(e))?)
    } else {
        None
    };

    // Parse dates if provided
    let start_date = if let Some(date_str) = args.start_date {
        Some(NaiveDate::parse_from_str(&date_str, "%Y-%m-%d")
            .map_err(|_| DevErpError::Validation(format!("Invalid start date format: {}. Expected YYYY-MM-DD", date_str)))?)
    } else {
        None
    };

    let end_date = if let Some(date_str) = args.end_date {
        Some(NaiveDate::parse_from_str(&date_str, "%Y-%m-%d")
            .map_err(|_| DevErpError::Validation(format!("Invalid end date format: {}. Expected YYYY-MM-DD", date_str)))?)
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

    // Create project input
    let input = CreateProject {
        name: args.name,
        description: args.description,
        code: args.code,
        status,
        priority,
        start_date,
        end_date,
        repository_url: args.repository_url,
        repository_branch: args.repository_branch,
        tags,
        metadata: None,
    };

    // Validate input
    input.validate()
        .map_err(|e| DevErpError::Validation(e))?;

    // Create project
    let project = service.create_project(input).await?;

    // Display success message
    println!("{} Project created successfully!", "✓".green().bold());
    println!();
    summary_line("ID", &project.id.to_string());
    summary_line("UUID", &project.uuid.to_string());
    summary_line("Name", &project.name);
    if let Some(ref desc) = project.description {
        summary_line("Description", desc);
    }
    if let Some(ref code) = project.code {
        summary_line("Code", code);
    }
    summary_line("Status", &project.status.to_string());
    summary_line("Priority", &project.priority.to_string());
    println!();

    Ok(())
}

/// Handle project list command
async fn handle_list(args: ListProjectArgs, format: OutputFormat) -> Result<()> {
    let service = create_service().await?;

    // Parse status filter if provided
    let status = if let Some(status_str) = args.status {
        Some(status_str.parse::<ProjectStatus>()
            .map_err(|e| DevErpError::Validation(e))?)
    } else {
        None
    };

    // Parse priority filter if provided
    let priority = if let Some(priority_str) = args.priority {
        Some(priority_str.parse::<Priority>()
            .map_err(|e| DevErpError::Validation(e))?)
    } else {
        None
    };

    // Parse tags filter if provided
    let tags = args.tags.map(|tags_str| {
        tags_str.split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect()
    });

    // Build filter
    let filter = ProjectFilter {
        status,
        priority,
        search: args.search,
        tags,
        offset: Some(args.pagination.offset()),
        limit: Some(args.pagination.limit()),
    };

    // Get projects
    let projects = service.list_projects(filter).await?;

    // Display results based on format
    match format {
        OutputFormat::Json => {
            return formatter::output_json(&projects);
        }
        _ => {
            // Table/Plain format
            if projects.is_empty() {
                empty_state("projects");
                return Ok(());
            }
        }
    }

    section_title(&format!("Projects ({} found)", projects.len()));
    println!();

    for project in &projects {
        println!("  {} {} - {}",
            "•".cyan(),
            project.name.bold(),
            project.status.to_string().dimmed()
        );
        println!("    ID: {} | UUID: {}",
            project.id.to_string().yellow(),
            project.uuid.to_string().dimmed()
        );

        if let Some(ref code) = project.code {
            println!("    Code: {}", code.cyan());
        }

        if let Some(ref desc) = project.description {
            let short_desc = if desc.len() > 100 {
                format!("{}...", &desc[..97])
            } else {
                desc.clone()
            };
            println!("    {}", short_desc.dimmed());
        }

        println!("    Priority: {} | Progress: {}%",
            project.priority.to_string().cyan(),
            project.progress_percentage.unwrap_or(0)
        );

        // Display dates if available
        if let Some(start_date) = project.start_date {
            print!("    ");
            if let Some(end_date) = project.end_date {
                println!("Period: {} → {}",
                    start_date.format("%Y-%m-%d").to_string().green(),
                    end_date.format("%Y-%m-%d").to_string().green()
                );
            } else {
                println!("Start: {}", start_date.format("%Y-%m-%d").to_string().green());
            }
        }

        // Display actual dates if available
        if project.actual_start_date.is_some() || project.actual_end_date.is_some() {
            print!("    Actual: ");
            if let Some(actual_start) = project.actual_start_date {
                print!("{}", actual_start.format("%Y-%m-%d").to_string().yellow());
                if let Some(actual_end) = project.actual_end_date {
                    print!(" → {}", actual_end.format("%Y-%m-%d").to_string().yellow());
                }
            } else if let Some(actual_end) = project.actual_end_date {
                print!("End: {}", actual_end.format("%Y-%m-%d").to_string().yellow());
            }
            println!();
        }

        // Display repository info if available
        if let Some(ref repo_url) = project.repository_url {
            print!("    Repository: {}", repo_url.blue());
            if let Some(ref branch) = project.repository_branch {
                print!(" ({})", branch.cyan());
            }
            println!();
        }

        // Display tags if available
        if let Some(ref tags) = project.tags {
            if !tags.is_empty() {
                println!("    Tags: {}",
                    tags.iter()
                        .map(|t| format!("#{}", t))
                        .collect::<Vec<_>>()
                        .join(" ")
                        .magenta()
                );
            }
        }

        println!();
    }

    // Show pagination info
    let output = PaginatedOutput::new(projects, args.pagination.page, args.pagination.per_page);
    output.print_metadata();

    Ok(())
}

/// Handle project show command
async fn handle_show(args: ShowProjectArgs, _format: OutputFormat) -> Result<()> {
    let service = create_service().await?;

    // Try to parse as UUID first, then as ID
    let project = if let Ok(uuid) = Uuid::parse_str(&args.identifier) {
        service.get_project_by_uuid(uuid).await?
    } else if let Ok(id) = args.identifier.parse::<i64>() {
        service.get_project(id).await?
    } else {
        return Err(DevErpError::Validation(
            "Invalid identifier. Must be a valid UUID or numeric ID".to_string()
        ).into());
    };

    // Display project details
    section_title(&format!("Project: {}", project.name));
    println!();

    summary_line("ID", &project.id.to_string());
    summary_line("UUID", &project.uuid.to_string());
    summary_line("Name", &project.name);

    if let Some(ref desc) = project.description {
        summary_line("Description", desc);
    }

    if let Some(ref code) = project.code {
        summary_line("Code", code);
    }

    summary_line("Status", &project.status.to_string());
    summary_line("Priority", &project.priority.to_string());
    summary_line("Progress", &format!("{}%", project.progress_percentage.unwrap_or(0)));

    if let Some(start_date) = project.start_date {
        summary_line("Start Date", &start_date.to_string());
    }

    if let Some(end_date) = project.end_date {
        summary_line("End Date", &end_date.to_string());
    }

    if let Some(actual_start) = project.actual_start_date {
        summary_line("Actual Start", &actual_start.to_string());
    }

    if let Some(actual_end) = project.actual_end_date {
        summary_line("Actual End", &actual_end.to_string());
    }

    if let Some(ref repo_url) = project.repository_url {
        summary_line("Repository", repo_url);
        if let Some(ref branch) = project.repository_branch {
            summary_line("Branch", branch);
        }
    }

    if let Some(ref tags) = project.tags {
        if !tags.is_empty() {
            summary_line("Tags", &tags.join(", "));
        }
    }

    println!();
    summary_line("Created", &project.created_at.format("%Y-%m-%d %H:%M:%S").to_string());
    summary_line("Updated", &project.updated_at.format("%Y-%m-%d %H:%M:%S").to_string());

    println!();

    Ok(())
}

/// Handle project update command
async fn handle_update(args: UpdateProjectArgs, _format: OutputFormat) -> Result<()> {
    let service = create_service().await?;

    // Get the project ID
    let id = if let Ok(uuid) = Uuid::parse_str(&args.identifier) {
        let project = service.get_project_by_uuid(uuid).await?;
        project.id
    } else if let Ok(id) = args.identifier.parse::<i64>() {
        id
    } else {
        return Err(DevErpError::Validation(
            "Invalid identifier. Must be a valid UUID or numeric ID".to_string()
        ).into());
    };

    // Parse status if provided
    let status = if let Some(status_str) = args.status {
        Some(status_str.parse::<ProjectStatus>()
            .map_err(|e| DevErpError::Validation(e))?)
    } else {
        None
    };

    // Parse priority if provided
    let priority = if let Some(priority_str) = args.priority {
        Some(priority_str.parse::<Priority>()
            .map_err(|e| DevErpError::Validation(e))?)
    } else {
        None
    };

    // Parse dates if provided
    let start_date = if let Some(date_str) = args.start_date {
        Some(NaiveDate::parse_from_str(&date_str, "%Y-%m-%d")
            .map_err(|_| DevErpError::Validation(format!("Invalid start date format: {}. Expected YYYY-MM-DD", date_str)))?)
    } else {
        None
    };

    let end_date = if let Some(date_str) = args.end_date {
        Some(NaiveDate::parse_from_str(&date_str, "%Y-%m-%d")
            .map_err(|_| DevErpError::Validation(format!("Invalid end date format: {}. Expected YYYY-MM-DD", date_str)))?)
    } else {
        None
    };

    let actual_start_date = if let Some(date_str) = args.actual_start_date {
        Some(NaiveDate::parse_from_str(&date_str, "%Y-%m-%d")
            .map_err(|_| DevErpError::Validation(format!("Invalid actual start date format: {}. Expected YYYY-MM-DD", date_str)))?)
    } else {
        None
    };

    let actual_end_date = if let Some(date_str) = args.actual_end_date {
        Some(NaiveDate::parse_from_str(&date_str, "%Y-%m-%d")
            .map_err(|_| DevErpError::Validation(format!("Invalid actual end date format: {}. Expected YYYY-MM-DD", date_str)))?)
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
    let input = UpdateProject {
        id,
        name: args.name,
        description: args.description,
        code: args.code,
        status,
        priority,
        start_date,
        end_date,
        actual_start_date,
        actual_end_date,
        progress_percentage: args.progress,
        repository_url: args.repository_url,
        repository_branch: args.repository_branch,
        tags,
        metadata: None,
    };

    // Validate input
    input.validate()
        .map_err(|e| DevErpError::Validation(e))?;

    // Update project
    let project = service.update_project(input).await?;

    // Display success message
    println!("{} Project updated successfully!", "✓".green().bold());
    println!();
    summary_line("ID", &project.id.to_string());
    summary_line("Name", &project.name);
    summary_line("Status", &project.status.to_string());
    summary_line("Priority", &project.priority.to_string());
    summary_line("Progress", &format!("{}%", project.progress_percentage.unwrap_or(0)));
    println!();

    Ok(())
}

/// Handle project delete command
async fn handle_delete(args: DeleteProjectArgs, _format: OutputFormat) -> Result<()> {
    let service = create_service().await?;

    // Get the project
    let project = if let Ok(uuid) = Uuid::parse_str(&args.identifier) {
        service.get_project_by_uuid(uuid).await?
    } else if let Ok(id) = args.identifier.parse::<i64>() {
        service.get_project(id).await?
    } else {
        return Err(DevErpError::Validation(
            "Invalid identifier. Must be a valid UUID or numeric ID".to_string()
        ).into());
    };

    // Confirm deletion
    if !args.confirm {
        let confirmed = confirm(&format!(
            "Are you sure you want to delete project '{}'? This action cannot be undone.",
            project.name
        ));

        if !confirmed {
            println!("Deletion cancelled.");
            return Ok(());
        }
    }

    // Delete project
    service.delete_project(project.id).await?;

    println!("{} Project '{}' deleted successfully.", "✓".green().bold(), project.name);

    Ok(())
}

/// Handle project archive command
async fn handle_archive(args: ArchiveProjectArgs, _format: OutputFormat) -> Result<()> {
    let service = create_service().await?;

    // Get the project
    let project = if let Ok(uuid) = Uuid::parse_str(&args.identifier) {
        service.get_project_by_uuid(uuid).await?
    } else if let Ok(id) = args.identifier.parse::<i64>() {
        service.get_project(id).await?
    } else {
        return Err(DevErpError::Validation(
            "Invalid identifier. Must be a valid UUID or numeric ID".to_string()
        ).into());
    };

    // Archive project (set status to Archived)
    let archived_project = service.archive_project(project.id).await?;

    println!("{} Project '{}' archived successfully.", "✓".green().bold(), archived_project.name);
    summary_line("Status", &archived_project.status.to_string());
    println!();

    Ok(())
}
