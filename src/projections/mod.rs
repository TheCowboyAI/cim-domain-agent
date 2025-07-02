//! Agent projections and read models

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::collections::{HashMap, HashSet};
use crate::aggregate::{AgentId, AgentType, AgentStatus};
use chrono::{DateTime, Utc};

/// Read model view of an agent
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentView {
    /// Agent ID
    pub id: AgentId,
    /// Agent type
    pub agent_type: AgentType,
    /// Current status
    pub status: AgentStatus,
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
    pub created_at: DateTime<Utc>,
    /// Last activity
    pub last_active: Option<DateTime<Utc>>,
    /// Version
    pub version: u64,
}
