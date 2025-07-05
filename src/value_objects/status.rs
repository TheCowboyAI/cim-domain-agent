//! Agent status value object

use serde::{Deserialize, Serialize};
use std::fmt;

/// Task-level status of an agent
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AgentTaskStatus {
    /// Agent is ready to accept tasks
    Idle,
    
    /// Agent is currently processing a task
    Processing {
        task_id: uuid::Uuid,
        progress: f32,
    },
    
    /// Agent is waiting for external input
    Waiting {
        reason: String,
        since: std::time::SystemTime,
    },
    
    /// Agent has encountered an error
    Error {
        message: String,
        recoverable: bool,
    },
    
    /// Agent is being configured or updated
    Configuring,
    
    /// Agent is shutting down
    Terminating,
    
    /// Agent has been terminated
    Terminated,
}

impl Default for AgentTaskStatus {
    fn default() -> Self {
        Self::Idle
    }
}

impl fmt::Display for AgentTaskStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Idle => write!(f, "Idle"),
            Self::Processing { task_id, progress } => {
                write!(f, "Processing task {} ({:.1}%)", task_id, progress * 100.0)
            }
            Self::Waiting { reason, .. } => write!(f, "Waiting: {reason}"),
            Self::Error { message, .. } => write!(f, "Error: {message}"),
            Self::Configuring => write!(f, "Configuring"),
            Self::Terminating => write!(f, "Terminating"),
            Self::Terminated => write!(f, "Terminated"),
        }
    }
} 