// CLI Interface Layer

pub mod commands;
pub mod output;

// Command handlers
pub mod project;
pub mod task;
pub mod resource;
pub mod timeline;
pub mod report;
pub mod config;

pub use commands::Cli;
use commands::Commands;
use crate::Result;

impl Cli {
    /// Execute the CLI command
    pub async fn execute(&self) -> Result<()> {
        match &self.command {
            Commands::Project(cmd) => project::handle(cmd.clone()).await,
            Commands::Task(cmd) => task::handle(cmd.clone()).await,
            Commands::Resource(cmd) => resource::handle(cmd.clone()).await,
            Commands::Timeline(cmd) => timeline::handle(cmd.clone()).await,
            Commands::Report(cmd) => report::handle(cmd.clone()).await,
            Commands::Config(cmd) => config::handle(cmd.clone()).await,
        }
    }
}

// Re-export commonly used types
pub use commands::{OutputFormat, PaginationOptions};
pub use output::{OutputManager, PaginatedOutput};
