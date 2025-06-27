//! ECS Components for the Agent domain
//!
//! This module contains all Bevy ECS components for representing agents
//! in the presentation layer while maintaining separation from domain logic.

pub mod agent;
pub mod capabilities;
pub mod authentication;
pub mod permissions;
pub mod tools;
pub mod metadata;
pub mod status;

// Re-export all components
pub use agent::*;
pub use capabilities::*;
pub use authentication::*;
pub use permissions::*;
pub use tools::*;
pub use metadata::*;
pub use status::*; 