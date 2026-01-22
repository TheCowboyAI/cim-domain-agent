# Copyright (c) 2025 - Cowboy AI, LLC.
#
# CIM Agent Deployment Module
# Declarative agent deployment using NixOS containers and extra-container

{ config, lib, pkgs, ... }:

with lib;

let
  cfg = config.services.cim.agents;

  # Parse agent .md file (YAML front-matter + markdown)
  parseAgentFile = agentFile:
    let
      content = builtins.readFile agentFile;
      # Extract YAML front-matter between ---
      frontMatterMatch = builtins.match "^---\n(.*)---\n.*" content;
      frontMatter = if frontMatterMatch != null
        then builtins.head frontMatterMatch
        else throw "Invalid agent file: missing YAML front-matter";
      # Parse YAML to Nix attrset
      agentConfig = builtins.fromJSON (builtins.readFile (
        pkgs.runCommand "agent-yaml-to-json" {
          buildInputs = [ pkgs.yq ];
        } ''
          echo '${frontMatter}' | yq -o json > $out
        ''
      ));
    in
      agentConfig;

  # Create agent container configuration
  mkAgentContainer = name: agentCfg: {
    # Container name: cim-agent-${name}
    "cim-agent-${name}" = {
      autoStart = agentCfg.autoStart or true;

      # Container network configuration
      privateNetwork = true;
      hostBridge = agentCfg.network.bridge or "br-cim";
      localAddress = agentCfg.network.localAddress or null;

      # Bind mounts for data persistence
      bindMounts = {
        "/var/lib/cim-agent" = {
          hostPath = "/var/lib/cim-agents/${name}";
          isReadOnly = false;
        };
        "/etc/cim-agent" = {
          hostPath = "/etc/cim-agents/${name}";
          isReadOnly = true;
        };
      };

      # Container configuration
      config = { config, pkgs, ... }: {
        # Basic system configuration
        system.stateVersion = "24.05";

        # Network configuration
        networking = {
          useHostResolvConf = mkForce false;
          firewall.enable = false;
        };

        # Services configuration
        services.resolved.enable = true;

        # CIM Agent service
        systemd.services.cim-agent = {
          description = "CIM Agent: ${agentCfg.agent.display_name}";
          wantedBy = [ "multi-user.target" ];
          after = [ "network-online.target" ];
          wants = [ "network-online.target" ];

          serviceConfig = {
            Type = "simple";
            Restart = agentCfg.deployment.restart.policy or "always";
            RestartSec = agentCfg.deployment.restart.interval_sec or 10;

            # Resource limits
            MemoryMax = agentCfg.deployment.resources.memory_max or "8G";
            CPUQuota = agentCfg.deployment.resources.cpu_quota or "200%";
            TasksMax = agentCfg.deployment.resources.tasks_max or 512;

            # Security
            DynamicUser = false;
            User = "cim-agent";
            Group = "cim-agent";

            # Directories
            WorkingDirectory = "/var/lib/cim-agent";
            StateDirectory = "cim-agent";
            CacheDirectory = "cim-agent";
            LogsDirectory = "cim-agent";

            # Environment from agent configuration
            EnvironmentFile = "/etc/cim-agent/agent.env";
          };

          # Start agent-runtime with agent config
          script = ''
            exec ${cfg.package}/bin/agent-runtime \
              --agent-file /etc/cim-agent/agent.md \
              --agent-id ${agentCfg.agent.id} \
              --nats-url ${agentCfg.nats.url}
          '';
        };

        # User for agent
        users.users.cim-agent = {
          isSystemUser = true;
          group = "cim-agent";
          home = "/var/lib/cim-agent";
          createHome = true;
        };

        users.groups.cim-agent = {};

        # Environment packages
        environment.systemPackages = [ cfg.package ];
      };
    };
  };

  # Generate agent environment file
  mkAgentEnv = name: agentCfg: pkgs.writeText "agent.env" ''
    # CIM Agent Environment: ${name}
    AGENT_ID=${agentCfg.agent.id}
    AGENT_NAME=${agentCfg.agent.name}
    AGENT_VERSION=${agentCfg.agent.version}

    # Model configuration
    MODEL_PROVIDER=${agentCfg.model.provider}
    OLLAMA_URL=${agentCfg.model.ollama.url or "http://localhost:11434"}
    OLLAMA_MODEL=${agentCfg.model.ollama.model}

    # NATS configuration
    NATS_URL=${agentCfg.nats.url}
    NATS_COMMAND_SUBJECT=${replaceStrings ["{agent_id}"] [agentCfg.agent.id] agentCfg.nats.subjects.commands}

    # Logging
    LOG_LEVEL=${agentCfg.deployment.logging.level or "info"}
    LOG_FORMAT=${agentCfg.deployment.logging.format or "json"}

    # Runtime
    RUST_BACKTRACE=1
  '';

  # Agent configurations from agent files
  agentConfigs = mapAttrs (name: agentFile:
    let
      parsed = parseAgentFile agentFile;
    in
      parsed // {
        # Add derived fields
        envFile = mkAgentEnv name parsed;
        containerConfig = mkAgentContainer name parsed;
      }
  ) cfg.agents;

in
{
  options.services.cim.agents = {
    enable = mkEnableOption "CIM Agent deployment system";

    package = mkOption {
      type = types.package;
      default = pkgs.cim-domain-agent;
      description = "The cim-domain-agent package to use";
    };

    agents = mkOption {
      type = types.attrsOf types.path;
      default = {};
      description = ''
        Attribute set of agent name to agent .md file path.
        Each agent will be deployed in its own container.
      '';
      example = literalExpression ''
        {
          nats-expert = ./agents/nats-expert.md;
          ddd-expert = ./agents/ddd-expert.md;
        }
      '';
    };

    network = {
      bridge = mkOption {
        type = types.str;
        default = "br-cim";
        description = "Network bridge for agent containers";
      };

      subnet = mkOption {
        type = types.str;
        default = "10.100.0.0/24";
        description = "Subnet for agent containers";
      };
    };

    dataDir = mkOption {
      type = types.path;
      default = "/var/lib/cim-agents";
      description = "Base directory for agent data";
    };

    configDir = mkOption {
      type = types.path;
      default = "/etc/cim-agents";
      description = "Base directory for agent configurations";
    };
  };

  config = mkIf cfg.enable {
    # Create network bridge for agent containers
    networking.bridges.${cfg.network.bridge} = {
      interfaces = [];
    };

    networking.interfaces.${cfg.network.bridge} = {
      ipv4.addresses = [{
        address = builtins.head (lib.splitString "/" cfg.network.subnet);
        prefixLength = lib.toInt (lib.last (lib.splitString "/" cfg.network.subnet));
      }];
    };

    # Create base directories
    system.activationScripts.cim-agents-dirs = ''
      mkdir -p ${cfg.dataDir}
      mkdir -p ${cfg.configDir}
    '';

    # Deploy each agent configuration
    system.activationScripts.cim-agents-config = lib.concatStringsSep "\n" (
      lib.mapAttrsToList (name: agentFile:
        let
          agentConfig = agentConfigs.${name};
        in
        ''
          # Create agent-specific directories
          mkdir -p ${cfg.dataDir}/${name}
          mkdir -p ${cfg.configDir}/${name}

          # Copy agent .md file
          cp ${agentFile} ${cfg.configDir}/${name}/agent.md

          # Copy environment file
          cp ${agentConfig.envFile} ${cfg.configDir}/${name}/agent.env

          # Set permissions
          chmod 755 ${cfg.dataDir}/${name}
          chmod 755 ${cfg.configDir}/${name}
          chmod 644 ${cfg.configDir}/${name}/agent.md
          chmod 600 ${cfg.configDir}/${name}/agent.env
        ''
      ) cfg.agents
    );

    # Create container configurations
    containers = lib.mkMerge (
      lib.mapAttrsToList (_name: agentCfg: agentCfg.containerConfig) agentConfigs
    );

    # Firewall: allow agent communication on bridge
    networking.firewall.interfaces.${cfg.network.bridge}.allowedTCPPorts = [ 4222 ]; # NATS
  };
}
