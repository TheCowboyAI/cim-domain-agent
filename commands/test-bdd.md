# Test BDD Scenario

Generate an executable Rust test from a BDD .feature file scenario. Maps Given/When/Then to test setup/action/assertion.

## Usage

`/test-bdd <feature file> [scenario name]`

## What It Does

1. Read the .feature file
2. Parse the specified scenario (or all scenarios if name omitted)
3. Generate a Rust `#[test]` function for each scenario
4. Map Given → test setup (construct aggregate in correct state)
5. Map When → test action (handle command)
6. Map Then → assertions (verify state, events, errors)
7. Handle Scenario Outline tables (generate one test per example row)

## Example

Given feature file:
```gherkin
Scenario: Start verification from New
  Given a Borrower in status New
  When I handle a StartVerification command
  Then the Borrower transitions to Verification
  And a VerificationStarted event is emitted
```

Generates:
```rust
#[test]
fn start_verification_from_new() {
    // Given
    let (borrower, _) = Borrower::create(/* valid data */);
    assert_eq!(borrower.state(), BorrowerState::New);

    // When
    let result = borrower.handle(BorrowerCommand::StartVerification);

    // Then
    let (borrower, events) = result.unwrap();
    assert_eq!(borrower.state(), BorrowerState::Verification);
    assert_eq!(events.len(), 1);
    assert!(matches!(events[0], BorrowerEvent::VerificationStarted));
}
```

For Scenario Outlines:
```gherkin
Scenario Outline: Valid transitions
  Given a Borrower in state <from>
  When the command <command> is handled
  Then the Borrower transitions to <to>

  Examples:
    | from | command | to |
    | New  | StartVerification | Verification |
    | Verification | CompleteVerification | Active |
```

Generates one test per row.

## Instructions

1. Read the .feature file
2. For each scenario, generate a `#[test]` function
3. Use the aggregate's actual API (create, handle)
4. For "Given state X" — construct aggregate then apply events to reach state X
5. For Scenario Outline — generate parameterized tests
6. Output complete test module
