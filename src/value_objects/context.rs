//! Agent context value object

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Context in which an agent operates
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[derive(Default)]
pub struct AgentContext {
    /// Current task being executed
    pub current_task: Option<TaskContext>,
    
    /// Conversation history
    pub conversation_history: Vec<ConversationTurn>,
    
    /// Environment variables
    pub environment: HashMap<String, String>,
    
    /// Session data
    pub session_data: HashMap<String, serde_json::Value>,
    
    /// User context
    pub user_context: Option<UserContext>,
    
    /// Domain-specific context
    pub domain_context: HashMap<String, serde_json::Value>,
}

/// Context of a specific task
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TaskContext {
    /// Task identifier
    pub task_id: uuid::Uuid,
    
    /// Task type
    pub task_type: String,
    
    /// Task parameters
    pub parameters: HashMap<String, serde_json::Value>,
    
    /// Start time
    pub started_at: std::time::SystemTime,
    
    /// Parent task if this is a subtask
    pub parent_task_id: Option<uuid::Uuid>,
}

/// A turn in a conversation
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConversationTurn {
    /// Who sent this message
    pub role: ConversationRole,
    
    /// The message content
    pub content: String,
    
    /// When it was sent
    pub timestamp: std::time::SystemTime,
    
    /// Additional metadata
    pub metadata: HashMap<String, serde_json::Value>,
}

/// Role in a conversation
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ConversationRole {
    User,
    Agent,
    System,
}

/// User context information
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UserContext {
    /// User identifier
    pub user_id: String,
    
    /// User preferences
    pub preferences: HashMap<String, serde_json::Value>,
    
    /// User permissions
    pub permissions: Vec<String>,
    
    /// User's current location/timezone
    pub timezone: Option<String>,
}


impl AgentContext {
    /// Add a conversation turn
    pub fn add_conversation_turn(&mut self, role: ConversationRole, content: String) {
        self.conversation_history.push(ConversationTurn {
            role,
            content,
            timestamp: std::time::SystemTime::now(),
            metadata: HashMap::new(),
        });
    }
    
    /// Get recent conversation history
    pub fn recent_conversation(&self, n: usize) -> &[ConversationTurn] {
        let len = self.conversation_history.len();
        if len > n {
            &self.conversation_history[len - n..]
        } else {
            &self.conversation_history
        }
    }
} 