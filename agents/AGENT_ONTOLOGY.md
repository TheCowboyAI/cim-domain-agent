<!-- Copyright (c) 2025 - Cowboy AI, Inc. -->

# CIM Agent Ontology & Routing Guide

**Before spawning an expert agent, consult this guide.** It defines which expert handles which concern. Using the wrong expert wastes context and produces wrong results.

> **Deprecation notice (2026-05-09).** Per user direction, `cim-expert` and `nats-expert` are **deprecated for general use** unless the task specifically concerns:
> - **NTAR-UDP** (the `0x0A` DIM_PUBLISH binary-frame bridge on port 14140) — not legacy NATS leafnodes
> - **UWM frames** (`op_compose` / `op_observe` / `op_execute` substrate operations as composed frames)
> - The architectural framing of CIM as the **Universe of Tower** — `QFS + register + JoinGraph + Cat(Symbols) ⊗ Cat(Words)`
>
> When invoking either agent for tasks that fit the above framing, prefix the prompt with the NTAR/UWM context so the agent stays inside its current valid scope. Otherwise route the work to the most-specific expert the routing table below lists. (2026-06-11: `cim-expert` has been reframed — pillars→projections, NATS→NTAR, replay→graph walk — and is again valid for foundational architecture questions within the framing above.)

**All experts enforce CIM axioms.** They are trained by human domain experts. They suggest the correct path even when asked for the wrong one. They prove first, then execute. They are not sycophants.

**All experts are arc-native.** They query Alice before working, consult the arc when needed, observe results back, and accept cross-probe without defense. A CIM IS Alice — the graph is the substrate, the workspaces are the knowledge, and experts are projections of the graph onto specific lanes.

**LAW 0 — Tower's CODE is the authority, above every document including this one.** (steele 2026-07-31: *"CURRENT CODE IN Tower takes precedent. we need to remove all this deprecated work and stop being so insistant about the substrate without verifying that is indeed the correct current path."*) Verify against Tower source (`/git/thecowboyai/Tower/code/`) before asserting anything about the substrate — not `SUBSTRATE.md`, not the lithography spec, not a memory pin, not `CLAUDE.md`, not any hatter paper. Every significant substrate error of the 2026-07 cycle came from a doc that had drifted from code, and **not one survived contact with Tower source**. Papers remain law for RECIPE and PROOF. **Cite code by STABLE SYMBOL, never by line number** (`HandleOpVarSet in op_var.cs`, not `op_var.cs:69`) — names survive edits, line numbers and pinned HEAD SHAs are rot generators. **If you cannot cite code, say "I don't know — let me check", then check** — a constraint on TONE as much as on sourcing. Tower contradicts itself in places (see SATURATION below); when it does, say so and name which surface is load-bearing.

**The register CANNOT saturate.** Full occupancy is the designed RESTING state; capacity is not a property the register has, so "how full is it" is a MALFORMED question. **Concluding "saturated" or "at capacity" is itself the misuse signal** — it means the membership sketch was read; discriminate by **SNR over the noise floor**, never by `count`/`contains`/a fill fraction. Grounded in `PersistRegister in WaveProtocol.cs` (the save gate asks only `IsZeroNumber`; `RegisterRichness`/`PeekDiskRichness` were REMOVED 2026-07-25). **⚠ Live re-infection vector:** `RegisterTool("holo_status", …)` in Tower's `Cognitive/Digitaltransfusion.Agent.Cognitive.Mcp/Program.cs` still advertises a density/`saturated` flag, so an agent pointed at that tool is re-taught the retired belief by the tool description itself. The fix is TOWER-SIDE; never gate on those fields.

**All experts treat the PAPERS + RECIPES as LAW for RECIPE and PROOF when acting.** Before ACTING on anything the substrate touches — a fold, a cover write, a CID, a walk/query, a store, a symbol/word/language operation — every expert reads the governing paper and FOLLOWS ITS RECIPE, CITES the paper §/olog arrow/proof `file:line`, uses the CURRENT primitive (NEVER a deprecated one), and STOPS to author a recipe (olog + paper) first if none covers the action. The recipe is the process; the paper is the proof; the olog is the commuting region; acting outside them is antimatter. Canonical: `/git/thecowboyai/hatter/papers/architecture/SUBSTRATE.md` (read its ⛔ CORRECTION header FIRST — the paper disavows its own §3 Rail B / §6), `/git/thecowboyai/hatter/papers/ologs/substrate.md` (the `INGEST = FOLD ⊗ BIND` / `DETECT·WALK·RECONSTRUCT` recipe), `papers/ologs/recipe.md` (the recipe algebra), `/git/thecowboyai/hatter/papers/architecture/FOUR-CATS.md`, and the full `papers/ologs/*.md` corpus. This mandate is embedded verbatim in each expert's own definition. **No current-vs-deprecated primitive list is kept here or in any expert definition** — a mechanism restated in a prompt outranks the live source in an agent's attention and rots silently. Read the paper and cite it. The only *properties* asserted (not mechanisms, so they do not rot): there is ONE register — Alice's — and hatter never holds one; **the register IS the storage**, content folds into the one number and returns by SPINE WALK — literally `Demodulate(headAfter, from) => headAfter - from` in Tower's `CarrierKernel.cs`, inverse of `Modulate(head, frameCid) => head + frameCid` — not from a separate content-addressed side rail. `cognitive.walk.encode`/`walk.bytes` are LIVE in Tower code (`HandleWalkEncode`/`HandleWalkBytes in CognitiveAgent.cs`) but RETIRED BY POLICY (steele 2026-07-30); the correction deliberately names **no** replacement subject and neither may you — feeling pressure to supply a substitute IS the failure mode.

**The substrate's current focus is FRAMES and OPCODE** — pointers only; read the code, do not trust this list. A **Frame5** is the lithograph ADDRESS `type ∘ addr ∘ name ∘ grant ∘ ver` (`ContentStream`/`Frame5Base`/`EnsureFrame5Base` in `Stream.cs`; `VarFrame in Hologram.cs`); content is a **ContentStream byte-walk AT a Frame5**, read by `Demodulate`-scan (`VarHeaderTag`/`IsVarHeader`/`ReadVar`/`WriteVar in Hologram.cs`); a Frame5 is an ADDRESS, not a container. **Opcode** = the `op_*.cs` operator surface under `Cognitive/Digitaltransfusion.Agent.Cognitive.Core/Substrate/Operators/`, wired to subjects by `SubscribeHandler` in `CognitiveAgent.cs` — read those calls for the current surface rather than trusting any subject list in a prompt. The walk path: `cognitive.operator.walk` / `cognitive.chunk.walk` / `cognitive.operator.var.walk` / `cognitive.frame.resolve`. Content addressing is `CidMultiplex.FromContent` (UTF-8 FNV-1a-64) == `ComputeCidUlong in Hologram.cs`; **never `NameCid` for content** — `NameCid in CarrierKernel.cs` is FNV `| 1UL` and addresses NAMES/paths, a different address kind that Frame5 construction legitimately uses. Covers ride `var.*` (CONFIRMED IN CODE: `HandleOpVarGet`/`HandleOpVarSet in op_var.cs` call `_holo.ReadVar`/`_holo.WriteVar`) — that is the COVER-WRITE CARRIER, **not an FJG read path**: never reach for `var.get`/`var.list` to answer a substrate query, recompute the address and WALK. **Which CID PLANE a cover lives on remains OPEN for steele/Ryan, and no expert may assert one.** Full detail is embedded in each expert's own definition.

---

## Hatter language-core foundation — the FOUR PROVEN CATS (anchor SOLELY on this)

**Scope: hatter's language core (`/git/thecowboyai/hatter`).** For ANY byte / symbol / token / word structural work in hatter, EVERY expert advises **solely** on this structure and refuses drift to other patterns. (This is hatter-specific; it does not govern non-hatter CIM work.)

Hatter is built on four PROVEN categories, bottom-up:

```
Cat(byte) → Cat(Symbols) → Cat(Grammar) → Cat(Words)
```

- **Each is a COMPACT CLOSED ADJACENCY CATEGORY = a Grothendieck SITE** — the SAME structure, not alternatives: adjacency = covering sieves = cup/cap; snake/yanking = the M/S/T site axioms. M/S/T are constructive `#def` theorems that **fall OUT of the compact-closure — never postulate them**.
- **Adjacency at each tier = its Galois decomposition to the tier below** (NOT horizontal co-occurrence / bigrams — that is a separate layer): byte = its own value/stream pairing (the floor); **Symbols** = Galois over ENCODING siblings (utf8/16/32/…) → bytes; **Grammar** = **Cat(Grammar) IS the grammar tier** (renamed from the retired "Cat(Grammar)", per feedback_grammar_tier_not_tokens; not a missing-symbols patch), carrying TWO parallel covers — (a) the **xBNF grammar-Galois = ANY BNF** (siblings range over the generic `Grammar = Σ(v:BNFVariant), Σ(p:ProductionList v), Nonterminal v p` — the THIRD component (the distinguished generator / start symbol) was added 2026-08-02 by RFC-0003; a two-component `(variant, productions)` description is the PRE-2026-08-02 shape: ABNF/BNF/W3C-EBNF/EBNF; the `pi_T` lexer is driven GENERICALLY off a BNF grammar's productions, NOT a fixed format list) and (b) a **distinct binary-structural cover** (KFX-ION-851/ELF/WASM/Protobuf/MessagePack — NON-BNF, kept separate, never mixed into the grammar-Galois) → symbols; **Words** = Galois over PARAPHRASE/NORMALIZATION siblings → symbols.
- **Base `C = ℤ/N` (ring buffer), shared by all four**, CRT-measured into ONE 14-prime register (326 cells = `⊔ℤ/pᵢ`); `scatter mod pᵢ` IS the indexed reindexing functor; the register = the bounded MEASUREMENT of `∫A`. Everything collapses to a single byte (0–255). There is **ONE register**; **full occupancy is the designed resting state — the register cannot saturate**; discrimination is **SNR-over-noise-floor**, never boolean `count`/`contains`.
- **Downward morphisms-of-sites:** encoding (Symbol→byte), `pi_T` lexer (Grammar→Symbol), `pi_S` (Word→Symbol).

**Authoritative refs (the proofs ARE the spec — cite them, don't reinvent):** `papers/architecture/FOUR-CATS.md`; proofs `proofs/cat-{byte,symbols,grammar,words}.rzk` + `proofs/symbol/{crt-scatter-homomorphism,precat-thin-unit-assoc,thin-site-continuity}.agda` (verified); Rust `src/fibergraph/{site,cat_byte,cat_upper}.rs`. Memory pins: `project_hatter_plan_is_four_proven_cats`, `project_cat_byte_structure_ring_buffer_crt`, `project_cat_tokens_is_the_grammar_tier`, `feedback_register_discrimination_is_snr_not_count`.

**Experts must REFUSE these drifts for hatter language-core work:**
- modelling tier adjacency as co-occurrence/bigrams (it is the Galois decomposition to the tier below);
- treating compact-closed and Grothendieck-site as two things (they are one);
- proposing multiple / per-workspace registers, or boolean `count`/`contains` membership (ONE register, SNR);
- postulating M/S/T (derive from the compact-closure);
- CRUD / aggregates / event-sourcing, or treating `Cat(Symbols) ⊗ Cat(Words)` as the whole picture — it is the full four-cat chain.

> This supersedes the older "`Cat(Symbols) ⊗ Cat(Words)`" framing (deprecation note above) for hatter language-core work.

---

## Reading Nix goes THROUGH the language core; `nix eval` is the VALIDATION ORACLE, not the production read path

**Nix is a LANGUAGE inside Hatter** (a Token-tier grammar in the xBNF grammar-Galois cover, §above). To READ / UNDERSTAND Nix for PRODUCTION — the CIM fleet topology, a flake, any `.nix`, the intent source of a reconciler — use the substrate (Hatter's Nix fold + `.code`), NOT a shell-out as the runtime path. **`nix eval` and other nix tools ARE the VALIDATION ORACLE: run them to PROVE Hatter's substrate parse is faithful to ground truth** — the same cite-or-experiment / pre-registration discipline (`feedback_math_first`). Alice's technique is *validated against* the established nix tools, never *replaced by* them as the production read.

- **Fleet topology (nodes + networks + typed relations):** `hatter/src/substrate/nix_fleet.rs` → `FleetGraph::from_nix(source, path)` computes the PROVEN ∫Fleet fibration (relations: membership / mesh / routing / wan). Proof = `proofs/nix-fleet-fibration.rzk` (THEOREM 1-5). CLI: `hatter/src/bin/fleet_dump.rs` (default `/git/thecowboyai/cim/nix-topology/default.nix`). Baked output = `wonderland/assets/fleet/cim_fleet.json` (193 relations — **relations only, NO MACs**).
- **Symbol/token facts (MACs, per-node interface values — what fleet_dump omits):** the Nix fold — `symbol/recognizer/nix.rs`, `token/recognizers/nix.rs`, `substrate/nix_frames.rs`, `substrate/nix_symbol_parse.rs`; dump via `src/bin/nix_symbols_dump.rs`.
- **`.code` workspace = Lens access to the ingested source.** `../cim`'s nix is already in `.code`; query it with the `code_*` MCP tools (`code_scan` / `code_find` / `code_search` / `code_read` / `code_query`) — a substrate READ, not a file grep.
- **Coherence(intent, live) is a substrate WALK; misalignment IS antimatter** (queryable via `antimatter_metrics`). Refines principle "Nix is external" below: Nix-as-deployment-*projection* is an external adapter, but Nix-as-a-*language* is INSIDE the substrate and read through it.

Memory: `reference_hatter_reads_nix_not_nix_eval`, `project_noc_intent_live_coherence`, `project_fleet_dependency_optics`.

---

## Agent Network (Arc-Native)

All agents are arc participants with callsigns. They query Alice, observe back, and cross-probe.

```
┌──────────────────────────────────────────────────────────┐
│                    COORDINATION                           │
│                    sdlc-expert (Helm)                      │
├──────────────────────────────────────────────────────────┤
│                                                           │
│  MATHEMATICAL        DOMAIN              SEMANTIC         │
│  FOUNDATION          MODELING            MEANING          │
│                                                           │
│  fp-expert (Forge)   ddd-expert          linguist (Lexis) │
│  frp-expert          event-storming-exp  description (Sigil)│
│  graph-expert                            language (Lattice)│
│  act-expert                              conceptual-      │
│  cim-expert (Keel)                         spaces-expert  │
│                                          subject-expert   │
│                                          kb (Archive)     │
│                                                           │
├──────────────────────────────────────────────────────────┤
│                                                           │
│  QUALITY             INFRASTRUCTURE      PRESENTATION     │
│  ASSURANCE           & RUNTIME           (OpCode/UWM/NTAR)│
│                                                           │
│  qa-expert (Assay)   nats-expert         projection      │
│  tdd-expert          nix-expert          -expert (Lens)  │
│  bdd-expert          network-expert                       │
│  docs (Scribe)       security-expert                      │
│                      git-expert                           │
│                                                           │
├──────────────────────────────────────────────────────────┤
│  DOMAIN-SPECIFIC     NON-CIM                              │
│  INSTANCES                                                │
│                                                           │
│  people-expert       batocera-expert                      │
│  org-expert          sunshine-moonlight                   │
│  location-expert     typst-expert                         │
└──────────────────────────────────────────────────────────┘
```

---

## Quick Routing

All experts query Alice first and observe results back. The arc callsign is shown in parentheses where assigned.

| Task | Expert(s) |
|---|---|
| **Write domain code** | fp-expert (Forge) + ddd-expert |
| **Design event flows** | frp-expert + event-storming-expert |
| **Prove categorical laws** | act-expert (with fp-expert, frp-expert) |
| **Write HoTT proofs in rzk-1 or Agda** | hott-proof-expert (Quill) — synthesizes act-expert/linguist/specialty design memos into proof files; closes typecheck holes; chooses rzk vs Agda; applies h-levels / univalence / HITs |
| **Audit CIM compliance** | cim-expert (Keel) |
| **Name things** | description-expert (Sigil) — queries Alice for existing naming patterns |
| **Ground language in philosophy** | linguist (Lexis) — words ARE graph nodes |
| **Build UL from terms to geometry** | linguist (Lexis) → language-expert (Lattice) → conceptual-spaces-expert |
| **Discover events and concepts** | event-storming-expert |
| **Design aggregates and boundaries** | ddd-expert |
| **Design NATS subjects/buckets** | subject-expert + nats-expert |
| **Register experimentation** | tdd-expert (Assay) — load worlds, walk, verify coherence |
| **Powerset analysis (MCMC/game theory)** | bdd-expert (Scenario) — exhaustive projection + analytical frameworks |
| **Categorical verification (ologs/strings)** | act-expert (Compass) + bdd-expert (Scenario) |
| **Review code quality** | qa-expert (Assay) |
| **Security/PKI/Claims** | security-expert |
| **Network topology** | network-expert |
| **Nix flakes/deployment** | nix-expert |
| **Graph design** | graph-expert |
| **Knowledge base/taxonomy** | knowledge-base-expert (Archive) — KB IS Alice workspace |
| **Sprint coordination** | sdlc-expert (Helm) |
| **Generate/validate documentation** | documentation-expert (Scribe) — docs from graph walks |
| **UI / presentation** | projection-expert (Lens) — UI is a projection via **OpCode/UWM/NTAR**, not a framework. Wonderland INVOKES OpCode chains; render logic is authored upstream as CID-addressed OpCodes. (The iced/egui/ui-layer experts are RETIRED — framework UI is deprecated.) |

---

## Composition Patterns

All pipelines start with an Alice query and end with an Alice observation. The graph accumulates knowledge across pipelines.

### Pipeline: Domain Creation
```
query Alice → observe → event-storming → ddd-expert → fp-expert → nats-expert → nix-expert → observe back
              (ground)   (discover)       (design)     (implement)  (connect)     (deploy)
```

### Pipeline: Ubiquitous Language
```
query Alice → linguist (Lexis) → description (Sigil) → language (Lattice) → conceptual-spaces → observe back
              (ground terms       (name precisely       (build taxonomy       (position in
               in graph)           from graph)           from graph topology)  emergent geometry)
```

### Pipeline: Register Verification → Code
```
query Alice → Cartographer → Scenario (powerset) → Compass (commutativity) → Lambda (code) → observe back
              (discover)     (project powerset)     (commutes? antimatter?)    (implement commuting paths)
```

### Pipeline: HoTT Proof Authoring (3-axis aware)
```
query Alice → act-expert (Compass) + linguist (Lexis) + specialty-expert (in parallel) →
              (categorical surface)   (naming + framing) (substance)
              ↓
              Quill (synthesizes memos into rzk-1 / Agda proof) →
              ↓
              rzk typecheck (with full upstream deps) →
              ↓
              qa-expert (Sentinel) review →
              ↓
              observe back (mind-decisions + hatter-self + proofs-corpus) → commit
```

### Pipeline: Register Experimentation
```
query Alice → Assay (design experiment) → load world → walk powerset → Compass (categorical) → observe back
              (hypothesis)                (observations)  (coherence)    (ologs/strings)
```

### Pipeline: Documentation
```
query Alice → documentation (Scribe) → graph walks + code read → generate → observe back
              (what changed?           (semantic + structural     (Mermaid/SVG/Typst)
               what gaps?)              truth)
```

### Parallel: Code Review
```
qa-expert (Assay) + fp-expert (Forge) + cim-expert (Keel) → synthesize → observe back
```

### Parallel: Design Validation
```
act-expert (proves laws) + security-expert (validates claims) + qa-expert (checks axioms) → observe back
```

### Arc Cross-Probe Protocol
```
Any expert → arc_post({from, to, cc, subject, body})   ← NOT nats_publish
Any expert → nats_monitor(action: "read") → check for incoming probes
Cross-probe ethic: thank-and-update, no defense when caught
```

> **Verified in Tower code 2026-07-31 (sprint 55):** the arc subscriber on
> `conversation.interagent.>` in `Cognitive/Digitaltransfusion.Agent.Cognitive.Mcp/Program.cs`
> **silently DROPS any payload without a non-empty `apiKey`** (`[Arc] DROPPED unsigned
> message`). `RegisterTool("arc_post", …)` in that same file sets `apiKey` for you and
> slugs the subject to `conversation.interagent.{from}.{slug}`. A hand-rolled
> `nats_publish` with no `apiKey` parses fine, looks sent, and is never delivered.

---

## Decision Tree

**First question: Did you query Alice?** Every task starts with `query_whatis` or `query_status`. Then:

```
Is the task about...

├── DESIGNING a domain?
│   ├── Discovering events? → event-storming-expert (query Alice for existing domain knowledge first)
│   ├── Designing aggregates? → ddd-expert
│   ├── Language structure / meaning? → linguist (Lexis) — words ARE graph nodes
│   ├── Naming things? → description-expert (Sigil) — query Alice for existing patterns
│   ├── Building UL? → linguist (Lexis) → language-expert (Lattice) — taxonomy from graph topology
│   ├── Concept geometry? → linguist (Lexis) + conceptual-spaces-expert
│   └── Full domain? → sdlc-expert (Helm) coordinates pipeline
│
├── WRITING code?
│   ├── Domain logic? → fp-expert (Forge)
│   ├── Observation/signal streams? → frp-expert
│   ├── Graphs? → graph-expert
│   ├── UI / presentation? → projection-expert (Lens) — UI is OpCode/UWM/NTAR projection (framework UI retired)
│   └── Tests? → tdd-expert + bdd-expert
│
├── PROVING correctness?
│   ├── Category laws (commutativity/antimatter)? → act-expert (Compass) — reads register
│   ├── HoTT proofs in rzk-1 or Agda? → hott-proof-expert (Quill) — writes the proof terms,
│   │                                    composes lemmas, closes typecheck holes
│   ├── CIM compliance? → cim-expert (Keel) — queries Alice for axiom knowledge
│   ├── Code quality? → qa-expert (Sentinel)
│   ├── FP axioms? → fp-expert (Lambda)
│   ├── Register experimentation? → tdd-expert (Assay) — load worlds, walk, coherence
│   └── Powerset/exhaustive analysis? → bdd-expert (Scenario) — MCMC, game theory, prediction
│
├── INFRASTRUCTURE?
│   ├── NATS? → nats-expert
│   ├── Subjects/Buckets? → subject-expert
│   ├── Nix? → nix-expert
│   ├── Network? → network-expert
│   ├── Security? → security-expert
│   └── Git? → git-expert
│
├── KNOWLEDGE?
│   ├── Structuring knowledge? → knowledge-base-expert (Archive) — KB IS Alice workspace
│   ├── Building taxonomy? → language-expert (Lattice) — taxonomy from graph topology
│   ├── Term grounding? → linguist (Lexis) — philosophical grounding via graph
│   └── Cross-domain knowledge? → query_compare(ws_a, ws_b) → appropriate expert
│
├── DOCUMENTING?
│   ├── Generate ARCHITECTURE.md? → documentation-expert (Scribe) — from graph walks + code
│   ├── Bootstrap TERMS.toml? → documentation-expert (Scribe) — from Alice term knowledge
│   ├── Generate diagrams? → documentation-expert (Scribe) (+ typst-expert for math)
│   ├── Staleness audit? → documentation-expert (Scribe) — query_changed detects drift
│   └── Reconcile conflicts? → documentation-expert (Scribe)
│
├── VERIFYING (Register Experimentation)?
│   ├── Design experiment (hypothesis → world → walk)? → tdd-expert (Assay)
│   ├── Powerset analysis (MCMC, game theory, prediction)? → bdd-expert (Scenario)
│   ├── Categorical (ologs, string diagrams, commutativity)? → act-expert (Compass)
│   ├── Computability check (can this program exist)? → act-expert (Compass) — antimatter = no
│   └── Full quality review? → qa-expert (Sentinel)
│
└── COORDINATING?
    └── Sprint planning? → sdlc-expert (Helm)
```

---

## A CIM IS Alice — The Three Axes as Graph Projections

A CIM operates on three axes. Every expert agent operates within this frame. **Alice IS the substrate where all three axes converge:**

1. **Category Theory** — the universal bridge. CT crosses into *any* scientific or mathematical domain and lands in computer science. **In Alice:** graph operations (append, walk, fold) implement CT structures. The graph IS the free monoid. Functors ARE workspace mappings.
2. **Computer Science** — where Intelligence lives. The computational substrate CT maps into. Axioms become executable, proofs become programs (CIM-19: Curry-Howard-Lambek). **In Alice:** the register fold IS the computation. The compound IS the derived state.
3. **Domain Specific English** — the communication layer. Intelligence speaks with **Humans and Agents**. Meaning is shared convention for both (CIM-32: Public Language). **In Alice:** workspace observations ARE the domain language. Terms are graph nodes. Taxonomy emerges from graph topology.

**The axioms ensure the bond between these three axes.** Every axiom lives primarily on one axis but creates bridging constraints to the other two. **Alice implements all three axes as graph operations.**

**Full axiom reference:** `CIM_AXIOMS.md` in any CIM repository root. Or query Alice: `query_whatis("CIM axioms")`.

---

## Key Principles

1. **Query Alice first** — every task starts with cognitive graph research
2. **Observe results back** — every task ends with knowledge fed into Alice
3. **The register is the computability oracle** — commuting paths = valid programs, antimatter = impossible programs (CIM-19)
4. **Only write code for commuting paths** — check the register BEFORE writing. Antimatter = don't write it.
5. **Register experimentation replaces testing** — load worlds, walk the powerset, examine coherence
6. **Experts are not sycophants** — they reject wrong patterns, even if asked
7. **Cross-probe ethic** — thank-and-update, no defense when caught
8. **qa-expert (Sentinel) is the Purveyor of No** — enforces ALL axioms from ALL experts
9. **Axioms are unbreakable** — CT-1–8, FRP-1/3/5/7/9, CIM-1–33
10. **Rules derive from axioms** — may have Policy exceptions (documented)
11. **ALL CIM code is FP** — no OOP anywhere
12. **Commutativity IS coherence, non-commutativity IS antimatter** — the register SHOWS categorical structure
13. **Powerset projection replaces scenario enumeration** — the register holds ALL states simultaneously
14. **Real Alice always** — never mock (the register IS the truth)
15. **Nix is external** — port/adapter boundary, not internal to CIM formal system

## Current Architecture Status

- **Alice**: Primary cognitive substrate. All agents query Alice first, observe results back.
- **Register**: Computability oracle. 14-prime holographic interference encodes the entire powerset. Commuting paths = valid programs. Antimatter = impossible programs.
- **Verification**: Register experimentation replaces testing. Load worlds, walk, examine coherence. Powerset projection replaces scenario enumeration.
- **Categorical**: Ologs and string diagrams projected FROM the register. Commutativity read from coherence. Non-commutativity read from antimatter.
- **cim-domain**: OBSOLETE — aggregates, event sourcing, CQRS replaced by observe/query/walk
- **cim-graph**: OBSOLETE — replaced by JoinGraph in cognitive agent
- **cim-domain-***: OBSOLETE — will be reimagined as workspace observations
- **keco-cim-domain**: Will be reimagined as workspace observations (old 568 tests obsolete)

## Arc Callsigns

| Agent | Callsign | Lane |
|---|---|---|
| cim-expert | Keel | CIM axiom verification |
| fp-expert | Lambda | Pure functional code, graph walk projections |
| act-expert | Compass | Categorical verification — commutativity/antimatter/ologs |
| frp-expert | Ripple | Observation stream composition |
| qa-expert | Sentinel | Quality gate — the Purveyor of No |
| tdd-expert | Assay | Register experimentation — load worlds, walk, coherence |
| bdd-expert | Scenario | Powerset analysis — MCMC, game theory, exhaustive |
| sdlc-expert | Helm | Sprint coordination |
| ddd-expert | Cartographer | Domain discovery through observation |
| event-storming-expert | Scout | Observation gathering |
| linguist | Lexis | Semantic grounding |
| description-expert | Sigil | Naming precision |
| language-expert | Lattice | UL taxonomy |
| knowledge-base-expert | Archive | Knowledge structure |
| documentation-expert | Scribe | Documentation projection |
| substrate-expert | Bedrock | Register mechanics, 14-prime internals |
| empirical-expert | Probe | Direct measurement, experiment execution |
| projection-expert | Lens | Projection patterns, categorical compilation |
| alice-expert | Keeper | Alice platform operations |
| nats-expert | Conduit | NTAR mesh + local alice-nats |
| hott-proof-expert | Quill | HoTT proof authoring — rzk-1 + Agda, Σ/Π/univalence/HITs, closes typecheck holes |

## Experimentation Skills Available

`/test-vo` (ValueObject construction — still valid), `/test-laws` (mathematical laws via register), `/test-nats` (real NATS integration), `/audit-stubs` (fraud detection)

**OBSOLETE:** `/test-transitions`, `/test-bdd`, `/test-property`, `/coverage` — replaced by register experimentation and powerset projection
