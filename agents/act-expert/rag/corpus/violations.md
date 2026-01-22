# Common Categorical Violations

<!-- Copyright (c) 2025 - Cowboy AI, LLC. -->

## Violation: Side Effects in Functor

**ID:** violation-functor-001
**Tags:** functor, side-effect, purity, critical
**Type:** violation
**Severity:** Critical

**Problem:** Functor operations contain side effects, breaking composition law and making behavior non-deterministic.

**Example:**
```rust
// ❌ VIOLATION
impl DomainFunctor for PersonToEmployee {
    async fn map_object(&self, obj: DomainObject) -> Result<DomainObject, DomainError> {
        // Side effect: database logging
        log_to_database(&obj).await;

        // Side effect: external API call
        notify_external_system(&obj).await;

        // Transformation (correct part)
        Ok(transform(obj))
    }
}
```

**Why It's Wrong:**
1. **Breaks Composition Law:** F(g ∘ f) ≠ F(g) ∘ F(f) when side effect ordering matters
2. **Non-Deterministic:** Different results on repeated calls
3. **Cannot Reason Mathematically:** Side effects invalidate categorical proofs
4. **Testing Fails:** Composition tests will fail due to side effect duplication

**Detection:**
- Composition tests fail
- Function produces different results on repeated calls with same input
- Logs show unexpected ordering or duplication

**Fix:**
```rust
// ✓ CORRECT
impl DomainFunctor for PersonToEmployee {
    async fn map_object(&self, obj: DomainObject) -> Result<DomainObject, DomainError> {
        // Pure transformation only - no side effects
        Ok(transform(obj))
    }
}

// Separate layer for side effects
async fn logged_map_object(
    functor: &impl DomainFunctor,
    obj: DomainObject
) -> Result<DomainObject, DomainError> {
    let result = functor.map_object(obj).await?;

    // Side effects isolated to outer layer
    log_to_database(&result).await;
    notify_external_system(&result).await;

    Ok(result)
}
```

**Test:**
```rust
#[tokio::test]
async fn test_functor_purity() {
    let functor = PersonToEmployee::new();
    let obj = test_object();

    // Call twice - should get identical results
    let result1 = functor.map_object(obj.clone()).await.unwrap();
    let result2 = functor.map_object(obj.clone()).await.unwrap();

    assert_eq!(result1, result2, "Functor must be pure");
}
```

**Related:** pattern-functor-001, def-functor-001, fix-purity-001

---

## Violation: Identity Not Preserved

**ID:** violation-functor-002
**Tags:** functor, identity, law-violation
**Type:** violation
**Severity:** Critical

**Problem:** Functor does not map identity morphisms to identity morphisms, violating the identity law F(id) = id.

**Example:**
```rust
// ❌ VIOLATION
impl DomainFunctor for BrokenFunctor {
    async fn map_morphism(&self, morph: DomainMorphism) -> Result<DomainMorphism, DomainError> {
        if morph.is_identity() {
            // WRONG: Adding extra transformation to identity
            return Ok(DomainMorphism::new(
                self.target_category(),
                "ExtraTransform", // Should be identity!
                morph.object_id()
            ));
        }

        // ... other morphisms
    }
}
```

**Why It's Wrong:**
- Identity law requires F(id_A) = id_F(A)
- Breaking this invalidates all functor reasoning
- Composition with identity will produce unexpected results

**Detection:**
```rust
#[tokio::test]
async fn test_identity_law() {
    let functor = BrokenFunctor::new();
    let identity = DomainMorphism::identity("SourceCategory");

    let mapped = functor.map_morphism(identity).await.unwrap();

    // This test will FAIL
    assert!(mapped.is_identity(), "F(id) must be identity");
}
```

**Fix:**
```rust
// ✓ CORRECT
impl DomainFunctor for CorrectFunctor {
    async fn map_morphism(&self, morph: DomainMorphism) -> Result<DomainMorphism, DomainError> {
        if morph.is_identity() {
            // Identity must map to identity in target category
            return Ok(DomainMorphism::identity(self.target_category()));
        }

        // ... other morphisms
    }
}
```

**Related:** law-identity-001, def-functor-001, fix-identity-001

---

## Violation: Composition Not Preserved

**ID:** violation-functor-003
**Tags:** functor, composition, law-violation
**Type:** violation
**Severity:** Critical

**Problem:** Functor does not preserve composition: F(g ∘ f) ≠ F(g) ∘ F(f)

**Example:**
```rust
// ❌ VIOLATION
impl DomainFunctor for BrokenComposition {
    async fn map_morphism(&self, morph: DomainMorphism) -> Result<DomainMorphism, DomainError> {
        match morph.event_type() {
            "PersonCreated" => {
                // Maps to single event
                Ok(create_employee_event())
            }
            "PersonHired" => {
                // Maps to single event
                Ok(update_employee_event())
            }
            "ComposedEvent" => {
                // WRONG: Handles composition specially instead of composing mappings
                // F(g ∘ f) computed differently than F(g) ∘ F(f)
                Ok(special_combined_event())
            }
        }
    }
}
```

**Why It's Wrong:**
- Violates fundamental functor law
- Breaks categorical reasoning
- Results depend on whether events are composed before or after mapping

**Detection:**
```rust
#[tokio::test]
async fn test_composition_law() {
    let functor = BrokenComposition::new();
    let f = person_created_event();
    let g = person_hired_event();

    // Left: F(g ∘ f)
    let composed = compose_events(f.clone(), g.clone());
    let left = functor.map_morphism(composed).await.unwrap();

    // Right: F(g) ∘ F(f)
    let mapped_f = functor.map_morphism(f).await.unwrap();
    let mapped_g = functor.map_morphism(g).await.unwrap();
    let right = compose_events(mapped_f, mapped_g);

    // This test will FAIL
    assert_eq!(left, right, "Must preserve composition");
}
```

**Fix:**
```rust
// ✓ CORRECT
impl DomainFunctor for CorrectComposition {
    async fn map_morphism(&self, morph: DomainMorphism) -> Result<DomainMorphism, DomainError> {
        // Handle each event type uniformly
        // Composition is automatically preserved
        match morph.event_type() {
            "PersonCreated" => Ok(create_employee_event()),
            "PersonHired" => Ok(update_employee_event()),
            // Composed events are handled by composition of mappings
            _ => Err(DomainError::UnknownEventType)
        }
    }
}
```

**Related:** law-composition-001, def-functor-001, fix-composition-001

---

## Violation: Naturality Square Does Not Commute

**ID:** violation-natural-transformation-001
**Tags:** natural-transformation, naturality, square
**Type:** violation
**Severity:** Critical

**Problem:** Natural transformation does not satisfy naturality condition: α_Y ∘ F(f) ≠ G(f) ∘ α_X

**Example:**
```rust
// ❌ VIOLATION
impl NaturalTransformation for BrokenTransformation {
    type SourceFunctor = FunctorF;
    type TargetFunctor = FunctorG;

    async fn transform_object(&self, obj: DomainObject) -> Result<DomainObject, DomainError> {
        // Transformation depends on object's state in non-uniform way
        if obj.has_property("special") {
            Ok(special_transformation(obj))  // Different path!
        } else {
            Ok(normal_transformation(obj))
        }
    }
}
```

**Why It's Wrong:**
- Natural transformations must be "uniform" across all objects
- Different treatment of different objects breaks naturality
- Violates fundamental categorical property

**Naturality Square:**
```
F(X) --F(f)--> F(Y)
  |             |
 α_X           α_Y
  |             |
  v             v
G(X) --G(f)--> G(Y)
```

Both paths F(X) → G(Y) must be equal.

**Detection:**
```rust
#[tokio::test]
async fn test_naturality_condition() {
    let alpha = BrokenTransformation::new();
    let f = FunctorF::new();
    let g = FunctorG::new();
    let morph = test_morphism();

    // Test naturality square
    let passed = alpha.verify_naturality(&f, &g, &morph).await.unwrap();

    // This test will FAIL
    assert!(passed, "Naturality square must commute");
}
```

**Fix:**
```rust
// ✓ CORRECT
impl NaturalTransformation for CorrectTransformation {
    type SourceFunctor = FunctorF;
    type TargetFunctor = FunctorG;

    async fn transform_object(&self, obj: DomainObject) -> Result<DomainObject, DomainError> {
        // Uniform transformation for all objects
        Ok(uniform_transformation(obj))
    }
}
```

**Related:** def-natural-transformation-001, def-naturality-condition-001, fix-naturality-001

---

## Violation: Anti-Corruption Layer Leaks

**ID:** violation-acl-001
**Tags:** anti-corruption, validation, domain-protection
**Type:** violation
**Severity:** High

**Problem:** Anti-corruption layer allows invalid external objects to corrupt internal domain.

**Example:**
```rust
// ❌ VIOLATION
impl DomainFunctor for LeakyACL {
    async fn map_object(&self, obj: DomainObject) -> Result<DomainObject, DomainError> {
        // WRONG: No validation of external object
        let internal = transform_to_internal(obj);

        // WRONG: No validation of internal invariants
        Ok(internal)  // May contain invalid state!
    }
}
```

**Why It's Wrong:**
- External systems may provide invalid data
- Domain invariants not enforced
- Corrupted state propagates through system

**Fix:**
```rust
// ✓ CORRECT
impl DomainFunctor for SecureACL {
    async fn map_object(&self, obj: DomainObject) -> Result<DomainObject, DomainError> {
        // 1. Validate external object
        validate_external(&obj)?;

        // 2. Transform to internal model
        let internal = transform_to_internal(obj);

        // 3. Validate internal invariants
        validate_internal(&internal)?;

        // 4. Only return if all validations pass
        Ok(internal)
    }
}
```

**Test:**
```rust
#[tokio::test]
async fn test_acl_rejects_invalid() {
    let acl = SecureACL::new();
    let invalid_obj = create_invalid_external_object();

    // ACL must reject invalid objects
    let result = acl.map_object(invalid_obj).await;
    assert!(result.is_err(), "ACL must reject invalid objects");
}

#[tokio::test]
async fn test_acl_validates_internal() {
    let acl = SecureACL::new();
    let obj = create_external_object_that_violates_internal_invariants();

    // ACL must enforce internal invariants
    let result = acl.map_object(obj).await;
    assert!(result.is_err(), "ACL must enforce internal invariants");
}
```

**Related:** pattern-anti-corruption-001, fix-acl-validation-001

---

## Violation: Context Mapping Loses Information

**ID:** violation-context-mapping-001
**Tags:** context-mapping, information-loss, reversibility
**Type:** violation
**Severity:** Medium

**Problem:** Context mapping functor loses critical information needed for downstream processing.

**Example:**
```rust
// ❌ VIOLATION - Information loss
impl DomainFunctor for LossyMapping {
    async fn map_object(&self, obj: DomainObject) -> Result<DomainObject, DomainError> {
        // Drops important properties
        Ok(DomainObject::new(
            obj.id(),
            // WRONG: Only maps name, loses all other properties
            vec![("name", obj.get("name"))],
        ))
    }
}
```

**Why It May Be Wrong:**
- Downstream context may need dropped information
- Round-trip Person → Employee → Person loses data
- May violate business requirements

**When It's Acceptable:**
- Forgetful functors intentionally drop structure
- Read models for public API (privacy)
- CQRS projections (optimization)

**Fix (when information must be preserved):**
```rust
// ✓ CORRECT - Preserve all necessary information
impl DomainFunctor for LosslessMapping {
    async fn map_object(&self, obj: DomainObject) -> Result<DomainObject, DomainError> {
        Ok(DomainObject::new(
            obj.id(),
            vec![
                ("name", obj.get("name")),
                ("email", obj.get("email")),
                ("created_at", obj.get("created_at")),
                // Preserve all properties needed downstream
            ],
        ))
    }
}
```

**Alternative Fix (when forgetful is intended):**
```rust
// ✓ CORRECT - Explicitly document information loss
pub struct ForgetfulFunctor {
    properties_to_keep: Vec<String>,
}

impl DomainFunctor for ForgetfulFunctor {
    async fn map_object(&self, obj: DomainObject) -> Result<DomainObject, DomainError> {
        // Explicitly configured which properties to keep
        let filtered: Vec<_> = obj.properties()
            .filter(|(k, _)| self.properties_to_keep.contains(k))
            .collect();

        Ok(DomainObject::new(obj.id(), filtered))
    }
}
```

**Related:** pattern-forgetful-functor-001, pattern-context-mapping-001

---

## Violation: Non-Deterministic Mapping

**ID:** violation-functor-004
**Tags:** functor, determinism, randomness
**Type:** violation
**Severity:** Critical

**Problem:** Functor mapping depends on randomness, time, or external state, making it non-deterministic.

**Example:**
```rust
// ❌ VIOLATION
impl DomainFunctor for NonDeterministicFunctor {
    async fn map_object(&self, obj: DomainObject) -> Result<DomainObject, DomainError> {
        // WRONG: Random ID generation
        let new_id = Uuid::new_v4();  // Different every time!

        // WRONG: Timestamp dependency
        let timestamp = Utc::now();  // Different every time!

        // WRONG: External state
        let count = self.call_counter.fetch_add(1, Ordering::SeqCst);  // Mutates!

        Ok(DomainObject::new(
            new_id,
            vec![
                ("timestamp", timestamp.to_string()),
                ("count", count.to_string()),
            ],
        ))
    }
}
```

**Why It's Wrong:**
- Same input produces different outputs
- Composition law cannot be verified
- Testing is impossible (results are random)
- Mathematical reasoning breaks down

**Detection:**
```rust
#[tokio::test]
async fn test_determinism() {
    let functor = NonDeterministicFunctor::new();
    let obj = test_object();

    let result1 = functor.map_object(obj.clone()).await.unwrap();
    let result2 = functor.map_object(obj.clone()).await.unwrap();

    // This test will FAIL
    assert_eq!(result1, result2, "Must be deterministic");
}
```

**Fix:**
```rust
// ✓ CORRECT - Deterministic mapping
impl DomainFunctor for DeterministicFunctor {
    async fn map_object(&self, obj: DomainObject) -> Result<DomainObject, DomainError> {
        // Derive ID deterministically from input
        let new_id = derive_id_from_object(&obj);

        // Use input properties only
        Ok(DomainObject::new(
            new_id,
            vec![
                ("name", obj.get("name")),
                ("type", "Employee"),
            ],
        ))
    }
}
```

**Alternative: Pure Function with External ID Generation:**
```rust
// ✓ CORRECT - ID generation moved outside functor
async fn map_with_id_generation(
    functor: &impl DomainFunctor,
    obj: DomainObject,
    id_generator: &impl IdGenerator,
) -> Result<DomainObject, DomainError> {
    // Generate ID outside pure functor
    let new_id = id_generator.generate();

    // Inject ID into object before mapping
    let obj_with_id = obj.with_id(new_id);

    // Functor remains pure
    functor.map_object(obj_with_id).await
}
```

**Related:** violation-functor-001, fix-purity-001

---

## Violation: Async Mutation in map_object

**ID:** violation-functor-005
**Tags:** functor, mutation, async, concurrency
**Type:** violation
**Severity:** High

**Problem:** Concurrent calls to `map_object` mutate shared state, causing race conditions.

**Example:**
```rust
// ❌ VIOLATION
pub struct MutableFunctor {
    cache: Arc<Mutex<HashMap<String, DomainObject>>>,  // Shared mutable state!
}

impl DomainFunctor for MutableFunctor {
    async fn map_object(&self, obj: DomainObject) -> Result<DomainObject, DomainError> {
        let mut cache = self.cache.lock().unwrap();

        // WRONG: Mutation breaks purity
        cache.insert(obj.id().to_string(), obj.clone());

        // Output depends on cache state (non-deterministic)
        let result = if cache.len() > 100 {
            transform_with_caching(obj)
        } else {
            transform_without_caching(obj)
        };

        Ok(result)
    }
}
```

**Why It's Wrong:**
- Race conditions in concurrent scenarios
- Non-deterministic (depends on call order)
- Violates purity requirement
- Testing is unreliable

**Detection:**
```rust
#[tokio::test]
async fn test_concurrent_purity() {
    let functor = Arc::new(MutableFunctor::new());
    let obj = test_object();

    // Spawn 100 concurrent calls
    let handles: Vec<_> = (0..100)
        .map(|_| {
            let f = functor.clone();
            let o = obj.clone();
            tokio::spawn(async move {
                f.map_object(o).await.unwrap()
            })
        })
        .collect();

    let results: Vec<_> = futures::future::join_all(handles)
        .await
        .into_iter()
        .map(|r| r.unwrap())
        .collect();

    // All results should be identical (will FAIL with mutation)
    for result in &results[1..] {
        assert_eq!(&results[0], result, "Concurrent calls must produce same result");
    }
}
```

**Fix:**
```rust
// ✓ CORRECT - Immutable, stateless functor
pub struct ImmutableFunctor {
    config: FunctorConfig,  // Immutable configuration only
}

impl DomainFunctor for ImmutableFunctor {
    async fn map_object(&self, obj: DomainObject) -> Result<DomainObject, DomainError> {
        // Pure transformation using only input and immutable config
        Ok(transform(obj, &self.config))
    }
}
```

**Alternative: Caching in Separate Layer:**
```rust
// ✓ CORRECT - Caching wrapper around pure functor
pub struct CachedFunctor<F: DomainFunctor> {
    inner: F,
    cache: Arc<DashMap<String, DomainObject>>,  // Thread-safe cache
}

impl<F: DomainFunctor> CachedFunctor<F> {
    async fn map_object_cached(&self, obj: DomainObject) -> Result<DomainObject, DomainError> {
        let key = obj.id().to_string();

        // Check cache
        if let Some(cached) = self.cache.get(&key) {
            return Ok(cached.clone());
        }

        // Call pure functor
        let result = self.inner.map_object(obj).await?;

        // Update cache
        self.cache.insert(key, result.clone());

        Ok(result)
    }
}
```

**Related:** violation-functor-001, fix-purity-001, pattern-functor-001
