// Output formatting utilities

use colored::Colorize;
use serde::Serialize;

/// Output format options
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputFormat {
    Table,
    Json,
    Plain,
}

/// Display a success message
pub fn success(message: &str) {
    println!("{} {}", "✓".green().bold(), message);
}

/// Display an error message
pub fn error(message: &str) {
    eprintln!("{} {}", "✗".red().bold(), message);
}

/// Display an info message
pub fn info(message: &str) {
    println!("{} {}", "ℹ".blue().bold(), message);
}

/// Display a warning message
pub fn warning(message: &str) {
    println!("{} {}", "⚠".yellow().bold(), message);
}

/// Output data in JSON format
pub fn output_json<T: Serialize>(data: &T) -> crate::Result<()> {
    let json = serde_json::to_string_pretty(data)
        .map_err(|e| crate::utils::error::DevErpError::Internal(format!("JSON serialization error: {}", e)))?;
    println!("{}", json);
    Ok(())
}

/// Print a table header
pub fn table_header(columns: &[&str]) {
    let header = columns
        .iter()
        .map(|col| col.bold().cyan().to_string())
        .collect::<Vec<_>>()
        .join(" | ");
    println!("{}", header);

    let separator = columns
        .iter()
        .map(|col| "-".repeat(col.len()))
        .collect::<Vec<_>>()
        .join("-+-");
    println!("{}", separator);
}

/// Print a table row
pub fn table_row(values: &[String]) {
    println!("{}", values.join(" | "));
}

/// Print a key-value pair
pub fn key_value(key: &str, value: &str) {
    println!("{}: {}", key.bold(), value);
}

/// Print a section header
pub fn section_header(title: &str) {
    println!("\n{}", title.bold().underline());
}
