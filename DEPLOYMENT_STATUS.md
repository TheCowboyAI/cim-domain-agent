# Agent Deployment Status

<!-- Copyright (c) 2025 - Cowboy AI, Inc. -->

**Last Updated:** 2026-01-21

## Current Deployment Architecture

### Deployment Locations

1. **Primary Agent Location:** `/git/thecowboyai/.claude/agents/`
   - Used by cim-dgx deployment script
   - Used by cim-dialog
   - **Total agents: 29**

2. **Development Location:** `/git/thecowboyai/cim-domain-agent/agents/`
   - Agent specifications and development
   - Source of truth for agent definitions
   - **Total agents: 30+** (includes documentation files)

3. **Deployment System:** cim-dgx
   - Location: `/git/thecowboyai/cim-dgx/agents/deploy-agents.sh`
   - Deploys from: `/git/thecowboyai/.claude/agents/`
   - Target: NATS at `nats://10.0.20.1:4222`
   - Method: NATS `agent.commands.deploy` requests

## Current Agents in Production

Agents currently in `/git/thecowboyai/.claude/agents/`:

1. ✅ act-expert.md
2. ✅ batocera-expert.md
3. ✅ bdd-expert.md
4. ✅ cim-domain-expert.md
5. ✅ cim-expert.md
6. ✅ cim-tea-ecs-expert.md
7. ✅ conceptual-spaces-expert.md (v2.0.0 - **NEEDS UPDATE**)
8. ✅ ddd-expert.md
9. ✅ domain-expert.md
10. ✅ domain-ontologist-researcher.md
11. ✅ egui-ui-expert.md
12. ✅ elm-architecture-expert.md
13. ✅ event-storming-expert.md
14. ✅ frp-expert.md
15. ✅ git-expert.md
16. ✅ graph-expert.md
17. ✅ iced-ui-expert.md
18. ✅ language-expert.md
19. ✅ location-expert.md
20. ✅ nats-expert.md
21. ✅ network-expert.md
22. ✅ nix-expert.md
23. ✅ org-expert.md
24. ✅ people-expert.md
25. ✅ qa-expert.md
26. ✅ sage.md
27. ✅ sdlc-distributed-expert.md
28. ✅ subject-expert.md
29. ✅ tdd-expert.md

## Agents Needing Deployment

### 1. NEW Agent: description-expert

**Status:** ❌ NOT in production (.claude/agents)
**Version:** v0.7.0
**Source:** `/git/thecowboyai/cim-domain-agent/agents/description-expert.md`
**Action:** **COPY to .claude/agents**

**Critical Foundation For:**
- language-expert
- ddd-expert
- domain-expert
- people-expert
- org-expert
- location-expert

**Capabilities:**
- Frege (1892): Sense/Reference distinction
- Russell (1905): Theory of descriptions
- Evans (1973): Causal theory of names
- Searle (1958): Cluster theory - **CLUSTER = CONCEPTUAL SPACE**
- Co-referring terms analysis
- Cross-linguistic similarity
- Entity identification through multiple descriptions

### 2. UPDATED Agent: conceptual-spaces-expert

**Status:** ⚠️  EXISTS but outdated
**Current Version:** v2.0.0 (in .claude/agents)
**New Version:** v0.2.0 (in cim-domain-agent)
**Source:** `/git/thecowboyai/cim-domain-agent/agents/conceptual-spaces-expert.md`
**Action:** **UPDATE in .claude/agents**

**Critical Addition:**
- Kripke (1970/1972): Possible worlds semantics
- **Possible worlds AS Conceptual Spaces**
- Modal logic (□P, ◊P)
- Rigid designators
- Counterfactual reasoning
- A posteriori necessities

**NOTE:** Version mismatch (2.0.0 vs 0.2.0) needs investigation. The cim-domain-agent version includes Kripke integration which is critical.

## Deployment Plan

### Phase 1: Copy New Agent to Production

```bash
# Copy description-expert to production location
cp /git/thecowboyai/cim-domain-agent/agents/description-expert.md \\
   /git/thecowboyai/.claude/agents/description-expert.md
```

### Phase 2: Update Existing Agent

```bash
# Check version mismatch
diff /git/thecowboyai/.claude/agents/conceptual-spaces-expert.md \\
     /git/thecowboyai/cim-domain-agent/agents/conceptual-spaces-expert.md

# Backup current version
cp /git/thecowboyai/.claude/agents/conceptual-spaces-expert.md \\
   /git/thecowboyai/.claude/agents/conceptual-spaces-expert.md.backup

# Update with Kripke integration
cp /git/thecowboyai/cim-domain-agent/agents/conceptual-spaces-expert.md \\
   /git/thecowboyai/.claude/agents/conceptual-spaces-expert.md
```

### Phase 3: Deploy via cim-dgx

```bash
cd /git/thecowboyai/cim-dgx/agents

# Deploy all agents (including new/updated ones)
./deploy-agents.sh
```

This will:
- Parse all 30 agents (29 existing + 1 new)
- Generate UUID v7 for each
- Send DeployAgent command via NATS
- Deploy to NATS cluster at `10.0.20.1:4222`

### Phase 4: Verify Deployment

```bash
# Check NATS agent events
nats stream view AGENT_EVENTS --server=nats://10.0.20.1:4222

# Test description-expert
nats request agents.description-expert.invoke \\
  '{"task":"Explain co-referring terms","context":"test"}' \\
  --server=nats://10.0.20.1:4222

# Test conceptual-spaces-expert
nats request agents.conceptual-spaces-expert.invoke \\
  '{"task":"Explain possible worlds as Conceptual Spaces","context":"test"}' \\
  --server=nats://10.0.20.1:4222
```

## Version Conflict Resolution

### Issue: conceptual-spaces-expert version mismatch

**Current in .claude/agents:** v2.0.0
**New in cim-domain-agent:** v0.2.0 (with Kripke)

**Investigation Needed:**
1. Is v2.0.0 a different agent entirely?
2. Should we merge v2.0.0 features with v0.2.0 (Kripke)?
3. Or replace v2.0.0 with v0.2.0?

**Recommendation:**
- Check content of v2.0.0 in .claude/agents
- If it lacks Kripke integration, replace with v0.2.0
- If it has additional features, merge them

## Quick Deploy Command

```bash
# One-liner to deploy both agents
cd /git/thecowboyai/cim-domain-agent && \\
cp agents/description-expert.md /git/thecowboyai/.claude/agents/ && \\
cp agents/conceptual-spaces-expert.md /git/thecowboyai/.claude/agents/ && \\
cd /git/thecowboyai/cim-dgx/agents && \\
./deploy-agents.sh
```

## Rollback Plan

If deployment fails:

```bash
# Remove description-expert
rm /git/thecowboyai/.claude/agents/description-expert.md

# Restore conceptual-spaces-expert backup
mv /git/thecowboyai/.claude/agents/conceptual-spaces-expert.md.backup \\
   /git/thecowboyai/.claude/agents/conceptual-spaces-expert.md

# Re-deploy original state
cd /git/thecowboyai/cim-dgx/agents && ./deploy-agents.sh
```

## Dependencies

### description-expert depends on:
- None (foundational agent)

### Agents depending on description-expert:
- language-expert (semantic extraction)
- ddd-expert (entity identification)
- domain-expert (domain modeling)
- people-expert (person identity)
- org-expert (organization identity)
- location-expert (location identity)

### conceptual-spaces-expert depends on:
- None (foundational agent)

### Agents depending on conceptual-spaces-expert:
- All domain agents (modal reasoning)
- ddd-expert (domain invariants)
- Event sourcing validation

## Expected Impact

### With description-expert:
- ✅ Better entity identification with partial information
- ✅ Cross-linguistic similarity analysis
- ✅ Flexible validation (no over-constraint)
- ✅ Cluster-based identity (Organizations, People, Locations, Policies)

### With updated conceptual-spaces-expert:
- ✅ Modal reasoning (necessary/possible properties)
- ✅ Counterfactual analysis
- ✅ Rigid designator identification
- ✅ Event ordering invariants
- ✅ A posteriori necessities in domains

## Risk Assessment

**Risk Level:** LOW

- Both agents are additive (add capabilities, don't break existing)
- description-expert is new (no breaking changes)
- conceptual-spaces-expert update is backward compatible
- Can rollback easily if needed
- No production systems depend on these yet

## Success Criteria

1. ✅ description-expert deployed and responding
2. ✅ conceptual-spaces-expert updated and responding
3. ✅ No errors in NATS agent events stream
4. ✅ Test queries return expected results
5. ✅ Dependent agents can query new capabilities

## Next Actions

1. **Investigate version conflict** in conceptual-spaces-expert
2. **Copy description-expert** to .claude/agents
3. **Update conceptual-spaces-expert** in .claude/agents
4. **Run deploy-agents.sh** from cim-dgx
5. **Verify deployment** via NATS
6. **Test integration** with dependent agents
7. **Monitor for 24 hours**

---

**Status:** Ready for deployment pending version conflict resolution
**Priority:** High (foundational semantic capabilities)
**Estimated Time:** 15-30 minutes
