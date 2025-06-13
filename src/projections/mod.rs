//! Agent projections and read models

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::collections::{HashMap, HashSet};

/// Read model view of an agent
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentView {
    /// Agent ID
    pub id: Uuid,
    /// Agent type
    pub agent_type: crate::AgentType,
    /// Current status
    pub status: crate::AgentStatus,
    /// Owner ID
    pub owner_id: Uuid,
    /// Agent name
    pub name: String,
    /// Description
    pub description: String,
    /// Capabilities
    pub capabilities: HashSet<String>,
    /// Permissions
    pub permissions: HashSet<String>,
    /// Enabled tools
    pub enabled_tools: Vec<String>,
    /// Configuration
    pub configuration: HashMap<String, serde_json::Value>,
    /// Tags
    pub tags: HashSet<String>,
    /// Creation timestamp
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Last activity
    pub last_active: Option<chrono::DateTime<chrono::Utc>>,
    /// Version
    pub version: u64,
}
