//! Agent command handler implementation

use crate::{Agent, commands::*};
use cim_core_domain::command::CommandHandler;
use cim_core_domain::repository::AggregateRepository;
use async_trait::async_trait;

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

#[async_trait]
impl<R: AggregateRepository<Agent> + Send + Sync> CommandHandler<DeployAgent> for AgentCommandHandler<R> {
    type Error = cim_core_domain::errors::DomainError;

    async fn handle(&self, command: DeployAgent) -> Result<(), Self::Error> {
        let mut agent = Agent::new(command.id, command.agent_type, command.owner_id);

        // Add metadata component
        agent.add_component(command.metadata)?;

        self.repository.save(&agent).await?;
        Ok(())
    }
}

#[async_trait]
impl<R: AggregateRepository<Agent> + Send + Sync> CommandHandler<ActivateAgent> for AgentCommandHandler<R> {
    type Error = cim_core_domain::errors::DomainError;

    async fn handle(&self, command: ActivateAgent) -> Result<(), Self::Error> {
        let mut agent = self.repository.load(&command.id.into()).await?;
        agent.activate()?;
        self.repository.save(&agent).await?;
        Ok(())
    }
}

// Additional command handlers would be implemented similarly...
