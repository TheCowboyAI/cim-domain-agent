# CIM Domain Agent

Agent management domain for the Composable Information Machine.

## Overview

This module handles all agent-related operations in the CIM ecosystem. Agents are autonomous entities that can perform actions, execute workflows, and interact with other system components through event-driven communication.

## Key Concepts

### Agent
An autonomous entity with:
- **Identity**: Unique identifier and metadata
- **Capabilities**: What the agent can do (skills, tools, integrations)
- **Permissions**: What the agent is allowed to do (access control)
- **Status**: Current operational state (active, suspended, offline)
- **Configuration**: Runtime settings and parameters

### Agent Types
- **System Agents**: Core system functionality (e.g., workflow orchestrator)
- **Integration Agents**: External system connectors (e.g., API bridges)
- **AI Agents**: Machine learning and NLP capabilities
- **User Agents**: Human-in-the-loop interfaces

### Agent Lifecycle
```
Deployed → Activated → Running ↔ Suspended → Decommissioned
                          ↓
                      Offline
```

## Architecture

### Components
- `AgentMarker`: ECS component marking entities as agents
- `AgentMetadata`: Core agent information (name, type, version)
- `CapabilitiesComponent`: Agent skills and abilities
- `PermissionsComponent`: Access control rules
- `AuthenticationComponent`: Credentials and auth methods
- `ConfigurationComponent`: Runtime configuration
- `ToolAccessComponent`: External tool integrations

### Commands
- `DeployAgent`: Create new agent instance
- `ActivateAgent`: Start agent operations
- `SuspendAgent`: Temporarily halt agent
- `DecommissionAgent`: Permanently remove agent
- `UpdateAgentCapabilities`: Modify agent skills
- `GrantAgentPermissions`: Add access rights
- `RevokeAgentPermissions`: Remove access rights
- `EnableAgentTools`: Allow tool usage
- `DisableAgentTools`: Restrict tool usage

### Events
All events are published to NATS subjects under `agent.events.*` using the subject algebra in `src/subjects.rs`:
- `agent.events.deployed`
- `agent.events.activated`
- `agent.events.suspended`
- `agent.events.decommissioned`
- `agent.events.went_offline`
- `agent.events.capabilities_changed`
- `agent.events.permissions_changed`
- `agent.events.tools_changed`

## Usage Examples

### Deploy a New Agent
```rust
use cim_domain_agent::{DeployAgent, AgentType, AgentMetadata};

let command = DeployAgent {
    agent_id: AgentId::new(),
    metadata: AgentMetadata {
        name: "DataProcessor".to_string(),
        agent_type: AgentType::System,
        version: "1.0.0".to_string(),
        description: Some("Processes incoming data streams".to_string()),
    },
    initial_capabilities: vec!["data.read", "data.transform"],
    initial_configuration: HashMap::new(),
};

// Send command through NATS
nats_client.publish("agent.commands.deploy", &command).await?;
```

### Grant Permissions
```rust
use cim_domain_agent::{GrantAgentPermissions, Permission};

let command = GrantAgentPermissions {
    agent_id,
    permissions: vec![
        Permission::new("workflow.execute"),
        Permission::new("graph.read"),
    ],
    granted_by: operator_id,
    reason: "Enable workflow execution".to_string(),
};

nats_client.publish("agent.commands.grant_permissions", &command).await?;
```

### Query Agent Status
```rust
use cim_domain_agent::{AgentQuery, AgentQueryHandler};

let query = AgentQuery::ByStatus(AgentStatus::Active);
let active_agents = query_handler.handle(query).await?;
```

## Integration Points

### NATS Subjects
- Commands: `agent.commands.*`
- Events: `agent.events.*`
- Queries: `agent.queries.*`
See `src/subjects.rs` for canonical builders and patterns.

### Domain Dependencies
- **Policy Domain**: Permission validation and enforcement
- **Workflow Domain**: Agent task execution
- **Identity Domain**: Agent authentication and authorization
- **Graph Domain**: Agent relationship management

### External Integrations
Agents can integrate with external systems through:
- REST API clients
- GraphQL endpoints
- Database connections
- Message queue consumers
- File system watchers

## Configuration

### Environment Variables
- `AGENT_MAX_CONCURRENT`: Maximum concurrent agents (default: 100)
- `AGENT_HEARTBEAT_INTERVAL`: Health check interval (default: 30s)
- `AGENT_TIMEOUT`: Offline detection timeout (default: 5m)

### Agent Configuration Schema
```rust
pub struct AgentConfig {
    pub runtime: RuntimeConfig,
    pub resources: ResourceLimits,
    pub retry_policy: RetryPolicy,
    pub monitoring: MonitoringConfig,
}
```

## Security Considerations

### Authentication
- Agents authenticate using JWT tokens or API keys
- Credentials stored securely in configuration
- Regular credential rotation supported

### Authorization
- Fine-grained permission model
- Policy-based access control
- Audit logging for all operations

### Isolation
- Agents run in isolated contexts
- Resource limits enforced
- Network access controlled

## Monitoring and Observability

### Metrics
- Agent count by status
- Command processing latency
- Event publication rate
- Tool usage statistics

### Health Checks
- Periodic heartbeat monitoring
- Resource usage tracking
- Error rate monitoring

### Logging
- Structured logging with correlation IDs
- Event stream capture
- Debug mode for troubleshooting

## Testing

Run checks with Nix:
```bash
nix flake check
```

### Test Coverage
- Unit tests for all aggregates
- Integration tests for command handlers
- Event flow tests with mock NATS
- Property-based tests for state machines

## Contributing

1. Follow DDD principles - agents are aggregates
2. All state changes through events
3. Commands validate business rules
4. Maintain event sourcing patterns
5. Add tests for new functionality

## License

See the main project LICENSE file. 