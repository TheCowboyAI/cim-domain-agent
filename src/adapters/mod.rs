// Copyright (c) 2025 - Cowboy AI, LLC.

//! AI Provider Adapters
//!
//! This module contains adapters that implement the ChatPort interface
//! for various AI providers. The recommended adapter is genai, which
//! supports multiple providers through a single interface.
//!
//! ## Architecture
//!
//! ```text
//! ┌─────────────────────────────────────────────────────────────────────┐
//! │                       ProviderRegistry                              │
//! │                                                                     │
//! │  MessageIntent ──> select_provider() ──> Adapter ──> Provider API  │
//! │                          │                                          │
//! │                          v                                          │
//! │               CapabilityRequirements                                │
//! │                          │                                          │
//! │                          v                                          │
//! │               find_capable_providers()                              │
//! └─────────────────────────────────────────────────────────────────────┘
//! ```
//!
//! ## Adapters
//!
//! - `GenaiAdapter` - Multi-provider adapter using genai crate (recommended)
//! - Legacy adapters - Individual provider adapters (via `ai-providers` feature)
//!
//! ## Usage
//!
//! ```ignore
//! use cim_domain_agent::adapters::{GenaiAdapter, ProviderRegistry};
//! use cim_domain_agent::capabilities::ProviderCapabilities;
//!
//! let mut registry = ProviderRegistry::new();
//!
//! // Register genai adapter with capabilities
//! let adapter = GenaiAdapter::new()?;
//! registry.register(
//!     ProviderType::OpenAI,
//!     adapter,
//!     ProviderCapabilities::openai_gpt4()
//! );
//!
//! // Select provider based on requirements
//! let requirements = CapabilityRequirements::vision();
//! let adapter = registry.select_provider(&requirements)?;
//! ```

mod genai_adapter;
mod provider_registry;

pub use genai_adapter::GenaiAdapter;
pub use provider_registry::ProviderRegistry;
