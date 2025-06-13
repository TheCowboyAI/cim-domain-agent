//! Agent command handler implementation

use crate::{Agent, commands::*};
use cim_domain::{CommandHandler, CommandEnvelope, CommandAcknowledgment, CommandStatus};
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
            Ok(_) => {
                // In a real implementation, we would publish events here
                CommandAcknowledgment {
                    command_id: envelope.id,
                    correlation_id: envelope.correlation_id,
                    status: CommandStatus::Accepted,
                    reason: None,
                }
            }
            Err(e) => {
                CommandAcknowledgment {
                    command_id: envelope.id,
                    correlation_id: envelope.correlation_id,
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

        // In a real implementation, we would load the agent from repository
        // For now, just return acknowledgment
        CommandAcknowledgment {
            command_id: envelope.id,
            correlation_id: envelope.correlation_id,
            status: CommandStatus::Accepted,
            reason: None,
        }
    }
}

// Additional command handlers would be implemented similarly...
