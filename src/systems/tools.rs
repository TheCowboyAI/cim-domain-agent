//! Tool management and execution systems for agents
//!
//! This module provides ECS systems for managing agent tools,
//! including tool registration, execution, and result handling.

use bevy::prelude::*;
use crate::components::{AgentEntity, AgentCapabilities};
use crate::events::AgentToolsChanged;
use crate::value_objects::{
    AgentId, Tool, ToolCategory, ToolUsage,
    ExecutionResult, ExecutionError, ToolAccess, ToolType, ToolConfig
};
use crate::systems::permissions::PermissionsComponent;
use std::collections::{HashMap, HashSet};
use uuid::Uuid;

/// Component representing available tools for an agent
#[derive(Component, Debug, Clone)]
pub struct ToolsComponent {
    pub available_tools: HashMap<String, Tool>,
    pub tool_usage_history: Vec<ToolUsage>,
    pub active_executions: HashSet<Uuid>,
}

impl Default for ToolsComponent {
    fn default() -> Self {
        Self {
            available_tools: HashMap::new(),
            tool_usage_history: Vec::new(),
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

/// Audit component for tool registrations
#[derive(Component, Debug, Clone)]
pub struct ToolRegistrationAudit {
    pub tool_id: String,
    pub tool_name: String,
    pub registered_at: std::time::SystemTime,
    pub tool_type: ToolType,
    pub category: ToolCategory,
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

/// Event for tool assignment requests
#[derive(Event, Debug, Clone)]
pub struct AssignToolRequest {
    pub agent_id: AgentId,
    pub tool_name: String,
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
        
        // Create audit entity for tool registration
        commands.spawn(ToolRegistrationAudit {
            tool_id: request.tool.id.clone(),
            tool_name: tool_name.clone(),
            registered_at: std::time::SystemTime::now(),
            tool_type: request.tool.tool_type.clone(),
            category: request.tool.category.clone(),
        });
        
        info!("Registered tool '{}' with category {:?}", tool_name, request.tool.category);
        
        // Tool registration doesn't trigger AgentToolsChanged as it's system-wide
        // AgentToolsChanged is only for agent-specific tool changes
    }
}

/// System to assign tools to agents
pub fn assign_tools_to_agents(
    mut commands: Commands,
    mut query: Query<(Entity, &AgentEntity, &mut ToolsComponent, &PermissionsComponent)>,
    registry: Res<ToolRegistry>,
    mut tool_assignments: EventReader<AssignToolRequest>,
    mut tools_changed: EventWriter<AgentToolsChanged>,
) {
    for assignment in tool_assignments.read() {
        if let Some((entity, agent, mut tools, permissions)) = query
            .iter_mut()
            .find(|(_, a, _, _)| AgentId::from_uuid(a.agent_id) == assignment.agent_id)
        {
            // Check if tool exists in registry
            if let Some(tool) = registry.tools.get(&assignment.tool_name) {
                // Check permissions
                let has_permission = tool.required_permissions.is_empty() || 
                    tool.required_permissions.iter().all(|required| {
                        permissions.permissions.iter()
                            .any(|p| &p.resource == required && !p.is_expired())
                    });
                
                if has_permission {
                    // Add tool to agent
                    tools.available_tools.insert(tool.name.clone(), tool.clone());
                    
                    // Usage tracking will be done when the tool is actually used
                    
                    // Create tool access for the event
                    let tool_access = ToolAccess::new(
                        tool.id.clone(),
                        tool.name.clone(),
                        match &tool.category {
                            ToolCategory::Analysis => ToolType::AIService,
                            ToolCategory::Query => ToolType::Database,
                            ToolCategory::Communication => ToolType::MessageQueue,
                            ToolCategory::Integration => ToolType::RestAPI,
                            _ => ToolType::Custom(format!("{:?}", tool.category)),
                        }
                    );
                    
                    // Trigger event
                    tools_changed.send(AgentToolsChanged {
                        agent_id: AgentId::from_uuid(agent.agent_id),
                        enabled: vec![tool_access],
                        disabled: vec![],
                        changed_at: chrono::Utc::now(),
                    });
                }
                // If permission denied, we don't send an event - just log it
            }
        }
    }
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
            .find(|(a, _, _)| AgentId::from_uuid(a.agent_id) == request.agent_id)
        {
            // Check if agent has the tool
            if let Some(tool) = tools.available_tools.get(&request.tool_name).cloned() {
                // Check if agent has execute capability
                if !capabilities.has("capability.execute") {
                    execution_responses.write(ExecuteToolResponse {
                        agent_id: request.agent_id.clone(),
                        tool_name: request.tool_name.clone(),
                        execution_id: request.execution_id,
                        result: ExecutionResult::failure(ExecutionError {
                            code: "NO_EXECUTE_CAPABILITY".to_string(),
                            message: "Agent does not have tool execution capability".to_string(),
                            stack_trace: None,
                            recoverable: false,
                            remediation: Some("Grant execute capability to the agent".to_string()),
                        }),
                    });
                    continue;
                }
                
                // Add to active executions
                tools.active_executions.insert(request.execution_id);
                
                // Simulate tool execution (in real implementation, this would call actual tool)
                let start_time = std::time::Instant::now();
                let result = execute_tool_mock(&tool, &request.parameters);
                let execution_time = start_time.elapsed();
                
                // Record usage
                let usage = ToolUsage {
                    tool_id: request.tool_name.clone(),
                    agent_id: agent.agent_id,
                    used_at: chrono::Utc::now(),
                    duration_ms: execution_time.as_millis() as u64,
                    success: result.success,
                    error: if result.success { None } else { result.error.as_ref().map(|e| e.message.clone()) },
                    input_summary: Some(request.parameters.clone()),
                    output_summary: result.output.clone(),
                };
                tools.tool_usage_history.push(usage);
                
                // Remove from active executions
                tools.active_executions.remove(&request.execution_id);
                
                // Send response
                execution_responses.write(ExecuteToolResponse {
                    agent_id: request.agent_id.clone(),
                    tool_name: request.tool_name.clone(),
                    execution_id: request.execution_id,
                    result,
                });
                
                // Tool execution doesn't trigger AgentToolsChanged
                // as it's not changing the tool availability
            } else {
                // Tool not available
                execution_responses.write(ExecuteToolResponse {
                    agent_id: request.agent_id.clone(),
                    tool_name: request.tool_name.clone(),
                    execution_id: request.execution_id,
                    result: ExecutionResult::failure(ExecutionError {
                        code: "TOOL_NOT_AVAILABLE".to_string(),
                        message: format!("Tool '{}' not available for agent", request.tool_name),
                        stack_trace: None,
                        recoverable: false,
                        remediation: Some("Assign the tool to the agent first".to_string()),
                    }),
                });
            }
        }
    }
}

/// System to remove tools from agents
pub fn handle_tool_removal(
    _commands: Commands,
    mut removal_requests: EventReader<RemoveToolRequest>,
    mut query: Query<(&AgentEntity, &mut ToolsComponent)>,
    mut tools_changed: EventWriter<AgentToolsChanged>,
) {
    for request in removal_requests.read() {
        if let Some((agent, mut tools)) = query
            .iter_mut()
            .find(|(a, _)| AgentId::from_uuid(a.agent_id) == request.agent_id)
        {
            if tools.available_tools.remove(&request.tool_name).is_some() {
                // Trigger event
                tools_changed.send(AgentToolsChanged {
                    agent_id: AgentId::from_uuid(agent.agent_id),
                    enabled: vec![],
                    disabled: vec![request.tool_name.clone()],
                    changed_at: chrono::Utc::now(),
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
    match &tool.category {
        ToolCategory::Analysis => {
            ExecutionResult::success(serde_json::json!({
                "analysis": "Mock analysis result",
                "parameters": parameters,
                "tool": tool.name
            }))
        }
        ToolCategory::Transformation => {
            ExecutionResult::success(serde_json::json!({
                "transformed": "Mock transformation result",
                "parameters": parameters,
                "tool": tool.name
            }))
        }
        ToolCategory::Query => {
            ExecutionResult::success(serde_json::json!({
                "query_result": "Mock query result",
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
        ToolCategory::DataManipulation => {
            ExecutionResult::success(serde_json::json!({
                "manipulated": "Mock data manipulation complete",
                "parameters": parameters,
                "tool": tool.name
            }))
        }
        ToolCategory::Integration => {
            ExecutionResult::success(serde_json::json!({
                "integrated": "Mock integration complete",
                "parameters": parameters,
                "tool": tool.name
            }))
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
        assert!(tools.tool_usage_history.is_empty());
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
        let tool = Tool::new(
            "test_tool".to_string(),
            "test_tool".to_string(),
            "Test tool".to_string(),
            ToolCategory::Analysis,
        );
        
        let params = serde_json::json!({ "test": true });
        let result = execute_tool_mock(&tool, &params);
        
        assert!(result.success);
        assert!(result.output.is_some());
    }
}
