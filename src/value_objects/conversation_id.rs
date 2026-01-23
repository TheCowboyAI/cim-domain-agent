// Copyright (c) 2025 - Cowboy AI, LLC.

//! ConversationId value object for agent conversations
//!
//! Represents a unique identifier for agent-to-agent conversations.
//! Uses UUID v7 for time-ordered, distributed unique identifiers.
//!
//! ## Usage
//!
//! ```
//! use cim_domain_agent::value_objects::ConversationId;
//!
//! // Create a new conversation ID
//! let conv_id = ConversationId::new();
//!
//! // Convert to string for NATS subjects
//! let subject = format!("agent.conversations.{}.request", conv_id);
//! ```
//!
//! ## Design Rationale
//!
//! - **UUID v7**: Time-ordered for natural chronological sorting
//! - **Copy semantics**: Lightweight, passed by value
//! - **Display trait**: Converts to hyphenated UUID string for subjects
//! - **Serialization**: Works with serde for persistence

use serde::{Deserialize, Serialize};
use std::fmt;
use uuid::Uuid;

/// Unique identifier for agent conversations
///
/// Conversations are first-class domain entities representing multi-turn
/// interactions between agents. All messages in a conversation share the
/// same ConversationId and are published to the same NATS subject namespace.
///
/// # Subject Pattern
///
/// Conversations use the subject pattern:
/// ```text
/// agent.conversations.{conversation_id}.{message_type}
/// ```
///
/// Where `message_type` is one of: `request`, `response`, `error`
///
/// # Examples
///
/// ```
/// use cim_domain_agent::value_objects::ConversationId;
///
/// let conv_id = ConversationId::new();
/// println!("Conversation: {}", conv_id);
/// // Output: Conversation: 01936f24-3c89-7f3e-8a5b-d4c8e6f2a9b1
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ConversationId(Uuid);

impl ConversationId {
    /// Create a new conversation ID using UUID v7 (time-ordered)
    ///
    /// UUID v7 provides:
    /// - Time-ordered sorting (chronological conversations)
    /// - Distributed unique generation (no coordination needed)
    /// - 128-bit uniqueness (collision-resistant)
    ///
    /// # Examples
    ///
    /// ```
    /// use cim_domain_agent::value_objects::ConversationId;
    ///
    /// let conv1 = ConversationId::new();
    /// let conv2 = ConversationId::new();
    /// assert_ne!(conv1, conv2); // Always unique
    /// ```
    pub fn new() -> Self {
        Self(Uuid::now_v7())
    }

    /// Create from an existing UUID
    ///
    /// Useful when reconstructing ConversationId from stored data.
    ///
    /// # Examples
    ///
    /// ```
    /// use cim_domain_agent::value_objects::ConversationId;
    /// use uuid::Uuid;
    ///
    /// let uuid = Uuid::now_v7();
    /// let conv_id = ConversationId::from_uuid(uuid);
    /// assert_eq!(conv_id.as_uuid(), uuid);
    /// ```
    pub fn from_uuid(uuid: Uuid) -> Self {
        Self(uuid)
    }

    /// Get the underlying UUID
    ///
    /// # Examples
    ///
    /// ```
    /// use cim_domain_agent::value_objects::ConversationId;
    ///
    /// let conv_id = ConversationId::new();
    /// let uuid = conv_id.as_uuid();
    /// assert_eq!(uuid.get_version_num(), 7); // UUID v7
    /// ```
    pub fn as_uuid(&self) -> Uuid {
        self.0
    }
}

impl fmt::Display for ConversationId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Display as hyphenated UUID for NATS subjects
        write!(f, "{}", self.0)
    }
}

impl Default for ConversationId {
    /// Default creates a new conversation ID
    ///
    /// Equivalent to `ConversationId::new()`
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_conversation_id_creation() {
        let conv_id = ConversationId::new();
        let uuid = conv_id.as_uuid();

        // Verify UUID v7
        assert_eq!(uuid.get_version_num(), 7, "Should use UUID v7");
    }

    #[test]
    fn test_conversation_id_uniqueness() {
        let conv1 = ConversationId::new();
        let conv2 = ConversationId::new();

        assert_ne!(conv1, conv2, "Each conversation ID should be unique");
    }

    #[test]
    fn test_conversation_id_display() {
        let uuid = Uuid::now_v7();
        let conv_id = ConversationId::from_uuid(uuid);

        // Display should match UUID hyphenated format
        assert_eq!(conv_id.to_string(), uuid.to_string());
    }

    #[test]
    fn test_conversation_id_from_uuid() {
        let uuid = Uuid::now_v7();
        let conv_id = ConversationId::from_uuid(uuid);

        assert_eq!(conv_id.as_uuid(), uuid);
    }

    #[test]
    fn test_conversation_id_default() {
        let conv_id = ConversationId::default();
        let uuid = conv_id.as_uuid();

        assert_eq!(uuid.get_version_num(), 7, "Default should use UUID v7");
    }

    #[test]
    fn test_conversation_id_copy_semantics() {
        let conv_id = ConversationId::new();
        let conv_id_copy = conv_id; // Copy

        assert_eq!(conv_id, conv_id_copy);
        assert_eq!(conv_id.as_uuid(), conv_id_copy.as_uuid());
    }

    #[test]
    fn test_conversation_id_serialization() {
        let conv_id = ConversationId::new();

        // Serialize to JSON
        let json = serde_json::to_string(&conv_id).expect("should serialize");

        // Deserialize back
        let deserialized: ConversationId =
            serde_json::from_str(&json).expect("should deserialize");

        assert_eq!(conv_id, deserialized);
    }

    #[test]
    fn test_conversation_id_hash() {
        use std::collections::HashSet;

        let conv1 = ConversationId::new();
        let conv2 = ConversationId::new();

        let mut set = HashSet::new();
        set.insert(conv1);
        set.insert(conv2);

        assert_eq!(set.len(), 2, "Should support HashSet");
        assert!(set.contains(&conv1));
        assert!(set.contains(&conv2));
    }

    #[test]
    fn test_conversation_id_eq() {
        let uuid = Uuid::now_v7();
        let conv_id1 = ConversationId::from_uuid(uuid);
        let conv_id2 = ConversationId::from_uuid(uuid);

        assert_eq!(conv_id1, conv_id2);
    }
}
