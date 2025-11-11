//! Agent metadata value object
//!
//! Core information about an agent (name, description, version, owner).

use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use uuid::Uuid;

/// Agent metadata
///
/// Immutable descriptive information about an agent.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AgentMetadata {
    /// Human-readable name
    name: String,

    /// Detailed description of the agent's purpose
    description: String,

    /// Semantic version (e.g., "1.0.0")
    version: String,

    /// Owner (person or organization) ID
    owner_id: Uuid,

    /// Tags for categorization and search
    tags: HashSet<String>,
}

impl AgentMetadata {
    /// Create new agent metadata
    ///
    /// # Arguments
    ///
    /// * `name` - Human-readable agent name
    /// * `description` - Agent purpose/description
    /// * `version` - Semantic version
    /// * `owner_id` - ID of owning person/organization
    ///
    /// # Examples
    ///
    /// ```
    /// use cim_domain_agent::value_objects_new::AgentMetadata;
    /// use uuid::Uuid;
    ///
    /// let metadata = AgentMetadata::new(
    ///     "DataProcessor",
    ///     "Processes incoming data streams",
    ///     "1.0.0",
    ///     Uuid::now_v7()
    /// );
    /// ```
    pub fn new(
        name: impl Into<String>,
        description: impl Into<String>,
        version: impl Into<String>,
        owner_id: Uuid,
    ) -> Self {
        Self {
            name: name.into(),
            description: description.into(),
            version: version.into(),
            owner_id,
            tags: HashSet::new(),
        }
    }

    /// Add a tag
    pub fn with_tag(mut self, tag: impl Into<String>) -> Self {
        self.tags.insert(tag.into());
        self
    }

    /// Add multiple tags
    pub fn with_tags(mut self, tags: impl IntoIterator<Item = impl Into<String>>) -> Self {
        for tag in tags {
            self.tags.insert(tag.into());
        }
        self
    }

    /// Get the agent name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get the description
    pub fn description(&self) -> &str {
        &self.description
    }

    /// Get the version
    pub fn version(&self) -> &str {
        &self.version
    }

    /// Get the owner ID
    pub fn owner_id(&self) -> Uuid {
        self.owner_id
    }

    /// Get the tags
    pub fn tags(&self) -> &HashSet<String> {
        &self.tags
    }

    /// Check if has a specific tag
    pub fn has_tag(&self, tag: &str) -> bool {
        self.tags.contains(tag)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_agent_metadata() {
        let owner_id = Uuid::now_v7();
        let metadata = AgentMetadata::new(
            "TestAgent",
            "A test agent",
            "1.0.0",
            owner_id
        );

        assert_eq!(metadata.name(), "TestAgent");
        assert_eq!(metadata.description(), "A test agent");
        assert_eq!(metadata.version(), "1.0.0");
        assert_eq!(metadata.owner_id(), owner_id);
        assert!(metadata.tags().is_empty());
    }

    #[test]
    fn test_metadata_with_tags() {
        let owner_id = Uuid::now_v7();
        let metadata = AgentMetadata::new("Test", "Test", "1.0.0", owner_id)
            .with_tag("ai")
            .with_tag("production");

        assert!(metadata.has_tag("ai"));
        assert!(metadata.has_tag("production"));
        assert!(!metadata.has_tag("development"));
    }

    #[test]
    fn test_metadata_with_multiple_tags() {
        let owner_id = Uuid::now_v7();
        let tags = vec!["tag1", "tag2", "tag3"];
        let metadata = AgentMetadata::new("Test", "Test", "1.0.0", owner_id)
            .with_tags(tags);

        assert_eq!(metadata.tags().len(), 3);
        assert!(metadata.has_tag("tag1"));
        assert!(metadata.has_tag("tag2"));
        assert!(metadata.has_tag("tag3"));
    }

    #[test]
    fn test_metadata_serialization() {
        let owner_id = Uuid::now_v7();
        let metadata = AgentMetadata::new("Test", "Test", "1.0.0", owner_id);

        let json = serde_json::to_string(&metadata).unwrap();
        let deserialized: AgentMetadata = serde_json::from_str(&json).unwrap();

        assert_eq!(metadata, deserialized);
    }
}
