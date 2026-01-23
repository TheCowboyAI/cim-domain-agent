// Copyright (c) 2025 - Cowboy AI, LLC.

//! AgentReference type for complete agent identification
//!
//! Represents a complete reference to an agent consisting of:
//! - CapabilityCluster (conceptual space)
//! - Agent name (human-readable description)
//! - AgentId (unique identifier)
//!
//! ## Design Rationale
//!
//! From Description Expert analysis (Frege/Russell/Evans/Searle):
//! - **Frege**: Sense (name) + Reference (ID) both present
//! - **Russell**: Logical form encoded (existence/uniqueness via UUID)
//! - **Evans**: Causal provenance preserved (ID traces to AgentDeployedEvent)
//! - **Searle**: Capability clusters define conceptual spaces
//!
//! ## Usage
//!
//! ```ignore
//! use cim_domain_agent::value_objects::{AgentReference, CapabilityCluster, AgentId};
//!
//! let sage_ref = AgentReference {
//!     capability: CapabilityCluster::Orchestration,
//!     name: "sage".to_string(),
//!     id: AgentId::new(),
//! };
//!
//! // For NATS headers
//! let header_value = sage_ref.to_header_value();
//! // "orchestration.sage.01936f11-4ea2-7f3e-9f3a-e6c8c6d8a5f1"
//! ```

use crate::value_objects::{AgentId, CapabilityCluster};
use serde::{Deserialize, Serialize};
use std::fmt;

/// Complete agent reference with capability, name, and ID
///
/// This type provides complete agent provenance in NATS headers,
/// enabling:
/// - Stable routing across renames (ID-based)
/// - Human-readable identification (name)
/// - Semantic organization (capability cluster)
///
/// # NATS Header Format
///
/// Format: `{capability-cluster}.{agent-name}.{agent-id}`
///
/// Example: `orchestration.sage.01936f11-4ea2-7f3e-9f3a-e6c8c6d8a5f1`
///
/// # Subject Patterns
///
/// Command: `agent.{capability}.{name}.{id}.command.{type}`
/// Event: `agent.{capability}.{name}.{id}.event.{type}`
///
/// Headers: Sender/Recipient use AgentReference format
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct AgentReference {
    /// Capability cluster (conceptual space)
    pub capability: CapabilityCluster,

    /// Agent name (human-readable description)
    pub name: String,

    /// Unique agent identifier (stable reference)
    pub id: AgentId,
}

impl AgentReference {
    /// Create a new agent reference
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use cim_domain_agent::value_objects::{AgentReference, CapabilityCluster, AgentId};
    ///
    /// let sage = AgentReference::new(
    ///     CapabilityCluster::Orchestration,
    ///     "sage".to_string(),
    ///     AgentId::new(),
    /// );
    /// ```
    pub fn new(capability: CapabilityCluster, name: String, id: AgentId) -> Self {
        Self {
            capability,
            name,
            id,
        }
    }

    /// Create from agent name and ID, inferring capability cluster
    ///
    /// Returns None if the agent name is not recognized.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use cim_domain_agent::value_objects::{AgentReference, AgentId};
    ///
    /// let sage = AgentReference::from_name_and_id("sage".to_string(), AgentId::new());
    /// assert!(sage.is_some());
    ///
    /// let unknown = AgentReference::from_name_and_id("unknown".to_string(), AgentId::new());
    /// assert!(unknown.is_none());
    /// ```
    pub fn from_name_and_id(name: String, id: AgentId) -> Option<Self> {
        let capability = CapabilityCluster::from_agent_name(&name)?;
        Some(Self {
            capability,
            name,
            id,
        })
    }

    /// Convert to NATS header value format
    ///
    /// Format: `{capability-cluster}.{agent-name}.{agent-id}`
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use cim_domain_agent::value_objects::{AgentReference, CapabilityCluster, AgentId};
    ///
    /// let sage = AgentReference::new(
    ///     CapabilityCluster::Orchestration,
    ///     "sage".to_string(),
    ///     AgentId::new(),
    /// );
    ///
    /// let header = sage.to_header_value();
    /// // "orchestration.sage.01936f11-4ea2-7f3e-9f3a-e6c8c6d8a5f1"
    /// ```
    pub fn to_header_value(&self) -> String {
        format!("{}.{}.{}", self.capability, self.name, self.id)
    }

    /// Parse from NATS header value format
    ///
    /// Format: `{capability-cluster}.{agent-name}.{agent-id}`
    ///
    /// Returns None if the format is invalid or the capability cluster is not recognized.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use cim_domain_agent::value_objects::AgentReference;
    ///
    /// let header = "orchestration.sage.01936f11-4ea2-7f3e-9f3a-e6c8c6d8a5f1";
    /// let agent_ref = AgentReference::from_header_value(header).unwrap();
    ///
    /// assert_eq!(agent_ref.name, "sage");
    /// ```
    pub fn from_header_value(value: &str) -> Option<Self> {
        let parts: Vec<&str> = value.splitn(3, '.').collect();
        if parts.len() != 3 {
            return None;
        }

        let capability = CapabilityCluster::from_str(parts[0])?;
        let name = parts[1].to_string();
        let uuid = uuid::Uuid::parse_str(parts[2]).ok()?;
        let id = AgentId::from_uuid(uuid);

        Some(Self {
            capability,
            name,
            id,
        })
    }

    /// Get the capability cluster
    pub fn capability(&self) -> CapabilityCluster {
        self.capability
    }

    /// Get the agent name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get the agent ID
    pub fn id(&self) -> AgentId {
        self.id
    }
}

impl fmt::Display for AgentReference {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_header_value())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_agent_reference_creation() {
        let agent_id = AgentId::new();
        let agent_ref = AgentReference::new(
            CapabilityCluster::Orchestration,
            "sage".to_string(),
            agent_id,
        );

        assert_eq!(agent_ref.capability, CapabilityCluster::Orchestration);
        assert_eq!(agent_ref.name, "sage");
        assert_eq!(agent_ref.id, agent_id);
    }

    #[test]
    fn test_agent_reference_from_name_and_id() {
        let agent_id = AgentId::new();

        // Known agent
        let sage = AgentReference::from_name_and_id("sage".to_string(), agent_id);
        assert!(sage.is_some());
        let sage = sage.unwrap();
        assert_eq!(sage.capability, CapabilityCluster::Orchestration);
        assert_eq!(sage.name, "sage");

        // Unknown agent
        let unknown = AgentReference::from_name_and_id("unknown-agent".to_string(), agent_id);
        assert!(unknown.is_none());
    }

    #[test]
    fn test_agent_reference_header_value() {
        let agent_id = AgentId::new();
        let agent_ref = AgentReference::new(
            CapabilityCluster::Orchestration,
            "sage".to_string(),
            agent_id,
        );

        let header = agent_ref.to_header_value();

        // Should be in format: capability.name.id
        // UUID contains hyphens but splitn(3) only splits into 3 parts
        assert!(header.starts_with("orchestration.sage."));
        assert!(header.contains(&agent_id.to_string()));
    }

    #[test]
    fn test_agent_reference_from_header_value() {
        let agent_id = AgentId::new();
        let original = AgentReference::new(
            CapabilityCluster::Orchestration,
            "sage".to_string(),
            agent_id,
        );

        let header = original.to_header_value();
        let parsed = AgentReference::from_header_value(&header).unwrap();

        assert_eq!(parsed.capability, original.capability);
        assert_eq!(parsed.name, original.name);
        assert_eq!(parsed.id, original.id);
    }

    #[test]
    fn test_agent_reference_from_header_value_invalid() {
        // Too few parts
        assert!(AgentReference::from_header_value("orchestration.sage").is_none());

        // Invalid capability
        assert!(AgentReference::from_header_value("invalid.sage.01936f11-4ea2-7f3e-9f3a-e6c8c6d8a5f1").is_none());

        // Invalid UUID
        assert!(AgentReference::from_header_value("orchestration.sage.invalid-uuid").is_none());
    }

    #[test]
    fn test_agent_reference_display() {
        let agent_id = AgentId::new();
        let agent_ref = AgentReference::new(
            CapabilityCluster::Orchestration,
            "sage".to_string(),
            agent_id,
        );

        let display = agent_ref.to_string();
        assert_eq!(display, agent_ref.to_header_value());
    }

    #[test]
    fn test_agent_reference_getters() {
        let agent_id = AgentId::new();
        let agent_ref = AgentReference::new(
            CapabilityCluster::DomainModeling,
            "ddd-expert".to_string(),
            agent_id,
        );

        assert_eq!(agent_ref.capability(), CapabilityCluster::DomainModeling);
        assert_eq!(agent_ref.name(), "ddd-expert");
        assert_eq!(agent_ref.id(), agent_id);
    }

    #[test]
    fn test_agent_reference_equality() {
        let agent_id = AgentId::new();
        let agent_ref1 = AgentReference::new(
            CapabilityCluster::Orchestration,
            "sage".to_string(),
            agent_id,
        );
        let agent_ref2 = AgentReference::new(
            CapabilityCluster::Orchestration,
            "sage".to_string(),
            agent_id,
        );

        assert_eq!(agent_ref1, agent_ref2);
    }

    #[test]
    fn test_agent_reference_hash() {
        use std::collections::HashSet;

        let agent_id1 = AgentId::new();
        let agent_id2 = AgentId::new();

        let sage = AgentReference::new(
            CapabilityCluster::Orchestration,
            "sage".to_string(),
            agent_id1,
        );
        let ddd = AgentReference::new(
            CapabilityCluster::DomainModeling,
            "ddd-expert".to_string(),
            agent_id2,
        );

        let mut set = HashSet::new();
        set.insert(sage.clone());
        set.insert(ddd.clone());

        assert_eq!(set.len(), 2);
        assert!(set.contains(&sage));
        assert!(set.contains(&ddd));
    }

    #[test]
    fn test_agent_reference_serialization() {
        let agent_id = AgentId::new();
        let agent_ref = AgentReference::new(
            CapabilityCluster::Orchestration,
            "sage".to_string(),
            agent_id,
        );

        // Serialize to JSON
        let json = serde_json::to_string(&agent_ref).expect("should serialize");

        // Deserialize back
        let deserialized: AgentReference =
            serde_json::from_str(&json).expect("should deserialize");

        assert_eq!(agent_ref, deserialized);
    }

    #[test]
    fn test_agent_reference_clone() {
        let agent_id = AgentId::new();
        let agent_ref = AgentReference::new(
            CapabilityCluster::Orchestration,
            "sage".to_string(),
            agent_id,
        );

        let cloned = agent_ref.clone();
        assert_eq!(agent_ref, cloned);
    }

    #[test]
    fn test_all_known_agents_roundtrip() {
        let agents = vec![
            ("sage", CapabilityCluster::Orchestration),
            ("ddd-expert", CapabilityCluster::DomainModeling),
            ("nats-expert", CapabilityCluster::Infrastructure),
            ("qa-expert", CapabilityCluster::QualityAssurance),
            ("fp-expert", CapabilityCluster::FunctionalProgramming),
            ("iced-ui-expert", CapabilityCluster::UiDesign),
            ("git-expert", CapabilityCluster::Sdlc),
            ("graph-expert", CapabilityCluster::ConceptualAnalysis),
            ("people-expert", CapabilityCluster::DomainEntities),
        ];

        for (name, expected_capability) in agents {
            let agent_id = AgentId::new();
            let agent_ref = AgentReference::from_name_and_id(name.to_string(), agent_id);

            assert!(agent_ref.is_some(), "Agent {} should be recognized", name);

            let agent_ref = agent_ref.unwrap();
            assert_eq!(agent_ref.capability, expected_capability);

            // Roundtrip through header value
            let header = agent_ref.to_header_value();
            let parsed = AgentReference::from_header_value(&header);

            assert!(parsed.is_some(), "Should parse header for {}", name);
            let parsed = parsed.unwrap();

            assert_eq!(parsed.capability, expected_capability);
            assert_eq!(parsed.name, name);
            assert_eq!(parsed.id, agent_id);
        }
    }
}
