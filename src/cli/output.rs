// Output formatting utilities for CLI

use crate::utils::formatter;
use serde::Serialize;
use colored::Colorize;

/// Print paginated results with metadata
pub struct PaginatedOutput<T> {
    pub items: Vec<T>,
    pub page: u32,
    pub per_page: u32,
    pub total: Option<usize>,
}

impl<T> PaginatedOutput<T> {
    pub fn new(items: Vec<T>, page: u32, per_page: u32) -> Self {
        Self {
            items,
            page,
            per_page,
            total: None,
        }
    }

    pub fn with_total(mut self, total: usize) -> Self {
        self.total = Some(total);
        self
    }

    pub fn print_metadata(&self) {
        let start = ((self.page - 1) * self.per_page) + 1;
        let end = start + self.items.len() as u32 - 1;

        if let Some(total) = self.total {
            println!(
                "\n{} Showing {}-{} of {} items (Page {})",
                "ℹ".blue().bold(),
                start,
                end,
                total,
                self.page
            );

            let total_pages = (total as f64 / self.per_page as f64).ceil() as u32;
            if self.page < total_pages {
                println!(
                    "{} Use --page {} to see more",
                    "→".cyan(),
                    self.page + 1
                );
            }
        } else {
            println!(
                "\n{} Showing {} items (Page {})",
                "ℹ".blue().bold(),
                self.items.len(),
                self.page
            );
        }
    }
}

/// Output manager that handles format selection
pub struct OutputManager {
    format: formatter::OutputFormat,
}

impl OutputManager {
    pub fn new(format: formatter::OutputFormat) -> Self {
        Self { format }
    }

    /// Output a single item
    pub fn output_single<T: Serialize>(&self, item: &T) -> crate::Result<()> {
        match self.format {
            formatter::OutputFormat::Json => formatter::output_json(item),
            formatter::OutputFormat::Table | formatter::OutputFormat::Plain => {
                // For single items in table/plain format, use key-value display
                formatter::output_json(item) // Fallback to JSON for complex structures
            }
        }
    }

    /// Output a list of items
    pub fn output_list<T: Serialize>(&self, items: &[T]) -> crate::Result<()> {
        match self.format {
            formatter::OutputFormat::Json => formatter::output_json(items),
            formatter::OutputFormat::Table | formatter::OutputFormat::Plain => {
                formatter::output_json(items) // Fallback to JSON for now
            }
        }
    }

    /// Output a paginated list
    pub fn output_paginated<T: Serialize>(
        &self,
        output: &PaginatedOutput<T>,
    ) -> crate::Result<()> {
        self.output_list(&output.items)?;

        if self.format != formatter::OutputFormat::Json {
            output.print_metadata();
        }

        Ok(())
    }
}

/// Print a divider line
pub fn divider() {
    println!("{}", "─".repeat(80).dimmed());
}

/// Print a section title
pub fn section_title(title: &str) {
    println!("\n{}", title.bold().cyan());
    divider();
}

/// Print a summary line (key: value)
pub fn summary_line(key: &str, value: &str) {
    println!("  {}: {}", key.bold(), value);
}

/// Print an indented message
pub fn indent(message: &str, level: usize) {
    let spacing = "  ".repeat(level);
    println!("{}{}", spacing, message);
}

/// Print a list item
pub fn list_item(text: &str) {
    println!("  {} {}", "•".cyan(), text);
}

/// Print a numbered item
pub fn numbered_item(number: usize, text: &str) {
    println!("  {}. {}", number.to_string().bold(), text);
}

/// Print an empty state message
pub fn empty_state(entity: &str) {
    println!("\n{} No {} found.", "ℹ".blue().bold(), entity);
    println!("  Use the 'create' command to add one.\n");
}

/// Confirm action with user
pub fn confirm(prompt: &str) -> bool {
    use std::io::{self, Write};

    print!("{} {} [y/N]: ", "?".yellow().bold(), prompt);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    matches!(input.trim().to_lowercase().as_str(), "y" | "yes")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_paginated_output_metadata() {
        let items = vec![1, 2, 3, 4, 5];
        let output = PaginatedOutput::new(items, 1, 10).with_total(25);

        assert_eq!(output.total, Some(25));
        assert_eq!(output.page, 1);
        assert_eq!(output.per_page, 10);
    }

    #[test]
    fn test_output_manager_creation() {
        let manager = OutputManager::new(formatter::OutputFormat::Table);
        assert_eq!(manager.format, formatter::OutputFormat::Table);
    }
}
