//! Agent domain events

pub mod agent_deployed;
pub mod agent_activated;
pub mod agent_suspended;
pub mod agent_decommissioned;
pub mod agent_went_offline;
pub mod capabilities_changed;
pub mod permissions_changed;
pub mod tools_changed;
pub mod ai_events;
pub mod authentication_events;

// Re-export all events
pub use agent_deployed::*;
pub use agent_activated::*;
pub use agent_suspended::*;
pub use agent_decommissioned::*;
pub use agent_went_offline::*;
pub use capabilities_changed::*;
pub use permissions_changed::*;
pub use tools_changed::*;
pub use ai_events::*;
pub use authentication_events::*;
