//! Status-related ECS components

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// Component for agent operational status
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub struct AgentStatus {
    /// Current operational state
    pub state: AgentState,
    /// Previous state (for transition tracking)
    pub previous_state: Option<AgentState>,
    /// When the current state was entered
    pub state_entered_at: chrono::DateTime<chrono::Utc>,
}

/// Agent operational states
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AgentState {
    /// Agent is being initialized
    Initializing,
    /// Agent is active and operational
    Active,
    /// Agent is temporarily suspended
    Suspended,
    /// Agent is offline/unavailable
    Offline,
    /// Agent is in maintenance mode
    Maintenance,
    /// Agent has been decommissioned
    Decommissioned,
    /// Agent has encountered an error
    Error,
}

impl AgentStatus {
    /// Create a new status component
    pub fn new(state: AgentState) -> Self {
        Self {
            state,
            previous_state: None,
            state_entered_at: chrono::Utc::now(),
        }
    }

    /// Transition to a new state
    pub fn transition_to(&mut self, new_state: AgentState) {
        if self.state != new_state {
            self.previous_state = Some(self.state);
            self.state = new_state;
            self.state_entered_at = chrono::Utc::now();
        }
    }

    /// Get the duration in the current state
    pub fn time_in_state(&self) -> chrono::Duration {
        chrono::Utc::now() - self.state_entered_at
    }

    /// Check if agent is operational
    pub fn is_operational(&self) -> bool {
        matches!(self.state, AgentState::Active | AgentState::Maintenance)
    }
}

impl Default for AgentStatus {
    fn default() -> Self {
        Self::new(AgentState::Initializing)
    }
}

/// Component for agent readiness
#[derive(Component, Debug, Clone)]
pub struct AgentReadiness {
    /// Overall readiness status
    pub is_ready: bool,
    /// Individual readiness checks
    pub checks: Vec<ReadinessCheck>,
    /// Last readiness evaluation
    pub last_checked: chrono::DateTime<chrono::Utc>,
}

/// Individual readiness check
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReadinessCheck {
    /// Check name
    pub name: String,
    /// Check status
    pub status: CheckStatus,
    /// Check message
    pub message: String,
    /// When this check was performed
    pub checked_at: chrono::DateTime<chrono::Utc>,
}

/// Status of a readiness check
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CheckStatus {
    /// Check passed
    Passed,
    /// Check failed
    Failed,
    /// Check is pending
    Pending,
    /// Check was skipped
    Skipped,
}

impl Default for AgentReadiness {
    fn default() -> Self {
        Self {
            is_ready: false,
            checks: Vec::new(),
            last_checked: chrono::Utc::now(),
        }
    }
}

impl AgentReadiness {
    /// Update readiness based on checks
    pub fn update_readiness(&mut self) {
        self.is_ready = self.checks.iter()
            .all(|check| matches!(check.status, CheckStatus::Passed | CheckStatus::Skipped));
        self.last_checked = chrono::Utc::now();
    }

    /// Add or update a readiness check
    pub fn update_check(&mut self, name: String, status: CheckStatus, message: String) {
        if let Some(check) = self.checks.iter_mut().find(|c| c.name == name) {
            check.status = status;
            check.message = message;
            check.checked_at = chrono::Utc::now();
        } else {
            self.checks.push(ReadinessCheck {
                name,
                status,
                message,
                checked_at: chrono::Utc::now(),
            });
        }
        self.update_readiness();
    }
}

/// Component for agent lifecycle events
#[derive(Component, Debug, Clone)]
pub struct AgentLifecycle {
    /// Lifecycle events
    pub events: Vec<LifecycleEvent>,
    /// Maximum events to keep
    pub max_events: usize,
}

/// Lifecycle event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LifecycleEvent {
    /// Event type
    pub event_type: LifecycleEventType,
    /// When the event occurred
    pub timestamp: chrono::DateTime<chrono::Utc>,
    /// Additional context
    pub context: Option<String>,
}

/// Types of lifecycle events
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LifecycleEventType {
    /// Agent was created
    Created,
    /// Agent was initialized
    Initialized,
    /// Agent was started
    Started,
    /// Agent was stopped
    Stopped,
    /// Agent was suspended
    Suspended,
    /// Agent was resumed
    Resumed,
    /// Agent entered maintenance mode
    MaintenanceStarted,
    /// Agent exited maintenance mode
    MaintenanceEnded,
    /// Agent was decommissioned
    Decommissioned,
    /// Agent encountered an error
    ErrorOccurred,
    /// Agent recovered from error
    ErrorRecovered,
}

impl Default for AgentLifecycle {
    fn default() -> Self {
        Self {
            events: vec![LifecycleEvent {
                event_type: LifecycleEventType::Created,
                timestamp: chrono::Utc::now(),
                context: None,
            }],
            max_events: 100,
        }
    }
}

impl AgentLifecycle {
    /// Add a lifecycle event
    pub fn add_event(&mut self, event_type: LifecycleEventType, context: Option<String>) {
        self.events.push(LifecycleEvent {
            event_type,
            timestamp: chrono::Utc::now(),
            context,
        });
        
        // Keep only the most recent events
        if self.events.len() > self.max_events {
            let remove_count = self.events.len() - self.max_events;
            self.events.drain(0..remove_count);
        }
    }
}

/// Component for agent activity tracking
#[derive(Component, Debug, Clone)]
pub struct AgentActivity {
    /// Last activity timestamp
    pub last_activity: chrono::DateTime<chrono::Utc>,
    /// Activity type
    pub activity_type: ActivityType,
    /// Activity count in the last hour
    pub hourly_activity_count: u64,
    /// Whether agent is currently active
    pub is_active: bool,
}

/// Types of agent activity
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ActivityType {
    /// Processing a request
    ProcessingRequest,
    /// Executing a tool
    ExecutingTool,
    /// Performing maintenance
    Maintenance,
    /// Idle
    Idle,
    /// Custom activity
    Custom(String),
}

impl Default for AgentActivity {
    fn default() -> Self {
        Self {
            last_activity: chrono::Utc::now(),
            activity_type: ActivityType::Idle,
            hourly_activity_count: 0,
            is_active: false,
        }
    }
}

impl AgentActivity {
    /// Update activity
    pub fn update_activity(&mut self, activity_type: ActivityType) {
        self.last_activity = chrono::Utc::now();
        self.activity_type = activity_type;
        self.hourly_activity_count += 1;
        self.is_active = !matches!(self.activity_type, ActivityType::Idle);
    }

    /// Get time since last activity
    pub fn time_since_activity(&self) -> chrono::Duration {
        chrono::Utc::now() - self.last_activity
    }
} 