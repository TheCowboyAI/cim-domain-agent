# Test NATS Integration

Generate NATS integration test scaffold for a CIM aggregate. Uses REAL NATS at localhost:4222 — never mock.

## Usage

`/test-nats <Aggregate name>`

## What It Generates

1. **Stream setup/teardown** — unique stream name per test, cleanup after
2. **Event publish/consume round trip** — publish event, consume via push consumer, verify
3. **Command request-reply** — send command, receive response, verify event produced
4. **Aggregate reconstitution** — append events, read back, fold to state, verify
5. **Multi-aggregate isolation** — two aggregates don't interfere
6. **CIM header verification** — all required headers present and correct

## Template

```rust
use async_nats::jetstream;

/// Unique stream name to avoid test interference
fn test_stream(prefix: &str) -> String {
    format!("TEST_{}_{}", prefix, uuid::Uuid::now_v7().simple())
}

/// Cleanup stream after test
async fn cleanup(js: &jetstream::Context, stream_name: &str) {
    let _ = js.delete_stream(stream_name).await;
}

#[tokio::test]
async fn event_publish_consume_round_trip() {
    let client = async_nats::connect("nats://localhost:4222").await.unwrap();
    let js = jetstream::new(client.clone());
    let stream_name = test_stream("AGGREGATE");

    // Setup
    js.get_or_create_stream(jetstream::stream::Config {
        name: stream_name.clone(),
        subjects: vec![format!("{}.>", stream_name)],
        ..Default::default()
    }).await.unwrap();

    // Publish event with CIM headers
    // Consume via push consumer
    // Verify event data matches
    // Verify CIM headers present

    // Teardown
    cleanup(&js, &stream_name).await;
}

#[tokio::test]
async fn command_request_reply() {
    // Send command via NATS request
    // Handler processes, produces events
    // Reply contains result
    // Events in stream
}

#[tokio::test]
async fn aggregate_reconstitution() {
    // Append 3+ events to stream
    // Read all events
    // Fold into state
    // Verify final state matches expected
}
```

## Instructions

1. Read the Aggregate's events and commands
2. Generate unique stream names
3. Create publish, consume, command-reply, and reconstitution tests
4. Verify CIM headers (Nats-Msg-Id, CIM-Correlation-Id, CIM-Causation-Id, CIM-Event-Type)
5. Always cleanup streams after test
