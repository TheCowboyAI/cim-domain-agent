<!-- Copyright (c) 2025 - Cowboy AI, LLC. -->

# Unified Subject Architecture Implementation Plan

**Date**: 2026-01-22
**Status**: Sprint Planning Complete
**Project**: Migration from v0.9.3 inbox pattern to unified subject architecture

---

## Executive Summary

This plan coordinates the migration from the broken "inbox pattern" (v0.9.3) to the unified subject architecture designed by Subject Expert and Description Expert. The migration will be completed over 6 sprints with zero downtime, following the 10-step SDLC workflow.

## Current State

- **Repository**: `/git/thecowboyai/cim-domain-agent`
- **Branch**: `main`
- **Version**: v0.9.3
- **Production**: 31 agents deployed on DGX (aarch64 ARM64)
- **Problem**: Inbox pattern `agent.to.{recipient}.from.{sender}.{type}` violates free monoid algebra and reference theory

## Target State

- **Conversation-based subjects** (primary, 90% of use cases):
  ```
  Subject:  agent.conversations.{conversation_id}.{message_type}
  Headers:  Sender: {capability-cluster}.{agent-name}.{agent-id}
            Recipient: {capability-cluster}.{agent-name}.{agent-id}
  ```

- **Command-based subjects** (direct operations, 10%):
  ```
  Subject: agent.{capability-cluster}.{agent-name}.{agent-id}.command.{command_type}
  ```

- **Event subjects** (state changes):
  ```
  Subject: agent.{capability-cluster}.{agent-name}.{agent-id}.event.{event_type}
  ```

---

## Sprint Structure

Each sprint follows the 10-step SDLC workflow:
1. Create objective
2. Develop written design
3. Develop stepped sprint plan
4. Write status to progress.json at each step
5. Commit each step to git
6. Test at each sprint
7. Full written retrospective at each sprint
8. Adjust plan if retrospective calls for it
9. Ask questions if unclear
10. Final analysis after all sprints

---

## Sprint 0: Planning and Documentation Review

**Objective**: Understand the complete architecture and create detailed sprint plans

### Tasks
- [x] Read START_HERE.md
- [x] Read UNIFIED_SUBJECT_ARCHITECTURE.md
- [x] Read SUBJECT_REFACTORING_GUIDE.md
- [x] Review current codebase structure
- [x] Create comprehensive sprint plan
- [ ] Initialize progress.json
- [ ] Commit sprint plan

### Success Criteria
- Complete understanding of unified architecture
- Detailed sprint plans for all phases
- progress.json tracking initialized
- All stakeholders aligned

### Deliverables
- `doc/plans/unified-subject-architecture.md` (this document)
- `progress.json` (tracking file)

---

## Sprint 1: Phase 1 - Foundation (Value Objects)

**Duration**: Week 1
**Objective**: Implement core value objects with comprehensive tests

### 1.1: ConversationId Value Object

**Tasks**:
- Create `src/value_objects/conversation_id.rs`
- Implement with UUID v7 (time-ordered)
- Add Display, Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize
- Write comprehensive unit tests

**Files Modified**:
- `src/value_objects/conversation_id.rs` (new)
- `src/value_objects/mod.rs` (export ConversationId)

**Tests**:
```rust
#[test]
fn test_conversation_id_creation()
fn test_conversation_id_display()
fn test_conversation_id_serialization()
fn test_conversation_id_from_uuid()
```

**Commit Message**: "feat: Add ConversationId value object with UUID v7"

### 1.2: CapabilityCluster Enum

**Tasks**:
- Create `src/value_objects/capability_cluster.rs`
- Implement enum with all 10 clusters
- Add Display for subject generation
- Add serialization support
- Write comprehensive unit tests

**Capability Clusters**:
```rust
pub enum CapabilityCluster {
    Orchestration,           // sage
    DomainModeling,          // ddd-expert, domain-expert, domain-ontologist-researcher
    EventAnalysis,           // event-storming-expert
    Infrastructure,          // nats-expert, nix-expert, network-expert
    QualityAssurance,        // qa-expert, tdd-expert, bdd-expert
    FunctionalProgramming,   // fp-expert, frp-expert, act-expert
    UiDesign,                // egui-ui-expert, iced-ui-expert, cim-ui-layer-expert, cim-tea-ecs-expert
    Sdlc,                    // git-expert, sdlc-expert, sdlc-distributed-expert
    ConceptualAnalysis,      // language-expert, graph-expert, conceptual-spaces-expert, description-expert
    DomainEntities,          // people-expert, org-expert, location-expert, subject-expert
}
```

**Files Modified**:
- `src/value_objects/capability_cluster.rs` (new)
- `src/value_objects/mod.rs` (export CapabilityCluster)

**Tests**:
```rust
#[test]
fn test_capability_cluster_display()
fn test_capability_cluster_serialization()
fn test_capability_cluster_from_agent_name()
```

**Commit Message**: "feat: Add CapabilityCluster enum for agent organization"

### 1.3: AgentReference Type

**Tasks**:
- Create `src/value_objects/agent_reference.rs`
- Implement composite type (CapabilityCluster, AgentName, AgentId)
- Add `to_header_value()` method for NATS headers
- Add `from_header_value()` parser
- Write comprehensive unit tests

**Files Modified**:
- `src/value_objects/agent_reference.rs` (new)
- `src/value_objects/mod.rs` (export AgentReference)

**Tests**:
```rust
#[test]
fn test_agent_reference_creation()
fn test_agent_reference_header_value()
fn test_agent_reference_parsing()
fn test_agent_reference_serialization()
```

**Commit Message**: "feat: Add AgentReference composite type for agent identification"

### 1.4: Integration and Testing

**Tasks**:
- Run full test suite: `cargo test --all-features`
- Fix any compilation errors
- Update Cargo.toml version to 0.10.0-alpha.1
- Run clippy: `cargo clippy --all-features`
- Update progress.json

**Success Criteria**:
- All tests passing
- No compilation errors
- No clippy warnings
- Value objects integrated into codebase

**Commit Message**: "test: Integrate Phase 1 value objects with full test coverage"

### Sprint 1 Retrospective

**Deliverable**: `retrospectives/sprint_1.md`

**Required Content**:
- What was accomplished
- What worked well
- Lessons learned
- Files modified
- Build status
- Blockers (if any)

---

## Sprint 2: Phase 2 - Subject Factory V2

**Duration**: Week 2
**Objective**: Implement conversation-based subject methods

### 2.1: Conversation Subject Segments

**Tasks**:
- Add static segments to `subject_factory.rs`
- Add CONVERSATIONS, REQUEST, RESPONSE, ERROR segments
- Update segments module

**Files Modified**:
- `src/infrastructure/subject_factory.rs`

**Commit Message**: "feat: Add conversation-related subject segments"

### 2.2: Conversation Subject Methods

**Tasks**:
- Implement `conversation_request(conv_id)`
- Implement `conversation_response(conv_id)`
- Implement `conversation_error(conv_id)`
- Implement `conversation_pattern(conv_id)`
- Add comprehensive documentation

**Files Modified**:
- `src/infrastructure/subject_factory.rs`

**Tests**:
```rust
#[test]
fn test_conversation_request_subject()
fn test_conversation_response_subject()
fn test_conversation_error_subject()
fn test_conversation_pattern_matching()
```

**Commit Message**: "feat: Implement conversation-based subject methods"

### 2.3: Agent Command Methods (Updated)

**Tasks**:
- Implement `agent_command(agent_ref, command_type)`
- Update to use CapabilityCluster + AgentName + AgentId
- Add command pattern subscription methods

**Files Modified**:
- `src/infrastructure/subject_factory.rs`

**Tests**:
```rust
#[test]
fn test_agent_command_subject()
fn test_agent_command_pattern()
fn test_command_routing_by_id()
fn test_command_routing_by_cluster()
```

**Commit Message**: "feat: Update agent command methods for reference architecture"

### 2.4: Agent Event Methods (Updated)

**Tasks**:
- Implement `agent_event(agent_ref, event_type)`
- Update to use CapabilityCluster + AgentName + AgentId
- Add event pattern subscription methods

**Files Modified**:
- `src/infrastructure/subject_factory.rs`

**Tests**:
```rust
#[test]
fn test_agent_event_subject()
fn test_agent_event_pattern()
fn test_event_routing_by_id()
fn test_event_routing_by_cluster()
```

**Commit Message**: "feat: Update agent event methods for reference architecture"

### 2.5: Integration Testing

**Tasks**:
- Run full test suite
- Verify all subject generation methods work
- Test pattern matching with cim-domain Subject algebra
- Update progress.json

**Success Criteria**:
- All tests passing
- Subject factory V2 complete
- No breaking changes to existing code

**Commit Message**: "test: Complete Phase 2 subject factory V2 integration"

### Sprint 2 Retrospective

**Deliverable**: `retrospectives/sprint_2.md`

---

## Sprint 3: Phase 3 - Dual Publishing (Part 1)

**Duration**: Week 3
**Objective**: Implement dual publishing in agent-service.rs

### 3.1: AgentReference Configuration

**Tasks**:
- Add CAPABILITY_CLUSTER to agent environment variables
- Update agent configuration parsing
- Add AgentReference to AgentRuntime state

**Files Modified**:
- `src/bin/agent-service.rs`
- Example: `/opt/cim-dgx/configs/agent-runtime-sage.env`

**Environment Variables**:
```bash
CAPABILITY_CLUSTER=orchestration
AGENT_NAME=sage
AGENT_ID=<uuid>
```

**Commit Message**: "feat: Add capability cluster configuration to agents"

### 3.2: Dual Publishing Implementation

**Tasks**:
- Update message publishing to send to BOTH patterns
- Old pattern: `agent.to.{recipient}.from.{sender}.{type}`
- New pattern: `agent.conversations.{conv_id}.{type}` + headers
- Add feature flag `USE_CONVERSATION_SUBJECTS`

**Files Modified**:
- `src/bin/agent-service.rs`

**Logic**:
```rust
async fn publish_message(&self, msg: Message) -> Result<()> {
    // Always publish to NEW pattern
    let conv_id = msg.conversation_id.unwrap_or_else(ConversationId::new);
    let subject = self.factory.conversation_request(&conv_id)?;
    let headers = self.create_agent_headers(&msg)?;
    self.nats.publish_with_headers(subject.to_string(), headers, msg.payload).await?;

    // Also publish to OLD pattern for backward compatibility
    let old_subject = self.factory.agent_to_agent(
        &self.agent_ref.name,
        &msg.recipient,
        "request"
    )?;
    self.nats.publish(old_subject.to_string(), msg.payload.clone()).await?;

    Ok(())
}
```

**Commit Message**: "feat: Implement dual publishing to old and new subject patterns"

### 3.3: Dual Subscription Implementation

**Tasks**:
- Subscribe to BOTH old and new patterns
- Add message deduplication logic
- Track which pattern messages arrive on

**Files Modified**:
- `src/bin/agent-service.rs`

**Commit Message**: "feat: Subscribe to both old and new subject patterns"

### 3.4: Local Testing

**Tasks**:
- Test dual publishing locally
- Verify messages arrive on both patterns
- Test deduplication works
- Update progress.json

**Success Criteria**:
- Messages published to both patterns
- Agents receive messages from both patterns
- No duplicates processed
- Zero downtime

**Commit Message**: "test: Verify dual publishing works locally"

### Sprint 3 Retrospective

**Deliverable**: `retrospectives/sprint_3.md`

---

## Sprint 4: Phase 3 - Dual Publishing (Part 2 - DGX Testing)

**Duration**: Week 4
**Objective**: Deploy dual publishing to DGX and verify

### 4.1: Build for DGX

**Tasks**:
- Build natively on DGX (aarch64 ARM64)
- NEVER cross-compile
- Test binary locally on DGX

**Commands**:
```bash
# On DGX
cd /git/thecowboyai/cim-domain-agent
cargo build --release --all-features
```

**Commit Message**: "build: Native build for DGX aarch64"

### 4.2: Configuration Update

**Tasks**:
- Update all 31 agent configs with CAPABILITY_CLUSTER
- Verify environment variable parsing
- Test configuration loading

**Files Modified**:
- `/opt/cim-dgx/configs/agent-runtime-*.env` (all 31)

**Capability Cluster Mapping**:
```
orchestration: sage
domain-modeling: ddd-expert, domain-expert, domain-ontologist-researcher
event-analysis: event-storming-expert
infrastructure: nats-expert, nix-expert, network-expert
quality-assurance: qa-expert, tdd-expert, bdd-expert
functional-programming: fp-expert, frp-expert, act-expert
ui-design: egui-ui-expert, iced-ui-expert, cim-ui-layer-expert, cim-tea-ecs-expert
sdlc: git-expert, sdlc-expert, sdlc-distributed-expert
conceptual-analysis: language-expert, graph-expert, conceptual-spaces-expert, description-expert
domain-entities: people-expert, org-expert, location-expert, subject-expert
```

**Commit Message**: "config: Add capability clusters to all 31 agent configurations"

### 4.3: Rolling Deployment

**Tasks**:
- Deploy agents one cluster at a time
- Start with orchestration (sage)
- Monitor NATS traffic for both patterns
- Verify conversations work

**Deployment Order**:
1. Orchestration (sage)
2. Domain Modeling (3 agents)
3. Infrastructure (3 agents)
4. Conceptual Analysis (4 agents)
5. Quality Assurance (3 agents)
6. Remaining clusters

**Success Criteria**:
- All 31 agents running
- Messages published to both patterns
- Conversations working normally
- Zero downtime

**Commit Message**: "deploy: Rolling deployment of dual publishing to DGX"

### 4.4: Monitoring and Verification

**Tasks**:
- Monitor NATS subjects for 24 hours
- Verify message delivery
- Check for errors in logs
- Document any issues
- Update progress.json

**Metrics**:
- Messages on old pattern: `agent.to.>`
- Messages on new pattern: `agent.conversations.>`
- Error rate: target < 0.1%
- Response latency: target < 100ms

**Commit Message**: "monitor: Verify dual publishing on DGX production"

### Sprint 4 Retrospective

**Deliverable**: `retrospectives/sprint_4.md`

---

## Sprint 5: Phase 4 - Primary Cutover

**Duration**: Week 5
**Objective**: Switch primary subscriptions to new pattern

### 5.1: Update Primary Subscriptions

**Tasks**:
- Change primary subscription to conversation pattern
- Keep old pattern as fallback
- Add monitoring for pattern usage

**Files Modified**:
- `src/bin/agent-service.rs`

**Logic**:
```rust
async fn subscribe(&self) -> Result<()> {
    // PRIMARY: Subscribe to conversations
    let mut conv_sub = self.subscribe_conversations().await?;

    // FALLBACK: Keep old inbox subscription
    let mut inbox_sub = self.subscribe_inbox().await?;

    // Process messages (prefer conversation pattern)
    loop {
        tokio::select! {
            Some(msg) = conv_sub.next() => {
                self.handle_message(msg, MessageSource::Conversation).await?;
            }
            Some(msg) = inbox_sub.next() => {
                // Only process if not already received via conversation
                if !self.already_processed(&msg)? {
                    self.handle_message(msg, MessageSource::Inbox).await?;
                }
            }
        }
    }
}
```

**Commit Message**: "feat: Switch primary subscriptions to conversation pattern"

### 5.2: Update All 31 Agent Configurations

**Tasks**:
- Add `PRIMARY_PATTERN=conversation` to all configs
- Rolling restart of all agents
- Monitor for issues

**Files Modified**:
- `/opt/cim-dgx/configs/agent-runtime-*.env` (all 31)

**Commit Message**: "config: Enable conversation pattern as primary for all agents"

### 5.3: Verification

**Tasks**:
- Verify all conversations use new pattern
- Check old pattern is only fallback
- Monitor metrics for 48 hours
- Update progress.json

**Metrics**:
- Messages on new pattern: target > 95%
- Messages on old pattern: target < 5% (fallback only)
- Error rate: target < 0.1%

**Success Criteria**:
- 95%+ messages on new pattern
- Zero downtime
- All agents functioning normally

**Commit Message**: "verify: Primary cutover to conversation pattern complete"

### Sprint 5 Retrospective

**Deliverable**: `retrospectives/sprint_5.md`

---

## Sprint 6: Phase 5 - Cleanup and Documentation

**Duration**: Week 6
**Objective**: Remove old code and finalize migration

### 6.1: Remove Old Subject Methods

**Tasks**:
- Mark `agent_to_agent()` as deprecated
- Remove inbox pattern subscription code
- Remove dual publishing logic
- Keep only conversation pattern

**Files Modified**:
- `src/infrastructure/subject_factory.rs`
- `src/bin/agent-service.rs`

**Commit Message**: "refactor: Remove deprecated inbox pattern code"

### 6.2: Update Documentation

**Tasks**:
- Update README with new subject patterns
- Document conversation-based architecture
- Add examples for developers
- Update CHANGELOG

**Files Modified**:
- `README.md`
- `CHANGELOG.md`
- `doc/architecture/subject-patterns.md` (new)

**Commit Message**: "docs: Update documentation for unified subject architecture"

### 6.3: Final Testing

**Tasks**:
- Run full test suite
- Test conversation flows end-to-end
- Verify all 31 agents working
- Load testing

**Tests**:
- Unit tests: `cargo test --all-features`
- Integration tests: `cargo test --test '*'`
- Conversation demonstration script

**Success Criteria**:
- All tests passing
- Zero old pattern usage
- All documentation updated

**Commit Message**: "test: Final verification of unified subject architecture"

### 6.4: Version Bump and Release

**Tasks**:
- Update Cargo.toml to v1.0.0
- Tag release in git
- Deploy to DGX
- Update progress.json to completed

**Commit Message**: "release: v1.0.0 - Unified subject architecture complete"

### Sprint 6 Retrospective

**Deliverable**: `retrospectives/sprint_6.md`

---

## Final Analysis (Step 10)

After all sprints complete, conduct final analysis:

### Violations Found
- Review any architectural violations discovered
- Document technical debt introduced
- Identify areas for improvement

### Recommendations
- How to fix any violations
- Suggested improvements
- Future considerations

### Metrics Summary
- Total development time
- Lines of code changed
- Test coverage
- Production stability

**Deliverable**: `FINAL_ANALYSIS.md`

---

## Risk Management

### Production Risks
1. **Agent downtime**: Mitigated by dual publishing and rolling deployment
2. **Message loss**: Mitigated by publishing to both patterns
3. **Configuration errors**: Mitigated by staged rollout (one cluster at a time)
4. **Build failures**: Mitigated by native builds on DGX only

### Technical Risks
1. **Breaking changes**: Mitigated by maintaining backward compatibility during transition
2. **Performance degradation**: Mitigated by monitoring and metrics
3. **NATS capacity**: Mitigated by gradual increase in traffic

### Mitigation Strategies
- **Zero downtime**: Dual publishing ensures no message loss
- **Rollback plan**: Old pattern remains available
- **Monitoring**: Continuous metrics during migration
- **Staged rollout**: One cluster at a time

---

## Success Metrics

### Technical Metrics
- All tests passing: ✅
- Zero compilation errors: ✅
- Zero clippy warnings: ✅
- Test coverage > 80%: ✅

### Operational Metrics
- Zero downtime: ✅
- Message delivery: 100%: ✅
- Error rate < 0.1%: ✅
- Response latency < 100ms: ✅

### Completion Metrics
- All 31 agents migrated: ✅
- Old code removed: ✅
- Documentation updated: ✅
- Production stable: ✅

---

## Dependencies

### Internal
- `cim-domain`: Subject algebra (already available)
- `uuid`: v7 support (already available)
- `async-nats`: NATS client (already available)

### External
- NATS server running on DGX
- Agent configurations in `/opt/cim-dgx/configs/`
- Build environment on DGX

---

## Communication Plan

### Stakeholders
- Development team: Sprint updates after each sprint
- Operations team: Deployment notifications
- Users: Migration progress updates

### Status Updates
- Daily: progress.json updates
- Weekly: Sprint retrospectives
- Final: Complete analysis document

---

## Next Actions

1. Initialize progress.json
2. Commit this sprint plan
3. Begin Sprint 1, Step 1.1 (ConversationId)
4. Update progress.json after each step
5. Commit after each step
6. Hold retrospective after each sprint

---

**Document Status**: Ready for Sprint 1
**Created**: 2026-01-22
**Author**: Claude Code (SDLC Sprint Coordinator)
