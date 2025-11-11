//! Value objects for agent domain (v0.8.1)
//!
//! Pure functional, immutable value objects following v0.8.1 standards.

mod agent_id;
mod agent_type;
mod agent_status;
mod agent_metadata;
mod capability;
mod permission;
mod tool_definition;
mod configuration;

pub use agent_id::AgentId;
pub use agent_type::AgentType;
pub use agent_status::AgentStatus;
pub use agent_metadata::AgentMetadata;
pub use capability::{Capability, CapabilityId, CapabilityPort};
pub use permission::{Permission, PermissionId};
pub use tool_definition::{ToolDefinition, ToolId};
pub use configuration::AgentConfiguration;
