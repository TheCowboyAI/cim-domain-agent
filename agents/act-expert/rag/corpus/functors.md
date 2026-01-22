# Functor Patterns in CIM

<!-- Copyright (c) 2025 - Cowboy AI, LLC. -->

## DomainFunctor Trait

**ID:** pattern-functor-001
**Tags:** functor, cim-domain, trait
**Type:** pattern

The core trait for functors between domain categories in CIM systems.

**Implementation:**
```rust
#[async_trait]
pub trait DomainFunctor: Send + Sync {
    type Source: Send + Sync;
    type Target: Send + Sync;

    async fn map_object(&self, obj: DomainObject) -> Result<DomainObject, DomainError>;
    async fn map_morphism(&self, morph: DomainMorphism) -> Result<DomainMorphism, DomainError>;

    fn source_category(&self) -> String;
    fn target_category(&self) -> String;
}
```

**Verification Requirements:**
1. **Identity Law:** `F(id) = id` must hold
2. **Composition Law:** `F(g ∘ f) = F(g) ∘ F(f)` must hold
3. **Purity:** Both `map_object` and `map_morphism` must be pure (no side effects)

**Test Template:**
```rust
#[tokio::test]
async fn verify_functor_identity_law() {
    let functor = MyFunctor::new();
    let identity = DomainMorphism::identity("SourceCategory");

    let mapped = functor.map_morphism(identity).await.unwrap();

    assert!(mapped.is_identity());
    assert_eq!(mapped.category(), "TargetCategory");
}

#[tokio::test]
async fn verify_functor_composition_law() {
    let functor = MyFunctor::new();
    let f = /* morphism A → B */;
    let g = /* morphism B → C */;

    // Left side: F(g ∘ f)
    let composed = compose_morphisms(f.clone(), g.clone());
    let left = functor.map_morphism(composed).await.unwrap();

    // Right side: F(g) ∘ F(f)
    let mapped_f = functor.map_morphism(f).await.unwrap();
    let mapped_g = functor.map_morphism(g).await.unwrap();
    let right = compose_morphisms(mapped_f, mapped_g);

    assert_eq!(left, right);
}
```

**Common Violations:** violation-functor-001, violation-functor-002
**Related:** def-functor-001, law-identity-001, law-composition-001

---

## Identity Functor

**ID:** pattern-identity-functor-001
**Tags:** functor, identity, trivial
**Type:** pattern

The identity functor maps a category to itself.

**Implementation:**
```rust
pub struct FunctorIdentity<C: Send + Sync> {
    category: String,
    _phantom: std::marker::PhantomData<C>,
}

#[async_trait]
impl<C: Send + Sync> DomainFunctor for FunctorIdentity<C> {
    type Source = C;
    type Target = C;

    async fn map_object(&self, obj: DomainObject) -> Result<DomainObject, DomainError> {
        Ok(obj) // Identity: no change
    }

    async fn map_morphism(&self, morph: DomainMorphism) -> Result<DomainMorphism, DomainError> {
        Ok(morph) // Identity: no change
    }

    fn source_category(&self) -> String {
        self.category.clone()
    }

    fn target_category(&self) -> String {
        self.category.clone()
    }
}
```

**Properties:**
- Always satisfies functor laws (trivially)
- Useful for testing and composition
- Neutral element for functor composition

**Verification:** Should always pass all tests.

**Related:** def-functor-001, def-identity-001

---

## Functor Composition

**ID:** pattern-functor-composition-001
**Tags:** functor, composition, higher-order
**Type:** pattern

Composing two functors F : C → D and G : D → E produces G ∘ F : C → E.

**Implementation:**
```rust
pub struct FunctorComposition<F, G>
where
    F: DomainFunctor,
    G: DomainFunctor,
{
    f: F,
    g: G,
}

#[async_trait]
impl<F, G> DomainFunctor for FunctorComposition<F, G>
where
    F: DomainFunctor + Send + Sync,
    G: DomainFunctor + Send + Sync,
{
    type Source = F::Source;
    type Target = G::Target;

    async fn map_object(&self, obj: DomainObject) -> Result<DomainObject, DomainError> {
        let intermediate = self.f.map_object(obj).await?;
        self.g.map_object(intermediate).await
    }

    async fn map_morphism(&self, morph: DomainMorphism) -> Result<DomainMorphism, DomainError> {
        let intermediate = self.f.map_morphism(morph).await?;
        self.g.map_morphism(intermediate).await
    }

    fn source_category(&self) -> String {
        self.f.source_category()
    }

    fn target_category(&self) -> String {
        self.g.target_category()
    }
}
```

**Properties:**
- If F and G satisfy functor laws, so does G ∘ F
- Composition is associative: (H ∘ G) ∘ F = H ∘ (G ∘ F)
- Identity is neutral: F ∘ Id = F = Id ∘ F

**Verification:** ACT expert should verify composed functors satisfy laws.

**Related:** def-composition-001, pattern-functor-001

---

## Context Mapping Functor

**ID:** pattern-context-mapping-001
**Tags:** functor, context-mapping, ddd
**Type:** pattern

Maps objects and events between bounded contexts while preserving domain semantics.

**Implementation:**
```rust
pub struct ContextMappingFunctor {
    source: String,
    target: String,
    object_mapper: Box<dyn ObjectMapper>,
    morphism_mapper: Box<dyn MorphismMapper>,
}

#[async_trait]
impl DomainFunctor for ContextMappingFunctor {
    type Source = ();
    type Target = ();

    async fn map_object(&self, obj: DomainObject) -> Result<DomainObject, DomainError> {
        // Transform object from source context to target context
        self.object_mapper.map(obj, &self.source, &self.target).await
    }

    async fn map_morphism(&self, morph: DomainMorphism) -> Result<DomainMorphism, DomainError> {
        // Transform event from source context to target context
        self.morphism_mapper.map(morph, &self.source, &self.target).await
    }

    fn source_category(&self) -> String {
        self.source.clone()
    }

    fn target_category(&self) -> String {
        self.target.clone()
    }
}
```

**Use Cases:**
- **Shared Kernel:** Map between tightly coupled contexts
- **Customer/Supplier:** Downstream maps upstream events
- **Conformist:** Accept upstream model as-is
- **Anticorruption Layer:** Protect domain from external models

**Verification:**
- Must preserve domain invariants
- Must preserve causation chains
- Must satisfy functor laws

**Related:** pattern-anti-corruption-001, def-functor-001

---

## Anti-Corruption Layer Functor

**ID:** pattern-anti-corruption-001
**Tags:** functor, acl, protection
**Type:** pattern

Protects domain integrity by validating and transforming external models.

**Implementation:**
```rust
pub struct AntiCorruptionFunctor<V>
where
    V: DomainValidator,
{
    source: String,
    target: String,
    validator: V,
    transformer: Box<dyn DomainTransformer>,
}

#[async_trait]
impl<V> DomainFunctor for AntiCorruptionFunctor<V>
where
    V: DomainValidator + Send + Sync,
{
    type Source = ();
    type Target = ();

    async fn map_object(&self, obj: DomainObject) -> Result<DomainObject, DomainError> {
        // 1. Validate external object
        self.validator.validate(&obj).await?;

        // 2. Transform to internal model
        let internal = self.transformer.transform(obj).await?;

        // 3. Validate internal model
        self.validator.validate_internal(&internal).await?;

        Ok(internal)
    }

    async fn map_morphism(&self, morph: DomainMorphism) -> Result<DomainMorphism, DomainError> {
        // Similar validation and transformation for events
        self.validator.validate_morphism(&morph).await?;
        let internal = self.transformer.transform_morphism(morph).await?;
        self.validator.validate_internal_morphism(&internal).await?;
        Ok(internal)
    }

    fn source_category(&self) -> String {
        self.source.clone()
    }

    fn target_category(&self) -> String {
        self.target.clone()
    }
}
```

**Validation Requirements:**
- Reject invalid external objects
- Enforce domain invariants
- Prevent corruption of internal model

**Verification:**
- Must satisfy functor laws (for valid inputs)
- Must reject invalid inputs
- Must preserve domain invariants

**Related:** pattern-context-mapping-001, def-functor-001

---

## Forgetful Functor

**ID:** pattern-forgetful-functor-001
**Tags:** functor, forgetful, projection
**Type:** pattern

"Forgets" structure by projecting to simpler representation.

**Implementation:**
```rust
pub struct ForgetfulFunctor {
    source: String,
    target: String,
    properties_to_forget: Vec<String>,
}

#[async_trait]
impl DomainFunctor for ForgetfulFunctor {
    type Source = ();
    type Target = ();

    async fn map_object(&self, obj: DomainObject) -> Result<DomainObject, DomainError> {
        let mut simplified = obj.clone();

        // Remove specified properties
        for property in &self.properties_to_forget {
            simplified.remove_property(property);
        }

        Ok(simplified)
    }

    async fn map_morphism(&self, morph: DomainMorphism) -> Result<DomainMorphism, DomainError> {
        let mut simplified = morph.clone();

        // Simplify event by removing properties
        for property in &self.properties_to_forget {
            simplified.remove_property(property);
        }

        Ok(simplified)
    }

    fn source_category(&self) -> String {
        self.source.clone()
    }

    fn target_category(&self) -> String {
        self.target.clone()
    }
}
```

**Use Cases:**
- Public API projections (hide internal details)
- CQRS read models (simplified views)
- Event notifications (minimal information)

**Properties:**
- Generally not invertible (information is lost)
- Must still satisfy functor laws
- Composition g ∘ f forgets more than f alone

**Verification:**
- Must satisfy functor laws for simplified objects
- Must not break domain invariants in target

**Related:** def-functor-001, pattern-cqrs-001

---

## Aggregate as Functor

**ID:** pattern-aggregate-functor-001
**Tags:** functor, aggregate, event-sourcing
**Type:** pattern

Event-sourced aggregates are functors from Event category to State category.

**Structure:**
```
Event Category → State Category
  Objects: Event types
  Morphisms: Event causation
        ↓ Functor F
  Objects: Aggregate states
  Morphisms: State transitions
```

**Implementation Pattern:**
```rust
impl EventSourcedAggregate for Person {
    type Event = PersonEvent;

    // Functor: map event to state transition
    fn apply_event(&self, event: &PersonEvent) -> Result<Self, DomainError> {
        match event {
            PersonEvent::Created(e) => {
                // Initial object in target category
                Ok(Person {
                    id: e.person_id,
                    version: 1,
                    // ... initialize from event
                })
            }
            PersonEvent::Hired(e) => {
                // State transition morphism
                Ok(Person {
                    version: self.version + 1,
                    employment_status: EmploymentStatus::Employed,
                    ..self.clone()
                })
            }
            // ... other events
        }
    }
}
```

**Functor Laws:**
1. **Identity:** Applying identity event produces same state
2. **Composition:** Applying events sequentially = applying composed event

**Verification Requirements:**
- `apply_event` must be pure (no side effects)
- State transitions must be deterministic
- Event causation must be preserved

**Example from cim-domain-person:**
```rust
// Person aggregate is a functor
// F: PersonEvent → PersonState
// Identity: PersonIdentity event
// Composition: Causation chains via causation_id
```

**Related:** def-functor-001, cim-pattern-001, pattern-event-sourcing-001

---

## Test Template: Functor Purity

**ID:** test-functor-purity-001
**Tags:** test, purity, side-effects
**Type:** test_template

Verify functor operations are pure (no side effects).

**Implementation:**
```rust
#[tokio::test]
async fn test_functor_purity() {
    let functor = MyFunctor::new();
    let obj = /* test object */;

    // Call twice with same input
    let result1 = functor.map_object(obj.clone()).await.unwrap();
    let result2 = functor.map_object(obj.clone()).await.unwrap();

    // Must produce identical results (purity check)
    assert_eq!(result1, result2, "Functor must be pure - same input should produce same output");
}

#[tokio::test]
async fn test_morphism_purity() {
    let functor = MyFunctor::new();
    let morph = /* test morphism */;

    // Call twice with same input
    let result1 = functor.map_morphism(morph.clone()).await.unwrap();
    let result2 = functor.map_morphism(morph.clone()).await.unwrap();

    // Must produce identical results
    assert_eq!(result1, result2, "Morphism mapping must be pure");
}
```

**Why This Matters:**
- Side effects break composition law
- Non-determinism makes verification impossible
- Pure functions are required for mathematical reasoning

**Common Failures:**
- Database logging in map_object
- Random ID generation
- External API calls
- Mutable state

**Related:** violation-functor-001, pattern-functor-001

---

## Test Template: Property-Based Functor Laws

**ID:** test-proptest-functors-001
**Tags:** test, property-based, proptest
**Type:** test_template

Use property-based testing to verify functor laws hold for all inputs.

**Implementation:**
```rust
use proptest::prelude::*;

proptest! {
    #[test]
    fn functor_preserves_identity(obj in arbitrary_domain_object()) {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let functor = MyFunctor::new();
            let identity = DomainMorphism::identity(functor.source_category());

            // F(id) should be identity in target
            let mapped = functor.map_morphism(identity).await.unwrap();
            prop_assert!(mapped.is_identity());
        });
    }

    #[test]
    fn functor_preserves_composition(
        f in arbitrary_morphism(),
        g in arbitrary_morphism_compatible_with(&f)
    ) {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let functor = MyFunctor::new();

            // F(g ∘ f)
            let composed = compose_morphisms(f.clone(), g.clone());
            let left = functor.map_morphism(composed).await.unwrap();

            // F(g) ∘ F(f)
            let mapped_f = functor.map_morphism(f).await.unwrap();
            let mapped_g = functor.map_morphism(g).await.unwrap();
            let right = compose_morphisms(mapped_f, mapped_g);

            prop_assert_eq!(left, right);
        });
    }
}
```

**Benefits:**
- Tests thousands of random inputs automatically
- Finds edge cases human testing might miss
- Provides high confidence in correctness

**Related:** pattern-functor-001, test-functor-purity-001
