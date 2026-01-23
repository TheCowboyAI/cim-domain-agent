// Copyright (c) 2025 - Cowboy AI, LLC.

//! CapabilityCluster enum for organizing agents by capability
//!
//! Represents the semantic grouping of agents based on their primary capabilities.
//! This is a key component of the unified subject architecture, providing hierarchical
//! routing and conceptual organization based on Searle's cluster theory.
//!
//! ## Design Rationale
//!
//! From Description Expert analysis (Searle v0.7.0):
//! - Capability clusters define conceptual spaces
//! - Names co-refer within clusters (Searle's cluster theory)
//! - Enables both individual and broadcast routing
//!
//! ## Usage
//!
//! ```
//! use cim_domain_agent::value_objects::CapabilityCluster;
//!
//! // Sage is in orchestration cluster
//! let cluster = CapabilityCluster::Orchestration;
//! println!("Cluster: {}", cluster); // "orchestration"
//!
//! // Subject pattern: agent.orchestration.sage.{id}.command.deploy
//! ```

use serde::{Deserialize, Serialize};
use std::fmt;

/// Capability clusters organizing agents by semantic capability
///
/// Each cluster represents a conceptual space of related agent capabilities.
/// This enables hierarchical routing patterns in NATS:
///
/// - Individual: `agent.{cluster}.{name}.{id}.>`
/// - Cluster-wide: `agent.{cluster}.*.*.>`
/// - All agents: `agent.*.*.*.>`
///
/// # Clusters and Their Agents
///
/// - **Orchestration**: sage
/// - **DomainModeling**: ddd-expert, domain-expert, domain-ontologist-researcher
/// - **EventAnalysis**: event-storming-expert
/// - **Infrastructure**: nats-expert, nix-expert, network-expert
/// - **QualityAssurance**: qa-expert, tdd-expert, bdd-expert
/// - **FunctionalProgramming**: fp-expert, frp-expert, act-expert
/// - **UiDesign**: egui-ui-expert, iced-ui-expert, cim-ui-layer-expert, cim-tea-ecs-expert
/// - **Sdlc**: git-expert, sdlc-expert, sdlc-distributed-expert
/// - **ConceptualAnalysis**: language-expert, graph-expert, conceptual-spaces-expert, description-expert, subject-expert
/// - **DomainEntities**: people-expert, org-expert, location-expert
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CapabilityCluster {
    /// Master orchestration and coordination (sage)
    Orchestration,

    /// Domain-driven design and modeling
    DomainModeling,

    /// Event storming and event analysis
    EventAnalysis,

    /// Infrastructure (NATS, Nix, Network)
    Infrastructure,

    /// Quality assurance and testing
    QualityAssurance,

    /// Functional and reactive programming
    FunctionalProgramming,

    /// User interface design and implementation
    UiDesign,

    /// Software development lifecycle
    Sdlc,

    /// Conceptual analysis and theory
    ConceptualAnalysis,

    /// Domain entities (People, Organizations, Locations)
    DomainEntities,
}

impl CapabilityCluster {
    /// Get the kebab-case string representation for NATS subjects
    ///
    /// # Examples
    ///
    /// ```
    /// use cim_domain_agent::value_objects::CapabilityCluster;
    ///
    /// assert_eq!(CapabilityCluster::Orchestration.as_str(), "orchestration");
    /// assert_eq!(CapabilityCluster::DomainModeling.as_str(), "domain-modeling");
    /// ```
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Orchestration => "orchestration",
            Self::DomainModeling => "domain-modeling",
            Self::EventAnalysis => "event-analysis",
            Self::Infrastructure => "infrastructure",
            Self::QualityAssurance => "quality-assurance",
            Self::FunctionalProgramming => "functional-programming",
            Self::UiDesign => "ui-design",
            Self::Sdlc => "sdlc",
            Self::ConceptualAnalysis => "conceptual-analysis",
            Self::DomainEntities => "domain-entities",
        }
    }

    /// Parse from kebab-case string
    ///
    /// # Examples
    ///
    /// ```
    /// use cim_domain_agent::value_objects::CapabilityCluster;
    ///
    /// let cluster = CapabilityCluster::from_str("orchestration").unwrap();
    /// assert_eq!(cluster, CapabilityCluster::Orchestration);
    ///
    /// assert!(CapabilityCluster::from_str("invalid").is_none());
    /// ```
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "orchestration" => Some(Self::Orchestration),
            "domain-modeling" => Some(Self::DomainModeling),
            "event-analysis" => Some(Self::EventAnalysis),
            "infrastructure" => Some(Self::Infrastructure),
            "quality-assurance" => Some(Self::QualityAssurance),
            "functional-programming" => Some(Self::FunctionalProgramming),
            "ui-design" => Some(Self::UiDesign),
            "sdlc" => Some(Self::Sdlc),
            "conceptual-analysis" => Some(Self::ConceptualAnalysis),
            "domain-entities" => Some(Self::DomainEntities),
            _ => None,
        }
    }

    /// Infer capability cluster from agent name
    ///
    /// This provides a convenient way to determine the cluster for any of the 31 agents.
    ///
    /// # Examples
    ///
    /// ```
    /// use cim_domain_agent::value_objects::CapabilityCluster;
    ///
    /// assert_eq!(
    ///     CapabilityCluster::from_agent_name("sage"),
    ///     Some(CapabilityCluster::Orchestration)
    /// );
    /// assert_eq!(
    ///     CapabilityCluster::from_agent_name("ddd-expert"),
    ///     Some(CapabilityCluster::DomainModeling)
    /// );
    /// ```
    pub fn from_agent_name(name: &str) -> Option<Self> {
        match name {
            // Orchestration
            "sage" => Some(Self::Orchestration),

            // Domain Modeling
            "ddd-expert" | "domain-expert" | "domain-ontologist-researcher" => {
                Some(Self::DomainModeling)
            }

            // Event Analysis
            "event-storming-expert" => Some(Self::EventAnalysis),

            // Infrastructure
            "nats-expert" | "nix-expert" | "network-expert" => Some(Self::Infrastructure),

            // Quality Assurance
            "qa-expert" | "tdd-expert" | "bdd-expert" => Some(Self::QualityAssurance),

            // Functional Programming
            "fp-expert" | "frp-expert" | "act-expert" => Some(Self::FunctionalProgramming),

            // UI Design
            "egui-ui-expert" | "iced-ui-expert" | "cim-ui-layer-expert"
            | "cim-tea-ecs-expert" => Some(Self::UiDesign),

            // SDLC
            "git-expert" | "sdlc-expert" | "sdlc-distributed-expert" => Some(Self::Sdlc),

            // Conceptual Analysis
            "language-expert" | "graph-expert" | "conceptual-spaces-expert"
            | "description-expert" | "subject-expert" => Some(Self::ConceptualAnalysis),

            // Domain Entities
            "people-expert" | "org-expert" | "location-expert" => Some(Self::DomainEntities),

            _ => None,
        }
    }

    /// Get all capability clusters
    ///
    /// # Examples
    ///
    /// ```
    /// use cim_domain_agent::value_objects::CapabilityCluster;
    ///
    /// let clusters = CapabilityCluster::all();
    /// assert_eq!(clusters.len(), 10);
    /// ```
    pub fn all() -> Vec<Self> {
        vec![
            Self::Orchestration,
            Self::DomainModeling,
            Self::EventAnalysis,
            Self::Infrastructure,
            Self::QualityAssurance,
            Self::FunctionalProgramming,
            Self::UiDesign,
            Self::Sdlc,
            Self::ConceptualAnalysis,
            Self::DomainEntities,
        ]
    }
}

impl fmt::Display for CapabilityCluster {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_capability_cluster_display() {
        assert_eq!(CapabilityCluster::Orchestration.to_string(), "orchestration");
        assert_eq!(
            CapabilityCluster::DomainModeling.to_string(),
            "domain-modeling"
        );
        assert_eq!(
            CapabilityCluster::EventAnalysis.to_string(),
            "event-analysis"
        );
        assert_eq!(
            CapabilityCluster::Infrastructure.to_string(),
            "infrastructure"
        );
        assert_eq!(
            CapabilityCluster::QualityAssurance.to_string(),
            "quality-assurance"
        );
        assert_eq!(
            CapabilityCluster::FunctionalProgramming.to_string(),
            "functional-programming"
        );
        assert_eq!(CapabilityCluster::UiDesign.to_string(), "ui-design");
        assert_eq!(CapabilityCluster::Sdlc.to_string(), "sdlc");
        assert_eq!(
            CapabilityCluster::ConceptualAnalysis.to_string(),
            "conceptual-analysis"
        );
        assert_eq!(
            CapabilityCluster::DomainEntities.to_string(),
            "domain-entities"
        );
    }

    #[test]
    fn test_capability_cluster_from_str() {
        assert_eq!(
            CapabilityCluster::from_str("orchestration"),
            Some(CapabilityCluster::Orchestration)
        );
        assert_eq!(
            CapabilityCluster::from_str("domain-modeling"),
            Some(CapabilityCluster::DomainModeling)
        );
        assert_eq!(
            CapabilityCluster::from_str("event-analysis"),
            Some(CapabilityCluster::EventAnalysis)
        );
        assert_eq!(CapabilityCluster::from_str("invalid"), None);
    }

    #[test]
    fn test_capability_cluster_roundtrip() {
        for cluster in CapabilityCluster::all() {
            let s = cluster.to_string();
            let parsed = CapabilityCluster::from_str(&s);
            assert_eq!(parsed, Some(cluster));
        }
    }

    #[test]
    fn test_capability_cluster_from_agent_name() {
        // Orchestration
        assert_eq!(
            CapabilityCluster::from_agent_name("sage"),
            Some(CapabilityCluster::Orchestration)
        );

        // Domain Modeling
        assert_eq!(
            CapabilityCluster::from_agent_name("ddd-expert"),
            Some(CapabilityCluster::DomainModeling)
        );
        assert_eq!(
            CapabilityCluster::from_agent_name("domain-expert"),
            Some(CapabilityCluster::DomainModeling)
        );
        assert_eq!(
            CapabilityCluster::from_agent_name("domain-ontologist-researcher"),
            Some(CapabilityCluster::DomainModeling)
        );

        // Event Analysis
        assert_eq!(
            CapabilityCluster::from_agent_name("event-storming-expert"),
            Some(CapabilityCluster::EventAnalysis)
        );

        // Infrastructure
        assert_eq!(
            CapabilityCluster::from_agent_name("nats-expert"),
            Some(CapabilityCluster::Infrastructure)
        );
        assert_eq!(
            CapabilityCluster::from_agent_name("nix-expert"),
            Some(CapabilityCluster::Infrastructure)
        );
        assert_eq!(
            CapabilityCluster::from_agent_name("network-expert"),
            Some(CapabilityCluster::Infrastructure)
        );

        // Quality Assurance
        assert_eq!(
            CapabilityCluster::from_agent_name("qa-expert"),
            Some(CapabilityCluster::QualityAssurance)
        );
        assert_eq!(
            CapabilityCluster::from_agent_name("tdd-expert"),
            Some(CapabilityCluster::QualityAssurance)
        );
        assert_eq!(
            CapabilityCluster::from_agent_name("bdd-expert"),
            Some(CapabilityCluster::QualityAssurance)
        );

        // Functional Programming
        assert_eq!(
            CapabilityCluster::from_agent_name("fp-expert"),
            Some(CapabilityCluster::FunctionalProgramming)
        );
        assert_eq!(
            CapabilityCluster::from_agent_name("frp-expert"),
            Some(CapabilityCluster::FunctionalProgramming)
        );
        assert_eq!(
            CapabilityCluster::from_agent_name("act-expert"),
            Some(CapabilityCluster::FunctionalProgramming)
        );

        // UI Design
        assert_eq!(
            CapabilityCluster::from_agent_name("egui-ui-expert"),
            Some(CapabilityCluster::UiDesign)
        );
        assert_eq!(
            CapabilityCluster::from_agent_name("iced-ui-expert"),
            Some(CapabilityCluster::UiDesign)
        );
        assert_eq!(
            CapabilityCluster::from_agent_name("cim-ui-layer-expert"),
            Some(CapabilityCluster::UiDesign)
        );
        assert_eq!(
            CapabilityCluster::from_agent_name("cim-tea-ecs-expert"),
            Some(CapabilityCluster::UiDesign)
        );

        // SDLC
        assert_eq!(
            CapabilityCluster::from_agent_name("git-expert"),
            Some(CapabilityCluster::Sdlc)
        );
        assert_eq!(
            CapabilityCluster::from_agent_name("sdlc-expert"),
            Some(CapabilityCluster::Sdlc)
        );
        assert_eq!(
            CapabilityCluster::from_agent_name("sdlc-distributed-expert"),
            Some(CapabilityCluster::Sdlc)
        );

        // Conceptual Analysis
        assert_eq!(
            CapabilityCluster::from_agent_name("language-expert"),
            Some(CapabilityCluster::ConceptualAnalysis)
        );
        assert_eq!(
            CapabilityCluster::from_agent_name("graph-expert"),
            Some(CapabilityCluster::ConceptualAnalysis)
        );
        assert_eq!(
            CapabilityCluster::from_agent_name("conceptual-spaces-expert"),
            Some(CapabilityCluster::ConceptualAnalysis)
        );
        assert_eq!(
            CapabilityCluster::from_agent_name("description-expert"),
            Some(CapabilityCluster::ConceptualAnalysis)
        );
        assert_eq!(
            CapabilityCluster::from_agent_name("subject-expert"),
            Some(CapabilityCluster::ConceptualAnalysis)
        );

        // Domain Entities
        assert_eq!(
            CapabilityCluster::from_agent_name("people-expert"),
            Some(CapabilityCluster::DomainEntities)
        );
        assert_eq!(
            CapabilityCluster::from_agent_name("org-expert"),
            Some(CapabilityCluster::DomainEntities)
        );
        assert_eq!(
            CapabilityCluster::from_agent_name("location-expert"),
            Some(CapabilityCluster::DomainEntities)
        );

        // Unknown
        assert_eq!(CapabilityCluster::from_agent_name("unknown-agent"), None);
    }

    #[test]
    fn test_capability_cluster_serialization() {
        let cluster = CapabilityCluster::Orchestration;

        // Serialize to JSON
        let json = serde_json::to_string(&cluster).expect("should serialize");

        // Deserialize back
        let deserialized: CapabilityCluster =
            serde_json::from_str(&json).expect("should deserialize");

        assert_eq!(cluster, deserialized);
    }

    #[test]
    fn test_capability_cluster_copy_semantics() {
        let cluster = CapabilityCluster::Orchestration;
        let cluster_copy = cluster; // Copy

        assert_eq!(cluster, cluster_copy);
    }

    #[test]
    fn test_capability_cluster_hash() {
        use std::collections::HashSet;

        let mut set = HashSet::new();
        set.insert(CapabilityCluster::Orchestration);
        set.insert(CapabilityCluster::DomainModeling);

        assert_eq!(set.len(), 2);
        assert!(set.contains(&CapabilityCluster::Orchestration));
        assert!(set.contains(&CapabilityCluster::DomainModeling));
    }

    #[test]
    fn test_capability_cluster_all() {
        let clusters = CapabilityCluster::all();
        assert_eq!(clusters.len(), 10);

        // Verify each cluster is unique
        use std::collections::HashSet;
        let set: HashSet<_> = clusters.into_iter().collect();
        assert_eq!(set.len(), 10);
    }

    #[test]
    fn test_all_31_agents_have_cluster() {
        let agents = vec![
            "sage",
            "ddd-expert",
            "domain-expert",
            "domain-ontologist-researcher",
            "event-storming-expert",
            "nats-expert",
            "nix-expert",
            "network-expert",
            "qa-expert",
            "tdd-expert",
            "bdd-expert",
            "fp-expert",
            "frp-expert",
            "act-expert",
            "egui-ui-expert",
            "iced-ui-expert",
            "cim-ui-layer-expert",
            "cim-tea-ecs-expert",
            "git-expert",
            "sdlc-expert",
            "sdlc-distributed-expert",
            "language-expert",
            "graph-expert",
            "conceptual-spaces-expert",
            "description-expert",
            "subject-expert",
            "people-expert",
            "org-expert",
            "location-expert",
        ];

        assert_eq!(agents.len(), 29, "Should have 29 agents listed");

        for agent in agents {
            assert!(
                CapabilityCluster::from_agent_name(agent).is_some(),
                "Agent {} should have a capability cluster",
                agent
            );
        }
    }
}
