//! Agent type value object
//!
//! Defines the different categories of agents in the system.

use serde::{Deserialize, Serialize};
use std::fmt;

/// Agent type classification
///
/// Distinguishes between different kinds of mechanical substitutes:
/// - System: Internal automation agents
/// - AI: Machine learning/AI-powered agents
/// - External: Third-party integration agents
/// - Integration: Service-to-service connectors
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AgentType {
    /// System automation agent
    ///
    /// Internal agents that perform system-level tasks:
    /// - Monitoring and health checks
    /// - Data cleanup and maintenance
    /// - Scheduled job execution
    /// - Internal workflow automation
    System,

    /// AI/ML-powered agent
    ///
    /// Agents with machine learning capabilities:
    /// - Natural language processing
    /// - Image/video analysis
    /// - Predictive analytics
    /// - Generative AI tasks
    AI,

    /// External integration agent
    ///
    /// Agents that interface with external systems:
    /// - Third-party API connectors
    /// - External service clients
    /// - Legacy system bridges
    /// - Partner integrations
    External,

    /// Service integration agent
    ///
    /// Agents that connect internal services:
    /// - Event bus connectors
    /// - Message queue consumers
    /// - Service mesh participants
    /// - Inter-domain communication
    Integration,
}

impl AgentType {
    /// Get all agent types
    ///
    /// # Examples
    ///
    /// ```
    /// use cim_domain_agent::value_objects_new::AgentType;
    ///
    /// let types = AgentType::all();
    /// assert_eq!(types.len(), 4);
    /// ```
    pub fn all() -> Vec<Self> {
        vec![
            Self::System,
            Self::AI,
            Self::External,
            Self::Integration,
        ]
    }

    /// Check if this agent type can have AI capabilities
    ///
    /// # Examples
    ///
    /// ```
    /// use cim_domain_agent::value_objects_new::AgentType;
    ///
    /// assert!(AgentType::AI.can_have_ai_capabilities());
    /// assert!(!AgentType::System.can_have_ai_capabilities());
    /// ```
    pub fn can_have_ai_capabilities(&self) -> bool {
        matches!(self, Self::AI)
    }

    /// Check if this agent type requires external configuration
    ///
    /// # Examples
    ///
    /// ```
    /// use cim_domain_agent::value_objects_new::AgentType;
    ///
    /// assert!(AgentType::External.requires_external_config());
    /// assert!(!AgentType::System.requires_external_config());
    /// ```
    pub fn requires_external_config(&self) -> bool {
        matches!(self, Self::External | Self::Integration)
    }

    /// Get a human-readable description
    ///
    /// # Examples
    ///
    /// ```
    /// use cim_domain_agent::value_objects_new::AgentType;
    ///
    /// let desc = AgentType::AI.description();
    /// assert!(desc.contains("AI"));
    /// ```
    pub fn description(&self) -> &'static str {
        match self {
            Self::System => "Internal system automation agent",
            Self::AI => "AI/ML-powered intelligent agent",
            Self::External => "External third-party integration agent",
            Self::Integration => "Internal service integration agent",
        }
    }

    /// Get the agent type code (for storage/serialization)
    ///
    /// # Examples
    ///
    /// ```
    /// use cim_domain_agent::value_objects_new::AgentType;
    ///
    /// assert_eq!(AgentType::AI.code(), "AI");
    /// assert_eq!(AgentType::System.code(), "SYSTEM");
    /// ```
    pub fn code(&self) -> &'static str {
        match self {
            Self::System => "SYSTEM",
            Self::AI => "AI",
            Self::External => "EXTERNAL",
            Self::Integration => "INTEGRATION",
        }
    }

    /// Parse agent type from code
    ///
    /// # Examples
    ///
    /// ```
    /// use cim_domain_agent::value_objects_new::AgentType;
    ///
    /// assert_eq!(AgentType::from_code("AI"), Some(AgentType::AI));
    /// assert_eq!(AgentType::from_code("SYSTEM"), Some(AgentType::System));
    /// assert_eq!(AgentType::from_code("INVALID"), None);
    /// ```
    pub fn from_code(code: &str) -> Option<Self> {
        match code.to_uppercase().as_str() {
            "SYSTEM" => Some(Self::System),
            "AI" => Some(Self::AI),
            "EXTERNAL" => Some(Self::External),
            "INTEGRATION" => Some(Self::Integration),
            _ => None,
        }
    }
}

impl fmt::Display for AgentType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.code())
    }
}

impl Default for AgentType {
    fn default() -> Self {
        Self::System
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_agent_type_all() {
        let types = AgentType::all();
        assert_eq!(types.len(), 4);
        assert!(types.contains(&AgentType::System));
        assert!(types.contains(&AgentType::AI));
        assert!(types.contains(&AgentType::External));
        assert!(types.contains(&AgentType::Integration));
    }

    #[test]
    fn test_can_have_ai_capabilities() {
        assert!(AgentType::AI.can_have_ai_capabilities());
        assert!(!AgentType::System.can_have_ai_capabilities());
        assert!(!AgentType::External.can_have_ai_capabilities());
        assert!(!AgentType::Integration.can_have_ai_capabilities());
    }

    #[test]
    fn test_requires_external_config() {
        assert!(!AgentType::System.requires_external_config());
        assert!(!AgentType::AI.requires_external_config());
        assert!(AgentType::External.requires_external_config());
        assert!(AgentType::Integration.requires_external_config());
    }

    #[test]
    fn test_description() {
        let desc = AgentType::AI.description();
        assert!(!desc.is_empty());
        assert!(desc.contains("AI"));
    }

    #[test]
    fn test_code() {
        assert_eq!(AgentType::System.code(), "SYSTEM");
        assert_eq!(AgentType::AI.code(), "AI");
        assert_eq!(AgentType::External.code(), "EXTERNAL");
        assert_eq!(AgentType::Integration.code(), "INTEGRATION");
    }

    #[test]
    fn test_from_code() {
        assert_eq!(AgentType::from_code("SYSTEM"), Some(AgentType::System));
        assert_eq!(AgentType::from_code("system"), Some(AgentType::System));
        assert_eq!(AgentType::from_code("AI"), Some(AgentType::AI));
        assert_eq!(AgentType::from_code("ai"), Some(AgentType::AI));
        assert_eq!(AgentType::from_code("EXTERNAL"), Some(AgentType::External));
        assert_eq!(AgentType::from_code("INTEGRATION"), Some(AgentType::Integration));
        assert_eq!(AgentType::from_code("INVALID"), None);
    }

    #[test]
    fn test_display() {
        assert_eq!(format!("{}", AgentType::System), "SYSTEM");
        assert_eq!(format!("{}", AgentType::AI), "AI");
    }

    #[test]
    fn test_serialization() {
        let agent_type = AgentType::AI;
        let json = serde_json::to_string(&agent_type).unwrap();
        let deserialized: AgentType = serde_json::from_str(&json).unwrap();
        assert_eq!(agent_type, deserialized);
    }

    #[test]
    fn test_equality() {
        assert_eq!(AgentType::AI, AgentType::AI);
        assert_ne!(AgentType::AI, AgentType::System);
    }

    #[test]
    fn test_hash() {
        use std::collections::HashSet;

        let mut set = HashSet::new();
        set.insert(AgentType::AI);
        set.insert(AgentType::System);
        set.insert(AgentType::AI); // Duplicate

        assert_eq!(set.len(), 2);
        assert!(set.contains(&AgentType::AI));
        assert!(set.contains(&AgentType::System));
    }
}
