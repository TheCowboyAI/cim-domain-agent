//! Authentication-related events for the agent domain

use serde::{Deserialize, Serialize};
use bevy::prelude::Event;
use crate::value_objects::{AgentId, SessionId};

/// Events related to agent authentication
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Event)]
pub enum AuthenticationEvent {
    /// Agent successfully authenticated
    Authenticated {
        agent_id: AgentId,
        session_id: SessionId,
    },
    
    /// Authentication attempt failed
    AuthenticationFailed {
        agent_id: AgentId,
        reason: String,
    },
    
    /// Agent logged out
    LoggedOut {
        agent_id: AgentId,
    },
    
    /// Session expired
    SessionExpired {
        agent_id: AgentId,
    },
}

impl AuthenticationEvent {
    /// Get the agent ID associated with this event
    pub fn agent_id(&self) -> &AgentId {
        match self {
            Self::Authenticated { agent_id, .. } => agent_id,
            Self::AuthenticationFailed { agent_id, .. } => agent_id,
            Self::LoggedOut { agent_id } => agent_id,
            Self::SessionExpired { agent_id } => agent_id,
        }
    }
    
    /// Get event type as string for logging/monitoring
    pub fn event_type(&self) -> &'static str {
        match self {
            Self::Authenticated { .. } => "authenticated",
            Self::AuthenticationFailed { .. } => "authentication_failed",
            Self::LoggedOut { .. } => "logged_out",
            Self::SessionExpired { .. } => "session_expired",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_authentication_event_agent_id() {
        let agent_id = AgentId::new();
        let session_id = SessionId::new();
        
        let event = AuthenticationEvent::Authenticated {
            agent_id: agent_id.clone(),
            session_id,
        };
        
        assert_eq!(event.agent_id(), &agent_id);
        assert_eq!(event.event_type(), "authenticated");
    }
    
    #[test]
    fn test_authentication_failed_event() {
        let agent_id = AgentId::new();
        
        let event = AuthenticationEvent::AuthenticationFailed {
            agent_id: agent_id.clone(),
            reason: "Invalid credentials".to_string(),
        };
        
        assert_eq!(event.agent_id(), &agent_id);
        assert_eq!(event.event_type(), "authentication_failed");
    }
} 