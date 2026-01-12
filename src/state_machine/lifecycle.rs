// Copyright (c) 2025 - Cowboy AI, LLC.

//! Agent Lifecycle State Machine
//!
//! Implements MealyStateMachine for agent lifecycle management.
//! The state machine defines valid transitions and produces events.
//!
//! ```text
//! ┌─────────┐    Deploy     ┌─────────┐
//! │  Init   │──────────────>│  Draft  │
//! └─────────┘               └────┬────┘
//!                                │ ConfigureModel
//!                                v
//!                          ┌────────────┐
//!                          │ Configured │<───────────┐
//!                          └─────┬──────┘            │
//!                                │ Activate          │
//!                                v                   │
//!                          ┌─────────┐     Resume    │
//!                       ┌─>│ Active  │<──────────┐   │
//!                       │  └────┬────┘           │   │
//!                       │       │ Suspend        │   │
//!                       │       v                │   │
//!                       │  ┌───────────┐         │   │
//!                       │  │ Suspended │─────────┘   │
//!                       │  └─────┬─────┘             │
//!       ConfigureModel  │        │                   │
//!       (in Active)     └────────┼───────────────────┘
//!                                │ Decommission (from any non-terminal)
//!                                v
//!                        ┌───────────────┐
//!                        │ Decommissioned│ (Terminal)
//!                        └───────────────┘
//! ```

use crate::state_machine::inputs::LifecycleCommand;
use crate::state_machine::outputs::*;
use crate::value_objects::ModelConfig;
use cim_domain::formal_domain::{AggregateState, DomainCommand, MealyStateMachine};
use serde::{Deserialize, Serialize};

/// Agent lifecycle states
///
/// Represents the possible states an agent can be in during its lifecycle.
/// The state machine enforces valid transitions between states.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AgentLifecycleState {
    /// Initial state - agent has not been deployed yet
    Init,

    /// Agent deployed but no model configured
    Draft,

    /// Agent has a model configured but is not active
    Configured { model: ModelConfig },

    /// Agent is operational and can process messages
    Active { model: ModelConfig },

    /// Agent is temporarily paused
    Suspended { model: ModelConfig, reason: String },

    /// Agent is permanently decommissioned (terminal state)
    Decommissioned { reason: Option<String> },
}

impl AgentLifecycleState {
    /// Check if the agent can process messages in this state
    pub fn is_operational(&self) -> bool {
        matches!(self, Self::Active { .. })
    }

    /// Check if this is a terminal state
    pub fn is_terminal(&self) -> bool {
        matches!(self, Self::Decommissioned { .. })
    }

    /// Get the model config if present
    pub fn model_config(&self) -> Option<&ModelConfig> {
        match self {
            Self::Configured { model } => Some(model),
            Self::Active { model } => Some(model),
            Self::Suspended { model, .. } => Some(model),
            _ => None,
        }
    }

    /// Check if model can be configured in this state
    pub fn can_configure_model(&self) -> bool {
        matches!(
            self,
            Self::Draft | Self::Configured { .. } | Self::Active { .. }
        )
    }

    /// Check if agent can be activated in this state
    pub fn can_activate(&self) -> bool {
        matches!(self, Self::Configured { .. } | Self::Suspended { .. })
    }

    /// Check if agent can be suspended in this state
    pub fn can_suspend(&self) -> bool {
        matches!(self, Self::Active { .. })
    }
}

impl AggregateState for AgentLifecycleState {
    fn all_states() -> Vec<Self> {
        vec![
            Self::Init,
            Self::Draft,
            Self::Configured {
                model: ModelConfig::mock(),
            },
            Self::Active {
                model: ModelConfig::mock(),
            },
            Self::Suspended {
                model: ModelConfig::mock(),
                reason: String::new(),
            },
            Self::Decommissioned { reason: None },
        ]
    }

    fn initial() -> Self {
        Self::Init
    }

    fn is_terminal(&self) -> bool {
        AgentLifecycleState::is_terminal(self)
    }

    fn can_transition_to(&self, other: &Self) -> bool {
        match (self, other) {
            // From Init
            (Self::Init, Self::Draft) => true,

            // From Draft
            (Self::Draft, Self::Configured { .. }) => true,
            (Self::Draft, Self::Decommissioned { .. }) => true,

            // From Configured
            (Self::Configured { .. }, Self::Configured { .. }) => true, // Reconfigure
            (Self::Configured { .. }, Self::Active { .. }) => true,
            (Self::Configured { .. }, Self::Decommissioned { .. }) => true,

            // From Active
            (Self::Active { .. }, Self::Configured { .. }) => true, // Reconfigure
            (Self::Active { .. }, Self::Suspended { .. }) => true,
            (Self::Active { .. }, Self::Decommissioned { .. }) => true,

            // From Suspended
            (Self::Suspended { .. }, Self::Active { .. }) => true, // Resume
            (Self::Suspended { .. }, Self::Decommissioned { .. }) => true,

            // From Decommissioned - terminal, no transitions
            (Self::Decommissioned { .. }, _) => false,

            // All other transitions are invalid
            _ => false,
        }
    }
}

/// Agent Lifecycle State Machine
///
/// Implements MealyStateMachine from cim-domain. The machine is stateless -
/// state is passed in with each call, and the machine computes the next
/// state and output events.
#[derive(Debug, Clone, Default)]
pub struct AgentLifecycleMachine;

impl AgentLifecycleMachine {
    /// Create a new lifecycle state machine
    pub fn new() -> Self {
        Self
    }
}

impl MealyStateMachine for AgentLifecycleMachine {
    type State = AgentLifecycleState;
    type Input = LifecycleCommand;
    type Output = Vec<LifecycleEvent>;

    fn transition(&self, state: Self::State, input: Self::Input) -> Self::State {
        match (&state, &input) {
            // Deploy: Init -> Draft
            (AgentLifecycleState::Init, LifecycleCommand::Deploy { .. }) => {
                AgentLifecycleState::Draft
            }

            // ConfigureModel: Draft/Configured/Active -> Configured
            (
                AgentLifecycleState::Draft,
                LifecycleCommand::ConfigureModel { config, .. },
            ) => AgentLifecycleState::Configured {
                model: config.clone(),
            },
            (
                AgentLifecycleState::Configured { .. },
                LifecycleCommand::ConfigureModel { config, .. },
            ) => AgentLifecycleState::Configured {
                model: config.clone(),
            },
            (
                AgentLifecycleState::Active { .. },
                LifecycleCommand::ConfigureModel { config, .. },
            ) => AgentLifecycleState::Active {
                model: config.clone(),
            },

            // Activate: Configured/Suspended -> Active
            (AgentLifecycleState::Configured { model }, LifecycleCommand::Activate { .. }) => {
                AgentLifecycleState::Active {
                    model: model.clone(),
                }
            }
            (AgentLifecycleState::Suspended { model, .. }, LifecycleCommand::Resume { .. }) => {
                AgentLifecycleState::Active {
                    model: model.clone(),
                }
            }

            // Suspend: Active -> Suspended
            (
                AgentLifecycleState::Active { model },
                LifecycleCommand::Suspend { reason, .. },
            ) => AgentLifecycleState::Suspended {
                model: model.clone(),
                reason: reason.clone(),
            },

            // Decommission: Any non-terminal -> Decommissioned
            (
                AgentLifecycleState::Draft,
                LifecycleCommand::Decommission { reason, .. },
            )
            | (
                AgentLifecycleState::Configured { .. },
                LifecycleCommand::Decommission { reason, .. },
            )
            | (
                AgentLifecycleState::Active { .. },
                LifecycleCommand::Decommission { reason, .. },
            )
            | (
                AgentLifecycleState::Suspended { .. },
                LifecycleCommand::Decommission { reason, .. },
            ) => AgentLifecycleState::Decommissioned {
                reason: reason.clone(),
            },

            // Invalid transitions - state unchanged
            _ => state,
        }
    }

    fn output(&self, state: Self::State, input: Self::Input) -> Self::Output {
        match (&state, &input) {
            // Deploy: Init -> Draft
            (
                AgentLifecycleState::Init,
                LifecycleCommand::Deploy {
                    agent_id,
                    person_id,
                    name,
                    description,
                },
            ) => vec![LifecycleEvent::Deployed(AgentDeployedOutput::new(
                *agent_id,
                *person_id,
                name.clone(),
                description.clone(),
            ))],

            // ConfigureModel: Draft/Configured/Active -> produce ModelConfigured
            (
                AgentLifecycleState::Draft
                | AgentLifecycleState::Configured { .. }
                | AgentLifecycleState::Active { .. },
                LifecycleCommand::ConfigureModel { agent_id, config },
            ) => vec![LifecycleEvent::ModelConfigured(ModelConfiguredOutput::new(
                *agent_id,
                config.clone(),
            ))],

            // Activate: Configured -> Active
            (
                AgentLifecycleState::Configured { .. },
                LifecycleCommand::Activate { agent_id },
            ) => vec![LifecycleEvent::Activated(AgentActivatedOutput::new(
                *agent_id,
            ))],

            // Resume: Suspended -> Active
            (AgentLifecycleState::Suspended { .. }, LifecycleCommand::Resume { agent_id }) => {
                vec![LifecycleEvent::Resumed(AgentResumedOutput::new(*agent_id))]
            }

            // Suspend: Active -> Suspended
            (
                AgentLifecycleState::Active { .. },
                LifecycleCommand::Suspend { agent_id, reason },
            ) => vec![LifecycleEvent::Suspended(AgentSuspendedOutput::new(
                *agent_id,
                reason.clone(),
            ))],

            // Decommission: Any non-terminal -> Decommissioned
            (
                AgentLifecycleState::Draft
                | AgentLifecycleState::Configured { .. }
                | AgentLifecycleState::Active { .. }
                | AgentLifecycleState::Suspended { .. },
                LifecycleCommand::Decommission { agent_id, reason },
            ) => vec![LifecycleEvent::Decommissioned(
                AgentDecommissionedOutput::new(*agent_id, reason.clone()),
            )],

            // Invalid transitions produce rejection events
            (state, cmd) => {
                let reason = match state {
                    AgentLifecycleState::Init => "Agent not yet deployed",
                    AgentLifecycleState::Draft => "Agent not yet configured",
                    AgentLifecycleState::Configured { .. } => "Invalid command for Configured state",
                    AgentLifecycleState::Active { .. } => "Invalid command for Active state",
                    AgentLifecycleState::Suspended { .. } => "Invalid command for Suspended state",
                    AgentLifecycleState::Decommissioned { .. } => {
                        "Agent is decommissioned - no commands allowed"
                    }
                };
                vec![LifecycleEvent::CommandRejected(CommandRejectedOutput::new(
                    cmd.agent_id(),
                    cmd.name(),
                    reason,
                ))]
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::value_objects::{AgentId, PersonId};

    fn create_test_ids() -> (AgentId, PersonId) {
        (AgentId::new(), PersonId::new())
    }

    #[test]
    fn test_deploy_transition() {
        let machine = AgentLifecycleMachine::new();
        let (agent_id, person_id) = create_test_ids();

        let cmd = LifecycleCommand::Deploy {
            agent_id,
            person_id,
            name: "TestAgent".to_string(),
            description: None,
        };

        let (new_state, events) = machine.step(AgentLifecycleState::Init, cmd);

        assert_eq!(new_state, AgentLifecycleState::Draft);
        assert_eq!(events.len(), 1);
        assert!(matches!(events[0], LifecycleEvent::Deployed(_)));
    }

    #[test]
    fn test_configure_model_transition() {
        let machine = AgentLifecycleMachine::new();
        let (agent_id, _) = create_test_ids();

        let cmd = LifecycleCommand::ConfigureModel {
            agent_id,
            config: ModelConfig::mock(),
        };

        let (new_state, events) = machine.step(AgentLifecycleState::Draft, cmd);

        assert!(matches!(new_state, AgentLifecycleState::Configured { .. }));
        assert_eq!(events.len(), 1);
        assert!(matches!(events[0], LifecycleEvent::ModelConfigured(_)));
    }

    #[test]
    fn test_activate_transition() {
        let machine = AgentLifecycleMachine::new();
        let (agent_id, _) = create_test_ids();
        let config = ModelConfig::mock();

        let state = AgentLifecycleState::Configured {
            model: config.clone(),
        };
        let cmd = LifecycleCommand::Activate { agent_id };

        let (new_state, events) = machine.step(state, cmd);

        assert!(matches!(new_state, AgentLifecycleState::Active { .. }));
        assert_eq!(events.len(), 1);
        assert!(matches!(events[0], LifecycleEvent::Activated(_)));
    }

    #[test]
    fn test_suspend_and_resume() {
        let machine = AgentLifecycleMachine::new();
        let (agent_id, _) = create_test_ids();
        let config = ModelConfig::mock();

        // Suspend
        let state = AgentLifecycleState::Active {
            model: config.clone(),
        };
        let cmd = LifecycleCommand::Suspend {
            agent_id,
            reason: "Maintenance".to_string(),
        };
        let (suspended_state, events) = machine.step(state, cmd);

        assert!(matches!(suspended_state, AgentLifecycleState::Suspended { .. }));
        assert!(matches!(events[0], LifecycleEvent::Suspended(_)));

        // Resume
        let cmd = LifecycleCommand::Resume { agent_id };
        let (active_state, events) = machine.step(suspended_state, cmd);

        assert!(matches!(active_state, AgentLifecycleState::Active { .. }));
        assert!(matches!(events[0], LifecycleEvent::Resumed(_)));
    }

    #[test]
    fn test_decommission_from_any_state() {
        let machine = AgentLifecycleMachine::new();
        let (agent_id, _) = create_test_ids();
        let config = ModelConfig::mock();

        // From Draft
        let cmd = LifecycleCommand::Decommission {
            agent_id,
            reason: Some("Test".to_string()),
        };
        let (state, events) = machine.step(AgentLifecycleState::Draft, cmd.clone());
        assert!(matches!(state, AgentLifecycleState::Decommissioned { .. }));
        assert!(matches!(events[0], LifecycleEvent::Decommissioned(_)));

        // From Active
        let active = AgentLifecycleState::Active { model: config };
        let (state, _) = machine.step(active, cmd);
        assert!(matches!(state, AgentLifecycleState::Decommissioned { .. }));
    }

    #[test]
    fn test_invalid_transition_produces_rejection() {
        let machine = AgentLifecycleMachine::new();
        let (agent_id, _) = create_test_ids();

        // Try to activate from Draft (no model configured)
        let cmd = LifecycleCommand::Activate { agent_id };
        let (state, events) = machine.step(AgentLifecycleState::Draft, cmd);

        // State unchanged
        assert_eq!(state, AgentLifecycleState::Draft);
        // Rejection event produced
        assert_eq!(events.len(), 1);
        assert!(matches!(events[0], LifecycleEvent::CommandRejected(_)));
    }

    #[test]
    fn test_decommissioned_is_terminal() {
        let machine = AgentLifecycleMachine::new();
        let (agent_id, _) = create_test_ids();

        let state = AgentLifecycleState::Decommissioned {
            reason: Some("Test".to_string()),
        };
        let cmd = LifecycleCommand::Activate { agent_id };

        let (new_state, events) = machine.step(state.clone(), cmd);

        // State unchanged
        assert_eq!(new_state, state);
        // Rejection event produced
        assert!(matches!(events[0], LifecycleEvent::CommandRejected(_)));
    }

    #[test]
    fn test_reconfigure_model_in_active_state() {
        let machine = AgentLifecycleMachine::new();
        let (agent_id, _) = create_test_ids();
        let original_config = ModelConfig::mock();
        let new_config = ModelConfig::mock().with_system_prompt("New prompt");

        let state = AgentLifecycleState::Active {
            model: original_config,
        };
        let cmd = LifecycleCommand::ConfigureModel {
            agent_id,
            config: new_config.clone(),
        };

        let (new_state, events) = machine.step(state, cmd);

        // Should stay Active with new config
        if let AgentLifecycleState::Active { model } = new_state {
            assert_eq!(model.system_prompt, new_config.system_prompt);
        } else {
            panic!("Expected Active state");
        }
        assert!(matches!(events[0], LifecycleEvent::ModelConfigured(_)));
    }
}
