//! Agent status value object
//!
//! Represents the operational state of an agent in its lifecycle.

use serde::{Deserialize, Serialize};
use std::fmt;

/// Agent operational status
///
/// Lifecycle: Deployed → Activated → (Active ↔ Suspended ↔ Offline) → Decommissioned
///
/// ```text
/// Deployed ──┐
///            ├──> Activated ──> Active ←──┐
///            │                    ↓        │
///            │                 Suspended ──┘
///            │                    ↓
///            │                 Offline
///            │                    ↓
///            └──────────────> Decommissioned
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AgentStatus {
    /// Agent has been deployed but not yet activated
    ///
    /// Initial state when an agent is first created.
    /// Configuration and setup can occur in this state.
    Deployed,

    /// Agent is operational and actively processing
    ///
    /// The agent is running and can execute tasks.
    Active,

    /// Agent is temporarily suspended
    ///
    /// The agent is paused but can be reactivated.
    /// State and configuration are preserved.
    Suspended,

    /// Agent is offline/unavailable
    ///
    /// The agent lost connectivity or crashed.
    /// May automatically transition back to Active when recovered.
    Offline,

    /// Agent has been permanently decommissioned
    ///
    /// Terminal state - agent cannot be reactivated.
    /// All resources should be cleaned up.
    Decommissioned,
}

impl AgentStatus {
    /// Check if the agent can execute tasks in this status
    ///
    /// # Examples
    ///
    /// ```
    /// use cim_domain_agent::AgentStatus;
    ///
    /// assert!(AgentStatus::Active.can_execute());
    /// assert!(!AgentStatus::Suspended.can_execute());
    /// assert!(!AgentStatus::Decommissioned.can_execute());
    /// ```
    pub fn can_execute(&self) -> bool {
        matches!(self, Self::Active)
    }

    /// Check if the agent can be activated from this status
    ///
    /// # Examples
    ///
    /// ```
    /// use cim_domain_agent::AgentStatus;
    ///
    /// assert!(AgentStatus::Deployed.can_activate());
    /// assert!(AgentStatus::Suspended.can_activate());
    /// assert!(!AgentStatus::Decommissioned.can_activate());
    /// ```
    pub fn can_activate(&self) -> bool {
        matches!(self, Self::Deployed | Self::Suspended | Self::Offline)
    }

    /// Check if the agent can be suspended from this status
    ///
    /// # Examples
    ///
    /// ```
    /// use cim_domain_agent::AgentStatus;
    ///
    /// assert!(AgentStatus::Active.can_suspend());
    /// assert!(!AgentStatus::Deployed.can_suspend());
    /// ```
    pub fn can_suspend(&self) -> bool {
        matches!(self, Self::Active)
    }

    /// Check if the agent can go offline from this status
    ///
    /// # Examples
    ///
    /// ```
    /// use cim_domain_agent::AgentStatus;
    ///
    /// assert!(AgentStatus::Active.can_go_offline());
    /// assert!(AgentStatus::Suspended.can_go_offline());
    /// ```
    pub fn can_go_offline(&self) -> bool {
        matches!(self, Self::Active | Self::Suspended)
    }

    /// Check if the agent can be decommissioned from this status
    ///
    /// # Examples
    ///
    /// ```
    /// use cim_domain_agent::AgentStatus;
    ///
    /// assert!(AgentStatus::Active.can_decommission());
    /// assert!(AgentStatus::Suspended.can_decommission());
    /// assert!(!AgentStatus::Decommissioned.can_decommission());
    /// ```
    pub fn can_decommission(&self) -> bool {
        !matches!(self, Self::Decommissioned)
    }

    /// Check if this is a terminal status
    ///
    /// # Examples
    ///
    /// ```
    /// use cim_domain_agent::AgentStatus;
    ///
    /// assert!(AgentStatus::Decommissioned.is_terminal());
    /// assert!(!AgentStatus::Active.is_terminal());
    /// ```
    pub fn is_terminal(&self) -> bool {
        matches!(self, Self::Decommissioned)
    }

    /// Get a human-readable description
    ///
    /// # Examples
    ///
    /// ```
    /// use cim_domain_agent::AgentStatus;
    ///
    /// let desc = AgentStatus::Active.description();
    /// assert!(desc.contains("operational"));
    /// ```
    pub fn description(&self) -> &'static str {
        match self {
            Self::Deployed => "Agent deployed but not yet activated",
            Self::Active => "Agent is operational and actively processing",
            Self::Suspended => "Agent is temporarily suspended",
            Self::Offline => "Agent is offline or unavailable",
            Self::Decommissioned => "Agent has been permanently decommissioned",
        }
    }

    /// Get the status code (for storage/display)
    ///
    /// # Examples
    ///
    /// ```
    /// use cim_domain_agent::AgentStatus;
    ///
    /// assert_eq!(AgentStatus::Active.code(), "ACTIVE");
    /// ```
    pub fn code(&self) -> &'static str {
        match self {
            Self::Deployed => "DEPLOYED",
            Self::Active => "ACTIVE",
            Self::Suspended => "SUSPENDED",
            Self::Offline => "OFFLINE",
            Self::Decommissioned => "DECOMMISSIONED",
        }
    }

    /// Parse status from code
    ///
    /// # Examples
    ///
    /// ```
    /// use cim_domain_agent::AgentStatus;
    ///
    /// assert_eq!(AgentStatus::from_code("ACTIVE"), Some(AgentStatus::Active));
    /// assert_eq!(AgentStatus::from_code("active"), Some(AgentStatus::Active));
    /// assert_eq!(AgentStatus::from_code("INVALID"), None);
    /// ```
    pub fn from_code(code: &str) -> Option<Self> {
        match code.to_uppercase().as_str() {
            "DEPLOYED" => Some(Self::Deployed),
            "ACTIVE" => Some(Self::Active),
            "SUSPENDED" => Some(Self::Suspended),
            "OFFLINE" => Some(Self::Offline),
            "DECOMMISSIONED" => Some(Self::Decommissioned),
            _ => None,
        }
    }

    /// Get all possible statuses
    ///
    /// # Examples
    ///
    /// ```
    /// use cim_domain_agent::AgentStatus;
    ///
    /// let statuses = AgentStatus::all();
    /// assert_eq!(statuses.len(), 5);
    /// ```
    pub fn all() -> Vec<Self> {
        vec![
            Self::Deployed,
            Self::Active,
            Self::Suspended,
            Self::Offline,
            Self::Decommissioned,
        ]
    }
}

impl fmt::Display for AgentStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.code())
    }
}

impl Default for AgentStatus {
    fn default() -> Self {
        Self::Deployed
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_execute() {
        assert!(AgentStatus::Active.can_execute());
        assert!(!AgentStatus::Deployed.can_execute());
        assert!(!AgentStatus::Suspended.can_execute());
        assert!(!AgentStatus::Offline.can_execute());
        assert!(!AgentStatus::Decommissioned.can_execute());
    }

    #[test]
    fn test_can_activate() {
        assert!(AgentStatus::Deployed.can_activate());
        assert!(AgentStatus::Suspended.can_activate());
        assert!(AgentStatus::Offline.can_activate());
        assert!(!AgentStatus::Active.can_activate());
        assert!(!AgentStatus::Decommissioned.can_activate());
    }

    #[test]
    fn test_can_suspend() {
        assert!(AgentStatus::Active.can_suspend());
        assert!(!AgentStatus::Deployed.can_suspend());
        assert!(!AgentStatus::Suspended.can_suspend());
        assert!(!AgentStatus::Offline.can_suspend());
        assert!(!AgentStatus::Decommissioned.can_suspend());
    }

    #[test]
    fn test_can_go_offline() {
        assert!(AgentStatus::Active.can_go_offline());
        assert!(AgentStatus::Suspended.can_go_offline());
        assert!(!AgentStatus::Deployed.can_go_offline());
        assert!(!AgentStatus::Offline.can_go_offline());
        assert!(!AgentStatus::Decommissioned.can_go_offline());
    }

    #[test]
    fn test_can_decommission() {
        assert!(AgentStatus::Deployed.can_decommission());
        assert!(AgentStatus::Active.can_decommission());
        assert!(AgentStatus::Suspended.can_decommission());
        assert!(AgentStatus::Offline.can_decommission());
        assert!(!AgentStatus::Decommissioned.can_decommission());
    }

    #[test]
    fn test_is_terminal() {
        assert!(AgentStatus::Decommissioned.is_terminal());
        assert!(!AgentStatus::Active.is_terminal());
        assert!(!AgentStatus::Suspended.is_terminal());
    }

    #[test]
    fn test_description() {
        let desc = AgentStatus::Active.description();
        assert!(!desc.is_empty());
    }

    #[test]
    fn test_code() {
        assert_eq!(AgentStatus::Deployed.code(), "DEPLOYED");
        assert_eq!(AgentStatus::Active.code(), "ACTIVE");
        assert_eq!(AgentStatus::Suspended.code(), "SUSPENDED");
        assert_eq!(AgentStatus::Offline.code(), "OFFLINE");
        assert_eq!(AgentStatus::Decommissioned.code(), "DECOMMISSIONED");
    }

    #[test]
    fn test_from_code() {
        assert_eq!(AgentStatus::from_code("ACTIVE"), Some(AgentStatus::Active));
        assert_eq!(AgentStatus::from_code("active"), Some(AgentStatus::Active));
        assert_eq!(AgentStatus::from_code("DEPLOYED"), Some(AgentStatus::Deployed));
        assert_eq!(AgentStatus::from_code("INVALID"), None);
    }

    #[test]
    fn test_all() {
        let statuses = AgentStatus::all();
        assert_eq!(statuses.len(), 5);
        assert!(statuses.contains(&AgentStatus::Deployed));
        assert!(statuses.contains(&AgentStatus::Active));
        assert!(statuses.contains(&AgentStatus::Suspended));
        assert!(statuses.contains(&AgentStatus::Offline));
        assert!(statuses.contains(&AgentStatus::Decommissioned));
    }

    #[test]
    fn test_display() {
        assert_eq!(format!("{}", AgentStatus::Active), "ACTIVE");
        assert_eq!(format!("{}", AgentStatus::Suspended), "SUSPENDED");
    }

    #[test]
    fn test_serialization() {
        let status = AgentStatus::Active;
        let json = serde_json::to_string(&status).unwrap();
        let deserialized: AgentStatus = serde_json::from_str(&json).unwrap();
        assert_eq!(status, deserialized);
    }

    #[test]
    fn test_lifecycle_transitions() {
        // Deployed -> Active
        let status = AgentStatus::Deployed;
        assert!(status.can_activate());

        // Active -> Suspended
        let status = AgentStatus::Active;
        assert!(status.can_suspend());

        // Suspended -> Active
        let status = AgentStatus::Suspended;
        assert!(status.can_activate());

        // Active -> Offline
        let status = AgentStatus::Active;
        assert!(status.can_go_offline());

        // Offline -> Active
        let status = AgentStatus::Offline;
        assert!(status.can_activate());

        // Any -> Decommissioned
        let status = AgentStatus::Active;
        assert!(status.can_decommission());

        // Decommissioned -> (terminal)
        let status = AgentStatus::Decommissioned;
        assert!(!status.can_activate());
        assert!(!status.can_decommission());
    }
}
