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
// pub use authentication::*;  // TODO: Implement authentication systems
// pub use permissions::*;     // TODO: Implement permissions systems
// pub use tools::*;           // TODO: Implement tools systems
// pub use monitoring::*;      // TODO: Implement monitoring systems
pub use query::*;

// Re-export commonly used systems
pub use lifecycle::{update_agent_status, process_agent_commands};
pub use capabilities::sync_agent_capabilities; 