use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Configuration {
    pub id: i64,
    pub config_key: String,
    pub config_value: String,
    pub description: Option<String>,
    #[sqlx(rename = "data_type")]
    pub data_type: ConfigDataType,
    pub is_encrypted: bool,
    pub is_required: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq, Eq)]
#[sqlx(type_name = "VARCHAR", rename_all = "lowercase")]
pub enum ConfigDataType {
    #[serde(rename = "string")]
    String,
    #[serde(rename = "integer")]
    Integer,
    #[serde(rename = "boolean")]
    Boolean,
    #[serde(rename = "json")]
    Json,
}

impl std::fmt::Display for ConfigDataType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfigDataType::String => write!(f, "string"),
            ConfigDataType::Integer => write!(f, "integer"),
            ConfigDataType::Boolean => write!(f, "boolean"),
            ConfigDataType::Json => write!(f, "json"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateConfiguration {
    pub config_key: String,
    pub config_value: String,
    pub description: Option<String>,
    pub data_type: ConfigDataType,
    pub is_encrypted: bool,
    pub is_required: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateConfiguration {
    pub config_key: String,
    pub config_value: String,
    pub description: Option<String>,
}

impl Configuration {
    pub fn validate_value(&self) -> Result<(), String> {
        match self.data_type {
            ConfigDataType::Integer => {
                self.config_value
                    .parse::<i64>()
                    .map_err(|_| format!("Value '{}' is not a valid integer", self.config_value))?;
            }
            ConfigDataType::Boolean => {
                self.config_value
                    .parse::<bool>()
                    .map_err(|_| format!("Value '{}' is not a valid boolean", self.config_value))?;
            }
            ConfigDataType::Json => {
                serde_json::from_str::<serde_json::Value>(&self.config_value)
                    .map_err(|_| format!("Value '{}' is not valid JSON", self.config_value))?;
            }
            ConfigDataType::String => {
                // String type accepts any value
            }
        }
        Ok(())
    }

    pub fn get_as_string(&self) -> String {
        self.config_value.clone()
    }

    pub fn get_as_integer(&self) -> Result<i64, String> {
        self.config_value
            .parse::<i64>()
            .map_err(|_| format!("Cannot parse '{}' as integer", self.config_value))
    }

    pub fn get_as_boolean(&self) -> Result<bool, String> {
        self.config_value
            .parse::<bool>()
            .map_err(|_| format!("Cannot parse '{}' as boolean", self.config_value))
    }

    pub fn get_as_json(&self) -> Result<serde_json::Value, String> {
        serde_json::from_str(&self.config_value)
            .map_err(|_| format!("Cannot parse '{}' as JSON", self.config_value))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_value_string() {
        let config = Configuration {
            id: 1,
            config_key: "test_key".to_string(),
            config_value: "any value".to_string(),
            description: None,
            data_type: ConfigDataType::String,
            is_encrypted: false,
            is_required: false,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        assert!(config.validate_value().is_ok());
    }

    #[test]
    fn test_validate_value_integer_valid() {
        let config = Configuration {
            id: 1,
            config_key: "test_key".to_string(),
            config_value: "42".to_string(),
            description: None,
            data_type: ConfigDataType::Integer,
            is_encrypted: false,
            is_required: false,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        assert!(config.validate_value().is_ok());
    }

    #[test]
    fn test_validate_value_integer_invalid() {
        let config = Configuration {
            id: 1,
            config_key: "test_key".to_string(),
            config_value: "not a number".to_string(),
            description: None,
            data_type: ConfigDataType::Integer,
            is_encrypted: false,
            is_required: false,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        assert!(config.validate_value().is_err());
    }

    #[test]
    fn test_validate_value_boolean_valid() {
        let config = Configuration {
            id: 1,
            config_key: "test_key".to_string(),
            config_value: "true".to_string(),
            description: None,
            data_type: ConfigDataType::Boolean,
            is_encrypted: false,
            is_required: false,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        assert!(config.validate_value().is_ok());
    }

    #[test]
    fn test_validate_value_json_valid() {
        let config = Configuration {
            id: 1,
            config_key: "test_key".to_string(),
            config_value: r#"{"key": "value"}"#.to_string(),
            description: None,
            data_type: ConfigDataType::Json,
            is_encrypted: false,
            is_required: false,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        assert!(config.validate_value().is_ok());
    }

    #[test]
    fn test_get_as_methods() {
        let config = Configuration {
            id: 1,
            config_key: "test_key".to_string(),
            config_value: "42".to_string(),
            description: None,
            data_type: ConfigDataType::Integer,
            is_encrypted: false,
            is_required: false,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        assert_eq!(config.get_as_string(), "42");
        assert_eq!(config.get_as_integer().unwrap(), 42);
    }
}
