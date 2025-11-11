//! NATS subject algebra for the Agent domain
//!
//! Aligns with `cim-domain` subject abstraction and conventions used across domains.

use cim_domain::{SubjectPattern, SubjectParser};

/// Agent domain subject root
pub const AGENT: &str = "agent";

/// Common subject categories
pub mod categories {
    pub const COMMANDS: &str = "commands";
    pub const EVENTS: &str = "events";
    pub const QUERIES: &str = "queries";
}

/// Agent commands
pub mod commands {
    pub const DEPLOY: &str = "deploy";
    pub const ACTIVATE: &str = "activate";
    pub const SUSPEND: &str = "suspend";
    pub const DECOMMISSION: &str = "decommission";
    pub const UPDATE_CAPABILITIES: &str = "update_capabilities";
    pub const GRANT_PERMISSIONS: &str = "grant_permissions";
    pub const REVOKE_PERMISSIONS: &str = "revoke_permissions";
    pub const ENABLE_TOOLS: &str = "enable_tools";
    pub const DISABLE_TOOLS: &str = "disable_tools";
}

/// Agent events
pub mod events {
    pub const DEPLOYED: &str = "deployed";
    pub const ACTIVATED: &str = "activated";
    pub const SUSPENDED: &str = "suspended";
    pub const DECOMMISSIONED: &str = "decommissioned";
    pub const WENT_OFFLINE: &str = "went_offline";
    pub const CAPABILITIES_CHANGED: &str = "capabilities_changed";
    pub const PERMISSIONS_CHANGED: &str = "permissions_changed";
    pub const TOOLS_CHANGED: &str = "tools_changed";
}

/// Build a subject string from parts (e.g., agent.commands.deploy)
pub fn subject(parts: &[&str]) -> String {
    parts.join(".")
}

/// Match helper using `cim-domain` subject patterns
pub fn matches_pattern(subject: &str, pattern: &str) -> bool {
    let parser = SubjectParser::default();
    match parser.matches(subject, pattern) {
        Ok(result) => result,
        Err(_) => false,
    }
}

/// Canonical subjects
pub mod canonical {
    use super::*;

    pub fn command(cmd: &str) -> String {
        subject(&[AGENT, categories::COMMANDS, cmd])
    }

    pub fn event(evt: &str) -> String {
        subject(&[AGENT, categories::EVENTS, evt])
    }

    pub fn query(q: &str) -> String {
        subject(&[AGENT, categories::QUERIES, q])
    }
}

/// Wildcard subscription patterns
pub mod patterns {
    use super::*;

    pub fn all() -> String { subject(&[AGENT, ">"]) }
    pub fn commands() -> String { subject(&[AGENT, categories::COMMANDS, ">"]) }
    pub fn events() -> String { subject(&[AGENT, categories::EVENTS, ">"]) }
    pub fn queries() -> String { subject(&[AGENT, categories::QUERIES, ">"]) }
}


