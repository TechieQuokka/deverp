// DevERP - IT Development Project Management ERP System
// Library root module

pub mod cli;
pub mod config;
pub mod domain;
pub mod infrastructure;
pub mod utils;

// Re-export commonly used types
pub use utils::error::DevErpError;
pub type Result<T> = std::result::Result<T, DevErpError>;
