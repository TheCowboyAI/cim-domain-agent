// Copyright (c) 2025 - Cowboy AI, Inc.

//! Pure functional configuration parser demonstration
//!
//! This example demonstrates:
//! 1. Pure function composition
//! 2. Iterator-based transformations
//! 3. Type-safe error handling with Result
//! 4. Ownership-aware APIs

use cim_domain_agent::config::{
    parse_agent_file, extract_sections, validate_config, ValidatedConfig,
};

const EXAMPLE_CONFIG: &str = r#"---
agent:
  id: "550e8400-e29b-41d4-a716-446655440000"
  name: "sage"
  display_name: "SAGE - Master CIM Orchestrator"
  version: "0.1.0"

conceptual_space:
  boundary: "meta-orchestrator"
  quality_dimensions:
    - dimension: "topology"
      weight: 1.0
      description: "Agent network topology and workflow orchestration"
    - dimension: "context"
      weight: 1.0
      description: "Cross-boundary context propagation"
  topology:
    centrality: 1.0
    connectivity: ["ALL"]
    distance_metrics:
      - metric: "workflow_complexity"
        description: "Number of agents and steps in workflow"

model:
  provider: "ollama"
  ollama:
    url: "http://localhost:11434"
    model: "llama3.1:70b"
  parameters:
    temperature: 0.7
    max_tokens: 8192
    top_p: 0.9
  rationale: |
    Orchestration requires understanding all conceptual boundaries,
    agent capabilities, and workflow composition. 70B+ recommended.

nats:
  subjects:
    base: "agent.sage"
    commands: "agent.sage.commands.>"
    events: "agent.sage.events.>"
    queries: "agent.sage.queries.>"
  streams:
    - "AGENT_LIFECYCLE"
    - "AGENT_WORKFLOW"
---

# SAGE - Master CIM Orchestrator

You are SAGE, the Strategic Agent for Guiding Execution.

## Role

Master orchestrator for CIM development workflows.

## Capabilities

- Multi-agent coordination
- Workflow composition
- Context propagation across boundaries
- Dynamic agent selection based on task requirements

## Knowledge Base

### Agent Network

You coordinate all specialized agents in the CIM ecosystem:
- cim-expert: Architecture and foundations
- ddd-expert: Domain modeling
- nats-expert: Event infrastructure
- nix-expert: System configuration
"#;

fn main() {
    println!("Pure Functional Configuration Parser Demo\n");
    println!("==========================================\n");

    // STEP 1: Parse the configuration file
    // Pure function: String → Result<AgentConfig, ParseError>
    println!("1. Parsing configuration file...");
    let config_result = parse_agent_file(EXAMPLE_CONFIG.to_string());

    match &config_result {
        Ok(config) => {
            println!("   ✓ Successfully parsed configuration");
            println!("   - Agent: {} ({})", config.agent.name, config.agent.version);
            println!("   - Model: {} via {}",
                config.model.ollama.as_ref().map(|o| o.model.as_str()).unwrap_or("unknown"),
                config.model.provider
            );
            println!();
        }
        Err(e) => {
            eprintln!("   ✗ Parse error: {}", e);
            return;
        }
    }

    // STEP 2: Extract markdown sections
    // Pure function: &str → Result<MarkdownSections, ParseError>
    println!("2. Extracting markdown sections...");
    let config = config_result.as_ref().unwrap();
    let sections_result = extract_sections(&config.system_prompt);

    match &sections_result {
        Ok(sections) => {
            println!("   ✓ Found {} sections:", sections.sections.len());
            for title in sections.sections.keys() {
                println!("     - {}", title);
            }
            println!();
        }
        Err(e) => {
            eprintln!("   ✗ Section extraction error: {}", e);
        }
    }

    // STEP 3: Validate configuration
    // Pure function: AgentConfig → Result<ValidatedConfig, ParseError>
    println!("3. Validating configuration...");
    let validated_result = validate_config(config_result.unwrap());

    match validated_result {
        Ok(validated) => {
            println!("   ✓ Configuration is valid");
            print_validated_config(&validated);
        }
        Err(e) => {
            eprintln!("   ✗ Validation error: {}", e);
        }
    }

    // STEP 4: Demonstrate composition
    println!("\n4. Demonstrating function composition...");
    demonstrate_composition();
}

/// Print details of validated configuration
fn print_validated_config(validated: &ValidatedConfig) {
    let config = validated.config();

    println!("\n   Agent Metadata:");
    println!("   ┌─────────────────────────────────────");
    println!("   │ ID:      {}", config.agent.id);
    println!("   │ Name:    {}", config.agent.name);
    if let Some(ref display_name) = config.agent.display_name {
        println!("   │ Display: {}", display_name);
    }
    println!("   │ Version: {}", config.agent.version);
    println!("   └─────────────────────────────────────");

    println!("\n   Model Configuration:");
    println!("   ┌─────────────────────────────────────");
    println!("   │ Provider:    {}", config.model.provider);
    println!("   │ Temperature: {}", config.model.parameters.temperature);
    println!("   │ Max Tokens:  {}", config.model.parameters.max_tokens);
    if let Some(ref ollama) = config.model.ollama {
        println!("   │ Ollama URL:  {}", ollama.url);
        println!("   │ Model:       {}", ollama.model);
    }
    println!("   └─────────────────────────────────────");

    if let Some(ref space) = config.conceptual_space {
        println!("\n   Conceptual Space:");
        println!("   ┌─────────────────────────────────────");
        println!("   │ Boundary: {}", space.boundary);
        println!("   │ Dimensions: {}", space.quality_dimensions.len());
        for dim in &space.quality_dimensions {
            println!("   │   - {} (weight: {})", dim.dimension, dim.weight);
        }
        println!("   └─────────────────────────────────────");
    }
}

/// Demonstrate pure function composition
fn demonstrate_composition() {
    use cim_domain_agent::config::validator::parse_and_validate;

    println!("   Composing parse_agent_file ∘ validate_config:");
    println!("   parse_and_validate(content) = validate_config(parse_agent_file(content))");
    println!();

    let result = parse_and_validate(EXAMPLE_CONFIG.to_string());

    match result {
        Ok(validated) => {
            println!("   ✓ Composed function succeeded");
            println!("   → Agent: {}", validated.config().agent.name);
        }
        Err(e) => {
            eprintln!("   ✗ Composed function failed: {}", e);
        }
    }

    // Demonstrate referential transparency
    println!("\n   Verifying referential transparency:");
    let result1 = parse_and_validate(EXAMPLE_CONFIG.to_string());
    let result2 = parse_and_validate(EXAMPLE_CONFIG.to_string());

    match (result1, result2) {
        (Ok(v1), Ok(v2)) => {
            if v1.config() == v2.config() {
                println!("   ✓ Multiple calls produce identical results");
                println!("   → parse_and_validate is referentially transparent");
            }
        }
        _ => println!("   ✗ Results differ (unexpected)"),
    }
}
