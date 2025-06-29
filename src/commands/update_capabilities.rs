//! Update agent capabilities command

use cim_domain::Command;
use crate::value_objects::AgentId;

/// Update agent capabilities
#[derive(Debug, Clone)]
pub struct UpdateAgentCapabilities {
    /// Agent ID
    pub id: AgentId,
    /// Capabilities to add
    pub add_capabilities: Vec<String>,
    /// Capabilities to remove
    pub remove_capabilities: Vec<String>,
}

impl Command for UpdateAgentCapabilities {
    type Aggregate = crate::aggregate::Agent;

    fn aggregate_id(&self) -> Option<cim_domain::EntityId<Self::Aggregate>> {
        Some(cim_domain::EntityId::from_uuid(self.id.into()))
    }
} 