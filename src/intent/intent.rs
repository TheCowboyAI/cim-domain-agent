// Copyright (c) 2025 - Cowboy AI, LLC.

//! Message Intent Types
//!
//! Defines the different types of messages that can be sent to AI providers.
//! Each intent type has different input requirements and response formats.

use crate::capabilities::{CapabilityRequirements, RuntimeCapabilities};
use crate::value_objects::ContextMessage;
use serde::{Deserialize, Serialize};

/// Message intent representing what type of AI interaction is requested
///
/// Each variant captures the specific inputs needed for that type of request.
/// The intent is used to:
/// 1. Infer capability requirements for provider selection
/// 2. Route to the appropriate adapter method
/// 3. Format the request for the provider's API
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MessageIntent {
    /// Multi-turn chat conversation
    Chat {
        /// Conversation context (system prompt + history + user message)
        context: Vec<ContextMessage>,
        /// Optional tools/functions to enable
        tools: Option<Vec<ToolDefinition>>,
        /// Whether to stream the response
        stream: bool,
    },

    /// One-shot text completion
    Completion {
        /// The prompt to complete
        prompt: String,
        /// Optional suffix for fill-in-the-middle
        suffix: Option<String>,
        /// Maximum tokens to generate
        max_tokens: Option<u32>,
    },

    /// Vision/image analysis
    Vision {
        /// Text context for the vision request
        context: Vec<ContextMessage>,
        /// Image inputs to analyze
        images: Vec<ImageInput>,
        /// Whether to stream the response
        stream: bool,
    },

    /// Generate embeddings for text
    Embedding {
        /// Text inputs to embed
        input: Vec<String>,
        /// Optional model override
        model: Option<String>,
    },

    /// Generate images from text
    ImageGeneration {
        /// The prompt describing the image
        prompt: String,
        /// Image size
        size: ImageSize,
        /// Image style
        style: ImageStyle,
        /// Number of images to generate
        n: u32,
    },
}

impl MessageIntent {
    /// Create a simple chat intent
    pub fn chat(context: Vec<ContextMessage>) -> Self {
        Self::Chat {
            context,
            tools: None,
            stream: true,
        }
    }

    /// Create a chat intent with tools
    pub fn chat_with_tools(context: Vec<ContextMessage>, tools: Vec<ToolDefinition>) -> Self {
        Self::Chat {
            context,
            tools: Some(tools),
            stream: true,
        }
    }

    /// Create a simple completion intent
    pub fn completion(prompt: impl Into<String>) -> Self {
        Self::Completion {
            prompt: prompt.into(),
            suffix: None,
            max_tokens: None,
        }
    }

    /// Create a vision intent
    pub fn vision(context: Vec<ContextMessage>, images: Vec<ImageInput>) -> Self {
        Self::Vision {
            context,
            images,
            stream: true,
        }
    }

    /// Create an embedding intent
    pub fn embedding(input: Vec<String>) -> Self {
        Self::Embedding { input, model: None }
    }

    /// Create an image generation intent
    pub fn image_generation(prompt: impl Into<String>) -> Self {
        Self::ImageGeneration {
            prompt: prompt.into(),
            size: ImageSize::default(),
            style: ImageStyle::default(),
            n: 1,
        }
    }

    /// Infer capability requirements from this intent
    pub fn capability_requirements(&self) -> CapabilityRequirements {
        match self {
            Self::Chat { tools, stream, .. } => {
                let mut caps = RuntimeCapabilities::TEXT_CHAT;
                if *stream {
                    caps |= RuntimeCapabilities::STREAMING;
                }
                if tools.is_some() {
                    caps |= RuntimeCapabilities::FUNCTION_CALLING;
                }
                CapabilityRequirements::new(caps)
            }

            Self::Completion { .. } => {
                CapabilityRequirements::new(RuntimeCapabilities::TEXT_CHAT)
            }

            Self::Vision { stream, .. } => {
                let mut caps = RuntimeCapabilities::TEXT_CHAT | RuntimeCapabilities::VISION;
                if *stream {
                    caps |= RuntimeCapabilities::STREAMING;
                }
                CapabilityRequirements::new(caps)
            }

            Self::Embedding { .. } => {
                CapabilityRequirements::new(RuntimeCapabilities::EMBEDDINGS)
            }

            Self::ImageGeneration { .. } => {
                CapabilityRequirements::new(RuntimeCapabilities::IMAGE_GENERATION)
            }
        }
    }

    /// Get a descriptive name for this intent type
    pub fn name(&self) -> &'static str {
        match self {
            Self::Chat { .. } => "chat",
            Self::Completion { .. } => "completion",
            Self::Vision { .. } => "vision",
            Self::Embedding { .. } => "embedding",
            Self::ImageGeneration { .. } => "image_generation",
        }
    }

    /// Check if this intent expects streaming response
    pub fn expects_streaming(&self) -> bool {
        match self {
            Self::Chat { stream, .. } => *stream,
            Self::Vision { stream, .. } => *stream,
            Self::Completion { .. } => false,
            Self::Embedding { .. } => false,
            Self::ImageGeneration { .. } => false,
        }
    }
}

/// Tool/function definition for function calling
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolDefinition {
    /// Tool name
    pub name: String,
    /// Tool description
    pub description: String,
    /// JSON schema for parameters
    pub parameters: serde_json::Value,
}

impl ToolDefinition {
    /// Create a new tool definition
    pub fn new(
        name: impl Into<String>,
        description: impl Into<String>,
        parameters: serde_json::Value,
    ) -> Self {
        Self {
            name: name.into(),
            description: description.into(),
            parameters,
        }
    }
}

/// Image input for vision requests
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ImageInput {
    /// Base64-encoded image data
    Base64 {
        data: String,
        media_type: String,
    },
    /// URL to an image
    Url { url: String },
}

impl ImageInput {
    /// Create from base64 data
    pub fn base64(data: impl Into<String>, media_type: impl Into<String>) -> Self {
        Self::Base64 {
            data: data.into(),
            media_type: media_type.into(),
        }
    }

    /// Create from URL
    pub fn url(url: impl Into<String>) -> Self {
        Self::Url { url: url.into() }
    }
}

/// Image size for generation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ImageSize {
    /// 256x256 pixels
    Small,
    /// 512x512 pixels
    Medium,
    /// 1024x1024 pixels
    Large,
    /// 1024x1792 (portrait)
    Portrait,
    /// 1792x1024 (landscape)
    Landscape,
}

impl Default for ImageSize {
    fn default() -> Self {
        Self::Large
    }
}

impl ImageSize {
    /// Get the dimensions as (width, height)
    pub fn dimensions(&self) -> (u32, u32) {
        match self {
            Self::Small => (256, 256),
            Self::Medium => (512, 512),
            Self::Large => (1024, 1024),
            Self::Portrait => (1024, 1792),
            Self::Landscape => (1792, 1024),
        }
    }

    /// Get the size string for API calls
    pub fn to_api_string(&self) -> &'static str {
        match self {
            Self::Small => "256x256",
            Self::Medium => "512x512",
            Self::Large => "1024x1024",
            Self::Portrait => "1024x1792",
            Self::Landscape => "1792x1024",
        }
    }
}

/// Image style for generation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ImageStyle {
    /// Natural, photorealistic style
    Natural,
    /// Vivid, dramatic style
    Vivid,
    /// Artistic style
    Artistic,
}

impl Default for ImageStyle {
    fn default() -> Self {
        Self::Natural
    }
}

impl ImageStyle {
    /// Get the style string for API calls
    pub fn to_api_string(&self) -> &'static str {
        match self {
            Self::Natural => "natural",
            Self::Vivid => "vivid",
            Self::Artistic => "artistic",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chat_intent_requirements() {
        let intent = MessageIntent::chat(vec![ContextMessage::user("Hello")]);
        let reqs = intent.capability_requirements();

        assert!(reqs.capabilities.contains(RuntimeCapabilities::TEXT_CHAT));
        assert!(reqs.capabilities.contains(RuntimeCapabilities::STREAMING));
    }

    #[test]
    fn test_chat_with_tools_requirements() {
        let tools = vec![ToolDefinition::new(
            "get_weather",
            "Get weather info",
            serde_json::json!({}),
        )];
        let intent = MessageIntent::chat_with_tools(vec![ContextMessage::user("Hello")], tools);
        let reqs = intent.capability_requirements();

        assert!(reqs
            .capabilities
            .contains(RuntimeCapabilities::FUNCTION_CALLING));
    }

    #[test]
    fn test_vision_intent_requirements() {
        let images = vec![ImageInput::url("https://example.com/image.jpg")];
        let intent = MessageIntent::vision(vec![ContextMessage::user("What's in this image?")], images);
        let reqs = intent.capability_requirements();

        assert!(reqs.capabilities.contains(RuntimeCapabilities::VISION));
    }

    #[test]
    fn test_embedding_intent_requirements() {
        let intent = MessageIntent::embedding(vec!["Hello world".to_string()]);
        let reqs = intent.capability_requirements();

        assert!(reqs.capabilities.contains(RuntimeCapabilities::EMBEDDINGS));
        assert!(!reqs.capabilities.contains(RuntimeCapabilities::TEXT_CHAT));
    }

    #[test]
    fn test_image_generation_requirements() {
        let intent = MessageIntent::image_generation("A sunset over mountains");
        let reqs = intent.capability_requirements();

        assert!(reqs
            .capabilities
            .contains(RuntimeCapabilities::IMAGE_GENERATION));
    }

    #[test]
    fn test_intent_names() {
        assert_eq!(MessageIntent::chat(vec![]).name(), "chat");
        assert_eq!(MessageIntent::completion("test").name(), "completion");
        assert_eq!(
            MessageIntent::embedding(vec![]).name(),
            "embedding"
        );
    }
}
