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
            Commands::Project(cmd) => project::handle(cmd.clone(), self.format).await,
            Commands::Task(cmd) => task::handle(cmd.clone(), self.format).await,
            Commands::Resource(cmd) => resource::handle(cmd.clone(), self.format).await,
            Commands::Timeline(cmd) => timeline::handle(cmd.clone(), self.format).await,
            Commands::Report(cmd) => report::handle(cmd.clone(), self.format).await,
            Commands::Config(cmd) => config::handle(cmd.clone(), self.format).await,
        }
    }
}

// Re-export commonly used types
pub use commands::{OutputFormat, PaginationOptions};
pub use output::{OutputManager, PaginatedOutput};
