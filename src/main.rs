use clap::Parser;
use deverp::cli::Cli;
use deverp::utils::logger;
use deverp::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logger
    logger::init()?;

    // Parse CLI arguments
    let cli = Cli::parse();

    // Handle commands
    cli.execute().await?;

    Ok(())
}
