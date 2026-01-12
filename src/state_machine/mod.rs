// Copyright (c) 2025 - Cowboy AI, LLC.

//! Agent Lifecycle State Machine
//!
//! This module implements a MealyStateMachine for agent lifecycle management.
//! The state machine is the single source of truth for valid state transitions.
//!
//! ## Architecture
//!
//! ```text
//! ┌─────────────────────────────────────────────────────────────────────┐
//! │                    AgentLifecycleMachine                             │
//! │                                                                      │
//! │    LifecycleCommand  ─────>  MealyStateMachine  ─────>  Vec<Event>  │
//! │         (Input)              (State Machine)            (Output)     │
//! │                                     │                                │
//! │                           AgentLifecycleState                        │
//! │                              (State Type)                            │
//! └─────────────────────────────────────────────────────────────────────┘
//! ```
//!
//! ## State Diagram
//!
//! ```text
//! Init ──Deploy──> Draft ──ConfigureModel──> Configured ──Activate──> Active
//!                    │                           │                      │ │
//!                    │                           │                      │ │
//!                    └─────Decommission──────────┴───────Suspend────────┘ │
//!                                │                          │             │
//!                                v                          v             │
//!                        Decommissioned <───────────── Suspended ──Resume─┘
//! ```
//!
//! ## Usage
//!
//! ```ignore
//! use cim_domain_agent::state_machine::{
//!     AgentLifecycleMachine, AgentLifecycleState, LifecycleCommand, LifecycleEvent
//! };
//! use cim_domain::formal_domain::MealyStateMachine;
//!
//! let machine = AgentLifecycleMachine::new();
//! let state = AgentLifecycleState::Init;
//!
//! let cmd = LifecycleCommand::Deploy {
//!     agent_id: AgentId::new(),
//!     person_id: PersonId::new(),
//!     name: "MyAgent".to_string(),
//!     description: None,
//! };
//!
//! let (new_state, events) = machine.step(state, cmd);
//! // new_state = Draft
//! // events = [LifecycleEvent::Deployed(...)]
//! ```

mod inputs;
mod lifecycle;
mod outputs;

pub use inputs::LifecycleCommand;
pub use lifecycle::{AgentLifecycleMachine, AgentLifecycleState};
pub use outputs::{
    AgentActivatedOutput, AgentDecommissionedOutput, AgentDeployedOutput, AgentResumedOutput,
    AgentSuspendedOutput, CommandRejectedOutput, LifecycleEvent, ModelConfiguredOutput,
};
