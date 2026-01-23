<!-- Copyright (c) 2025 - Cowboy AI, LLC. -->

# Subject Factory Refactoring Guide

## Overview

This guide shows how to refactor the subject factory from the incorrect "inbox pattern" to the mathematically correct conversation-based pattern.

## Before: Inbox Pattern (Incorrect)

```rust
// Current implementation
pub fn agent_to_agent(
    &self,
    from_agent: &str,
    to_agent: &str,
    message_type: &str,
) -> SubjectFactoryResult<Subject> {
    // Subject: agent.to.sage.from.ddd-expert.question
    let to_keyword = SubjectSegment::new("to")?;
    let to_seg = SubjectSegment::new(to_agent)?;
    let from_keyword = SubjectSegment::new("from")?;
    let from_seg = SubjectSegment::new(from_agent)?;
    let msg_seg = SubjectSegment::new(message_type)?;
    Ok(self
        .domain
        .append(to_keyword)
        .append(to_seg)
        .append(from_keyword)
        .append(from_seg)
        .append(msg_seg))
}

// Agents subscribe to:
let pattern = factory.agent_pattern("sage")?; // "agent.to.sage.>"

// Problems:
// 1. ❌ Routing info (to/from) in subject violates semantic purity
// 2. ❌ Difficult to get conversation history
// 3. ❌ Agents must subscribe to all messages addressed to them
// 4. ❌ No natural grouping of related messages
```

## After: Conversation Pattern (Correct)

```rust
// Refactored implementation
pub fn conversation_request(
    &self,
    conv_id: &ConversationId,
) -> SubjectFactoryResult<Subject> {
    // Subject: agent.conversations.conv-123.request
    let conversations = SubjectSegment::new("conversations")?;
    let conv_segment = SubjectSegment::new(conv_id.to_string())?;
    let request = SubjectSegment::new("request")?;
    Ok(self
        .domain
        .append(conversations)
        .append(conv_segment)
        .append(request))
}

pub fn conversation_response(
    &self,
    conv_id: &ConversationId,
) -> SubjectFactoryResult<Subject> {
    // Subject: agent.conversations.conv-123.response
    let conversations = SubjectSegment::new("conversations")?;
    let conv_segment = SubjectSegment::new(conv_id.to_string())?;
    let response = SubjectSegment::new("response")?;
    Ok(self
        .domain
        .append(conversations)
        .append(conv_segment)
        .append(response))
}

pub fn conversation_pattern(
    &self,
    conv_id: &ConversationId,
) -> SubjectFactoryResult<SubjectPattern> {
    // Pattern: agent.conversations.conv-123.>
    let pattern_str = format!("{}.conversations.{}.>", self.domain, conv_id);
    SubjectPattern::parse(&pattern_str).map_err(Into::into)
}

// All conversation participants subscribe to:
let pattern = factory.conversation_pattern(&conv_id)?; // "agent.conversations.conv-123.>"

// Benefits:
// 1. ✅ Pure semantic hierarchy (no routing metadata)
// 2. ✅ Complete conversation history in one namespace
// 3. ✅ Efficient pattern matching by NATS
// 4. ✅ Natural message grouping
```

## New Value Object: ConversationId

```rust
// src/value_objects/conversation_id.rs

use uuid::Uuid;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ConversationId(Uuid);

impl ConversationId {
    /// Create a new conversation ID using UUID v7 (time-ordered)
    pub fn new() -> Self {
        Self(Uuid::now_v7())
    }

    /// Create from existing UUID
    pub fn from_uuid(uuid: Uuid) -> Self {
        Self(uuid)
    }

    /// Get the underlying UUID
    pub fn as_uuid(&self) -> Uuid {
        self.0
    }
}

impl fmt::Display for ConversationId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Default for ConversationId {
    fn default() -> Self {
        Self::new()
    }
}
```

## Complete Refactored Subject Factory

```rust
// src/infrastructure/subject_factory.rs

use crate::value_objects::{AgentId, MessageId, ConversationId};
use cim_domain::{Subject, SubjectError, SubjectPattern, SubjectSegment};
use once_cell::sync::Lazy;

/// Static segments for conversation subjects
mod conversation_segments {
    use super::*;

    pub static CONVERSATIONS: Lazy<SubjectSegment> =
        Lazy::new(|| SubjectSegment::new("conversations").expect("valid segment"));

    pub static REQUEST: Lazy<SubjectSegment> =
        Lazy::new(|| SubjectSegment::new("request").expect("valid segment"));

    pub static RESPONSE: Lazy<SubjectSegment> =
        Lazy::new(|| SubjectSegment::new("response").expect("valid segment"));

    pub static ERROR: Lazy<SubjectSegment> =
        Lazy::new(|| SubjectSegment::new("error").expect("valid segment"));

    pub static METADATA: Lazy<SubjectSegment> =
        Lazy::new(|| SubjectSegment::new("metadata").expect("valid segment"));
}

impl AgentSubjectFactory {
    // ========================================================================
    // Conversation Subjects (PRIMARY PATTERN)
    // ========================================================================

    /// Conversation request subject: `{domain}.conversations.{conv_id}.request`
    ///
    /// Used when an agent sends a request within a conversation.
    /// Routing metadata (sender, recipient) goes in NATS headers.
    pub fn conversation_request(
        &self,
        conv_id: &ConversationId,
    ) -> SubjectFactoryResult<Subject> {
        let conv_segment = SubjectSegment::new(conv_id.to_string())?;
        Ok(self
            .domain
            .append(conversation_segments::CONVERSATIONS.clone())
            .append(conv_segment)
            .append(conversation_segments::REQUEST.clone()))
    }

    /// Conversation response subject: `{domain}.conversations.{conv_id}.response`
    ///
    /// Used when an agent responds within a conversation.
    pub fn conversation_response(
        &self,
        conv_id: &ConversationId,
    ) -> SubjectFactoryResult<Subject> {
        let conv_segment = SubjectSegment::new(conv_id.to_string())?;
        Ok(self
            .domain
            .append(conversation_segments::CONVERSATIONS.clone())
            .append(conv_segment)
            .append(conversation_segments::RESPONSE.clone()))
    }

    /// Conversation error subject: `{domain}.conversations.{conv_id}.error`
    ///
    /// Used when an error occurs within a conversation.
    pub fn conversation_error(
        &self,
        conv_id: &ConversationId,
    ) -> SubjectFactoryResult<Subject> {
        let conv_segment = SubjectSegment::new(conv_id.to_string())?;
        Ok(self
            .domain
            .append(conversation_segments::CONVERSATIONS.clone())
            .append(conv_segment)
            .append(conversation_segments::ERROR.clone()))
    }

    /// Conversation metadata subject: `{domain}.conversations.{conv_id}.metadata`
    ///
    /// Used for conversation lifecycle events (started, completed, archived).
    pub fn conversation_metadata(
        &self,
        conv_id: &ConversationId,
    ) -> SubjectFactoryResult<Subject> {
        let conv_segment = SubjectSegment::new(conv_id.to_string())?;
        Ok(self
            .domain
            .append(conversation_segments::CONVERSATIONS.clone())
            .append(conv_segment)
            .append(conversation_segments::METADATA.clone()))
    }

    /// Conversation pattern: `{domain}.conversations.{conv_id}.>`
    ///
    /// Subscribe to ALL messages in a conversation (requests, responses, errors, metadata).
    /// This is the primary subscription pattern for conversation participants.
    pub fn conversation_pattern(
        &self,
        conv_id: &ConversationId,
    ) -> SubjectFactoryResult<SubjectPattern> {
        let pattern_str = format!("{}.conversations.{}.>", self.domain, conv_id);
        SubjectPattern::parse(&pattern_str).map_err(Into::into)
    }

    /// All conversations pattern: `{domain}.conversations.>`
    ///
    /// Subscribe to ALL conversation activity across the system.
    /// Useful for monitoring, auditing, or analytics.
    pub fn all_conversations_pattern(&self) -> SubjectFactoryResult<SubjectPattern> {
        let pattern_str = format!("{}.conversations.>", self.domain);
        SubjectPattern::parse(&pattern_str).map_err(Into::into)
    }

    // ========================================================================
    // Agent Command Subjects (DIRECT ADDRESSING)
    // ========================================================================

    /// Agent command subject: `{domain}.commands.{agent_name}.{command_type}`
    ///
    /// Direct command to a specific agent. Agent subscribes to its own commands.
    pub fn agent_command(
        &self,
        agent_name: &str,
        command_type: &str,
    ) -> SubjectFactoryResult<Subject> {
        let commands = SubjectSegment::new("commands")?;
        let agent_segment = SubjectSegment::new(agent_name)?;
        let command_segment = SubjectSegment::new(command_type)?;
        Ok(self
            .domain
            .append(commands)
            .append(agent_segment)
            .append(command_segment))
    }

    /// Agent commands pattern: `{domain}.commands.{agent_name}.>`
    ///
    /// Subscribe to ALL commands for a specific agent.
    pub fn agent_commands_pattern(
        &self,
        agent_name: &str,
    ) -> SubjectFactoryResult<SubjectPattern> {
        let pattern_str = format!("{}.commands.{}.>", self.domain, agent_name);
        SubjectPattern::parse(&pattern_str).map_err(Into::into)
    }

    // ========================================================================
    // Broadcast Subjects (ALL AGENTS)
    // ========================================================================

    /// Broadcast subject: `{domain}.broadcast.{message_type}`
    ///
    /// Send message to ALL agents (system-wide announcements).
    pub fn broadcast(
        &self,
        message_type: &str,
    ) -> SubjectFactoryResult<Subject> {
        let broadcast = SubjectSegment::new("broadcast")?;
        let msg_type = SubjectSegment::new(message_type)?;
        Ok(self
            .domain
            .append(broadcast)
            .append(msg_type))
    }

    /// Broadcast pattern: `{domain}.broadcast.>`
    ///
    /// Subscribe to ALL broadcast messages.
    pub fn broadcast_pattern(&self) -> SubjectFactoryResult<SubjectPattern> {
        let pattern_str = format!("{}.broadcast.>", self.domain);
        SubjectPattern::parse(&pattern_str).map_err(Into::into)
    }
}
```

## Usage Examples

### Example 1: Creating a Conversation

```rust
use cim_domain_agent::infrastructure::AgentSubjectFactory;
use cim_domain_agent::value_objects::ConversationId;

async fn create_conversation(nats: &async_nats::Client) -> Result<ConversationId, Box<dyn Error>> {
    let factory = AgentSubjectFactory::new("agent");
    let conv_id = ConversationId::new();

    // Publish conversation metadata (conversation started)
    let subject = factory.conversation_metadata(&conv_id)?;
    let metadata = serde_json::json!({
        "conversation_id": conv_id.to_string(),
        "started_at": chrono::Utc::now().to_rfc3339(),
        "participants": ["sage", "ddd-expert"],
    });

    nats.publish_with_headers(
        subject.to_string(),
        async_nats::HeaderMap::from_iter([
            ("Event-Type".to_string(), "conversation.started".to_string()),
        ]),
        serde_json::to_vec(&metadata)?.into(),
    ).await?;

    Ok(conv_id)
}
```

### Example 2: Subscribing to a Conversation

```rust
async fn subscribe_to_conversation(
    nats: &async_nats::Client,
    conv_id: ConversationId,
) -> Result<async_nats::Subscriber, Box<dyn Error>> {
    let factory = AgentSubjectFactory::new("agent");

    // Subscribe to ALL messages in the conversation
    let pattern = factory.conversation_pattern(&conv_id)?;
    let subscriber = nats.subscribe(pattern.to_string()).await?;

    Ok(subscriber)
}
```

### Example 3: Sending a Request in a Conversation

```rust
async fn send_request(
    nats: &async_nats::Client,
    conv_id: ConversationId,
    from_agent: &str,
    to_agent: &str,
    message: &str,
) -> Result<(), Box<dyn Error>> {
    let factory = AgentSubjectFactory::new("agent");

    // Routing metadata goes in headers (NOT in subject)
    let headers = async_nats::HeaderMap::from_iter([
        ("Sender".to_string(), from_agent.to_string()),
        ("Recipient".to_string(), to_agent.to_string()),
        ("Conversation-Id".to_string(), conv_id.to_string()),
        ("Message-Id".to_string(), uuid::Uuid::now_v7().to_string()),
        ("Timestamp".to_string(), chrono::Utc::now().to_rfc3339()),
    ]);

    let subject = factory.conversation_request(&conv_id)?;
    nats.publish_with_headers(
        subject.to_string(),
        headers,
        message.as_bytes().into(),
    ).await?;

    Ok(())
}
```

### Example 4: Processing Conversation Messages

```rust
async fn process_conversation_messages(
    mut subscriber: async_nats::Subscriber,
    agent_name: &str,
) {
    while let Some(msg) = subscriber.next().await {
        // Check if message is addressed to this agent
        let recipient = msg.headers.as_ref()
            .and_then(|h| h.get("Recipient"))
            .map(|v| v.as_str());

        if recipient != Some(agent_name) {
            // Message not for this agent, skip
            continue;
        }

        // Extract sender
        let sender = msg.headers.as_ref()
            .and_then(|h| h.get("Sender"))
            .map(|v| v.as_str())
            .unwrap_or("unknown");

        // Process message based on subject suffix
        let subject = msg.subject.as_str();
        if subject.ends_with(".request") {
            handle_request(&msg, sender).await;
        } else if subject.ends_with(".response") {
            handle_response(&msg, sender).await;
        } else if subject.ends_with(".error") {
            handle_error(&msg, sender).await;
        }
    }
}
```

## Migration Checklist

### Phase 1: Preparation
- [ ] Add `ConversationId` value object to `src/value_objects/conversation_id.rs`
- [ ] Add conversation segments to `subject_factory.rs`
- [ ] Implement conversation subject methods
- [ ] Write tests for conversation subjects

### Phase 2: Parallel Implementation
- [ ] Update services to support BOTH patterns temporarily
- [ ] Add feature flag for conversation-based subjects
- [ ] Test with real agent conversations

### Phase 3: Migration
- [ ] Mark `agent_to_agent()` as `#[deprecated]`
- [ ] Update all agent implementations to use conversation pattern
- [ ] Update documentation and examples

### Phase 4: Cleanup
- [ ] Remove deprecated methods
- [ ] Remove old inbox pattern tests
- [ ] Update CHANGELOG with breaking changes

## Testing Strategy

### Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_conversation_subjects() {
        let factory = AgentSubjectFactory::new("agent");
        let conv_id = ConversationId::new();

        // Request subject
        let subject = factory.conversation_request(&conv_id).unwrap();
        assert_eq!(
            subject.to_string(),
            format!("agent.conversations.{}.request", conv_id)
        );

        // Response subject
        let subject = factory.conversation_response(&conv_id).unwrap();
        assert_eq!(
            subject.to_string(),
            format!("agent.conversations.{}.response", conv_id)
        );

        // Pattern matching
        let pattern = factory.conversation_pattern(&conv_id).unwrap();
        let request_subject = Subject::parse(&format!(
            "agent.conversations.{}.request",
            conv_id
        )).unwrap();
        assert!(request_subject.matches(&pattern));
    }

    #[test]
    fn test_monoid_properties() {
        let domain = Subject::parse("agent").unwrap();
        let conversations = Subject::parse("conversations").unwrap();
        let conv_id = Subject::parse("conv-123").unwrap();

        // Associativity
        let left = domain.concat(&conversations).concat(&conv_id);
        let right = domain.concat(&conversations.concat(&conv_id));
        assert_eq!(left, right);

        // Identity
        let root = Subject::root();
        assert_eq!(domain.concat(&root), domain);
        assert_eq!(root.concat(&domain), domain);
    }
}
```

### Integration Tests

```rust
#[tokio::test]
async fn test_conversation_flow() {
    let nats = connect_to_nats().await.unwrap();
    let factory = AgentSubjectFactory::new("agent");
    let conv_id = ConversationId::new();

    // Subscribe to conversation
    let pattern = factory.conversation_pattern(&conv_id).unwrap();
    let mut subscriber = nats.subscribe(pattern.to_string()).await.unwrap();

    // Send request
    let request_subject = factory.conversation_request(&conv_id).unwrap();
    nats.publish(request_subject.to_string(), "test message".into())
        .await
        .unwrap();

    // Receive request
    let msg = subscriber.next().await.unwrap();
    assert_eq!(msg.payload.as_ref(), b"test message");
}
```

## Summary

**Key Changes**:
1. ✅ Introduced `ConversationId` as first-class value object
2. ✅ Conversation subjects use semantic namespace hierarchy
3. ✅ Routing metadata moved to NATS headers
4. ✅ Pattern matching leverages free monoid structure
5. ✅ Deprecated inbox pattern methods

**Benefits**:
1. ✅ Mathematically correct subject algebra
2. ✅ Efficient O(1) pattern matching by NATS
3. ✅ Complete conversation history in single namespace
4. ✅ Clean separation of concerns (routing vs semantics)
5. ✅ Easier to reason about and debug

**Next Steps**:
1. Implement `ConversationId` value object
2. Add conversation methods to `AgentSubjectFactory`
3. Write comprehensive tests
4. Update agent implementations
5. Deprecate old patterns
