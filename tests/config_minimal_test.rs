// Copyright (c) 2025 - Cowboy AI, Inc.

//! Minimal test to verify config module compiles

use cim_domain_agent::config::parse_agent_file;

const MINIMAL_CONFIG: &str = r#"---
agent:
  id: ""
  name: "test"
  version: "1.0.0"

model:
  provider: "test"
  parameters:
    temperature: 0.7
    max_tokens: 100
---

# Test

Content
"#;

#[test]
fn test_config_module_compiles() {
    let result = parse_agent_file(MINIMAL_CONFIG.to_string());
    assert!(result.is_ok());
}
