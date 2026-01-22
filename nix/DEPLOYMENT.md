# CIM Agent Deployment Guide

<!-- Copyright (c) 2025 - Cowboy AI, LLC. -->

## Overview

This guide explains how to deploy CIM agents across NixOS systems using declarative containers and `extra-container`.

**Key Features:**
- ✅ **Declarative**: Agents defined in configuration.nix
- ✅ **Deterministic**: Nix ensures reproducible builds
- ✅ **Isolated**: Each agent runs in its own container
- ✅ **Resource-controlled**: Memory, CPU, and task limits enforced
- ✅ **Multi-host**: Deploy agents across multiple machines
- ✅ **CID-ready**: Future support for loading agents from IPLD

## Architecture

```
┌─────────────────────────────────────────────────────────┐
│              NixOS Host (dgx-spark-01/02/03)            │
├─────────────────────────────────────────────────────────┤
│                                                         │
│  ┌────────────────┐  ┌────────────────┐  ┌───────────┐ │
│  │  Container 1   │  │  Container 2   │  │Container N│ │
│  │                │  │                │  │           │ │
│  │ cim-agent-     │  │ cim-agent-     │  │cim-agent- │ │
│  │ nats-expert    │  │ ddd-expert     │  │sage       │ │
│  │                │  │                │  │           │ │
│  │ ┌────────────┐ │  │ ┌────────────┐ │  │┌─────────┐│ │
│  │ │agent-runtime│ │  │ │agent-runtime│ │  ││agent-  ││ │
│  │ │+ agent.md  │ │  │ │+ agent.md  │ │  ││runtime  ││ │
│  │ └────────────┘ │  │ └────────────┘ │  │└─────────┘│ │
│  │                │  │                │  │           │ │
│  │ /var/lib/      │  │ /var/lib/      │  │/var/lib/  │ │
│  │ cim-agent      │  │ cim-agent      │  │cim-agent  │ │
│  └────────────────┘  └────────────────┘  └───────────┘ │
│          │                    │                  │      │
└──────────┼────────────────────┼──────────────────┼──────┘
           │                    │                  │
           └────────────────────┴──────────────────┘
                             │
                    ┌────────▼──────────┐
                    │  NATS Cluster     │
                    │  10.0.20.1:4222   │
                    │  10.0.20.2:4222   │
                    │  10.0.20.3:4222   │
                    └───────────────────┘
```

## Quick Start

### 1. Import cim-domain-agent flake

In your NixOS `flake.nix`:

```nix
{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

    cim-domain-agent.url = "github:thecowboyai/cim-domain-agent";
    # Or: path:/path/to/local/cim-domain-agent

    extra-container.url = "github:erikarvstedt/extra-container";
  };

  outputs = { self, nixpkgs, cim-domain-agent, extra-container, ... }: {
    nixosConfigurations.my-host = nixpkgs.lib.nixosSystem {
      system = "x86_64-linux";
      modules = [
        ./configuration.nix
        cim-domain-agent.nixosModules.agent  # Import agent module
      ];
    };
  };
}
```

### 2. Deploy a single agent

In your `configuration.nix`:

```nix
{ config, pkgs, lib, ... }:

{
  # Deploy nats-expert agent in a container
  containers.cim-agent-nats-expert = {
    autoStart = true;

    privateNetwork = true;
    hostAddress = "10.100.0.1";
    localAddress = "10.100.0.2/24";

    bindMounts."/var/lib/cim-agent" = {
      hostPath = "/var/lib/cim-agents/nats-expert";
      isReadOnly = false;
    };

    config = { config, pkgs, ... }: {
      system.stateVersion = "24.05";

      # Import CIM agent module
      imports = [ <cim-domain-agent/nix/modules/agent.nix> ];

      services.cim-agent = {
        enable = true;

        # Path to agent .md file
        agentFile = "${inputs.cim-domain-agent}/agents/nats-expert.md";

        # Package
        package = inputs.cim-domain-agent.packages.${pkgs.system}.default;

        # Infrastructure
        natsUrl = "nats://10.0.20.1:4222";
        ollamaUrl = "http://10.0.20.1:11434";

        # Resource limits (from agent ontology)
        memoryMax = "8G";
        cpuQuota = "300%";  # 3 cores

        # Logging
        logLevel = "info";
        logFormat = "json";
      };

      # Networking
      networking = {
        useHostResolvConf = lib.mkForce false;
        firewall.enable = false;
      };

      services.resolved.enable = true;
    };
  };

  # Create host data directory
  system.activationScripts.cim-agents-dirs = ''
    mkdir -p /var/lib/cim-agents/nats-expert
    chmod 755 /var/lib/cim-agents/nats-expert
  '';
}
```

### 3. Deploy

```bash
# Build and switch
nixos-rebuild switch

# Check status
systemctl status container@cim-agent-nats-expert

# View logs
journalctl -u container@cim-agent-nats-expert -f

# Shell into container
nixos-container root-login cim-agent-nats-expert
```

## Multi-Agent Deployment

See [`nix/examples/agent-deployment-extra-container.nix`](./examples/agent-deployment-extra-container.nix) for deploying multiple agents.

### Example: Deploy 3 agents

```nix
{ config, pkgs, lib, ... }:

let
  cim-domain-agent = inputs.cim-domain-agent;
  agentPackage = cim-domain-agent.packages.${pkgs.system}.default;

  # Define agents
  agents = {
    nats-expert = {
      agentFile = "${cim-domain-agent}/agents/nats-expert.md";
      memoryMax = "8G";
      cpuQuota = "300%";
      ip = "10.100.0.2";
    };

    ddd-expert = {
      agentFile = "${cim-domain-agent}/agents/ddd-expert.md";
      memoryMax = "8G";
      cpuQuota = "300%";
      ip = "10.100.0.3";
    };

    sage = {
      agentFile = "${cim-domain-agent}/agents/sage.md";
      memoryMax = "16G";
      cpuQuota = "400%";
      ip = "10.100.0.10";
    };
  };

  mkAgentContainer = name: agentCfg: {
    autoStart = true;
    privateNetwork = true;
    hostAddress = "10.100.0.1";
    localAddress = "${agentCfg.ip}/24";

    bindMounts."/var/lib/cim-agent" = {
      hostPath = "/var/lib/cim-agents/${name}";
      isReadOnly = false;
    };

    config = { config, pkgs, ... }: {
      system.stateVersion = "24.05";
      imports = [ cim-domain-agent.nixosModules.agent ];

      services.cim-agent = {
        enable = true;
        agentFile = agentCfg.agentFile;
        package = agentPackage;
        natsUrl = "nats://10.0.20.1:4222";
        ollamaUrl = "http://10.0.20.1:11434";
        inherit (agentCfg) memoryMax cpuQuota;
        logLevel = "info";
        logFormat = "json";
      };

      networking.useHostResolvConf = lib.mkForce false;
      networking.firewall.enable = false;
      services.resolved.enable = true;
    };
  };
in
{
  # Create directories
  system.activationScripts.cim-agents-dirs = lib.concatStringsSep "\n" (
    lib.mapAttrsToList (name: _: ''
      mkdir -p /var/lib/cim-agents/${name}
      chmod 755 /var/lib/cim-agents/${name}
    '') agents
  );

  # Deploy containers
  containers = lib.mapAttrs (name: cfg: mkAgentContainer name cfg) agents;
}
```

## Multi-Host Deployment

See [`nix/examples/extra-container-advanced.nix`](./examples/extra-container-advanced.nix) for deploying agents across multiple NixOS hosts.

### Strategy

Based on agent ontology and resource requirements:

**dgx-spark-01 (Infrastructure Layer):**
- nats-expert (8G, 3 cores)
- nix-expert (8G, 3 cores)
- network-expert (8G, 3 cores)
- sage (16G, 4 cores)

**dgx-spark-02 (Domain & Foundation Layer):**
- cim-expert (16G, 4 cores)
- ddd-expert (8G, 3 cores)
- domain-expert (8G, 3 cores)
- event-storming-expert (8G, 3 cores)
- sage (16G, 4 cores)

**dgx-spark-03 (Development & Specialized Layer):**
- tdd-expert (12G, 3 cores)
- bdd-expert (12G, 3 cores)
- frp-expert (8G, 3 cores)
- iced-ui-expert (12G, 3 cores)
- sage (16G, 4 cores)

### Configuration per host

Each host imports the same base configuration but deploys different agents:

```nix
# hosts/dgx-spark-01/agents.nix
{ config, pkgs, lib, ... }:

let
  cim-domain-agent = inputs.cim-domain-agent;

  # Infrastructure agents for this host
  infrastructureAgents = {
    nats-expert = { ... };
    nix-expert = { ... };
    network-expert = { ... };
    sage = { ... };
  };

in
{
  containers = lib.mapAttrs mkAgentContainer infrastructureAgents;
}
```

## Using extra-container CLI

For development and testing, use `extra-container` CLI:

### Create and start a container

```bash
# Create nats-expert container
extra-container create --attr containers.cim-agent-nats-expert

# Start container
extra-container start cim-agent-nats-expert
```

### Manage containers

```bash
# List containers
extra-container list

# Shell into container
extra-container shell cim-agent-nats-expert

# Stop container
extra-container stop cim-agent-nats-expert

# Destroy container
extra-container destroy cim-agent-nats-expert
```

### Update agent configuration

```bash
# Edit agent .md file
vim agents/nats-expert.md

# Rebuild container with new config
extra-container update cim-agent-nats-expert
```

## Agent Configuration

Agents are configured via `.md` files with YAML front-matter:

```markdown
---
agent:
  id: "c3ea221c-0562-4ea0-92b3-6d24224dcd61"
  name: "nats-expert"
  display_name: "NATS Infrastructure Expert"
  version: "0.1.0"

conceptual_space:
  boundary: "infrastructure-enabler"
  quality_dimensions:
    - dimension: "topology"
      weight: 0.8
      description: "NATS cluster topology..."

model:
  provider: "ollama"
  ollama:
    url: "http://localhost:11434"
    model: "llama3.1:70b"

nats:
  url: "nats://10.0.20.1:4222"
  subjects: { ... }

deployment:
  target_node: "dgx-spark-01"
  resources:
    memory_max: "8G"
    cpu_quota: "300%"
    tasks_max: 512

dependencies: { ... }
---

# NATS Infrastructure Expert - System Prompt

{agent system prompt...}

# Knowledge Base

{embedded knowledge...}

# Examples

{examples...}
```

### Override agent config in NixOS

```nix
services.cim-agent = {
  enable = true;
  agentFile = ./agents/nats-expert.md;

  # Override specific values from agent file
  natsUrl = "nats://custom-url:4222";
  ollamaUrl = "http://custom-ollama:11434";

  # Override resource limits
  memoryMax = "16G";  # Increase from 8G in agent file
  cpuQuota = "400%";   # Increase from 300%

  # Override logging
  logLevel = "debug";  # More verbose than agent file
};
```

## Resource Management

### Memory Limits

Based on agent ontology:
- Infrastructure agents: 8G (llama3.1:70b)
- Domain agents: 8G (llama3.1:70b)
- Critical agents: 16G (larger models)
- Development agents: 12G (code-focused models)

### CPU Quotas

- 200% = 2 cores
- 300% = 3 cores
- 400% = 4 cores

### Task Limits

Default: 512 tasks per agent (prevents fork bombs)

### Monitoring

```nix
# Enable Prometheus monitoring
services.prometheus.exporters.node = {
  enable = true;
  enabledCollectors = [ "systemd" ];
};

# Monitor container metrics
systemd-cgtop
```

## Networking

### Container Network

Agents communicate via private container network:
- Bridge: br-cim (10.100.0.1)
- Subnet: 10.100.X.0/24 (X = node ID)
- Agent IPs: 10.100.X.Y (Y = agent-specific)

### NATS Cluster

Agents connect to NATS cluster:
- nats://10.0.20.1:4222
- nats://10.0.20.2:4222
- nats://10.0.20.3:4222

### Firewall

```nix
networking.firewall = {
  interfaces."ve-+".allowedTCPPorts = [ 4222 11434 ];
  trustedInterfaces = [ "ve-+" ];
};
```

## Troubleshooting

### Container won't start

```bash
# Check systemd logs
journalctl -u container@cim-agent-NAME -xe

# Check container config
nixos-container show-ip cim-agent-NAME

# Test manually
nixos-container start cim-agent-NAME
nixos-container root-login cim-agent-NAME
```

### Agent runtime errors

```bash
# View agent logs
journalctl -u cim-agent -f

# Inside container
nixos-container root-login cim-agent-NAME
systemctl status cim-agent
```

### Resource exhaustion

```bash
# Check memory usage
systemd-cgtop

# Check CPU usage
htop

# Adjust limits in configuration.nix
services.cim-agent.memoryMax = "16G";
```

### NATS connection issues

```bash
# Test NATS connectivity from container
nixos-container root-login cim-agent-NAME
ping 10.0.20.1
nats account info
```

## Best Practices

1. **Declarative First**: Define agents in configuration.nix, not manual scripts
2. **Resource Planning**: Use agent ontology to determine resource allocation
3. **Network Isolation**: Use private networks for agent containers
4. **Data Persistence**: Bind mount /var/lib/cim-agent for agent state
5. **Monitoring**: Enable systemd metrics and Prometheus exporters
6. **Testing**: Use extra-container CLI for development before deploying
7. **Version Control**: Track agent .md files and NixOS configs in git
8. **Rollback**: NixOS generations allow easy rollback on failure

## Security

### Container Isolation

- Private network per container
- No new privileges (NoNewPrivileges=true)
- Protected system (ProtectSystem=strict)
- Protected home (ProtectHome=true)
- Temporary /tmp (PrivateTmp=true)

### NATS Security

- JWT-based authentication via NSC
- Credentials mounted read-only: /run/secrets/nats-creds
- Per-agent credentials with minimal permissions

### Resource Limits

- MemoryMax prevents OOM
- CPUQuota prevents CPU monopolization
- TasksMax prevents fork bombs

## Migration from Shell Script Deployment

Old approach (cim-dgx/deploy/scripts/deploy-agent.sh):
- Manual SSH deployment
- Imperative bash scripts
- No rollback
- Configuration drift

New approach (Nix + extra-container):
- Declarative NixOS configuration
- Atomic deployments
- Easy rollback via generations
- No configuration drift

### Migration Steps

1. Convert .env files to agent .md files (see agents/MIGRATION_PLAN.md)
2. Create NixOS configuration with containers
3. Deploy with nixos-rebuild
4. Verify agents running
5. Remove old deployment scripts

## Future Enhancements

### CID-Based Agent Loading

Load agents from IPLD Object Store by CID:

```nix
services.cim-agent = {
  enable = true;
  agentCid = "bafkreigh2akiscaildcqabsyg3dfr6ah3htps...";
  package = cim-domain-agent;
};
```

Benefits:
- Content-addressed agents (immutable, verifiable)
- Version agents by CID
- Distribute agents across network
- Cryptographic integrity

### Dynamic Agent Scaling

Scale agents based on load:

```nix
services.cim-agent-pool = {
  enable = true;
  agentFile = ./agents/ddd-expert.md;
  minInstances = 1;
  maxInstances = 5;
  scaleMetric = "queue-depth";
};
```

## References

- [Agent Ontology](../agents/AGENT_ONTOLOGY.md)
- [Agent Migration Plan](../agents/MIGRATION_PLAN.md)
- [extra-container](https://github.com/erikarvstedt/extra-container)
- [NixOS Containers](https://nixos.org/manual/nixos/stable/index.html#ch-containers)

## Support

For questions or issues:
- Review examples: `nix/examples/`
- Check agent definitions: `agents/`
- Consult ontology: `agents/AGENT_ONTOLOGY.md`
- Contact: CIM development team
