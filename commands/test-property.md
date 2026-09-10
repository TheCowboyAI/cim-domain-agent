# Test Property

Generate a proptest strategy and property-based test for a CIM function or type. Property tests verify behavior for ARBITRARY inputs, not just hand-picked examples.

## ⛔ PROOF FIRST — proptest is the gate-6 check, never the evidence

**This command does NOT establish a law.** Under PROOFS FIRST, a law is established by a
proof; proptest is the EMPIRICAL step of gate 6, run AFTER the theorem exists, to see
whether the implementation follows it. A green proptest over a law with no proof is a
measurement artifact wearing evidence's clothes — it cannot distinguish "the law holds"
from "the generator never produced a falsifying input".

**Before generating anything, answer the six gates out loud:**
1. What are we trying to do? 2. Is it possible (bounded, terminating)? 3. Have we already
proven it — SEARCH the corpus first; a hit is a CITATION, not a new file. 3.5. Is the
SCOPE drawn? 4. Can it be proven — write the proof of the INTENT. 5. Is there a commuting
olog / string diagram? 6. Design the code around it, by the scientific method.

**State the FALSIFIER in the generated file**: *what result would withdraw this?* A law you
cannot falsify has not been tested.

## Laws are stated over the CARRIER, not over aggregates or event logs

| law | statement | where it lives |
|---|---|---|
| **Carrier round-trip** | `Demodulate(Modulate(h, c), h) == c` | `CarrierKernel.cs` — Modulate is `a+b`, Demodulate is `ab−b` |
| **Fold monotonicity** | folding an observation never decreases any cell; the register accumulates and never mutates | `AddResidue` touches ALL FOURTEEN cells — one observation, fourteen positions |
| **Walk determinism** | same vantage (seed × ranking) ⟹ same walk. Different vantage ⟹ a different coherent projection of the SAME substrate | `Stream.cs` |
| **Order-forgetting** | `sum(A ++ B) == sum(B ++ A)` — the head does not determine the walk | `walk-algebra.rzk::wa-sum-forgets-order` |
| **Quorum detection** | `Count(cid)` is a DETECTION COUNT in `0..14`; `Contains(cid) ⟺ Count(cid) == 14`, and a negative answer is EXACT | `prime-quorum-detection.rzk` A.2 — **not** a min; min is the dormant A.3 and ranges over `ResidueCounts` |

⛔ **Do NOT state a law over an Aggregate, a command/event pair, an event log, or a
declared ConceptualSpace.** None of those exist. If the thing you want to test is
reachable by walking from a seed, test the WALK.

⛔ **ABSTENTION BEATS GUESSING.** State whether the property permits abstention. 19 honest
misses with 0 wrong answers is a stronger result than 69% with wrong answers in it, and no
accuracy number alone distinguishes them.

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
