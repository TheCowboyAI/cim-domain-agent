//! Decommission agent command

use cim_domain::Command;
use crate::value_objects::AgentId;

/// Decommission an agent
#[derive(Debug, Clone)]
pub struct DecommissionAgent {
    /// Agent ID
    pub id: AgentId,
}

impl Command for DecommissionAgent {
    type Aggregate = crate::aggregate::Agent;

    fn aggregate_id(&self) -> Option<cim_domain::EntityId<Self::Aggregate>> {
        Some(cim_domain::EntityId::from_uuid(self.id.into()))
    }
} 