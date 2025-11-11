//! Capabilities updated event

use crate::value_objects_new::{AgentId, Capability};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Capabilities updated event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapabilitiesUpdatedEvent {
    pub agent_id: AgentId,
    pub added_capabilities: Vec<Capability>,
    pub removed_capabilities: Vec<String>, // Capability IDs
    pub updated_at: DateTime<Utc>,
    pub updated_by: Option<String>,
}

impl CapabilitiesUpdatedEvent {
    pub fn new(
        agent_id: AgentId,
        added_capabilities: Vec<Capability>,
        removed_capabilities: Vec<String>,
        updated_by: Option<String>,
    ) -> Self {
        Self {
            agent_id,
            added_capabilities,
            removed_capabilities,
            updated_at: Utc::now(),
            updated_by,
        }
    }
}
