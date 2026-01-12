//! Agent ID value object
//!
//! Unique identifier for agents using UUID v7 (time-ordered).

use serde::{Deserialize, Serialize};
use std::fmt;
use uuid::Uuid;

/// Agent unique identifier
///
/// Uses UUID v7 for time-ordered identifiers, ensuring:
/// - Chronological ordering
/// - Monotonicity for better database performance
/// - Embedded timestamp information
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct AgentId(Uuid);

impl AgentId {
    /// Create a new Agent ID with UUID v7 (time-ordered)
    ///
    /// # Examples
    ///
    /// ```
    /// use cim_domain_agent::value_objects::AgentId;
    ///
    /// let id = AgentId::new();
    /// assert!(id.as_uuid().get_version_num() == 7);
    /// ```
    pub fn new() -> Self {
        Self(Uuid::now_v7())
    }

    /// Create an Agent ID from an existing UUID
    ///
    /// # Arguments
    ///
    /// * `uuid` - The UUID to wrap
    ///
    /// # Examples
    ///
    /// ```
    /// use cim_domain_agent::value_objects::AgentId;
    /// use uuid::Uuid;
    ///
    /// let uuid = Uuid::now_v7();
    /// let id = AgentId::from_uuid(uuid);
    /// assert_eq!(id.as_uuid(), &uuid);
    /// ```
    pub fn from_uuid(uuid: Uuid) -> Self {
        Self(uuid)
    }

    /// Get the underlying UUID
    ///
    /// # Examples
    ///
    /// ```
    /// use cim_domain_agent::value_objects::AgentId;
    ///
    /// let id = AgentId::new();
    /// let uuid = id.as_uuid();
    /// ```
    pub fn as_uuid(&self) -> &Uuid {
        &self.0
    }

    /// Convert to the underlying UUID
    ///
    /// # Examples
    ///
    /// ```
    /// use cim_domain_agent::value_objects::AgentId;
    ///
    /// let id = AgentId::new();
    /// let uuid = id.to_uuid();
    /// ```
    pub fn to_uuid(self) -> Uuid {
        self.0
    }
}

impl Default for AgentId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for AgentId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<Uuid> for AgentId {
    fn from(uuid: Uuid) -> Self {
        Self::from_uuid(uuid)
    }
}

impl From<AgentId> for Uuid {
    fn from(id: AgentId) -> Self {
        id.to_uuid()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_agent_id() {
        let id = AgentId::new();
        assert_eq!(id.as_uuid().get_version_num(), 7);
    }

    #[test]
    fn test_agent_id_from_uuid() {
        let uuid = Uuid::now_v7();
        let id = AgentId::from_uuid(uuid);
        assert_eq!(id.as_uuid(), &uuid);
    }

    #[test]
    fn test_agent_id_to_uuid() {
        let uuid = Uuid::now_v7();
        let id = AgentId::from_uuid(uuid);
        assert_eq!(id.to_uuid(), uuid);
    }

    #[test]
    fn test_agent_id_display() {
        let uuid = Uuid::now_v7();
        let id = AgentId::from_uuid(uuid);
        assert_eq!(format!("{}", id), format!("{}", uuid));
    }

    #[test]
    fn test_agent_id_equality() {
        let uuid = Uuid::now_v7();
        let id1 = AgentId::from_uuid(uuid);
        let id2 = AgentId::from_uuid(uuid);
        assert_eq!(id1, id2);
    }

    #[test]
    fn test_agent_id_hash() {
        use std::collections::HashSet;

        let id1 = AgentId::new();
        let id2 = AgentId::new();

        let mut set = HashSet::new();
        set.insert(id1);
        set.insert(id2);

        assert_eq!(set.len(), 2);
        assert!(set.contains(&id1));
        assert!(set.contains(&id2));
    }

    #[test]
    fn test_agent_id_serialization() {
        let id = AgentId::new();
        let json = serde_json::to_string(&id).unwrap();
        let deserialized: AgentId = serde_json::from_str(&json).unwrap();
        assert_eq!(id, deserialized);
    }

    #[test]
    fn test_agent_id_ordering() {
        use std::thread;
        use std::time::Duration;

        let id1 = AgentId::new();
        thread::sleep(Duration::from_millis(1));
        let id2 = AgentId::new();

        // UUID v7 is time-ordered, so later IDs should have later timestamps
        assert_ne!(id1, id2);
    }
}
