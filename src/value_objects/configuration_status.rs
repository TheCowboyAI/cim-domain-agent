// Copyright (c) 2025 - Cowboy AI, LLC.

//! Configuration lifecycle status value object
//!
//! Tracks the lifecycle state of a ModelConfiguration aggregate.

use serde::{Deserialize, Serialize};
use std::fmt;

/// Lifecycle status of a ModelConfiguration
///
/// # State Transitions
///
/// ```text
/// Draft → Active → Deprecated → Archived
///   ↑       ↓
///   └───────┘ (can return to Draft for modifications)
/// ```
///
/// - **Draft**: Configuration is being created/modified, not yet in use
/// - **Active**: Configuration is live and being used by agents
/// - **Deprecated**: Configuration is being phased out, use discouraged
/// - **Archived**: Configuration is historical only, cannot be used
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ConfigurationStatus {
    /// Configuration is being created or modified
    ///
    /// - Can be edited freely
    /// - Not yet assigned to agents
    /// - Can transition to Active
    Draft,

    /// Configuration is live and in use
    ///
    /// - Can be assigned to agents
    /// - Should not be modified without careful consideration
    /// - Can transition to Deprecated or back to Draft
    Active,

    /// Configuration is being phased out
    ///
    /// - Existing agents can continue using it
    /// - New agents should not use it
    /// - Can transition to Archived
    Deprecated,

    /// Configuration is historical only
    ///
    /// - Cannot be used by agents
    /// - Retained for audit trail
    /// - Terminal state (no transitions out)
    Archived,
}

impl ConfigurationStatus {
    /// Check if configuration can be edited
    ///
    /// Only Draft configurations can be freely edited.
    ///
    /// # Example
    ///
    /// ```
    /// use cim_domain_agent::value_objects::ConfigurationStatus;
    ///
    /// assert!(ConfigurationStatus::Draft.can_edit());
    /// assert!(!ConfigurationStatus::Active.can_edit());
    /// assert!(!ConfigurationStatus::Archived.can_edit());
    /// ```
    pub fn can_edit(&self) -> bool {
        matches!(self, ConfigurationStatus::Draft)
    }

    /// Check if configuration can be assigned to agents
    ///
    /// Only Active configurations can be assigned to new agents.
    ///
    /// # Example
    ///
    /// ```
    /// use cim_domain_agent::value_objects::ConfigurationStatus;
    ///
    /// assert!(!ConfigurationStatus::Draft.can_assign());
    /// assert!(ConfigurationStatus::Active.can_assign());
    /// assert!(!ConfigurationStatus::Deprecated.can_assign());
    /// ```
    pub fn can_assign(&self) -> bool {
        matches!(self, ConfigurationStatus::Active)
    }

    /// Check if configuration can be used by existing agents
    ///
    /// Active and Deprecated configurations can still be used by agents
    /// that already have them assigned.
    ///
    /// # Example
    ///
    /// ```
    /// use cim_domain_agent::value_objects::ConfigurationStatus;
    ///
    /// assert!(!ConfigurationStatus::Draft.can_use());
    /// assert!(ConfigurationStatus::Active.can_use());
    /// assert!(ConfigurationStatus::Deprecated.can_use());
    /// assert!(!ConfigurationStatus::Archived.can_use());
    /// ```
    pub fn can_use(&self) -> bool {
        matches!(self, ConfigurationStatus::Active | ConfigurationStatus::Deprecated)
    }

    /// Check if this is a terminal state
    ///
    /// Archived is the only terminal state.
    ///
    /// # Example
    ///
    /// ```
    /// use cim_domain_agent::value_objects::ConfigurationStatus;
    ///
    /// assert!(!ConfigurationStatus::Draft.is_terminal());
    /// assert!(!ConfigurationStatus::Active.is_terminal());
    /// assert!(!ConfigurationStatus::Deprecated.is_terminal());
    /// assert!(ConfigurationStatus::Archived.is_terminal());
    /// ```
    pub fn is_terminal(&self) -> bool {
        matches!(self, ConfigurationStatus::Archived)
    }

    /// Check if transition to another status is valid
    ///
    /// # Valid Transitions
    ///
    /// - Draft → Active
    /// - Active → Draft (for modifications)
    /// - Active → Deprecated
    /// - Deprecated → Archived
    /// - Deprecated → Active (if revived)
    ///
    /// # Example
    ///
    /// ```
    /// use cim_domain_agent::value_objects::ConfigurationStatus;
    ///
    /// assert!(ConfigurationStatus::Draft.can_transition_to(ConfigurationStatus::Active));
    /// assert!(ConfigurationStatus::Active.can_transition_to(ConfigurationStatus::Deprecated));
    /// assert!(!ConfigurationStatus::Archived.can_transition_to(ConfigurationStatus::Active));
    /// ```
    pub fn can_transition_to(&self, target: ConfigurationStatus) -> bool {
        match (self, target) {
            // Draft can go to Active
            (ConfigurationStatus::Draft, ConfigurationStatus::Active) => true,

            // Active can go to Draft (for edits) or Deprecated
            (ConfigurationStatus::Active, ConfigurationStatus::Draft) => true,
            (ConfigurationStatus::Active, ConfigurationStatus::Deprecated) => true,

            // Deprecated can go to Archived or be revived to Active
            (ConfigurationStatus::Deprecated, ConfigurationStatus::Archived) => true,
            (ConfigurationStatus::Deprecated, ConfigurationStatus::Active) => true,

            // Archived is terminal - no transitions out
            (ConfigurationStatus::Archived, _) => false,

            // All other transitions invalid
            _ => false,
        }
    }

    /// Get human-readable description
    pub fn description(&self) -> &'static str {
        match self {
            ConfigurationStatus::Draft => "Configuration is being created or modified",
            ConfigurationStatus::Active => "Configuration is live and in use",
            ConfigurationStatus::Deprecated => "Configuration is being phased out",
            ConfigurationStatus::Archived => "Configuration is historical only",
        }
    }
}

impl Default for ConfigurationStatus {
    fn default() -> Self {
        ConfigurationStatus::Draft
    }
}

impl fmt::Display for ConfigurationStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            ConfigurationStatus::Draft => "Draft",
            ConfigurationStatus::Active => "Active",
            ConfigurationStatus::Deprecated => "Deprecated",
            ConfigurationStatus::Archived => "Archived",
        };
        write!(f, "{}", s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_edit() {
        assert!(ConfigurationStatus::Draft.can_edit());
        assert!(!ConfigurationStatus::Active.can_edit());
        assert!(!ConfigurationStatus::Deprecated.can_edit());
        assert!(!ConfigurationStatus::Archived.can_edit());
    }

    #[test]
    fn test_can_assign() {
        assert!(!ConfigurationStatus::Draft.can_assign());
        assert!(ConfigurationStatus::Active.can_assign());
        assert!(!ConfigurationStatus::Deprecated.can_assign());
        assert!(!ConfigurationStatus::Archived.can_assign());
    }

    #[test]
    fn test_can_use() {
        assert!(!ConfigurationStatus::Draft.can_use());
        assert!(ConfigurationStatus::Active.can_use());
        assert!(ConfigurationStatus::Deprecated.can_use());
        assert!(!ConfigurationStatus::Archived.can_use());
    }

    #[test]
    fn test_is_terminal() {
        assert!(!ConfigurationStatus::Draft.is_terminal());
        assert!(!ConfigurationStatus::Active.is_terminal());
        assert!(!ConfigurationStatus::Deprecated.is_terminal());
        assert!(ConfigurationStatus::Archived.is_terminal());
    }

    #[test]
    fn test_valid_transitions() {
        // Draft → Active
        assert!(ConfigurationStatus::Draft.can_transition_to(ConfigurationStatus::Active));

        // Active → Draft (for edits)
        assert!(ConfigurationStatus::Active.can_transition_to(ConfigurationStatus::Draft));

        // Active → Deprecated
        assert!(ConfigurationStatus::Active.can_transition_to(ConfigurationStatus::Deprecated));

        // Deprecated → Archived
        assert!(ConfigurationStatus::Deprecated.can_transition_to(ConfigurationStatus::Archived));

        // Deprecated → Active (revival)
        assert!(ConfigurationStatus::Deprecated.can_transition_to(ConfigurationStatus::Active));
    }

    #[test]
    fn test_invalid_transitions() {
        // Draft → Archived (must go through Active/Deprecated)
        assert!(!ConfigurationStatus::Draft.can_transition_to(ConfigurationStatus::Archived));

        // Active → Archived (must go through Deprecated)
        assert!(!ConfigurationStatus::Active.can_transition_to(ConfigurationStatus::Archived));

        // Archived → anything (terminal state)
        assert!(!ConfigurationStatus::Archived.can_transition_to(ConfigurationStatus::Active));
        assert!(!ConfigurationStatus::Archived.can_transition_to(ConfigurationStatus::Draft));
        assert!(!ConfigurationStatus::Archived.can_transition_to(ConfigurationStatus::Deprecated));
    }

    #[test]
    fn test_display() {
        assert_eq!(format!("{}", ConfigurationStatus::Draft), "Draft");
        assert_eq!(format!("{}", ConfigurationStatus::Active), "Active");
        assert_eq!(format!("{}", ConfigurationStatus::Deprecated), "Deprecated");
        assert_eq!(format!("{}", ConfigurationStatus::Archived), "Archived");
    }

    #[test]
    fn test_serialization() {
        let status = ConfigurationStatus::Active;
        let json = serde_json::to_string(&status).unwrap();
        assert_eq!(json, "\"active\"");

        let deserialized: ConfigurationStatus = serde_json::from_str(&json).unwrap();
        assert_eq!(status, deserialized);
    }

    #[test]
    fn test_default() {
        assert_eq!(ConfigurationStatus::default(), ConfigurationStatus::Draft);
    }

    #[test]
    fn test_description() {
        assert!(!ConfigurationStatus::Draft.description().is_empty());
        assert!(!ConfigurationStatus::Active.description().is_empty());
        assert!(!ConfigurationStatus::Deprecated.description().is_empty());
        assert!(!ConfigurationStatus::Archived.description().is_empty());
    }

    #[test]
    fn test_hash_and_eq() {
        use std::collections::HashSet;

        let mut set = HashSet::new();
        set.insert(ConfigurationStatus::Draft);
        set.insert(ConfigurationStatus::Active);
        set.insert(ConfigurationStatus::Draft); // Duplicate

        assert_eq!(set.len(), 2); // Only 2 unique statuses
        assert!(set.contains(&ConfigurationStatus::Draft));
        assert!(set.contains(&ConfigurationStatus::Active));
        assert!(!set.contains(&ConfigurationStatus::Deprecated));
    }
}
