// Timeline and Milestone entities and related types

use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Type};

/// Timeline entity representing a project timeline or schedule
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Timeline {
    pub id: i64,
    pub project_id: i64,

    pub name: String,
    pub description: Option<String>,

    #[sqlx(try_from = "String")]
    pub timeline_type: TimelineType,

    pub start_date: NaiveDate,
    pub end_date: NaiveDate,

    #[sqlx(try_from = "String")]
    pub status: TimelineStatus,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

/// Timeline type enum
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Type)]
#[serde(rename_all = "lowercase")]
#[sqlx(type_name = "varchar")]
pub enum TimelineType {
    Project,
    Sprint,
    Release,
    Phase,
}

impl TimelineType {
    pub fn as_str(&self) -> &'static str {
        match self {
            TimelineType::Project => "project",
            TimelineType::Sprint => "sprint",
            TimelineType::Release => "release",
            TimelineType::Phase => "phase",
        }
    }
}

impl std::fmt::Display for TimelineType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl std::str::FromStr for TimelineType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "project" => Ok(TimelineType::Project),
            "sprint" => Ok(TimelineType::Sprint),
            "release" => Ok(TimelineType::Release),
            "phase" => Ok(TimelineType::Phase),
            _ => Err(format!("Invalid timeline type: {}", s)),
        }
    }
}

impl TryFrom<String> for TimelineType {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.parse()
    }
}

/// Timeline status enum
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Type)]
#[serde(rename_all = "lowercase")]
#[sqlx(type_name = "varchar")]
pub enum TimelineStatus {
    Planned,
    Active,
    Completed,
    Cancelled,
}

impl TimelineStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            TimelineStatus::Planned => "planned",
            TimelineStatus::Active => "active",
            TimelineStatus::Completed => "completed",
            TimelineStatus::Cancelled => "cancelled",
        }
    }
}

impl std::fmt::Display for TimelineStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl std::str::FromStr for TimelineStatus {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "planned" => Ok(TimelineStatus::Planned),
            "active" => Ok(TimelineStatus::Active),
            "completed" => Ok(TimelineStatus::Completed),
            "cancelled" => Ok(TimelineStatus::Cancelled),
            _ => Err(format!("Invalid timeline status: {}", s)),
        }
    }
}

impl TryFrom<String> for TimelineStatus {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.parse()
    }
}

/// Milestone entity representing a timeline milestone
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Milestone {
    pub id: i64,
    pub timeline_id: i64,
    pub project_id: i64,

    pub name: String,
    pub description: Option<String>,

    pub target_date: NaiveDate,
    pub actual_date: Option<NaiveDate>,

    #[sqlx(try_from = "String")]
    pub status: MilestoneStatus,

    pub completion_percentage: i32,

    pub metadata: Option<sqlx::types::JsonValue>,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

/// Milestone status enum
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Type)]
#[serde(rename_all = "lowercase")]
#[sqlx(type_name = "varchar")]
pub enum MilestoneStatus {
    Pending,
    InProgress,
    Completed,
    Missed,
    Cancelled,
}

impl MilestoneStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            MilestoneStatus::Pending => "pending",
            MilestoneStatus::InProgress => "in_progress",
            MilestoneStatus::Completed => "completed",
            MilestoneStatus::Missed => "missed",
            MilestoneStatus::Cancelled => "cancelled",
        }
    }
}

impl std::fmt::Display for MilestoneStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl std::str::FromStr for MilestoneStatus {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "pending" => Ok(MilestoneStatus::Pending),
            "in_progress" => Ok(MilestoneStatus::InProgress),
            "completed" => Ok(MilestoneStatus::Completed),
            "missed" => Ok(MilestoneStatus::Missed),
            "cancelled" => Ok(MilestoneStatus::Cancelled),
            _ => Err(format!("Invalid milestone status: {}", s)),
        }
    }
}

impl TryFrom<String> for MilestoneStatus {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.parse()
    }
}

/// Input for creating a new timeline
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTimeline {
    pub project_id: i64,
    pub name: String,
    pub description: Option<String>,
    pub timeline_type: Option<TimelineType>,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub status: Option<TimelineStatus>,
}

impl CreateTimeline {
    /// Validate the create timeline input
    pub fn validate(&self) -> Result<(), String> {
        // Name cannot be empty
        if self.name.trim().is_empty() {
            return Err("Timeline name cannot be empty".to_string());
        }

        // Name length validation (max 255 characters based on schema)
        if self.name.len() > 255 {
            return Err("Timeline name cannot exceed 255 characters".to_string());
        }

        // Date validation
        if self.end_date < self.start_date {
            return Err("End date must be after or equal to start date".to_string());
        }

        Ok(())
    }
}

/// Input for updating an existing timeline
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateTimeline {
    pub id: i64,
    pub name: Option<String>,
    pub description: Option<String>,
    pub timeline_type: Option<TimelineType>,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
    pub status: Option<TimelineStatus>,
}

impl UpdateTimeline {
    /// Validate the update timeline input
    pub fn validate(&self) -> Result<(), String> {
        // Name validation if provided
        if let Some(ref name) = self.name {
            if name.trim().is_empty() {
                return Err("Timeline name cannot be empty".to_string());
            }
            if name.len() > 255 {
                return Err("Timeline name cannot exceed 255 characters".to_string());
            }
        }

        Ok(())
    }
}

/// Input for creating a new milestone
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateMilestone {
    pub timeline_id: i64,
    pub project_id: i64,
    pub name: String,
    pub description: Option<String>,
    pub target_date: NaiveDate,
    pub status: Option<MilestoneStatus>,
    pub completion_percentage: Option<i32>,
    pub metadata: Option<sqlx::types::JsonValue>,
}

impl CreateMilestone {
    /// Validate the create milestone input
    pub fn validate(&self) -> Result<(), String> {
        // Name cannot be empty
        if self.name.trim().is_empty() {
            return Err("Milestone name cannot be empty".to_string());
        }

        // Name length validation (max 255 characters based on schema)
        if self.name.len() > 255 {
            return Err("Milestone name cannot exceed 255 characters".to_string());
        }

        // Completion percentage validation
        if let Some(completion) = self.completion_percentage {
            if !(0..=100).contains(&completion) {
                return Err("Completion percentage must be between 0 and 100".to_string());
            }
        }

        Ok(())
    }
}

/// Input for updating an existing milestone
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateMilestone {
    pub id: i64,
    pub name: Option<String>,
    pub description: Option<String>,
    pub target_date: Option<NaiveDate>,
    pub actual_date: Option<NaiveDate>,
    pub status: Option<MilestoneStatus>,
    pub completion_percentage: Option<i32>,
    pub metadata: Option<sqlx::types::JsonValue>,
}

impl UpdateMilestone {
    /// Validate the update milestone input
    pub fn validate(&self) -> Result<(), String> {
        // Name validation if provided
        if let Some(ref name) = self.name {
            if name.trim().is_empty() {
                return Err("Milestone name cannot be empty".to_string());
            }
            if name.len() > 255 {
                return Err("Milestone name cannot exceed 255 characters".to_string());
            }
        }

        // Completion percentage validation
        if let Some(completion) = self.completion_percentage {
            if !(0..=100).contains(&completion) {
                return Err("Completion percentage must be between 0 and 100".to_string());
            }
        }

        Ok(())
    }
}

/// Filter options for listing timelines
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TimelineFilter {
    pub project_id: Option<i64>,
    pub timeline_type: Option<TimelineType>,
    pub status: Option<TimelineStatus>,
    pub offset: Option<i64>,
    pub limit: Option<i64>,
}

impl TimelineFilter {
    /// Get the limit with a default value
    pub fn get_limit(&self) -> i64 {
        self.limit.unwrap_or(50).min(100) // Default 50, max 100
    }

    /// Get the offset with a default value
    pub fn get_offset(&self) -> i64 {
        self.offset.unwrap_or(0).max(0) // Default 0, minimum 0
    }
}

/// Filter options for listing milestones
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MilestoneFilter {
    pub timeline_id: Option<i64>,
    pub project_id: Option<i64>,
    pub status: Option<MilestoneStatus>,
    pub offset: Option<i64>,
    pub limit: Option<i64>,
}

impl MilestoneFilter {
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
    fn test_timeline_type_from_str() {
        assert_eq!("project".parse::<TimelineType>().unwrap(), TimelineType::Project);
        assert_eq!("sprint".parse::<TimelineType>().unwrap(), TimelineType::Sprint);
        assert_eq!("release".parse::<TimelineType>().unwrap(), TimelineType::Release);
        assert_eq!("phase".parse::<TimelineType>().unwrap(), TimelineType::Phase);
        assert!("invalid".parse::<TimelineType>().is_err());
    }

    #[test]
    fn test_timeline_status_from_str() {
        assert_eq!("planned".parse::<TimelineStatus>().unwrap(), TimelineStatus::Planned);
        assert_eq!("active".parse::<TimelineStatus>().unwrap(), TimelineStatus::Active);
        assert_eq!("completed".parse::<TimelineStatus>().unwrap(), TimelineStatus::Completed);
        assert_eq!("cancelled".parse::<TimelineStatus>().unwrap(), TimelineStatus::Cancelled);
        assert!("invalid".parse::<TimelineStatus>().is_err());
    }

    #[test]
    fn test_milestone_status_from_str() {
        assert_eq!("pending".parse::<MilestoneStatus>().unwrap(), MilestoneStatus::Pending);
        assert_eq!("in_progress".parse::<MilestoneStatus>().unwrap(), MilestoneStatus::InProgress);
        assert_eq!("completed".parse::<MilestoneStatus>().unwrap(), MilestoneStatus::Completed);
        assert_eq!("missed".parse::<MilestoneStatus>().unwrap(), MilestoneStatus::Missed);
        assert_eq!("cancelled".parse::<MilestoneStatus>().unwrap(), MilestoneStatus::Cancelled);
        assert!("invalid".parse::<MilestoneStatus>().is_err());
    }

    #[test]
    fn test_create_timeline_validation() {
        let valid = CreateTimeline {
            project_id: 1,
            name: "Sprint 1".to_string(),
            description: None,
            timeline_type: Some(TimelineType::Sprint),
            start_date: NaiveDate::from_ymd_opt(2025, 1, 1).unwrap(),
            end_date: NaiveDate::from_ymd_opt(2025, 1, 14).unwrap(),
            status: None,
        };
        assert!(valid.validate().is_ok());

        // Empty name
        let empty_name = CreateTimeline {
            name: "".to_string(),
            ..valid.clone()
        };
        assert!(empty_name.validate().is_err());

        // Invalid date range
        let invalid_dates = CreateTimeline {
            start_date: NaiveDate::from_ymd_opt(2025, 12, 31).unwrap(),
            end_date: NaiveDate::from_ymd_opt(2025, 1, 1).unwrap(),
            ..valid.clone()
        };
        assert!(invalid_dates.validate().is_err());
    }

    #[test]
    fn test_create_milestone_validation() {
        let valid = CreateMilestone {
            timeline_id: 1,
            project_id: 1,
            name: "Feature Complete".to_string(),
            description: None,
            target_date: NaiveDate::from_ymd_opt(2025, 1, 10).unwrap(),
            status: None,
            completion_percentage: Some(50),
            metadata: None,
        };
        assert!(valid.validate().is_ok());

        // Empty name
        let empty_name = CreateMilestone {
            name: "".to_string(),
            ..valid.clone()
        };
        assert!(empty_name.validate().is_err());

        // Invalid completion percentage
        let invalid_completion = CreateMilestone {
            completion_percentage: Some(150),
            ..valid.clone()
        };
        assert!(invalid_completion.validate().is_err());
    }

    #[test]
    fn test_filter_defaults() {
        let timeline_filter = TimelineFilter::default();
        assert_eq!(timeline_filter.get_limit(), 50);
        assert_eq!(timeline_filter.get_offset(), 0);

        let milestone_filter = MilestoneFilter::default();
        assert_eq!(milestone_filter.get_limit(), 50);
        assert_eq!(milestone_filter.get_offset(), 0);
    }
}
