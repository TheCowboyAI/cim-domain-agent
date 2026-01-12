// Copyright (c) 2025 - Cowboy AI, LLC.

//! AI Provider Capabilities
//!
//! Defines the capabilities that different AI providers may support.
//! These form the basis of the capability lattice for provider routing.

use serde::{Deserialize, Serialize};

/// Individual capability that a provider may support
///
/// Each capability represents a specific feature or mode of operation.
/// Providers advertise their capabilities, and requests declare requirements.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Capability {
    /// Basic text chat (all providers)
    TextChat,

    /// Streaming responses (most providers)
    Streaming,

    /// System prompt support
    SystemPrompt,

    /// Multi-turn conversation context
    MultiTurn,

    /// Function/tool calling (OpenAI, Anthropic)
    FunctionCalling,

    /// Vision/image input (GPT-4V, Claude 3)
    Vision,

    /// JSON mode output (OpenAI)
    JsonMode,

    /// Code execution (Claude with computer use)
    CodeExecution,

    /// Long context (128k+ tokens)
    LongContext,

    /// Embedding generation
    Embeddings,

    /// Image generation (DALL-E, Stable Diffusion)
    ImageGeneration,

    /// Audio input (Whisper)
    AudioInput,

    /// Audio output (TTS)
    AudioOutput,
}

impl Capability {
    /// All defined capabilities
    pub fn all() -> Vec<Self> {
        vec![
            Self::TextChat,
            Self::Streaming,
            Self::SystemPrompt,
            Self::MultiTurn,
            Self::FunctionCalling,
            Self::Vision,
            Self::JsonMode,
            Self::CodeExecution,
            Self::LongContext,
            Self::Embeddings,
            Self::ImageGeneration,
            Self::AudioInput,
            Self::AudioOutput,
        ]
    }

    /// Basic capabilities that most chat providers support
    pub fn basic_chat() -> Vec<Self> {
        vec![
            Self::TextChat,
            Self::Streaming,
            Self::SystemPrompt,
            Self::MultiTurn,
        ]
    }
}

impl std::fmt::Display for Capability {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TextChat => write!(f, "text_chat"),
            Self::Streaming => write!(f, "streaming"),
            Self::SystemPrompt => write!(f, "system_prompt"),
            Self::MultiTurn => write!(f, "multi_turn"),
            Self::FunctionCalling => write!(f, "function_calling"),
            Self::Vision => write!(f, "vision"),
            Self::JsonMode => write!(f, "json_mode"),
            Self::CodeExecution => write!(f, "code_execution"),
            Self::LongContext => write!(f, "long_context"),
            Self::Embeddings => write!(f, "embeddings"),
            Self::ImageGeneration => write!(f, "image_generation"),
            Self::AudioInput => write!(f, "audio_input"),
            Self::AudioOutput => write!(f, "audio_output"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_capabilities() {
        let all = Capability::all();
        assert!(all.len() >= 10);
        assert!(all.contains(&Capability::TextChat));
        assert!(all.contains(&Capability::Vision));
    }

    #[test]
    fn test_basic_chat() {
        let basic = Capability::basic_chat();
        assert!(basic.contains(&Capability::TextChat));
        assert!(basic.contains(&Capability::Streaming));
        assert!(!basic.contains(&Capability::Vision));
    }
}
