# Technical Debt Tracking

**Last Updated**: 2026-01-22

## Active Migration: Unified Subject Architecture (v0.10.0-alpha → v1.0.0)

### Sprint 3 Complete ✅
- Dual subscription implementation
- Feature flag support
- Metrics and logging
- Deployment documentation

### Remaining Work

#### Sprint 4-7: Subject Architecture Migration
- [ ] Sprint 4: DGX testing and deployment
- [ ] Sprint 5: Primary cutover to new patterns
- [ ] Sprint 6: Full cutover and monitoring
- [ ] Sprint 7: Cleanup legacy code

#### Future Sprint 8+: Model Configuration Refactoring

**Issue**: `ModelConfiguredEvent` is deprecated in favor of `ModelConfigurationAssigned`

**Impact**:
- Warning level only (not blocking)
- Affects `handle_configure_model()` in agent-service.rs
- Part of separating ModelConfiguration into its own aggregate

**Root Cause**:
- Old pattern: Agent aggregate stores full `ModelConfig` object
- New pattern: ModelConfiguration is separate aggregate, Agent stores only `ModelConfigurationId`

**Required Changes**:
1. Create ModelConfiguration aggregate and repository
2. Update ConfigureModel command flow:
   - First create/lookup ModelConfiguration
   - Get ModelConfigurationId
   - Create ModelConfigurationAssignedEvent with ID
   - Agent stores only the ID reference
3. Update handle_configure_model() to use new pattern
4. Update Agent.model_config() → Agent.model_configuration_id()
5. Add ModelConfiguration lookup service

**Files Affected**:
- `src/bin/agent-service.rs` - Command handler
- `src/aggregate/mod.rs` - Agent.model_config() method
- `src/commands/mod.rs` - ConfigureModel command (may need refactoring)
- `src/services/mod.rs` - May need ModelConfigurationService

**Estimated Effort**: 1-2 weeks
**Priority**: Medium (functional but not optimal)
**Risk**: Low (additive changes, doesn't affect subject architecture)

## Known False Positives

### rust-analyzer Warnings in tokio::select!
- **Warning**: "unused variable: agent_ref_subscriber"
- **Status**: False positive
- **Reason**: Variable IS used in select! block but macro expansion confuses analyzer
- **Action**: Ignore (no code change needed)

## Test Status

- ✅ 301 tests passing (--lib --all-features)
- ✅ Zero test failures
- ⚠️ Example `config_parser_demo.rs` has compilation errors (not critical)

## Build Status

- ✅ Library builds successfully with warnings
- ✅ All tests pass
- ⚠️ OpenSSL issues outside nix shell (expected on NixOS)
- ✅ Builds correctly in `nix develop` shell

---

**Migration Philosophy**: We accept deprecation warnings during migration phases as they document the transition path. Warnings are resolved systematically as sprints progress, not ad-hoc during unrelated work.
