//! Suspend agent command

use cim_domain::Command;
use crate::value_objects::AgentId;

/// Suspend an agent
#[derive(Debug, Clone)]
pub struct SuspendAgent {
    /// Agent ID
    pub id: AgentId,
    /// Reason for suspension
    pub reason: String,
}

impl Command for SuspendAgent {
    type Aggregate = crate::aggregate::Agent;

    fn aggregate_id(&self) -> Option<cim_domain::EntityId<Self::Aggregate>> {
        Some(cim_domain::EntityId::from_uuid(self.id.into()))
    }
} 