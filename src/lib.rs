//! Agent domain module
//!
//! This module contains all agent-related domain logic including:
//! - Agent aggregate and components
//! - Agent commands and events
//! - Agent command and query handlers

pub mod aggregate;
pub mod commands;
pub mod events;
pub mod handlers;
pub mod projections;
pub mod queries;
pub mod value_objects;

// ECS modules
pub mod components;
pub mod systems;

// Re-export main types
pub use aggregate::{
    Agent, AgentMarker, AgentMetadata, AgentStatus, AgentType,
    AuthMethod, AuthenticationComponent, CapabilitiesComponent,
    ConfigurationComponent, PermissionsComponent, ToolAccessComponent,
    ToolDefinition, ToolUsageStats,
};

pub use commands::{
    ActivateAgent, DecommissionAgent, DeployAgent, DisableAgentTools,
    EnableAgentTools, GrantAgentPermissions, RemoveAgentConfiguration,
    RevokeAgentPermissions, SetAgentConfiguration, SetAgentOffline,
    SuspendAgent, ChangeAgentCapabilities,
};

pub use events::{
    AgentActivated, AgentCapabilitiesAdded, AgentCapabilitiesRemoved,
    AgentConfigurationRemoved, AgentConfigurationSet, AgentDecommissioned,
    AgentDeployed, AgentPermissionsGranted, AgentPermissionsRevoked,
    AgentSuspended, AgentToolsDisabled, AgentToolsEnabled, AgentWentOffline,
};

pub use handlers::{AgentCommandHandler, AgentEventHandler};
pub use projections::AgentView;
pub use queries::{AgentQuery, AgentQueryHandler};

// Re-export ECS types
pub use components::*;
pub use systems::*;
