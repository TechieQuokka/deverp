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
    Create,
    /// List resources
    List,
    /// Show resource details
    Show,
    /// Update a resource
    Update,
    /// Delete a resource
    Delete,
    /// Link resource to project
    Link,
    /// Unlink resource from project
    Unlink,
    /// Show resource usage statistics
    Usage,
}

/// Timeline management subcommands
#[derive(Subcommand, Clone)]
pub enum TimelineCommand {
    /// Create a new timeline
    Create,
    /// List timelines
    List,
    /// Show timeline details
    Show,
    /// Update a timeline
    Update,
    /// Delete a timeline
    Delete,
    /// Add milestone
    AddMilestone,
    /// Update milestone
    UpdateMilestone,
    /// Complete milestone
    CompleteMilestone,
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
    /// Show current configuration
    Show,
    /// Set configuration value
    Set,
    /// Reset to default configuration
    Reset,
    /// Test database connection
    TestDb,
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
