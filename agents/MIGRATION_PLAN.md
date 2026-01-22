# Agent Consolidation Migration Plan

<!-- Copyright (c) 2025 - Cowboy AI, LLC. -->

## Executive Summary

**Goal:** Consolidate split agent architecture (YAML + MD in cim-dgx) into unified single-file format (MD with YAML front-matter in cim-domain-agent).

**Key Changes:**
1. **Single Source of Truth**: One `.md` file per agent with complete configuration
2. **Conceptual Space Focus**: System prompts rewritten to focus on boundary enforcement and quality dimensions
3. **Knowledge Integration**: Knowledge/examples embedded as markdown sections (not separate dirs)
4. **Fine-Tuning Clarity**: Agent file IS the model fine-tuning mechanism

**Benefits:**
- ✅ Easier maintenance (one file to edit)
- ✅ Better conceptual clarity (boundaries and dimensions explicit)
- ✅ Version control friendly (single file per agent)
- ✅ CID-ready (future: load agents from IPLD by CID)

## Current State Analysis

### Current Architecture (cim-dgx)

```
/git/thecowboyai/cim-dgx/deploy/agents/
├── config.template.yaml           # Template for all agents
├── INDEX.md                       # Registry of agents
├── SHARED-AGENT-BASE-PROMPT.md   # Base prompt loaded by ALL agents
├── {agent-name}/
│   ├── config.yaml                # Agent-specific YAML config
│   ├── system-prompt.md          # Agent-specific system prompt
│   ├── knowledge/                # Optional: domain knowledge files
│   └── examples/                 # Optional: example files
```

**Total Agents:** 30 (25 listed in INDEX.md, 5 additional)

**Problems:**
1. **Split Configuration**: YAML and MD files duplicated information
2. **Generic Prompts**: System prompts are "regurgitated Claude instructions"
3. **No Conceptual Mapping**: Missing boundary/dimension specifications
4. **Location Wrong**: Should be in cim-domain-agent, not cim-dgx
5. **Hard to Maintain**: Changes require editing multiple files

### Target Architecture (cim-domain-agent)

```
/git/thecowboyai/cim-domain-agent/agents/
├── TEMPLATE.md                    # Template for new agents
├── AGENT_ONTOLOGY.md             # Boundary/dimension mapping
├── MIGRATION_PLAN.md             # This file
├── {agent-name}.md               # Complete agent (YAML front-matter + MD content)
```

**Example Agent Structure:**
```markdown
---
# YAML Front-matter (all configuration)
agent: {...}
conceptual_space: {...}
model: {...}
nats: {...}
deployment: {...}
dependencies: {...}
testing: {...}
documentation: {...}
---

# Agent Display Name - CIM Agent System Prompt

{Rewritten system prompt focused on:}
- Conceptual boundary enforcement
- Quality dimension measurement
- Pure functional patterns
- CIM-specific concepts (not generic Claude patterns)

# Knowledge Base
{Embedded knowledge sections}

# Examples
{Embedded examples}

# Testing and Validation
{Test scenarios}
```

## Migration Phases

### Phase 1: Foundation (Complete) ✅

**Status:** Complete

**Deliverables:**
- [x] TEMPLATE.md - Template for unified format
- [x] AGENT_ONTOLOGY.md - Boundary/dimension mapping
- [x] nats-expert.md - Complete example in new format
- [x] MIGRATION_PLAN.md - This migration plan

### Phase 2: Code Updates (In Progress)

**Status:** In Progress

**Tasks:**
1. Update `cim-domain-agent` code to load unified `.md` files
2. Implement YAML front-matter parser
3. Extract system prompt from markdown content
4. Load knowledge/examples from embedded sections
5. Validate conceptual_space configuration

**Implementation Details:**

```rust
// src/agent_config.rs

use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentConfig {
    pub agent: AgentIdentity,
    pub conceptual_space: ConceptualSpace,
    pub model: ModelConfig,
    pub nats: NatsConfig,
    pub deployment: DeploymentConfig,
    pub dependencies: DependencyConfig,
    pub testing: TestingConfig,
    pub documentation: DocumentationConfig,

    // Parsed from markdown content
    #[serde(skip)]
    pub system_prompt: String,

    #[serde(skip)]
    pub knowledge_base: Vec<KnowledgeSection>,

    #[serde(skip)]
    pub examples: Vec<ExampleSection>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConceptualSpace {
    pub boundary: String,  // "domain", "quality", "theory", "infrastructure-enabler"
    pub primary_supported_boundaries: Option<Vec<String>>,
    pub quality_dimensions: Vec<QualityDimension>,
    pub topology: TopologyConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityDimension {
    pub dimension: String,
    pub weight: f64,  // 0.0-1.0
    pub description: String,
    pub metrics: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopologyConfig {
    pub centrality: f64,  // 0.0-1.0
    pub connectivity: Vec<String>,  // Connected agent names
    pub distance_metrics: Vec<DistanceMetric>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DistanceMetric {
    pub metric: String,
    pub description: String,
}

impl AgentConfig {
    /// Load agent from unified .md file with YAML front-matter
    pub fn load_from_unified_md<P: AsRef<Path>>(path: P) -> Result<Self, AgentConfigError> {
        let content = std::fs::read_to_string(path)?;

        // Split front-matter and markdown content
        let (front_matter, markdown) = Self::split_front_matter(&content)?;

        // Parse YAML front-matter
        let mut config: AgentConfig = serde_yaml::from_str(&front_matter)?;

        // Parse markdown sections
        let (system_prompt, knowledge, examples) = Self::parse_markdown_sections(&markdown)?;

        config.system_prompt = system_prompt;
        config.knowledge_base = knowledge;
        config.examples = examples;

        // Validate conceptual space configuration
        config.validate_conceptual_space()?;

        Ok(config)
    }

    fn split_front_matter(content: &str) -> Result<(String, String), AgentConfigError> {
        // Look for YAML front-matter delimited by ---
        if !content.starts_with("---") {
            return Err(AgentConfigError::MissingFrontMatter);
        }

        // Find closing ---
        let rest = &content[3..];
        if let Some(end_pos) = rest.find("\n---\n") {
            let front_matter = rest[..end_pos].to_string();
            let markdown = rest[end_pos + 5..].to_string();
            Ok((front_matter, markdown))
        } else {
            Err(AgentConfigError::MalformedFrontMatter)
        }
    }

    fn parse_markdown_sections(markdown: &str) -> Result<(String, Vec<KnowledgeSection>, Vec<ExampleSection>), AgentConfigError> {
        // Parse markdown into sections
        // Extract system prompt (everything up to # Knowledge Base or # Examples)
        // Extract knowledge base sections
        // Extract example sections

        // TODO: Implement markdown parsing
        todo!("Parse markdown sections")
    }

    fn validate_conceptual_space(&self) -> Result<(), AgentConfigError> {
        // Validate boundary is one of: domain, quality, theory, infrastructure-enabler
        let valid_boundaries = ["domain", "quality", "theory", "infrastructure-enabler"];
        if !valid_boundaries.contains(&self.conceptual_space.boundary.as_str()) {
            return Err(AgentConfigError::InvalidBoundary(self.conceptual_space.boundary.clone()));
        }

        // Validate quality dimension weights are in [0.0, 1.0]
        for dim in &self.conceptual_space.quality_dimensions {
            if dim.weight < 0.0 || dim.weight > 1.0 {
                return Err(AgentConfigError::InvalidDimensionWeight(dim.dimension.clone(), dim.weight));
            }
        }

        // Validate topology centrality in [0.0, 1.0]
        if self.conceptual_space.topology.centrality < 0.0 || self.conceptual_space.topology.centrality > 1.0 {
            return Err(AgentConfigError::InvalidCentrality(self.conceptual_space.topology.centrality));
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct KnowledgeSection {
    pub title: String,
    pub content: String,
}

#[derive(Debug)]
pub struct ExampleSection {
    pub title: String,
    pub scenario: String,
    pub content: String,
}

#[derive(Debug, thiserror::Error)]
pub enum AgentConfigError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("YAML parsing error: {0}")]
    Yaml(#[from] serde_yaml::Error),

    #[error("Missing YAML front-matter")]
    MissingFrontMatter,

    #[error("Malformed YAML front-matter")]
    MalformedFrontMatter,

    #[error("Invalid conceptual boundary: {0}")]
    InvalidBoundary(String),

    #[error("Invalid quality dimension weight for {0}: {1} (must be 0.0-1.0)")]
    InvalidDimensionWeight(String, f64),

    #[error("Invalid topology centrality: {0} (must be 0.0-1.0)")]
    InvalidCentrality(f64),
}
```

**Files to Update:**
- `src/agent_config.rs` - Add unified MD loading
- `src/lib.rs` - Export new types
- `Cargo.toml` - Add dependencies (serde_yaml if not present)

### Phase 3: Agent Migration (25 agents)

**Status:** Pending

**Agents to Migrate:**

#### Priority 1: Foundation Layer (Deploy First) - 3 agents
- [ ] **nats-expert** - Already migrated (example)
- [ ] nix-expert
- [ ] network-expert

#### Priority 2: Core Theory Layer - 5 agents
- [ ] cim-expert - Cross-boundary architecture
- [ ] act-expert - Category Theory
- [ ] frp-expert - Functional Reactive Programming
- [ ] fp-expert - Functional Programming
- [ ] graph-expert - Graph Theory

#### Priority 3: Domain Layer - 5 agents
- [ ] ddd-expert - Domain-Driven Design
- [ ] domain-expert - Domain creation
- [ ] event-storming-expert - Collaborative discovery
- [ ] domain-ontologist-researcher - Ontology research
- [ ] cim-domain-expert - CIM domain patterns

#### Priority 4: Quality/Spaces Layer - 2 agents
- [ ] conceptual-spaces-expert - Gärdenfors' theory
- [ ] language-expert - Semantic extraction

#### Priority 5: Development Support - 3 agents
- [ ] tdd-expert - Test-driven development
- [ ] bdd-expert - Behavior-driven development
- [ ] qa-expert - Quality assurance

#### Priority 6: UI/Presentation - 4 agents
- [ ] iced-ui-expert - Iced GUI framework
- [ ] egui-ui-expert - egui immediate mode
- [ ] cim-ui-layer-expert - Display/Communication bridge

#### Priority 7: Domain-Specific - 3 agents
- [ ] people-expert - Person domain
- [ ] org-expert - Organization domain
- [ ] location-expert - Location domain

#### Priority 8: Infrastructure Support - 2 agents
- [ ] subject-expert - Subject algebra
- [ ] git-expert - Version control

#### Priority 9: Orchestration & SDLC - 2 agents
- [ ] sage - Master orchestrator
- [ ] sdlc-distributed-expert - Distributed SDLC

**Migration Process Per Agent:**

1. **Read Existing Files:**
   ```bash
   # From cim-dgx
   cat /git/thecowboyai/cim-dgx/deploy/agents/{agent}/config.yaml
   cat /git/thecowboyai/cim-dgx/deploy/agents/{agent}/system-prompt.md
   cat /git/thecowboyai/cim-dgx/deploy/agents/{agent}/knowledge/*.md
   cat /git/thecowboyai/cim-dgx/deploy/agents/{agent}/examples/*.md
   ```

2. **Identify Conceptual Boundary:**
   - Refer to AGENT_ONTOLOGY.md for boundary mapping
   - Determine primary boundary and supported boundaries
   - Identify quality dimensions (with weights)
   - Map topology (centrality, connectivity)

3. **Rewrite System Prompt:**
   - Focus on boundary enforcement (NOT generic Claude patterns)
   - Explain quality dimensions and how agent measures them
   - Emphasize pure functional patterns (NO OOP)
   - Include CIM-specific concepts (event sourcing, content addressing, etc.)
   - Remove generic "helpful assistant" language
   - Add proactive guidance rules

4. **Embed Knowledge:**
   - Copy knowledge files into markdown sections
   - Use ## headings for organization
   - Convert examples to markdown format

5. **Create Unified File:**
   ```bash
   # To cim-domain-agent
   vim /git/thecowboyai/cim-domain-agent/agents/{agent}.md
   ```

6. **Validate:**
   ```bash
   # Test loading
   cargo test --test agent_config_tests -- {agent}
   ```

### Phase 4: Deployment Updates

**Status:** Pending

**Tasks:**
1. Update `cim-dgx` deployment scripts to load from `cim-domain-agent/agents/`
2. Remove old split files from `cim-dgx/deploy/agents/`
3. Update agent-runtime to use new loading mechanism
4. Test deployed agents with new configuration
5. Update INDEX.md with new file locations

**Deployment Script Updates:**

```bash
# OLD: deploy/scripts/deploy-agent.sh
AGENT_CONFIG="/opt/cim-dgx/configs/agent-runtime-${AGENT_NAME}.env"
AGENT_YAML="/opt/cim-dgx/deploy/agents/${AGENT_NAME}/config.yaml"
SYSTEM_PROMPT="/opt/cim-dgx/deploy/agents/${AGENT_NAME}/system-prompt.md"

# NEW: deploy/scripts/deploy-agent.sh
AGENT_MD="/opt/cim-dgx/cim-domain-agent/agents/${AGENT_NAME}.md"
```

### Phase 5: Testing & Validation

**Status:** Pending

**Test Scenarios:**

1. **Unit Tests:** Load each agent file, validate YAML parsing
2. **Integration Tests:** Deploy agent, send InvokeAgent command, validate response
3. **Boundary Tests:** Validate agents enforce their conceptual boundaries
4. **Dimension Tests:** Validate agents measure quality dimensions correctly
5. **Collaboration Tests:** Multi-agent workflows via sage orchestration

**Test Implementation:**

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_nats_expert() {
        let config = AgentConfig::load_from_unified_md("agents/nats-expert.md")
            .expect("Failed to load nats-expert");

        assert_eq!(config.agent.name, "nats-expert");
        assert_eq!(config.conceptual_space.boundary, "infrastructure-enabler");
        assert!(config.conceptual_space.quality_dimensions.len() >= 3);

        // Validate topology dimension exists
        let topology_dim = config.conceptual_space.quality_dimensions
            .iter()
            .find(|d| d.dimension == "topology")
            .expect("Missing topology dimension");

        assert_eq!(topology_dim.weight, 0.8);
    }

    #[test]
    fn test_validate_conceptual_space() {
        // Valid configuration
        let valid_config = AgentConfig {
            // ... valid config
        };
        assert!(valid_config.validate_conceptual_space().is_ok());

        // Invalid boundary
        let mut invalid_config = valid_config.clone();
        invalid_config.conceptual_space.boundary = "invalid".to_string();
        assert!(invalid_config.validate_conceptual_space().is_err());

        // Invalid dimension weight
        let mut invalid_config = valid_config.clone();
        invalid_config.conceptual_space.quality_dimensions[0].weight = 1.5;
        assert!(invalid_config.validate_conceptual_space().is_err());
    }
}
```

### Phase 6: Documentation & Rollout

**Status:** Pending

**Tasks:**
1. Update cim-domain-agent README.md with new agent format
2. Document agent creation process using TEMPLATE.md
3. Update deployment documentation
4. Train team on new format
5. Archive old cim-dgx agent files

## Migration Timeline

**Estimated Timeline:** 2-3 weeks

**Week 1:**
- [ ] Phase 2: Code updates (2-3 days)
- [ ] Phase 3: Migrate Priority 1-3 agents (10 agents, ~3 days)

**Week 2:**
- [ ] Phase 3: Migrate Priority 4-7 agents (12 agents, ~4 days)
- [ ] Phase 4: Deployment updates (1 day)

**Week 3:**
- [ ] Phase 3: Migrate Priority 8-9 agents (4 agents, ~1 day)
- [ ] Phase 5: Testing & validation (2 days)
- [ ] Phase 6: Documentation & rollout (2 days)

## Success Criteria

### Technical Success
- [ ] All 30 agents migrated to unified format
- [ ] All agents load successfully from `.md` files
- [ ] Conceptual space configurations validated
- [ ] System prompts focus on boundary enforcement
- [ ] Zero generic "Claude assistant" language remaining
- [ ] Knowledge/examples embedded in agent files
- [ ] Deployed agents functional on DGX cluster

### Conceptual Success
- [ ] Every agent explicitly maps to conceptual boundary
- [ ] Every agent defines quality dimensions with weights
- [ ] Every agent explains boundary enforcement rules
- [ ] Agent topology (connectivity) clearly specified
- [ ] Pure functional patterns emphasized (OOP anti-patterns rejected)
- [ ] CIM-specific concepts (event sourcing, CID, FRP) prominent

### Operational Success
- [ ] Agent maintenance easier (single file per agent)
- [ ] Version control cleaner (unified diffs)
- [ ] Deployment simpler (one source of truth)
- [ ] Team understands new format
- [ ] Documentation complete and accessible

## Risks & Mitigation

### Risk 1: Breaking Existing Deployments
**Mitigation:** Keep old files until all agents successfully deployed with new format

### Risk 2: System Prompts Too Long
**Mitigation:** Target 4K-8K tokens per prompt, use concise language

### Risk 3: YAML Parsing Complexity
**Mitigation:** Use well-tested serde_yaml library, comprehensive error handling

### Risk 4: Knowledge Base Too Large
**Mitigation:** Summarize knowledge, link to external docs where appropriate

### Risk 5: Team Resistance to New Format
**Mitigation:** Provide clear documentation, migration examples, training session

## Future Enhancements

### Phase 7: CID-Based Agent Loading (Future)

**Goal:** Load agents from IPLD Object Store by CID

**Benefits:**
- Content-addressed agents (immutable, verifiable)
- Version agents by CID
- Distribute agents across network
- Agent updates via CID change (not file system)

**Implementation:**
```rust
impl AgentConfig {
    pub async fn load_from_cid(cid: &Cid) -> Result<Self, AgentConfigError> {
        // Fetch agent content from IPLD Object Store
        let content = ipld_store.get(cid).await?;

        // Parse as unified MD format
        Self::load_from_unified_md_content(&content)
    }
}
```

**Agent Version Management:**
```yaml
# Agent references other agents by CID
dependencies:
  required:
    - agent: nats-expert
      cid: bafkreigh2akiscaildcqabsyg3dfr6ah3htps...
      version: "0.1.0"
```

## Conclusion

This migration consolidates the split agent architecture into a unified, maintainable format focused on CIM conceptual foundations. Each agent becomes a self-contained fine-tuning mechanism that enforces conceptual boundaries and optimizes quality dimensions.

**Next Steps:**
1. Review and approve this migration plan
2. Begin Phase 2: Code updates
3. Start Priority 1 agent migration (nats-expert already complete)
4. Iterate on format based on migration experience
5. Complete all 30 agents within 3-week timeline

**Questions? Concerns?**
- Review TEMPLATE.md for format questions
- Review AGENT_ONTOLOGY.md for conceptual boundary mapping
- Review nats-expert.md for complete example
- Contact: CIM development team
