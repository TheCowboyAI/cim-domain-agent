// Copyright (c) 2025 - Cowboy AI, Inc.

//! Agent Reference Pattern - Implementation Example
//!
//! Demonstrates the correct way to reference agents in NATS subjects
//! based on Frege/Russell/Evans/Searle reference theory.
//!
//! See: AGENT_REFERENCE_THEORY.md

use uuid::Uuid;

// ============================================================================
// Core Types
// ============================================================================

/// Agent unique identifier (rigid designator)
///
/// This is a **Russellian proper name** - it directly refers to THE agent
/// without any descriptive content. Once assigned, it never changes.
///
/// Corresponds to Evans' "dominant causal source" - the AgentDeployedEvent
/// that created this agent is the causal origin of this ID.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AgentId(Uuid);

impl AgentId {
    pub fn new() -> Self {
        Self(Uuid::now_v7())
    }

    pub fn to_string(&self) -> String {
        self.0.to_string()
    }
}

/// Agent name (Fregean sense)
///
/// This is a **definite description** in Russell's terms:
/// "THE agent with orchestrator role" = "sage"
///
/// It provides the **mode of presentation** (Frege) - how humans
/// conceive of the agent's role and capabilities.
///
/// Unlike AgentId, this CAN change (rename operations).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AgentName(String);

impl AgentName {
    pub fn new(name: impl Into<String>) -> Self {
        Self(name.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Capability cluster (Searle's cluster description)
///
/// Agents are identified not by a single property, but by a **cluster**
/// of related properties. The capability cluster is the top-level grouping.
///
/// Examples:
/// - "orchestration" - agents that coordinate workflows
/// - "domain-modeling" - agents that analyze domain boundaries
/// - "event-analysis" - agents that discover domain events
///
/// This corresponds to the agent's position in **conceptual space** (v0.7.0).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CapabilityCluster(String);

impl CapabilityCluster {
    pub fn orchestration() -> Self {
        Self("orchestration".to_string())
    }

    pub fn domain_modeling() -> Self {
        Self("domain-modeling".to_string())
    }

    pub fn event_analysis() -> Self {
        Self("event-analysis".to_string())
    }

    pub fn infrastructure() -> Self {
        Self("infrastructure".to_string())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Operation type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Operation {
    Command,
    Event,
    Query,
    Reply,
}

impl Operation {
    pub fn as_str(&self) -> &str {
        match self {
            Operation::Command => "command",
            Operation::Event => "event",
            Operation::Query => "query",
            Operation::Reply => "reply",
        }
    }
}

// ============================================================================
// Agent Reference - Frege's Sense ⊕ Reference
// ============================================================================

/// Complete agent reference
///
/// This combines:
/// - **Sense** (AgentName) - Fregean mode of presentation
/// - **Reference** (AgentId) - Direct designation (rigid designator)
/// - **Cluster** (CapabilityCluster) - Searle's cluster identification
///
/// Together, these provide:
/// 1. Human readability (name)
/// 2. Denotational precision (ID)
/// 3. Conceptual space membership (cluster)
#[derive(Debug, Clone)]
pub struct AgentReference {
    /// Capability cluster (Searle)
    pub cluster: CapabilityCluster,

    /// Agent name - sense (Frege)
    pub name: AgentName,

    /// Agent ID - reference (Frege/Russell)
    pub id: AgentId,
}

impl AgentReference {
    pub fn new(cluster: CapabilityCluster, name: AgentName, id: AgentId) -> Self {
        Self { cluster, name, id }
    }

    /// Create reference for Sage orchestrator
    pub fn sage(id: AgentId) -> Self {
        Self {
            cluster: CapabilityCluster::orchestration(),
            name: AgentName::new("sage"),
            id,
        }
    }

    /// Create reference for DDD Expert
    pub fn ddd_expert(id: AgentId) -> Self {
        Self {
            cluster: CapabilityCluster::domain_modeling(),
            name: AgentName::new("ddd-expert"),
            id,
        }
    }

    /// Create reference for Event Storming Expert
    pub fn eventstorming_expert(id: AgentId) -> Self {
        Self {
            cluster: CapabilityCluster::event_analysis(),
            name: AgentName::new("eventstorming-expert"),
            id,
        }
    }
}

// ============================================================================
// Subject Factory - Theory-Grounded Subject Generation
// ============================================================================

/// Subject factory implementing the reference theory pattern
///
/// Subject structure:
/// ```
/// agent.{capability-cluster}.{agent-name}.{agent-id}.{operation}.{detail}
/// ```
///
/// This satisfies:
/// - **Frege**: Both sense (name) and reference (ID) present
/// - **Russell**: Existence/uniqueness presuppositions explicit
/// - **Evans**: Causal provenance preserved via ID
/// - **Searle**: Cluster-based identification
pub struct AgentSubjectFactory;

impl AgentSubjectFactory {
    /// Generate command subject
    ///
    /// Example:
    /// ```
    /// agent.orchestration.sage.01936f11-4ea2-7f3e-9f3a-e6c8c6d8a5f1.command.task_analysis
    /// ```
    ///
    /// **Frege Analysis:**
    /// - Sense: "sage" (orchestrator role)
    /// - Reference: "01936f11..." (THE agent)
    /// - Together: "THE sage orchestrator with ID X"
    pub fn command_subject(
        agent: &AgentReference,
        command_type: &str,
    ) -> String {
        format!(
            "agent.{}.{}.{}.command.{}",
            agent.cluster.as_str(),
            agent.name.as_str(),
            agent.id.to_string(),
            command_type
        )
    }

    /// Generate event subject
    ///
    /// Example:
    /// ```
    /// agent.domain-modeling.ddd-expert.01936f12-a3b4-7c5e-8d2f-1a2b3c4d5e6f.event.aggregate_identified
    /// ```
    pub fn event_subject(
        agent: &AgentReference,
        event_type: &str,
    ) -> String {
        format!(
            "agent.{}.{}.{}.event.{}",
            agent.cluster.as_str(),
            agent.name.as_str(),
            agent.id.to_string(),
            event_type
        )
    }

    /// Generate query subject
    pub fn query_subject(
        agent: &AgentReference,
        query_type: &str,
    ) -> String {
        format!(
            "agent.{}.{}.{}.query.{}",
            agent.cluster.as_str(),
            agent.name.as_str(),
            agent.id.to_string(),
            query_type
        )
    }

    /// Generate reply subject
    pub fn reply_subject(
        agent: &AgentReference,
        correlation_id: Uuid,
    ) -> String {
        format!(
            "agent.{}.{}.{}.reply.{}",
            agent.cluster.as_str(),
            agent.name.as_str(),
            agent.id.to_string(),
            correlation_id
        )
    }

    // ========================================================================
    // Subscription Patterns (Wildcards)
    // ========================================================================

    /// Subscribe to all commands for a specific agent (by ID)
    ///
    /// Pattern: `agent.*.*.{agent-id}.command.>`
    ///
    /// **Why this works:**
    /// - ID is rigid designator (Evans) - stable across renames
    /// - Captures all operations for THE agent
    pub fn all_commands_for_agent_id(agent_id: AgentId) -> String {
        format!("agent.*.*.{}.command.>", agent_id.to_string())
    }

    /// Subscribe to all commands for a specific agent (by name)
    ///
    /// Pattern: `agent.*.{agent-name}.*.command.>`
    ///
    /// **Warning:** Fragile! If agent is renamed, this breaks.
    /// Prefer subscribing by ID when possible.
    pub fn all_commands_for_agent_name(agent_name: &str) -> String {
        format!("agent.*.{}.*.command.>", agent_name)
    }

    /// Subscribe to all operations for a capability cluster
    ///
    /// Pattern: `agent.{cluster}.>.*.>`
    ///
    /// **Searle's cluster routing:**
    /// - Routes to all agents in the capability cluster
    /// - Useful for broadcast/multicast patterns
    pub fn all_operations_for_cluster(cluster: &CapabilityCluster) -> String {
        format!("agent.{}.>.*.>", cluster.as_str())
    }

    /// Subscribe to all commands across all agents
    ///
    /// Pattern: `agent.*.*.*.command.>`
    pub fn all_commands() -> String {
        "agent.*.*.*.command.>".to_string()
    }

    /// Subscribe to all events across all agents
    ///
    /// Pattern: `agent.*.*.*.event.>`
    pub fn all_events() -> String {
        "agent.*.*.*.event.>".to_string()
    }
}

// ============================================================================
// Examples: The Right Way to Reference Agents
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sage_reference() {
        let sage_id = AgentId::new();
        let sage = AgentReference::sage(sage_id);

        // Command to Sage: task analysis
        let subject = AgentSubjectFactory::command_subject(&sage, "task_analysis");

        println!("Sage command subject: {}", subject);
        assert!(subject.contains("orchestration"));
        assert!(subject.contains("sage"));
        assert!(subject.contains(&sage_id.to_string()));
        assert!(subject.contains("command"));
        assert!(subject.contains("task_analysis"));
    }

    #[test]
    fn test_ddd_expert_reference() {
        let ddd_id = AgentId::new();
        let ddd = AgentReference::ddd_expert(ddd_id);

        // Event from DDD Expert: aggregate identified
        let subject = AgentSubjectFactory::event_subject(&ddd, "aggregate_identified");

        println!("DDD Expert event subject: {}", subject);
        assert!(subject.contains("domain-modeling"));
        assert!(subject.contains("ddd-expert"));
        assert!(subject.contains(&ddd_id.to_string()));
        assert!(subject.contains("event"));
    }

    #[test]
    fn test_subscription_by_id() {
        // Subscribe to all commands for specific agent (by ID)
        let agent_id = AgentId::new();
        let pattern = AgentSubjectFactory::all_commands_for_agent_id(agent_id);

        println!("Subscribe by ID pattern: {}", pattern);
        assert!(pattern.contains(&agent_id.to_string()));
        assert!(pattern.contains("agent.*.*."));
        assert!(pattern.ends_with(".command.>"));
    }

    #[test]
    fn test_subscription_by_cluster() {
        // Subscribe to all orchestration operations
        let cluster = CapabilityCluster::orchestration();
        let pattern = AgentSubjectFactory::all_operations_for_cluster(&cluster);

        println!("Subscribe by cluster pattern: {}", pattern);
        assert!(pattern.starts_with("agent.orchestration."));
        assert!(pattern.ends_with(".>.*.>"));
    }

    #[test]
    fn test_frege_sense_reference_composition() {
        // Demonstrate Frege's sense + reference composition

        let agent_id = AgentId::new();
        let sage = AgentReference::sage(agent_id);

        // Sense (name): "sage" - how we think about the agent
        println!("Sense (name): {}", sage.name.as_str());

        // Reference (ID): The agent itself
        println!("Reference (ID): {}", sage.id.to_string());

        // Complete denotation: sense + reference
        let subject = AgentSubjectFactory::command_subject(&sage, "orchestrate");
        println!("Complete denotation: {}", subject);

        // Both are present in the subject
        assert!(subject.contains("sage"));  // Sense
        assert!(subject.contains(&agent_id.to_string()));  // Reference
    }

    #[test]
    fn test_evans_causal_provenance() {
        // Demonstrate Evans' causal provenance

        // Causal origin: Agent creation event
        let deployed_at = Uuid::now_v7();  // Event ID
        let agent_id = AgentId::new();     // Agent ID assigned

        println!("AgentDeployedEvent: {}", deployed_at);
        println!("Agent ID (causal result): {}", agent_id.to_string());

        // All future references trace back to this event
        let sage = AgentReference::sage(agent_id);

        let subject1 = AgentSubjectFactory::command_subject(&sage, "task1");
        let subject2 = AgentSubjectFactory::event_subject(&sage, "event1");

        // Both subjects contain the same agent_id (causal provenance)
        assert!(subject1.contains(&agent_id.to_string()));
        assert!(subject2.contains(&agent_id.to_string()));

        println!("Causal chain preserved in subjects:");
        println!("  Deploy event {} -> Agent ID {}", deployed_at, agent_id.to_string());
        println!("  Subject 1: {}", subject1);
        println!("  Subject 2: {}", subject2);
    }

    #[test]
    fn test_russell_definite_description() {
        // Demonstrate Russell's definite description analysis

        let agent_id = AgentId::new();
        let sage = AgentReference::sage(agent_id);

        // "The sage orchestrator" = Definite description
        // Russell: ∃x(Orchestrator(x) ∧ Name(x, "sage") ∧ Id(x, agent_id) ∧
        //            ∀y((Orchestrator(y) ∧ Name(y, "sage")) → y = x))

        let subject = AgentSubjectFactory::command_subject(&sage, "task");

        println!("Russell's analysis of 'the sage orchestrator':");
        println!("  Existence: agent exists (by construction)");
        println!("  Uniqueness: agent_id {} is unique", agent_id.to_string());
        println!("  Predication: orchestrator role (cluster)");
        println!("  Subject: {}", subject);

        // Subject encodes the logical form
        assert!(subject.contains("orchestration"));  // Predication
        assert!(subject.contains("sage"));           // Name
        assert!(subject.contains(&agent_id.to_string()));  // Uniqueness
    }

    #[test]
    fn test_searle_cluster_identification() {
        // Demonstrate Searle's cluster theory

        let agent_id = AgentId::new();

        // Agent identified by CLUSTER of properties:
        let cluster_properties = vec![
            "Has orchestrator role",
            "Coordinates all agents",
            "topology dimension = 1.0",
            "context dimension = 1.0",
            "Deployed on all nodes",
        ];

        println!("Searle cluster for 'sage':");
        for prop in &cluster_properties {
            println!("  - {}", prop);
        }

        // Capability cluster = top-level cluster grouping
        let sage = AgentReference::sage(agent_id);
        let subject = AgentSubjectFactory::command_subject(&sage, "task");

        println!("Cluster routing subject: {}", subject);
        assert!(subject.contains("orchestration"));  // Cluster membership

        // Subscribe to all agents in cluster
        let cluster_pattern = AgentSubjectFactory::all_operations_for_cluster(
            &sage.cluster
        );
        println!("Cluster subscription: {}", cluster_pattern);
    }

    #[test]
    fn test_rename_stability() {
        // Demonstrate ID stability across renames (Evans)

        let agent_id = AgentId::new();

        // Time T1: Agent named "master-coordinator"
        let old_ref = AgentReference::new(
            CapabilityCluster::orchestration(),
            AgentName::new("master-coordinator"),
            agent_id,
        );

        let old_subject = AgentSubjectFactory::command_subject(&old_ref, "task");
        println!("T1 (old name): {}", old_subject);

        // Time T2: Agent renamed to "sage"
        let new_ref = AgentReference::sage(agent_id);
        let new_subject = AgentSubjectFactory::command_subject(&new_ref, "task");
        println!("T2 (new name): {}", new_subject);

        // ID remains the same (Evans: stable reference)
        assert!(old_subject.contains(&agent_id.to_string()));
        assert!(new_subject.contains(&agent_id.to_string()));

        // Names differ (descriptions changed)
        assert!(old_subject.contains("master-coordinator"));
        assert!(new_subject.contains("sage"));

        // Subscription by ID unaffected:
        let id_pattern = AgentSubjectFactory::all_commands_for_agent_id(agent_id);
        println!("ID-based subscription (stable): {}", id_pattern);
    }
}

// ============================================================================
// Summary: Why This Pattern is Correct
// ============================================================================
//
// # Why This Subject Pattern Works
//
// ## Frege: Sense + Reference
// - **Sense** (name): "sage" tells you the ROLE (orchestrator)
// - **Reference** (ID): uuid tells you THE AGENT (specific entity)
// - **Together**: Complete denotation (informative + precise)
//
// ## Russell: Definite Descriptions
// - "The sage orchestrator" = ∃x(Orchestrator(x) ∧ Name(x, "sage") ∧ ...)
// - Subject structure encodes logical form
// - Existence/uniqueness presuppositions explicit
//
// ## Evans: Causal Provenance
// - Agent ID assigned at creation (AgentDeployedEvent)
// - All references trace back to this causal origin
// - ID stable across renames (dominant causal source)
//
// ## Searle: Cluster Identification
// - Capability cluster = conceptual space membership
// - Agents identified by CLUSTER of properties
// - Supports flexible, weighted identification
//
// ## Result: Mathematically Rigorous + Humanly Usable
// - Pattern: `agent.{cluster}.{name}.{id}.{operation}.{detail}`
// - Satisfies all four reference theories
// - Efficient NATS wildcard routing
// - Compositional (monoid structure)
