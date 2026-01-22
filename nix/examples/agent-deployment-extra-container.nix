# Copyright (c) 2025 - Cowboy AI, LLC.
#
# Example: Deploy CIM Agents using extra-container
#
# This configuration shows how to deploy CIM agents in declarative containers
# using extra-container: https://github.com/erikarvstedt/extra-container
#
# Usage:
#   1. Import this file in your NixOS configuration
#   2. Set services.cim-agents.agentFiles to your agent .md files
#   3. nixos-rebuild switch
#
# With extra-container CLI:
#   extra-container create --start <container-name>
#   extra-container shell <container-name>
#   extra-container destroy <container-name>

{ config, lib, pkgs, ... }:

let
  # Import cim-domain-agent flake
  cim-domain-agent = builtins.getFlake "github:thecowboyai/cim-domain-agent";

  # Agent package
  agentPackage = cim-domain-agent.packages.${pkgs.system}.default;

  # Define agents to deploy
  agents = {
    nats-expert = {
      agentFile = "${cim-domain-agent}/agents/nats-expert.md";
      memoryMax = "8G";
      cpuQuota = "300%";  # 3 cores
      network = {
        localAddress = "10.100.0.2/24";
        hostAddress = "10.100.0.1";
      };
    };

    ddd-expert = {
      agentFile = "${cim-domain-agent}/agents/ddd-expert.md";
      memoryMax = "8G";
      cpuQuota = "300%";
      network = {
        localAddress = "10.100.0.3/24";
        hostAddress = "10.100.0.1";
      };
    };

    # Add more agents as needed...
  };

  # Create container configuration for each agent
  mkAgentContainer = name: agentCfg: {
    autoStart = true;
    privateNetwork = true;
    hostAddress = agentCfg.network.hostAddress;
    localAddress = agentCfg.network.localAddress;

    # Bind mounts for data persistence
    bindMounts = {
      "/var/lib/cim-agent" = {
        hostPath = "/var/lib/cim-agents/${name}";
        isReadOnly = false;
      };
    };

    config = { config, pkgs, ... }: {
      system.stateVersion = "24.05";

      # Import CIM agent module
      imports = [ cim-domain-agent.nixosModules.agent ];

      # Configure agent
      services.cim-agent = {
        enable = true;
        agentFile = agentCfg.agentFile;
        package = agentPackage;

        # Infrastructure
        natsUrl = "nats://10.0.20.1:4222";
        ollamaUrl = "http://10.0.20.1:11434";

        # Resource limits (from agent config)
        memoryMax = agentCfg.memoryMax;
        cpuQuota = agentCfg.cpuQuota;

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

in
{
  # Create host directories for agent data
  system.activationScripts.cim-agents-dirs = lib.concatStringsSep "\n" (
    lib.mapAttrsToList (name: _: ''
      mkdir -p /var/lib/cim-agents/${name}
      chmod 755 /var/lib/cim-agents/${name}
    '') agents
  );

  # Deploy each agent in its own container
  containers = lib.mapAttrs mkAgentContainer agents;

  # Alternative: Use extra-container for more flexibility
  # See: extra-container-example-advanced.nix
}
