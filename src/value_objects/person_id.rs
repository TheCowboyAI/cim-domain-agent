// Copyright (c) 2025 - Cowboy AI, LLC.

//! Person ID value object
//!
//! Unique identifier for persons - required binding for Agent ownership.
//! An Agent is a Person's automaton and MUST have a PersonId.

use serde::{Deserialize, Serialize};
use std::fmt;
use uuid::Uuid;

/// Person unique identifier
///
/// Required binding for Agent ownership. Every Agent must be bound to a Person.
/// Uses UUID v7 for time-ordered identifiers, ensuring:
/// - Chronological ordering
/// - Monotonicity for better database performance
/// - Embedded timestamp information
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct PersonId(Uuid);

impl PersonId {
    /// Create a new Person ID with UUID v7 (time-ordered)
    pub fn new() -> Self {
        Self(Uuid::now_v7())
    }

    /// Create a Person ID from an existing UUID
    pub fn from_uuid(uuid: Uuid) -> Self {
        Self(uuid)
    }

    /// Get the underlying UUID
    pub fn as_uuid(&self) -> &Uuid {
        &self.0
    }

    /// Convert to the underlying UUID
    pub fn to_uuid(self) -> Uuid {
        self.0
    }
}

impl Default for PersonId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for PersonId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<Uuid> for PersonId {
    fn from(uuid: Uuid) -> Self {
        Self::from_uuid(uuid)
    }
}

impl From<PersonId> for Uuid {
    fn from(id: PersonId) -> Self {
        id.to_uuid()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_person_id() {
        let id = PersonId::new();
        assert_eq!(id.as_uuid().get_version_num(), 7);
    }

    #[test]
    fn test_person_id_from_uuid() {
        let uuid = Uuid::now_v7();
        let id = PersonId::from_uuid(uuid);
        assert_eq!(id.as_uuid(), &uuid);
    }

    #[test]
    fn test_person_id_display() {
        let uuid = Uuid::now_v7();
        let id = PersonId::from_uuid(uuid);
        assert_eq!(format!("{}", id), format!("{}", uuid));
    }

    #[test]
    fn test_person_id_serialization() {
        let id = PersonId::new();
        let json = serde_json::to_string(&id).unwrap();
        let deserialized: PersonId = serde_json::from_str(&json).unwrap();
        assert_eq!(id, deserialized);
    }
}
