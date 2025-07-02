//! Monitoring and observability systems for agents
//!
//! This module provides ECS systems for monitoring agent health,
//! performance, and behavior.

use bevy::prelude::*;
use crate::components::{AgentEntity, AgentStatus, AgentState};
use crate::events::AuthenticationEvent;
use crate::value_objects::{AgentId, PerformanceMetrics};
use crate::systems::tools::ToolsComponent;
use crate::systems::authentication::AuthenticationState;
use std::collections::VecDeque;
use std::time::{Duration, Instant, SystemTime};

/// Component for tracking agent metrics
#[derive(Component, Debug, Clone)]
pub struct MetricsComponent {
    pub performance: PerformanceMetrics,
    pub health_score: f32,
    pub error_count: u32,
    pub warning_count: u32,
    pub last_activity: SystemTime,
    pub uptime: Duration,
    pub startup_time: Instant,
}

impl Default for MetricsComponent {
    fn default() -> Self {
        Self {
            performance: PerformanceMetrics::default(),
            health_score: 100.0,
            error_count: 0,
            warning_count: 0,
            last_activity: SystemTime::now(),
            uptime: Duration::from_secs(0),
            startup_time: Instant::now(),
        }
    }
}

/// Component for tracking agent activity history
#[derive(Component, Debug)]
pub struct ActivityHistoryComponent {
    pub recent_activities: VecDeque<ActivityRecord>,
    pub max_history_size: usize,
}

impl Default for ActivityHistoryComponent {
    fn default() -> Self {
        Self {
            recent_activities: VecDeque::new(),
            max_history_size: 1000,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ActivityRecord {
    pub timestamp: SystemTime,
    pub activity_type: ActivityType,
    pub details: String,
    pub success: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ActivityType {
    ToolExecution,
    Authentication,
    PermissionChange,
    StatusChange,
    Error,
    Warning,
}

/// Resource for system-wide monitoring configuration
#[derive(Resource, Debug)]
pub struct MonitoringConfig {
    pub metrics_update_interval: Duration,
    pub health_check_interval: Duration,
    pub alert_thresholds: AlertThresholds,
}

impl Default for MonitoringConfig {
    fn default() -> Self {
        Self {
            metrics_update_interval: Duration::from_secs(5),
            health_check_interval: Duration::from_secs(30),
            alert_thresholds: AlertThresholds::default(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct AlertThresholds {
    pub error_rate_threshold: f32,
    pub response_time_threshold: Duration,
    pub memory_usage_threshold: f32,
    pub cpu_usage_threshold: f32,
}

impl Default for AlertThresholds {
    fn default() -> Self {
        Self {
            error_rate_threshold: 0.1, // 10% error rate
            response_time_threshold: Duration::from_secs(5),
            memory_usage_threshold: 0.9, // 90% memory usage
            cpu_usage_threshold: 0.8, // 80% CPU usage
        }
    }
}

/// Event for monitoring alerts
#[derive(Event, Debug, Clone)]
pub struct MonitoringAlert {
    pub agent_id: AgentId,
    pub alert_type: AlertType,
    pub severity: AlertSeverity,
    pub message: String,
    pub timestamp: SystemTime,
}

#[derive(Debug, Clone, PartialEq)]
pub enum AlertType {
    HighErrorRate,
    SlowResponse,
    HighMemoryUsage,
    HighCpuUsage,
    HealthDegraded,
    AuthenticationFailure,
}

#[derive(Debug, Clone, PartialEq)]
pub enum AlertSeverity {
    Info,
    Warning,
    Error,
    Critical,
}

/// System to update agent metrics
pub fn update_agent_metrics(
    mut query: Query<(
        &AgentEntity,
        &mut MetricsComponent,
        &ToolsComponent,
        &AuthenticationState,
    )>,
    _time: Res<Time>,
) {
    for (_agent, mut metrics, tools, _auth_state) in &mut query {
        // Update uptime
        metrics.uptime = metrics.startup_time.elapsed();
        
        // Update performance metrics based on tool usage history
        let recent_usage: Vec<_> = tools.tool_usage_history.iter()
            .filter(|u| {
                let age = chrono::Utc::now().signed_duration_since(u.used_at);
                age.num_minutes() < 60 // Last hour
            })
            .collect();
        
        let total_executions = recent_usage.len();
        let total_errors = recent_usage.iter().filter(|u| !u.success).count();
        let total_time: u64 = recent_usage.iter().map(|u| u.duration_ms).sum();
        
        if total_executions > 0 {
            metrics.performance.quality_metrics.error_rate = total_errors as f32 / total_executions as f32;
            metrics.performance.task_metrics.avg_duration = Duration::from_millis(total_time / total_executions as u64);
        }
        
        // Update task metrics
        metrics.performance.task_metrics.tasks_completed = recent_usage.iter().filter(|u| u.success).count();
        metrics.performance.task_metrics.tasks_failed = total_errors;
        
        // Simulate resource usage (in real implementation, would get actual metrics)
        let memory_usage = 0.3 + (tools.active_executions.len() as f32 * 0.1);
        let cpu_usage = 0.2 + (tools.active_executions.len() as f32 * 0.15);
        metrics.performance.resource_metrics.avg_cpu_percent = cpu_usage * 100.0;
        metrics.performance.resource_metrics.peak_cpu_percent = (cpu_usage * 1.2).min(100.0);
        metrics.performance.resource_metrics.avg_memory_bytes = (memory_usage * 1024.0 * 1024.0 * 1024.0) as usize; // Simulate GB
    }
}

/// System to perform health checks
pub fn perform_health_checks(
    _commands: Commands,
    mut query: Query<(&AgentEntity, &mut MetricsComponent, &AgentStatus)>,
    config: Res<MonitoringConfig>,
    mut alerts: EventWriter<MonitoringAlert>,
) {
    for (agent, mut metrics, status) in &mut query {
        let mut health_score: f32 = 100.0;
        
        // Check error rate
        if metrics.performance.quality_metrics.error_rate > config.alert_thresholds.error_rate_threshold {
            health_score -= 20.0;
            alerts.write(MonitoringAlert {
                agent_id: AgentId::from_uuid(agent.agent_id),
                alert_type: AlertType::HighErrorRate,
                severity: AlertSeverity::Warning,
                message: format!(
                    "Error rate {:.1}% exceeds threshold",
                    metrics.performance.quality_metrics.error_rate * 100.0
                ),
                timestamp: SystemTime::now(),
            });
        }
        
        // Check response time
        if metrics.performance.task_metrics.avg_duration > config.alert_thresholds.response_time_threshold {
            health_score -= 15.0;
            alerts.write(MonitoringAlert {
                agent_id: AgentId::from_uuid(agent.agent_id),
                alert_type: AlertType::SlowResponse,
                severity: AlertSeverity::Warning,
                message: format!(
                    "Average response time {:?} exceeds threshold",
                    metrics.performance.task_metrics.avg_duration
                ),
                timestamp: SystemTime::now(),
            });
        }
        
        // Check resource usage
        let memory_usage_percent = (metrics.performance.resource_metrics.avg_memory_bytes as f32) / (4.0 * 1024.0 * 1024.0 * 1024.0); // Assume 4GB total
        if memory_usage_percent > config.alert_thresholds.memory_usage_threshold {
            health_score -= 25.0;
            alerts.write(MonitoringAlert {
                agent_id: AgentId::from_uuid(agent.agent_id),
                alert_type: AlertType::HighMemoryUsage,
                severity: AlertSeverity::Error,
                message: format!(
                    "Memory usage {:.1}% exceeds threshold",
                    memory_usage_percent * 100.0
                ),
                timestamp: SystemTime::now(),
            });
        }
        
        if metrics.performance.resource_metrics.avg_cpu_percent / 100.0 > config.alert_thresholds.cpu_usage_threshold {
            health_score -= 20.0;
            alerts.write(MonitoringAlert {
                agent_id: AgentId::from_uuid(agent.agent_id),
                alert_type: AlertType::HighCpuUsage,
                severity: AlertSeverity::Warning,
                message: format!(
                    "CPU usage {:.1}% exceeds threshold",
                    metrics.performance.resource_metrics.avg_cpu_percent
                ),
                timestamp: SystemTime::now(),
            });
        }
        
        // Check agent status
        match status.state {
            AgentState::Offline | AgentState::Error => health_score = 0.0,
            AgentState::Suspended => health_score *= 0.5,
            _ => {}
        }
        
        // Update health score
        metrics.health_score = health_score.max(0.0_f32);
        
        // Send critical alert if health is very low
        if metrics.health_score < 25.0 {
            alerts.write(MonitoringAlert {
                agent_id: AgentId::from_uuid(agent.agent_id),
                alert_type: AlertType::HealthDegraded,
                severity: AlertSeverity::Critical,
                message: format!("Agent health critically low: {:.1}", metrics.health_score),
                timestamp: SystemTime::now(),
            });
        }
    }
}

/// System to track agent activities
pub fn track_agent_activities(
    mut query: Query<(&AgentEntity, &mut ActivityHistoryComponent)>,
    mut tool_events: EventReader<crate::events::AgentToolsChanged>,
    mut auth_events: EventReader<AuthenticationEvent>,
) {
    // Track tool events
    for event in tool_events.read() {
        if let Some((_, mut history)) = query.iter_mut()
            .find(|(a, _)| AgentId::from_uuid(a.agent_id) == event.agent_id)
        {
            if !event.enabled.is_empty() {
                add_activity_record(
                    &mut history,
                    ActivityType::PermissionChange,
                    format!("Tools enabled: {} tools", event.enabled.len()),
                    true,
                );
            }
            if !event.disabled.is_empty() {
                add_activity_record(
                    &mut history,
                    ActivityType::PermissionChange,
                    format!("Tools disabled: {} tools", event.disabled.len()),
                    true,
                );
            }
        }
    }
    
    // Track authentication events
    for event in auth_events.read() {
        let agent_id = event.agent_id();
        if let Some((_, mut history)) = query.iter_mut()
            .find(|(a, _)| AgentId::from_uuid(a.agent_id) == *agent_id)
        {
            match event {
                AuthenticationEvent::Authenticated { .. } => {
                    add_activity_record(
                        &mut history,
                        ActivityType::Authentication,
                        "Agent authenticated successfully".to_string(),
                        true,
                    );
                }
                AuthenticationEvent::AuthenticationFailed { reason, .. } => {
                    add_activity_record(
                        &mut history,
                        ActivityType::Authentication,
                        format!("Authentication failed: {}", reason),
                        false,
                    );
                }
                _ => {}
            }
        }
    }
}

/// Helper function to add activity record
fn add_activity_record(
    history: &mut ActivityHistoryComponent,
    activity_type: ActivityType,
    details: String,
    success: bool,
) {
    let record = ActivityRecord {
        timestamp: SystemTime::now(),
        activity_type,
        details,
        success,
    };
    
    history.recent_activities.push_back(record);
    
    // Trim history if it exceeds max size
    while history.recent_activities.len() > history.max_history_size {
        history.recent_activities.pop_front();
    }
}

/// System to generate monitoring reports
pub fn generate_monitoring_reports(
    query: Query<(&AgentEntity, &MetricsComponent, &ActivityHistoryComponent)>,
    mut report_requests: EventReader<GenerateReportRequest>,
    mut report_responses: EventWriter<MonitoringReport>,
) {
    for request in report_requests.read() {
        if let Some((_agent, metrics, history)) = query.iter()
            .find(|(a, _, _)| AgentId::from_uuid(a.agent_id) == request.agent_id)
        {
            let report = MonitoringReport {
                agent_id: request.agent_id.clone(),
                timestamp: SystemTime::now(),
                health_score: metrics.health_score,
                performance_summary: metrics.performance.clone(),
                error_count: metrics.error_count,
                warning_count: metrics.warning_count,
                uptime: metrics.uptime,
                recent_activities: history.recent_activities.iter()
                    .rev()
                    .take(request.activity_count.unwrap_or(10))
                    .cloned()
                    .collect(),
            };
            
            report_responses.write(report);
        }
    }
}

/// Event to request monitoring report
#[derive(Event, Debug, Clone)]
pub struct GenerateReportRequest {
    pub agent_id: AgentId,
    pub activity_count: Option<usize>,
}

/// Monitoring report
#[derive(Event, Debug, Clone)]
pub struct MonitoringReport {
    pub agent_id: AgentId,
    pub timestamp: SystemTime,
    pub health_score: f32,
    pub performance_summary: PerformanceMetrics,
    pub error_count: u32,
    pub warning_count: u32,
    pub uptime: Duration,
    pub recent_activities: Vec<ActivityRecord>,
}

/// Plugin to register monitoring systems
pub struct MonitoringPlugin;

impl Plugin for MonitoringPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<MonitoringConfig>()
            .add_event::<MonitoringAlert>()
            .add_event::<GenerateReportRequest>()
            .add_event::<MonitoringReport>()
            .add_systems(
                Update,
                (
                    update_agent_metrics,
                    perform_health_checks,
                    track_agent_activities,
                    generate_monitoring_reports,
                ),
            );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_metrics_component_default() {
        let metrics = MetricsComponent::default();
        assert_eq!(metrics.health_score, 100.0);
        assert_eq!(metrics.error_count, 0);
        assert_eq!(metrics.warning_count, 0);
    }
    
    #[test]
    fn test_activity_history_default() {
        let history = ActivityHistoryComponent::default();
        assert!(history.recent_activities.is_empty());
        assert_eq!(history.max_history_size, 1000);
    }
    
    #[test]
    fn test_monitoring_config_default() {
        let config = MonitoringConfig::default();
        assert_eq!(config.metrics_update_interval, Duration::from_secs(5));
        assert_eq!(config.health_check_interval, Duration::from_secs(30));
    }
}
