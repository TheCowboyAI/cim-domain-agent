//! Authentication systems for agents
//!
//! This module provides ECS systems for handling agent authentication,
//! including token validation, session management, and authentication state updates.

use bevy_ecs::prelude::*;
use bevy::prelude::{Time, EventReader, EventWriter, Plugin, App, Update};
use chrono::{DateTime, Utc, Duration};
use crate::components::{AgentEntity, AgentCapabilities};
use crate::events::AuthenticationEvent;
use crate::value_objects::{AgentId, AuthToken, SessionId};
use std::collections::HashMap;
use uuid::Uuid;
use tracing::{info, warn, error, debug};

/// Component representing authentication state
#[derive(Component, Debug, Clone)]
pub struct AuthenticationState {
    pub is_authenticated: bool,
    pub session_id: Option<SessionId>,
    pub token: Option<AuthToken>,
    pub expires_at: Option<std::time::SystemTime>,
}

impl Default for AuthenticationState {
    fn default() -> Self {
        Self {
            is_authenticated: false,
            session_id: None,
            token: None,
            expires_at: None,
        }
    }
}

/// Marker component for authenticated agents
#[derive(Component, Debug, Clone)]
pub struct AuthenticatedAgent {
    pub authenticated_at: std::time::SystemTime,
}

/// Resource for managing authentication sessions
#[derive(Resource, Debug, Default)]
pub struct AuthenticationManager {
    sessions: HashMap<SessionId, AgentId>,
    tokens: HashMap<AuthToken, SessionId>,
}

/// Event for authentication requests
#[derive(Event, Debug, Clone)]
pub struct AuthenticationRequest {
    pub agent_id: AgentId,
    pub token: AuthToken,
}

/// Event for authentication responses
#[derive(Event, Debug, Clone)]
pub struct AuthenticationResponse {
    pub agent_id: AgentId,
    pub success: bool,
    pub session_id: Option<SessionId>,
    pub message: String,
}

/// System to handle authentication requests
pub fn handle_authentication_requests(
    mut commands: Commands,
    mut auth_requests: EventReader<AuthenticationRequest>,
    mut auth_responses: EventWriter<AuthenticationResponse>,
    mut auth_manager: ResMut<AuthenticationManager>,
    mut agent_query: Query<(Entity, &AgentEntity, &mut AuthenticationState)>,
) {
    for request in auth_requests.read() {
        // Find the agent entity
        let agent_entity = agent_query
            .iter_mut()
            .find(|(_, agent, _)| AgentId::from_uuid(agent.agent_id) == request.agent_id);

        if let Some((entity, agent, mut auth_state)) = agent_entity {
            // Validate token (simplified for now)
            let is_valid = validate_token(&request.token);

            if is_valid {
                // Create new session
                let session_id = SessionId(Uuid::new_v4());
                
                // Update authentication state
                auth_state.is_authenticated = true;
                auth_state.session_id = Some(session_id.clone());
                auth_state.token = Some(request.token.clone());
                auth_state.expires_at = Some(
                    std::time::SystemTime::now() + std::time::Duration::from_secs(3600)
                );

                // Store session
                auth_manager.sessions.insert(session_id.clone(), request.agent_id.clone());
                auth_manager.tokens.insert(request.token.clone(), session_id.clone());

                // Log authentication for this specific entity
                info!("Agent {:?} authenticated with entity {:?}", agent.agent_id, entity);

                // Add marker component to entity for authenticated agents
                commands.entity(entity).insert(AuthenticatedAgent {
                    authenticated_at: std::time::SystemTime::now(),
                });

                // Send success response
                auth_responses.write(AuthenticationResponse {
                    agent_id: request.agent_id.clone(),
                    success: true,
                    session_id: Some(session_id.clone()),
                    message: "Authentication successful".to_string(),
                });

                // Send authentication event
                commands.trigger(AuthenticationEvent::Authenticated {
                    agent_id: request.agent_id.clone(),
                    session_id,
                });
            } else {
                // Log failed authentication attempt for this entity
                warn!("Failed authentication attempt for agent {:?} on entity {:?}", agent.agent_id, entity);

                // Send failure response
                auth_responses.write(AuthenticationResponse {
                    agent_id: request.agent_id.clone(),
                    success: false,
                    session_id: None,
                    message: "Invalid token".to_string(),
                });

                commands.trigger(AuthenticationEvent::AuthenticationFailed {
                    agent_id: request.agent_id.clone(),
                    reason: "Invalid token".to_string(),
                });
            }
        } else {
            // Agent not found
            auth_responses.write(AuthenticationResponse {
                agent_id: request.agent_id.clone(),
                success: false,
                session_id: None,
                message: "Agent not found".to_string(),
            });
        }
    }
}

/// System to check and update authentication expiry
pub fn check_authentication_expiry(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &AgentEntity, &mut AuthenticationState)>,
    mut auth_manager: ResMut<AuthenticationManager>,
) {
    let current_time = std::time::SystemTime::now();
    let delta_time = time.delta();

    // Log periodic authentication status check
    debug!("Checking authentication expiry - delta time: {:?}", delta_time);

    for (entity, agent, mut auth_state) in &mut query {
        if let Some(expires_at) = auth_state.expires_at {
            if current_time > expires_at && auth_state.is_authenticated {
                // Session expired
                auth_state.is_authenticated = false;
                
                // Calculate how long the session was active
                let session_duration = current_time.duration_since(
                    expires_at - std::time::Duration::from_secs(3600)
                ).unwrap_or_default();
                
                info!("Session expired for agent {:?} after {:?}", 
                    AgentId::from_uuid(agent.agent_id), 
                    session_duration
                );
                
                if let Some(session_id) = &auth_state.session_id {
                    // Remove from manager
                    auth_manager.sessions.remove(session_id);
                    
                    if let Some(token) = &auth_state.token {
                        auth_manager.tokens.remove(token);
                    }
                }

                auth_state.session_id = None;
                auth_state.token = None;
                auth_state.expires_at = None;

                // Remove authenticated marker component
                commands.entity(entity).remove::<AuthenticatedAgent>();

                // Send expiry event
                commands.trigger(AuthenticationEvent::SessionExpired {
                    agent_id: AgentId::from_uuid(agent.agent_id),
                });
            }
        }
    }
}

/// System to handle logout requests
pub fn handle_logout_requests(
    mut commands: Commands,
    mut logout_events: EventReader<LogoutRequest>,
    mut query: Query<(&AgentEntity, &mut AuthenticationState)>,
    mut auth_manager: ResMut<AuthenticationManager>,
) {
    for logout in logout_events.read() {
        if let Some((agent, mut auth_state)) = query
            .iter_mut()
            .find(|(a, _)| AgentId::from_uuid(a.agent_id) == logout.agent_id)
        {
            if auth_state.is_authenticated {
                // Clear authentication
                auth_state.is_authenticated = false;
                
                if let Some(session_id) = &auth_state.session_id {
                    auth_manager.sessions.remove(session_id);
                    
                    if let Some(token) = &auth_state.token {
                        auth_manager.tokens.remove(token);
                    }
                }

                auth_state.session_id = None;
                auth_state.token = None;
                auth_state.expires_at = None;

                // Send logout event
                commands.trigger(AuthenticationEvent::LoggedOut {
                    agent_id: AgentId::from_uuid(agent.agent_id),
                });
            }
        }
    }
}

/// Event for logout requests
#[derive(Event, Debug, Clone)]
pub struct LogoutRequest {
    pub agent_id: AgentId,
}

/// System to sync authentication state with agent capabilities
pub fn sync_auth_with_capabilities(
    mut query: Query<(&AuthenticationState, &mut AgentCapabilities), Changed<AuthenticationState>>,
) {
    for (auth_state, mut capabilities) in &mut query {
        // Update capabilities based on authentication state
        if auth_state.is_authenticated {
            // Enable authenticated capabilities
            capabilities.add("capability.execute".to_string());
            capabilities.add("capability.read".to_string());
            capabilities.add("capability.write".to_string());
        } else {
            // Disable capabilities that require authentication
            capabilities.remove("capability.execute");
            capabilities.remove("capability.read");
            capabilities.remove("capability.write");
        }
    }
}

// Helper function to validate token (simplified)
fn validate_token(token: &AuthToken) -> bool {
    // In a real implementation, this would validate against a token store
    // or external authentication service
    !token.0.is_empty() && token.0.len() > 10
}

/// Plugin to register authentication systems
pub struct AuthenticationPlugin;

impl Plugin for AuthenticationPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<AuthenticationManager>()
            .add_event::<AuthenticationRequest>()
            .add_event::<AuthenticationResponse>()
            .add_event::<LogoutRequest>()
            .add_systems(
                Update,
                (
                    handle_authentication_requests,
                    check_authentication_expiry,
                    handle_logout_requests,
                    sync_auth_with_capabilities,
                ),
            );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_authentication_state_default() {
        let auth_state = AuthenticationState::default();
        assert!(!auth_state.is_authenticated);
        assert!(auth_state.session_id.is_none());
        assert!(auth_state.token.is_none());
        assert!(auth_state.expires_at.is_none());
    }

    #[test]
    fn test_token_validation() {
        assert!(validate_token(&AuthToken("valid_token_12345".to_string())));
        assert!(!validate_token(&AuthToken("short".to_string())));
        assert!(!validate_token(&AuthToken("".to_string())));
    }
}
