// Copyright (c) 2025 - Cowboy AI, LLC.

//! # CIM Domain Agent v0.9.2 - State Machine Driven Design
//!
//! Agent domain for the Composable Information Machine (CIM).
//!
//! An Agent is a Person's automaton that configures and loads an AI model,
//! forwarding messages and streaming responses via NATS pub/sub.
//!
//! ## Design Principles
//!
//! 1. **Agent = Person's Automaton**: Strict `PersonId` binding enforced at deployment
//! 2. **State Machine Driven**: Agent lifecycle is a formal MealyStateMachine
//! 3. **Stateless Messages**: Message processing is stream transformation, NOT aggregate state
//! 4. **Event-Driven**: Lifecycle events persisted; message events to NATS only
//! 5. **Capability-Based Routing**: Lattice algebra for provider selection
//! 6. **All Modalities**: Chat, completion, vision, embeddings, image generation
//!
//! ## Architecture
//!
//! ```text
//! ┌─────────────────────────────────────────────────────────────────────┐
//! │                          NATS Domain Channel                        │
//! │  agent.commands.{cmd} ←───────────────────→ agent.events.{id}.{evt} │
//! └────────────┬────────────────────────────────────────────────────────┘
//!              │
//!              v
//! ┌────────────────────────────────────────────────────────────────────┐
//! │              Agent Aggregate (MealyStateMachine)                    │
//! │  ┌─────────┐    ┌────────────┐    ┌─────────┐                      │
//! │  │ Draft   │───>│ Configured │───>│ Active  │<──┐                  │
//! │  └─────────┘    └────────────┘    └────┬────┘   │                  │
//! │                                        │        │                  │
//! │                                   ┌────v────┐   │                  │
//! │                                   │Suspended│───┘                  │
//! │                                   └────┬────┘                      │
//! │                                        v                           │
//! │                               ┌──────────────┐                     │
//! │                               │Decommissioned│                     │
//! │                               └──────────────┘                     │
//! └────────────────────────────────────────────────────────────────────┘
//!              │
//!              │ (only Active agents)
//!              v
//! ┌────────────────────────────────────────────────────────────────────┐
//! │          AgentMessageService (Domain Service)                       │
//! │                                                                     │
//! │  MessageIntent ──> CapabilityRouter ──> Provider Adapter           │
//! │                         │                    │                      │
//! │               ┌─────────┴─────────┐          │                      │
//! │               │ Capability Lattice │          │                      │
//! │               │   meet/join/satisfies        │                      │
//! │               └───────────────────┘          v                      │
//! │                              ┌──────────────────────┐               │
//! │                              │   Stream Transform   │               │
//! │                              └──────────┬───────────┘               │
//! │                                         │                           │
//! │  Message Events (NATS only, not persisted) ─────────────>          │
//! └────────────────────────────────────────────────────────────────────┘
//! ```
//!
//! ## Module Structure
//!
//! - `state_machine`: Agent lifecycle MealyStateMachine implementation
//! - `capabilities`: Capability lattice for provider routing
//! - `intent`: Multi-modal message intents (chat, vision, embeddings, etc.)
//! - `adapters`: Provider adapters (genai-based multi-provider support)
//! - `services`: Domain services (AgentMessageService, CapabilityRouter)
//! - `ports`: Hexagonal port interfaces (ChatPort, ChatStream)
//! - `aggregate`: Agent aggregate with event sourcing
//! - `commands`/`events`: CQRS command and event types
//! - `value_objects`: Domain value objects
//! - `infrastructure`: Event store, NATS integration

// Core domain modules
pub mod aggregate;
pub mod commands;
pub mod events;
pub mod value_objects;
pub mod infrastructure;

// State machine for agent lifecycle
pub mod state_machine;

// Capability lattice for provider routing
pub mod capabilities;

// Message intents for multi-modal AI
pub mod intent;

// Hexagonal architecture - Ports & Adapters for AI providers
pub mod ports;

// AI Provider Adapters (genai-based)
pub mod adapters;

// Domain Services
pub mod services;

// Pure functional configuration parser
pub mod config;

// Re-export primary types
pub use aggregate::Agent;
pub use commands::*;
pub use events::*;
pub use value_objects::*;
pub use infrastructure::*;
pub use ports::*;
pub use state_machine::*;
pub use capabilities::*;
pub use intent::*;
pub use adapters::*;
pub use services::*;
pub use config::*;
