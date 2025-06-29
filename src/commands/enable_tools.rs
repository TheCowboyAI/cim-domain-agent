//! Enable tools command

use cim_domain::Command;
use crate::value_objects::{AgentId, ToolAccess};

/// Enable tools for an agent
#[derive(Debug, Clone)]
pub struct EnableAgentTools {
    /// Agent ID
    pub id: AgentId,
    /// Tools to enable
    pub tools: Vec<ToolAccess>,
}

impl Command for EnableAgentTools {
    type Aggregate = crate::aggregate::Agent;

    fn aggregate_id(&self) -> Option<cim_domain::EntityId<Self::Aggregate>> {
        Some(cim_domain::EntityId::from_uuid(self.id.into()))
    }
} 