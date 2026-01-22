// Copyright (c) 2025 - Cowboy AI, LLC.

//! ModelConfiguration identifier value object
//!
//! Unique identifier for model configuration aggregates using UUID v7
//! for time-ordered IDs.

use serde::{Deserialize, Serialize};
use std::fmt;
use uuid::Uuid;

/// Unique identifier for a ModelConfiguration aggregate
///
/// Uses UUID v7 for time-ordered identifiers, which provides:
/// - Chronological sorting
/// - Database index efficiency
/// - Distributed generation without coordination
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ModelConfigurationId(Uuid);

impl ModelConfigurationId {
    /// Create a new ModelConfigurationId with UUID v7 (time-ordered)
    ///
    /// # Example
    ///
    /// ```
    /// use cim_domain_agent::value_objects::ModelConfigurationId;
    ///
    /// let id = ModelConfigurationId::new();
    /// assert!(!id.to_string().is_empty());
    /// ```
    pub fn new() -> Self {
        Self(Uuid::now_v7())
    }

    /// Create from an existing UUID
    ///
    /// # Example
    ///
    /// ```
    /// use cim_domain_agent::value_objects::ModelConfigurationId;
    /// use uuid::Uuid;
    ///
    /// let uuid = Uuid::now_v7();
    /// let id = ModelConfigurationId::from_uuid(uuid);
    /// assert_eq!(id.as_uuid(), &uuid);
    /// ```
    pub fn from_uuid(uuid: Uuid) -> Self {
        Self(uuid)
    }

    /// Get the inner UUID
    ///
    /// # Example
    ///
    /// ```
    /// use cim_domain_agent::value_objects::ModelConfigurationId;
    ///
    /// let id = ModelConfigurationId::new();
    /// let uuid = id.as_uuid();
    /// assert_eq!(uuid.get_version_num(), 7);
    /// ```
    pub fn as_uuid(&self) -> &Uuid {
        &self.0
    }

    /// Convert to UUID
    pub fn to_uuid(self) -> Uuid {
        self.0
    }

    /// Parse from string representation
    ///
    /// # Example
    ///
    /// ```
    /// use cim_domain_agent::value_objects::ModelConfigurationId;
    ///
    /// let id = ModelConfigurationId::new();
    /// let id_str = id.to_string();
    /// let parsed = ModelConfigurationId::parse_str(&id_str).unwrap();
    /// assert_eq!(id, parsed);
    /// ```
    pub fn parse_str(s: &str) -> Result<Self, uuid::Error> {
        Ok(Self(Uuid::parse_str(s)?))
    }
}

impl Default for ModelConfigurationId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for ModelConfigurationId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<Uuid> for ModelConfigurationId {
    fn from(uuid: Uuid) -> Self {
        Self::from_uuid(uuid)
    }
}

impl From<ModelConfigurationId> for Uuid {
    fn from(id: ModelConfigurationId) -> Self {
        id.to_uuid()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_creates_uuid_v7() {
        let id = ModelConfigurationId::new();
        assert_eq!(id.as_uuid().get_version_num(), 7);
    }

    #[test]
    fn test_from_uuid() {
        let uuid = Uuid::now_v7();
        let id = ModelConfigurationId::from_uuid(uuid);
        assert_eq!(id.as_uuid(), &uuid);
    }

    #[test]
    fn test_to_uuid() {
        let uuid = Uuid::now_v7();
        let id = ModelConfigurationId::from_uuid(uuid);
        assert_eq!(id.to_uuid(), uuid);
    }

    #[test]
    fn test_display() {
        let id = ModelConfigurationId::new();
        let display = format!("{}", id);
        assert!(display.contains('-')); // UUID format has dashes
    }

    #[test]
    fn test_parse_str() {
        let id = ModelConfigurationId::new();
        let id_str = id.to_string();
        let parsed = ModelConfigurationId::parse_str(&id_str).unwrap();
        assert_eq!(id, parsed);
    }

    #[test]
    fn test_parse_str_invalid() {
        let result = ModelConfigurationId::parse_str("not-a-uuid");
        assert!(result.is_err());
    }

    #[test]
    fn test_serialization() {
        let id = ModelConfigurationId::new();
        let json = serde_json::to_string(&id).unwrap();
        let deserialized: ModelConfigurationId = serde_json::from_str(&json).unwrap();
        assert_eq!(id, deserialized);
    }

    #[test]
    fn test_hash_and_eq() {
        use std::collections::HashSet;

        let id1 = ModelConfigurationId::new();
        let id2 = ModelConfigurationId::from_uuid(*id1.as_uuid());
        let id3 = ModelConfigurationId::new();

        assert_eq!(id1, id2);
        assert_ne!(id1, id3);

        let mut set = HashSet::new();
        set.insert(id1);
        assert!(set.contains(&id2));
        assert!(!set.contains(&id3));
    }

    #[test]
    fn test_time_ordering() {
        let id1 = ModelConfigurationId::new();
        std::thread::sleep(std::time::Duration::from_millis(2));
        let id2 = ModelConfigurationId::new();

        // UUID v7 should be time-ordered
        assert!(id1.as_uuid() < id2.as_uuid());
    }
}
