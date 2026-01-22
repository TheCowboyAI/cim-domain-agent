# CIM Agents OCI Container - CID-Based Deployment

<!-- Copyright (c) 2025 - Cowboy AI, Inc. -->

## Correct Architecture

**Single OCI Container** with **CID-addressed agents** for **local LLMs**:

```
DGX Node (10.0.20.1)
├── NATS Cluster (port 4222)
├── Ollama (port 11434) - Mistral-7b or Llama4-8b
│
└── OCI Container: cim-agents
    ├── Auto-starts at boot (systemd)
    ├── Single agent-service process
    ├── Subscribes to: agent.commands.deploy
    ├── Mounts: /git/thecowboyai/cim-domain-agent/agents/
    ├── Uses genai.rs for local model interaction
    └── Agents activated by CID via NATS:
        ├── description-expert (CID: sha256:abc123...)
        ├── conceptual-spaces-expert (CID: sha256:def456...)
        ├── sage (CID: sha256:789...)
        └── ... (all agents in agents/*.md)
```

## Key Differences from Claude Agents

| Aspect | This (CID-based) | Claude .claude/agents |
|--------|------------------|----------------------|
| **Location** | `cim-domain-agent/agents/` | `.claude/agents/` |
| **Addressing** | CID (SHA-256 hash) | Filename |
| **Models** | Mistral-7b, Llama4-8b (local) | Claude API (cloud) |
| **Activation** | CID + status via NATS | Direct loading |
| **Pre-tuning** | Via genai.rs | N/A |
| **Copying** | ❌ No file copying | ✅ Files copied |

## Quick Start

```bash
cd /git/thecowboyai/cim-domain-agent/nix

# Deploy to DGX
./deploy-agents-container.sh

# Verify
systemctl status cim-agents
journalctl -u cim-agents -f
```

## What the Deployment Does

1. **Computes CIDs** for all agents in `cim-domain-agent/agents/*.md`
2. **Builds container** with agent-service + genai.rs support
3. **Loads to podman** on DGX
4. **Installs systemd** for auto-start at boot
5. **Starts container** connecting to NATS + Ollama
6. **Activates agents** by sending CID + status to NATS

## Agent Activation via NATS

Instead of copying .md files, we send CID-based activation:

```bash
# Example: Activate description-expert
nats req agent.commands.deploy '{
  "agent_id": "uuid-v7",
  "name": "description-expert",
  "cid": "sha256:abc123...",
  "person_id": "00000000-0000-0000-0000-000000000000",
  "description": "CID-based agent",
  "activated": true
}' --server=nats://10.0.20.1:4222
```

Container receives CID, loads `.md` spec from mounted volume, initializes agent.

## Agent Specs Location

**Mounted at**: `/agents` inside container
**Source**: `/git/thecowboyai/cim-domain-agent/agents/` on host

```bash
# Check agents in container
podman exec cim-agents ls -la /agents/

# Compute CID for an agent
podman exec cim-agents compute-cid /agents/description-expert.md
```

## Local LLM Configuration

Container uses **Ollama** with local models:

```bash
# Default configuration
MODEL_PROVIDER=ollama
MODEL_URL=http://host.containers.internal:11434
MODEL_NAME=mistral:7b-instruct

# Or use Llama 4
MODEL_NAME=llama3.2:8b-instruct
```

## Testing Agents

```bash
# Test SAGE (master orchestrator)
nats req agents.sage.request '{"prompt":"What is CIM?"}'

# Test description-expert (Searle cluster theory)
nats req agents.description-expert.request '{
  "prompt":"Explain how co-referring terms work"
}'

# Test conceptual-spaces-expert (Kripke possible worlds)
nats req agents.conceptual-spaces-expert.request '{
  "prompt":"Explain rigid designators in modal logic"
}'
```

## Container Management

```bash
# Service commands
systemctl start cim-agents
systemctl stop cim-agents
systemctl restart cim-agents
systemctl status cim-agents
journalctl -u cim-agents -f

# Container commands
podman ps
podman logs -f cim-agents
podman exec -it cim-agents bash

# Check CIDs
podman exec cim-agents sh -c 'for f in /agents/*.md; do echo "$f: $(compute-cid $f)"; done'
```

## Adding/Updating Agents

### Add New Agent

1. Create `{name}.md` in `cim-domain-agent/agents/`
2. Rebuild and redeploy:
   ```bash
   ./deploy-agents-container.sh
   ```
3. CID computed automatically, agent activated via NATS

### Update Existing Agent

1. Modify `{name}.md` in `cim-domain-agent/agents/`
2. Rebuild and redeploy:
   ```bash
   ./deploy-agents-container.sh
   ```
3. New CID computed, updated agent activated

**Note**: CID changes when content changes - this is content-addressable storage.

## New Agents Deployed

### description-expert (v0.7.0)
- **CID-based**: Content-addressable
- **Foundations**: Frege, Russell, Evans, Searle
- **Key insight**: CLUSTER = CONCEPTUAL SPACE
- **Model**: Mistral-7b or Llama4-8b

### conceptual-spaces-expert (v0.2.0)
- **CID-based**: Content-addressable
- **Foundations**: Gärdenfors + Kripke
- **Key insight**: Possible worlds AS Conceptual Spaces
- **Model**: Mistral-7b or Llama4-8b

## Troubleshooting

### Container won't start
```bash
journalctl -u cim-agents -n 50
podman logs cim-agents
systemctl status ollama  # Check Ollama is running
```

### Agents not activating
```bash
# Check NATS connection
nats stream view AGENT_EVENTS

# Verify agents mounted
podman exec cim-agents ls -la /agents/

# Check CID computation
podman exec cim-agents compute-cid /agents/description-expert.md
```

### Wrong model
```bash
# Check Ollama models
ollama list

# Pull required model
ollama pull mistral:7b-instruct
# or
ollama pull llama3.2:8b-instruct
```

## Files

```
nix/
├── cim-agents-container.nix      # OCI container (CID-based)
├── cim-agents.service             # Systemd service
├── deploy-agents-container.sh     # Deployment + NATS activation
└── README.md                      # This file
```

## Architecture Benefits

✅ **Content-Addressable**: Agents identified by CID (SHA-256)
✅ **No File Copying**: Mount agents/ directory directly
✅ **Local LLMs**: Mistral-7b, Llama4-8b via Ollama
✅ **NATS Activation**: CID + status sent to NATS
✅ **genai.rs**: Pre-configured for local models
✅ **Auto-Start**: Systemd ensures agents run at boot
✅ **SAGE Routing**: Intelligent request routing
