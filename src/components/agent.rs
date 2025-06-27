//! Core agent ECS components

use bevy_ecs::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Marker component for agent entities
#[derive(Component, Debug, Clone, Copy)]
pub struct AgentEntity {
    /// The agent's unique identifier
    pub agent_id: Uuid,
}

/// Component representing the agent's type
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AgentTypeComponent {
    /// Human-controlled agent
    Human,
    /// AI/ML model agent
    AI,
    /// System/service agent
    System,
    /// External integration agent
    External,
}

/// Component for the agent's owner
#[derive(Component, Debug, Clone, Copy)]
pub struct AgentOwner {
    /// ID of the person or organization that owns this agent
    pub owner_id: Uuid,
}

/// Component tracking agent relationships
#[derive(Component, Debug, Clone)]
pub struct AgentRelationships {
    /// Parent agent if this is a sub-agent
    pub parent_agent: Option<Uuid>,
    /// Child agents managed by this agent
    pub child_agents: Vec<Uuid>,
    /// Agents this agent collaborates with
    pub collaborators: Vec<Uuid>,
}

impl Default for AgentRelationships {
    fn default() -> Self {
        Self {
            parent_agent: None,
            child_agents: Vec::new(),
            collaborators: Vec::new(),
        }
    }
}

/// Component for agent resource usage
#[derive(Component, Debug, Clone)]
pub struct AgentResourceUsage {
    /// CPU usage percentage (0-100)
    pub cpu_usage: f32,
    /// Memory usage in MB
    pub memory_usage_mb: f32,
    /// Network bandwidth usage in MB/s
    pub network_usage_mbps: f32,
    /// Storage usage in GB
    pub storage_usage_gb: f32,
    /// Last updated timestamp
    pub last_updated: chrono::DateTime<chrono::Utc>,
}

impl Default for AgentResourceUsage {
    fn default() -> Self {
        Self {
            cpu_usage: 0.0,
            memory_usage_mb: 0.0,
            network_usage_mbps: 0.0,
            storage_usage_gb: 0.0,
            last_updated: chrono::Utc::now(),
        }
    }
}

/// Component for agent deployment information
#[derive(Component, Debug, Clone)]
pub struct AgentDeployment {
    /// Environment where the agent is deployed
    pub environment: String,
    /// Host/server identifier
    pub host: String,
    /// Container/process ID if applicable
    pub container_id: Option<String>,
    /// Deployment timestamp
    pub deployed_at: chrono::DateTime<chrono::Utc>,
    /// Deployment version
    pub version: String,
}

/// Component for agent health status
#[derive(Component, Debug, Clone)]
pub struct AgentHealth {
    /// Overall health score (0-100)
    pub health_score: f32,
    /// Last health check timestamp
    pub last_check: chrono::DateTime<chrono::Utc>,
    /// Number of consecutive failed health checks
    pub failed_checks: u32,
    /// Health issues if any
    pub issues: Vec<HealthIssue>,
}

/// Health issue details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthIssue {
    /// Issue severity
    pub severity: HealthSeverity,
    /// Issue description
    pub description: String,
    /// When the issue was detected
    pub detected_at: chrono::DateTime<chrono::Utc>,
}

/// Health issue severity levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HealthSeverity {
    /// Minor issue, agent can continue operating
    Low,
    /// Moderate issue, may affect performance
    Medium,
    /// Severe issue, agent functionality impaired
    High,
    /// Critical issue, agent may fail
    Critical,
}

impl Default for AgentHealth {
    fn default() -> Self {
        Self {
            health_score: 100.0,
            last_check: chrono::Utc::now(),
            failed_checks: 0,
            issues: Vec::new(),
        }
    }
} 