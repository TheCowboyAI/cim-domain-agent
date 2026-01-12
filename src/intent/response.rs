// Copyright (c) 2025 - Cowboy AI, LLC.

//! Response Types for Message Intents
//!
//! Defines the response types for different intent types.
//! These are what adapters return after processing intents.

use crate::value_objects::{FinishReason, TokenUsage};
use serde::{Deserialize, Serialize};

/// Response from a chat or completion intent
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatResponse {
    /// The generated text content
    pub content: String,
    /// Finish reason
    pub finish_reason: FinishReason,
    /// Token usage statistics
    pub usage: Option<TokenUsage>,
    /// Any tool calls made
    pub tool_calls: Option<Vec<ToolCall>>,
}

impl ChatResponse {
    /// Create a new chat response
    pub fn new(content: impl Into<String>) -> Self {
        Self {
            content: content.into(),
            finish_reason: FinishReason::Stop,
            usage: None,
            tool_calls: None,
        }
    }

    /// Create a response with tool calls
    pub fn with_tool_calls(mut self, tool_calls: Vec<ToolCall>) -> Self {
        self.tool_calls = Some(tool_calls);
        self.finish_reason = FinishReason::ToolCalls;
        self
    }
}

/// A tool call made by the model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCall {
    /// Unique ID for this call
    pub id: String,
    /// Tool name
    pub name: String,
    /// Arguments as JSON
    pub arguments: serde_json::Value,
}

impl ToolCall {
    /// Create a new tool call
    pub fn new(
        id: impl Into<String>,
        name: impl Into<String>,
        arguments: serde_json::Value,
    ) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            arguments,
        }
    }
}

/// Response from an embedding intent
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmbeddingResponse {
    /// Generated embeddings (one per input)
    pub embeddings: Vec<Vec<f32>>,
    /// Model used for embedding
    pub model: String,
    /// Token usage
    pub usage: Option<TokenUsage>,
}

impl EmbeddingResponse {
    /// Create a new embedding response
    pub fn new(embeddings: Vec<Vec<f32>>, model: impl Into<String>) -> Self {
        Self {
            embeddings,
            model: model.into(),
            usage: None,
        }
    }

    /// Get embedding dimension
    pub fn dimension(&self) -> Option<usize> {
        self.embeddings.first().map(|e| e.len())
    }
}

/// Response from an image generation intent
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageGenerationResponse {
    /// Generated images
    pub images: Vec<GeneratedImage>,
    /// Model used
    pub model: String,
}

impl ImageGenerationResponse {
    /// Create a new image generation response
    pub fn new(images: Vec<GeneratedImage>, model: impl Into<String>) -> Self {
        Self {
            images,
            model: model.into(),
        }
    }
}

/// A generated image
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneratedImage {
    /// Base64-encoded image data
    pub data: Option<String>,
    /// URL to the image
    pub url: Option<String>,
    /// Revised prompt used
    pub revised_prompt: Option<String>,
}

impl GeneratedImage {
    /// Create from base64 data
    pub fn from_base64(data: impl Into<String>) -> Self {
        Self {
            data: Some(data.into()),
            url: None,
            revised_prompt: None,
        }
    }

    /// Create from URL
    pub fn from_url(url: impl Into<String>) -> Self {
        Self {
            data: None,
            url: Some(url.into()),
            revised_prompt: None,
        }
    }

    /// Set the revised prompt
    pub fn with_revised_prompt(mut self, prompt: impl Into<String>) -> Self {
        self.revised_prompt = Some(prompt.into());
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chat_response() {
        let resp = ChatResponse::new("Hello, world!");
        assert_eq!(resp.content, "Hello, world!");
        assert_eq!(resp.finish_reason, FinishReason::Stop);
    }

    #[test]
    fn test_chat_response_with_tools() {
        let tool_call = ToolCall::new(
            "call_123",
            "get_weather",
            serde_json::json!({"location": "NYC"}),
        );
        let resp = ChatResponse::new("").with_tool_calls(vec![tool_call]);

        assert_eq!(resp.finish_reason, FinishReason::ToolCalls);
        assert!(resp.tool_calls.is_some());
    }

    #[test]
    fn test_token_usage() {
        let usage = TokenUsage::new(100, 50);
        assert_eq!(usage.prompt_tokens, 100);
        assert_eq!(usage.completion_tokens, 50);
        assert_eq!(usage.total_tokens, 150);
    }

    #[test]
    fn test_embedding_response() {
        let embeddings = vec![vec![0.1, 0.2, 0.3], vec![0.4, 0.5, 0.6]];
        let resp = EmbeddingResponse::new(embeddings, "text-embedding-ada-002");

        assert_eq!(resp.dimension(), Some(3));
        assert_eq!(resp.embeddings.len(), 2);
    }
}
