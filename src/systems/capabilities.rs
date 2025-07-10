//! Capabilities management systems

use bevy::prelude::*;
use crate::components::*;
use crate::events::*;
use uuid::Uuid;

/// System for adding capabilities to agents
///
/// ```mermaid
/// graph LR
///     A[ChangeAgentCapabilitiesCommand] --> B[manage_capabilities_system]
///     B --> C{Agent Found?}
///     C -->|Yes| D[Update Capabilities]
///     C -->|No| E[Log Error]
///     D --> F[Emit Events]
/// ```
pub fn manage_capabilities_system(
    mut capability_events: EventReader<ChangeAgentCapabilitiesCommand>,
    mut agent_query: Query<(&AgentEntity, &mut AgentCapabilities)>,
    mut changed_events: EventWriter<AgentCapabilitiesChanged>,
) {
    for capability_cmd in capability_events.read() {
        // Find the agent
        let agent_found = agent_query.iter_mut()
            .find(|(entity, _)| entity.agent_id == capability_cmd.agent_id);

        if let Some((_, mut capabilities)) = agent_found {
            // Add new capabilities
            let mut actually_added = Vec::new();
            for cap in &capability_cmd.capabilities_to_add {
                if !capabilities.has(cap) {
                    capabilities.add(cap.clone());
                    actually_added.push(cap.clone());
                }
            }

            // Remove capabilities
            let mut actually_removed = Vec::new();
            for cap in &capability_cmd.capabilities_to_remove {
                if capabilities.remove(cap) {
                    actually_removed.push(cap.clone());
                }
            }

            // Emit event for changes
            if !actually_added.is_empty() || !actually_removed.is_empty() {
                changed_events.write(AgentCapabilitiesChanged {
                    agent_id: crate::value_objects::AgentId::from_uuid(capability_cmd.agent_id),
                    added: actually_added,
                    removed: actually_removed,
                    changed_at: chrono::Utc::now(),
                });
            }
        }
    }
}

/// System for updating capability usage statistics
///
/// ```mermaid
/// graph LR
///     A[CapabilityUsedEvent] --> B[update_capability_usage_system]
///     B --> C[Find Agent]
///     C --> D[Update Stats]
///     D --> E[Calculate Metrics]
/// ```
pub fn update_capability_usage_system(
    mut usage_events: EventReader<CapabilityUsedEvent>,
    mut agent_query: Query<(Entity, &AgentEntity, Option<&mut CapabilityUsageStats>)>,
    mut commands: Commands,
) {
    for usage_event in usage_events.read() {
        // Find the agent
        let agent_found = agent_query.iter_mut()
            .find(|(_, agent_entity, _)| agent_entity.agent_id == usage_event.agent_id);

        if let Some((entity, agent_entity, usage_stats)) = agent_found {
            if let Some(mut stats) = usage_stats {
                // Update existing stats
                let count = {
                    let cap_stats = stats.usage_count
                        .entry(usage_event.capability.clone())
                        .or_insert(0);
                    *cap_stats += 1;
                    *cap_stats
                };

                stats.last_used.insert(
                    usage_event.capability.clone(),
                    usage_event.used_at
                );

                // Update success rate
                let current_rate = stats.success_rate
                    .get(&usage_event.capability)
                    .copied()
                    .unwrap_or(0.0);
                
                let new_rate = if usage_event.success {
                    (current_rate * (count as f32 - 1.0) + 1.0) / count as f32
                } else {
                    (current_rate * (count as f32 - 1.0)) / count as f32
                };
                
                stats.success_rate.insert(usage_event.capability.clone(), new_rate);
            } else {
                // Create new stats component
                let mut new_stats = CapabilityUsageStats::default();
                new_stats.usage_count.insert(usage_event.capability.clone(), 1);
                new_stats.last_used.insert(usage_event.capability.clone(), usage_event.used_at);
                new_stats.success_rate.insert(
                    usage_event.capability.clone(),
                    if usage_event.success { 1.0 } else { 0.0 }
                );

                // Add the component to the entity
                commands.entity(entity).insert(new_stats);
            }
        }
    }
}

/// System for checking capability requirements
///
/// ```mermaid
/// graph LR
///     A[Timer] --> B[check_capability_requirements_system]
///     B --> C[For Each Agent]
///     C --> D[Check Requirements]
///     D --> E{Met?}
///     E -->|No| F[Update Readiness]
/// ```
pub fn check_capability_requirements_system(
    mut agent_query: Query<(
        &AgentEntity,
        &AgentCapabilities,
        Option<&CapabilityRequirements>,
        &mut AgentReadiness,
    )>,
) {
    for (_entity, capabilities, requirements, mut readiness) in agent_query.iter_mut() {
        if let Some(reqs) = requirements {
            // Check required capabilities
            let missing_required: Vec<_> = reqs.required
                .iter()
                .filter(|req| !capabilities.has(req))
                .cloned()
                .collect();

            // Check incompatible capabilities
            let has_incompatible: Vec<_> = reqs.incompatible
                .iter()
                .filter(|inc| capabilities.has(inc))
                .cloned()
                .collect();

            // Update readiness check
            let status = if missing_required.is_empty() && has_incompatible.is_empty() {
                CheckStatus::Passed
            } else {
                CheckStatus::Failed
            };

            let message = if !missing_required.is_empty() {
                format!("Missing required capabilities: {missing_required:?}")
            } else if !has_incompatible.is_empty() {
                format!("Has incompatible capabilities: {has_incompatible:?}")
            } else {
                "All capability requirements met".to_string()
            };

            readiness.update_check(
                "capability_requirements".to_string(),
                status,
                message
            );
        }
    }
}

/// System for categorizing agent capabilities
///
/// ```mermaid
/// graph LR
///     A[CapabilityCategorizeCommand] --> B[categorize_capabilities_system]
///     B --> C[Find Agent]
///     C --> D[Update Categories]
///     D --> E[Store Categories]
/// ```
pub fn categorize_capabilities_system(
    mut categorize_events: EventReader<CapabilityCategorizeCommand>,
    mut agent_query: Query<(Entity, &AgentEntity, &AgentCapabilities, Option<&mut CapabilityCategories>)>,
    mut commands: Commands,
) {
    for categorize_cmd in categorize_events.read() {
        // Find the agent
        let agent_found = agent_query.iter_mut()
            .find(|(_, agent_entity, _, _)| agent_entity.agent_id == categorize_cmd.agent_id);

        if let Some((entity, _, _capabilities, categories)) = agent_found {
            if let Some(mut cats) = categories {
                // Update existing categories
                cats.add_to_category(
                    categorize_cmd.category.clone(),
                    categorize_cmd.capability.clone()
                );
            } else {
                // Create new categories component
                let mut new_cats = CapabilityCategories::default();
                new_cats.add_to_category(
                    categorize_cmd.category.clone(),
                    categorize_cmd.capability.clone()
                );
                
                // Find the entity in the query and add the component
                if let Some((entity, agent_entity, _, _)) = agent_query.iter()
                    .find(|(_, agent_entity, _, _)| agent_entity.agent_id == categorize_cmd.agent_id) {
                    commands.entity(entity).insert(new_cats);
                }
            }
        }
    }
}

/// Sync agent capabilities with domain events
pub fn sync_agent_capabilities(
    mut events: EventReader<AgentCapabilitiesChanged>,
    mut query: Query<(&AgentEntity, &mut AgentCapabilities)>,
) {
    for event in events.read() {
        // Find the agent with matching ID
        for (agent_entity, mut capabilities) in query.iter_mut() {
            if agent_entity.agent_id == *event.agent_id.as_uuid() {
                // Add new capabilities
                for cap in &event.added {
                    capabilities.add(cap.clone());
                }
                
                // Remove capabilities
                for cap in &event.removed {
                    capabilities.remove(cap);
                }
            }
        }
    }
}

// Command events for capability management
#[derive(Event)]
pub struct ChangeAgentCapabilitiesCommand {
    pub agent_id: Uuid,
    pub capabilities_to_add: Vec<String>,
    pub capabilities_to_remove: Vec<String>,
}

#[derive(Event)]
pub struct CapabilityUsedEvent {
    pub agent_id: Uuid,
    pub capability: String,
    pub success: bool,
    pub used_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Event)]
pub struct CapabilityCategorizeCommand {
    pub agent_id: Uuid,
    pub capability: String,
    pub category: String,
} 