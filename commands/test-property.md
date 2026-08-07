# Test Property

Generate a proptest strategy and property-based test for a CIM function or type. Property tests verify behavior for ARBITRARY inputs, not just hand-picked examples.

## Usage

`/test-property <function or type> [property]`

Where `[property]` is optional: `deterministic`, `commutative`, `associative`, `idempotent`, `total`, `round-trip`, or describe the property.

## What It Generates

1. **Arbitrary strategy** — proptest `Arbitrary` impl or custom strategy for the type
2. **Property assertion** — the property that must hold for all generated inputs
3. **Shrinking** — proptest auto-shrinks failing cases to minimal counterexample

## Common Properties

### Deterministic (same input = same output)
```rust
proptest! {
    #[test]
    fn deterministic(input in arb_input()) {
        let r1 = function(&input);
        let r2 = function(&input);
        prop_assert_eq!(r1, r2);
    }
}
```

### Idempotent (applying twice = applying once)
```rust
proptest! {
    #[test]
    fn idempotent(input in arb_input()) {
        let once = function(input.clone());
        let twice = function(function(input));
        prop_assert_eq!(once, twice);
    }
}
```

### Round-trip (encode then decode = identity)
```rust
proptest! {
    #[test]
    fn round_trip(entity in arb_entity()) {
        let encoded = serde_json::to_vec(&entity).unwrap();
        let decoded: Entity = serde_json::from_slice(&encoded).unwrap();
        prop_assert_eq!(entity, decoded);
    }
}
```

### Total (defined for all inputs — no panic)
```rust
proptest! {
    #[test]
    fn total(input in any::<i64>()) {
        // Must not panic for any input
        let _ = function(input);
    }
}
```

## Instructions

1. Read the function/type signature
2. Create proptest strategy that generates valid and boundary inputs
3. Generate the property test
4. Output the strategy + test
