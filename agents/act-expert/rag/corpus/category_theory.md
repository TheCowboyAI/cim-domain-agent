# Category Theory Definitions

<!-- Copyright (c) 2025 - Cowboy AI, LLC. -->

## Category

**ID:** def-category-001
**Tags:** fundamental, category, structure
**Type:** definition

A category C consists of:
- **Objects:** ob(C) = {A, B, C, ...}
- **Morphisms:** hom(A,B) = arrows from A to B
- **Composition:** ∘ : hom(B,C) × hom(A,B) → hom(A,C)
- **Identity:** id_A : A → A for each object A

**Laws:**
1. **Associativity:** (f ∘ g) ∘ h = f ∘ (g ∘ h)
2. **Identity:** id_B ∘ f = f = f ∘ id_A

**CIM Application:** Categories represent domain boundaries in CIM systems. Each bounded context is a category where:
- Objects = Domain aggregates
- Morphisms = Domain events
- Composition = Event causation chains
- Identity = No-op events

**Example:**
```rust
// Category: Person domain
// Objects: Person aggregates in various states
// Morphisms: PersonEvent variants (Created, Updated, Hired, etc.)
// Composition: Event causation via causation_id
```

**Related:** def-functor-001, def-morphism-001, def-object-001

---

## Functor

**ID:** def-functor-001
**Tags:** fundamental, functor, mapping
**Type:** definition

A functor F : C → D between categories C and D consists of:
- **Object mapping:** F_obj : ob(C) → ob(D)
- **Morphism mapping:** F_mor : hom_C(A,B) → hom_D(F(A), F(B))

**Laws:**
1. **Identity Law:** F(id_A) = id_F(A)
2. **Composition Law:** F(g ∘ f) = F(g) ∘ F(f)

**Intuition:** Functors preserve structure. They map objects to objects and arrows to arrows while respecting composition and identity.

**CIM Application:** Functors map between bounded contexts while preserving event structure:
```rust
// Person → Employee functor
// Maps Person objects to Employee objects
// Maps PersonEvent morphisms to EmployeeEvent morphisms
// Preserves causation chains
```

**Why Laws Matter:**
- **Identity Law:** Ensures no-op events remain no-ops after mapping
- **Composition Law:** Ensures event chains remain consistent after mapping

**Verification:** ACT expert verifies these laws hold for all `DomainFunctor` implementations.

**Related:** def-category-001, pattern-functor-001, law-identity-001, law-composition-001

---

## Morphism

**ID:** def-morphism-001
**Tags:** fundamental, morphism, arrow
**Type:** definition

A morphism f : A → B is an arrow between objects A and B in a category.

**Properties:**
- **Domain:** dom(f) = A (source object)
- **Codomain:** cod(f) = B (target object)
- **Composable:** If cod(f) = dom(g), then g ∘ f : A → C exists

**Types of Morphisms:**
1. **Identity:** id_A : A → A (no-op)
2. **Isomorphism:** f : A → B with inverse f⁻¹ : B → A
3. **Endomorphism:** f : A → A (self-arrow)
4. **Epimorphism:** Right-cancellable (surjective-like)
5. **Monomorphism:** Left-cancellable (injective-like)

**CIM Application:** Morphisms are domain events in event-sourced aggregates:
```rust
// PersonCreated : InitialState → Person
// PersonHired : Person → Person (endomorphism)
// PersonArchived : Person → ArchivedPerson
```

**Related:** def-category-001, def-composition-001

---

## Natural Transformation

**ID:** def-natural-transformation-001
**Tags:** fundamental, natural-transformation, morphism
**Type:** definition

A natural transformation α : F ⇒ G between functors F, G : C → D is a family of morphisms:
- **Components:** α_X : F(X) → G(X) for each object X in C
- **Naturality Condition:** α_Y ∘ F(f) = G(f) ∘ α_X for each morphism f : X → Y

**Naturality Square:**
```
F(X) --F(f)--> F(Y)
  |             |
 α_X           α_Y
  |             |
  v             v
G(X) --G(f)--> G(Y)
```

The square commutes: both paths from F(X) to G(Y) are equal.

**Intuition:** Natural transformations are "uniform" mappings between functors - they work the same way at every object.

**CIM Application:** Natural transformations represent context-preserving migrations:
```rust
// α : PersonV1Functor ⇒ PersonV2Functor
// Migrates all Person objects from v1 schema to v2 schema
// Preserves event structure during migration
```

**Verification:** ACT expert verifies naturality squares commute for all morphisms.

**Related:** def-functor-001, def-naturality-condition-001

---

## Adjunction

**ID:** def-adjunction-001
**Tags:** advanced, adjunction, duality
**Type:** definition

An adjunction F ⊣ G between categories C and D consists of:
- **Left Adjoint:** F : C → D
- **Right Adjoint:** G : D → C
- **Unit:** η : Id_C ⇒ G ∘ F
- **Counit:** ε : F ∘ G ⇒ Id_D

**Triangle Identities:**
1. ε_F ∘ F(η) = id_F
2. G(ε) ∘ η_G = id_G

**Intuition:** Adjunctions capture "best approximation" relationships. F is the "free" construction, G is the "forgetful" functor.

**CIM Application:** CQRS command/query separation is an adjunction:
```rust
// F : Commands → State (write model)
// G : State → Queries (read model)
// η : Commands → Queries (eventual consistency)
// ε : Write → Read → Write (round-trip)
```

**Verification:** ACT expert can verify triangle identities for CQRS implementations.

**Related:** def-functor-001, def-natural-transformation-001, pattern-cqrs-adjunction-001

---

## Identity Morphism

**ID:** def-identity-001
**Tags:** fundamental, identity, morphism
**Type:** definition

For each object A in a category, there exists an identity morphism id_A : A → A such that:
- **Left Identity:** id_B ∘ f = f for any f : A → B
- **Right Identity:** f ∘ id_A = f for any f : A → B

**Properties:**
- Unique for each object
- Neutral element for composition
- Always exists

**CIM Application:** Identity events represent no-op state transitions:
```rust
impl DomainFunctor for MyFunctor {
    async fn map_morphism(&self, morph: DomainMorphism) -> Result<DomainMorphism, DomainError> {
        if morph.is_identity() {
            // Identity must map to identity
            return Ok(DomainMorphism::identity(self.target_category()));
        }
        // ... other morphisms
    }
}
```

**Verification:** ACT expert checks F(id_A) = id_F(A) for all DomainFunctor implementations.

**Related:** def-category-001, law-identity-001

---

## Composition

**ID:** def-composition-001
**Tags:** fundamental, composition, operation
**Type:** definition

Composition ∘ : hom(B,C) × hom(A,B) → hom(A,C) takes two compatible morphisms and produces their composite.

**Notation:** Given f : A → B and g : B → C, their composition is g ∘ f : A → C

**Laws:**
1. **Associativity:** (h ∘ g) ∘ f = h ∘ (g ∘ f)
2. **Identity:** id_B ∘ f = f = f ∘ id_A

**CIM Application:** Event causation chains:
```rust
// Event f: PersonCreated
// Event g: PersonHired
// Composition g ∘ f: Person created and then hired
// Tracked via causation_id field
```

**Verification:** ACT expert checks F(g ∘ f) = F(g) ∘ F(f) for all DomainFunctor implementations.

**Related:** def-morphism-001, law-composition-001

---

## Isomorphism

**ID:** def-isomorphism-001
**Tags:** advanced, isomorphism, equivalence
**Type:** definition

An isomorphism f : A → B is a morphism with an inverse f⁻¹ : B → A such that:
- f⁻¹ ∘ f = id_A
- f ∘ f⁻¹ = id_B

**Properties:**
- Objects A and B are "the same" up to isomorphism
- All structure is preserved in both directions
- Unique up to composition with isomorphisms

**CIM Application:** Lossless context mappings:
```rust
// PersonToEmployee is an isomorphism if:
// 1. EmployeeToPerson exists
// 2. Round-trip preserves all information
// 3. Person ≅ Employee (structurally equivalent)
```

**Verification:** ACT expert can verify isomorphism properties for context mappings.

**Related:** def-morphism-001, def-natural-isomorphism-001
