# Copyright (c) 2025 - Cowboy AI, LLC.
#
# Advanced Example: Multi-Host Agent Deployment with extra-container
#
# This shows how to deploy agents across multiple NixOS hosts using
# extra-container's advanced features and flake-based configuration.
#
# Directory structure:
#   hosts/
#     dgx-spark-01/
#       configuration.nix
#       agents.nix  # This file
#     dgx-spark-02/
#       configuration.nix
#       agents.nix
#     dgx-spark-03/
#       configuration.nix
#       agents.nix

{ config, lib, pkgs, ... }:

let
  # Import cim-domain-agent
  cim-domain-agent = builtins.getFlake "github:thecowboyai/cim-domain-agent";

  # Determine which agents to deploy on this host
  # Based on agent ontology and deployment strategy

  # Infrastructure agents -> dgx-spark-01
  infrastructureAgents = lib.optionalAttrs (config.networking.hostName == "dgx-spark-01") {
    nats-expert = {
      agentFile = "${cim-domain-agent}/agents/nats-expert.md";
      priority = "high";
      memoryMax = "8G";
      cpuQuota = "300%";
    };

    nix-expert = {
      agentFile = "${cim-domain-agent}/agents/nix-expert.md";
      priority = "high";
      memoryMax = "8G";
      cpuQuota = "300%";
    };

    network-expert = {
      agentFile = "${cim-domain-agent}/agents/network-expert.md";
      priority = "high";
      memoryMax = "8G";
      cpuQuota = "300%";
    };
  };

  # Domain & Foundation agents -> dgx-spark-02
  domainAgents = lib.optionalAttrs (config.networking.hostName == "dgx-spark-02") {
    cim-expert = {
      agentFile = "${cim-domain-agent}/agents/cim-expert.md";
      priority = "critical";
      memoryMax = "16G";
      cpuQuota = "400%";
    };

    ddd-expert = {
      agentFile = "${cim-domain-agent}/agents/ddd-expert.md";
      priority = "high";
      memoryMax = "8G";
      cpuQuota = "300%";
    };

    domain-expert = {
      agentFile = "${cim-domain-agent}/agents/domain-expert.md";
      priority = "high";
      memoryMax = "8G";
      cpuQuota = "300%";
    };

    event-storming-expert = {
      agentFile = "${cim-domain-agent}/agents/event-storming-expert.md";
      priority = "medium";
      memoryMax = "8G";
      cpuQuota = "300%";
    };
  };

  # Development & Specialized agents -> dgx-spark-03
  developmentAgents = lib.optionalAttrs (config.networking.hostName == "dgx-spark-03") {
    tdd-expert = {
      agentFile = "${cim-domain-agent}/agents/tdd-expert.md";
      priority = "high";
      memoryMax = "12G";
      cpuQuota = "300%";
    };

    bdd-expert = {
      agentFile = "${cim-domain-agent}/agents/bdd-expert.md";
      priority = "medium";
      memoryMax = "12G";
      cpuQuota = "300%";
    };

    frp-expert = {
      agentFile = "${cim-domain-agent}/agents/frp-expert.md";
      priority = "high";
      memoryMax = "8G";
      cpuQuota = "300%";
    };
  };

  # Orchestrator on all hosts
  orchestratorAgents = {
    sage = {
      agentFile = "${cim-domain-agent}/agents/sage.md";
      priority = "critical";
      memoryMax = "16G";
      cpuQuota = "400%";
    };
  };

  # Merge all agents for this host
  allAgents = infrastructureAgents // domainAgents // developmentAgents // orchestratorAgents;

  # Network configuration for containers
  containerSubnet = "10.100.${toString config.cim.nodeId}.0/24";
  containerGateway = "10.100.${toString config.cim.nodeId}.1";

  # Create agent container with extra-container features
  mkAgentContainer = name: agentCfg: let
    containerIp = config.cim.agentIps.${name} or "10.100.${toString config.cim.nodeId}.${toString (lib.trivial.hashString name)}";
  in {
    autoStart = true;

    # Network isolation
    privateNetwork = true;
    hostAddress = containerGateway;
    localAddress = "${containerIp}/24";

    # Bind mounts
    bindMounts = {
      "/var/lib/cim-agent" = {
        hostPath = "/var/lib/cim-agents/${name}";
        isReadOnly = false;
      };

      # Shared NATS credentials (read-only)
      "/run/secrets/nats-creds" = {
        hostPath = "/run/secrets/nats-creds";
        isReadOnly = true;
      };
    };

    # Additional host devices (for GPU access if needed)
    allowedDevices = lib.optionals (agentCfg.priority == "critical") [
      { node = "/dev/nvidia0"; modifier = "rw"; }
      { node = "/dev/nvidiactl"; modifier = "rw"; }
      { node = "/dev/nvidia-uvm"; modifier = "rw"; }
    ];

    config = { config, pkgs, lib, ... }: {
      system.stateVersion = "24.05";

      imports = [ cim-domain-agent.nixosModules.agent ];

      services.cim-agent = {
        enable = true;
        agentFile = agentCfg.agentFile;
        package = cim-domain-agent.packages.${pkgs.system}.default;

        # Infrastructure endpoints
        natsUrl = "nats://10.0.20.1:4222,nats://10.0.20.2:4222,nats://10.0.20.3:4222";
        ollamaUrl = "http://10.0.20.${toString config.cim.nodeId}:11434";

        # Resource limits from agent config
        inherit (agentCfg) memoryMax cpuQuota;

        # Priority-based restart policy
        restartPolicy = if agentCfg.priority == "critical" then "always" else "on-failure";
        restartSec = if agentCfg.priority == "critical" then 5 else 10;

        # Logging
        logLevel = if agentCfg.priority == "critical" then "debug" else "info";
        logFormat = "json";

        # Extra environment
        extraEnv = {
          RUST_BACKTRACE = "full";
          NATS_CREDS_FILE = "/run/secrets/nats-creds/agent.creds";
        };
      };

      # Networking
      networking = {
        useHostResolvConf = lib.mkForce false;
        firewall.enable = false;
        nameservers = [ "10.0.0.1" "8.8.8.8" ];
      };

      services.resolved.enable = true;

      # GPU support for critical agents
      hardware.opengl.enable = agentCfg.priority == "critical";
    };

    # Container-specific systemd overrides
    extraFlags = [
      "--bind=/dev/nvidia0"
      "--bind=/dev/nvidiactl"
      "--bind=/dev/nvidia-uvm"
    ] ++ lib.optionals (agentCfg.priority == "critical") [
      "--property=DeviceAllow=/dev/nvidia0 rw"
      "--property=DeviceAllow=/dev/nvidiactl rw"
      "--property=DeviceAllow=/dev/nvidia-uvm rw"
    ];
  };

in
{
  # CIM-specific options for this host
  options.cim = with lib; {
    nodeId = mkOption {
      type = types.int;
      description = "Node ID (1-3 for dgx-spark-01/02/03)";
    };

    agentIps = mkOption {
      type = types.attrsOf types.str;
      default = {};
      description = "Static IP assignments for agents";
    };
  };

  config = {
    # Assign node ID based on hostname
    cim.nodeId =
      if config.networking.hostName == "dgx-spark-01" then 1
      else if config.networking.hostName == "dgx-spark-02" then 2
      else if config.networking.hostName == "dgx-spark-03" then 3
      else 0;

    # Static IP assignments for agents
    cim.agentIps = {
      sage = "10.100.${toString config.cim.nodeId}.10";
      nats-expert = "10.100.1.20";
      nix-expert = "10.100.1.21";
      network-expert = "10.100.1.22";
      cim-expert = "10.100.2.20";
      ddd-expert = "10.100.2.21";
      domain-expert = "10.100.2.22";
      event-storming-expert = "10.100.2.23";
      tdd-expert = "10.100.3.20";
      bdd-expert = "10.100.3.21";
      frp-expert = "10.100.3.22";
    };

    # Create host directories
    system.activationScripts.cim-agents-dirs = lib.concatStringsSep "\n" (
      lib.mapAttrsToList (name: _: ''
        mkdir -p /var/lib/cim-agents/${name}
        chmod 755 /var/lib/cim-agents/${name}
      '') allAgents
    );

    # Deploy containers for agents on this host
    containers = lib.mapAttrs mkAgentContainer allAgents;

    # Firewall: allow agent traffic
    networking.firewall = {
      interfaces."ve-+".allowedTCPPorts = [ 4222 11434 ]; # NATS, Ollama
      trustedInterfaces = [ "ve-+" ];
    };

    # Monitoring: Prometheus metrics for agents
    services.prometheus.exporters.node = {
      enable = true;
      enabledCollectors = [ "systemd" ];
    };
  };
}
