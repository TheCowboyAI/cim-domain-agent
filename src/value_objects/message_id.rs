// Copyright (c) 2025 - Cowboy AI, LLC.

//! Message ID value object
//!
//! Unique identifier for tracking request/response pairs in agent dialogs.

use serde::{Deserialize, Serialize};
use std::fmt;
use uuid::Uuid;

/// Message unique identifier
///
/// Used to correlate messages sent to a model with streaming response chunks.
/// Uses UUID v7 for time-ordered identifiers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct MessageId(Uuid);

impl MessageId {
    /// Create a new Message ID with UUID v7 (time-ordered)
    pub fn new() -> Self {
        Self(Uuid::now_v7())
    }

    /// Create a Message ID from an existing UUID
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

impl Default for MessageId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for MessageId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<Uuid> for MessageId {
    fn from(uuid: Uuid) -> Self {
        Self::from_uuid(uuid)
    }
}

impl From<MessageId> for Uuid {
    fn from(id: MessageId) -> Self {
        id.to_uuid()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_message_id() {
        let id = MessageId::new();
        assert_eq!(id.as_uuid().get_version_num(), 7);
    }

    #[test]
    fn test_message_id_from_uuid() {
        let uuid = Uuid::now_v7();
        let id = MessageId::from_uuid(uuid);
        assert_eq!(id.as_uuid(), &uuid);
    }

    #[test]
    fn test_message_id_display() {
        let uuid = Uuid::now_v7();
        let id = MessageId::from_uuid(uuid);
        assert_eq!(format!("{}", id), format!("{}", uuid));
    }

    #[test]
    fn test_message_id_serialization() {
        let id = MessageId::new();
        let json = serde_json::to_string(&id).unwrap();
        let deserialized: MessageId = serde_json::from_str(&json).unwrap();
        assert_eq!(id, deserialized);
    }
}
