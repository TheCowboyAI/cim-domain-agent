# Test State Machine Transitions

Generate exhaustive state machine transition tests for a CIM Aggregate. Tests every valid AND every invalid transition to ensure totality (FRP-5) and that all possible states are representable while undesirable states are unrepresentable (CIM-6).

## Usage

`/test-transitions <Aggregate name>`

## What It Generates

1. **Every valid transition** — `Scenario Outline` with table of (from_state, command, to_state, event)
2. **Every invalid transition** — `Scenario Outline` with table of (from_state, command) that must be rejected
3. **Terminal state verification** — terminal states reject ALL commands
4. **Initial state** — aggregate starts in the correct initial state
5. **Event correctness** — each transition produces the correct event type
6. **State reachability** — every state is reachable from initial

## Instructions

1. Read the Aggregate's source file
2. Find the State enum (all variants)
3. Find all Command variants
4. Map the MealyStateMachine: for every (State, Command) pair, determine if valid or invalid
5. Generate a test for EVERY cell in the State × Command matrix
6. Identify terminal states (no outgoing transitions)
7. Verify every non-terminal state has at least one outgoing transition
8. Output the complete test module

## Example

```rust
#[cfg(test)]
mod transition_tests {
    use super::*;

    // ─── Valid Transitions ────────────────────────────────
    #[test]
    fn new_to_verification() {
        let (agg, _) = Aggregate::create(/* ... */);
        let (agg, events) = agg.handle(Command::StartVerification).unwrap();
        assert_eq!(agg.state(), State::Verification);
        assert_eq!(events.len(), 1);
        assert!(matches!(events[0], Event::VerificationStarted));
    }

    // ─── Invalid Transitions ─────────────────────────────
    #[test]
    fn new_rejects_complete_verification() {
        let (agg, _) = Aggregate::create(/* ... */);
        let result = agg.handle(Command::CompleteVerification);
        assert!(result.is_err());
    }

    // ─── Terminal State ──────────────────────────────────
    #[test]
    fn terminal_rejects_all() {
        let agg = make_terminal_aggregate();
        for cmd in all_commands() {
            let result = agg.clone().handle(cmd);
            assert!(result.is_err());
        }
    }
}
```
