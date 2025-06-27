//! ECS event wrappers for domain events

use bevy_ecs::prelude::*;
use super::*;

// Re-export domain events with ECS Event trait
#[derive(Event, Debug, Clone)]
pub struct AgentDeployedEvent(pub AgentDeployed);

#[derive(Event, Debug, Clone)]
pub struct AgentActivatedEvent(pub AgentActivated);

#[derive(Event, Debug, Clone)]
pub struct AgentSuspendedEvent(pub AgentSuspended);

#[derive(Event, Debug, Clone)]
pub struct AgentWentOfflineEvent(pub AgentWentOffline);

#[derive(Event, Debug, Clone)]
pub struct AgentDecommissionedEvent(pub AgentDecommissioned);

#[derive(Event, Debug, Clone)]
pub struct AgentCapabilitiesAddedEvent(pub AgentCapabilitiesAdded);

#[derive(Event, Debug, Clone)]
pub struct AgentCapabilitiesRemovedEvent(pub AgentCapabilitiesRemoved);

#[derive(Event, Debug, Clone)]
pub struct AgentPermissionsGrantedEvent(pub AgentPermissionsGranted);

#[derive(Event, Debug, Clone)]
pub struct AgentPermissionsRevokedEvent(pub AgentPermissionsRevoked);

#[derive(Event, Debug, Clone)]
pub struct AgentToolsEnabledEvent(pub AgentToolsEnabled);

#[derive(Event, Debug, Clone)]
pub struct AgentToolsDisabledEvent(pub AgentToolsDisabled);

#[derive(Event, Debug, Clone)]
pub struct AgentConfigurationRemovedEvent(pub AgentConfigurationRemoved);

#[derive(Event, Debug, Clone)]
pub struct AgentConfigurationSetEvent(pub AgentConfigurationSet); 