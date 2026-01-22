// Copyright (c) 2025 - Cowboy AI, LLC.

//! Model capability constraints value object
//!
//! Defines the capabilities and limitations of an AI model.

use serde::{Deserialize, Serialize};

/// Constraints and capabilities of an AI model
///
/// Captures the technical limitations and features supported by a specific
/// model. Used to validate that model usage stays within capabilities and
/// to enable feature-based model selection.
///
/// # Example
///
/// ```
/// use cim_domain_agent::value_objects::ModelConstraints;
///
/// let constraints = ModelConstraints {
///     max_context_window: 200_000,
///     supports_streaming: true,
///     supports_function_calling: true,
///     supports_vision: true,
/// };
///
/// assert!(constraints.supports_streaming);
/// assert!(constraints.validate().is_ok());
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ModelConstraints {
    /// Maximum context window in tokens
    ///
    /// Defines how much text (input + output) the model can process
    /// in a single request. Common values:
    /// - GPT-4: 8,192 or 32,768 tokens
    /// - Claude 3 Opus: 200,000 tokens
    /// - Llama 3: 8,192 tokens
    pub max_context_window: u32,

    /// Whether the model supports streaming responses
    ///
    /// Streaming allows receiving partial responses as they're generated,
    /// improving perceived latency for long responses.
    pub supports_streaming: bool,

    /// Whether the model supports function/tool calling
    ///
    /// Function calling enables structured output and integration with
    /// external tools and APIs.
    pub supports_function_calling: bool,

    /// Whether the model supports vision (image inputs)
    ///
    /// Vision capability allows processing images along with text,
    /// enabling multimodal interactions.
    pub supports_vision: bool,
}

impl ModelConstraints {
    /// Create constraints for a typical modern LLM
    ///
    /// Sensible defaults:
    /// - 128K context window
    /// - Streaming supported
    /// - Function calling supported
    /// - Vision not supported (text-only)
    ///
    /// # Example
    ///
    /// ```
    /// use cim_domain_agent::value_objects::ModelConstraints;
    ///
    /// let constraints = ModelConstraints::default_llm();
    /// assert_eq!(constraints.max_context_window, 128_000);
    /// assert!(constraints.supports_streaming);
    /// ```
    pub fn default_llm() -> Self {
        Self {
            max_context_window: 128_000,
            supports_streaming: true,
            supports_function_calling: true,
            supports_vision: false,
        }
    }

    /// Create constraints for GPT-4 Turbo
    ///
    /// # Example
    ///
    /// ```
    /// use cim_domain_agent::value_objects::ModelConstraints;
    ///
    /// let constraints = ModelConstraints::gpt4_turbo();
    /// assert_eq!(constraints.max_context_window, 128_000);
    /// assert!(constraints.supports_function_calling);
    /// assert!(constraints.supports_vision);
    /// ```
    pub fn gpt4_turbo() -> Self {
        Self {
            max_context_window: 128_000,
            supports_streaming: true,
            supports_function_calling: true,
            supports_vision: true,
        }
    }

    /// Create constraints for Claude 3 Opus
    ///
    /// # Example
    ///
    /// ```
    /// use cim_domain_agent::value_objects::ModelConstraints;
    ///
    /// let constraints = ModelConstraints::claude3_opus();
    /// assert_eq!(constraints.max_context_window, 200_000);
    /// assert!(constraints.supports_vision);
    /// ```
    pub fn claude3_opus() -> Self {
        Self {
            max_context_window: 200_000,
            supports_streaming: true,
            supports_function_calling: true,
            supports_vision: true,
        }
    }

    /// Create constraints for Llama 3 (8B/70B)
    ///
    /// # Example
    ///
    /// ```
    /// use cim_domain_agent::value_objects::ModelConstraints;
    ///
    /// let constraints = ModelConstraints::llama3();
    /// assert_eq!(constraints.max_context_window, 8_192);
    /// assert!(!constraints.supports_vision);
    /// ```
    pub fn llama3() -> Self {
        Self {
            max_context_window: 8_192,
            supports_streaming: true,
            supports_function_calling: false,
            supports_vision: false,
        }
    }

    /// Create constraints for Ollama local models (sensible defaults)
    ///
    /// # Example
    ///
    /// ```
    /// use cim_domain_agent::value_objects::ModelConstraints;
    ///
    /// let constraints = ModelConstraints::ollama_default();
    /// assert_eq!(constraints.max_context_window, 4_096);
    /// ```
    pub fn ollama_default() -> Self {
        Self {
            max_context_window: 4_096,
            supports_streaming: true,
            supports_function_calling: false,
            supports_vision: false,
        }
    }

    /// Builder: set maximum context window
    ///
    /// # Example
    ///
    /// ```
    /// use cim_domain_agent::value_objects::ModelConstraints;
    ///
    /// let constraints = ModelConstraints::default_llm()
    ///     .with_max_context_window(32_000);
    ///
    /// assert_eq!(constraints.max_context_window, 32_000);
    /// ```
    pub fn with_max_context_window(mut self, tokens: u32) -> Self {
        self.max_context_window = tokens;
        self
    }

    /// Builder: set streaming support
    pub fn with_streaming(mut self, supported: bool) -> Self {
        self.supports_streaming = supported;
        self
    }

    /// Builder: set function calling support
    pub fn with_function_calling(mut self, supported: bool) -> Self {
        self.supports_function_calling = supported;
        self
    }

    /// Builder: set vision support
    pub fn with_vision(mut self, supported: bool) -> Self {
        self.supports_vision = supported;
        self
    }

    /// Validate constraints
    ///
    /// # Errors
    ///
    /// Returns error if:
    /// - Context window is 0 or unreasonably large (> 1M tokens)
    ///
    /// # Example
    ///
    /// ```
    /// use cim_domain_agent::value_objects::ModelConstraints;
    ///
    /// let valid = ModelConstraints::default_llm();
    /// assert!(valid.validate().is_ok());
    ///
    /// let invalid = ModelConstraints {
    ///     max_context_window: 0,
    ///     ..ModelConstraints::default_llm()
    /// };
    /// assert!(invalid.validate().is_err());
    /// ```
    pub fn validate(&self) -> Result<(), String> {
        if self.max_context_window == 0 {
            return Err("Context window must be greater than 0".to_string());
        }

        if self.max_context_window > 1_000_000 {
            return Err(format!(
                "Context window {} exceeds maximum of 1,000,000 tokens",
                self.max_context_window
            ));
        }

        Ok(())
    }

    /// Check if requested tokens fit within context window
    ///
    /// # Example
    ///
    /// ```
    /// use cim_domain_agent::value_objects::ModelConstraints;
    ///
    /// let constraints = ModelConstraints::default_llm();
    /// assert!(constraints.can_fit_tokens(50_000));
    /// assert!(!constraints.can_fit_tokens(200_000));
    /// ```
    pub fn can_fit_tokens(&self, tokens: u32) -> bool {
        tokens <= self.max_context_window
    }

    /// Get a summary of capabilities as human-readable string
    ///
    /// # Example
    ///
    /// ```
    /// use cim_domain_agent::value_objects::ModelConstraints;
    ///
    /// let constraints = ModelConstraints::claude3_opus();
    /// let summary = constraints.capability_summary();
    /// assert!(summary.contains("200000"));
    /// assert!(summary.contains("streaming"));
    /// ```
    pub fn capability_summary(&self) -> String {
        let mut features = Vec::new();

        if self.supports_streaming {
            features.push("streaming");
        }
        if self.supports_function_calling {
            features.push("function calling");
        }
        if self.supports_vision {
            features.push("vision");
        }

        let features_str = if features.is_empty() {
            "text-only".to_string()
        } else {
            features.join(", ")
        };

        format!(
            "{} token context, supports: {}",
            self.max_context_window, features_str
        )
    }
}

impl Default for ModelConstraints {
    fn default() -> Self {
        Self::default_llm()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_llm() {
        let constraints = ModelConstraints::default_llm();
        assert_eq!(constraints.max_context_window, 128_000);
        assert!(constraints.supports_streaming);
        assert!(constraints.supports_function_calling);
        assert!(!constraints.supports_vision);
    }

    #[test]
    fn test_gpt4_turbo() {
        let constraints = ModelConstraints::gpt4_turbo();
        assert_eq!(constraints.max_context_window, 128_000);
        assert!(constraints.supports_vision);
        assert!(constraints.supports_function_calling);
    }

    #[test]
    fn test_claude3_opus() {
        let constraints = ModelConstraints::claude3_opus();
        assert_eq!(constraints.max_context_window, 200_000);
        assert!(constraints.supports_vision);
        assert!(constraints.supports_streaming);
    }

    #[test]
    fn test_llama3() {
        let constraints = ModelConstraints::llama3();
        assert_eq!(constraints.max_context_window, 8_192);
        assert!(!constraints.supports_vision);
        assert!(!constraints.supports_function_calling);
    }

    #[test]
    fn test_ollama_default() {
        let constraints = ModelConstraints::ollama_default();
        assert_eq!(constraints.max_context_window, 4_096);
        assert!(constraints.supports_streaming);
    }

    #[test]
    fn test_builders() {
        let constraints = ModelConstraints::default_llm()
            .with_max_context_window(50_000)
            .with_streaming(false)
            .with_function_calling(true)
            .with_vision(true);

        assert_eq!(constraints.max_context_window, 50_000);
        assert!(!constraints.supports_streaming);
        assert!(constraints.supports_function_calling);
        assert!(constraints.supports_vision);
    }

    #[test]
    fn test_validate_success() {
        let constraints = ModelConstraints::default_llm();
        assert!(constraints.validate().is_ok());
    }

    #[test]
    fn test_validate_zero_context() {
        let constraints = ModelConstraints {
            max_context_window: 0,
            ..ModelConstraints::default_llm()
        };
        assert!(constraints.validate().is_err());
    }

    #[test]
    fn test_validate_too_large_context() {
        let constraints = ModelConstraints {
            max_context_window: 2_000_000,
            ..ModelConstraints::default_llm()
        };
        assert!(constraints.validate().is_err());
    }

    #[test]
    fn test_can_fit_tokens() {
        let constraints = ModelConstraints::default_llm();
        assert!(constraints.can_fit_tokens(50_000));
        assert!(constraints.can_fit_tokens(128_000));
        assert!(!constraints.can_fit_tokens(200_000));
    }

    #[test]
    fn test_capability_summary() {
        let constraints = ModelConstraints::claude3_opus();
        let summary = constraints.capability_summary();
        assert!(summary.contains("200000"));
        assert!(summary.contains("streaming"));
        assert!(summary.contains("function calling"));
        assert!(summary.contains("vision"));
    }

    #[test]
    fn test_capability_summary_text_only() {
        let constraints = ModelConstraints {
            max_context_window: 4096,
            supports_streaming: false,
            supports_function_calling: false,
            supports_vision: false,
        };
        let summary = constraints.capability_summary();
        assert!(summary.contains("text-only"));
    }

    #[test]
    fn test_serialization() {
        let constraints = ModelConstraints::gpt4_turbo();
        let json = serde_json::to_string(&constraints).unwrap();
        let deserialized: ModelConstraints = serde_json::from_str(&json).unwrap();
        assert_eq!(constraints, deserialized);
    }

    #[test]
    fn test_equality() {
        let c1 = ModelConstraints::default_llm();
        let c2 = ModelConstraints::default_llm();
        let c3 = ModelConstraints::gpt4_turbo();

        assert_eq!(c1, c2);
        assert_ne!(c1, c3);
    }
}
