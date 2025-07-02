//! Tool management and execution systems for agents
//!
//! This module provides ECS systems for managing agent tools,
//! including tool registration, execution, and result handling.

use bevy::prelude::*;
use crate::components::{AgentEntity, AgentCapabilities};
use crate::events::ToolsChangedEvent;
use crate::value_objects::{
    AgentId, Tool, ToolCategory, ToolPermission, ToolUsage,
    ExecutionResult, ToolAccess, ToolType, ToolConfig
};
use crate::systems::permissions::PermissionsComponent;
use std::collections::{HashMap, HashSet};
use uuid::Uuid;

/// Component representing available tools for an agent
#[derive(Component, Debug, Clone)]
pub struct ToolsComponent {
    pub available_tools: HashMap<String, Tool>,
    pub tool_usage: HashMap<String, ToolUsage>,
    pub active_executions: HashSet<Uuid>,
}

impl Default for ToolsComponent {
    fn default() -> Self {
        Self {
            available_tools: HashMap::new(),
            tool_usage: HashMap::new(),
            active_executions: HashSet::new(),
        }
    }
}

/// Resource for managing tool registry
#[derive(Resource, Debug, Default)]
pub struct ToolRegistry {
    /// All registered tools in the system
    pub tools: HashMap<String, Tool>,
    /// Tool configurations
    pub configurations: HashMap<String, ToolConfig>,
    /// Tool access policies
    pub access_policies: HashMap<String, ToolAccess>,
}

/// Event for tool registration
#[derive(Event, Debug, Clone)]
pub struct RegisterToolRequest {
    pub tool: Tool,
    pub config: Option<ToolConfig>,
    pub access: Option<ToolAccess>,
}

/// Event for tool execution requests
#[derive(Event, Debug, Clone)]
pub struct ExecuteToolRequest {
    pub agent_id: AgentId,
    pub tool_name: String,
    pub parameters: serde_json::Value,
    pub execution_id: Uuid,
}

/// Event for tool execution results
#[derive(Event, Debug, Clone)]
pub struct ExecuteToolResponse {
    pub agent_id: AgentId,
    pub tool_name: String,
    pub execution_id: Uuid,
    pub result: ExecutionResult,
}

/// System to handle tool registration
pub fn handle_tool_registration(
    mut registry: ResMut<ToolRegistry>,
    mut registration_requests: EventReader<RegisterToolRequest>,
    mut commands: Commands,
) {
    for request in registration_requests.read() {
        let tool_name = request.tool.name.clone();
        
        // Register tool
        registry.tools.insert(tool_name.clone(), request.tool.clone());
        
        // Store configuration if provided
        if let Some(config) = &request.config {
            registry.configurations.insert(tool_name.clone(), config.clone());
        }
        
        // Store access policy if provided
        if let Some(access) = &request.access {
            registry.access_policies.insert(tool_name.clone(), access.clone());
        }
        
        // Trigger registration event
        commands.trigger(ToolsChangedEvent::ToolRegistered {
            tool_name,
        });
    }
}

/// System to assign tools to agents
pub fn assign_tools_to_agents(
    mut commands: Commands,
    mut query: Query<(Entity, &AgentEntity, &mut ToolsComponent, &PermissionsComponent)>,
    registry: Res<ToolRegistry>,
    mut tool_assignments: EventReader<AssignToolRequest>,
) {
    for assignment in tool_assignments.read() {
        if let Some((entity, agent, mut tools, permissions)) = query
            .iter_mut()
            .find(|(_, a, _, _)| a.agent_id == assignment.agent_id)
        {
            // Check if tool exists in registry
            if let Some(tool) = registry.tools.get(&assignment.tool_name) {
                // Check permissions
                let has_permission = match &tool.required_permission {
                    ToolPermission::None => true,
                    ToolPermission::Execute => permissions.permissions.iter()
                        .any(|p| p.resource == "tool_execution" && !p.is_expired()),
                    ToolPermission::Admin => permissions.permissions.iter()
                        .any(|p| p.resource == "system" && p.access_level == crate::value_objects::AccessLevel::Admin && !p.is_expired()),
                    ToolPermission::Custom(resource) => permissions.permissions.iter()
                        .any(|p| p.resource == resource && !p.is_expired()),
                };
                
                if has_permission {
                    // Add tool to agent
                    tools.available_tools.insert(tool.name.clone(), tool.clone());
                    
                    // Initialize usage tracking
                    tools.tool_usage.insert(
                        tool.name.clone(),
                        ToolUsage {
                            total_uses: 0,
                            successful_uses: 0,
                            failed_uses: 0,
                            last_used: None,
                            average_execution_time: std::time::Duration::from_secs(0),
                        },
                    );
                    
                    // Trigger event
                    commands.trigger(ToolsChangedEvent::ToolAssigned {
                        agent_id: agent.agent_id.clone(),
                        tool_name: tool.name.clone(),
                    });
                } else {
                    // Permission denied
                    commands.trigger(ToolsChangedEvent::ToolAssignmentFailed {
                        agent_id: agent.agent_id.clone(),
                        tool_name: tool.name.clone(),
                        reason: "Insufficient permissions".to_string(),
                    });
                }
            }
        }
    }
}

/// Event for tool assignment requests
#[derive(Event, Debug, Clone)]
pub struct AssignToolRequest {
    pub agent_id: AgentId,
    pub tool_name: String,
}

/// System to handle tool execution
pub fn handle_tool_execution(
    mut commands: Commands,
    mut execution_requests: EventReader<ExecuteToolRequest>,
    mut execution_responses: EventWriter<ExecuteToolResponse>,
    mut query: Query<(&AgentEntity, &mut ToolsComponent, &AgentCapabilities)>,
    registry: Res<ToolRegistry>,
    time: Res<Time>,
) {
    for request in execution_requests.read() {
        if let Some((agent, mut tools, capabilities)) = query
            .iter_mut()
            .find(|(a, _, _)| a.agent_id == request.agent_id)
        {
            // Check if agent has the tool
            if let Some(tool) = tools.available_tools.get(&request.tool_name) {
                // Check if agent can execute tools
                if !capabilities.can_execute_tools {
                    execution_responses.send(ExecuteToolResponse {
                        agent_id: request.agent_id.clone(),
                        tool_name: request.tool_name.clone(),
                        execution_id: request.execution_id,
                        result: ExecutionResult::failure(
                            "Agent does not have tool execution capability".to_string()
                        ),
                    });
                    continue;
                }
                
                // Add to active executions
                tools.active_executions.insert(request.execution_id);
                
                // Simulate tool execution (in real implementation, this would call actual tool)
                let start_time = std::time::Instant::now();
                let result = execute_tool_mock(tool, &request.parameters);
                let execution_time = start_time.elapsed();
                
                // Update usage statistics
                if let Some(usage) = tools.tool_usage.get_mut(&request.tool_name) {
                    usage.total_uses += 1;
                    if result.success {
                        usage.successful_uses += 1;
                    } else {
                        usage.failed_uses += 1;
                    }
                    usage.last_used = Some(std::time::SystemTime::now());
                    
                    // Update average execution time
                    let total_time = usage.average_execution_time * usage.total_uses as u32;
                    usage.average_execution_time = (total_time + execution_time) / (usage.total_uses as u32);
                }
                
                // Remove from active executions
                tools.active_executions.remove(&request.execution_id);
                
                // Send response
                execution_responses.send(ExecuteToolResponse {
                    agent_id: request.agent_id.clone(),
                    tool_name: request.tool_name.clone(),
                    execution_id: request.execution_id,
                    result,
                });
                
                // Trigger event
                commands.trigger(ToolsChangedEvent::ToolExecuted {
                    agent_id: agent.agent_id.clone(),
                    tool_name: request.tool_name.clone(),
                });
            } else {
                // Tool not available
                execution_responses.send(ExecuteToolResponse {
                    agent_id: request.agent_id.clone(),
                    tool_name: request.tool_name.clone(),
                    execution_id: request.execution_id,
                    result: ExecutionResult::failure(
                        format!("Tool '{}' not available for agent", request.tool_name)
                    ),
                });
            }
        }
    }
}

/// System to remove tools from agents
pub fn handle_tool_removal(
    mut commands: Commands,
    mut removal_requests: EventReader<RemoveToolRequest>,
    mut query: Query<(&AgentEntity, &mut ToolsComponent)>,
) {
    for request in removal_requests.read() {
        if let Some((agent, mut tools)) = query
            .iter_mut()
            .find(|(a, _)| a.agent_id == request.agent_id)
        {
            if tools.available_tools.remove(&request.tool_name).is_some() {
                // Also remove usage stats
                tools.tool_usage.remove(&request.tool_name);
                
                // Trigger event
                commands.trigger(ToolsChangedEvent::ToolRemoved {
                    agent_id: agent.agent_id.clone(),
                    tool_name: request.tool_name.clone(),
                });
            }
        }
    }
}

/// Event for tool removal requests
#[derive(Event, Debug, Clone)]
pub struct RemoveToolRequest {
    pub agent_id: AgentId,
    pub tool_name: String,
}

/// Mock tool execution function
fn execute_tool_mock(tool: &Tool, parameters: &serde_json::Value) -> ExecutionResult {
    // Simulate different tool behaviors based on category
    match tool.category {
        ToolCategory::Analysis => {
            ExecutionResult::success(serde_json::json!({
                "analysis": "Mock analysis result",
                "parameters": parameters,
                "tool": tool.name
            }))
        }
        ToolCategory::Generation => {
            ExecutionResult::success(serde_json::json!({
                "generated": "Mock generated content",
                "parameters": parameters,
                "tool": tool.name
            }))
        }
        ToolCategory::Communication => {
            ExecutionResult::success(serde_json::json!({
                "message": "Mock communication sent",
                "parameters": parameters,
                "tool": tool.name
            }))
        }
        ToolCategory::DataProcessing => {
            ExecutionResult::success(serde_json::json!({
                "processed": "Mock data processing complete",
                "parameters": parameters,
                "tool": tool.name
            }))
        }
        ToolCategory::SystemControl => {
            // Simulate restricted tool
            if parameters.get("authorized").and_then(|v| v.as_bool()).unwrap_or(false) {
                ExecutionResult::success(serde_json::json!({
                    "system": "Mock system control executed",
                    "parameters": parameters,
                    "tool": tool.name
                }))
            } else {
                ExecutionResult::failure("Unauthorized system control access".to_string())
            }
        }
        ToolCategory::Custom(_) => {
            ExecutionResult::success(serde_json::json!({
                "custom": "Mock custom tool executed",
                "parameters": parameters,
                "tool": tool.name
            }))
        }
    }
}

/// Plugin to register tool systems
pub struct ToolsPlugin;

impl Plugin for ToolsPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<ToolRegistry>()
            .add_event::<RegisterToolRequest>()
            .add_event::<AssignToolRequest>()
            .add_event::<ExecuteToolRequest>()
            .add_event::<ExecuteToolResponse>()
            .add_event::<RemoveToolRequest>()
            .add_systems(
                Update,
                (
                    handle_tool_registration,
                    assign_tools_to_agents,
                    handle_tool_execution,
                    handle_tool_removal,
                ),
            );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_tools_component_default() {
        let tools = ToolsComponent::default();
        assert!(tools.available_tools.is_empty());
        assert!(tools.tool_usage.is_empty());
        assert!(tools.active_executions.is_empty());
    }
    
    #[test]
    fn test_tool_registry_default() {
        let registry = ToolRegistry::default();
        assert!(registry.tools.is_empty());
        assert!(registry.configurations.is_empty());
        assert!(registry.access_policies.is_empty());
    }
    
    #[test]
    fn test_mock_tool_execution() {
        let tool = Tool {
            name: "test_tool".to_string(),
            description: "Test tool".to_string(),
            category: ToolCategory::Analysis,
            required_permission: ToolPermission::None,
            version: "1.0.0".to_string(),
        };
        
        let params = serde_json::json!({ "test": true });
        let result = execute_tool_mock(&tool, &params);
        
        assert!(result.success);
        assert!(result.output.is_some());
    }
}
