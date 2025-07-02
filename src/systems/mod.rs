//! ECS Systems for the Agent domain
//!
//! This module contains all Bevy ECS systems for agent behavior
//! and interaction in the presentation layer.

pub mod lifecycle;
pub mod capabilities;
pub mod authentication;
pub mod permissions;
pub mod tools;
pub mod monitoring;
pub mod query;

// Re-export all systems
pub use lifecycle::*;
pub use capabilities::*;
pub use authentication::*;
pub use permissions::*;
pub use tools::*;
pub use monitoring::*;
pub use query::*;

// Re-export commonly used systems
pub use lifecycle::update_agent_status;
pub use capabilities::sync_agent_capabilities;
pub use authentication::{handle_authentication_requests, check_authentication_expiry};
pub use permissions::{handle_permission_changes, sync_permissions_with_capabilities};
pub use tools::{handle_tool_registration, handle_tool_execution};
pub use monitoring::{update_agent_metrics, perform_health_checks}; 