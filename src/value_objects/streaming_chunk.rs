// Copyright (c) 2025 - Cowboy AI, LLC.

//! Streaming chunk value object
//!
//! Represents a partial response from an AI model during streaming.

use serde::{Deserialize, Serialize};

/// Reason why model generation finished
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FinishReason {
    /// Natural stop - model completed response
    Stop,
    /// Hit max_tokens limit
    Length,
    /// Content was filtered by safety systems
    ContentFilter,
    /// Function/tool call requested
    ToolCalls,
    /// Error occurred during generation
    Error,
}

impl FinishReason {
    /// Check if this is a successful completion
    pub fn is_success(&self) -> bool {
        matches!(self, FinishReason::Stop | FinishReason::ToolCalls)
    }

    /// Check if generation was truncated
    pub fn is_truncated(&self) -> bool {
        matches!(self, FinishReason::Length)
    }
}

/// A streaming chunk from an AI model response
///
/// Represents a partial response during streaming generation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StreamingChunk {
    /// Zero-based index of this chunk in the stream
    pub chunk_index: u32,

    /// The text content of this chunk
    pub content: String,

    /// Whether this is the final chunk
    pub is_final: bool,

    /// Reason why generation finished (only present on final chunk)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finish_reason: Option<FinishReason>,
}

impl StreamingChunk {
    /// Create a new streaming chunk
    pub fn new(chunk_index: u32, content: impl Into<String>) -> Self {
        Self {
            chunk_index,
            content: content.into(),
            is_final: false,
            finish_reason: None,
        }
    }

    /// Create a final streaming chunk
    pub fn final_chunk(
        chunk_index: u32,
        content: impl Into<String>,
        finish_reason: FinishReason,
    ) -> Self {
        Self {
            chunk_index,
            content: content.into(),
            is_final: true,
            finish_reason: Some(finish_reason),
        }
    }

    /// Create an empty final chunk (just signals completion)
    pub fn completion(chunk_index: u32, finish_reason: FinishReason) -> Self {
        Self {
            chunk_index,
            content: String::new(),
            is_final: true,
            finish_reason: Some(finish_reason),
        }
    }

    /// Check if this chunk has content
    pub fn has_content(&self) -> bool {
        !self.content.is_empty()
    }

    /// Get the content length in characters
    pub fn content_len(&self) -> usize {
        self.content.len()
    }
}

/// Token usage statistics for a completed response
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct TokenUsage {
    /// Number of tokens in the prompt
    pub prompt_tokens: u32,

    /// Number of tokens in the completion
    pub completion_tokens: u32,

    /// Total tokens (prompt + completion)
    pub total_tokens: u32,
}

impl TokenUsage {
    /// Create new token usage statistics
    pub fn new(prompt_tokens: u32, completion_tokens: u32) -> Self {
        Self {
            prompt_tokens,
            completion_tokens,
            total_tokens: prompt_tokens + completion_tokens,
        }
    }
}

/// Message role in a conversation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MessageRole {
    /// System message (sets behavior)
    System,
    /// User message (from human)
    User,
    /// Assistant message (from model)
    Assistant,
}

/// A message in a conversation context
///
/// Used to provide conversation history to stateless message requests.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContextMessage {
    /// The role of this message
    pub role: MessageRole,

    /// The content of this message
    pub content: String,
}

impl ContextMessage {
    /// Create a system message
    pub fn system(content: impl Into<String>) -> Self {
        Self {
            role: MessageRole::System,
            content: content.into(),
        }
    }

    /// Create a user message
    pub fn user(content: impl Into<String>) -> Self {
        Self {
            role: MessageRole::User,
            content: content.into(),
        }
    }

    /// Create an assistant message
    pub fn assistant(content: impl Into<String>) -> Self {
        Self {
            role: MessageRole::Assistant,
            content: content.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_finish_reason_success() {
        assert!(FinishReason::Stop.is_success());
        assert!(FinishReason::ToolCalls.is_success());
        assert!(!FinishReason::Length.is_success());
        assert!(!FinishReason::ContentFilter.is_success());
        assert!(!FinishReason::Error.is_success());
    }

    #[test]
    fn test_streaming_chunk_new() {
        let chunk = StreamingChunk::new(0, "Hello");
        assert_eq!(chunk.chunk_index, 0);
        assert_eq!(chunk.content, "Hello");
        assert!(!chunk.is_final);
        assert!(chunk.finish_reason.is_none());
    }

    #[test]
    fn test_streaming_chunk_final() {
        let chunk = StreamingChunk::final_chunk(5, " world!", FinishReason::Stop);
        assert_eq!(chunk.chunk_index, 5);
        assert_eq!(chunk.content, " world!");
        assert!(chunk.is_final);
        assert_eq!(chunk.finish_reason, Some(FinishReason::Stop));
    }

    #[test]
    fn test_streaming_chunk_completion() {
        let chunk = StreamingChunk::completion(10, FinishReason::Length);
        assert_eq!(chunk.chunk_index, 10);
        assert!(chunk.content.is_empty());
        assert!(chunk.is_final);
        assert_eq!(chunk.finish_reason, Some(FinishReason::Length));
    }

    #[test]
    fn test_token_usage() {
        let usage = TokenUsage::new(100, 50);
        assert_eq!(usage.prompt_tokens, 100);
        assert_eq!(usage.completion_tokens, 50);
        assert_eq!(usage.total_tokens, 150);
    }

    #[test]
    fn test_context_message() {
        let system = ContextMessage::system("You are helpful.");
        assert_eq!(system.role, MessageRole::System);

        let user = ContextMessage::user("Hello!");
        assert_eq!(user.role, MessageRole::User);

        let assistant = ContextMessage::assistant("Hi there!");
        assert_eq!(assistant.role, MessageRole::Assistant);
    }

    #[test]
    fn test_streaming_chunk_serialization() {
        let chunk = StreamingChunk::final_chunk(3, "Done", FinishReason::Stop);
        let json = serde_json::to_string(&chunk).unwrap();
        let deserialized: StreamingChunk = serde_json::from_str(&json).unwrap();
        assert_eq!(chunk, deserialized);
    }
}
