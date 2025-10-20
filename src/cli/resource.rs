// Resource CLI commands implementation

use std::sync::Arc;
use colored::Colorize;
use uuid::Uuid;

use super::commands::{
    ResourceCommand, CreateResourceArgs, ListResourceArgs, ShowResourceArgs,
    UpdateResourceArgs, DeleteResourceArgs, LinkResourceArgs, UnlinkResourceArgs,
    UsageResourceArgs,
};
use super::output::{section_title, summary_line, confirm, empty_state};
use crate::config::settings::Settings;
use crate::domain::resource::{
    entity::{CreateResource, UpdateResource, ResourceFilter, ResourceType, ResourceStatus, LinkResourceToProject},
    service::ResourceService,
};
use crate::infrastructure::{database, repositories::resource_repo::PostgresResourceRepository};
use crate::utils::error::DevErpError;
use crate::Result;

/// Handle resource commands
pub async fn handle(command: ResourceCommand) -> Result<()> {
    match command {
        ResourceCommand::Create(args) => handle_create(args).await,
        ResourceCommand::List(args) => handle_list(args).await,
        ResourceCommand::Show(args) => handle_show(args).await,
        ResourceCommand::Update(args) => handle_update(args).await,
        ResourceCommand::Delete(args) => handle_delete(args).await,
        ResourceCommand::Link(args) => handle_link(args).await,
        ResourceCommand::Unlink(args) => handle_unlink(args).await,
        ResourceCommand::Usage(args) => handle_usage(args).await,
    }
}

/// Create database connection and resource service
async fn create_service() -> Result<ResourceService> {
    let settings = Settings::default();
    let pool = database::establish_connection(&settings.database).await?;
    let repository = Arc::new(PostgresResourceRepository::new(pool));
    Ok(ResourceService::new(repository))
}

/// Handle resource create command
async fn handle_create(args: CreateResourceArgs) -> Result<()> {
    let service = create_service().await?;

    // Parse resource type
    let resource_type = args.resource_type.parse::<ResourceType>()
        .map_err(|e| DevErpError::Validation(e))?;

    // Parse status if provided
    let status = if let Some(status_str) = args.status {
        Some(status_str.parse::<ResourceStatus>()
            .map_err(|e| DevErpError::Validation(e))?)
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

    // Create resource input
    let input = CreateResource {
        name: args.name,
        description: args.description,
        resource_type,
        version: args.version,
        url: args.url,
        documentation_url: args.documentation_url,
        license: args.license,
        status,
        metadata: None,
        tags,
    };

    // Create resource
    let resource = service.create_resource(input).await?;

    // Display success message
    section_title("Resource Created");
    println!("{}: {}", "ID".bright_cyan(), resource.id);
    println!("{}: {}", "UUID".bright_cyan(), resource.uuid);
    println!("{}: {}", "Name".bright_cyan(), resource.name.bold());
    println!("{}: {}", "Type".bright_cyan(), resource.resource_type);
    if let Some(desc) = &resource.description {
        println!("{}: {}", "Description".bright_cyan(), desc);
    }
    if let Some(version) = &resource.version {
        println!("{}: {}", "Version".bright_cyan(), version);
    }
    if let Some(url) = &resource.url {
        println!("{}: {}", "URL".bright_cyan(), url);
    }
    println!("{}: {}", "Created".bright_cyan(), resource.created_at.format("%Y-%m-%d %H:%M:%S"));

    Ok(())
}

/// Handle resource list command
async fn handle_list(args: ListResourceArgs) -> Result<()> {
    let service = create_service().await?;

    // Parse resource type if provided
    let resource_type = if let Some(type_str) = args.resource_type {
        Some(type_str.parse::<ResourceType>()
            .map_err(|e| DevErpError::Validation(e))?)
    } else {
        None
    };

    // Parse status if provided
    let status = if let Some(status_str) = args.status {
        Some(status_str.parse::<ResourceStatus>()
            .map_err(|e| DevErpError::Validation(e))?)
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

    // Create filter
    let filter = ResourceFilter {
        resource_type,
        status,
        name_contains: args.search,
        tags,
        offset: Some(args.pagination.offset()),
        limit: Some(args.pagination.limit()),
    };

    // Get resources
    let resources = service.list_resources(filter).await?;

    if resources.is_empty() {
        empty_state("No resources found");
        return Ok(());
    }

    // Display resources
    section_title(&format!("Resources ({})", resources.len()));
    println!();

    for resource in resources {
        println!("  {} {}", "●".bright_green(), resource.name.bold());
        println!("    {}: {} | {}: {}",
            "ID".dimmed(), resource.id,
            "UUID".dimmed(), resource.uuid
        );
        println!("    {}: {} | {}: {}",
            "Type".dimmed(), resource.resource_type,
            "Status".dimmed(),
            resource.status.as_ref().map(|s| format!("{}", s)).unwrap_or_else(|| "active".to_string())
        );
        if let Some(version) = &resource.version {
            println!("    {}: {}", "Version".dimmed(), version);
        }
        if let Some(desc) = &resource.description {
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

/// Handle resource show command
async fn handle_show(args: ShowResourceArgs) -> Result<()> {
    let service = create_service().await?;

    // Try to parse as UUID first, otherwise as ID
    let resource = if let Ok(uuid) = args.identifier.parse::<Uuid>() {
        service.get_resource_by_uuid(uuid).await?
    } else {
        let id = args.identifier.parse::<i64>()
            .map_err(|_| DevErpError::Validation(
                format!("Invalid resource identifier: {}. Must be a valid ID or UUID", args.identifier)
            ))?;
        service.get_resource(id).await?
    };

    // Display resource details
    section_title("Resource Details");
    println!();
    println!("{}: {}", "ID".bright_cyan(), resource.id);
    println!("{}: {}", "UUID".bright_cyan(), resource.uuid);
    println!("{}: {}", "Name".bright_cyan(), resource.name.bold());
    println!("{}: {}", "Type".bright_cyan(), resource.resource_type);
    println!("{}: {}", "Status".bright_cyan(),
        resource.status.as_ref().map(|s| format!("{}", s)).unwrap_or_else(|| "active".to_string())
    );

    if let Some(desc) = &resource.description {
        println!();
        println!("{}:", "Description".bright_cyan());
        println!("  {}", desc);
    }

    if let Some(version) = &resource.version {
        println!();
        println!("{}: {}", "Version".bright_cyan(), version);
    }

    if let Some(url) = &resource.url {
        println!("{}: {}", "URL".bright_cyan(), url);
    }

    if let Some(doc_url) = &resource.documentation_url {
        println!("{}: {}", "Documentation URL".bright_cyan(), doc_url);
    }

    if let Some(license) = &resource.license {
        println!("{}: {}", "License".bright_cyan(), license);
    }

    if let Some(tags) = &resource.tags {
        if !tags.is_empty() {
            println!();
            println!("{}:", "Tags".bright_cyan());
            for tag in tags {
                println!("  - {}", tag);
            }
        }
    }

    println!();
    println!("{}: {}", "Created".bright_cyan(), resource.created_at.format("%Y-%m-%d %H:%M:%S"));
    println!("{}: {}", "Updated".bright_cyan(), resource.updated_at.format("%Y-%m-%d %H:%M:%S"));

    Ok(())
}

/// Handle resource update command
async fn handle_update(args: UpdateResourceArgs) -> Result<()> {
    let service = create_service().await?;

    // Try to parse as UUID first, otherwise as ID
    let id = if let Ok(uuid) = args.identifier.parse::<Uuid>() {
        let resource = service.get_resource_by_uuid(uuid).await?;
        resource.id
    } else {
        args.identifier.parse::<i64>()
            .map_err(|_| DevErpError::Validation(
                format!("Invalid resource identifier: {}. Must be a valid ID or UUID", args.identifier)
            ))?
    };

    // Parse resource type if provided
    let resource_type = if let Some(type_str) = args.resource_type {
        Some(type_str.parse::<ResourceType>()
            .map_err(|e| DevErpError::Validation(e))?)
    } else {
        None
    };

    // Parse status if provided
    let status = if let Some(status_str) = args.status {
        Some(status_str.parse::<ResourceStatus>()
            .map_err(|e| DevErpError::Validation(e))?)
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
    let input = UpdateResource {
        id,
        name: args.name,
        description: args.description,
        resource_type,
        version: args.version,
        url: args.url,
        documentation_url: args.documentation_url,
        license: args.license,
        status,
        metadata: None,
        tags,
    };

    // Update resource
    let resource = service.update_resource(input).await?;

    // Display success message
    section_title("Resource Updated");
    println!("{}: {}", "ID".bright_cyan(), resource.id);
    println!("{}: {}", "Name".bright_cyan(), resource.name.bold());
    println!("{}: {}", "Updated".bright_cyan(), resource.updated_at.format("%Y-%m-%d %H:%M:%S"));

    Ok(())
}

/// Handle resource delete command
async fn handle_delete(args: DeleteResourceArgs) -> Result<()> {
    let service = create_service().await?;

    // Try to parse as UUID first, otherwise as ID
    let id = if let Ok(uuid) = args.identifier.parse::<Uuid>() {
        let resource = service.get_resource_by_uuid(uuid).await?;
        resource.id
    } else {
        args.identifier.parse::<i64>()
            .map_err(|_| DevErpError::Validation(
                format!("Invalid resource identifier: {}. Must be a valid ID or UUID", args.identifier)
            ))?
    };

    // Get resource to display name
    let resource = service.get_resource(id).await?;

    // Confirm deletion
    if !args.confirm {
        let confirmed = confirm(&format!("Are you sure you want to delete resource '{}'?", resource.name));
        if !confirmed {
            println!("Deletion cancelled.");
            return Ok(());
        }
    }

    // Delete resource
    service.delete_resource(id).await?;

    summary_line("Resource Deleted", &format!("'{}' deleted successfully", resource.name));

    Ok(())
}

/// Handle resource link command
async fn handle_link(args: LinkResourceArgs) -> Result<()> {
    let service = create_service().await?;

    // Create link input
    let input = LinkResourceToProject {
        project_id: args.project_id,
        resource_id: args.resource_id,
        usage_notes: args.usage_notes,
        version_used: args.version_used,
        is_critical: Some(args.is_critical),
    };

    // Link resource to project
    let project_resource = service.link_resource_to_project(input).await?;

    // Display success message
    section_title("Resource Linked to Project");
    println!("{}: {}", "Project ID".bright_cyan(), project_resource.project_id);
    println!("{}: {}", "Resource ID".bright_cyan(), project_resource.resource_id);
    if let Some(notes) = &project_resource.usage_notes {
        println!("{}: {}", "Usage Notes".bright_cyan(), notes);
    }
    if let Some(version) = &project_resource.version_used {
        println!("{}: {}", "Version Used".bright_cyan(), version);
    }
    if let Some(critical) = project_resource.is_critical {
        println!("{}: {}", "Critical".bright_cyan(), if critical { "Yes" } else { "No" });
    }

    Ok(())
}

/// Handle resource unlink command
async fn handle_unlink(args: UnlinkResourceArgs) -> Result<()> {
    let service = create_service().await?;

    // Unlink resource from project
    service.unlink_resource_from_project(args.project_id, args.resource_id).await?;

    summary_line(
        "Resource Unlinked",
        &format!("Resource {} unlinked from project {}", args.resource_id, args.project_id)
    );

    Ok(())
}

/// Handle resource usage command
async fn handle_usage(args: UsageResourceArgs) -> Result<()> {
    let service = create_service().await?;

    if let Some(resource_id) = args.resource_id {
        // Get usage for specific resource
        let stats = service.get_resource_usage(resource_id).await?;

        section_title(&format!("Resource Usage: {}", stats.resource_name));
        println!();
        println!("{}: {}", "Resource ID".bright_cyan(), stats.resource_id);
        println!("{}: {}", "Resource Type".bright_cyan(), stats.resource_type);
        println!("{}: {}", "Total Projects".bright_cyan(), stats.total_projects);
        println!("{}: {}", "Critical Projects".bright_cyan(), stats.critical_projects);
    } else {
        // Get usage for all resources
        let all_stats = service.get_all_resource_usage().await?;

        if all_stats.is_empty() {
            empty_state("No resources found");
            return Ok(());
        }

        section_title(&format!("Resource Usage Statistics ({})", all_stats.len()));
        println!();

        for stats in all_stats {
            println!("  {} {}", "●".bright_green(), stats.resource_name.bold());
            println!("    {}: {} | {}: {}",
                "ID".dimmed(), stats.resource_id,
                "Type".dimmed(), stats.resource_type
            );
            println!("    {}: {} | {}: {}",
                "Projects".dimmed(), stats.total_projects,
                "Critical".dimmed(), stats.critical_projects
            );
            println!();
        }
    }

    Ok(())
}
