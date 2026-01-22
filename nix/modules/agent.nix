# Copyright (c) 2025 - Cowboy AI, LLC.
#
# CIM Agent Module - Single Agent Deployment
# For use with extra-container: https://github.com/erikarvstedt/extra-container

{ config, lib, pkgs, ... }:

with lib;

let
  cfg = config.services.cim-agent;

in
{
  options.services.cim-agent = {
    enable = mkEnableOption "CIM Agent";

    agentFile = mkOption {
      type = types.path;
      description = ''
        Path to agent .md file with YAML front-matter.
        Contains complete agent configuration and system prompt.
      '';
      example = "/path/to/agents/nats-expert.md";
    };

    package = mkOption {
      type = types.package;
      description = "The cim-domain-agent package providing agent-runtime binary";
    };

    # Override specific agent config values
    agentId = mkOption {
      type = types.nullOr types.str;
      default = null;
      description = "Override agent ID from agent file (usually auto-generated)";
    };

    natsUrl = mkOption {
      type = types.str;
      default = "nats://10.0.20.1:4222";
      description = "NATS server URL";
    };

    ollamaUrl = mkOption {
      type = types.str;
      default = "http://localhost:11434";
      description = "Ollama server URL";
    };

    dataDir = mkOption {
      type = types.path;
      default = "/var/lib/cim-agent";
      description = "Data directory for agent state";
    };

    # Resource limits
    memoryMax = mkOption {
      type = types.str;
      default = "8G";
      description = "Maximum memory for agent (systemd MemoryMax)";
    };

    cpuQuota = mkOption {
      type = types.str;
      default = "200%";
      description = "CPU quota for agent (systemd CPUQuota)";
    };

    tasksMax = mkOption {
      type = types.int;
      default = 512;
      description = "Maximum number of tasks for agent (systemd TasksMax)";
    };

    # Restart policy
    restartPolicy = mkOption {
      type = types.enum [ "always" "on-failure" "no" ];
      default = "always";
      description = "Systemd restart policy";
    };

    restartSec = mkOption {
      type = types.int;
      default = 10;
      description = "Seconds to wait before restarting on failure";
    };

    # Logging
    logLevel = mkOption {
      type = types.enum [ "trace" "debug" "info" "warn" "error" ];
      default = "info";
      description = "Log level";
    };

    logFormat = mkOption {
      type = types.enum [ "json" "text" ];
      default = "json";
      description = "Log format";
    };

    # Additional environment variables
    extraEnv = mkOption {
      type = types.attrs;
      default = {};
      description = "Additional environment variables";
      example = { RUST_BACKTRACE = "1"; };
    };
  };

  config = mkIf cfg.enable {
    # Create agent user
    users.users.cim-agent = {
      isSystemUser = true;
      group = "cim-agent";
      home = cfg.dataDir;
      createHome = true;
      description = "CIM Agent Service User";
    };

    users.groups.cim-agent = {};

    # Create directories
    systemd.tmpfiles.rules = [
      "d ${cfg.dataDir} 0755 cim-agent cim-agent -"
      "d ${cfg.dataDir}/logs 0755 cim-agent cim-agent -"
      "d ${cfg.dataDir}/cache 0755 cim-agent cim-agent -"
    ];

    # CIM Agent systemd service
    systemd.services.cim-agent = {
      description = "CIM Agent Runtime";
      wantedBy = [ "multi-user.target" ];
      after = [ "network-online.target" ];
      wants = [ "network-online.target" ];

      serviceConfig = {
        Type = "simple";
        Restart = cfg.restartPolicy;
        RestartSec = cfg.restartSec;

        # User/Group
        User = "cim-agent";
        Group = "cim-agent";

        # Resource limits
        MemoryMax = cfg.memoryMax;
        CPUQuota = cfg.cpuQuota;
        TasksMax = cfg.tasksMax;

        # Directories
        WorkingDirectory = cfg.dataDir;
        StateDirectory = "cim-agent";
        CacheDirectory = "cim-agent";
        LogsDirectory = "cim-agent";

        # Security hardening
        NoNewPrivileges = true;
        PrivateTmp = true;
        ProtectSystem = "strict";
        ProtectHome = true;
        ReadWritePaths = [ cfg.dataDir ];

        # Logging
        StandardOutput = "journal";
        StandardError = "journal";
        SyslogIdentifier = "cim-agent";
      };

      environment = {
        # Agent configuration
        CIM_AGENT_FILE = cfg.agentFile;
        CIM_AGENT_ID = mkIf (cfg.agentId != null) cfg.agentId;

        # Infrastructure
        NATS_URL = cfg.natsUrl;
        OLLAMA_URL = cfg.ollamaUrl;

        # Logging
        LOG_LEVEL = cfg.logLevel;
        LOG_FORMAT = cfg.logFormat;
        RUST_LOG = "cim_domain_agent=${cfg.logLevel}";

        # Runtime
        RUST_BACKTRACE = "1";
      } // cfg.extraEnv;

      # Start agent-runtime
      script = ''
        exec ${cfg.package}/bin/agent-runtime \
          --agent-file ${cfg.agentFile} \
          --nats-url ${cfg.natsUrl}
      '';
    };

    # Install agent package
    environment.systemPackages = [ cfg.package ];
  };
}
