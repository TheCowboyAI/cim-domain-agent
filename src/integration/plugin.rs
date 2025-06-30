//! Bevy plugin for agent domain

use bevy::prelude::*;
use crate::systems::*;

/// Plugin that integrates agent domain with Bevy
pub struct AgentPlugin;

impl Plugin for AgentPlugin {
    fn build(&self, app: &mut App) {
        app
            // Add agent systems
            .add_systems(Update, (
                update_agent_status,
                sync_agent_capabilities,
            ))
            // Add agent events
            .add_event::<crate::events::AgentDeployed>()
            .add_event::<crate::events::AgentActivated>()
            .add_event::<crate::events::AgentSuspended>()
            .add_event::<crate::events::AgentDecommissioned>()
            .add_event::<crate::events::AgentWentOffline>()
            .add_event::<crate::events::AgentCapabilitiesChanged>()
            .add_event::<crate::events::AgentPermissionsChanged>()
            .add_event::<crate::events::AgentToolsChanged>();
    }
} 