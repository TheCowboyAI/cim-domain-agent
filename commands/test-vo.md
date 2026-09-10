# Test ValueObject Construction

Generate construction validation tests for a CIM ValueObject. Tests every valid and invalid construction path to ensure illegal states are unrepresentable (CIM-6).

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

`/test-vo <ValueObject name>`

## What It Generates

For the given ValueObject type:

1. **Valid construction** — every valid constructor produces the correct value
2. **Invalid construction** — every invalid input is rejected with appropriate error
3. **Boundary values** — min, max, edge cases
4. **Equality** — same values are equal, different values are not
5. **Clone** — cloned value equals original
6. **Debug** — debug representation doesn't expose sensitive data (for SSN, etc.)
7. **Concept association** — ValueObject declares its Concepts
8. **PartialOrder** — if orderable, ordering is correct

## Instructions

1. Read the ValueObject's source file
2. Find ALL constructors and validation rules
3. Generate a test for EVERY validation path
4. Generate proptest for valid range if applicable
5. Output the complete test module

## Example

For `Money`:
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_construction() {
        let m = Money::from_cents(100).unwrap();
        assert_eq!(m.amount_minor(), 100);
    }

    #[test]
    fn rejects_negative() {
        assert!(Money::from_cents(-1).is_err());
    }

    #[test]
    fn zero_is_valid() {
        let m = Money::from_cents(0).unwrap();
        assert_eq!(m.amount_minor(), 0);
    }

    #[test]
    fn equality() {
        assert_eq!(Money::from_cents(100).unwrap(), Money::from_cents(100).unwrap());
        assert_ne!(Money::from_cents(100).unwrap(), Money::from_cents(200).unwrap());
    }
}
```
