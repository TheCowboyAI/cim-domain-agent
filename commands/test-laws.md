# Test Mathematical Laws

Generate property-based tests (proptest) for Category Theory axioms on a CIM type. Verifies the mathematical foundations that the architecture depends on.

## Usage

`/test-laws <type> [law]`

Where `[law]` is optional: `monoid`, `functor`, `adjunction`, `monad`, `mealy`, or `all` (default).

## What It Generates

### Monoid Laws (CT-8) — for event logs, ValueObject collections

```rust
proptest! {
    #[test]
    fn monoid_identity_left(events in vec(arb_event(), 0..50)) {
        let empty: Vec<Event> = vec![];
        let result = fold(empty.into_iter().chain(events.iter().cloned()), initial());
        let expected = fold(events.into_iter(), initial());
        prop_assert_eq!(result, expected);
    }

    #[test]
    fn monoid_identity_right(events in vec(arb_event(), 0..50)) {
        let empty: Vec<Event> = vec![];
        let result = fold(events.iter().cloned().chain(empty.into_iter()), initial());
        let expected = fold(events.into_iter(), initial());
        prop_assert_eq!(result, expected);
    }

    #[test]
    fn monoid_associativity(
        a in vec(arb_event(), 0..20),
        b in vec(arb_event(), 0..20),
    ) {
        let ab = fold(a.iter().chain(b.iter()).cloned(), initial());
        let a_then_b = fold(b.into_iter(), fold(a.into_iter(), initial()));
        prop_assert_eq!(ab, a_then_b);
    }
}
```

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
