//! Agent aggregate (v0.8.1)
//!
//! Pure functional event-sourced aggregate representing an autonomous agent.

use crate::events_new::*;
use crate::value_objects_new::*;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

/// Agent aggregate - Immutable event-sourced representation
///
/// Represents an autonomous mechanical substitute that can:
/// - Execute tasks
/// - Use capabilities (AI, integrations, etc.)
/// - Access tools and external systems
/// - Operate under permissions and constraints
///
/// # Lifecycle
///
/// Deployed → Activated → (Active ↔ Suspended ↔ Offline) → Decommissioned
///
/// # Event Sourcing
///
/// All state changes are represented as immutable events:
/// - No mutable methods
/// - State is reconstructed from event history
/// - Pure functional transformations
///
/// # Examples
///
/// ```
/// use cim_domain_agent::aggregate_new::Agent;
/// use cim_domain_agent::value_objects_new::*;
/// use cim_domain_agent::events_new::*;
/// use uuid::Uuid;
///
/// // Create agent from events
/// let agent_id = AgentId::new();
/// let metadata = AgentMetadata::new(
///     "DataProcessor",
///     "Processes data streams",
///     "1.0.0",
///     Uuid::now_v7()
/// );
///
/// let deployed_event = AgentEvent::AgentDeployed(AgentDeployedEvent::new(
///     agent_id,
///     AgentType::System,
///     metadata,
///     Some("admin".to_string())
/// ));
///
/// let agent = Agent::empty().apply_event(&deployed_event).unwrap();
/// assert_eq!(agent.status(), AgentStatus::Deployed);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Agent {
    /// Agent unique identifier
    id: AgentId,

    /// Agent type (System, AI, External, Integration)
    agent_type: AgentType,

    /// Current operational status
    status: AgentStatus,

    /// Agent metadata (name, description, version, owner)
    metadata: AgentMetadata,

    /// Agent capabilities (what it can do)
    capabilities: HashMap<String, Capability>,

    /// Agent permissions (what it's allowed to do)
    permissions: HashSet<PermissionId>,

    /// Available tools
    tools: HashMap<String, ToolDefinition>,

    /// Runtime configuration
    configuration: AgentConfiguration,

    /// Event sourcing version
    version: u64,
}

impl Agent {
    /// Create an empty agent for event replay
    ///
    /// This is the starting point for reconstructing agent state from events.
    ///
    /// # Examples
    ///
    /// ```
    /// use cim_domain_agent::aggregate_new::Agent;
    ///
    /// let agent = Agent::empty();
    /// assert_eq!(agent.version(), 0);
    /// ```
    pub fn empty() -> Self {
        Self {
            id: AgentId::new(), // Placeholder, will be set by first event
            agent_type: AgentType::System, // Placeholder
            status: AgentStatus::Deployed,
            metadata: AgentMetadata::new("", "", "0.0.0", uuid::Uuid::nil()),
            capabilities: HashMap::new(),
            permissions: HashSet::new(),
            tools: HashMap::new(),
            configuration: AgentConfiguration::new(),
            version: 0,
        }
    }

    /// Create a new agent (initial state)
    ///
    /// This is a convenience method for creating agents in tests or scenarios
    /// where you need an initialized aggregate without events.
    ///
    /// # Arguments
    ///
    /// * `id` - Agent identifier
    /// * `agent_type` - Type of agent
    /// * `metadata` - Agent metadata
    ///
    /// # Examples
    ///
    /// ```
    /// use cim_domain_agent::aggregate_new::Agent;
    /// use cim_domain_agent::value_objects_new::*;
    /// use uuid::Uuid;
    ///
    /// let id = AgentId::new();
    /// let metadata = AgentMetadata::new("Test", "Test agent", "1.0.0", Uuid::now_v7());
    /// let agent = Agent::new(id, AgentType::System, metadata);
    ///
    /// assert_eq!(agent.id(), id);
    /// assert_eq!(agent.agent_type(), AgentType::System);
    /// ```
    pub fn new(id: AgentId, agent_type: AgentType, metadata: AgentMetadata) -> Self {
        Self {
            id,
            agent_type,
            status: AgentStatus::Deployed,
            metadata,
            capabilities: HashMap::new(),
            permissions: HashSet::new(),
            tools: HashMap::new(),
            configuration: AgentConfiguration::new(),
            version: 0,
        }
    }

    // ========================================
    // Getters (Pure, No Side Effects)
    // ========================================

    /// Get the agent ID
    pub fn id(&self) -> AgentId {
        self.id
    }

    /// Get the agent type
    pub fn agent_type(&self) -> AgentType {
        self.agent_type
    }

    /// Get the current status
    pub fn status(&self) -> AgentStatus {
        self.status
    }

    /// Get the metadata
    pub fn metadata(&self) -> &AgentMetadata {
        &self.metadata
    }

    /// Get the capabilities
    pub fn capabilities(&self) -> &HashMap<String, Capability> {
        &self.capabilities
    }

    /// Get a specific capability
    pub fn capability(&self, id: &str) -> Option<&Capability> {
        self.capabilities.get(id)
    }

    /// Check if agent has a capability
    pub fn has_capability(&self, id: &str) -> bool {
        self.capabilities.contains_key(id)
    }

    /// Get the permissions
    pub fn permissions(&self) -> &HashSet<PermissionId> {
        &self.permissions
    }

    /// Check if agent has a permission
    pub fn has_permission(&self, permission_id: &PermissionId) -> bool {
        self.permissions.contains(permission_id)
    }

    /// Get the tools
    pub fn tools(&self) -> &HashMap<String, ToolDefinition> {
        &self.tools
    }

    /// Get a specific tool
    pub fn tool(&self, id: &str) -> Option<&ToolDefinition> {
        self.tools.get(id)
    }

    /// Get the configuration
    pub fn configuration(&self) -> &AgentConfiguration {
        &self.configuration
    }

    /// Get the version
    pub fn version(&self) -> u64 {
        self.version
    }

    // ========================================
    // Status Queries
    // ========================================

    /// Check if agent can execute tasks
    pub fn is_operational(&self) -> bool {
        self.status.can_execute()
    }

    /// Check if agent is in a terminal state
    pub fn is_decommissioned(&self) -> bool {
        self.status.is_terminal()
    }

    // ========================================
    // Pure Event Application (EventSourced Trait)
    // ========================================

    /// Apply an event to produce a new agent state
    ///
    /// This is a pure function - it doesn't mutate state, it returns a new agent.
    ///
    /// # Arguments
    ///
    /// * `event` - The event to apply
    ///
    /// # Returns
    ///
    /// A new `Agent` instance with the event applied, or an error.
    ///
    /// # Examples
    ///
    /// ```
    /// use cim_domain_agent::aggregate_new::Agent;
    /// use cim_domain_agent::events_new::*;
    /// use cim_domain_agent::value_objects_new::*;
    /// use uuid::Uuid;
    ///
    /// let agent = Agent::empty();
    /// let event = AgentEvent::AgentActivated(AgentActivatedEvent::new(
    ///     agent.id(),
    ///     Some("system".to_string())
    /// ));
    ///
    /// let new_agent = agent.apply_event(&event).unwrap();
    /// assert_eq!(new_agent.status(), AgentStatus::Active);
    /// assert_eq!(agent.status(), AgentStatus::Deployed); // Original unchanged
    /// ```
    pub fn apply_event(&self, event: &AgentEvent) -> Result<Self, String> {
        let mut new_agent = self.clone();

        match event {
            AgentEvent::AgentDeployed(e) => {
                new_agent.id = e.agent_id;
                new_agent.agent_type = e.agent_type;
                new_agent.metadata = e.metadata.clone();
                new_agent.status = AgentStatus::Deployed;
            }

            AgentEvent::AgentActivated(_e) => {
                if !new_agent.status.can_activate() {
                    return Err(format!(
                        "Cannot activate agent from status: {}",
                        new_agent.status
                    ));
                }
                new_agent.status = AgentStatus::Active;
            }

            AgentEvent::AgentSuspended(_e) => {
                if !new_agent.status.can_suspend() {
                    return Err(format!(
                        "Cannot suspend agent from status: {}",
                        new_agent.status
                    ));
                }
                new_agent.status = AgentStatus::Suspended;
            }

            AgentEvent::AgentDecommissioned(_e) => {
                if !new_agent.status.can_decommission() {
                    return Err(format!(
                        "Cannot decommission agent from status: {}",
                        new_agent.status
                    ));
                }
                new_agent.status = AgentStatus::Decommissioned;
            }

            AgentEvent::AgentWentOffline(_e) => {
                if !new_agent.status.can_go_offline() {
                    return Err(format!(
                        "Cannot go offline from status: {}",
                        new_agent.status
                    ));
                }
                new_agent.status = AgentStatus::Offline;
            }

            AgentEvent::CapabilitiesUpdated(e) => {
                // Add new capabilities
                for cap in &e.added_capabilities {
                    new_agent.capabilities.insert(
                        cap.id().as_str().to_string(),
                        cap.clone()
                    );
                }

                // Remove capabilities
                for cap_id in &e.removed_capabilities {
                    new_agent.capabilities.remove(cap_id);
                }
            }

            AgentEvent::PermissionsGranted(e) => {
                for perm in &e.permissions {
                    new_agent.permissions.insert(perm.id().clone());
                }
            }

            AgentEvent::PermissionsRevoked(e) => {
                for perm_id in &e.permission_ids {
                    new_agent.permissions.remove(perm_id);
                }
            }

            AgentEvent::ToolsEnabled(e) => {
                for tool in &e.tools {
                    new_agent.tools.insert(
                        tool.id().as_str().to_string(),
                        tool.clone()
                    );
                }
            }

            AgentEvent::ToolsDisabled(e) => {
                for tool_id in &e.tool_ids {
                    new_agent.tools.remove(tool_id.as_str());
                }
            }

            AgentEvent::ConfigurationChanged(e) => {
                for key in &e.changed_keys {
                    if let Some(value) = e.new_values.get(key) {
                        new_agent.configuration.set(key.clone(), value.clone());
                    }
                }
            }
        }

        new_agent.version += 1;
        Ok(new_agent)
    }

    /// Apply multiple events in sequence
    ///
    /// # Examples
    ///
    /// ```
    /// use cim_domain_agent::aggregate_new::Agent;
    /// use cim_domain_agent::events_new::*;
    /// use cim_domain_agent::value_objects_new::*;
    /// use uuid::Uuid;
    ///
    /// let events = vec![
    ///     AgentEvent::AgentDeployed(AgentDeployedEvent::new(
    ///         AgentId::new(),
    ///         AgentType::System,
    ///         AgentMetadata::new("Test", "Test", "1.0.0", Uuid::now_v7()),
    ///         None
    ///     )),
    ///     AgentEvent::AgentActivated(AgentActivatedEvent::new(
    ///         AgentId::new(),
    ///         None
    ///     )),
    /// ];
    ///
    /// let agent = Agent::empty().apply_events(&events).unwrap();
    /// assert_eq!(agent.status(), AgentStatus::Active);
    /// assert_eq!(agent.version(), 2);
    /// ```
    pub fn apply_events(&self, events: &[AgentEvent]) -> Result<Self, String> {
        let mut agent = self.clone();
        for event in events {
            agent = agent.apply_event(event)?;
        }
        Ok(agent)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    #[test]
    fn test_empty_agent() {
        let agent = Agent::empty();
        assert_eq!(agent.version(), 0);
        assert_eq!(agent.status(), AgentStatus::Deployed);
    }

    #[test]
    fn test_new_agent() {
        let id = AgentId::new();
        let metadata = AgentMetadata::new("Test", "Test agent", "1.0.0", Uuid::now_v7());
        let agent = Agent::new(id, AgentType::AI, metadata);

        assert_eq!(agent.id(), id);
        assert_eq!(agent.agent_type(), AgentType::AI);
        assert_eq!(agent.status(), AgentStatus::Deployed);
    }

    #[test]
    fn test_apply_deployed_event() {
        let agent = Agent::empty();
        let agent_id = AgentId::new();
        let metadata = AgentMetadata::new("Test", "Test", "1.0.0", Uuid::now_v7());

        let event = AgentEvent::AgentDeployed(AgentDeployedEvent::new(
            agent_id,
            AgentType::System,
            metadata.clone(),
            None,
        ));

        let new_agent = agent.apply_event(&event).unwrap();
        assert_eq!(new_agent.id(), agent_id);
        assert_eq!(new_agent.agent_type(), AgentType::System);
        assert_eq!(new_agent.version(), 1);
    }

    #[test]
    fn test_apply_activated_event() {
        let id = AgentId::new();
        let metadata = AgentMetadata::new("Test", "Test", "1.0.0", Uuid::now_v7());
        let agent = Agent::new(id, AgentType::System, metadata);

        let event = AgentEvent::AgentActivated(AgentActivatedEvent::new(id, None));
        let new_agent = agent.apply_event(&event).unwrap();

        assert_eq!(new_agent.status(), AgentStatus::Active);
        assert_eq!(new_agent.version(), 1);
    }

    #[test]
    fn test_lifecycle_state_machine() {
        let id = AgentId::new();
        let metadata = AgentMetadata::new("Test", "Test", "1.0.0", Uuid::now_v7());
        let mut agent = Agent::new(id, AgentType::System, metadata);

        // Deployed -> Active
        let event = AgentEvent::AgentActivated(AgentActivatedEvent::new(id, None));
        agent = agent.apply_event(&event).unwrap();
        assert_eq!(agent.status(), AgentStatus::Active);

        // Active -> Suspended
        let event = AgentEvent::AgentSuspended(AgentSuspendedEvent::new(
            id,
            "Maintenance".to_string(),
            None,
        ));
        agent = agent.apply_event(&event).unwrap();
        assert_eq!(agent.status(), AgentStatus::Suspended);

        // Suspended -> Active
        let event = AgentEvent::AgentActivated(AgentActivatedEvent::new(id, None));
        agent = agent.apply_event(&event).unwrap();
        assert_eq!(agent.status(), AgentStatus::Active);

        // Active -> Offline
        let event = AgentEvent::AgentWentOffline(AgentWentOfflineEvent::new(id, None));
        agent = agent.apply_event(&event).unwrap();
        assert_eq!(agent.status(), AgentStatus::Offline);

        // Offline -> Active
        let event = AgentEvent::AgentActivated(AgentActivatedEvent::new(id, None));
        agent = agent.apply_event(&event).unwrap();
        assert_eq!(agent.status(), AgentStatus::Active);

        // Active -> Decommissioned (terminal)
        let event = AgentEvent::AgentDecommissioned(AgentDecommissionedEvent::new(
            id,
            Some("End of life".to_string()),
            None,
        ));
        agent = agent.apply_event(&event).unwrap();
        assert_eq!(agent.status(), AgentStatus::Decommissioned);
        assert!(agent.is_decommissioned());

        // Cannot activate from decommissioned
        let event = AgentEvent::AgentActivated(AgentActivatedEvent::new(id, None));
        assert!(agent.apply_event(&event).is_err());
    }

    #[test]
    fn test_apply_multiple_events() {
        let id = AgentId::new();
        let metadata = AgentMetadata::new("Test", "Test", "1.0.0", Uuid::now_v7());

        let events = vec![
            AgentEvent::AgentDeployed(AgentDeployedEvent::new(
                id,
                AgentType::AI,
                metadata,
                None,
            )),
            AgentEvent::AgentActivated(AgentActivatedEvent::new(id, None)),
        ];

        let agent = Agent::empty().apply_events(&events).unwrap();
        assert_eq!(agent.status(), AgentStatus::Active);
        assert_eq!(agent.version(), 2);
    }

    #[test]
    fn test_immutability() {
        let id = AgentId::new();
        let metadata = AgentMetadata::new("Test", "Test", "1.0.0", Uuid::now_v7());
        let agent = Agent::new(id, AgentType::System, metadata);

        let event = AgentEvent::AgentActivated(AgentActivatedEvent::new(id, None));
        let new_agent = agent.apply_event(&event).unwrap();

        // Original agent unchanged
        assert_eq!(agent.status(), AgentStatus::Deployed);
        assert_eq!(agent.version(), 0);

        // New agent has changes
        assert_eq!(new_agent.status(), AgentStatus::Active);
        assert_eq!(new_agent.version(), 1);
    }
}
