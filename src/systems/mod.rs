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