# CIM Domain Agent — Implementation Plan

This directory contains the implementation roadmap and planning documents for the CIM Domain Agent, aligned with the upstream CIM project conventions.

## Directory Structure

```
plan/
├── readme.md                    # This file (authoritative local plan)
├── phase-1-foundation.md        # Core infrastructure setup (optional, future)
├── phase-2-domains.md           # Domain module implementation (optional, future)
├── phase-3-intelligence.md      # AI and semantic features (optional, future)
├── phase-4-visualization.md     # 3D graph and UI (optional, future)
├── phase-5-distribution.md      # Edge computing and scaling (optional, future)
├── milestone-tracking.md        # Progress tracking (optional, future)
└── release-strategy.md          # Release and deployment plan (optional, future)
```

If a file listed above does not exist yet, treat it as a placeholder to be created when that scope becomes active.

## Implementation Phases

### Phase 1: Foundation (Weeks 1–2)
- Core event and component scaffolding
- Nix flake configuration and checks
- Domain module structure (DDD + EDA layout)
- Basic AI provider interfaces and mock implementations
- CI-compatible linting and formatting (via Nix)

### Phase 2: Domain Building (Weeks 3–4)
- Agent lifecycle commands, events, and handlers
- Permissions, capabilities, and constraints components
- Query/projection scaffolding for read models
- Cross-module event flow validation

### Phase 3: Intelligence Layer (Weeks 5–6)
- Semantic search abstractions and vector store integration hooks
- Provider manager orchestration
- Conceptual reasoning systems alignment
- Deterministic test fixtures for AI boundaries

### Phase 4: Visualization (Weeks 7–8)
- Bevy-based demos wired to domain events (optional in this repo)
- Real-time event inspection aids for debugging
- Performance measurements for demo paths

### Phase 5: Distribution (Weeks 9–10)
- Nix-based packaging and `nix run` ergonomics
- Optional multi-tenant or edge-node scaffolding
- Release notes and versioning policy

## Success Criteria
- ✅ All lints passing before any build or run
- ✅ `nix flake check` green
- ✅ Documentation updated alongside code changes
- ✅ Tests for critical paths (where present) are reliable and deterministic
- ✅ Minimal, well-structured public APIs with explicit types

## Development Principles

### Test-First and Incremental Delivery
1. Capture user stories in `doc/user-stories.md` (or inline in PRs)
2. Add/extend tests or compile-time checks
3. Implement minimally to satisfy behavior
4. Refactor for clarity, maintainability, and boundaries
5. Update documentation

### Quality Gates
- Lints and format checks must pass prior to build
- `nix flake check` must pass locally and in CI
- No secrets in code or configuration
- Public APIs documented and stable within a phase

## Technology Stack
- Language: Rust (stable)
- Build/Run: NixOS (`nix flake check`, `nix build`, `nix run`)
- Visualization: Bevy (demos optional in this module)
- AI Providers: OpenAI/Anthropic/Ollama abstractions
- Storage/Search: Pluggable vector store abstractions

## Risks and Mitigations
- API drift across modules → enforce explicit types and version notes
- Integration flakiness → deterministic mocks and fixtures
- Performance surprises → measure demo scenarios early

## Metrics
- Lint and check pass rate per change
- Test reliability and runtime (if tests present)
- Demo runtime performance (optional)

## Communication & Docs
- Keep `readme.md` and `doc/` up-to-date with behavior changes
- Summarize notable decisions in PR descriptions

## How To Use This Plan
- Treat this document as the canonical local plan referenced by `.cursor/rules`
- Add phase files when scope demands deeper detail
- Update sections as phases complete and new milestones are defined
