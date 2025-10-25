use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

/// Resource types for development resources
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq, Eq)]
#[sqlx(type_name = "VARCHAR", rename_all = "lowercase")]
pub enum ResourceType {
    Library,
    Api,
    Tool,
    Service,
    Documentation,
    Other,
}

impl std::fmt::Display for ResourceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ResourceType::Library => write!(f, "library"),
            ResourceType::Api => write!(f, "api"),
            ResourceType::Tool => write!(f, "tool"),
            ResourceType::Service => write!(f, "service"),
            ResourceType::Documentation => write!(f, "documentation"),
            ResourceType::Other => write!(f, "other"),
        }
    }
}

impl std::str::FromStr for ResourceType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "library" => Ok(ResourceType::Library),
            "api" => Ok(ResourceType::Api),
            "tool" => Ok(ResourceType::Tool),
            "service" => Ok(ResourceType::Service),
            "documentation" => Ok(ResourceType::Documentation),
            "other" => Ok(ResourceType::Other),
            _ => Err(format!("Invalid resource type: {}", s)),
        }
    }
}

/// Resource status
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq, Eq)]
#[sqlx(type_name = "VARCHAR", rename_all = "lowercase")]
pub enum ResourceStatus {
    Active,
    Deprecated,
    Archived,
}

impl std::fmt::Display for ResourceStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ResourceStatus::Active => write!(f, "active"),
            ResourceStatus::Deprecated => write!(f, "deprecated"),
            ResourceStatus::Archived => write!(f, "archived"),
        }
    }
}

impl std::str::FromStr for ResourceStatus {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "active" => Ok(ResourceStatus::Active),
            "deprecated" => Ok(ResourceStatus::Deprecated),
            "archived" => Ok(ResourceStatus::Archived),
            _ => Err(format!("Invalid resource status: {}", s)),
        }
    }
}

/// Resource entity representing development resources
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Resource {
    pub id: i64,
    pub uuid: Uuid,
    pub name: String,
    pub description: Option<String>,
    #[sqlx(rename = "resource_type")]
    pub resource_type: ResourceType,
    pub version: Option<String>,
    pub url: Option<String>,
    pub documentation_url: Option<String>,
    pub license: Option<String>,
    pub status: Option<ResourceStatus>,
    #[sqlx(default)]
    pub metadata: Option<serde_json::Value>,
    #[sqlx(default)]
    pub tags: Option<Vec<String>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

/// Input for creating a new resource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateResource {
    pub name: String,
    pub description: Option<String>,
    pub resource_type: ResourceType,
    pub version: Option<String>,
    pub url: Option<String>,
    pub documentation_url: Option<String>,
    pub license: Option<String>,
    pub status: Option<ResourceStatus>,
    pub metadata: Option<serde_json::Value>,
    pub tags: Option<Vec<String>>,
}

/// Input for updating a resource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateResource {
    pub id: i64,
    pub name: Option<String>,
    pub description: Option<String>,
    pub resource_type: Option<ResourceType>,
    pub version: Option<String>,
    pub url: Option<String>,
    pub documentation_url: Option<String>,
    pub license: Option<String>,
    pub status: Option<ResourceStatus>,
    pub metadata: Option<serde_json::Value>,
    pub tags: Option<Vec<String>>,
}

/// Filter options for querying resources
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ResourceFilter {
    pub resource_type: Option<ResourceType>,
    pub status: Option<ResourceStatus>,
    pub name_contains: Option<String>,
    pub tags: Option<Vec<String>>,
    pub offset: Option<i64>,
    pub limit: Option<i64>,
}

/// ProjectResource entity representing the link between projects and resources
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ProjectResource {
    pub project_id: i64,
    pub resource_id: i64,
    pub usage_notes: Option<String>,
    pub version_used: Option<String>,
    pub is_critical: Option<bool>,
    pub added_at: DateTime<Utc>,
    pub removed_at: Option<DateTime<Utc>>,
}

/// Input for linking a resource to a project
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinkResourceToProject {
    pub project_id: i64,
    pub resource_id: i64,
    pub usage_notes: Option<String>,
    pub version_used: Option<String>,
    pub is_critical: Option<bool>,
}

/// Input for updating project-resource link
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateProjectResource {
    pub project_id: i64,
    pub resource_id: i64,
    pub usage_notes: Option<String>,
    pub version_used: Option<String>,
    pub is_critical: Option<bool>,
}

/// Resource usage statistics
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ResourceUsageStats {
    pub resource_id: i64,
    pub resource_name: String,
    pub resource_type: ResourceType,
    pub total_projects: i64,
    pub critical_projects: i64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resource_type_display() {
        assert_eq!(ResourceType::Library.to_string(), "library");
        assert_eq!(ResourceType::Api.to_string(), "api");
        assert_eq!(ResourceType::Tool.to_string(), "tool");
    }

    #[test]
    fn test_resource_type_from_str() {
        assert_eq!(
            "library".parse::<ResourceType>().unwrap(),
            ResourceType::Library
        );
        assert_eq!("API".parse::<ResourceType>().unwrap(), ResourceType::Api);
        assert!("invalid".parse::<ResourceType>().is_err());
    }

    #[test]
    fn test_resource_status_display() {
        assert_eq!(ResourceStatus::Active.to_string(), "active");
        assert_eq!(ResourceStatus::Deprecated.to_string(), "deprecated");
    }

    #[test]
    fn test_resource_status_from_str() {
        assert_eq!(
            "active".parse::<ResourceStatus>().unwrap(),
            ResourceStatus::Active
        );
        assert_eq!(
            "DEPRECATED".parse::<ResourceStatus>().unwrap(),
            ResourceStatus::Deprecated
        );
        assert!("invalid".parse::<ResourceStatus>().is_err());
    }
}
