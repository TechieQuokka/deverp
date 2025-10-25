// CLI Interface Layer

pub mod commands;
pub mod output;

// Command handlers
pub mod config;
pub mod project;
pub mod report;
pub mod resource;
pub mod task;
pub mod timeline;

use crate::Result;
pub use commands::Cli;
use commands::Commands;

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
