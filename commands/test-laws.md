# Test Mathematical Laws

Generate property-based tests (proptest) for Category Theory axioms on a CIM type. Verifies the mathematical foundations that the architecture depends on.

## Usage

`/test-laws <type> [law]`

Where `[law]` is optional: `monoid`, `functor`, `adjunction`, `monad`, `mealy`, or `all` (default).

## What It Generates

### Monoid Laws (CT-8) — over the CARRIER, not over event logs

⛔ This section stated its laws over `Vec<Event>` and "event logs". There are no events and
no event log. Substituting `Vec<Observation>` would NOT fix it — a `Vec` of anything the
register can reproduce is a MATERIALIZED WALK, which is the residency defect. The monoid
that actually exists is the FOLD onto the head.

**The carrier is `u64` and the operation is `Modulate`.** `Modulate(head, cid) => head + cid`
with identity `0`; `Demodulate(headAfter, from) => headAfter - from` recovers the addend.
That is a monoid on `u64` under addition — associative, with a two-sided identity — and it
is the one the substrate runs on.

```rust
proptest! {
    /// Left identity: folding nothing changes nothing.
    /// FALSIFIER: any head h for which modulate(h, 0) != h.
    #[test]
    fn monoid_identity_left(h: u64) {
        prop_assert_eq!(modulate(h, 0), h);
    }

    /// Right identity: the empty fold is the identity from either side.
    /// FALSIFIER: any cid c for which modulate(0, c) != c.
    #[test]
    fn monoid_identity_right(c: u64) {
        prop_assert_eq!(modulate(0, c), c);
    }

    /// Associativity: fold order does not change the head.
    /// FALSIFIER: any (a,b,c) where the two groupings differ.
    #[test]
    fn monoid_associativity(a: u64, b: u64, c: u64) {
        prop_assert_eq!(modulate(modulate(a, b), c), modulate(a, modulate(b, c)));
    }

    /// Round-trip: Demodulate inverts Modulate. This is the law that makes a
    /// FRAME an address rather than a container — content is recovered, not shipped.
    /// FALSIFIER: any (h,c) where the recovered value differs from c.
    #[test]
    fn carrier_round_trips(h: u64, c: u64) {
        prop_assert_eq!(demodulate(modulate(h, c), h), c);
    }
}
```

⚠ **Order-forgetting is a SEPARATE law and it is already PROVEN — do not re-prove it here.**
`walk-algebra.rzk::wa-sum-forgets-order` gives `sum(A ++ B) = sum(B ++ A)`, i.e. the head
does NOT determine the walk. Cite it; a proptest cannot establish it and would only be
sampling the theorem.

### Functor Laws (CT-2) — for lift, context maps, concept associations

```rust
#[test]
fn functor_identity() {
    let obj = test_object();
    assert_eq!(functor.map(|x| x, obj.clone()), obj);
}

#[test]
fn functor_composition() {
    let obj = test_object();
    let fg = functor.map(|x| g(f(x)), obj.clone());
    let f_then_g = functor.map(g, functor.map(f, obj));
    assert_eq!(fg, f_then_g);
}
```

### Adjunction Laws (CT-6) — for lift/unlift

```rust
proptest! {
    #[test]
    fn adjunction_unit(entity in arb_entity()) {
        // unlift(lift(x)) = Some(x)
        let node = entity.lift();
        let recovered = Entity::unlift(&node);
        prop_assert_eq!(recovered, Some(entity));
    }
}
```

### Monad Laws (CT-4) — if applicable

```rust
#[test]
fn monad_left_identity() {
    // pure(a).bind(f) = f(a)
}

#[test]
fn monad_right_identity() {
    // m.bind(pure) = m
}

#[test]
fn monad_associativity() {
    // m.bind(f).bind(g) = m.bind(|x| f(x).bind(g))
}
```

### Mealy Machine — determinism

```rust
proptest! {
    #[test]
    fn mealy_deterministic(state in arb_state(), input in arb_command()) {
        let (s1, o1) = machine.step(state.clone(), input.clone());
        let (s2, o2) = machine.step(state, input);
        prop_assert_eq!(s1, s2);
        prop_assert_eq!(o1, o2);
    }
}
```

## Instructions

1. Read the type's source file
2. Determine which laws apply
3. Create proptest arbitrary generators for the type
4. Generate tests for each applicable law
5. Output complete test module with proptest strategies
