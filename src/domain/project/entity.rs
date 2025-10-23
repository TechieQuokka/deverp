// Project entity and related types

use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Type};
use uuid::Uuid;

/// Project entity representing a development project
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Project {
    pub id: i64,
    pub uuid: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub code: Option<String>,

    #[sqlx(try_from = "String")]
    pub status: ProjectStatus,

    #[sqlx(try_from = "String")]
    pub priority: Priority,

    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
    pub actual_start_date: Option<NaiveDate>,
    pub actual_end_date: Option<NaiveDate>,

    pub progress_percentage: Option<i32>,

    pub repository_url: Option<String>,
    pub repository_branch: Option<String>,

    pub tags: Option<Vec<String>>,
    pub metadata: Option<sqlx::types::JsonValue>,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

/// Project status enum
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Type)]
#[serde(rename_all = "lowercase")]
#[sqlx(type_name = "varchar", rename_all = "snake_case")]
pub enum ProjectStatus {
    Planning,
    Active,
    OnHold,
    Completed,
    Archived,
    Cancelled,
}

impl ProjectStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            ProjectStatus::Planning => "planning",
            ProjectStatus::Active => "active",
            ProjectStatus::OnHold => "on_hold",
            ProjectStatus::Completed => "completed",
            ProjectStatus::Archived => "archived",
            ProjectStatus::Cancelled => "cancelled",
        }
    }
}

impl std::fmt::Display for ProjectStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl std::str::FromStr for ProjectStatus {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "planning" => Ok(ProjectStatus::Planning),
            "active" => Ok(ProjectStatus::Active),
            "on_hold" => Ok(ProjectStatus::OnHold),
            "completed" => Ok(ProjectStatus::Completed),
            "archived" => Ok(ProjectStatus::Archived),
            "cancelled" => Ok(ProjectStatus::Cancelled),
            _ => Err(format!("Invalid project status: {}", s)),
        }
    }
}

impl TryFrom<String> for ProjectStatus {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.parse()
    }
}

/// Priority enum
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Type)]
#[serde(rename_all = "lowercase")]
#[sqlx(type_name = "varchar", rename_all = "lowercase")]
pub enum Priority {
    Low,
    Medium,
    High,
    Critical,
}

impl Priority {
    pub fn as_str(&self) -> &'static str {
        match self {
            Priority::Low => "low",
            Priority::Medium => "medium",
            Priority::High => "high",
            Priority::Critical => "critical",
        }
    }
}

impl std::fmt::Display for Priority {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl std::str::FromStr for Priority {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "low" => Ok(Priority::Low),
            "medium" => Ok(Priority::Medium),
            "high" => Ok(Priority::High),
            "critical" => Ok(Priority::Critical),
            _ => Err(format!("Invalid priority: {}", s)),
        }
    }
}

impl TryFrom<String> for Priority {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.parse()
    }
}

/// Input for creating a new project
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateProject {
    pub name: String,
    pub description: Option<String>,
    pub code: Option<String>,
    pub status: Option<ProjectStatus>,
    pub priority: Option<Priority>,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
    pub repository_url: Option<String>,
    pub repository_branch: Option<String>,
    pub tags: Option<Vec<String>>,
    pub metadata: Option<sqlx::types::JsonValue>,
}

impl CreateProject {
    /// Validate the create project input
    pub fn validate(&self) -> Result<(), String> {
        // Name cannot be empty
        if self.name.trim().is_empty() {
            return Err("Project name cannot be empty".to_string());
        }

        // Name length validation (max 255 characters based on schema)
        if self.name.len() > 255 {
            return Err("Project name cannot exceed 255 characters".to_string());
        }

        // Code validation if provided (max 50 characters based on schema)
        if let Some(ref code) = self.code {
            if code.len() > 50 {
                return Err("Project code cannot exceed 50 characters".to_string());
            }
            if code.trim().is_empty() {
                return Err("Project code cannot be empty if provided".to_string());
            }
        }

        // Date validation
        if let (Some(start), Some(end)) = (&self.start_date, &self.end_date) {
            if end < start {
                return Err("End date must be after or equal to start date".to_string());
            }
        }

        Ok(())
    }
}

/// Input for updating an existing project
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateProject {
    pub id: i64,
    pub name: Option<String>,
    pub description: Option<String>,
    pub code: Option<String>,
    pub status: Option<ProjectStatus>,
    pub priority: Option<Priority>,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
    pub actual_start_date: Option<NaiveDate>,
    pub actual_end_date: Option<NaiveDate>,
    pub progress_percentage: Option<i32>,
    pub repository_url: Option<String>,
    pub repository_branch: Option<String>,
    pub tags: Option<Vec<String>>,
    pub metadata: Option<sqlx::types::JsonValue>,
}

impl UpdateProject {
    /// Validate the update project input
    pub fn validate(&self) -> Result<(), String> {
        // Name validation if provided
        if let Some(ref name) = self.name {
            if name.trim().is_empty() {
                return Err("Project name cannot be empty".to_string());
            }
            if name.len() > 255 {
                return Err("Project name cannot exceed 255 characters".to_string());
            }
        }

        // Code validation if provided
        if let Some(ref code) = self.code {
            if code.len() > 50 {
                return Err("Project code cannot exceed 50 characters".to_string());
            }
            if code.trim().is_empty() {
                return Err("Project code cannot be empty if provided".to_string());
            }
        }

        // Progress percentage validation
        if let Some(progress) = self.progress_percentage {
            if !(0..=100).contains(&progress) {
                return Err("Progress percentage must be between 0 and 100".to_string());
            }
        }

        Ok(())
    }
}

/// Filter options for listing projects
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ProjectFilter {
    pub status: Option<ProjectStatus>,
    pub priority: Option<Priority>,
    pub search: Option<String>,
    pub tags: Option<Vec<String>>,
    pub offset: Option<i64>,
    pub limit: Option<i64>,
}

impl ProjectFilter {
    /// Get the limit with a default value
    pub fn get_limit(&self) -> i64 {
        self.limit.unwrap_or(50).min(100) // Default 50, max 100
    }

    /// Get the offset with a default value
    pub fn get_offset(&self) -> i64 {
        self.offset.unwrap_or(0).max(0) // Default 0, minimum 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_project_status_from_str() {
        assert_eq!("planning".parse::<ProjectStatus>().unwrap(), ProjectStatus::Planning);
        assert_eq!("active".parse::<ProjectStatus>().unwrap(), ProjectStatus::Active);
        assert_eq!("on_hold".parse::<ProjectStatus>().unwrap(), ProjectStatus::OnHold);
        assert_eq!("completed".parse::<ProjectStatus>().unwrap(), ProjectStatus::Completed);
        assert_eq!("archived".parse::<ProjectStatus>().unwrap(), ProjectStatus::Archived);
        assert_eq!("cancelled".parse::<ProjectStatus>().unwrap(), ProjectStatus::Cancelled);
        assert!("invalid".parse::<ProjectStatus>().is_err());
    }

    #[test]
    fn test_priority_from_str() {
        assert_eq!("low".parse::<Priority>().unwrap(), Priority::Low);
        assert_eq!("medium".parse::<Priority>().unwrap(), Priority::Medium);
        assert_eq!("high".parse::<Priority>().unwrap(), Priority::High);
        assert_eq!("critical".parse::<Priority>().unwrap(), Priority::Critical);
        assert!("invalid".parse::<Priority>().is_err());
    }

    #[test]
    fn test_create_project_validation() {
        // Valid project
        let valid = CreateProject {
            name: "Test Project".to_string(),
            description: None,
            code: None,
            status: None,
            priority: None,
            start_date: None,
            end_date: None,
            repository_url: None,
            repository_branch: None,
            tags: None,
            metadata: None,
        };
        assert!(valid.validate().is_ok());

        // Empty name
        let empty_name = CreateProject {
            name: "".to_string(),
            ..valid.clone()
        };
        assert!(empty_name.validate().is_err());

        // Name too long
        let long_name = CreateProject {
            name: "a".repeat(256),
            ..valid.clone()
        };
        assert!(long_name.validate().is_err());

        // Invalid date range
        let invalid_dates = CreateProject {
            start_date: Some(NaiveDate::from_ymd_opt(2025, 12, 31).unwrap()),
            end_date: Some(NaiveDate::from_ymd_opt(2025, 1, 1).unwrap()),
            ..valid.clone()
        };
        assert!(invalid_dates.validate().is_err());
    }

    #[test]
    fn test_update_project_validation() {
        let valid = UpdateProject {
            id: 1,
            name: Some("Updated Project".to_string()),
            description: None,
            code: None,
            status: None,
            priority: None,
            start_date: None,
            end_date: None,
            actual_start_date: None,
            actual_end_date: None,
            progress_percentage: Some(50),
            repository_url: None,
            repository_branch: None,
            tags: None,
            metadata: None,
        };
        assert!(valid.validate().is_ok());

        // Invalid progress percentage
        let invalid_progress = UpdateProject {
            progress_percentage: Some(150),
            ..valid.clone()
        };
        assert!(invalid_progress.validate().is_err());
    }

    #[test]
    fn test_project_filter_defaults() {
        let filter = ProjectFilter::default();
        assert_eq!(filter.get_limit(), 50);
        assert_eq!(filter.get_offset(), 0);

        let custom_filter = ProjectFilter {
            limit: Some(200),
            offset: Some(-10),
            ..Default::default()
        };
        assert_eq!(custom_filter.get_limit(), 100); // Capped at 100
        assert_eq!(custom_filter.get_offset(), 0); // Minimum 0
    }
}
