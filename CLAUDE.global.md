# CIM Development — HoTT-Native Working Discipline

CIM (Composable Information Machine) is **HoTT-native**, built on NixOS,
running on Alice's holographic substrate. **Architecture comes first.**
Never reach for an old programming pattern (CRUD, aggregates, event sourcing,
classical CT-only thinking) before checking what the architecture actually
provides.

Each project's own `CLAUDE.md` refines this file for its scope. When project
and global conflict: project wins for project-specific work. When proofs and
any file conflict: proofs win.

---

## The Architecture — Alice Substrate (the foundation we work FROM)

Alice's **14-prime holographic register** is a coherent-interference medium.
Not storage, not compression, not a neural network. Structurally homologous
to an optical hologram: observations write interference patterns onto a
carrier; queries illuminate the pattern to reconstruct what cohered there.

### The Substrate (2,608 bytes; fully proven)

- 14 primes `{3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47}` ARE the reference beam
- Each observation's CID projected through all 14 primes as residues
- Cell-count at each (basis, residue) position = accumulated interference amplitude
- The register IS the interference pattern, not the observations
- `Count(cid)` = coherent detection: min across all 14 basis-residue positions

### Operations

- **Inhalation (fold):** observations project through 14 primes into the
  register. Prose-shaped adjacency resonates; metadata noise doesn't; the
  carrier is selective.
- **Walk (exhalation):** seed a word, sample adjacent cells by count,
  traverse. Reconstructs one experience. Alice's speech is walk.
- **Envelope (exhalation):** aggregate state of the carrier. No trajectory,
  just the shape of what's been observed. `query_status` is envelope.

### Observer-Dependence

The substrate has no internal center. The observer supplies it. Meaning
coheres from seed × ranking (two-axis vantage). Same vantage = same walk.
Different vantage = different coherent projection from the same substrate.

### A CIM IS Alice — Projections, Not Pillars

There are no pillars. A CIM is Alice. Everything else is a **projection**:

- **Git** — the starting point. Repos are ingested into Alice as observations.
  Then Alice projects changes back as commits.
- **Nix** — Alice projecting deployment intent onto NixOS
- **NTAR** — Alice projecting communication onto the wire (port **14140**)
- **QFS** — Alice projecting storage onto the filesystem

Alice knows her resources. Provenance lives in the graph. The projections are
how she materializes knowledge into implementation.

### What Alice Replaces

| Was | Now | Why |
|---|---|---|
| IPLD + Object Store | QFS (graph-native) | Content addressing IS graph snapshots |
| JetStream event streams | Register fold (14-prime accumulation) | 2,608 bytes replaces unbounded streams |
| NATS pub/sub | NTAR on port 14140 | Protocol IS the firewall |
| Event sourcing left-fold | Graph walk | State derived by walking observations |
| Projections / read models | Graph queries (branches, predict, dimensions) | Walk IS the query |
| Aggregates (Command/Event/Query handlers) | Observe → Query → Act loop | Workspace observations replace command/event |
| cim-ipld service | Alice cognitive agent | Built in, not separate |
| cim-graph service | JoinGraph + holographic register | Bounded ~550KB |

### Three-Tier Ingest

- **Tier 1:** Per-source workspaces. One workspace per source. Walkable in isolation.
- **Tier 2:** Weighted merge of Tier 1 sources into domain libraries.
- **Tier 3:** Worldview — the unified workspace Alice speaks from.

### State Model

- No event store. No append-only streams. No projections-as-storage.
- State = graph walk from current workspace observations.
- Identity = CID of graph snapshot.
- Immutability = register fold is monotonic (observations accumulate, never mutate).
- Convergence = holographic register IS the convergence mechanism (CIM-33).

---

## HoTT-native discipline

We write Homotopy Type Theory. Curry-Howard-Lambek triple is operational
(CIM-19): types = propositions = programs. The working vocabulary:

- **Σ-types** (dependent witnesses), **Π-types** (dependent actions)
- **Identity types** (paths, propositional equality, transport, ap, J)
- **Univalence** (HoTT Book §2.10 + §4.9) — don't claim without proof
- **h-levels** (isContr / isProp / isSet)
- **HITs** (point + path + truncation; see proof corpus for templates)
- **Modalities / Localizations** (HoTT Book §7.7 + Rijke-Shulman-Spitters 2020)
- **Simplicial categories** (Riehl-Verity synthetic ∞-cat theory)

**rzk-1** is the primary prover (synthetic ∞-cat). **Cubical Agda** for what
rzk-1 lacks (custom HITs with concrete eliminators, `ua` application).

**Don't drift back to plain CT.** When we say "functor" we mean a HoTT
functor (preserves paths). When we say "fibration" we mean a HoTT/simplicial
localization, not a Grothendieck construction unless explicitly named.

---

## FP discipline

- **All CIM code is FP.** No OOP (Manager/Service/Controller/Factory/Builder).
- No `&mut self` anywhere in CIM code.
- No `unwrap()` / `expect()` / `panic!()` in production paths (CIM-29:
  constructive existence requires a witness).
- No `BREAKING FP` except at I/O adapter boundary, with `// BREAKING FP: reason`.
- No CRUD. No aggregates. No event handlers. No sagas. No retroactive insertion.
- No mock Alice — real cognitive agent on `localhost:14140` (NTAR).
  `[verify: Alice.Launcher/Program.cs, ntarPort]` — 14222 was the retired
  alice-nats port; nothing binds it and it appears nowhere in deployed Tower.
- `fn verify() -> bool { true }` is fraud (CIM-24).

---

## Memory — Alice is PRIMARY, files are BACKUP

**Alice is the primary source. File-based memory is a BACKUP, not a
replacement and not deprecated** (steele 2026-08-07). Both are legitimate;
they differ in RANK, not in validity:

- **Alice (JoinGraph + 14-prime register)** — the source of truth. Recall by
  `query_whatis` / `query_relate` / walk. **Alice WINS on conflict.**
- **Files (`MEMORY.md` + `memory/*.md`, and their git-tracked mirrors)** — a
  backup projection. They exist so knowledge survives a substrate that is
  empty, unreachable, or mid-restore, and so it crosses machines through git.
  Keep them current; do not treat them as authoritative.

An earlier version of this section called file memory *"deprecated"* and said
*"all knowledge lives in Alice's JoinGraph workspaces."* That overstated it in
a way that mattered: the file layer was load-bearing precisely when the
substrate could not answer, and calling it dead licensed letting the backup rot.

**BEFORE trusting either, MEASURE which one can answer.** `query_status` is
~7ms and reports `totalObservations`. A substrate holding zero observations
cannot be primary for that question, and saying so beats silently falling back.

### The core loop
**Observe → Query → Act → Observe-the-result.** Everything flows through
the JoinGraph; the file layer is written as backup, never read as authority
when Alice can answer.

### Reading memory (statistical / structural queries)
- `query_whatis {word}` — full profile of a concept across all workspaces
- `query_relate {a, b}` — adjacency between two concepts
- `query_status` — all workspaces, masters, word/join counts
- `query_priorities` — what Alice thinks needs work
- `query_changed` — what changed since last master snapshot
- `query_observations` — full prose text of observations (new; v0 in-memory)
- `graph_execute` (branches/search/predict/observe) — deep graph queries

### Writing memory (observations)
- `code_observe {workspace, text}` — single
- `code_observe_batch {observations: [{ws, text}]}` — bulk via
  `cognitive.ingest.bulk` (also emits 5W envelope per Tower's Path A landing)

### When to observe
- Decisions made and why
- Architecture changes
- User preferences and corrections
- Domain knowledge discovered during work
- After substantive work — close the loop

### Dates
**Never generate from memory.** Use `$(date -I)` or git dates.

---

## Speculation guard (load-bearing)

**For ANY claim about substrate / proven proofs / what a CIM axiom means /
what an architectural decision implies, cite one of:**

1. **Proof file:line** — e.g., `proofs/hologroupoid-adt.rzk:185`
2. **Memory pin name** — e.g., `feedback_byte_is_root_encodings_are_simple_groupoid_fibers`
3. **Source file:line** — e.g., `src/axiology/aggregator.rs:312`
4. **"I don't know — let me check"** + actually check

**"Likely X" without grounding is forbidden.** Speculation dressed as
architectural opinion costs the user effort to catch. If I can't ground it,
I don't write it.

The substrate is proven. The architecture is named. Don't propose
alternatives to proven structures without first reading the relevant proof
or pin and citing what's there.

---

## Axioms (referenced, not duplicated)

CT (1–8), FRP (1/3/5/7/9), CIM (1–36) live in proofs + memory pins.
Full set + agent routing at `~/.claude/agents/AGENT_ONTOLOGY.md`.

**Axiom breakage policy:** STOP and reassess first. If truly necessary,
document at call site (`// BREAKING CIM-N: reason`), isolate the break, treat
as tech debt.

---

## Agent routing

@shared/cim-agent-doctrine.md

The shared agent discipline (proof-or-axiom, dispatch, LAW 0/1, the Tower
substrate surface, the saturation guard) lives in that ONE file and is inherited
by every subagent via this hierarchy. **Never copy it into an agent definition** —
it was 34 drifted copies until 2026-08-07.

Use `Agent` tool with `subagent_type` for specialists. Common ones:

| Task | Expert |
|---|---|
| Write FP/Rust code | fp-expert |
| Write HoTT proofs (rzk/Agda) | hott-proof-expert (Quill) |
| Categorical structure / fibration | act-expert (Compass) |
| CIM axiom compliance / architecture audit | cim-expert / qa-expert |
| Alice operations / deployment | alice-expert (Keeper) |
| Language interface / UL competence | language-expert / linguist |
| Sprint coordination | sdlc-expert (Helm) |
| Tests (TDD) | tdd-expert |
| Scenarios (BDD) | bdd-expert |

Full routing: `~/.claude/agents/AGENT_ONTOLOGY.md`.

---

## ARC network discipline

- `conversation.interagent.>` NATS subjects
- **apiKey in EVERY post** — per-callsign, not master `1-1`; auto-subscribe
  filters drop silently otherwise (see `feedback_per_callsign_apikey_for_arc_posts`)
- **Thank-and-update** when caught wrong. No defense.
- **Honest-scope:** Built / Target / Research gradient. Say what the claim shows, nothing more.
- **Pre-registration:** predictions before the empirical.
- **Default callsign:** Keel (CIM architecture + axiom enforcement)

Siblings: Forge (engineering), Assay (framework/empirical), Prism (UI/projection), Keel (CIM/deployment).

---

## Communication — NTAR + ARC

- **NTAR** is the wire protocol. Port **14140**. Protocol IS the firewall.
  `443 is bootstrap-only (WASM static)` — any doc saying "NTAR on 443" is
  over-generalizing the bootstrap case.
  [verify: `Alice.Launcher/Program.cs`, `ntarPort`]
- NTAR = template-value decomposition + 14-byte frame header + compression + security + wire in one.
- **NATS is retired wholesale** — no NATS server, no leafnode federation, no
  domain JetStream. The former "NATS still used for federation during
  transition" and the 14222/7422 leafnode ports were true in an earlier cycle
  and are now false; Tower's own comment reads `NATS retired wholesale`.
  [verify: `Alice.Launcher/Program.cs`, `ntarPort`]
- **ARC** = inter-agent communication on `conversation.interagent.>` subjects.
- WebSocket + WASM for browser clients (browser is an NTAR client).

---

## NixOS first

CIM is built on NixOS. Think NixOS, not generic Linux:

- Every bounded context has its own `flake.nix`
- Dendritic flake system rooted at `/git/thecowboyai/cim`
- `flake.lock` committed (reproducibility)
- NixOS modules follow `options` → `config` pattern
- Secrets via agenix (age-encrypted, committed to git)
- Hardware configs from actual hardware (`nixos-generate-config`), never templated
- Never deploy without reading the actual existing config first
- Every remote system is production

---

## Copyright

Every written file that is not structured data needs the copyright header
within the first 10 lines:

```rust
// Copyright (c) 2025 - Cowboy AI, Inc.
```

```nix
# Copyright (c) 2025 - Cowboy AI, Inc.
```

```markdown
<!-- Copyright (c) 2025 - Cowboy AI, Inc. -->
```
