# Copyright (c) 2025 - Cowboy AI, Inc.

# CIM Agents OCI Container
#
# Single container running CID-addressed agents with ports & adapters architecture.
# Agents are content-addressable and activated via NATS with CID + status.
#
# Architecture (Ports & Adapters):
# - One service process subscribes to agent.commands.deploy
# - Agents defined in cim-domain-agent/agents/*.md (CID-addressed)
# - Port: AI model interaction interface
# - Adapter: genai.rs (one implementation, supports multiple providers)
# - SAGE routes user requests to appropriate agents
# - CID + activation status sent to NATS (no file copying)
#
# Deployment:
# - Auto-starts at boot via systemd
# - Runs as OCI container (podman/docker)
# - Isolated from host OS
# - Connects to NATS cluster at 10.0.20.1:4222
# - Mounts cim-domain-agent/agents/ for CID-based agent specs

{ pkgs ? import <nixpkgs> {} }:

let
  # Agent service binary from cim-domain-agent
  agentService = pkgs.rustPlatform.buildRustPackage {
    pname = "cim-agent-service";
    version = "0.9.2";

    src = /git/thecowboyai/cim-domain-agent;

    cargoLock = {
      lockFile = /git/thecowboyai/cim-domain-agent/Cargo.lock;
    };

    # Build only the agent-service binary
    cargoBuildFlags = [ "--bin" "agent-service" ];

    nativeBuildInputs = with pkgs; [
      pkg-config
    ];

    buildInputs = with pkgs; [
      openssl
    ];
  };

  # Container startup script
  startupScript = pkgs.writeShellScriptBin "start-agents" ''
    #!/usr/bin/env bash
    set -euo pipefail

    echo "Starting CIM Agent Service (CID-based)..."
    echo "Agent specs mounted at: /agents"
    echo "CID computation handled by cim-domain/cim-ipld"
    echo ""
    echo "Connecting to NATS at $NATS_URL"
    echo "Using genai.rs adapter (ports & adapters)"
    echo "Model configuration via NATS ConfigureModel commands"
    echo "Subscribing to: agent.commands.deploy"
    echo "SAGE routing enabled"
    echo ""

    # Start agent service
    # Service uses genai.rs port/adapter for model interaction
    # Agents activated by CID via NATS deploy commands
    exec /bin/agent-service
  '';

in
pkgs.dockerTools.buildLayeredImage {
  name = "cim-agents";
  tag = "latest";

  contents = with pkgs; [
    bashInteractive
    coreutils
    agentService
    startupScript
  ];

  config = {
    Cmd = [ "${startupScript}/bin/start-agents" ];

    Env = [
      "PATH=/bin"
      "NATS_URL=nats://10.0.20.1:4222"
      "STREAM_NAME=AGENT_EVENTS"
      "LOG_LEVEL=info"
      "RUST_LOG=info"
      # Adapter configuration via NATS ConfigureModel commands
      # genai.rs port/adapter handles model provider abstraction
    ];

    # Mount point for agent files
    Volumes = {
      "/agents" = {};
    };

    WorkingDir = "/";

    # Expose no ports (uses NATS for all communication)
    ExposedPorts = {};

    Labels = {
      "org.opencontainers.image.title" = "CIM Agents";
      "org.opencontainers.image.description" = "CID-addressed agents with ports & adapters architecture via NATS with SAGE routing";
      "org.opencontainers.image.version" = "0.9.2";
      "org.opencontainers.image.vendor" = "Cowboy AI, Inc.";
      "cim.agent.addressing" = "cid";
      "cim.agent.architecture" = "ports-adapters";
      "cim.agent.adapter" = "genai.rs";
    };
  };
}
