// Copyright (c) 2025 - Cowboy AI, LLC.

//! Capability Lattice for AI Provider Routing
//!
//! This module implements a capability lattice for selecting AI providers
//! based on their supported features. The lattice provides:
//!
//! - **meet (∧)**: Find common capabilities between providers
//! - **join (∨)**: Combine capabilities from multiple providers
//! - **satisfies (≥)**: Check if a provider meets requirements
//!
//! ## Architecture
//!
//! ```text
//! ┌───────────────────────────────────────────────────────────────────┐
//! │                     Capability Lattice                             │
//! │                                                                    │
//! │    MessageIntent ──> infer_requirements() ──> CapabilityRequirements
//! │                                                        │          │
//! │                                                        v          │
//! │    ProviderRegistry ──> select_provider() ──> satisfies() check   │
//! │         │                                              │          │
//! │         v                                              v          │
//! │    Available Providers           Selected Provider (or error)     │
//! └───────────────────────────────────────────────────────────────────┘
//! ```
//!
//! ## Mathematical Foundation
//!
//! The capability set forms a bounded lattice where:
//! - Bottom (⊥) = empty capabilities
//! - Top (⊤) = all capabilities
//! - Partial order defined by subset inclusion
//!
//! This enables efficient provider selection through lattice operations.
//!
//! ## Usage
//!
//! ```ignore
//! use cim_domain_agent::capabilities::{
//!     RuntimeCapabilities, CapabilityRequirements, ProviderCapabilities
//! };
//!
//! // Define what capabilities are required
//! let requirements = CapabilityRequirements::vision().with_streaming();
//!
//! // Check if a provider satisfies requirements
//! let provider = ProviderCapabilities::openai_gpt4();
//! if provider.satisfies(&requirements.capabilities) {
//!     // Use this provider
//! }
//! ```

mod capability;
mod lattice;
mod requirements;

pub use capability::Capability;
pub use lattice::{ProviderCapabilities, RuntimeCapabilities};
pub use requirements::{CapabilityRequirements, RequirementSource};
