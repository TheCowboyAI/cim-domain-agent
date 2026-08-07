# Test ValueObject Construction

Generate construction validation tests for a CIM ValueObject. Tests every valid and invalid construction path to ensure illegal states are unrepresentable (CIM-6).

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
