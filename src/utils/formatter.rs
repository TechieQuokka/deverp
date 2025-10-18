// Output formatting utilities

use colored::Colorize;

pub fn success(message: &str) {
    println!("{} {}", "✓".green().bold(), message);
}

pub fn error(message: &str) {
    eprintln!("{} {}", "✗".red().bold(), message);
}

pub fn info(message: &str) {
    println!("{} {}", "ℹ".blue().bold(), message);
}

pub fn warning(message: &str) {
    println!("{} {}", "⚠".yellow().bold(), message);
}
