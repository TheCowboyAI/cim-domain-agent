//! Rate limiting system for agent operations
//!
//! This module implements rate limiting to prevent agents from overwhelming resources.

use bevy::prelude::*;
use crate::components::{AgentEntity, AgentConstraints};
use crate::value_objects::constraint::RateLimits;
use std::collections::HashMap;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;
use std::sync::Arc;
use tracing::warn;

/// Rate limiter resource for tracking agent usage
#[derive(Resource)]
pub struct RateLimiter {
    /// Track request counts per agent
    request_counts: Arc<RwLock<HashMap<uuid::Uuid, RequestTracker>>>,
    /// Track token usage per agent
    token_counts: Arc<RwLock<HashMap<uuid::Uuid, TokenTracker>>>,
    /// Track API calls per agent
    api_call_counts: Arc<RwLock<HashMap<uuid::Uuid, ApiCallTracker>>>,
}

impl Default for RateLimiter {
    fn default() -> Self {
        Self {
            request_counts: Arc::new(RwLock::new(HashMap::new())),
            token_counts: Arc::new(RwLock::new(HashMap::new())),
            api_call_counts: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

/// Track requests per minute
#[derive(Debug, Clone)]
struct RequestTracker {
    /// Window start time
    window_start: Instant,
    /// Request count in current window
    count: u32,
}

/// Track tokens per minute
#[derive(Debug, Clone)]
struct TokenTracker {
    /// Window start time
    window_start: Instant,
    /// Token count in current window
    count: u32,
}

/// Track API calls per hour
#[derive(Debug, Clone)]
struct ApiCallTracker {
    /// Window start time
    window_start: Instant,
    /// API call count in current window
    count: u32,
}

impl RateLimiter {
    /// Check if an agent can make a request
    pub async fn check_request_limit(&self, agent_id: uuid::Uuid, rate_limits: &RateLimits) -> Result<(), RateLimitError> {
        if let Some(limit) = rate_limits.requests_per_minute {
            let mut trackers = self.request_counts.write().await;
            let now = Instant::now();
            
            let tracker = trackers.entry(agent_id).or_insert(RequestTracker {
                window_start: now,
                count: 0,
            });
            
            // Reset window if needed
            if now.duration_since(tracker.window_start) >= Duration::from_secs(60) {
                tracker.window_start = now;
                tracker.count = 0;
            }
            
            if tracker.count >= limit {
                return Err(RateLimitError::RequestsPerMinuteExceeded { 
                    limit, 
                    current: tracker.count 
                });
            }
            
            tracker.count += 1;
        }
        
        Ok(())
    }
    
    /// Check if an agent can use tokens
    pub async fn check_token_limit(&self, agent_id: uuid::Uuid, tokens: u32, rate_limits: &RateLimits) -> Result<(), RateLimitError> {
        if let Some(limit) = rate_limits.tokens_per_minute {
            let mut trackers = self.token_counts.write().await;
            let now = Instant::now();
            
            let tracker = trackers.entry(agent_id).or_insert(TokenTracker {
                window_start: now,
                count: 0,
            });
            
            // Reset window if needed
            if now.duration_since(tracker.window_start) >= Duration::from_secs(60) {
                tracker.window_start = now;
                tracker.count = 0;
            }
            
            if tracker.count + tokens > limit {
                return Err(RateLimitError::TokensPerMinuteExceeded { 
                    limit, 
                    current: tracker.count,
                    requested: tokens,
                });
            }
            
            tracker.count += tokens;
        }
        
        Ok(())
    }
    
    /// Check if an agent can make an API call
    pub async fn check_api_call_limit(&self, agent_id: uuid::Uuid, rate_limits: &RateLimits) -> Result<(), RateLimitError> {
        if let Some(limit) = rate_limits.api_calls_per_hour {
            let mut trackers = self.api_call_counts.write().await;
            let now = Instant::now();
            
            let tracker = trackers.entry(agent_id).or_insert(ApiCallTracker {
                window_start: now,
                count: 0,
            });
            
            // Reset window if needed
            if now.duration_since(tracker.window_start) >= Duration::from_secs(3600) {
                tracker.window_start = now;
                tracker.count = 0;
            }
            
            if tracker.count >= limit {
                return Err(RateLimitError::ApiCallsPerHourExceeded { 
                    limit, 
                    current: tracker.count 
                });
            }
            
            tracker.count += 1;
        }
        
        Ok(())
    }
    
    /// Get current usage stats for an agent
    pub async fn get_usage_stats(&self, agent_id: uuid::Uuid) -> UsageStats {
        let request_trackers = self.request_counts.read().await;
        let token_trackers = self.token_counts.read().await;
        let api_trackers = self.api_call_counts.read().await;
        
        UsageStats {
            requests_in_window: request_trackers.get(&agent_id).map(|t| t.count).unwrap_or(0),
            tokens_in_window: token_trackers.get(&agent_id).map(|t| t.count).unwrap_or(0),
            api_calls_in_window: api_trackers.get(&agent_id).map(|t| t.count).unwrap_or(0),
        }
    }
}

/// Usage statistics for an agent
#[derive(Debug, Clone)]
pub struct UsageStats {
    pub requests_in_window: u32,
    pub tokens_in_window: u32,
    pub api_calls_in_window: u32,
}

/// Rate limit errors
#[derive(Debug, Clone, thiserror::Error)]
pub enum RateLimitError {
    #[error("Requests per minute limit exceeded: {limit} (current: {current})")]
    RequestsPerMinuteExceeded { limit: u32, current: u32 },
    
    #[error("Tokens per minute limit exceeded: {limit} (current: {current}, requested: {requested})")]
    TokensPerMinuteExceeded { limit: u32, current: u32, requested: u32 },
    
    #[error("API calls per hour limit exceeded: {limit} (current: {current})")]
    ApiCallsPerHourExceeded { limit: u32, current: u32 },
}

/// Event for rate limit violations
#[derive(Event)]
pub struct RateLimitViolation {
    pub agent_id: uuid::Uuid,
    pub error: RateLimitError,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

/// System to enforce rate limits on agent operations
pub fn enforce_rate_limits_system(
    mut operation_events: EventReader<AgentOperationRequest>,
    agent_query: Query<(&AgentEntity, &AgentConstraints)>,
    rate_limiter: Res<RateLimiter>,
    mut violation_events: EventWriter<RateLimitViolation>,
    mut approved_events: EventWriter<AgentOperationApproved>,
) {
    for operation in operation_events.read() {
        // Find the agent
        let agent_found = agent_query.iter()
            .find(|(entity, _)| entity.agent_id == operation.agent_id);
        
        if let Some((_, constraints)) = agent_found {
            let rate_limits = &constraints.constraint.rate_limits;
            
            // Use tokio runtime to run async checks
            let agent_id = operation.agent_id;
            let request_counts = rate_limiter.request_counts.clone();
            let token_counts = rate_limiter.token_counts.clone();
            let api_call_counts = rate_limiter.api_call_counts.clone();
            let rate_limits_clone = rate_limits.clone();
            let operation_clone = operation.clone();
            
            // Check all rate limits
            let check_result = futures::executor::block_on(async {
                // Create a temporary rate limiter with the cloned Arcs
                let temp_limiter = RateLimiter {
                    request_counts: request_counts.clone(),
                    token_counts: token_counts.clone(),
                    api_call_counts: api_call_counts.clone(),
                };
                
                // Check request limit
                if let Err(e) = temp_limiter.check_request_limit(agent_id, &rate_limits_clone).await {
                    return Err(e);
                }
                
                // Check token limit if applicable
                if let Some(token_count) = operation_clone.estimated_tokens {
                    if let Err(e) = temp_limiter.check_token_limit(agent_id, token_count, &rate_limits_clone).await {
                        return Err(e);
                    }
                }
                
                // Check API call limit if applicable
                if operation_clone.is_api_call {
                    if let Err(e) = temp_limiter.check_api_call_limit(agent_id, &rate_limits_clone).await {
                        return Err(e);
                    }
                }
                
                Ok(())
            });
            
            match check_result {
                Ok(()) => {
                    // Operation approved
                    approved_events.write(AgentOperationApproved {
                        agent_id: operation.agent_id,
                        operation_id: operation.operation_id,
                        approved_at: chrono::Utc::now(),
                    });
                }
                Err(error) => {
                    // Rate limit violated
                    violation_events.write(RateLimitViolation {
                        agent_id: operation.agent_id,
                        error,
                        timestamp: chrono::Utc::now(),
                    });
                }
            }
        }
    }
}

/// Request for an agent operation that needs rate limiting
#[derive(Event, Clone)]
pub struct AgentOperationRequest {
    pub agent_id: uuid::Uuid,
    pub operation_id: uuid::Uuid,
    pub operation_type: String,
    pub estimated_tokens: Option<u32>,
    pub is_api_call: bool,
}

/// Event when an operation is approved
#[derive(Event)]
pub struct AgentOperationApproved {
    pub agent_id: uuid::Uuid,
    pub operation_id: uuid::Uuid,
    pub approved_at: chrono::DateTime<chrono::Utc>,
}

/// System to monitor and report rate limit usage
pub fn monitor_rate_limit_usage_system(
    agent_query: Query<&AgentEntity>,
    rate_limiter: Res<RateLimiter>,
    time: Res<Time>,
) {
    // Only check every 5 seconds
    if time.elapsed_secs() % 5.0 > time.delta_secs() {
        return;
    }
    
    for agent_entity in agent_query.iter() {
        let agent_id = agent_entity.agent_id;
        
        // Get usage stats
        let stats = futures::executor::block_on(
            rate_limiter.get_usage_stats(agent_id)
        );
        
        // Log high usage
        if stats.requests_in_window > 50 {
            warn!(
                "Agent {} has high request rate: {} requests/min",
                agent_id, stats.requests_in_window
            );
        }
        
        if stats.tokens_in_window > 40000 {
            warn!(
                "Agent {} has high token usage: {} tokens/min",
                agent_id, stats.tokens_in_window
            );
        }
    }
}