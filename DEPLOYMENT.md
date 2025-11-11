# Agent Domain Deployment Guide

This guide covers deploying the **cim-domain-agent** service in production environments.

## Table of Contents

1. [Architecture Overview](#architecture-overview)
2. [Prerequisites](#prerequisites)
3. [Deployment Strategies](#deployment-strategies)
4. [Configuration](#configuration)
5. [NATS Setup](#nats-setup)
6. [Monitoring & Observability](#monitoring--observability)
7. [Security](#security)
8. [Scaling](#scaling)
9. [Disaster Recovery](#disaster-recovery)

## Architecture Overview

### CIM Deployment Topology

```
┌─────────────────────────────────────────────────────────────┐
│                    Client (Development)                      │
│  ┌──────────────────────────────────────────────────────┐   │
│  │  Local NATS Client                                    │   │
│  └──────────────────┬───────────────────────────────────┘   │
└────────────────────│────────────────────────────────────────┘
                     │
                     ↓ (Leaf Node Connection)
┌─────────────────────────────────────────────────────────────┐
│                    Leaf Node (Server)                        │
│  ┌──────────────────────────────────────────────────────┐   │
│  │  NATS Server (JetStream Enabled)                     │   │
│  └───┬──────────────────────────────────────────────────┘   │
│      │                                                        │
│  ┌───▼────────────┬──────────────┬──────────────┐          │
│  │ agent-service  │ other-service│ other-service│          │
│  └────────────────┴──────────────┴──────────────┘          │
└─────────────────────────────────────────────────────────────┘
                     │
                     ↓ (Cluster Connection - Optional)
┌─────────────────────────────────────────────────────────────┐
│                NATS Cluster (High Availability)              │
│  ┌──────────┐  ┌──────────┐  ┌──────────┐                  │
│  │ Leaf  Node│─│ Leaf Node│─│ Leaf Node│                  │
│  │    1      │  │    2     │  │    3     │                  │
│  └──────────┘  └──────────┘  └──────────┘                  │
└─────────────────────────────────────────────────────────────┘
```

### Service Components

1. **agent-service**: Production NATS service binary
2. **NATS JetStream**: Event persistence and message bus
3. **Event Store**: Historical event storage (JetStream)
4. **Snapshot Store**: Periodic state snapshots (In-memory or external)

## Prerequisites

### System Requirements

- **OS**: Linux (NixOS recommended), macOS, or Windows (WSL2)
- **RAM**: Minimum 2GB, recommended 4GB+
- **CPU**: 2+ cores
- **Disk**: 10GB+ for event storage
- **Network**: Low-latency connection to NATS server

### Software Dependencies

- **Rust**: 1.70+ (for building from source)
- **NATS Server**: 2.10+ with JetStream enabled
- **Optional**: Docker/Podman for containerized deployment

### NATS Server Installation

#### Using nats-server Binary

```bash
# Download and install
curl -L https://github.com/nats-io/nats-server/releases/download/v2.10.7/nats-server-v2.10.7-linux-amd64.tar.gz | tar xz
sudo mv nats-server-v2.10.7-linux-amd64/nats-server /usr/local/bin/

# Start with JetStream
nats-server -js
```

#### Using Docker

```bash
docker run -p 4222:4222 -p 8222:8222 -p 6222:6222 \
  -v ./nats-data:/data \
  nats:latest -js -sd /data
```

#### Using NixOS

```nix
{ config, pkgs, ... }:
{
  services.nats = {
    enable = true;
    jetstream = true;
    settings = {
      jetstream = {
        store_dir = "/var/lib/nats/jetstream";
        max_memory_store = 1073741824; # 1GB
        max_file_store = 10737418240;  # 10GB
      };
    };
  };
}
```

## Deployment Strategies

### 1. Single Server Deployment

**Best for**: Development, testing, small-scale production

```bash
# Build the service
cargo build --release --bin agent-service

# Create systemd service
sudo tee /etc/systemd/system/agent-service.service << EOF
[Unit]
Description=CIM Agent Domain Service
After=network.target nats.service
Requires=nats.service

[Service]
Type=simple
User=agent-service
Group=agent-service
WorkingDirectory=/opt/cim-domain-agent
Environment=NATS_URL=nats://localhost:4222
Environment=STREAM_NAME=AGENT_EVENTS
Environment=LOG_LEVEL=info
Environment=SNAPSHOT_FREQUENCY=100
ExecStart=/opt/cim-domain-agent/target/release/agent-service
Restart=on-failure
RestartSec=5s

[Install]
WantedBy=multi-user.target
EOF

# Enable and start
sudo systemctl enable agent-service
sudo systemctl start agent-service
```

### 2. Clustered Deployment

**Best for**: High availability, production workloads

#### NATS Cluster Configuration

```conf
# nats-node1.conf
server_name: node1
listen: 0.0.0.0:4222
http: 8222

jetstream {
  store_dir: /var/lib/nats/jetstream
  max_memory_store: 1GB
  max_file_store: 10GB
}

cluster {
  name: agent_cluster
  listen: 0.0.0.0:6222
  routes: [
    nats://node2:6222
    nats://node3:6222
  ]
}

# Replicate across cluster
accounts {
  $SYS {
    jetstream: enabled
  }
}
```

#### Deploy Multiple agent-service Instances

```bash
# On each node
NATS_URL=nats://node1:4222,nats://node2:4222,nats://node3:4222 \
  ./target/release/agent-service
```

### 3. Containerized Deployment

#### Dockerfile

```dockerfile
FROM rust:1.75 as builder
WORKDIR /build
COPY . .
RUN cargo build --release --bin agent-service

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /build/target/release/agent-service /usr/local/bin/
ENTRYPOINT ["/usr/local/bin/agent-service"]
```

#### Docker Compose

```yaml
version: '3.8'

services:
  nats:
    image: nats:latest
    command: ["-js", "-sd", "/data"]
    ports:
      - "4222:4222"
      - "8222:8222"
      - "6222:6222"
    volumes:
      - nats-data:/data

  agent-service:
    build: .
    environment:
      - NATS_URL=nats://nats:4222
      - STREAM_NAME=AGENT_EVENTS
      - LOG_LEVEL=info
      - SNAPSHOT_FREQUENCY=100
    depends_on:
      - nats
    restart: unless-stopped

volumes:
  nats-data:
```

### 4. Kubernetes Deployment

#### Deployment Manifest

```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: agent-service
  labels:
    app: agent-service
spec:
  replicas: 3
  selector:
    matchLabels:
      app: agent-service
  template:
    metadata:
      labels:
        app: agent-service
    spec:
      containers:
      - name: agent-service
        image: your-registry/agent-service:latest
        env:
        - name: NATS_URL
          value: "nats://nats-cluster:4222"
        - name: STREAM_NAME
          value: "AGENT_EVENTS"
        - name: LOG_LEVEL
          value: "info"
        - name: SNAPSHOT_FREQUENCY
          value: "100"
        resources:
          requests:
            memory: "256Mi"
            cpu: "250m"
          limits:
            memory: "512Mi"
            cpu: "500m"
        livenessProbe:
          exec:
            command:
            - /bin/sh
            - -c
            - "pgrep agent-service"
          initialDelaySeconds: 30
          periodSeconds: 10
        readinessProbe:
          exec:
            command:
            - /bin/sh
            - -c
            - "pgrep agent-service"
          initialDelaySeconds: 5
          periodSeconds: 5
---
apiVersion: v1
kind: Service
metadata:
  name: agent-service
spec:
  selector:
    app: agent-service
  ports:
  - protocol: TCP
    port: 8080
    targetPort: 8080
  type: ClusterIP
```

## Configuration

### Environment Variables

| Variable | Default | Description |
|----------|---------|-------------|
| `NATS_URL` | `nats://localhost:4222` | NATS server URL(s), comma-separated for cluster |
| `STREAM_NAME` | `AGENT_EVENTS` | JetStream stream name |
| `LOG_LEVEL` | `info` | Logging level: trace, debug, info, warn, error |
| `SNAPSHOT_FREQUENCY` | `100` | Create snapshot every N events |

### NATS Stream Configuration

The service automatically creates a JetStream stream with these defaults:

```rust
jetstream::stream::Config {
    name: "AGENT_EVENTS",
    subjects: vec!["agent.commands.>", "agent.events.>"],
    max_age: Duration::from_secs(365 * 24 * 60 * 60), // 1 year
    storage: StorageType::File,
    retention: RetentionPolicy::Limits,
    ..Default::default()
}
```

### Tuning Snapshot Frequency

**Low-frequency events** (< 100/day): `SNAPSHOT_FREQUENCY=100`
**Medium-frequency events** (100-1000/day): `SNAPSHOT_FREQUENCY=50`
**High-frequency events** (> 1000/day): `SNAPSHOT_FREQUENCY=25`

## NATS Setup

### Enabling JetStream

```conf
# nats-server.conf
jetstream {
  store_dir: /var/lib/nats/jetstream
  max_memory_store: 1GB
  max_file_store: 10GB
}
```

### Creating Streams Manually

```bash
# Using nats CLI
nats stream add AGENT_EVENTS \
  --subjects "agent.commands.>,agent.events.>" \
  --retention limits \
  --storage file \
  --max-age 1y
```

### Stream Monitoring

```bash
# Check stream info
nats stream info AGENT_EVENTS

# List consumers
nats consumer list AGENT_EVENTS

# Monitor messages
nats stream view AGENT_EVENTS
```

## Monitoring & Observability

### Logging

The service uses **tracing** for structured logging:

```rust
// Logs appear as:
2024-01-15T10:30:45.123Z INFO agent-service: Starting agent service...
2024-01-15T10:30:45.456Z INFO agent-service: Connected to NATS
2024-01-15T10:30:46.789Z INFO agent-service: Agent deployed: 01234567-89ab-cdef-0123-456789abcdef
```

### Metrics Collection

**Recommended**: Export to Prometheus

```bash
# Use NATS Prometheus exporter
docker run -p 7777:7777 \
  natsio/prometheus-nats-exporter:latest \
  -varz http://nats-server:8222/varz
```

### Health Checks

```bash
# Check service is running
ps aux | grep agent-service

# Check NATS connection
nats account info

# Verify event stream
nats stream info AGENT_EVENTS
```

### Tracing

Enable distributed tracing with OpenTelemetry:

```rust
// Add to Cargo.toml
tracing-opentelemetry = "0.21"
opentelemetry-jaeger = "0.20"

// Configure in main()
let tracer = opentelemetry_jaeger::new_agent_pipeline()
    .with_service_name("agent-service")
    .install_simple()?;
```

## Security

### Authentication

**NATS JWT Authentication**:

```conf
# nats-server.conf
authorization {
  users = [
    {
      user: "agent-service"
      password: "$2a$11$..." # bcrypt hash
      permissions: {
        publish: ["agent.events.>"]
        subscribe: ["agent.commands.>"]
      }
    }
  ]
}
```

### TLS Encryption

```conf
# nats-server.conf
tls {
  cert_file: "/etc/nats/certs/server-cert.pem"
  key_file: "/etc/nats/certs/server-key.pem"
  ca_file: "/etc/nats/certs/ca.pem"
  verify: true
}
```

Connect with TLS:

```bash
NATS_URL=tls://nats.example.com:4222 ./agent-service
```

### Subject-Based Authorization

Restrict access by subject patterns:

```conf
permissions: {
  publish: {
    allow: ["agent.events.{user}.>"]
  }
  subscribe: {
    allow: ["agent.commands.{user}.>"]
  }
}
```

## Scaling

### Horizontal Scaling

**Multiple service instances** can run concurrently:

- Commands are load-balanced across instances
- Events are published to all subscribers
- Use NATS queue groups for workload distribution

```rust
// Enable queue groups for load balancing
let subscriber = client
    .queue_subscribe("agent.commands.>", "agent-workers")
    .await?;
```

### Vertical Scaling

**Increase resources** for high-throughput scenarios:

- **CPU**: More cores for concurrent command processing
- **Memory**: Larger snapshot cache
- **Disk**: Faster SSD for event store

### Event Store Scaling

**JetStream Replication**:

```bash
nats stream add AGENT_EVENTS \
  --subjects "agent.commands.>,agent.events.>" \
  --replicas 3 \
  --storage file
```

## Disaster Recovery

### Backup Strategies

#### 1. JetStream Snapshots

```bash
# Backup stream
nats stream backup AGENT_EVENTS ./backups/agent-events-$(date +%Y%m%d).tar.gz

# Restore stream
nats stream restore AGENT_EVENTS ./backups/agent-events-20240115.tar.gz
```

#### 2. File-Based Backups

```bash
# Backup JetStream data directory
tar -czf jetstream-backup-$(date +%Y%m%d).tar.gz \
  /var/lib/nats/jetstream/
```

#### 3. Continuous Replication

Use NATS clustering with replicas=3 for automatic replication.

### Recovery Procedures

#### Service Failure

```bash
# Restart service
sudo systemctl restart agent-service

# Check logs
journalctl -u agent-service -f
```

#### NATS Cluster Failover

NATS clients automatically failover to healthy nodes:

```bash
NATS_URL=nats://node1:4222,nats://node2:4222,nats://node3:4222
```

#### Data Corruption

```bash
# Restore from backup
nats stream restore AGENT_EVENTS ./backups/latest.tar.gz

# Restart service to reconnect
sudo systemctl restart agent-service
```

## Operational Runbook

### Daily Operations

```bash
# Check service status
sudo systemctl status agent-service

# View recent logs
journalctl -u agent-service --since "1 hour ago"

# Monitor NATS stream
nats stream info AGENT_EVENTS
```

### Weekly Maintenance

```bash
# Backup event stream
nats stream backup AGENT_EVENTS ./backups/weekly-$(date +%Y%m%d).tar.gz

# Check disk usage
df -h /var/lib/nats/jetstream

# Review error logs
journalctl -u agent-service -p err --since "1 week ago"
```

### Monthly Tasks

```bash
# Archive old events (if retention policy allows)
# Rotate backups (keep last 6 months)
find ./backups -name "*.tar.gz" -mtime +180 -delete

# Performance review
nats stream report
```

## Troubleshooting

### Service Won't Start

```bash
# Check NATS connectivity
nats-server --version
nats account info

# Verify environment variables
env | grep NATS

# Check file permissions
ls -la /opt/cim-domain-agent

# Review systemd logs
journalctl -u agent-service -n 50
```

### High Memory Usage

```bash
# Check snapshot frequency (reduce if too high)
# Monitor event throughput
nats stream info AGENT_EVENTS

# Adjust JetStream limits
# nats-server.conf
jetstream {
  max_memory_store: 2GB
}
```

### Event Loss

```bash
# Verify JetStream is enabled
nats stream list

# Check stream replicas
nats stream info AGENT_EVENTS

# Review NATS server logs
journalctl -u nats -n 100
```

## Additional Resources

- [USAGE.md](./USAGE.md) - API usage examples
- [NATS Documentation](https://docs.nats.io/)
- [NixOS Deployment](https://nixos.org/manual/nixos/stable/)
- [Kubernetes Operators](https://nats-io.github.io/k8s/)

## Support

For deployment issues:
- Check [GitHub Issues](https://github.com/TheCowboyAI/cim-domain-agent/issues)
- Review [CIM Documentation](https://github.com/TheCowboyAI/cim)
- Contact CIM support team
