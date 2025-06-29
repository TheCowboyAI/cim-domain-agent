//! Agent command handler implementation

use crate::{Agent, commands::*, aggregate::{AgentMarker, AgentType}};
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

        // Convert value object AgentType to aggregate AgentType
        let aggregate_agent_type = match command.agent_type {
            crate::value_objects::AgentType::User => AgentType::Human,
            crate::value_objects::AgentType::AI => AgentType::AI,
            crate::value_objects::AgentType::System => AgentType::System,
            crate::value_objects::AgentType::Integration => AgentType::External,
            _ => AgentType::System, // Default for other types
        };

        let mut agent = Agent::new(
            command.id.into(), 
            aggregate_agent_type, 
            command.owner_id
        );

        // Create metadata from command fields
        let metadata = crate::aggregate::AgentMetadata {
            name: command.name,
            description: command.description.unwrap_or_default(),
            tags: std::collections::HashSet::new(),
            created_at: chrono::Utc::now(),
            last_active: None,
        };

        // Add metadata component
        match agent.add_component(metadata) {
            Ok(_events) => {
                // Add initial capabilities if any
                if !command.initial_capabilities.is_empty() {
                    let capabilities_component = crate::aggregate::CapabilitiesComponent::new(
                        command.initial_capabilities
                    );
                    if let Err(e) = agent.add_component(capabilities_component) {
                        return CommandAcknowledgment {
                            command_id: envelope.id,
                            correlation_id: envelope.identity.correlation_id.clone(),
                            status: CommandStatus::Rejected,
                            reason: Some(format!("Failed to add capabilities: {e}")),
                        };
                    }
                }

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
        let entity_id = EntityId::<AgentMarker>::from_uuid(command.id.into());
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
