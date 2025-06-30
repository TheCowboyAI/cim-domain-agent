//! Integration module for domain-ECS communication

pub mod bridge;
pub mod plugin;
pub mod conceptual_spaces;

pub use bridge::*;
pub use plugin::*;
pub use conceptual_spaces::*; 