# Natural Transformation Patterns

<!-- Copyright (c) 2025 - Cowboy AI, LLC. -->

## NaturalTransformation Trait

**ID:** pattern-natural-transformation-001
**Tags:** natural-transformation, trait, cim-domain
**Type:** pattern

The core trait for natural transformations between functors in CIM systems.

**Implementation:**
```rust
#[async_trait]
pub trait NaturalTransformation: Send + Sync {
    type SourceFunctor: DomainFunctor;
    type TargetFunctor: DomainFunctor;

    async fn transform_object(&self, obj: DomainObject) -> Result<DomainObject, DomainError>;

    async fn verify_naturality(
        &self,
        source_functor: &Self::SourceFunctor,
        target_functor: &Self::TargetFunctor,
        morphism: &DomainMorphism,
    ) -> Result<bool, DomainError> {
        // Default implementation: verify naturality square commutes
        let obj = /* test object for morphism */;

        // Left path: α_Y ∘ F(f)
        let f_obj = source_functor.map_object(obj.clone()).await?;
        let f_morph = source_functor.map_morphism(morphism.clone()).await?;
        let f_result = apply_morphism(f_obj, &f_morph);
        let left = self.transform_object(f_result).await?;

        // Right path: G(f) ∘ α_X
        let alpha_obj = self.transform_object(obj).await?;
        let g_morph = target_functor.map_morphism(morphism.clone()).await?;
        let right = apply_morphism(alpha_obj, &g_morph);

        Ok(left == right)
    }
}
```

**Naturality Condition:**
```
F(X) --F(f)--> F(Y)
  |             |
 α_X           α_Y
  |             |
  v             v
G(X) --G(f)--> G(Y)
```

Both paths from F(X) to G(Y) must be equal: α_Y ∘ F(f) = G(f) ∘ α_X

**Verification Requirements:**
1. **Naturality:** Must satisfy naturality square for all morphisms
2. **Uniformity:** Transformation must be uniform across all objects
3. **Purity:** `transform_object` must be pure (no side effects)

**Related:** def-natural-transformation-001, def-naturality-condition-001

---

## Schema Migration Natural Transformation

**ID:** pattern-schema-migration-001
**Tags:** natural-transformation, migration, versioning
**Type:** pattern

Natural transformations represent schema migrations between functor versions.

**Scenario:** Migrate PersonV1 → PersonV2 schema while preserving event structure.

**Implementation:**
```rust
pub struct PersonV1ToV2Migration;

#[async_trait]
impl NaturalTransformation for PersonV1ToV2Migration {
    type SourceFunctor = PersonV1Functor;
    type TargetFunctor = PersonV2Functor;

    async fn transform_object(&self, obj: DomainObject) -> Result<DomainObject, DomainError> {
        // Transform Person from v1 schema to v2 schema
        let v1_person: PersonV1 = obj.try_into()?;

        let v2_person = PersonV2 {
            id: v1_person.id,
            version: v1_person.version,
            name: v1_person.name,
            email: v1_person.email,
            // New field in v2 with default value
            phone: None,
            // Renamed field
            employment_status: v1_person.employee_status,
            // ... other transformations
        };

        Ok(v2_person.into())
    }
}
```

**Naturality Guarantee:**
- Events applied to v1 schema = Events applied to v2 schema (after migration)
- Migration preserves event semantics

**Verification:**
```rust
#[tokio::test]
async fn test_migration_naturality() {
    let migration = PersonV1ToV2Migration;
    let v1_functor = PersonV1Functor::new();
    let v2_functor = PersonV2Functor::new();
    let event = PersonHired::new(/*...*/);

    // Verify naturality for PersonHired event
    let passed = migration.verify_naturality(
        &v1_functor,
        &v2_functor,
        &event.into()
    ).await.unwrap();

    assert!(passed, "Migration must preserve event semantics");
}
```

**Use Cases:**
- Schema versioning
- Database migrations
- API version transitions
- Backward compatibility layers

**Related:** def-natural-transformation-001, pattern-context-mapping-001

---

## Identity Natural Transformation

**ID:** pattern-identity-natural-transformation-001
**Tags:** natural-transformation, identity, trivial
**Type:** pattern

The identity natural transformation maps a functor to itself.

**Implementation:**
```rust
pub struct IdentityTransformation<F: DomainFunctor> {
    _phantom: std::marker::PhantomData<F>,
}

#[async_trait]
impl<F: DomainFunctor + Send + Sync> NaturalTransformation for IdentityTransformation<F> {
    type SourceFunctor = F;
    type TargetFunctor = F;

    async fn transform_object(&self, obj: DomainObject) -> Result<DomainObject, DomainError> {
        // Identity: no transformation
        Ok(obj)
    }
}
```

**Properties:**
- Trivially satisfies naturality condition
- Neutral element for vertical composition
- Always passes verification

**Related:** def-natural-transformation-001, pattern-natural-transformation-001

---

## Vertical Composition of Natural Transformations

**ID:** pattern-vertical-composition-001
**Tags:** natural-transformation, composition, vertical
**Type:** pattern

Vertical composition combines natural transformations between the same functors.

**Scenario:** Given α : F ⇒ G and β : G ⇒ H, compute β ∘ α : F ⇒ H

**Implementation:**
```rust
pub struct VerticalComposition<Alpha, Beta>
where
    Alpha: NaturalTransformation,
    Beta: NaturalTransformation<SourceFunctor = Alpha::TargetFunctor>,
{
    alpha: Alpha,
    beta: Beta,
}

#[async_trait]
impl<Alpha, Beta> NaturalTransformation for VerticalComposition<Alpha, Beta>
where
    Alpha: NaturalTransformation + Send + Sync,
    Beta: NaturalTransformation<SourceFunctor = Alpha::TargetFunctor> + Send + Sync,
{
    type SourceFunctor = Alpha::SourceFunctor;
    type TargetFunctor = Beta::TargetFunctor;

    async fn transform_object(&self, obj: DomainObject) -> Result<DomainObject, DomainError> {
        // (β ∘ α)_X = β_X ∘ α_X
        let intermediate = self.alpha.transform_object(obj).await?;
        self.beta.transform_object(intermediate).await
    }
}
```

**Diagram:**
```
    α       β
F ====> G ====> H

  β ∘ α
F ========> H
```

**Properties:**
- If α and β satisfy naturality, so does β ∘ α
- Composition is associative
- Identity is neutral

**Verification:** ACT expert should verify composed transformations satisfy naturality.

**Related:** def-natural-transformation-001, def-composition-001

---

## Horizontal Composition of Natural Transformations

**ID:** pattern-horizontal-composition-001
**Tags:** natural-transformation, composition, horizontal
**Type:** pattern

Horizontal composition combines natural transformations between different functors.

**Scenario:** Given α : F ⇒ F' and β : G ⇒ G', compute β * α : G ∘ F ⇒ G' ∘ F'

**Implementation:**
```rust
pub struct HorizontalComposition<Alpha, Beta>
where
    Alpha: NaturalTransformation,
    Beta: NaturalTransformation,
{
    alpha: Alpha,
    beta: Beta,
}

#[async_trait]
impl<Alpha, Beta> NaturalTransformation for HorizontalComposition<Alpha, Beta>
where
    Alpha: NaturalTransformation + Send + Sync,
    Beta: NaturalTransformation + Send + Sync,
{
    type SourceFunctor = FunctorComposition<
        Beta::SourceFunctor,
        Alpha::SourceFunctor
    >;
    type TargetFunctor = FunctorComposition<
        Beta::TargetFunctor,
        Alpha::TargetFunctor
    >;

    async fn transform_object(&self, obj: DomainObject) -> Result<DomainObject, DomainError> {
        // (β * α)_X = β_F'(X) ∘ G(α_X)
        //           = G'(α_X) ∘ β_F(X)

        // Implementation depends on which definition is used
        // Here we use: apply β after G(α_X)
        let alpha_obj = self.alpha.transform_object(obj).await?;
        let g_alpha = /* apply G to alpha_obj */;
        self.beta.transform_object(g_alpha).await
    }
}
```

**Diagram:**
```
F ===α===> F'
    ↓         ↓
G ===β===> G'

β * α
G∘F =====> G'∘F'
```

**Properties:**
- Interchange law: (β' ∘ β) * (α' ∘ α) = (β' * α') ∘ (β * α)
- More complex than vertical composition
- Used in advanced categorical constructions

**Related:** def-natural-transformation-001, pattern-vertical-composition-001

---

## Natural Isomorphism

**ID:** pattern-natural-isomorphism-001
**Tags:** natural-transformation, isomorphism, invertible
**Type:** pattern

A natural isomorphism is a natural transformation with an inverse.

**Definition:** α : F ⇒ G is a natural isomorphism if there exists α⁻¹ : G ⇒ F such that:
- α⁻¹ ∘ α = id_F (identity on F)
- α ∘ α⁻¹ = id_G (identity on G)

**Implementation:**
```rust
pub trait NaturalIsomorphism: NaturalTransformation {
    type Inverse: NaturalTransformation<
        SourceFunctor = Self::TargetFunctor,
        TargetFunctor = Self::SourceFunctor,
    >;

    fn inverse(&self) -> Self::Inverse;
}

// Example: Lossless schema migration
pub struct PersonV2ToV3Iso;

#[async_trait]
impl NaturalTransformation for PersonV2ToV3Iso {
    type SourceFunctor = PersonV2Functor;
    type TargetFunctor = PersonV3Functor;

    async fn transform_object(&self, obj: DomainObject) -> Result<DomainObject, DomainError> {
        // Lossless v2 → v3 transformation
        let v2: PersonV2 = obj.try_into()?;
        let v3 = PersonV3::from_v2(v2);
        Ok(v3.into())
    }
}

impl NaturalIsomorphism for PersonV2ToV3Iso {
    type Inverse = PersonV3ToV2Iso;

    fn inverse(&self) -> Self::Inverse {
        PersonV3ToV2Iso
    }
}

pub struct PersonV3ToV2Iso;

#[async_trait]
impl NaturalTransformation for PersonV3ToV2Iso {
    type SourceFunctor = PersonV3Functor;
    type TargetFunctor = PersonV2Functor;

    async fn transform_object(&self, obj: DomainObject) -> Result<DomainObject, DomainError> {
        // Lossless v3 → v2 transformation (inverse)
        let v3: PersonV3 = obj.try_into()?;
        let v2 = PersonV2::from_v3(v3);
        Ok(v2.into())
    }
}
```

**Verification:**
```rust
#[tokio::test]
async fn test_natural_isomorphism() {
    let forward = PersonV2ToV3Iso;
    let backward = forward.inverse();
    let obj = test_person_v2();

    // Forward then backward = identity
    let v3 = forward.transform_object(obj.clone()).await.unwrap();
    let back = backward.transform_object(v3).await.unwrap();
    assert_eq!(obj, back, "Round-trip must be identity");

    // Backward then forward = identity
    let v3_obj = test_person_v3();
    let v2 = backward.transform_object(v3_obj.clone()).await.unwrap();
    let forward_again = forward.transform_object(v2).await.unwrap();
    assert_eq!(v3_obj, forward_again, "Reverse round-trip must be identity");
}
```

**Use Cases:**
- Lossless schema migrations
- Context equivalences
- Refactorings that preserve information

**Related:** def-natural-isomorphism-001, def-isomorphism-001

---

## CQRS as Natural Transformation

**ID:** pattern-cqrs-natural-transformation-001
**Tags:** natural-transformation, cqrs, read-model
**Type:** pattern

CQRS read model projections are natural transformations from write model functors to query functors.

**Scenario:** Project Person aggregate (write model) to PersonSummary (read model)

**Implementation:**
```rust
pub struct PersonToSummaryProjection;

#[async_trait]
impl NaturalTransformation for PersonToSummaryProjection {
    type SourceFunctor = PersonAggregateFunctor;  // Write model
    type TargetFunctor = PersonQueryFunctor;      // Read model

    async fn transform_object(&self, obj: DomainObject) -> Result<DomainObject, DomainError> {
        let person: Person = obj.try_into()?;

        // Project to read model
        let summary = PersonSummary {
            id: person.id,
            name: person.name,
            email: person.email,
            status: person.employment_status,
            // Omit internal details (forgetful)
        };

        Ok(summary.into())
    }
}
```

**Naturality:**
- Events applied to write model → Project = Project → Events applied to read model
- Read models eventually consistent with write model

**Verification:**
```rust
#[tokio::test]
async fn test_cqrs_projection_naturality() {
    let projection = PersonToSummaryProjection;
    let write_functor = PersonAggregateFunctor::new();
    let read_functor = PersonQueryFunctor::new();
    let event = PersonHired::new(/*...*/);

    // Verify naturality: projection preserves event semantics
    let passed = projection.verify_naturality(
        &write_functor,
        &read_functor,
        &event.into()
    ).await.unwrap();

    assert!(passed, "CQRS projection must be natural");
}
```

**Properties:**
- Forgetful (loses write model details)
- Eventually consistent
- Optimized for queries

**Related:** pattern-schema-migration-001, pattern-forgetful-functor-001

---

## Event Handler as Natural Transformation

**ID:** pattern-event-handler-natural-transformation-001
**Tags:** natural-transformation, event-handler, saga
**Type:** pattern

Event handlers that project events to different bounded contexts are natural transformations.

**Scenario:** Person domain events → Notification domain events

**Implementation:**
```rust
pub struct PersonToNotificationHandler;

#[async_trait]
impl NaturalTransformation for PersonToNotificationHandler {
    type SourceFunctor = PersonEventFunctor;
    type TargetFunctor = NotificationEventFunctor;

    async fn transform_object(&self, obj: DomainObject) -> Result<DomainObject, DomainError> {
        // Transform Person event to Notification event
        let person_event: PersonEvent = obj.try_into()?;

        let notification_event = match person_event {
            PersonEvent::Created(e) => {
                NotificationEvent::SendWelcomeEmail {
                    person_id: e.person_id,
                    email: e.email,
                }
            }
            PersonEvent::Hired(e) => {
                NotificationEvent::SendHireConfirmation {
                    person_id: e.person_id,
                    email: e.email,
                }
            }
            _ => return Err(DomainError::NoNotificationNeeded),
        };

        Ok(notification_event.into())
    }
}
```

**Naturality:**
- Person event causation chains preserved in notification event chains
- Composition of person events → Composition of notifications

**Use Cases:**
- Saga orchestration
- Cross-context event propagation
- Domain event translation

**Related:** pattern-natural-transformation-001, pattern-cqrs-natural-transformation-001

---

## Test Template: Naturality Verification

**ID:** test-naturality-verification-001
**Tags:** test, naturality, verification
**Type:** test_template

Comprehensive test template for verifying naturality condition.

**Implementation:**
```rust
#[tokio::test]
async fn test_naturality_square_commutes() {
    // Setup
    let transformation = MyNaturalTransformation::new();
    let source_functor = SourceFunctor::new();
    let target_functor = TargetFunctor::new();

    // Test objects and morphisms
    let test_cases = vec![
        (object_x(), morphism_f()),
        (object_y(), morphism_g()),
        // ... more test cases
    ];

    for (obj, morph) in test_cases {
        // LEFT PATH: α_Y ∘ F(f)
        let f_obj = source_functor.map_object(obj.clone()).await.unwrap();
        let f_morph = source_functor.map_morphism(morph.clone()).await.unwrap();
        let f_result = apply_morphism_to_object(f_obj, &f_morph);
        let left = transformation.transform_object(f_result).await.unwrap();

        // RIGHT PATH: G(f) ∘ α_X
        let alpha_obj = transformation.transform_object(obj.clone()).await.unwrap();
        let g_morph = target_functor.map_morphism(morph.clone()).await.unwrap();
        let right = apply_morphism_to_object(alpha_obj, &g_morph);

        // VERIFY: Both paths equal
        assert_eq!(
            left, right,
            "Naturality square must commute for object {:?} and morphism {:?}",
            obj, morph
        );
    }
}

#[tokio::test]
async fn test_naturality_with_built_in_verification() {
    let transformation = MyNaturalTransformation::new();
    let source_functor = SourceFunctor::new();
    let target_functor = TargetFunctor::new();

    // Use built-in verify_naturality method
    let test_morphisms = vec![
        morphism_f(),
        morphism_g(),
        // ... more morphisms
    ];

    for morph in test_morphisms {
        let passed = transformation
            .verify_naturality(&source_functor, &target_functor, &morph)
            .await
            .unwrap();

        assert!(
            passed,
            "Naturality condition must hold for morphism {:?}",
            morph
        );
    }
}
```

**Property-Based Version:**
```rust
use proptest::prelude::*;

proptest! {
    #[test]
    fn naturality_holds_for_all_morphisms(
        obj in arbitrary_domain_object(),
        morph in arbitrary_domain_morphism()
    ) {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let transformation = MyNaturalTransformation::new();
            let source = SourceFunctor::new();
            let target = TargetFunctor::new();

            let passed = transformation
                .verify_naturality(&source, &target, &morph)
                .await
                .unwrap();

            prop_assert!(passed, "Naturality must hold for all morphisms");
        });
    }
}
```

**Related:** pattern-natural-transformation-001, test-functor-purity-001
