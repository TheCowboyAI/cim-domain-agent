//! Metadata-related ECS components

use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

/// Component for agent metadata
#[derive(Component, Debug, Clone)]
pub struct AgentMetadata {
    /// Human-readable name
    pub name: String,
    /// Agent description
    pub description: String,
    /// Tags for categorization
    pub tags: HashSet<String>,
    /// Creation timestamp
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Last activity timestamp
    pub last_active: Option<chrono::DateTime<chrono::Utc>>,
    /// Custom metadata fields
    pub custom_fields: HashMap<String, serde_json::Value>,
}

impl AgentMetadata {
    /// Create new metadata with name and description
    pub fn new(name: String, description: String) -> Self {
        Self {
            name,
            description,
            tags: HashSet::new(),
            created_at: chrono::Utc::now(),
            last_active: None,
            custom_fields: HashMap::new(),
        }
    }

    /// Add a tag
    pub fn add_tag(&mut self, tag: String) {
        self.tags.insert(tag);
    }

    /// Remove a tag
    pub fn remove_tag(&mut self, tag: &str) -> bool {
        self.tags.remove(tag)
    }

    /// Update last active timestamp
    pub fn update_activity(&mut self) {
        self.last_active = Some(chrono::Utc::now());
    }
}

/// Component for agent configuration
#[derive(Component, Debug, Clone)]
pub struct AgentConfiguration {
    /// Configuration data as JSON
    pub config: serde_json::Value,
    /// Configuration version
    pub version: String,
    /// Last updated timestamp
    pub updated_at: chrono::DateTime<chrono::Utc>,
    /// Configuration source (file, API, etc.)
    pub source: ConfigurationSource,
}

/// Configuration sources
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConfigurationSource {
    /// Loaded from file
    File(String),
    /// Set via API
    API,
    /// Default configuration
    Default,
    /// Environment variables
    Environment,
    /// Custom source
    Custom(String),
}

impl Default for AgentConfiguration {
    fn default() -> Self {
        Self {
            config: serde_json::json!({}),
            version: "1.0.0".to_string(),
            updated_at: chrono::Utc::now(),
            source: ConfigurationSource::Default,
        }
    }
}

/// Component for agent metrics
#[derive(Component, Debug, Clone)]
pub struct AgentMetrics {
    /// Performance metrics
    pub performance: PerformanceMetrics,
    /// Usage metrics
    pub usage: UsageMetrics,
    /// Error metrics
    pub errors: ErrorMetrics,
    /// Last metrics update
    pub last_updated: chrono::DateTime<chrono::Utc>,
}

/// Performance metrics
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    /// Average response time in milliseconds
    pub avg_response_time_ms: f64,
    /// 95th percentile response time
    pub p95_response_time_ms: f64,
    /// 99th percentile response time
    pub p99_response_time_ms: f64,
    /// Throughput (operations per second)
    pub throughput_ops: f64,
}

/// Usage metrics
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UsageMetrics {
    /// Total operations performed
    pub total_operations: u64,
    /// Operations in the last hour
    pub operations_last_hour: u64,
    /// Active sessions
    pub active_sessions: u32,
    /// Total uptime in seconds
    pub uptime_seconds: u64,
}

/// Error metrics
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ErrorMetrics {
    /// Total errors
    pub total_errors: u64,
    /// Errors in the last hour
    pub errors_last_hour: u64,
    /// Error rate (errors per operation)
    pub error_rate: f64,
    /// Most common error types
    pub error_types: HashMap<String, u64>,
}

impl Default for AgentMetrics {
    fn default() -> Self {
        Self {
            performance: PerformanceMetrics::default(),
            usage: UsageMetrics::default(),
            errors: ErrorMetrics::default(),
            last_updated: chrono::Utc::now(),
        }
    }
}

/// Component for agent documentation
#[derive(Component, Debug, Clone)]
#[derive(Default)]
pub struct AgentDocumentation {
    /// API documentation
    pub api_docs: String,
    /// Usage examples
    pub examples: Vec<UsageExample>,
    /// Frequently asked questions
    pub faqs: Vec<FAQ>,
    /// External links
    pub links: Vec<DocumentationLink>,
}

/// Usage example
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsageExample {
    /// Example title
    pub title: String,
    /// Example description
    pub description: String,
    /// Example code or configuration
    pub code: String,
    /// Programming language or format
    pub language: String,
}

/// Frequently asked question
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FAQ {
    /// Question
    pub question: String,
    /// Answer
    pub answer: String,
    /// Category
    pub category: String,
}

/// Documentation link
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentationLink {
    /// Link title
    pub title: String,
    /// URL
    pub url: String,
    /// Link type
    pub link_type: LinkType,
}

/// Documentation link types
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum LinkType {
    /// API reference
    APIReference,
    /// Tutorial
    Tutorial,
    /// GitHub repository
    GitHub,
    /// External documentation
    External,
}


/// Component for agent versioning
#[derive(Component, Debug, Clone)]
pub struct AgentVersion {
    /// Current version
    pub version: semver::Version,
    /// Previous versions
    pub version_history: Vec<VersionRecord>,
    /// Compatibility information
    pub compatibility: CompatibilityInfo,
}

/// Version record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionRecord {
    /// Version number
    pub version: String,
    /// Release date
    pub released_at: chrono::DateTime<chrono::Utc>,
    /// Change summary
    pub changes: Vec<String>,
    /// Whether this version is deprecated
    pub deprecated: bool,
}

/// Compatibility information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompatibilityInfo {
    /// Minimum supported client version
    pub min_client_version: String,
    /// Maximum supported client version
    pub max_client_version: Option<String>,
    /// Compatible API versions
    pub api_versions: Vec<String>,
}

impl Default for AgentVersion {
    fn default() -> Self {
        Self {
            version: semver::Version::new(1, 0, 0),
            version_history: Vec::new(),
            compatibility: CompatibilityInfo {
                min_client_version: "1.0.0".to_string(),
                max_client_version: None,
                api_versions: vec!["v1".to_string()],
            },
        }
    }
} 