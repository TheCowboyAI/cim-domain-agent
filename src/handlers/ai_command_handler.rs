//! AI command handler implementation
//!
//! This module handles AI-specific commands and integrates with AI providers.

use crate::{
    Agent, 
    commands::ai_commands::*,
    aggregate::{AgentMarker, AICapabilitiesComponent},
    ai_providers::{ProviderConfig, AIProviderFactory, GraphAnalysisProvider, GraphData},
};
use cim_domain::{CommandHandler, CommandEnvelope, CommandAcknowledgment, CommandStatus, EntityId};
use cim_domain::AggregateRepository;
use std::sync::Arc;
use tokio::sync::RwLock;

/// AI command handler with provider configuration
pub struct AICommandHandler<R: AggregateRepository<Agent>> {
    repository: R,
    provider_config: ProviderConfig,
    provider: Arc<RwLock<Option<Box<dyn GraphAnalysisProvider>>>>,
}

impl<R: AggregateRepository<Agent>> AICommandHandler<R> {
    /// Create a new AI command handler with provider configuration
    pub fn new(repository: R, provider_config: ProviderConfig) -> Self {
        Self { 
            repository,
            provider_config,
            provider: Arc::new(RwLock::new(None)),
        }
    }
    
    /// Initialize the AI provider based on configuration
    pub async fn initialize_provider(&self) -> Result<(), String> {
        match AIProviderFactory::create_provider(&self.provider_config) {
            Ok(provider) => {
                let mut provider_lock = self.provider.write().await;
                *provider_lock = Some(provider);
                Ok(())
            }
            Err(e) => Err(format!("Failed to initialize AI provider: {}", e))
        }
    }
    
    /// Get or create the AI provider
    async fn get_provider(&self) -> Result<Box<dyn GraphAnalysisProvider>, String> {
        let provider_lock = self.provider.read().await;
        if provider_lock.is_none() {
            drop(provider_lock);
            self.initialize_provider().await?;
        }
        
        AIProviderFactory::create_provider(&self.provider_config)
            .map_err(|e| format!("Failed to create provider: {}", e))
    }
}

impl<R: AggregateRepository<Agent> + Send + Sync> CommandHandler<ConfigureAICapabilities> for AICommandHandler<R> {
    fn handle(&mut self, envelope: CommandEnvelope<ConfigureAICapabilities>) -> CommandAcknowledgment {
        let command = envelope.command;
        
        // Load the agent
        let entity_id = EntityId::<AgentMarker>::from_uuid(command.agent_id.into());
        match self.repository.load(entity_id) {
            Ok(Some(mut agent)) => {
                // Create AI capabilities component
                let ai_component = AICapabilitiesComponent {
                    capabilities: command.capabilities,
                    provider_config: self.provider_config.clone(),
                    last_updated: chrono::Utc::now(),
                };
                
                // Add the component to the agent
                match agent.add_component(ai_component) {
                    Ok(_events) => {
                        // Save the updated agent
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
                                reason: Some(format!("Failed to save agent: {}", e)),
                            }
                        }
                    }
                    Err(e) => CommandAcknowledgment {
                        command_id: envelope.id,
                        correlation_id: envelope.identity.correlation_id.clone(),
                        status: CommandStatus::Rejected,
                        reason: Some(format!("Failed to add AI capabilities: {}", e)),
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
                reason: Some(format!("Failed to load agent: {}", e)),
            }
        }
    }
}

/// Async command handler for graph analysis
pub struct AsyncAICommandHandler<R: AggregateRepository<Agent>> {
    handler: AICommandHandler<R>,
}

impl<R: AggregateRepository<Agent>> AsyncAICommandHandler<R> {
    pub fn new(repository: R, provider_config: ProviderConfig) -> Self {
        Self {
            handler: AICommandHandler::new(repository, provider_config),
        }
    }
    
    /// Handle graph analysis request asynchronously
    pub async fn handle_graph_analysis(
        &mut self, 
        envelope: CommandEnvelope<RequestGraphAnalysis>
    ) -> CommandAcknowledgment {
        let command = envelope.command;
        
        // Load the agent
        let entity_id = EntityId::<AgentMarker>::from_uuid(command.agent_id.into());
        match self.handler.repository.load(entity_id) {
            Ok(Some(agent)) => {
                // Check if agent has AI capabilities
                if !agent.has_ai_capabilities() {
                    return CommandAcknowledgment {
                        command_id: envelope.id,
                        correlation_id: envelope.identity.correlation_id.clone(),
                        status: CommandStatus::Rejected,
                        reason: Some("Agent does not have AI capabilities configured".to_string()),
                    };
                }
                
                // Get the AI provider
                match self.handler.get_provider().await {
                    Ok(provider) => {
                        // Convert graph ID to graph data (this would normally fetch from graph domain)
                        let graph_data = GraphData {
                            graph_id: command.graph_id,
                            nodes: vec![], // Would be populated from graph domain
                            edges: vec![], // Would be populated from graph domain
                            metadata: std::collections::HashMap::new(),
                        };
                        
                        // Check if provider supports the requested capability
                        if !provider.supports_capability(&command.analysis_type) {
                            return CommandAcknowledgment {
                                command_id: envelope.id,
                                correlation_id: envelope.identity.correlation_id.clone(),
                                status: CommandStatus::Rejected,
                                reason: Some(format!(
                                    "Provider does not support capability: {:?}",
                                    command.analysis_type
                                )),
                            };
                        }
                        
                        // Perform the analysis
                        match provider.analyze_graph(
                            graph_data, 
                            command.analysis_type,
                            command.parameters
                        ).await {
                            Ok(result) => {
                                // Store the result (would emit an event in real implementation)
                                CommandAcknowledgment {
                                    command_id: envelope.id,
                                    correlation_id: envelope.identity.correlation_id.clone(),
                                    status: CommandStatus::Accepted,
                                    reason: Some(format!("Analysis completed: {:?}", result.summary)),
                                }
                            }
                            Err(e) => CommandAcknowledgment {
                                command_id: envelope.id,
                                correlation_id: envelope.identity.correlation_id.clone(),
                                status: CommandStatus::Rejected,
                                reason: Some(format!("Analysis failed: {}", e)),
                            }
                        }
                    }
                    Err(e) => CommandAcknowledgment {
                        command_id: envelope.id,
                        correlation_id: envelope.identity.correlation_id.clone(),
                        status: CommandStatus::Rejected,
                        reason: Some(format!("Failed to get AI provider: {}", e)),
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
                reason: Some(format!("Failed to load agent: {}", e)),
            }
        }
    }
    
    /// Handle transformation suggestions request
    pub async fn handle_transformation_suggestions(
        &mut self,
        envelope: CommandEnvelope<RequestTransformationSuggestions>
    ) -> CommandAcknowledgment {
        let command = envelope.command;
        
        // Load the agent
        let entity_id = EntityId::<AgentMarker>::from_uuid(command.agent_id.into());
        match self.handler.repository.load(entity_id) {
            Ok(Some(agent)) => {
                // Check if agent has AI capabilities
                if !agent.has_ai_capabilities() {
                    return CommandAcknowledgment {
                        command_id: envelope.id,
                        correlation_id: envelope.identity.correlation_id.clone(),
                        status: CommandStatus::Rejected,
                        reason: Some("Agent does not have AI capabilities configured".to_string()),
                    };
                }
                
                // Get the AI provider
                match self.handler.get_provider().await {
                    Ok(provider) => {
                        // Convert graph ID to graph data
                        let graph_data = GraphData {
                            graph_id: command.graph_id,
                            nodes: vec![], // Would be populated from graph domain
                            edges: vec![], // Would be populated from graph domain
                            metadata: std::collections::HashMap::new(),
                        };
                        
                        // Get transformation suggestions
                        match provider.suggest_transformations(
                            graph_data,
                            vec![command.purpose],
                            command.constraints
                        ).await {
                            Ok(suggestions) => {
                                CommandAcknowledgment {
                                    command_id: envelope.id,
                                    correlation_id: envelope.identity.correlation_id.clone(),
                                    status: CommandStatus::Accepted,
                                    reason: Some(format!("Generated {} suggestions", suggestions.len())),
                                }
                            }
                            Err(e) => CommandAcknowledgment {
                                command_id: envelope.id,
                                correlation_id: envelope.identity.correlation_id.clone(),
                                status: CommandStatus::Rejected,
                                reason: Some(format!("Failed to generate suggestions: {}", e)),
                            }
                        }
                    }
                    Err(e) => CommandAcknowledgment {
                        command_id: envelope.id,
                        correlation_id: envelope.identity.correlation_id.clone(),
                        status: CommandStatus::Rejected,
                        reason: Some(format!("Failed to get AI provider: {}", e)),
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
                reason: Some(format!("Failed to load agent: {}", e)),
            }
        }
    }
}