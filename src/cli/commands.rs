// CLI Command Definitions

use clap::{Parser, Subcommand, ValueEnum};

/// DevERP CLI Application
#[derive(Parser)]
#[command(name = "deverp")]
#[command(about = "DevERP - IT Development Project Management ERP System", long_about = None)]
#[command(version)]
pub struct Cli {
    /// Output format
    #[arg(short, long, value_enum, global = true, default_value = "table")]
    pub format: OutputFormat,

    /// Verbose output
    #[arg(short, long, global = true)]
    pub verbose: bool,

    #[command(subcommand)]
    pub command: Commands,
}

/// Output format options
#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum OutputFormat {
    /// Table format (default)
    Table,
    /// JSON format
    Json,
    /// Plain text format
    Plain,
}

impl From<OutputFormat> for crate::utils::formatter::OutputFormat {
    fn from(format: OutputFormat) -> Self {
        match format {
            OutputFormat::Table => crate::utils::formatter::OutputFormat::Table,
            OutputFormat::Json => crate::utils::formatter::OutputFormat::Json,
            OutputFormat::Plain => crate::utils::formatter::OutputFormat::Plain,
        }
    }
}

/// Main command categories
#[derive(Subcommand)]
pub enum Commands {
    /// Project management operations
    #[command(subcommand)]
    Project(ProjectCommand),

    /// Task management operations
    #[command(subcommand)]
    Task(TaskCommand),

    /// Resource tracking and allocation
    #[command(subcommand)]
    Resource(ResourceCommand),

    /// Timeline and scheduling management
    #[command(subcommand)]
    Timeline(TimelineCommand),

    /// Report generation and analytics
    #[command(subcommand)]
    Report(ReportCommand),

    /// System configuration
    #[command(subcommand)]
    Config(ConfigCommand),
}

/// Project management subcommands
#[derive(Subcommand, Clone)]
pub enum ProjectCommand {
    /// Create a new project
    Create(CreateProjectArgs),
    /// List all projects
    List(ListProjectArgs),
    /// Show project details
    Show(ShowProjectArgs),
    /// Update a project
    Update(UpdateProjectArgs),
    /// Delete a project
    Delete(DeleteProjectArgs),
    /// Archive a project
    Archive(ArchiveProjectArgs),
}

/// Arguments for creating a new project
#[derive(Parser, Clone, Debug)]
pub struct CreateProjectArgs {
    /// Project name
    #[arg(short, long)]
    pub name: String,

    /// Project description
    #[arg(short, long)]
    pub description: Option<String>,

    /// Short project code (e.g., PROJ-001)
    #[arg(short, long)]
    pub code: Option<String>,

    /// Project status
    #[arg(short, long)]
    pub status: Option<String>,

    /// Project priority
    #[arg(short = 'p', long)]
    pub priority: Option<String>,

    /// Start date (YYYY-MM-DD)
    #[arg(long)]
    pub start_date: Option<String>,

    /// End date (YYYY-MM-DD)
    #[arg(long)]
    pub end_date: Option<String>,

    /// Repository URL
    #[arg(long)]
    pub repository_url: Option<String>,

    /// Repository branch
    #[arg(long, default_value = "main")]
    pub repository_branch: Option<String>,

    /// Tags (comma-separated)
    #[arg(long)]
    pub tags: Option<String>,
}

/// Arguments for listing projects
#[derive(Parser, Clone, Debug)]
pub struct ListProjectArgs {
    /// Filter by status
    #[arg(short, long)]
    pub status: Option<String>,

    /// Filter by priority
    #[arg(short, long)]
    pub priority: Option<String>,

    /// Search by name or description
    #[arg(short = 'q', long)]
    pub search: Option<String>,

    /// Filter by tags (comma-separated)
    #[arg(long)]
    pub tags: Option<String>,

    /// Pagination options
    #[command(flatten)]
    pub pagination: PaginationOptions,
}

/// Arguments for showing project details
#[derive(Parser, Clone, Debug)]
pub struct ShowProjectArgs {
    /// Project ID or UUID
    pub identifier: String,
}

/// Arguments for updating a project
#[derive(Parser, Clone, Debug)]
pub struct UpdateProjectArgs {
    /// Project ID or UUID
    pub identifier: String,

    /// New project name
    #[arg(short, long)]
    pub name: Option<String>,

    /// New project description
    #[arg(short, long)]
    pub description: Option<String>,

    /// New project code
    #[arg(short, long)]
    pub code: Option<String>,

    /// New project status
    #[arg(short, long)]
    pub status: Option<String>,

    /// New project priority
    #[arg(short = 'p', long)]
    pub priority: Option<String>,

    /// New start date (YYYY-MM-DD)
    #[arg(long)]
    pub start_date: Option<String>,

    /// New end date (YYYY-MM-DD)
    #[arg(long)]
    pub end_date: Option<String>,

    /// Actual start date (YYYY-MM-DD)
    #[arg(long)]
    pub actual_start_date: Option<String>,

    /// Actual end date (YYYY-MM-DD)
    #[arg(long)]
    pub actual_end_date: Option<String>,

    /// Progress percentage (0-100)
    #[arg(long)]
    pub progress: Option<i32>,

    /// Repository URL
    #[arg(long)]
    pub repository_url: Option<String>,

    /// Repository branch
    #[arg(long)]
    pub repository_branch: Option<String>,

    /// Tags (comma-separated)
    #[arg(long)]
    pub tags: Option<String>,
}

/// Arguments for deleting a project
#[derive(Parser, Clone, Debug)]
pub struct DeleteProjectArgs {
    /// Project ID or UUID
    pub identifier: String,

    /// Confirm deletion without prompt
    #[arg(long)]
    pub confirm: bool,
}

/// Arguments for archiving a project
#[derive(Parser, Clone, Debug)]
pub struct ArchiveProjectArgs {
    /// Project ID or UUID
    pub identifier: String,
}

/// Task management subcommands
#[derive(Subcommand, Clone)]
pub enum TaskCommand {
    /// Create a new task
    Create(CreateTaskArgs),
    /// List tasks
    List(ListTaskArgs),
    /// Show task details
    Show(ShowTaskArgs),
    /// Update a task
    Update(UpdateTaskArgs),
    /// Delete a task
    Delete(DeleteTaskArgs),
    /// Add task dependency
    AddDependency(AddDependencyArgs),
    /// Remove task dependency
    RemoveDependency(RemoveDependencyArgs),
    /// Add task comment
    AddComment(AddCommentArgs),
}

/// Arguments for creating a new task
#[derive(Parser, Clone, Debug)]
pub struct CreateTaskArgs {
    /// Project ID
    #[arg(long)]
    pub project_id: i64,

    /// Task title
    #[arg(short, long)]
    pub title: String,

    /// Task description
    #[arg(short, long)]
    pub description: Option<String>,

    /// Parent task ID
    #[arg(long)]
    pub parent_task_id: Option<i64>,

    /// Task number (e.g., TASK-001)
    #[arg(long)]
    pub task_number: Option<String>,

    /// Task status
    #[arg(short, long)]
    pub status: Option<String>,

    /// Task priority
    #[arg(short, long)]
    pub priority: Option<String>,

    /// Assigned to
    #[arg(long)]
    pub assigned_to: Option<String>,

    /// Estimated hours
    #[arg(long)]
    pub estimated_hours: Option<f64>,

    /// Due date (YYYY-MM-DD HH:MM:SS or YYYY-MM-DD)
    #[arg(long)]
    pub due_date: Option<String>,

    /// Task type
    #[arg(long)]
    pub task_type: Option<String>,

    /// Tags (comma-separated)
    #[arg(long)]
    pub tags: Option<String>,
}

/// Arguments for listing tasks
#[derive(Parser, Clone, Debug)]
pub struct ListTaskArgs {
    /// Filter by project ID
    #[arg(long)]
    pub project_id: Option<i64>,

    /// Filter by status
    #[arg(short, long)]
    pub status: Option<String>,

    /// Filter by priority
    #[arg(short, long)]
    pub priority: Option<String>,

    /// Filter by task type
    #[arg(long)]
    pub task_type: Option<String>,

    /// Filter by assigned to
    #[arg(long)]
    pub assigned_to: Option<String>,

    /// Filter by parent task ID
    #[arg(long)]
    pub parent_task_id: Option<i64>,

    /// Pagination options
    #[command(flatten)]
    pub pagination: PaginationOptions,
}

/// Arguments for showing task details
#[derive(Parser, Clone, Debug)]
pub struct ShowTaskArgs {
    /// Task ID or UUID
    pub identifier: String,
}

/// Arguments for updating a task
#[derive(Parser, Clone, Debug)]
pub struct UpdateTaskArgs {
    /// Task ID or UUID
    pub identifier: String,

    /// New task title
    #[arg(short, long)]
    pub title: Option<String>,

    /// New task description
    #[arg(short, long)]
    pub description: Option<String>,

    /// New task status
    #[arg(short, long)]
    pub status: Option<String>,

    /// New task priority
    #[arg(short, long)]
    pub priority: Option<String>,

    /// New assigned to
    #[arg(long)]
    pub assigned_to: Option<String>,

    /// New estimated hours
    #[arg(long)]
    pub estimated_hours: Option<f64>,

    /// New actual hours
    #[arg(long)]
    pub actual_hours: Option<f64>,

    /// New due date (YYYY-MM-DD HH:MM:SS or YYYY-MM-DD)
    #[arg(long)]
    pub due_date: Option<String>,

    /// New task type
    #[arg(long)]
    pub task_type: Option<String>,

    /// New tags (comma-separated)
    #[arg(long)]
    pub tags: Option<String>,
}

/// Arguments for deleting a task
#[derive(Parser, Clone, Debug)]
pub struct DeleteTaskArgs {
    /// Task ID or UUID
    pub identifier: String,

    /// Confirm deletion without prompt
    #[arg(long)]
    pub confirm: bool,
}

/// Arguments for adding a task dependency
#[derive(Parser, Clone, Debug)]
pub struct AddDependencyArgs {
    /// Task ID
    #[arg(long)]
    pub task_id: i64,

    /// Depends on task ID
    #[arg(long)]
    pub depends_on_task_id: i64,

    /// Dependency type (finish_to_start, start_to_start, finish_to_finish, start_to_finish)
    #[arg(long, default_value = "finish_to_start")]
    pub dependency_type: Option<String>,
}

/// Arguments for removing a task dependency
#[derive(Parser, Clone, Debug)]
pub struct RemoveDependencyArgs {
    /// Task ID
    #[arg(long)]
    pub task_id: i64,

    /// Depends on task ID
    #[arg(long)]
    pub depends_on_task_id: i64,
}

/// Arguments for adding a task comment
#[derive(Parser, Clone, Debug)]
pub struct AddCommentArgs {
    /// Task ID
    #[arg(long)]
    pub task_id: i64,

    /// Comment text
    #[arg(short, long)]
    pub comment: String,

    /// Comment author
    #[arg(long)]
    pub author: Option<String>,
}

/// Resource management subcommands
#[derive(Subcommand, Clone)]
pub enum ResourceCommand {
    /// Create a new resource
    Create(CreateResourceArgs),
    /// List resources
    List(ListResourceArgs),
    /// Show resource details
    Show(ShowResourceArgs),
    /// Update a resource
    Update(UpdateResourceArgs),
    /// Delete a resource
    Delete(DeleteResourceArgs),
    /// Link resource to project
    Link(LinkResourceArgs),
    /// Unlink resource from project
    Unlink(UnlinkResourceArgs),
    /// Show resource usage statistics
    Usage(UsageResourceArgs),
}

/// Arguments for creating a new resource
#[derive(Parser, Clone, Debug)]
pub struct CreateResourceArgs {
    /// Resource name
    #[arg(short, long)]
    pub name: String,

    /// Resource description
    #[arg(short, long)]
    pub description: Option<String>,

    /// Resource type (library, api, tool, service, documentation, other)
    #[arg(short = 't', long)]
    pub resource_type: String,

    /// Version
    #[arg(short, long)]
    pub version: Option<String>,

    /// Resource URL
    #[arg(short, long)]
    pub url: Option<String>,

    /// Documentation URL
    #[arg(long)]
    pub documentation_url: Option<String>,

    /// License
    #[arg(short, long)]
    pub license: Option<String>,

    /// Status (active, deprecated, archived)
    #[arg(short, long)]
    pub status: Option<String>,

    /// Tags (comma-separated)
    #[arg(long)]
    pub tags: Option<String>,
}

/// Arguments for listing resources
#[derive(Parser, Clone, Debug)]
pub struct ListResourceArgs {
    /// Filter by resource type
    #[arg(short = 't', long)]
    pub resource_type: Option<String>,

    /// Filter by status
    #[arg(short, long)]
    pub status: Option<String>,

    /// Search by name
    #[arg(short = 'q', long)]
    pub search: Option<String>,

    /// Filter by tags (comma-separated)
    #[arg(long)]
    pub tags: Option<String>,

    /// Pagination options
    #[command(flatten)]
    pub pagination: PaginationOptions,
}

/// Arguments for showing resource details
#[derive(Parser, Clone, Debug)]
pub struct ShowResourceArgs {
    /// Resource ID or UUID
    pub identifier: String,
}

/// Arguments for updating a resource
#[derive(Parser, Clone, Debug)]
pub struct UpdateResourceArgs {
    /// Resource ID or UUID
    pub identifier: String,

    /// New resource name
    #[arg(short, long)]
    pub name: Option<String>,

    /// New resource description
    #[arg(short, long)]
    pub description: Option<String>,

    /// New resource type
    #[arg(short = 't', long)]
    pub resource_type: Option<String>,

    /// New version
    #[arg(short, long)]
    pub version: Option<String>,

    /// New resource URL
    #[arg(short, long)]
    pub url: Option<String>,

    /// New documentation URL
    #[arg(long)]
    pub documentation_url: Option<String>,

    /// New license
    #[arg(short, long)]
    pub license: Option<String>,

    /// New status
    #[arg(short, long)]
    pub status: Option<String>,

    /// New tags (comma-separated)
    #[arg(long)]
    pub tags: Option<String>,
}

/// Arguments for deleting a resource
#[derive(Parser, Clone, Debug)]
pub struct DeleteResourceArgs {
    /// Resource ID or UUID
    pub identifier: String,

    /// Confirm deletion without prompt
    #[arg(long)]
    pub confirm: bool,
}

/// Arguments for linking a resource to a project
#[derive(Parser, Clone, Debug)]
pub struct LinkResourceArgs {
    /// Project ID
    #[arg(long)]
    pub project_id: i64,

    /// Resource ID
    #[arg(long)]
    pub resource_id: i64,

    /// Usage notes
    #[arg(long)]
    pub usage_notes: Option<String>,

    /// Version used in this project
    #[arg(long)]
    pub version_used: Option<String>,

    /// Mark as critical for this project
    #[arg(long)]
    pub is_critical: bool,
}

/// Arguments for unlinking a resource from a project
#[derive(Parser, Clone, Debug)]
pub struct UnlinkResourceArgs {
    /// Project ID
    #[arg(long)]
    pub project_id: i64,

    /// Resource ID
    #[arg(long)]
    pub resource_id: i64,
}

/// Arguments for showing resource usage
#[derive(Parser, Clone, Debug)]
pub struct UsageResourceArgs {
    /// Resource ID (optional, if not provided shows all resources)
    pub resource_id: Option<i64>,
}

/// Timeline management subcommands
#[derive(Subcommand, Clone)]
pub enum TimelineCommand {
    /// Create a new timeline
    Create(CreateTimelineArgs),
    /// List timelines
    List(ListTimelineArgs),
    /// Show timeline details
    Show(ShowTimelineArgs),
    /// Update a timeline
    Update(UpdateTimelineArgs),
    /// Delete a timeline
    Delete(DeleteTimelineArgs),
    /// Add milestone
    AddMilestone(AddMilestoneArgs),
    /// Update milestone
    UpdateMilestone(UpdateMilestoneArgs),
    /// Complete milestone
    CompleteMilestone(CompleteMilestoneArgs),
}

/// Arguments for creating a new timeline
#[derive(Parser, Clone, Debug)]
pub struct CreateTimelineArgs {
    /// Project ID
    #[arg(long)]
    pub project_id: i64,

    /// Timeline name
    #[arg(short, long)]
    pub name: String,

    /// Timeline description
    #[arg(short, long)]
    pub description: Option<String>,

    /// Timeline type (project, sprint, release, phase)
    #[arg(short = 't', long, default_value = "project")]
    pub timeline_type: String,

    /// Start date (YYYY-MM-DD)
    #[arg(long)]
    pub start_date: String,

    /// End date (YYYY-MM-DD)
    #[arg(long)]
    pub end_date: String,

    /// Status (planned, active, completed, cancelled)
    #[arg(short, long)]
    pub status: Option<String>,
}

/// Arguments for listing timelines
#[derive(Parser, Clone, Debug)]
pub struct ListTimelineArgs {
    /// Filter by project ID
    #[arg(long)]
    pub project_id: Option<i64>,

    /// Filter by timeline type
    #[arg(short = 't', long)]
    pub timeline_type: Option<String>,

    /// Filter by status
    #[arg(short, long)]
    pub status: Option<String>,

    /// Pagination options
    #[command(flatten)]
    pub pagination: PaginationOptions,
}

/// Arguments for showing timeline details
#[derive(Parser, Clone, Debug)]
pub struct ShowTimelineArgs {
    /// Timeline ID
    pub id: i64,
}

/// Arguments for updating a timeline
#[derive(Parser, Clone, Debug)]
pub struct UpdateTimelineArgs {
    /// Timeline ID
    pub id: i64,

    /// New timeline name
    #[arg(short, long)]
    pub name: Option<String>,

    /// New timeline description
    #[arg(short, long)]
    pub description: Option<String>,

    /// New timeline type
    #[arg(short = 't', long)]
    pub timeline_type: Option<String>,

    /// New start date (YYYY-MM-DD)
    #[arg(long)]
    pub start_date: Option<String>,

    /// New end date (YYYY-MM-DD)
    #[arg(long)]
    pub end_date: Option<String>,

    /// New status
    #[arg(short, long)]
    pub status: Option<String>,
}

/// Arguments for deleting a timeline
#[derive(Parser, Clone, Debug)]
pub struct DeleteTimelineArgs {
    /// Timeline ID
    pub id: i64,

    /// Confirm deletion without prompt
    #[arg(long)]
    pub confirm: bool,
}

/// Arguments for adding a milestone
#[derive(Parser, Clone, Debug)]
pub struct AddMilestoneArgs {
    /// Timeline ID
    #[arg(long)]
    pub timeline_id: i64,

    /// Project ID
    #[arg(long)]
    pub project_id: i64,

    /// Milestone name
    #[arg(short, long)]
    pub name: String,

    /// Milestone description
    #[arg(short, long)]
    pub description: Option<String>,

    /// Target date (YYYY-MM-DD)
    #[arg(long)]
    pub target_date: String,

    /// Status (pending, in_progress, completed, missed, cancelled)
    #[arg(short, long)]
    pub status: Option<String>,
}

/// Arguments for updating a milestone
#[derive(Parser, Clone, Debug)]
pub struct UpdateMilestoneArgs {
    /// Milestone ID
    pub id: i64,

    /// New milestone name
    #[arg(short, long)]
    pub name: Option<String>,

    /// New milestone description
    #[arg(short, long)]
    pub description: Option<String>,

    /// New target date (YYYY-MM-DD)
    #[arg(long)]
    pub target_date: Option<String>,

    /// New actual date (YYYY-MM-DD)
    #[arg(long)]
    pub actual_date: Option<String>,

    /// New status
    #[arg(short, long)]
    pub status: Option<String>,

    /// New completion percentage (0-100)
    #[arg(long)]
    pub completion_percentage: Option<i32>,
}

/// Arguments for completing a milestone
#[derive(Parser, Clone, Debug)]
pub struct CompleteMilestoneArgs {
    /// Milestone ID
    pub id: i64,

    /// Actual completion date (YYYY-MM-DD), defaults to today
    #[arg(long)]
    pub actual_date: Option<String>,
}

/// Report generation subcommands
#[derive(Subcommand, Clone)]
pub enum ReportCommand {
    /// Overall status report
    Status,
    /// Project summary report
    ProjectSummary,
    /// Task analytics report
    TaskAnalytics,
    /// Resource usage report
    ResourceUsage,
    /// Timeline progress report
    TimelineProgress,
}

/// Configuration subcommands
#[derive(Subcommand, Clone)]
pub enum ConfigCommand {
    /// Show all configurations or a specific configuration
    Show {
        /// Configuration key to show (optional, shows all if not provided)
        #[arg(short, long)]
        key: Option<String>,
    },
    /// Set a configuration value
    Set {
        /// Configuration key
        key: String,
        /// Configuration value
        value: String,
        /// Optional description
        #[arg(short, long)]
        description: Option<String>,
    },
    /// Reset all configurations to default values
    Reset {
        /// Confirm reset (required to prevent accidental resets)
        #[arg(short, long)]
        confirm: bool,
    },
    /// Test database connection
    TestDb {
        /// Show detailed database information
        #[arg(short, long)]
        verbose: bool,
    },
}

/// Common pagination options
#[derive(Parser, Debug, Clone)]
pub struct PaginationOptions {
    /// Page number (starting from 1)
    #[arg(long, default_value = "1")]
    pub page: u32,

    /// Number of items per page
    #[arg(long, default_value = "50")]
    pub per_page: u32,
}

impl PaginationOptions {
    /// Calculate offset for database queries
    pub fn offset(&self) -> i64 {
        ((self.page.saturating_sub(1)) * self.per_page) as i64
    }

    /// Get limit for database queries
    pub fn limit(&self) -> i64 {
        self.per_page as i64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pagination_offset() {
        let opts = PaginationOptions { page: 1, per_page: 10 };
        assert_eq!(opts.offset(), 0);

        let opts = PaginationOptions { page: 2, per_page: 10 };
        assert_eq!(opts.offset(), 10);

        let opts = PaginationOptions { page: 5, per_page: 20 };
        assert_eq!(opts.offset(), 80);
    }

    #[test]
    fn test_pagination_limit() {
        let opts = PaginationOptions { page: 1, per_page: 10 };
        assert_eq!(opts.limit(), 10);

        let opts = PaginationOptions { page: 2, per_page: 25 };
        assert_eq!(opts.limit(), 25);
    }

    #[test]
    fn test_output_format_conversion() {
        let table_format: crate::utils::formatter::OutputFormat = OutputFormat::Table.into();
        assert_eq!(table_format, crate::utils::formatter::OutputFormat::Table);

        let json_format: crate::utils::formatter::OutputFormat = OutputFormat::Json.into();
        assert_eq!(json_format, crate::utils::formatter::OutputFormat::Json);
    }
}
