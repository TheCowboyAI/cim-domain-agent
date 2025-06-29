//! Disable tools command

use cim_domain::Command;
use crate::value_objects::AgentId;

/// Disable tools for an agent
#[derive(Debug, Clone)]
pub struct DisableAgentTools {
    /// Agent ID
    pub id: AgentId,
    /// Tool IDs to disable
    pub tool_ids: Vec<String>,
}

impl Command for DisableAgentTools {
    type Aggregate = crate::aggregate::Agent;

    fn aggregate_id(&self) -> Option<cim_domain::EntityId<Self::Aggregate>> {
        Some(cim_domain::EntityId::from_uuid(self.id.into()))
    }
} 