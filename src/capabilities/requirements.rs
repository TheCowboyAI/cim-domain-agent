// Copyright (c) 2025 - Cowboy AI, LLC.

//! Capability Requirements
//!
//! Infers required capabilities from message intents and requests.
//! This enables automatic routing to capable providers.

use super::RuntimeCapabilities;
use serde::{Deserialize, Serialize};

/// Requirements inferred from a request
///
/// Contains the required capabilities and optional constraints.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapabilityRequirements {
    /// Required capabilities
    pub capabilities: RuntimeCapabilities,
    /// Minimum context length needed
    pub min_context_length: Option<u32>,
    /// Whether streaming is required
    pub requires_streaming: bool,
    /// Source of the requirements (for debugging)
    pub source: RequirementSource,
}

impl CapabilityRequirements {
    /// Create new requirements with the given capabilities
    pub fn new(capabilities: RuntimeCapabilities) -> Self {
        Self {
            capabilities,
            min_context_length: None,
            requires_streaming: false,
            source: RequirementSource::Explicit,
        }
    }

    /// Requirements for basic text chat
    pub fn text_chat() -> Self {
        Self {
            capabilities: RuntimeCapabilities::TEXT_CHAT,
            min_context_length: None,
            requires_streaming: false,
            source: RequirementSource::Inferred,
        }
    }

    /// Requirements for streaming chat
    pub fn streaming_chat() -> Self {
        Self {
            capabilities: RuntimeCapabilities::TEXT_CHAT | RuntimeCapabilities::STREAMING,
            min_context_length: None,
            requires_streaming: true,
            source: RequirementSource::Inferred,
        }
    }

    /// Requirements for vision/image input
    pub fn vision() -> Self {
        Self {
            capabilities: RuntimeCapabilities::TEXT_CHAT | RuntimeCapabilities::VISION,
            min_context_length: None,
            requires_streaming: false,
            source: RequirementSource::Inferred,
        }
    }

    /// Requirements for function calling
    pub fn function_calling() -> Self {
        Self {
            capabilities: RuntimeCapabilities::TEXT_CHAT | RuntimeCapabilities::FUNCTION_CALLING,
            min_context_length: None,
            requires_streaming: false,
            source: RequirementSource::Inferred,
        }
    }

    /// Requirements for embedding generation
    pub fn embeddings() -> Self {
        Self {
            capabilities: RuntimeCapabilities::EMBEDDINGS,
            min_context_length: None,
            requires_streaming: false,
            source: RequirementSource::Inferred,
        }
    }

    /// Requirements for image generation
    pub fn image_generation() -> Self {
        Self {
            capabilities: RuntimeCapabilities::IMAGE_GENERATION,
            min_context_length: None,
            requires_streaming: false,
            source: RequirementSource::Inferred,
        }
    }

    /// Requirements for long context
    pub fn long_context(min_tokens: u32) -> Self {
        Self {
            capabilities: RuntimeCapabilities::TEXT_CHAT | RuntimeCapabilities::LONG_CONTEXT,
            min_context_length: Some(min_tokens),
            requires_streaming: false,
            source: RequirementSource::Inferred,
        }
    }

    /// Add streaming requirement
    pub fn with_streaming(mut self) -> Self {
        self.capabilities |= RuntimeCapabilities::STREAMING;
        self.requires_streaming = true;
        self
    }

    /// Add minimum context length
    pub fn with_min_context(mut self, tokens: u32) -> Self {
        self.min_context_length = Some(tokens);
        if tokens > 32_000 {
            self.capabilities |= RuntimeCapabilities::LONG_CONTEXT;
        }
        self
    }

    /// Merge with another set of requirements
    pub fn merge(&self, other: &Self) -> Self {
        Self {
            capabilities: self.capabilities.join(&other.capabilities),
            min_context_length: match (self.min_context_length, other.min_context_length) {
                (Some(a), Some(b)) => Some(a.max(b)),
                (Some(a), None) => Some(a),
                (None, Some(b)) => Some(b),
                (None, None) => None,
            },
            requires_streaming: self.requires_streaming || other.requires_streaming,
            source: RequirementSource::Merged,
        }
    }
}

impl Default for CapabilityRequirements {
    fn default() -> Self {
        Self::text_chat()
    }
}

/// Source of capability requirements
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RequirementSource {
    /// Explicitly specified by caller
    Explicit,
    /// Inferred from request content
    Inferred,
    /// Merged from multiple sources
    Merged,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_text_chat_requirements() {
        let req = CapabilityRequirements::text_chat();
        assert!(req.capabilities.contains(RuntimeCapabilities::TEXT_CHAT));
        assert!(!req.requires_streaming);
    }

    #[test]
    fn test_streaming_chat_requirements() {
        let req = CapabilityRequirements::streaming_chat();
        assert!(req.capabilities.contains(RuntimeCapabilities::TEXT_CHAT));
        assert!(req.capabilities.contains(RuntimeCapabilities::STREAMING));
        assert!(req.requires_streaming);
    }

    #[test]
    fn test_with_streaming() {
        let req = CapabilityRequirements::text_chat().with_streaming();
        assert!(req.capabilities.contains(RuntimeCapabilities::STREAMING));
        assert!(req.requires_streaming);
    }

    #[test]
    fn test_with_min_context_adds_long_context() {
        let req = CapabilityRequirements::text_chat().with_min_context(100_000);
        assert!(req.capabilities.contains(RuntimeCapabilities::LONG_CONTEXT));
        assert_eq!(req.min_context_length, Some(100_000));
    }

    #[test]
    fn test_merge_requirements() {
        let req1 = CapabilityRequirements::vision();
        let req2 = CapabilityRequirements::streaming_chat();

        let merged = req1.merge(&req2);

        assert!(merged.capabilities.contains(RuntimeCapabilities::VISION));
        assert!(merged.capabilities.contains(RuntimeCapabilities::STREAMING));
        assert!(merged.requires_streaming);
    }
}
