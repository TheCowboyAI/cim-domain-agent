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
pub struct AgentCapabilitiesChangedEvent(pub AgentCapabilitiesChanged);

#[derive(Event, Debug, Clone)]
pub struct AgentPermissionsChangedEvent(pub AgentPermissionsChanged);

#[derive(Event, Debug, Clone)]
pub struct AgentToolsChangedEvent(pub AgentToolsChanged);

// AI-specific events
#[derive(Event, Debug, Clone)]
pub struct AICapabilitiesConfiguredEvent(pub AICapabilitiesConfigured);

#[derive(Event, Debug, Clone)]
pub struct GraphAnalysisCompletedEvent(pub GraphAnalysisCompleted);

#[derive(Event, Debug, Clone)]
pub struct TransformationSuggestionsGeneratedEvent(pub TransformationSuggestionsGenerated);

#[derive(Event, Debug, Clone)]
pub struct AIRecommendationsExecutedEvent(pub AIRecommendationsExecuted);

#[derive(Event, Debug, Clone)]
pub struct AgentTrainedOnPatternsEvent(pub AgentTrainedOnPatterns);

#[derive(Event, Debug, Clone)]
pub struct AIAnalysisFailedEvent(pub AIAnalysisFailed); 