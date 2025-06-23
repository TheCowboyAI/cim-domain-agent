//! Agent command handler implementation

use crate::{Agent, commands::*, aggregate::AgentMarker};
use cim_domain::{CommandHandler, CommandEnvelope, CommandAcknowledgment, CommandStatus, EntityId};
use cim_domain::AggregateRepository;

/// Agent command handler
pub struct AgentCommandHandler<R: AggregateRepository<Agent>> {
    repository: R,
}

impl<R: AggregateRepository<Agent>> AgentCommandHandler<R> {
    /// Create a new agent command handler
    pub fn new(repository: R) -> Self {
        Self { repository }
    }
}

impl<R: AggregateRepository<Agent> + Send + Sync> CommandHandler<DeployAgent> for AgentCommandHandler<R> {
    fn handle(&mut self, envelope: CommandEnvelope<DeployAgent>) -> CommandAcknowledgment {
        let command = envelope.command;

        let mut agent = Agent::new(command.id, command.agent_type, command.owner_id);

        // Add metadata component
        match agent.add_component(command.metadata) {
            Ok(_events) => {
                // Save the agent to repository
                match self.repository.save(&agent) {
                    Ok(_) => CommandAcknowledgment {
                        command_id: envelope.id,
                        correlation_id: envelope.identity.correlation_id.clone(),
                        status: CommandStatus::Accepted,
                        reason: None,
                    },
                    Err(e) => CommandAcknowledgment {
                        command_id: envelope.id,
                        correlation_id: envelope.identity.correlation_id.clone(),
                        status: CommandStatus::Rejected,
                        reason: Some(format!("Failed to save agent: {e}")),
                    }
                }
            }
            Err(e) => {
                CommandAcknowledgment {
                    command_id: envelope.id,
                    correlation_id: envelope.identity.correlation_id.clone(),
                    status: CommandStatus::Rejected,
                    reason: Some(e.to_string()),
                }
            }
        }
    }
}

impl<R: AggregateRepository<Agent> + Send + Sync> CommandHandler<ActivateAgent> for AgentCommandHandler<R> {
    fn handle(&mut self, envelope: CommandEnvelope<ActivateAgent>) -> CommandAcknowledgment {
        let command = envelope.command;

        // Load the agent from repository
        let entity_id = EntityId::<AgentMarker>::from_uuid(command.id);
        match self.repository.load(entity_id) {
            Ok(Some(mut agent)) => {
                // Activate the agent
                match agent.activate() {
                    Ok(_events) => {
                        // Save the agent back to repository
                        match self.repository.save(&agent) {
                            Ok(_) => CommandAcknowledgment {
                                command_id: envelope.id,
                                correlation_id: envelope.identity.correlation_id.clone(),
                                status: CommandStatus::Accepted,
                                reason: None,
                            },
                            Err(e) => CommandAcknowledgment {
                                command_id: envelope.id,
                                correlation_id: envelope.identity.correlation_id.clone(),
                                status: CommandStatus::Rejected,
                                reason: Some(format!("Failed to save agent: {e}")),
                            }
                        }
                    }
                    Err(e) => CommandAcknowledgment {
                        command_id: envelope.id,
                        correlation_id: envelope.identity.correlation_id.clone(),
                        status: CommandStatus::Rejected,
                        reason: Some(format!("Failed to activate agent: {e}")),
                    }
                }
            }
            Ok(None) => CommandAcknowledgment {
                command_id: envelope.id,
                correlation_id: envelope.identity.correlation_id.clone(),
                status: CommandStatus::Rejected,
                reason: Some("Agent not found".to_string()),
            },
            Err(e) => CommandAcknowledgment {
                command_id: envelope.id,
                correlation_id: envelope.identity.correlation_id.clone(),
                status: CommandStatus::Rejected,
                reason: Some(format!("Failed to load agent: {e}")),
            }
        }
    }
}

// Additional command handlers would be implemented similarly...
