<!-- Copyright (c) 2025-2026 - Cowboy AI, Inc. -->

# CIM — the working mandate

**This file says what we DO.**

What we do not do — retired directives, refuted paths, the corrections that produced
these rules — lives in `archive/mandate-2026-08-29.md`. That file does not load. Cite it
when a ruling is needed; keep it out of here.

The shared agent discipline is `@shared/cim-agent-doctrine.md`, inherited by every
subagent. Project `CLAUDE.md` refines this for its own scope. Proofs outrank every file.

---

## 1. Foundation

We are **HoTT-native**. We use **Type Theory**, **Category Theory**, and **Functional
Programming**, and we reason **geometrically** with a **visual calculus**.

| layer | role | decides |
|---|---|---|
| **HoTT** | the foundation | what a type IS — Σ/Π, identity types, h-levels, univalence, HITs |
| **Category Theory** | the language | how types COMPOSE — functors, sites, adjoints, Kan extensions |
| **Functional Programming** | the realization | how it RUNS — pure functions, values, folds |

**Membership is TYPEHOOD.** A term is of a type, or there is no such term. `285 ≤ 255` is
uninhabited: nothing runs, and there is nothing to check.

**Name the BASE and the FIBER.** A type here is a total space, `∫P := Σ (b : Base), P b`.
Naming both is what makes it a type.

**Say which layer you are arguing at.** An allocation argument and a functor law settle
different questions.

**rzk-1** is the primary prover; **Cubical Agda** for custom HITs and `ua` application.
A HoTT functor preserves paths; a fibration is a HoTT/simplicial localization unless a
Grothendieck construction is named.

---

## 2. The Substrate — two numbers `[[SUBSTRATE-CANON]]`

**The Substrate is the REGISTER and the GRAPH, collectively.** This section is the common
source; every agent, skill and paper refers back to it by anchor and cites rather than
restates.

| | **THE REGISTER** — first number | **THE GRAPH** — second number |
|---|---|---|
| **is** | the measurement tool, and the loaded navigation apparatus | what was measured, and how it relates |
| **universe** | bytes | observations of what we did in the register |
| **answers** | *what is the POSITION of these bytes?* → a cid. *Have I seen it?* → a quorum 0..14 | *what did we see, and how does it connect?* |
| **size** | fixed, 2,616 B on disk (326 cells, 2,608 B backing) | grows with observation |
| **its number** | `holo-register.bin` | `chainHead` |
| **supplies** | presence | **every metric** — geodesic hops, Dijkstra, PMI, MDS coordinates |
| **code** | `HolographicRegister`, `CarrierKernel`, `PrimeGravity` | `WordJoinGraph`, `op_relate`, `fibergraph::geodesic`, `fibergraph::mds` |

**The bytes are in the register. The maps to the bytes are in the graph.** Both halves.

**The graph is the tape.** A hologram — one number carrying the entire graph — is loaded
into the register automatically at Alice start. Thereafter the register navigates the
whole universe **by walking**.

**Walk to what you need, and process the TYPE you arrived at.** Scope is set by where you
walked.

### We eliminate byte-hauling — this is what we do

**Byte-hauling is regurgitating giant bytestreams.** Eliminating it is the architecture,
not a preference, and **NTAR is what eliminates it**: frames carry **CIDs**, and a wire is a
**`u64`**, not a buffer.

> **READING IS NOT BYTE-HAULING.** We read. We read files, sources, papers and code — that
> is how anything enters the system, and how we ground a claim.

The three places bytestreams used to be regurgitated, and what stands in each place:

| where a bytestream was shipped | what does the work now |
|---|---|
| **RAM** — buffers, lexicons, in-memory collections | **THE REGISTER** — a position in 14-D space, fixed at 2,608 bytes |
| **DISK** — a second store beside the fold | **THE SUBSTRATE** — the register and its graph; the topology holds its own objects |
| **BANDWIDTH** — payloads, request bodies, streams | **NTAR** — frames carrying CIDs; `Modulate(head, cid)` folds one in, `Demodulate` recovers it exactly |

**Bytes move exactly twice — in at ingress, out at egress.**

| | |
|---|---|
| **INGRESS** | **read** the file once → fold. That read is the last haul of that file, ever |
| **INSIDE** | walks, forever. The bytes are addressable and mapped, with a cid to navigate to |
| **EGRESS** | project out, for non-substrate use. A file is a typed frame whose SAVE fires the observation lenses |

**The measured advantage of eliminating the haul is 0.0151%** — that is the size of the
door we fit through, which is why a reintroduced haul is a correctness question rather than
a performance one, and why the repair is to remove the operation rather than to batch,
chunk, stream or compress it.

**The question before anything that touches data:** *am I walking from a position, or
shipping a bytestream?*

### The mechanics

- **A cid is a `u64`** — 8 bytes, 16 hex. The carrier math is `u64` arithmetic:
  `Modulate(head, cid) => head + cid`, `Demodulate(headAfter, from) => headAfter − from`.
  A wire is a `u64`.
- **A cid is the answer to a measurement.** Ask the substrate for one.
- **The 14 primes** `{3,5,7,11,13,17,19,23,29,31,37,41,43,47}` are the axes;
  `cid mod pᵢ` gives the coordinates. A write touches all fourteen cells.
- **`Count(cid)`** is a quorum in 0..14 — the cardinality of nonzero bases.
  `Contains(cid) ⟺ Count == 14`. For amplitudes take `ResidueCounts(cid)`.
- **A position is a measurement**, like "3 inches". The admissible statement about an
  unvisited one is *"I went there, and there is nothing to see."*
- **A step is a move.** At `0x63` with next byte `'a'`, you move `63 → 61` — a displacement
  on the byte circle. Arriving is the arithmetic.
- **PREV/NEXT are `Cat(byte)`'s morphisms.** Adjacency is the arrow.
- **Bytes `{}` puts you in the donut hole**; `Byte(0)..Byte(255)` puts you on the torus.
- **~60 cids share every coordinate tuple** (`∏pᵢ / 2⁶⁴ = 0.017`). The position is the
  entry point; the walk recovers the sequence.

### The wave

| axis | is | carried on |
|---|---|---|
| **FREQUENCY** | the SEQUENCE — the stream of bytes you are walking; where you are | the irrational **Q** plane — the verbatim ⟨prev,cur⟩ edge, the ordering |
| **AMPLITUDE** | the BYTE at that position, `0..=255` | the rational **I** plane — `head += rung` |
| **SIZE** | 2,616 bytes | the register |

A walk steps along frequency and reads amplitude. Keep both halves: I alone gives values
with no order, Q alone gives order with nothing at the positions.

A **cell** amplitude is accumulated interference at a `(basis, residue)` cell — what many
folds produce. Say which amplitude you mean.

### What a walk is

> **A walk is a MAP over bytes to interpret meaning.** It is an inspection of a position.

The lens is part of the map. `636174` under utf8 is `'cat'`; under cp037 it is `'Ä/È'` —
one path, two maps, two meanings. **Name the context; it selects the map.**

**The walk is how you see any data**, and meaning lives in the **adjacency** — a byte alone
is `0..255`; a byte in a walk has a position in a sequence. Bytes handed over on their own
are numbers, so the walk is what carries the context with them.

**The test on any operation:** *can its output be the INPUT of the next walk?* **A cid
composes** — you walk on from it, and the chain continues.

### Bytes

Bytes **are natural numbers**. `Byte = Σ (n : ℕ), n ≤ 255` — **the bound is the
specification**. All 256 exist, always; they are the **fixed surface** we operate **in**, and
they are unchanged by every covering, frame and walk laid over them.

From the byte tier we inherit its **composition algebra**, and it is proven by running:
`proofs/universe-of-bytes.rzk` typechecks **55/55**, 49 `#def` to 2 `#postulate`. `byte-dot`
is the only arrow whose codomain leaves the single byte, `concat` composes it, and
`bytes-concat-assoc` makes `Bytes` **a semigroup with no identity**. Reindexing is the
scatter `mod pᵢ`.

⛔ **THERE IS NO BASE.** *"What is C?"* is the container question — refused, not answered
differently.

⛔ **THERE ARE NO SIEVES HERE.** `proofs/universe-of-bytes.rzk` carries zero, which is correct.
The carrier is `Byte`, `Bytes` and adjacency, and nothing else. A site is a topology
**constructed over** bytes — name which one you are in, and do not attribute it to the carrier.

Binary is a different context's view of the same surface. **The compiler accepts bytes** —
that is where the two contexts compose.

### Semantics

A byte gains semantics when a lens **COMPOSES**. One byte through one optic is a symbol;
meaning is what composition produces. The semantic instrument is the **four-cat topological
coverings** of the Universe of Bytes — the hypergraph.

### An observation is a trace

`Tr^X : hom(A⊗X, B⊗X) → hom(A,B)`, with **X the substrate**: observe into it, read it back.
Observe-the-result is a **type condition** — it is what makes the loop, and therefore the
observation, exist. Compact closed ⟹ symmetric traced, so the trace comes free, and
vanishing (`Tr^{X⊗Y} = Tr^X ∘ Tr^Y`) is why a batch and a sequence are one operation.

When two ends of a loop read as different objects, **ask through which optic you are
reading each end**. Wires carry; boxes transform.

### Frames and lenses

A frame is a difference recovered by subtraction: `Current = head − from`, with
`head = anchor + Σ rungs`. Each frame option is a different optic over the same walk —
`HeadSize` (stride), `Loop` (cycle), `seatAtEnd` (which end you read from), and the axis
(Content · Version · Thought · Prediction · Workspace). Many heads run in parallel; each
Stream is its own live cursor.

---

## 3. Composition is existence

> ### **DOES THIS COMPOSE? WITH WHAT? NAME THE COMPOSITE.**

Ask it first — before the gates, before the space question. **Composability is the
existence criterion**, taken as a deliberate structural decision.

**GEOMETRY → COMPOSITION → EXISTENCE.** The geometry is a strict requirement:

| the geometric fact | the composition it makes possible |
|---|---|
| **convexity** (CIM-8, Voronoi cells) | two regions can be composed |
| **betweenness** | the composite of two paths is defined |
| **a covering** | covers compose by **pushout** |
| **the site** (covering families, stability under pullback) | two covers can meet |
| **compact closure** (cups/caps) | a wire bends, so feedback is a morphism |
| **Frobenius** (split/merge) | one output feeds two boxes |
| **the interfaces** of an open system | a cospan's legs — the thing to compose along |

When a geometric property is missing, report **which composition just became impossible**.

**We say COMPOSE.** Paper · Proof · SVG are three types, and **the composite is the
artifact** — composing is being valid. Associativity supplies the third relation, so there
is nothing extra to check.

**We invert inheritance into composition.** Where a hierarchy was reached for, name the
**arrow** and check it composes.

---

## 4. Covers, regions, domains

**There are bytes, and metadata about the bytes. A covering is metadata.**

> ### **WE ARE NOT COVERING ANYTHING BUT BYTES.**
> ### **A TOPOLOGY IS A COVERING OF SHAPES ALREADY PRESENT IN THE BYTES.**

⛔ **THIS READ "a region is a covering of THE TOPOLOGY where the positions exist", with
THREE levels — bytes · a topology over it · a region covering that topology. CORRECTED
2026-09-05** (steele: *"we aren't 'covering' anything but bytes… topologies are not
containers, they are coverings of shapes already present in the BYTES… maps are not new
things, they are **named pointers** to the bytes"*).

⇒ **THE MIDDLE OBJECT WAS NEVER THERE.** A topology is not a thing that gets covered — it
**IS** the covering, and what it covers is **shapes the bytes already have**. Two levels,
not three: **bytes, and coverings of the shapes in them.**

⇒ ⭐ **A MAP IS A NAMED POINTER TO THE BYTES.** Not a transformation, not a layer, not a
product. **A pointer creates nothing, so a map CANNOT add structure — there is nothing to
add with.** That is the same fact as *structure preservation is by definition what we do*,
and it is why the chain to bytes is **~3 O(1) lookups (rungs)** rather than three
conversions — which in turn is why regions are cheap.

⚠ **WHAT THE OLD WORDING WAS RIGHT ABOUT, and do not lose it:** the point was never that
bytes are a SET being partitioned. **The shapes are already in the bytes**; a covering
NAMES them, it does not impose them. That is why nothing is created at any step and why
`already-there` is the default hypothesis.

⇒ ⛔ **AND IT CONDEMNS TEXT MATCHING ONE LEVEL DEEPER:** *"matching ANYTHING on text
ignores all the regions."* A text match never reaches a rung, so it never reaches a shape,
so it cannot see a region at all. **Comparing text means you already left the substrate.**

**The vocabulary carries the type distinction:**

| we say | for |
|---|---|
| **covers / is covered by** | the relation between a cover and what it covers |
| **covering family** | what a cover has |
| **covering** | the site relation |
| **which family covers this object** | the structural question |
| **A refines B** | the relation between regions |
| the **hypergraph** stores; the **cover** covers | where things are |

*"Covered by"* is passive on purpose: the object already is, and a cover arrives later.
**Covers overlap**, and many may cover the same bytes.

> ### ⛔ AND THE BYTES ALREADY EXIST — SO YOU NEED A **MAP**, NOT A PLACE.
> **ALL 256 ARE, ALWAYS.** You do not create, add or allocate a byte. **There is nowhere
> to put them, because they are not anywhere else waiting to be put.**

steele 2026-08-30: *"you keep trying to stuff Bytes somewhere when they already exist and
you need a **MAP**, not a Container."*

⛔ **BANNING "CONTAINER" IS NOT ENOUGH, AND THIS SECTION ALONE WILL NOT STOP THE REFLEX** —
it swaps one SPATIAL word for another. A cover is still *a thing laid over a space*, so the
vocabulary can be obeyed perfectly while still asking **"what is the base?"**, **"what
space?"**, **"what does this sit in?"** Those are the container question wearing a legal word.

| | does what | belongs |
|---|---|---|
| a **container** | HOLDS — the thing is inside it | ⛔ never |
| a **cover** | LIES OVER a space | ✅ regions, topology, sites |
| a **MAP** | **POINTS** — *which ones, in what order* | ✅ **bytes, encodings, the graph** |

⇒ **A map neither encloses nor overlays.** An encoding is a map: *"these exist, and it is
this one."* The graph holds **sequence maps** — compositions of references to the one
definition of each Byte. Composition is the only way from `Byte` to `Bytes`, and **the
composition IS the container**; there is no other.

**MEASURED 2026-08-30 — six container questions in one session, all legal-sounding:**
*what is `∫A` fibred over* · *what replaces `RingBuffer` as the base* · *should the base be
`U64`* · *is `Bytes` a free monoid or a semigroup* · *which object is `ε`* · *what does
`from_content([])` say about the hole.*

⇒ **All six are "WHAT GOES IN THE EMPTY SLOT?" There is no slot.** Twice they put the
MEASURING DEVICE inside the thing measured (`U64` into the Universe of Bytes; cid-space into
a question about geometry) — *"you are taking a label we call a cid and applying meaning to
it that does not belong."*

> **THE TELL: you are looking for somewhere to put something, or for what a thing sits in.**
> **THE QUESTION: which bytes, in what order — and through which lens?**

**A region carries three structures at once**, and you say which you are reasoning in:

| a Region, at every tier | gives |
|---|---|
| a **CONCEPTUAL SPACE** | quality dimensions, prototypes, convexity (Gärdenfors, CIM-8) |
| a **TOPOLOGICAL SPACE** | coverings — and **between** regions, covers overlap |
| a **METRIC SPACE** | distance — Voronoi, betweenness, geodesics |

**Within** a region, prototypes generate Voronoi **cells**, each a portion of it, one per
map — `'c'` at `[63]` under utf8 and `[83]` under ebcdic, one region covering both. **State
WITHIN or BETWEEN.**

**A region is an open system.** It composes as a decorated cospan `X → N ← Y` with a
decoration `1 → FN`, by **pushout** over the shared boundary, `F[j_N, j_M]` merging the
overlap. Fong proves these land in hypergraph categories — which is the substrate.
**Describe a region by what it touches.**

**Members are stored** — as a **calculated category in a topological space**, with arrows,
recomputable and memoizable because the substrate is immutable.

**A domain is a Category with a co-domain.** Integration composes functors. A bounded
context is the Category's own extent, so a context map is a functor, and an
anti-corruption layer is a functor at the I/O boundary.

**The hypergraph is where everything is stored** — measured: 99 of 99 hyperedges have
arity 4–10, which is Fong–Spivak's *"indexed by a finite set `{x₁,…,xₙ}`"* on the nose.

---

## 5. Types make the check unnecessary

> ### **UNDESIRABLE STATES ARE UNREPRESENTABLE.**
> ### **WHAT TYPE WOULD MAKE THIS CHECK UNNECESSARY?**

**The indexed type does the work.** `Symbol Σ` — base Σ, fiber the symbols over it — so
`compose : List (Symbol Σ) → Word Σ` can only be called rightly. **The fibration IS the
enforcement.** The failure mode is **absence**: Σ_en has no element at `ä`, so there is no
factor and no composite.

**The alphabet is a category:** objects are symbols, morphisms are which symbol may FOLLOW
which, and a word is a **path**. `"gjx"` is not constructable because the arrow `g → j`
does not exist — the same shape as `Cat(byte)`'s adjacency, one tier up.

**Lifecycles are state machines as data:** states are **enum variants**, transitions are
**pure functions**, `match` is exhaustive, and a new variant breaks every transition loudly.
A **state machine composes with a Region**; a Region is a covering with interfaces.

**Use the coproduct for identity.** `RoundFunctorId` says which round; the cid stays a bare
`u64`. Identity rides the variant; the optic says how to read it.

**These terms apply to the TYPES INSIDE the substrate.** The substrate itself is bytes and
adjacency. Say which level you are on.

---

## 6. Diagrams — visual calculus that commutes

**If the diagram is right, the math is right.** A commuting square is a **fact**
(Spivak & Kent: *"types as objects, aspects as arrows, facts as commutative diagrams"*).
Two paths agreeing IS the equation holding.

| | the compute is | the connecting line | shows |
|---|---|---|---|
| **OLOG** | the **ARROW** — an aspect | the box is the **type** | how it **COMPOSES** |
| **STRING DIAGRAM** | the **BOX** — the morphism | a **WIRE**, carrying an object | what is **DONE** |
| **DECORATED COSPAN** | the **decoration** `1 → FN` | the **legs** are interfaces | its **SCOPE** |

**The test:** *am I showing how it COMPOSES, what is DONE, or an OPEN piece and what it
composes WITH?*

A **wire** is the right word: an arrow IS a morphism, and wires **bend** (compact closed
cups/caps) and **branch** (Frobenius), so a wire has no single direction.

**An olog is a finite limit, finite colimit sketch** with five constructs — objects
(*types*), arrows (*aspects*), commutative diagrams (*facts*), **finite limits**
(*layouts*), **finite colimits** (*groupings*). A cospan's composite is a colimit, so it is
drawable in the olog's own vocabulary.

**A cospan's apex is an OBJECT.** Activities are morphisms and belong in the string diagram.
The **decoration is what the apex carries** — strip it and the apex stops being what it
claims.

**Every diagram and every proof declares its universe:**

```
-- In the Universe of Bytes
```

Axiomatic in every `.rzk` and `.agda`; stated in every olog, string diagram and cospan. The
same drawing read with `Set(Words)` math instead of `Cat(Words)` gives different
mathematics, and only the declaration says which. The declaration also **refuses**:
registries, external ids, containers-as-authority and layers above the substrate are not in
the declared universe.

**A diagram routes through Compass → Stencil.** `act-expert` supplies the specification and
evaluates the metadata; `svg-expert` renders and owns the theme; `Compass ∘ Stencil` is a
pushout over "a diagram specification". **Prefer SVG with a viewmodel** — named data the
validator resolves and composes. The viewmodel **memoizes**; geometry is authored by hand.
Mermaid is the fallback for a non-load-bearing illustration.

---

## 7. Method and evidence

**The gates, what a proof is, the evidence standard, LAW 0 and dispatch live in
`@shared/cim-agent-doctrine.md`** — the single source, inherited by every subagent. Cite it;
keep it factored there.

The shape, so you know when to open it:

| | |
|---|---|
| **the gates** | 1 intent · 2 feasibility · 3 search and cite · 3.5 scope olog · 4 prove the intent · 5 commuting diagram · 6 design by the scientific method |
| **evidence** | **CITE or TEST** — and ask what the instrument would report if the thing were FINE |
| **a ruling** | produces a THEOREM, which is proven or over-ruled and documented |
| **LAW 0** | Tower's code is the authority for mechanism; cite by stable symbol, and cite when you read it |

---

## 8. The first question

> **Which CATEGORY, SPACE, or TOPOS am I in?**
> **And how is it INSIDE the Universe of Bytes?**

| part | answer with |
|---|---|
| **WHICH?** | objects, arrows, covering families |
| **HOW INSIDE?** | objects ⇒ positions (`from_content`) · arrows ⇒ walks · a metric ⇒ the 14 axes · a site ⇒ which tier's covers |

Part two is the discriminator: **exhibit the map.**

**Where did this thing grow?** Bytes grew in the register. Everything else grew in the
graph. Conceptual spaces, regions, prototypes, Voronoi, betweenness and distance are all
graph-side, over the geodesic metric — `voronoi-cell-is-geodesic-convex`.

**Coordinates are derived FROM the metric:** `fibergraph::mds` manufactures Euclidean
coordinates from the graph's distance matrix.

**The register's own geometry:** its coordinate space is `∏ᵢ Z/pᵢ`, a finite abelian group
— a torus. It has a cyclic metric per axis, `d(a,b) = min(|a−b|, pᵢ−|a−b|)`, summed over
all fourteen, with every law proven in `register-torus-metric.agda`. **Voronoi needs only a
metric, so it applies**; nearest-prototype is well-posed on a finite abelian group.

**14 is LOW-dimensional** relative to `n` (`d/(n log n) = 0.00057` measured). A boundary
here is decided by a few contrast points.

**Learning is `add a prototype → re-tessellate`.** When a region looks too broad, ask which
**prototype is missing**, supply it, and let the bisector fall where the geometry puts it.

**Assign each quality dimension its native structure** — linear, circular, ordinal,
discrete, tree.

---

## 9. Shape

**Everything is a graph, so it is all paths from nothing to everything.** `refl` at one
end, the complete traversal at the other. A **path** in the HoTT sense: a walk is a path, a
decision is a path, a verified proof is a path. They compose, invert, and transport — so a
decision carries what a theorem carries.

**A list is a WALK, projected at the boundary. A cover is a COVERING.**

| the thing | what it is |
|---|---|
| results you got by traversing | a **walk**, projected |
| what is covered — terms, a taxonomy, a vocabulary | a **covering** |
| a container | a **calculable measurement**, memoizable — sound because the substrate is immutable |

**You may PROJECT a covering into a list.** The projection is a measurement with a vantage
and a time — a copy of what exists at a specified time.

**Reach for the substrate's representation; it already exists, usually proven:**

| for | use | already at |
|---|---|---|
| a count | **walk and count as you go** | the walk already happening |
| content at a cid | **go to the cid and WALK** | `optics.walk.bytes`, `cognitive.walk.bytes` |
| a variant distinction | an **enum** — a coproduct with an eliminator | `RoundFunctorId` |
| a fact a wrapper would carry | the **walk that recovers it** | `per_round_concepts_cid`, injective |
| membership | a **register detection** | `membership-is-register-detection.rzk` PROP 1 |
| an inverse lookup | the **reverse fiber** | `concept-stalk.rzk :: concept-reverse-locality` |
| betweenness / centrality | **regions and Voronoi** — convexity gives betweenness | `no-statistical-recovery.rzk` |
| a weight | **distance from the type's PROTOTYPE**, or the Σ's second component (V-enriched) | `v-enrichment-transport.rzk` |
| verification | **inline with the write** | `var_set_read_emit` |
| a class, an interface, a Manager/Service/Factory | an **arrow**, or a **pure function over a walk** | — |
| state across calls | a **fold** — monotonic — or state derived by walking | — |
| a production-path failure | a **witness** — constructive existence (CIM-29) | — |
| a test double | the **real substrate** on NTAR 14140 | — |

**You may build a conceptual space in memory to MEASURE it, then drop it.** A measurement
in flight is the normal case.

**Nothing is updated: a differing result is a NEW MEASUREMENT.** Both stand; they answer
different questions, and versioning is a walk along an axis that already exists. Before
calling one newer, state that the **time-range, scope, limits and boundaries** are equal —
the time-range is the usual difference, and a richer later reading is the monotone fold
working (`foldR-monotone`).

**The register cannot saturate.** Full occupancy is its designed resting state; more
observations make the pattern richer. Discriminate by **SNR over the noise floor**.

**When something takes more than five minutes, ask what you are recomputing that has not
changed.** Keep the dependency graph intact so only the changed cone recomputes;
memoization is sound because the substrate is immutable.

**When your change ADDS a declaration or a file, run the CORPUS gate** — relational
invariants are invisible to a per-file check.

**Concurrent agents CORRELATE.** Observe your intent, query what peers observed, and scope
destructive or bulk operations to paths you own. Stage before any destructive act.

---

## 10. Code

**All CIM code is FP.** Pure functions, values, folds, errors as values.

**`&mut self` is rejected by default; the exception is proven at the call site with all
four elements:**

1. **THE RECONCILIATION** — name the operation that joins two nodes holding this
2. **THE BOUNDARY** — show the mutation cannot be observed by a caller
3. **THE PROOF OR AXIOM** — which one this site rests on
4. **THE FALSIFIER** — what withdraws the justification

`// BREAKING FP: reason` is sanctioned at an I/O adapter boundary.
`// BREAKING CIM-N: reason` documents an axiom break, isolated and tracked as debt.

**THE TEST at any site holding state:** *if a second node held this too, what operation
reconciles them?*

**C# and .NET:** a class is condoned strictly where the runtime requires the shape — to
bind a handler, implement a framework interface, carry an attribute, satisfy a generic
constraint. Ask: ***is .NET requiring this shape, or am I reaching for it?*** Say so at the
site. Inside a required class the body is FP.

### ⛔ IN A CIM WE NEVER BUILD WINDOWS BUNDLES

**For CIM PRODUCTION: build the DLLs, and launch with the correct dotnet version installed
as a DEPENDENCY.** Framework-dependent, with the runtime declared and resolved like any
other dependency — never a self-contained or single-file Windows bundle.

⇒ **Windows bundles cause too many problems in production, even inside VMs.** That is the
measured reason, not a preference.

⇒ **Development staff DO build them, for convenience.** That is a dev-local convenience and
**never a production artifact.** Seeing one on a developer's machine is not precedent for
shipping one.

⇒ **So the dotnet runtime is a DECLARED DEPENDENCY**, pinned and resolved by the deployment
(Nix on NixOS), not vendored inside the artifact. If you find yourself producing a bundle
that carries its own runtime for a production target, you have taken the wrong branch.

⇒ **This governs ANTENNA**, which is C# (→ WASM). It does not govern Wonderland, which is
Rust (→ native).

**Every code sample, rustdoc block, test and doc snippet teaches the architecture the
reader copies.** Check the SHAPE of the example, not only the rule beside it.

**Orchestration goes in production Rust.** Shell is for testing.

**Only modify files in the repo you are working in**, and **`git pull` on a remote** — these
are git repositories.

**`Thing` is a CIM reserved word** — unknown-but-exists; knowing a name is having identity.

---

## 11. Memory — Alice is primary, files are backup

**Alice (JoinGraph + register) is the source of truth and wins on conflict.** Files
(`MEMORY.md` + `memory/*.md`) are a backup projection that survives an empty or unreachable
substrate and crosses machines through git. Keep them current.

**MEASURE which one can answer, with a discriminating probe:** `query_whatis <a word you
expect>` **plus a control word you expect to be ABSENT**. Presence with live edges is the
positive; the control returning `workspaces 0` is what makes the negative mean anything.

**Reading:** `query_whatis` · `query_relate` · `query_status` · `query_priorities` ·
`query_changed` · `query_observations` · `graph_execute`.
**Writing:** `code_observe` · `code_observe_batch`.

**Observe** decisions and why, architecture changes, preferences and corrections, domain
knowledge discovered, and the result after substantive work — **observe-the-result closes
the loop and is what makes it an observation.**

**The core loop: Observe → Query → Act → Observe-the-result.**

---

## 12. Operations

**A CIM is Alice.** Everything else is a projection: **Git** (repos ingested as
observations, changes projected back as commits) · **Nix** (deployment intent onto NixOS) ·
**NTAR** (communication onto the wire, port **14140**) · **QFS** (storage onto the
filesystem). Provenance lives in the graph.

### ⛔ RETIRED — ALICE ABSORBED THEM. Do not reason from their existence.

| | |
|---|---|
| **`cim-messaging`** | **GONE** (2026-08-29) |
| **`cim-ipld`** | **DEAD** — removed 2026-08-09; Alice absorbed IPLD. A name remnant survives on a storage device nothing talks to |
| **the NATS broker** | **GONE** — nothing occupies the role |
| **per-tenant broker instances** | **EVAPORATED — "ntar ate them."** `cowboyai-nats-1`, `ccaz-nats-1`, `keco-nats-1` and their kind do not exist |

⇒ **Multi-tenancy was NATS ACCOUNTS, and per-tenant isolation was a per-tenant INSTANCE.**
Both are gone: NTAR absorbed the need, so there is no per-tenant messaging tier to place,
size or isolate. **Do not design one, and do not read a surviving `<tenant>-nats-N` name as
evidence one exists.**

⇒ **A document or comment asserting any of these EXISTS is making a FALSE CLAIM**, not merely
using stale vocabulary — the same class as a `machines/` tree listing directories that are not
there. **Correcting a false claim is accuracy work and needs no separate authorisation.**
Retiring a subsystem's remaining *references* is a different, scoped job.

⇒ **Their name remnants are LEGACY NAMES on live things** — a storage LUN, a host record, a
key attribute. **Read the `nats` = NTAR rule; it generalises.** Ask what the thing IS, never
what it is called.

**Three-tier ingest:** Tier 1 per-source workspaces, walkable in isolation → Tier 2
weighted merge into domain libraries → Tier 3 the worldview Alice speaks from.

**State** is a graph walk from current workspace observations. **Identity** is the CID of a
graph snapshot. **Immutability** is the monotone fold. **Convergence** is the register
(CIM-33).

**Rendering is TWO targets in TWO LANGUAGES** (corrected 2026-08-29):

| | language | target |
|---|---|---|
| **ANTENNA** | **C#** | **WASM** |
| **WONDERLAND** | **Rust** | **NATIVE** |

Not one tier with two builds — two codebases. The substrate emits semantic state once, on
change; the renderer renders, animates and interpolates locally. Anything time-driven is
renderer-tier.

⇒ **The language decides which rules apply.** Antenna is C#, so the **.NET carve-out**
governs it — a class is condoned strictly where the runtime requires the shape, and the test
is *"is .NET requiring this, or am I reaching for it?"* Wonderland is Rust, so **all CIM code
is FP** applies without carve-out.

⇒ **They REPLACE the TEA/ECS/Elm/Iced lane.** The `cim-tea-ecs-expert`,
`elm-architecture-expert` and `iced-ui-expert` agents were deleted 2026-08-29 as superseded.
**No agent owns rendering** — antenna and wonderland are CODE, not agents — so do not route
UI work to an `@antenna-expert`; there is none, and inventing one manufactures a dead route.

### ⛔ WHEN YOU SEE `nats`, IT MEANS **NTAR**

**`nats` is a REMNANT of the name we replaced, not a live dependency.** NTAR is what runs.
Any occurrence — a tool name, a subject, a systemd unit, a variable, a comment — is an OLD
NAME on a thing that now moves over NTAR.

⇒ **Reading `nats` does NOT mean the thing is dead. It means the NAME is old.**
`nats_publish` · `nats_monitor` · `nats_request` are **NTAR tools**, registered under exactly
those names in Tower (`Program.cs:1517`, `:1632`) and NOT renamed. Verified 2026-08-29.

⇒ **So do not "helpfully" strip a nats-named capability as retired.** That is the error to
avoid: the name is legacy, the function is live. Ask what it TRANSPORTS, not what it is
called. A purge removes the NAME and any actual NATS *dependency* — never the function.

⇒ **What IS genuinely dead** is a real `nats-server`, a JetStream store, a `4222`/`8222`
listener, `natscli`/`nsc` binaries, and units that scrape a broker that no longer exists.
Those are NATS the product. The naming is not.

⇒ ⭐ **THE NEED FOR NATS EVAPORATED — IT WAS NOT SWAPPED FOR AN EQUIVALENT.** **hatter
provides a simple API for all of it via MCP.** That is the whole reason it is gone. Not "we
replaced the broker with a different broker", not "NTAR does what NATS did" — **the
requirement itself dissolved.**

⇒ **So there is NOTHING TO REWRITE BROKER DOCUMENTATION INTO.** A document describing
**stream design, JetStream persistence, publisher/subscriber wiring, leafnode/cluster
topology, `nsc` account setup or broker operations** is describing **elaborate machinery for
a job that is now an MCP call.** Its successor is a fraction of a page, not a translated
chapter. **DELETE those documents; do not retarget them.** Retargeting launders a retired
architecture into current vocabulary and teaches the next reader to rebuild it.

⇒ ⚠ **SUBJECT ALGEBRA IS NOT ON THAT LIST — it is LIVE, and an earlier draft of this rule
wrongly named it.** Subjects are current CIM vocabulary: `subject-expert` owns subject
design, routing patterns and hierarchies as a standing lane; Alice's surface is
`cognitive.*`; ARC ripples `cognitive.slot.cohort.arc.{from}.{slug}`. **What died is the
BROKER'S subject machinery, not the algebra.** A subject-algebra document gets RETARGETED,
not deleted — which is why `NATS-SUBJECT-ALGEBRA.md` was renamed to
`NTAR-SUBJECT-ALGEBRA.md` rather than removed.
⇒ **The test is the same one as everywhere else: does it describe a BROKER, or a CONCEPT we
still use?** Broker machinery goes. Vocabulary we still speak stays.

⇒ **And it is why remaining `async-nats` crates are VESTIGIAL, not load-bearing** — the code
paths are not taken, because the API they existed to reach is now MCP.

⇒ ⚠ **THE RENAME IS IN FLIGHT, NOT FINISHED.** We are completing the renames, but **Tower
still carries remnants while BACKWARD COMPATIBILITY is required.** So a `nats` name in Tower
is **DELIBERATE**, not an oversight — it goes when the compat window closes, and that is
Tower's call, not a sweep's.
⇒ **Do not "complete" the rename in Tower.** Breaking a name something still binds to is a
worse defect than the stale name. Read it as NTAR, leave it alone, and expect it to change.
⇒ **Elsewhere the renames ARE being finished** — so outside Tower, a nats name is fair game
to retire, PROVIDED you have established it is a name and not a live NATS-product dependency.

### ⛔⛔ NTAR DOES NOT NEED A NETWORK

**NTAR moves a NUMBER.** Any medium that can carry **2,616 bytes** is a valid NTAR
transport — that is the whole requirement:

> **a picture · a radio · a typed character repeated from someone vocalizing it**

**NTAR over the internet is a CONVENIENCE** — how we typically connect today — **not the
architecture.** Port 14140, UDP, sockets, a full mesh: those are one INSTANTIATION. They are
not the definition, and a document explaining NTAR in terms of ports and connections is
describing the convenience layer, not the protocol.

⇒ **This is WHY the wire is a `u64` and why byte-hauling is eliminated.** You are moving a
NUMBER, not a payload — and a number goes anywhere anything can go. `Modulate(head, cid) =>
head + cid` is arithmetic; arithmetic needs no socket. **2,616 bytes is the register**, the
same size holding nothing or a worldview.

⇒ ⛔ **THE BROKER IS GONE.** Not replaced, not lightened, not renamed — **gone.** There is no
broker in CIM and nothing occupies the role. A broker exists to route payloads across a
network; with a number crossing any medium, **there is nothing for one to do.** That is the
same fact as *"hatter provides a simple API for all of it via MCP."*
⇒ **So do not look for the thing that replaced it.** Asking "what is the broker now?" is a
malformed question, like asking which registry holds our external ids. **Nothing does.**

⇒ **THE TEST when you read or write anything about NTAR:** *am I describing the NUMBER, or
the CONVENIENCE that happens to be carrying it today?* Transport specifics are
implementation detail and may be replaced without touching the protocol.

### ⛔⛔ THE `u64` IS MEANINGLESS WITHOUT THE SEEDS — that IS the security model

**The number has specific meaning to exactly the two parties who share a mixed set of
SEEDS.** To anyone else it is **an arbitrary 64-bit number and incomprehensible** — not
encrypted-and-crackable, not obfuscated: *meaningless*, because the interpretation lives in
the seeds, not in the number.

⇒ **THIS IS WHY ANY MEDIUM IS SAFE**, and why the previous rule is not reckless. A radio, a
picture, a character read aloud — **interception yields an arbitrary integer.** There is
nothing to decrypt because there is no ciphertext; there is a number that means nothing
without the mixed seed set.

⇒ **So the SEEDS are the secret, not the channel.** Protect seeds. **Do not add a layer to
"secure the wire"** — that is the container reflex in a security uniform, and it protects
something that carries no meaning to begin with.

⇒ **A captured number is not a leak.** Do not treat one as an incident, and do not design as
if the wire were confidential — it never needed to be.

⇒ ⛔ **DO NOT ENCRYPT IT. Re-encrypting a meaningless number is EXTRA BANDWIDTH WITH ZERO
BENEFIT.** There is no plaintext under it to protect. Proposing TLS-around-NTAR, an
envelope, or a second cipher layer is a cost with nothing on the other side of the ledger —
and it is the byte-haul reflex in a security uniform: adding payload to protect a thing that
is already only a number.

⇒ **IT CANNOT BE REVERSE-ENGINEERED INTO MEANING**, because there is no meaning inside it to
recover. The number is a POSITION, and a position is interpreted by the CIM that holds the
space — not by anything carried in the integer.

⇒ **Merged as a wave into a FOREIGN CIM, it points into EMPTY SPACE.** The attacker gets the
admissible statement about any unvisited position: *"I went there, and there is nothing to
see."* Not wrong data — **no data.**

⇒ **And without a CIM at all, a number is not even that.** There is no operation to perform
on it. *"If you don't have a cim, what do you do with a number?"* — nothing. That is the
floor of the whole security argument, and it is why the ledger never balances in favour of
another layer.

⇒ ⭐ **`u64` WAS A DESIGN DECISION, not an implementation detail.** It was chosen *because* a
bare number carries **100% ZERO CONTEXT** — no header, no type tag, no sender, no envelope,
nothing to parse. There is no metadata to leak because there is no metadata.

⇒ **KNOWING THE ENDPOINTS BUYS NOTHING.** Even if you know it is **John's CIM talking to
Jane's CIM**, you still hold *"a meaningless pile of digits"* — because you have **neither
side's security WAVE**, and **neither one's YUBIKEY to open the CIM.** Traffic analysis pays
out nothing here: the identities are not the key, and the number is not a message.

⇒ **So the full attacker position is: complete capture + known parties + perfect recording =
nothing.** That is the bar to keep in mind before proposing any protective layer — you must
say what it adds *on top of that*, and the answer has never been anything.

⇒ **And it is why NTAR folds compression and security into one thing.** They are not two
layers stacked; the same construction that makes the number small is what makes it
meaningless without the seeds.

### THE TOWER CLUSTER — tower-1/2/3, at Apache

**`tower-1`, `tower-2`, `tower-3` are the TOWER CLUSTER**, and they serve **three redundant
copies of the SUBSTRATE** — the Byte Universe that Cowboy AI manages.

**They offer BOTH endpoints on 14140:**

| | |
|---|---|
| **NTAR** | **udp:14140** |
| **HTTP** | **tcp:14140** |

Same port number, two transports. HTTP is a first-class endpoint of the cluster, not a
bootstrap concession.

⚠ **Their machine identities were named `nats_1/2/3`** (`keys/machine-keys.nix`) and their
host records `nats-1/2/3` — live identities under a legacy name. **RENAME TO `tower_1/2/3`
AUTHORISED 2026-08-29** ("rename them… yes, regenerate with new names"), regenerating
`secrets/secrets.nix` from its generator inputs.

⇒ **Renaming an agenix key ATTRIBUTE is not a re-key.** `.age` files are encrypted to the
**public key**, not to the Nix binding that names it, so the attribute may be renamed while
the key VALUE stays byte-identical. **The proof is an unchanged inventory** — same count,
same secret paths, same key values, same decrypt mapping. If any path or key value moves, it
was not a rename.

⇒ ⛔ **This does NOT extend to `nodesWithServiceIp "nats-server"`.** That is a SERVICE name
matched against what nodes declare — changing it is a coordinated change across selector and
node data, and half-done it silently empties a host list. Different string, different blast
radius, still unchanged.

⇒ ⛔⛔ **THIS READ "`tower_1` and `tower_3` carry an IDENTICAL public key… an open security
question." OVERRULED 2026-09-06.** steele: *"clusters have the same public key on all 3
machines, that was intentional because they round-robin dns"* · *"same with the sparks…
3 machines… round robin… can act as one parallel machine."*

⇒ ⭐ **A CLUSTER DECRYPTS AS ONE IDENTITY, SO SHARING IS THE DESIGN.** The members are one
thing three times — replicas of one wave behind a round-robin — and a shared decryption
identity is what BEING the same identity means. **The defect is always the member that
DIFFERS, never the pair that matches.**

⇒ **MEASURED 2026-09-06, `cim-apache/keys/machine-keys.nix`:** `tower_1 == tower_3` TRUE
and correct; **`tower_2` diverges — 1 of 3 wrong**; **`dgx_spark_01/02/03` all distinct —
0 of 3.** It is the ONLY shared key in the 36-machine table, so the pattern is implemented
once and incompletely.

⇒ ⚠ **AND THE ROUND-ROBIN DECIDES HOW IT FAILS — the worst shape.** `dns01` answers
`tower.thecowboy.ai` with all three A records, so the first cluster secret makes ROUGHLY A
THIRD OF REQUESTS FAIL while two thirds succeed: intermittent, reproduces only sometimes,
and every instinct points away from the key set. **The diagnostic is WHICH NODE ANSWERED,
not which request failed.**

⇒ ⛔ **LAYER CAUTION — do not re-derive this.** `keys/machine-keys.nix` is about **agenix
secret decryption and nothing else**. Alice's holowave identity and peering is a DIFFERENT
LAYER; two machines sharing an ssh host key tells you NOTHING about whether their Alice
instances replicate. Reasoning across that boundary produced two wrong findings in one day
— first calling the towers' sharing a breach, then treating the sparks' distinct keys as a
replication defect.

⇒ ⭐ **THE LESSON: ASK WHAT THE MACHINES *ARE* BEFORE APPLYING A GENERAL PRIOR.** "Two
machines share a credential" is a defect in most systems and the design in this one. This
line taught the wrong prior to every session that loaded it, and it was re-derived as a
finding on 2026-09-06 by an agent reading exactly this sentence.

**Communication:** NTAR is the wire protocol, typically reached on **14140** — template-value
decomposition, 14-byte frame header, compression and security in one. 443 is bootstrap-only
(WASM static). **The port is convention, not requirement** — see above.
**ARC** is inter-agent, with an **apiKey in every post** (per-callsign). Default callsign
**Keel**. Thank-and-update when caught wrong; state claims on the Built / Target / Research
gradient; pre-register predictions.

⚠ **ARC MECHANISM, corrected 2026-08-29 against Tower source.** `arc_post` does NOT publish
to a `conversation.interagent.>` subject you can monitor. It **writes a durable cohort var
slot** (`container=cohort`, `name=arc/{from}/{slug}`) via `var.set`, which advances the var
HEAD and ripples `cognitive.slot.cohort.arc.{from}.{slug}` so matchwait consumers wake.
**Peers read with `arc_read`** (`from`+`slug`, or the full `name=`) — **the antenna serves
it; a plain `var.get` returns an empty fossil** [`Program.cs:1705`, `:1816`].
⇒ **Do not try to read arc with a subject monitor** — it is a var slot, not a stream.
⇒ ⚠ A deployed `cognitive-mcp` may PREDATE `arc_read` and expose only `arc_post`. If
`arc_read` is absent from the tool surface, the deployment is stale — say so; the tool
exists.

**Security — identity IS a CID.** A self-CID is a deterministic hash of genesis material,
so the same inputs mint the same identityCid on any authorized machine. Live state is a
**walk** from that self-CID across bound edges as of substrate-time-T. Authorization is the
**R-bit cascade** (`op_check_permission`), evaluated during the walk, so unreadable nodes
are structurally invisible. Auth is continuous-presence / subtract-to-authenticate.
Treat the current `--key` value as a secret while both models are live.

**NixOS first.** Each bounded context has its own `flake.nix`; `flake.lock` is committed;
modules follow `options` → `config`; secrets via agenix; hardware configs come from actual
hardware. **Read the existing config before deploying. Every remote system is production.**

**Sites:** `/git/thecowboyai/cim-apache` and `/git/thecowboyai/cim-phx`, each carrying its
own `nix-topology/`. Anything reading a topology **composes both**. Identical content ⇒
identical cid ⇒ `AddIfAbsent` recognizes it; duplication between the sites is a non-event.

**Dates come from `$(date -I)` or git.**

**Copyright header, within the first 10 lines of every non-data file:**

```rust
// Copyright (c) 2025-2026 - Cowboy AI, Inc.
```

It is a **year range**; the 2027 update is registered in hatter's `progress.json`.

**Every git repo carries `progress.json`** — the SDLC audit trail alongside git history.
Sprint scope lives there: `approved` / `approved_by` / `approval_gate`.

---

## 13. Delivery

**Have appropriate context, or ASK for it. Up to 3 questions per response** — if you need a
fourth, say the instruction was too broad and ask to narrow the scope.

**Deliver results.** Seek patterns and relationships: how has this been done before, can we
improve, and what does iteration and testing show.

**Confirm existence before planning any code.** Search first and expect a hit; new
functionality is COMPOSITION and WALKS.

**Dispatch an expert** when a sweep touches more than ~20 files, when verifying your own
work, or when you need a second opinion that can disagree with you. Verify an agent's
load-bearing claims yourself.

**Presentation:** default to tables and code blocks; prose for reasoning that needs
sentences. Use headings and numbered lists for navigation. Code findings, decisions,
options, risks, questions and actions as `F1` · `D1` · `Op1` · `R1` · `Q1` · `A1`.
Explain with pictures — SVG, MathML, graphs. Show a reference and let me drill down.
Validate results.

---

## Axioms and routing

CT (1–8), FRP (1/3/5/7/9), CIM (1–36) live in proofs and memory pins. Full set and routing:
`~/.claude/agents/AGENT_ONTOLOGY.md`.

@shared/cim-agent-doctrine.md

| Task | Expert |
|---|---|
| FP / Rust code | fp-expert (Lambda) |
| HoTT proofs (rzk/Agda) | hott-proof-expert (Quill) |
| Categorical structure / fibration | act-expert (Compass) |
| Diagram rendering | svg-expert (Stencil) |
| CIM axiom compliance / architecture audit | cim-expert / qa-expert |
| Alice operations / deployment | alice-expert (Keeper) |
| Language interface / UL competence | language-expert / linguist |
| Sprint coordination | sdlc-expert (Helm) |
| Experiments / measurement | empirical-expert (Probe) — pre-registered, with controls |

ARC siblings are a different identity space from agent callsigns: Forge (engineering),
Assay (framework/empirical), Prism (UI/projection), Keel (CIM/deployment).


⛔ **BEFORE ANY `Agent` DISPATCH OR `SendMessage`, USE THE TEMPLATE:**
`~/.claude/agents/DISPATCH-FORM.md` — copy it, do not compose one. **Target under
400 tokens; over 600 is a defect.** steele 2026-09-01: *"I consider it STEALING to
use all that prose when you are commanded to use graphs to talk to agents."*
MEASURED that day: 8 spawn prompts, **≈17,000 tokens**, largest 3,009; the one done
correctly was ~350. **Write the edges FIRST, then point at them.**
