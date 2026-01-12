// Copyright (c) 2025 - Cowboy AI, LLC.

//! Message Intent Module
//!
//! Defines the types of AI interactions supported by the agent.
//! Each intent type captures the specific inputs and expected outputs
//! for that type of request.
//!
//! ## Architecture
//!
//! ```text
//! ┌─────────────────────────────────────────────────────────────────────┐
//! │                        Message Intent                               │
//! │                                                                     │
//! │   User Request ──> MessageIntent ──> capability_requirements()      │
//! │                          │                    │                     │
//! │                          │                    v                     │
//! │                          │           CapabilityRequirements         │
//! │                          │                    │                     │
//! │                          v                    v                     │
//! │                    genai Adapter ←── Provider Selection             │
//! │                          │                                          │
//! │                          v                                          │
//! │                    IntentResponse                                   │
//! └─────────────────────────────────────────────────────────────────────┘
//! ```
//!
//! ## Intent Types
//!
//! - **Chat**: Multi-turn conversations with optional tool use
//! - **Completion**: One-shot text completion
//! - **Vision**: Image analysis with text
//! - **Embedding**: Generate vector embeddings
//! - **ImageGeneration**: Create images from text
//!
//! ## Usage
//!
//! ```ignore
//! use cim_domain_agent::intent::{MessageIntent, ImageInput, ImageSize};
//!
//! // Create a chat intent
//! let chat = MessageIntent::chat(vec![
//!     ContextMessage::system("You are helpful"),
//!     ContextMessage::user("What is Rust?"),
//! ]);
//!
//! // Create a vision intent
//! let vision = MessageIntent::vision(
//!     vec![ContextMessage::user("What's in this image?")],
//!     vec![ImageInput::url("https://example.com/image.jpg")],
//! );
//!
//! // Create an image generation intent
//! let image_gen = MessageIntent::ImageGeneration {
//!     prompt: "A sunset over mountains".to_string(),
//!     size: ImageSize::Large,
//!     style: ImageStyle::Natural,
//!     n: 1,
//! };
//! ```

mod intent;
mod response;

pub use intent::{ImageInput, ImageSize, ImageStyle, MessageIntent, ToolDefinition};
pub use response::{
    ChatResponse, EmbeddingResponse, GeneratedImage, ImageGenerationResponse, ToolCall,
};
