//! Agent domain commands

pub mod deploy_agent;
pub mod activate_agent;
pub mod suspend_agent;
pub mod decommission_agent;
pub mod update_capabilities;
pub mod grant_permissions;
pub mod revoke_permissions;
pub mod enable_tools;
pub mod disable_tools;
pub mod ai_commands;

// Re-export all commands
pub use deploy_agent::*;
pub use activate_agent::*;
pub use suspend_agent::*;
pub use decommission_agent::*;
pub use update_capabilities::*;
pub use grant_permissions::*;
pub use revoke_permissions::*;
pub use enable_tools::*;
pub use disable_tools::*;
pub use ai_commands::*;
