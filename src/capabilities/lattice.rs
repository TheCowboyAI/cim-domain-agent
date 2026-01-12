// Copyright (c) 2025 - Cowboy AI, LLC.

//! Capability Lattice
//!
//! A lattice algebra for AI provider capabilities. Enables:
//! - meet (∧): Intersection of capabilities
//! - join (∨): Union of capabilities
//! - satisfies (≥): Check if capabilities satisfy requirements
//!
//! ## Mathematical Properties
//!
//! This forms a bounded lattice where:
//! - Bottom element (⊥) = no capabilities = `RuntimeCapabilities::empty()`
//! - Top element (⊤) = all capabilities = `RuntimeCapabilities::all()`
//! - meet is intersection (AND)
//! - join is union (OR)
//!
//! ## Lattice Laws
//!
//! For all capabilities A, B, C:
//! - Associativity: (A ∧ B) ∧ C = A ∧ (B ∧ C)
//! - Commutativity: A ∧ B = B ∧ A
//! - Absorption: A ∧ (A ∨ B) = A
//! - Identity: A ∧ ⊤ = A, A ∨ ⊥ = A

use bitflags::bitflags;
use serde::{Deserialize, Serialize};

bitflags! {
    /// Runtime capabilities as a bit field
    ///
    /// This provides O(1) capability checking and efficient storage.
    /// The lattice operations (meet/join) are simple bitwise operations.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct RuntimeCapabilities: u32 {
        /// Basic text chat
        const TEXT_CHAT = 0b0000_0000_0000_0001;
        /// Streaming responses
        const STREAMING = 0b0000_0000_0000_0010;
        /// System prompt support
        const SYSTEM_PROMPT = 0b0000_0000_0000_0100;
        /// Multi-turn conversations
        const MULTI_TURN = 0b0000_0000_0000_1000;
        /// Function/tool calling
        const FUNCTION_CALLING = 0b0000_0000_0001_0000;
        /// Vision/image input
        const VISION = 0b0000_0000_0010_0000;
        /// JSON mode output
        const JSON_MODE = 0b0000_0000_0100_0000;
        /// Code execution
        const CODE_EXECUTION = 0b0000_0000_1000_0000;
        /// Long context (128k+)
        const LONG_CONTEXT = 0b0000_0001_0000_0000;
        /// Embedding generation
        const EMBEDDINGS = 0b0000_0010_0000_0000;
        /// Image generation
        const IMAGE_GENERATION = 0b0000_0100_0000_0000;
        /// Audio input
        const AUDIO_INPUT = 0b0000_1000_0000_0000;
        /// Audio output
        const AUDIO_OUTPUT = 0b0001_0000_0000_0000;

        /// Basic chat capabilities (common to most providers)
        const BASIC_CHAT = Self::TEXT_CHAT.bits()
            | Self::STREAMING.bits()
            | Self::SYSTEM_PROMPT.bits()
            | Self::MULTI_TURN.bits();

        /// Advanced chat capabilities
        const ADVANCED_CHAT = Self::BASIC_CHAT.bits()
            | Self::FUNCTION_CALLING.bits()
            | Self::JSON_MODE.bits();

        /// Multimodal capabilities
        const MULTIMODAL = Self::VISION.bits()
            | Self::AUDIO_INPUT.bits()
            | Self::AUDIO_OUTPUT.bits()
            | Self::IMAGE_GENERATION.bits();
    }
}

impl RuntimeCapabilities {
    /// Lattice meet operation (intersection, AND)
    ///
    /// Returns capabilities that exist in BOTH self and other.
    /// This is the greatest lower bound in the lattice.
    #[inline]
    pub fn meet(&self, other: &Self) -> Self {
        *self & *other
    }

    /// Lattice join operation (union, OR)
    ///
    /// Returns capabilities that exist in EITHER self or other.
    /// This is the least upper bound in the lattice.
    #[inline]
    pub fn join(&self, other: &Self) -> Self {
        *self | *other
    }

    /// Check if self satisfies the required capabilities
    ///
    /// Returns true if self contains ALL capabilities in required.
    /// In lattice terms: self ≥ required (self is above or equal to required)
    #[inline]
    pub fn satisfies(&self, required: &Self) -> bool {
        self.contains(*required)
    }

    /// Check if this represents the bottom element (no capabilities)
    #[inline]
    pub fn is_bottom(&self) -> bool {
        self.is_empty()
    }

    /// Check if this represents the top element (all capabilities)
    #[inline]
    pub fn is_top(&self) -> bool {
        *self == Self::all()
    }

    /// Get human-readable list of capabilities
    pub fn to_vec(&self) -> Vec<&'static str> {
        let mut result = Vec::new();
        if self.contains(Self::TEXT_CHAT) {
            result.push("text_chat");
        }
        if self.contains(Self::STREAMING) {
            result.push("streaming");
        }
        if self.contains(Self::SYSTEM_PROMPT) {
            result.push("system_prompt");
        }
        if self.contains(Self::MULTI_TURN) {
            result.push("multi_turn");
        }
        if self.contains(Self::FUNCTION_CALLING) {
            result.push("function_calling");
        }
        if self.contains(Self::VISION) {
            result.push("vision");
        }
        if self.contains(Self::JSON_MODE) {
            result.push("json_mode");
        }
        if self.contains(Self::CODE_EXECUTION) {
            result.push("code_execution");
        }
        if self.contains(Self::LONG_CONTEXT) {
            result.push("long_context");
        }
        if self.contains(Self::EMBEDDINGS) {
            result.push("embeddings");
        }
        if self.contains(Self::IMAGE_GENERATION) {
            result.push("image_generation");
        }
        if self.contains(Self::AUDIO_INPUT) {
            result.push("audio_input");
        }
        if self.contains(Self::AUDIO_OUTPUT) {
            result.push("audio_output");
        }
        result
    }
}

impl Default for RuntimeCapabilities {
    fn default() -> Self {
        Self::empty()
    }
}

impl std::fmt::Display for RuntimeCapabilities {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let caps = self.to_vec();
        if caps.is_empty() {
            write!(f, "(none)")
        } else {
            write!(f, "{}", caps.join(", "))
        }
    }
}

/// Provider capabilities with metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderCapabilities {
    /// The provider's name
    pub provider_name: String,
    /// Runtime capabilities
    pub capabilities: RuntimeCapabilities,
    /// Maximum context length (tokens)
    pub max_context_length: Option<u32>,
    /// Whether streaming is the default mode
    pub streaming_default: bool,
}

impl ProviderCapabilities {
    /// Create capabilities for a provider
    pub fn new(name: impl Into<String>, capabilities: RuntimeCapabilities) -> Self {
        Self {
            provider_name: name.into(),
            capabilities,
            max_context_length: None,
            streaming_default: true,
        }
    }

    /// Create capabilities for OpenAI GPT-4
    pub fn openai_gpt4() -> Self {
        Self {
            provider_name: "openai-gpt4".to_string(),
            capabilities: RuntimeCapabilities::ADVANCED_CHAT
                | RuntimeCapabilities::VISION
                | RuntimeCapabilities::LONG_CONTEXT,
            max_context_length: Some(128_000),
            streaming_default: true,
        }
    }

    /// Create capabilities for Anthropic Claude
    pub fn anthropic_claude() -> Self {
        Self {
            provider_name: "anthropic-claude".to_string(),
            capabilities: RuntimeCapabilities::ADVANCED_CHAT
                | RuntimeCapabilities::VISION
                | RuntimeCapabilities::LONG_CONTEXT
                | RuntimeCapabilities::CODE_EXECUTION,
            max_context_length: Some(200_000),
            streaming_default: true,
        }
    }

    /// Create capabilities for Ollama (local models)
    pub fn ollama() -> Self {
        Self {
            provider_name: "ollama".to_string(),
            capabilities: RuntimeCapabilities::BASIC_CHAT,
            max_context_length: Some(8_192),
            streaming_default: true,
        }
    }

    /// Create capabilities for mock provider (testing)
    pub fn mock() -> Self {
        Self {
            provider_name: "mock".to_string(),
            capabilities: RuntimeCapabilities::BASIC_CHAT,
            max_context_length: Some(4_096),
            streaming_default: false,
        }
    }

    /// Check if this provider satisfies the required capabilities
    pub fn satisfies(&self, required: &RuntimeCapabilities) -> bool {
        self.capabilities.satisfies(required)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_meet_operation() {
        let a = RuntimeCapabilities::TEXT_CHAT | RuntimeCapabilities::VISION;
        let b = RuntimeCapabilities::TEXT_CHAT | RuntimeCapabilities::STREAMING;

        let meet = a.meet(&b);

        assert!(meet.contains(RuntimeCapabilities::TEXT_CHAT));
        assert!(!meet.contains(RuntimeCapabilities::VISION));
        assert!(!meet.contains(RuntimeCapabilities::STREAMING));
    }

    #[test]
    fn test_join_operation() {
        let a = RuntimeCapabilities::TEXT_CHAT | RuntimeCapabilities::VISION;
        let b = RuntimeCapabilities::TEXT_CHAT | RuntimeCapabilities::STREAMING;

        let join = a.join(&b);

        assert!(join.contains(RuntimeCapabilities::TEXT_CHAT));
        assert!(join.contains(RuntimeCapabilities::VISION));
        assert!(join.contains(RuntimeCapabilities::STREAMING));
    }

    #[test]
    fn test_satisfies() {
        let provider = RuntimeCapabilities::BASIC_CHAT | RuntimeCapabilities::VISION;
        let required = RuntimeCapabilities::TEXT_CHAT | RuntimeCapabilities::STREAMING;

        assert!(provider.satisfies(&required));

        let vision_required = RuntimeCapabilities::VISION | RuntimeCapabilities::EMBEDDINGS;
        assert!(!provider.satisfies(&vision_required)); // Missing EMBEDDINGS
    }

    #[test]
    fn test_lattice_laws_associativity() {
        let a = RuntimeCapabilities::TEXT_CHAT;
        let b = RuntimeCapabilities::VISION;
        let c = RuntimeCapabilities::STREAMING;

        // (A ∧ B) ∧ C = A ∧ (B ∧ C)
        assert_eq!(a.meet(&b).meet(&c), a.meet(&b.meet(&c)));

        // (A ∨ B) ∨ C = A ∨ (B ∨ C)
        assert_eq!(a.join(&b).join(&c), a.join(&b.join(&c)));
    }

    #[test]
    fn test_lattice_laws_commutativity() {
        let a = RuntimeCapabilities::TEXT_CHAT | RuntimeCapabilities::VISION;
        let b = RuntimeCapabilities::STREAMING | RuntimeCapabilities::EMBEDDINGS;

        // A ∧ B = B ∧ A
        assert_eq!(a.meet(&b), b.meet(&a));

        // A ∨ B = B ∨ A
        assert_eq!(a.join(&b), b.join(&a));
    }

    #[test]
    fn test_lattice_laws_absorption() {
        let a = RuntimeCapabilities::TEXT_CHAT | RuntimeCapabilities::VISION;
        let b = RuntimeCapabilities::STREAMING;

        // A ∧ (A ∨ B) = A
        assert_eq!(a.meet(&a.join(&b)), a);

        // A ∨ (A ∧ B) = A
        assert_eq!(a.join(&a.meet(&b)), a);
    }

    #[test]
    fn test_lattice_identity() {
        let a = RuntimeCapabilities::TEXT_CHAT | RuntimeCapabilities::VISION;
        let top = RuntimeCapabilities::all();
        let bottom = RuntimeCapabilities::empty();

        // A ∧ ⊤ = A
        assert_eq!(a.meet(&top), a);

        // A ∨ ⊥ = A
        assert_eq!(a.join(&bottom), a);
    }

    #[test]
    fn test_provider_capabilities() {
        let gpt4 = ProviderCapabilities::openai_gpt4();
        let claude = ProviderCapabilities::anthropic_claude();

        // Both support vision
        assert!(gpt4.satisfies(&RuntimeCapabilities::VISION));
        assert!(claude.satisfies(&RuntimeCapabilities::VISION));

        // Only Claude supports code execution
        assert!(!gpt4.satisfies(&RuntimeCapabilities::CODE_EXECUTION));
        assert!(claude.satisfies(&RuntimeCapabilities::CODE_EXECUTION));
    }

    #[test]
    fn test_to_vec() {
        let caps = RuntimeCapabilities::TEXT_CHAT | RuntimeCapabilities::STREAMING;
        let vec = caps.to_vec();

        assert!(vec.contains(&"text_chat"));
        assert!(vec.contains(&"streaming"));
        assert_eq!(vec.len(), 2);
    }
}
