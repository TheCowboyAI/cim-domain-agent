//! Agent query definitions

use uuid::Uuid;
use cim_domain::Query;

/// Query for agent information
#[derive(Debug, Clone)]
pub enum AgentQuery {
    /// Get agent by ID
    GetById(Uuid),
    /// Get agents by owner
    GetByOwner(Uuid),
    /// Get agents by type
    GetByType(crate::AgentType),
    /// Get agents by status
    GetByStatus(crate::AgentStatus),
    /// Get agents with capability
    GetByCapability(String),
}

impl Query for AgentQuery {}

/// Agent query handler
pub struct AgentQueryHandler<R> {
    read_model: R,
}

impl<R> AgentQueryHandler<R> {
    /// Create a new query handler
    pub fn new(read_model: R) -> Self {
        Self { read_model }
    }
}
