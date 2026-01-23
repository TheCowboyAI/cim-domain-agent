# NATS Subject Migration Guide: From Inbox Pattern to Reference Theory Pattern

**Version:** 1.0
**Date:** 2026-01-22
**Status:** RFC (Request for Comments)

## Executive Summary

This guide provides a step-by-step migration from the current "inbox pattern" to the theoretically-grounded reference pattern based on Frege/Russell/Evans/Searle.

**Current Pattern (DEPRECATED):**
```
agent.to.{recipient-name}.>
agent.to.{recipient-name}.from.{sender-name}.{type}
```

**New Pattern (RECOMMENDED):**
```
agent.{capability-cluster}.{agent-name}.{agent-id}.{operation}.{detail}
```

**Migration Timeline:** 6 months (dual-publishing), then full cutover

## Why Migrate?

### Problems with Current Pattern

1. **Asymmetric** - "to recipient" but also "from sender" (inconsistent)
2. **Name-based only** - No AgentId (breaks on renames)
3. **No capability routing** - Can't subscribe to "all orchestrators"
4. **Spatial metaphor** - "inbox" implies mailbox (wrong abstraction)
5. **No reference theory foundation** - Arbitrary structure

### Benefits of New Pattern

1. **Mathematically rigorous** - Based on Frege/Russell/Evans/Searle
2. **Dual reference** - Both name (sense) and ID (reference)
3. **Cluster-based routing** - Subscribe by capability
4. **Compositional** - Subject algebra (monoid operations)
5. **Stable across renames** - ID is rigid designator

## Phase 1: Preparation (Week 1-2)

### 1.1 Assign Capability Clusters

Update each agent definition with `capability_cluster`:

```yaml
# agents/sage.md
---
agent:
  id: ""
  name: "sage"
  display_name: "SAGE - Master CIM Orchestrator"
  version: "0.1.0"

conceptual_space:
  boundary: "meta-orchestrator"
  capability_cluster: "orchestration"  # ← ADD THIS
```

**Cluster Assignments:**

| Cluster | Agents |
|---------|--------|
| `orchestration` | sage |
| `domain-modeling` | ddd-expert, domain-expert, domain-ontologist-researcher |
| `event-analysis` | event-storming-expert |
| `infrastructure` | nats-expert, nix-expert, network-expert |
| `quality-assurance` | qa-expert, tdd-expert, bdd-expert |
| `functional-programming` | fp-expert, frp-expert, act-expert |
| `ui-design` | egui-ui-expert, iced-ui-expert, cim-ui-layer-expert |
| `sdlc` | git-expert, sdlc-distributed-expert |
| `conceptual-analysis` | language-expert, graph-expert, conceptual-spaces-expert, description-expert |
| `domain-entities` | people-expert, org-expert, location-expert, subject-expert |

### 1.2 Update AgentSubjectFactory

Create new subject factory in `src/infrastructure/subject_factory_v2.rs`:

```rust
// Copyright (c) 2025 - Cowboy AI, Inc.

//! Agent Subject Factory v2.0 - Reference Theory Pattern
//!
//! Implements: agent.{cluster}.{name}.{id}.{operation}.{detail}

use crate::value_objects::AgentId;
use crate::infrastructure::DomainResult;

pub struct AgentSubjectFactoryV2;

impl AgentSubjectFactoryV2 {
    /// Generate command subject
    pub fn command(
        capability: &str,
        name: &str,
        id: AgentId,
        command_type: &str,
    ) -> DomainResult<String> {
        Ok(format!(
            "agent.{}.{}.{}.command.{}",
            capability, name, id, command_type
        ))
    }

    /// Generate event subject
    pub fn event(
        capability: &str,
        name: &str,
        id: AgentId,
        event_type: &str,
    ) -> DomainResult<String> {
        Ok(format!(
            "agent.{}.{}.{}.event.{}",
            capability, name, id, event_type
        ))
    }

    /// Generate query subject
    pub fn query(
        capability: &str,
        name: &str,
        id: AgentId,
        query_type: &str,
    ) -> DomainResult<String> {
        Ok(format!(
            "agent.{}.{}.{}.query.{}",
            capability, name, id, query_type
        ))
    }

    /// Generate reply subject
    pub fn reply(
        capability: &str,
        name: &str,
        id: AgentId,
        correlation_id: uuid::Uuid,
    ) -> DomainResult<String> {
        Ok(format!(
            "agent.{}.{}.{}.reply.{}",
            capability, name, id, correlation_id
        ))
    }

    // Subscription patterns
    pub fn subscribe_by_id(id: AgentId, operation: &str) -> String {
        format!("agent.*.*.{}.{}.>", id, operation)
    }

    pub fn subscribe_by_cluster(capability: &str, operation: &str) -> String {
        format!("agent.{}.*.*.{}.>", capability, operation)
    }

    pub fn subscribe_by_name(name: &str, operation: &str) -> String {
        format!("agent.*.{}.*.{}.>", name, operation)
    }

    pub fn subscribe_all_operations(id: AgentId) -> String {
        format!("agent.*.*.{}.>", id)
    }
}
```

### 1.3 Create Migration Config

Add feature flag to toggle between old and new patterns:

```rust
// src/infrastructure/migration_config.rs

#[derive(Debug, Clone)]
pub struct MigrationConfig {
    /// Publish to both old and new subjects (dual-publish)
    pub dual_publish: bool,

    /// Subscribe to both old and new patterns (dual-subscribe)
    pub dual_subscribe: bool,

    /// Use new pattern as primary (when dual_publish=true)
    pub new_pattern_primary: bool,
}

impl Default for MigrationConfig {
    fn default() -> Self {
        Self {
            dual_publish: true,      // Phase 2: Start with dual publish
            dual_subscribe: true,    // Phase 2: Subscribe to both
            new_pattern_primary: false,  // Phase 3: Switch to new pattern
        }
    }
}
```

## Phase 2: Dual Publishing (Week 3-8, ~1.5 months)

### 2.1 Implement Dual Publisher

Publish to BOTH old and new subjects:

```rust
// src/infrastructure/dual_publisher.rs

pub struct DualPublisher {
    jetstream: jetstream::Context,
    config: MigrationConfig,
    factory_v1: AgentSubjectFactory,  // Old pattern
    factory_v2: AgentSubjectFactoryV2, // New pattern
}

impl DualPublisher {
    pub async fn publish_command(
        &self,
        agent_id: AgentId,
        agent_name: &str,
        capability: &str,
        command_type: &str,
        payload: Vec<u8>,
    ) -> Result<(), PublishError> {
        if self.config.dual_publish {
            // Publish to OLD subject
            let old_subject = format!("agent.to.{}.command.{}", agent_name, command_type);
            self.jetstream.publish(old_subject, payload.clone().into()).await?;

            // Publish to NEW subject
            let new_subject = self.factory_v2.command(
                capability, agent_name, agent_id, command_type
            )?;
            self.jetstream.publish(new_subject, payload.into()).await?;
        } else if self.config.new_pattern_primary {
            // Only new pattern
            let subject = self.factory_v2.command(
                capability, agent_name, agent_id, command_type
            )?;
            self.jetstream.publish(subject, payload.into()).await?;
        } else {
            // Only old pattern (backward compat)
            let subject = format!("agent.to.{}.command.{}", agent_name, command_type);
            self.jetstream.publish(subject, payload.into()).await?;
        }

        Ok(())
    }
}
```

### 2.2 Implement Dual Subscriber

Subscribe to BOTH old and new patterns:

```rust
// src/infrastructure/dual_subscriber.rs

pub struct DualSubscriber {
    client: async_nats::Client,
    config: MigrationConfig,
}

impl DualSubscriber {
    pub async fn subscribe_commands(
        &self,
        agent_id: AgentId,
        agent_name: &str,
    ) -> Result<Vec<async_nats::Subscriber>, SubscribeError> {
        let mut subscribers = Vec::new();

        if self.config.dual_subscribe {
            // Subscribe to OLD pattern
            let old_pattern = format!("agent.to.{}.command.>", agent_name);
            subscribers.push(self.client.subscribe(old_pattern).await?);

            // Subscribe to NEW pattern
            let new_pattern = AgentSubjectFactoryV2::subscribe_by_id(
                agent_id, "command"
            );
            subscribers.push(self.client.subscribe(new_pattern).await?);
        } else if self.config.new_pattern_primary {
            // Only new pattern
            let pattern = AgentSubjectFactoryV2::subscribe_by_id(agent_id, "command");
            subscribers.push(self.client.subscribe(pattern).await?);
        } else {
            // Only old pattern
            let pattern = format!("agent.to.{}.command.>", agent_name);
            subscribers.push(self.client.subscribe(pattern).await?);
        }

        Ok(subscribers)
    }
}
```

### 2.3 Monitor Migration Progress

Track usage of old vs new patterns:

```rust
// src/infrastructure/migration_metrics.rs

use std::sync::atomic::{AtomicU64, Ordering};

pub struct MigrationMetrics {
    old_pattern_publishes: AtomicU64,
    new_pattern_publishes: AtomicU64,
    old_pattern_receives: AtomicU64,
    new_pattern_receives: AtomicU64,
}

impl MigrationMetrics {
    pub fn record_publish(&self, is_new_pattern: bool) {
        if is_new_pattern {
            self.new_pattern_publishes.fetch_add(1, Ordering::Relaxed);
        } else {
            self.old_pattern_publishes.fetch_add(1, Ordering::Relaxed);
        }
    }

    pub fn record_receive(&self, subject: &str) {
        if subject.starts_with("agent.") && subject.split('.').count() >= 5 {
            self.new_pattern_receives.fetch_add(1, Ordering::Relaxed);
        } else {
            self.old_pattern_receives.fetch_add(1, Ordering::Relaxed);
        }
    }

    pub fn report(&self) -> MigrationReport {
        MigrationReport {
            old_publishes: self.old_pattern_publishes.load(Ordering::Relaxed),
            new_publishes: self.new_pattern_publishes.load(Ordering::Relaxed),
            old_receives: self.old_pattern_receives.load(Ordering::Relaxed),
            new_receives: self.new_pattern_receives.load(Ordering::Relaxed),
        }
    }
}

pub struct MigrationReport {
    pub old_publishes: u64,
    pub new_publishes: u64,
    pub old_receives: u64,
    pub new_receives: u64,
}

impl MigrationReport {
    pub fn new_pattern_percentage(&self) -> f64 {
        let total_publishes = self.old_publishes + self.new_publishes;
        if total_publishes == 0 {
            return 0.0;
        }
        (self.new_publishes as f64 / total_publishes as f64) * 100.0
    }
}
```

### 2.4 Weekly Migration Reports

Generate weekly reports:

```bash
#!/bin/bash
# scripts/migration-report.sh

echo "=== NATS Subject Migration Report ==="
echo "Date: $(date -I)"
echo ""

# Query NATS monitoring
nats -s nats://10.0.20.1:4222 monitor --json | jq '
  .messages |
  map(select(.subject | startswith("agent."))) |
  group_by(.subject | split(".") | length) |
  map({
    pattern: (if (.[0].subject | split(".") | length) >= 5 then "NEW" else "OLD" end),
    count: length
  })
'

echo ""
echo "Target: 100% new pattern by Week 8"
```

## Phase 3: Primary Cutover (Week 9-12, ~1 month)

### 3.1 Switch to New Pattern Primary

Update config:

```rust
MigrationConfig {
    dual_publish: true,          // Still publish both
    dual_subscribe: true,         // Still subscribe both
    new_pattern_primary: true,    // ← Switch primary
}
```

### 3.2 Update Documentation

Update all docs to show new pattern as primary:

```markdown
# Agent Communication Patterns

**CURRENT (Recommended):**
```
agent.orchestration.sage.{uuid}.command.task_analysis
```

**DEPRECATED (Legacy, will be removed):**
```
agent.to.sage.command.task_analysis
```
```

### 3.3 Deprecation Warnings

Add warnings when old pattern detected:

```rust
pub fn detect_old_pattern(subject: &str) -> Option<String> {
    if subject.starts_with("agent.to.") {
        let new_pattern = migrate_subject_to_v2(subject)?;
        Some(format!(
            "WARNING: Old subject pattern detected: {}\n\
             Consider migrating to: {}",
            subject, new_pattern
        ))
    } else {
        None
    }
}

fn migrate_subject_to_v2(old_subject: &str) -> Option<String> {
    // Parse old: agent.to.{name}.{operation}.{detail}
    let parts: Vec<&str> = old_subject.split('.').collect();
    if parts.len() < 4 || parts[1] != "to" {
        return None;
    }

    let agent_name = parts[2];
    let operation = parts[3];
    let detail = parts.get(4).unwrap_or(&"");

    // Need to look up agent_id and capability from registry
    // For now, show template:
    Some(format!(
        "agent.{{cluster}}.{}.{{agent-id}}.{}.{}",
        agent_name, operation, detail
    ))
}
```

## Phase 4: Full Cutover (Week 13-16, ~1 month)

### 4.1 Stop Publishing to Old Pattern

Update config:

```rust
MigrationConfig {
    dual_publish: false,         // ← Stop dual publishing
    dual_subscribe: true,         // Still subscribe (grace period)
    new_pattern_primary: true,
}
```

### 4.2 Monitor for Old Pattern Usage

Alert if old pattern messages detected:

```rust
pub async fn monitor_old_pattern_usage(
    subscriber: &mut async_nats::Subscriber,
) {
    while let Some(msg) = subscriber.next().await {
        if msg.subject.starts_with("agent.to.") {
            eprintln!(
                "ERROR: Old pattern message received: {}\n\
                 Please migrate sender to new pattern!",
                msg.subject
            );
            // Send alert to ops team
        }
    }
}
```

### 4.3 Grace Period (2 weeks)

Keep old pattern subscriptions active for 2 more weeks to catch stragglers.

### 4.4 Final Cutover

After grace period, remove all old pattern code:

```rust
MigrationConfig {
    dual_publish: false,
    dual_subscribe: false,     // ← Remove old subscriptions
    new_pattern_primary: true,
}
```

## Phase 5: Cleanup (Week 17-18)

### 5.1 Remove Old Code

Delete deprecated code:

```bash
# Remove old subject factory
rm src/infrastructure/subject_factory_v1.rs

# Remove migration code (no longer needed)
rm src/infrastructure/dual_publisher.rs
rm src/infrastructure/dual_subscriber.rs
rm src/infrastructure/migration_config.rs
rm src/infrastructure/migration_metrics.rs

# Rename v2 to primary
mv src/infrastructure/subject_factory_v2.rs src/infrastructure/subject_factory.rs
```

### 5.2 Update Tests

Remove all old pattern tests:

```rust
// REMOVE these tests:
#[test]
fn test_old_inbox_pattern() { ... }

// KEEP these tests:
#[test]
fn test_new_reference_pattern() { ... }
```

### 5.3 Final Documentation Update

Update all docs to only show new pattern:

```markdown
# NATS Subject Patterns (v2.0)

All agent communication uses the reference theory pattern:

```
agent.{capability}.{name}.{id}.{operation}.{detail}
```

Examples:
- Commands: `agent.orchestration.sage.{uuid}.command.task_analysis`
- Events: `agent.domain-modeling.ddd-expert.{uuid}.event.boundary_defined`
- Queries: `agent.event-analysis.eventstorming-expert.{uuid}.query.domain_events`
```

## Testing Strategy

### Unit Tests

Test both old and new patterns during migration:

```rust
#[cfg(test)]
mod migration_tests {
    use super::*;

    #[test]
    fn test_dual_publish() {
        let config = MigrationConfig {
            dual_publish: true,
            dual_subscribe: false,
            new_pattern_primary: false,
        };

        // Verify both subjects generated
        let old = old_pattern_subject("sage", "command");
        let new = new_pattern_subject("orchestration", "sage", agent_id, "command");

        assert_ne!(old, new);  // Different patterns
        // Both should be published
    }

    #[test]
    fn test_new_pattern_only() {
        let config = MigrationConfig {
            dual_publish: false,
            dual_subscribe: false,
            new_pattern_primary: true,
        };

        // Only new pattern should be used
    }
}
```

### Integration Tests

Test real NATS communication:

```rust
#[tokio::test]
async fn test_migration_dual_subscribe() {
    let client = connect_nats().await?;

    // Subscribe to both patterns
    let old_sub = client.subscribe("agent.to.sage.command.>").await?;
    let new_sub = client.subscribe("agent.orchestration.sage.*.command.>").await?;

    // Publish to old pattern
    client.publish("agent.to.sage.command.test", "payload".into()).await?;

    // Should receive on old subscription
    let msg = timeout(Duration::from_millis(100), old_sub.next()).await??;
    assert!(msg.is_some());

    // Publish to new pattern
    let subject = format!("agent.orchestration.sage.{}.command.test", agent_id);
    client.publish(subject, "payload".into()).await?;

    // Should receive on new subscription
    let msg = timeout(Duration::from_millis(100), new_sub.next()).await??;
    assert!(msg.is_some());
}
```

## Rollback Plan

If issues arise, rollback to previous phase:

### From Phase 4 → Phase 3

```rust
// Re-enable dual publishing
MigrationConfig {
    dual_publish: true,      // ← Re-enable
    dual_subscribe: true,
    new_pattern_primary: true,
}
```

### From Phase 3 → Phase 2

```rust
// Switch back to old pattern as primary
MigrationConfig {
    dual_publish: true,
    dual_subscribe: true,
    new_pattern_primary: false,  // ← Switch back
}
```

### From Phase 2 → Phase 1

```rust
// Disable dual operations
MigrationConfig {
    dual_publish: false,   // ← Disable
    dual_subscribe: false, // ← Disable
    new_pattern_primary: false,
}
```

## Success Criteria

### Phase 2 Success (Dual Publishing)
- [ ] All agents publish to both old and new subjects
- [ ] All agents subscribe to both patterns
- [ ] No message loss detected
- [ ] Metrics show 50%+ new pattern usage

### Phase 3 Success (Primary Cutover)
- [ ] New pattern is primary
- [ ] All new code uses new pattern
- [ ] Documentation updated
- [ ] Deprecation warnings active
- [ ] Metrics show 80%+ new pattern usage

### Phase 4 Success (Full Cutover)
- [ ] Old pattern publishing stopped
- [ ] Metrics show 100% new pattern usage
- [ ] No old pattern messages in logs
- [ ] All agents on new pattern

### Phase 5 Success (Cleanup)
- [ ] Old code removed
- [ ] Tests updated
- [ ] Documentation finalized
- [ ] Zero technical debt

## Timeline Summary

| Phase | Duration | Key Milestone |
|-------|----------|---------------|
| Phase 1: Prep | 2 weeks | Capability clusters assigned, v2 factory implemented |
| Phase 2: Dual Pub | 6 weeks | 50%+ new pattern usage |
| Phase 3: Primary | 4 weeks | 80%+ new pattern usage, docs updated |
| Phase 4: Cutover | 4 weeks | 100% new pattern, old pub stopped |
| Phase 5: Cleanup | 2 weeks | Old code removed, docs finalized |
| **Total** | **18 weeks (~4.5 months)** | Complete migration |

## References

- `AGENT_REFERENCE_THEORY.md` - Theoretical foundation
- `examples/agent_reference_pattern.rs` - Implementation example
- `agents/conceptual-spaces-expert.md` v0.7.0 - Searle cluster theory
- `src/infrastructure/nats_integration.rs` - Current implementation

## Appendix: Quick Reference

### Old Pattern (DEPRECATED)
```
agent.to.{name}.command.{type}
agent.to.{name}.from.{sender}.{type}
```

### New Pattern (CURRENT)
```
agent.{cluster}.{name}.{id}.command.{type}
agent.{cluster}.{name}.{id}.event.{type}
agent.{cluster}.{name}.{id}.query.{type}
agent.{cluster}.{name}.{id}.reply.{correlation-id}
```

### Subscription Patterns
```rust
// By ID (stable, recommended):
"agent.*.*.{agent-id}.command.>"

// By cluster (broadcast):
"agent.{cluster}.*.*.command.>"

// By name (fragile, use sparingly):
"agent.*.{name}.*.command.>"

// All commands:
"agent.*.*.*.command.>"
```

---

**Next Steps:**
1. Review this RFC with team
2. Approve migration timeline
3. Assign capability clusters (Phase 1)
4. Begin implementation (Phase 2)
