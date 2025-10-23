// Timeline CLI commands implementation

use std::sync::Arc;
use chrono::{NaiveDate, Local};
use colored::Colorize;

use super::commands::{
    TimelineCommand, CreateTimelineArgs, ListTimelineArgs, ShowTimelineArgs,
    UpdateTimelineArgs, DeleteTimelineArgs, AddMilestoneArgs, UpdateMilestoneArgs,
    CompleteMilestoneArgs, OutputFormat,
};
use super::output::{section_title, summary_line, confirm, empty_state};
use crate::config::settings::Settings;
use crate::domain::timeline::{
    entity::{
        CreateTimeline, UpdateTimeline, TimelineFilter, TimelineType, TimelineStatus,
        CreateMilestone, UpdateMilestone, MilestoneStatus,
    },
    service::TimelineService,
};
use crate::infrastructure::{
    database,
    repositories::timeline_repo::{PostgresTimelineRepository, PostgresMilestoneRepository},
};
use crate::utils::error::DevErpError;
use crate::Result;

/// Handle timeline commands
pub async fn handle(command: TimelineCommand, _format: OutputFormat) -> Result<()> {
    match command {
        TimelineCommand::Create(args) => handle_create(args).await,
        TimelineCommand::List(args) => handle_list(args).await,
        TimelineCommand::Show(args) => handle_show(args).await,
        TimelineCommand::Update(args) => handle_update(args).await,
        TimelineCommand::Delete(args) => handle_delete(args).await,
        TimelineCommand::AddMilestone(args) => handle_add_milestone(args).await,
        TimelineCommand::UpdateMilestone(args) => handle_update_milestone(args).await,
        TimelineCommand::CompleteMilestone(args) => handle_complete_milestone(args).await,
    }
}

/// Create database connection and timeline service
async fn create_service() -> Result<TimelineService> {
    let settings = Settings::default();
    let pool = database::establish_connection(&settings.database).await?;
    let timeline_repository = Arc::new(PostgresTimelineRepository::new(pool.clone()));
    let milestone_repository = Arc::new(PostgresMilestoneRepository::new(pool));
    Ok(TimelineService::new(timeline_repository, milestone_repository))
}

/// Handle timeline create command
async fn handle_create(args: CreateTimelineArgs) -> Result<()> {
    let service = create_service().await?;

    // Parse timeline type
    let timeline_type = args.timeline_type.parse::<TimelineType>()
        .map_err(|e| DevErpError::Validation(e))?;

    // Parse status if provided
    let status = if let Some(status_str) = args.status {
        Some(status_str.parse::<TimelineStatus>()
            .map_err(|e| DevErpError::Validation(e))?)
    } else {
        None
    };

    // Parse dates
    let start_date = NaiveDate::parse_from_str(&args.start_date, "%Y-%m-%d")
        .map_err(|_| DevErpError::Validation(
            format!("Invalid start date format: {}. Expected YYYY-MM-DD", args.start_date)
        ))?;

    let end_date = NaiveDate::parse_from_str(&args.end_date, "%Y-%m-%d")
        .map_err(|_| DevErpError::Validation(
            format!("Invalid end date format: {}. Expected YYYY-MM-DD", args.end_date)
        ))?;

    // Create timeline input
    let input = CreateTimeline {
        project_id: args.project_id,
        name: args.name,
        description: args.description,
        timeline_type: Some(timeline_type),
        start_date,
        end_date,
        status,
    };

    // Create timeline
    let timeline = service.create_timeline(input).await?;

    // Display success message
    section_title("Timeline Created");
    println!("{}: {}", "ID".bright_cyan(), timeline.id);
    println!("{}: {}", "Name".bright_cyan(), timeline.name.bold());
    println!("{}: {}", "Project ID".bright_cyan(), timeline.project_id);
    println!("{}: {}", "Type".bright_cyan(), timeline.timeline_type);
    println!("{}: {}", "Status".bright_cyan(), timeline.status);
    println!("{}: {} to {}", "Period".bright_cyan(), timeline.start_date, timeline.end_date);
    if let Some(desc) = &timeline.description {
        println!("{}: {}", "Description".bright_cyan(), desc);
    }
    println!("{}: {}", "Created".bright_cyan(), timeline.created_at.format("%Y-%m-%d %H:%M:%S"));

    Ok(())
}

/// Handle timeline list command
async fn handle_list(args: ListTimelineArgs) -> Result<()> {
    let service = create_service().await?;

    // Parse timeline type if provided
    let timeline_type = if let Some(type_str) = args.timeline_type {
        Some(type_str.parse::<TimelineType>()
            .map_err(|e| DevErpError::Validation(e))?)
    } else {
        None
    };

    // Parse status if provided
    let status = if let Some(status_str) = args.status {
        Some(status_str.parse::<TimelineStatus>()
            .map_err(|e| DevErpError::Validation(e))?)
    } else {
        None
    };

    // Create filter
    let filter = TimelineFilter {
        project_id: args.project_id,
        timeline_type,
        status,
        offset: Some(args.pagination.offset()),
        limit: Some(args.pagination.limit()),
    };

    // Get timelines
    let timelines = service.list_timelines(filter).await?;

    if timelines.is_empty() {
        empty_state("No timelines found");
        return Ok(());
    }

    // Display timelines
    section_title(&format!("Timelines ({})", timelines.len()));
    println!();

    for timeline in timelines {
        println!("  {} {}", "●".bright_green(), timeline.name.bold());
        println!("    {}: {} | {}: {}",
            "ID".dimmed(), timeline.id,
            "Project".dimmed(), timeline.project_id
        );
        println!("    {}: {} | {}: {}",
            "Type".dimmed(), timeline.timeline_type,
            "Status".dimmed(), timeline.status
        );
        println!("    {}: {} to {}",
            "Period".dimmed(), timeline.start_date, timeline.end_date
        );
        if let Some(desc) = &timeline.description {
            let short_desc = if desc.len() > 60 {
                format!("{}...", &desc[..60])
            } else {
                desc.clone()
            };
            println!("    {}: {}", "Description".dimmed(), short_desc);
        }
        println!();
    }

    Ok(())
}

/// Handle timeline show command
async fn handle_show(args: ShowTimelineArgs) -> Result<()> {
    let service = create_service().await?;

    // Get timeline
    let timeline = service.get_timeline(args.id).await?;

    // Get milestones for this timeline
    let milestones = service.get_milestones_by_timeline(args.id).await?;

    // Display timeline details
    section_title("Timeline Details");
    println!();
    println!("{}: {}", "ID".bright_cyan(), timeline.id);
    println!("{}: {}", "Name".bright_cyan(), timeline.name.bold());
    println!("{}: {}", "Project ID".bright_cyan(), timeline.project_id);
    println!("{}: {}", "Type".bright_cyan(), timeline.timeline_type);
    println!("{}: {}", "Status".bright_cyan(), timeline.status);
    println!("{}: {} to {}", "Period".bright_cyan(), timeline.start_date, timeline.end_date);

    if let Some(desc) = &timeline.description {
        println!();
        println!("{}:", "Description".bright_cyan());
        println!("  {}", desc);
    }

    println!();
    println!("{}: {}", "Created".bright_cyan(), timeline.created_at.format("%Y-%m-%d %H:%M:%S"));
    println!("{}: {}", "Updated".bright_cyan(), timeline.updated_at.format("%Y-%m-%d %H:%M:%S"));

    // Display milestones
    if !milestones.is_empty() {
        println!();
        section_title(&format!("Milestones ({})", milestones.len()));
        println!();

        for milestone in milestones {
            let status_color = match milestone.status.as_str() {
                "completed" => "✓".bright_green(),
                "in_progress" => "◐".bright_yellow(),
                "missed" => "✗".bright_red(),
                _ => "○".dimmed(),
            };

            println!("  {} {}", status_color, milestone.name.bold());
            println!("    {}: {} | {}: {}%",
                "ID".dimmed(), milestone.id,
                "Progress".dimmed(), milestone.completion_percentage
            );
            println!("    {}: {} | {}: {}",
                "Target".dimmed(), milestone.target_date,
                "Status".dimmed(), milestone.status
            );
            if let Some(actual) = milestone.actual_date {
                println!("    {}: {}", "Completed".dimmed(), actual);
            }
            if let Some(desc) = &milestone.description {
                let short_desc = if desc.len() > 50 {
                    format!("{}...", &desc[..50])
                } else {
                    desc.clone()
                };
                println!("    {}: {}", "Description".dimmed(), short_desc);
            }
            println!();
        }
    }

    Ok(())
}

/// Handle timeline update command
async fn handle_update(args: UpdateTimelineArgs) -> Result<()> {
    let service = create_service().await?;

    // Parse timeline type if provided
    let timeline_type = if let Some(type_str) = args.timeline_type {
        Some(type_str.parse::<TimelineType>()
            .map_err(|e| DevErpError::Validation(e))?)
    } else {
        None
    };

    // Parse status if provided
    let status = if let Some(status_str) = args.status {
        Some(status_str.parse::<TimelineStatus>()
            .map_err(|e| DevErpError::Validation(e))?)
    } else {
        None
    };

    // Parse dates if provided
    let start_date = if let Some(date_str) = args.start_date {
        Some(NaiveDate::parse_from_str(&date_str, "%Y-%m-%d")
            .map_err(|_| DevErpError::Validation(
                format!("Invalid start date format: {}. Expected YYYY-MM-DD", date_str)
            ))?)
    } else {
        None
    };

    let end_date = if let Some(date_str) = args.end_date {
        Some(NaiveDate::parse_from_str(&date_str, "%Y-%m-%d")
            .map_err(|_| DevErpError::Validation(
                format!("Invalid end date format: {}. Expected YYYY-MM-DD", date_str)
            ))?)
    } else {
        None
    };

    // Create update input
    let input = UpdateTimeline {
        id: args.id,
        name: args.name,
        description: args.description,
        timeline_type,
        start_date,
        end_date,
        status,
    };

    // Update timeline
    let timeline = service.update_timeline(input).await?;

    // Display success message
    section_title("Timeline Updated");
    println!("{}: {}", "ID".bright_cyan(), timeline.id);
    println!("{}: {}", "Name".bright_cyan(), timeline.name.bold());
    println!("{}: {}", "Updated".bright_cyan(), timeline.updated_at.format("%Y-%m-%d %H:%M:%S"));

    Ok(())
}

/// Handle timeline delete command
async fn handle_delete(args: DeleteTimelineArgs) -> Result<()> {
    let service = create_service().await?;

    // Get timeline to display name
    let timeline = service.get_timeline(args.id).await?;

    // Confirm deletion
    if !args.confirm {
        let confirmed = confirm(&format!("Are you sure you want to delete timeline '{}'?", timeline.name));
        if !confirmed {
            println!("Deletion cancelled.");
            return Ok(());
        }
    }

    // Delete timeline
    service.delete_timeline(args.id).await?;

    summary_line("Timeline Deleted", &format!("'{}' deleted successfully", timeline.name));

    Ok(())
}

/// Handle add milestone command
async fn handle_add_milestone(args: AddMilestoneArgs) -> Result<()> {
    let service = create_service().await?;

    // Parse status if provided
    let status = if let Some(status_str) = args.status {
        Some(status_str.parse::<MilestoneStatus>()
            .map_err(|e| DevErpError::Validation(e))?)
    } else {
        None
    };

    // Parse target date
    let target_date = NaiveDate::parse_from_str(&args.target_date, "%Y-%m-%d")
        .map_err(|_| DevErpError::Validation(
            format!("Invalid target date format: {}. Expected YYYY-MM-DD", args.target_date)
        ))?;

    // Create milestone input
    let input = CreateMilestone {
        timeline_id: args.timeline_id,
        project_id: args.project_id,
        name: args.name,
        description: args.description,
        target_date,
        status,
        completion_percentage: None,
        metadata: None,
    };

    // Create milestone
    let milestone = service.create_milestone(input).await?;

    // Display success message
    section_title("Milestone Added");
    println!("{}: {}", "ID".bright_cyan(), milestone.id);
    println!("{}: {}", "Name".bright_cyan(), milestone.name.bold());
    println!("{}: {}", "Timeline ID".bright_cyan(), milestone.timeline_id);
    println!("{}: {}", "Target Date".bright_cyan(), milestone.target_date);
    println!("{}: {}", "Status".bright_cyan(), milestone.status);
    if let Some(desc) = &milestone.description {
        println!("{}: {}", "Description".bright_cyan(), desc);
    }
    println!("{}: {}", "Created".bright_cyan(), milestone.created_at.format("%Y-%m-%d %H:%M:%S"));

    Ok(())
}

/// Handle update milestone command
async fn handle_update_milestone(args: UpdateMilestoneArgs) -> Result<()> {
    let service = create_service().await?;

    // Parse status if provided
    let status = if let Some(status_str) = args.status {
        Some(status_str.parse::<MilestoneStatus>()
            .map_err(|e| DevErpError::Validation(e))?)
    } else {
        None
    };

    // Parse target date if provided
    let target_date = if let Some(date_str) = args.target_date {
        Some(NaiveDate::parse_from_str(&date_str, "%Y-%m-%d")
            .map_err(|_| DevErpError::Validation(
                format!("Invalid target date format: {}. Expected YYYY-MM-DD", date_str)
            ))?)
    } else {
        None
    };

    // Parse actual date if provided
    let actual_date = if let Some(date_str) = args.actual_date {
        Some(NaiveDate::parse_from_str(&date_str, "%Y-%m-%d")
            .map_err(|_| DevErpError::Validation(
                format!("Invalid actual date format: {}. Expected YYYY-MM-DD", date_str)
            ))?)
    } else {
        None
    };

    // Validate completion percentage
    if let Some(pct) = args.completion_percentage {
        if pct < 0 || pct > 100 {
            return Err(DevErpError::Validation(
                "Completion percentage must be between 0 and 100".to_string()
            ).into());
        }
    }

    // Create update input
    let input = UpdateMilestone {
        id: args.id,
        name: args.name,
        description: args.description,
        target_date,
        actual_date,
        status,
        completion_percentage: args.completion_percentage,
        metadata: None,
    };

    // Update milestone
    let milestone = service.update_milestone(input).await?;

    // Display success message
    section_title("Milestone Updated");
    println!("{}: {}", "ID".bright_cyan(), milestone.id);
    println!("{}: {}", "Name".bright_cyan(), milestone.name.bold());
    println!("{}: {}%", "Progress".bright_cyan(), milestone.completion_percentage);
    println!("{}: {}", "Status".bright_cyan(), milestone.status);
    println!("{}: {}", "Updated".bright_cyan(), milestone.updated_at.format("%Y-%m-%d %H:%M:%S"));

    Ok(())
}

/// Handle complete milestone command
async fn handle_complete_milestone(args: CompleteMilestoneArgs) -> Result<()> {
    let service = create_service().await?;

    // Parse actual date or use today
    let actual_date = if let Some(date_str) = args.actual_date {
        NaiveDate::parse_from_str(&date_str, "%Y-%m-%d")
            .map_err(|_| DevErpError::Validation(
                format!("Invalid actual date format: {}. Expected YYYY-MM-DD", date_str)
            ))?
    } else {
        Local::now().naive_local().date()
    };

    // Update milestone to completed status
    let input = UpdateMilestone {
        id: args.id,
        name: None,
        description: None,
        target_date: None,
        actual_date: Some(actual_date),
        status: Some(MilestoneStatus::Completed),
        completion_percentage: Some(100),
        metadata: None,
    };

    let milestone = service.update_milestone(input).await?;

    // Display success message
    section_title("Milestone Completed");
    println!("{}: {}", "ID".bright_cyan(), milestone.id);
    println!("{}: {}", "Name".bright_cyan(), milestone.name.bold());
    println!("{}: {}", "Completed On".bright_cyan(), milestone.actual_date.unwrap());
    println!("{}: {}", "Status".bright_cyan(), milestone.status);

    Ok(())
}
