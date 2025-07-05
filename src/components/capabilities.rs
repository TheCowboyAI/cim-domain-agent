//! Capabilities-related ECS components

use bevy_ecs::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

/// Component representing agent capabilities
#[derive(Component, Debug, Clone)]
pub struct AgentCapabilities {
    /// Set of capability identifiers
    pub capabilities: HashSet<String>,
    /// Capability metadata (descriptions, versions, etc.)
    pub metadata: HashMap<String, serde_json::Value>,
}

impl AgentCapabilities {
    /// Create a new capabilities component
    pub fn new() -> Self {
        Self {
            capabilities: HashSet::new(),
            metadata: HashMap::new(),
        }
    }

    /// Create with initial capabilities
    pub fn with_capabilities(capabilities: Vec<String>) -> Self {
        Self {
            capabilities: capabilities.into_iter().collect(),
            metadata: HashMap::new(),
        }
    }

    /// Add a capability
    pub fn add(&mut self, capability: String) {
        self.capabilities.insert(capability);
    }

    /// Remove a capability
    pub fn remove(&mut self, capability: &str) -> bool {
        self.capabilities.remove(capability)
    }

    /// Check if agent has a capability
    pub fn has(&self, capability: &str) -> bool {
        self.capabilities.contains(capability)
    }
}

impl Default for AgentCapabilities {
    fn default() -> Self {
        Self::new()
    }
}

/// Component for capability requirements
#[derive(Component, Debug, Clone)]
#[derive(Default)]
pub struct CapabilityRequirements {
    /// Required capabilities for the agent to function
    pub required: HashSet<String>,
    /// Optional capabilities that enhance functionality
    pub optional: HashSet<String>,
    /// Capabilities that are incompatible
    pub incompatible: HashSet<String>,
}


/// Component tracking capability usage
#[derive(Component, Debug, Clone)]
#[derive(Default)]
pub struct CapabilityUsageStats {
    /// Usage count per capability
    pub usage_count: HashMap<String, u64>,
    /// Last used timestamp per capability
    pub last_used: HashMap<String, chrono::DateTime<chrono::Utc>>,
    /// Success rate per capability (0.0 - 1.0)
    pub success_rate: HashMap<String, f32>,
}


/// Component for capability categories
#[derive(Component, Debug, Clone)]
#[derive(Default)]
pub struct CapabilityCategories {
    /// Mapping of categories to capabilities
    pub categories: HashMap<String, HashSet<String>>,
}

impl CapabilityCategories {
    /// Get all capabilities in a category
    pub fn get_category(&self, category: &str) -> Option<&HashSet<String>> {
        self.categories.get(category)
    }

    /// Add a capability to a category
    pub fn add_to_category(&mut self, category: String, capability: String) {
        self.categories
            .entry(category)
            .or_default()
            .insert(capability);
    }
}


/// Predefined capability types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum StandardCapability {
    /// Can read data
    Read,
    /// Can write data
    Write,
    /// Can execute operations
    Execute,
    /// Can manage other agents
    ManageAgents,
    /// Can access external APIs
    ExternalAPI,
    /// Can process natural language
    NaturalLanguage,
    /// Can perform computations
    Compute,
    /// Can store data
    Storage,
    /// Can send notifications
    Notify,
    /// Can schedule tasks
    Schedule,
}

impl StandardCapability {
    /// Convert to string identifier
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Read => "capability.read",
            Self::Write => "capability.write",
            Self::Execute => "capability.execute",
            Self::ManageAgents => "capability.manage_agents",
            Self::ExternalAPI => "capability.external_api",
            Self::NaturalLanguage => "capability.natural_language",
            Self::Compute => "capability.compute",
            Self::Storage => "capability.storage",
            Self::Notify => "capability.notify",
            Self::Schedule => "capability.schedule",
        }
    }
} 