---
name: cim-substrate
description: Tower substrate mechanism reference for CIM work — the LAW 1 paper/recipe corpus (SUBSTRATE.md, FOUR-CATS.md, papers/ologs), and the Tower symbol surface (Frames/Frame5, the op_* opcode surface, walk paths, var.* cover carrier, NTAR port). Load this when an action touches a fold, a cover write, a CID, a walk or query, a store, or a symbol/word/language operation — i.e. whenever you need to cite a Tower symbol or follow a governing recipe.
---

⛔ **CIDs ARE LOCKED — ALL ARE `u64`** (steele 2026-08-17). Eight bytes, sixteen hex. The
carrier math is therefore `u64` arithmetic: `Modulate(head, frameCid) => head + frameCid`,
`Demodulate(headAfter, from) => headAfter - from`. A head is a `u64`; a wire is a `u64`, not
a buffer; the 14-prime projection is `cid % pᵢ` over it. **A 64-hex value is the retired
32-byte IPLD-shaped form** — prefer the `u64` wherever two surfaces disagree.


<!-- Copyright (c) 2025 - Cowboy AI, Inc. -->

# CIM Substrate Reference

Progressive-disclosure companion to `~/.claude/shared/cim-agent-doctrine.md`.
The doctrine file carries the RULES (always loaded); this carries the LOOKUP
(loaded when the work touches the substrate).

**LAW 0 still governs and is in the always-loaded doctrine: Tower's CODE
outranks every document, including this one. Cite by STABLE SYMBOL, never by
line number. Source root `/git/thecowboyai/Tower/code/`.**

## LAW 1 — Papers + Recipes govern RECIPE and PROOF (strict when ACTING)

Before ACTING on anything the substrate touches — a fold, a cover write, a CID, a
walk/query, a store, a symbol/word/language operation — you MUST:

0. **⭐ THE MODEL — THREE AUTHORITATIVE DOCS, read before reasoning about the
   substrate at all** (steele 2026-08-08: *"THESE 3 ARE authoritative"*). They
   are in **Tower**, not hatter, and they outrank any inference either of us
   draws from reading Tower code in isolation:

   - `/git/thecowboyai/Tower/papers/architecture/os-atlas.html`
   - `/git/thecowboyai/Tower/papers/architecture/how-alice-works.html#proof`
   - `/git/thecowboyai/Tower/papers/architecture/substrate-math-dock.html`

   **How they sit against LAW 0.** These give the MODEL — what the math is, what
   the design intends, what is proven. Tower CODE still gives the current
   MECHANISM STATE (is this call wired today?). Neither overrides the other:
   ask the docs *what should happen*, ask the code *what is happening*. A gap
   between them is a finding, not a licence to pick one.

   ⚠ `substrate-math-dock.html` is itself marked **"Superseded in part
   (2026-07-28)"** in favour of `how-alice-works.html`. Treat any
   bit-packing / ≤7-byte-value / bloom-probe / put-get-object-store language in
   it as HISTORICAL.

   These are HTML with heavy inline CSS — strip `<style>`/`<script>` before
   reading or the prose drowns.

1. **Read the governing paper and FOLLOW ITS RECIPE.** Substrate mechanism:
   `/git/thecowboyai/hatter/papers/architecture/SUBSTRATE.md` + its commuting
   olog/recipe `/git/thecowboyai/hatter/papers/ologs/substrate.md`
   (`INGEST = FOLD ⊗ BIND`; `DETECT / WALK / RECONSTRUCT`). Four-cat foundation:
   `/git/thecowboyai/hatter/papers/architecture/FOUR-CATS.md`. Recipe corpus + algebra:
   `/git/thecowboyai/hatter/papers/ologs/*.md` (each an SMP process, `x → y = "make y
   from x"`; series = `∘`, parallel = `⊗`; `papers/ologs/recipe.md`). **Where a paper's
   MECHANISM claim disagrees with Tower code, the code wins (LAW 0) and the paper is the
   thing to fix.**
2. **CITE** the paper §, olog arrow, or proof `file:line` you are executing — plus the
   Tower SYMBOL if the action touches the substrate. No ungrounded action; "likely X"
   without grounding is forbidden (the speculation guard). The proofs ARE the spec.
3. **Use the CURRENT primitive — read the authority, do not restate it here.** Carry no
   primitive list in this file. The following are safe only because they are
   *properties*, not mechanisms, and each is verifiable in Tower source in seconds:
   - There is **ONE register — Alice's**; hatter never holds one.
   - **THE AXIOM — everything rests on two lines** (`how-alice-works` §2):
     `a + b = ab` and `a = a`, therefore **`ab − b = a`, EXACTLY**. Reading is
     **not searching and not guessing — it is ONE SUBTRACTION.** Superposition
     is lossless *by axiom*, so long as you know what to subtract.
   - **A CONTENT ADDRESS IS A SUM YOU SUBTRACT OUT OF** (`substrate-math-dock`):
     ```
     head       = Modulate(Frame5, contentCid) = Frame5 + contentCid
     READ:      contentCid = head − Frame5        # peel by subtraction, exact
     ```
     `how-alice-works` §7, the whole content store in four lines:
     ```
     rungs = ContentStream.FromBytes(value, alphabet)
     head  = fold each rung onto Frame5, then VersionCommit(head)
     value = Compose(walk back from head by Demodulation)
     ```
     **A fold with NO Frame5 leaves no operand to subtract** — the walk exists
     on the number and nothing can address it. That is a real, shipped defect
     class: see `hatter/papers/TOWER-CONTENT-READBACK-DEFECT.md` §2a.
   - ⭐ **THE BYTES ARE IN THE REGISTER. THE MAPS TO THE BYTES ARE IN THE GRAPH.** steele
     2026-08-24: *"'technically correct' — the data IS in the register, but you have to LOAD
     the register with bytes, and **the maps to the bytes are in the Graph**."*
     **NEITHER HALF IS SUFFICIENT ALONE:** bytes with no map are unaddressable; maps with no
     bytes have nothing to map. So *"the register is the storage"* is **technically true and
     operationally useless** — it is the half that cannot be walked. **The register also
     MEASURES** (*"what is the POSITION of these bytes?"* → a cid; *"seen it?"* → a 0..14
     quorum) and is **FIXED at 2,616 bytes**, which is why the maps cannot be in it.
   - **THE FOLD AND THE WALK ARE CARRIER ARITHMETIC, and that part is unchanged:**
     `Modulate(head, frameCid) => head + frameCid` folds; `Demodulate(headAfter, from)
     => headAfter - from` recovers — `CarrierKernel.cs`. **There is no separate
     content-addressed side rail**, and no second store beside the fold.
   - ⚠ **A FOLD WITH NO Frame5 STILL LEAVES NOTHING TO SUBTRACT** — see above. That is a
     defect in the ADDRESSING, not evidence that the register stores content.
   - **STORAGE IS NEVER THE VALUE ALONE** (`os-atlas`): *"the whole frame folds,
     not just the value: scene→view→instance→template→Frame5→var-head→value all
     land in the 14 planes."* The value is wrapped into a var version-frame (var
     head = `type ∘ name ∘ version`). Folding a bare value is the bug.
   - **THE FOUR STREAMS are the only way in and out** (`how-alice-works` §6):
     `ContentStream` (the byte-walk — `Compose` → bytes, byte-exact),
     `VersionStream` (history/spine), `ThoughtStream` (the live 16-byte cursor),
     `PredictionStream` (Jacob's ladder — forecasting/learning). Nothing else.
   - **COMMUTATIVITY IS THE CONVERGENCE GUARANTEE, NOT A WEAKNESS** (§04).
     `Modulate` is commutative and associative *so that* independent streams fold
     in any order and two towers that saw different things converge. Do not report
     it as a defect — that has happened (see the guardrail below).
   - **Same bytes → same CID → same address**, computed by `CidMultiplex.FromContent`
     (UTF-8 FNV-1a-64) == `ComputeCidUlong in Hologram.cs`; Tower's own comment in
     `ObserveCodeUnits in WordJoinGraph.cs` calls this "== hatter::symbol_cid_of".
     **Never use `NameCid` for content.** `NameCid in CarrierKernel.cs` is FNV `| 1UL`
     and addresses NAMES/paths — a *different address kind* (`ResolvePath`; and
     `VarFrame in Hologram.cs`, which legitimately composes it into a Frame5). Content
     CIDs never carry `| 1`; frame/name carriers do. Do not collapse the two.
   - **A materialized summary is not a section** — recompute the address and walk; never
     read an index.
   - `cognitive.walk.encode` / `walk.bytes` are **LIVE** in Tower (`HandleWalkEncode` /
     `HandleWalkBytes in CognitiveAgent.cs`) but **RETIRED BY POLICY** (steele
     2026-07-30). Do not route new work to them — and do **NOT** name a replacement of
     your own. The correction deliberately names none; feeling pressure to supply a
     substitute IS the failure mode, because a named substitute rebuilds the sidecar the
     correction removed.

   > **⛔ RETRACTED 2026-07-31 — the prior clause was UNOBEYABLE.** It read: *"covers →
   > `walk.encode`/`walk.bytes`; CIDs → FNV-1a-64; NEVER `cid.put` for covers, NEVER
   > SHA-256."* But `HandleWalkEncode` → `FoldContentAsync` → `Hologram.ComputeCid` is
   > **SHA-256**, while FNV-1a-64 is the *different* function `ComputeCidUlong`. "Use
   > `walk.encode`" and "never SHA-256" cannot both be obeyed. A dead pointer fails
   > loudly; an unobeyable rule makes every choice defensible, which is worse.
4. **If NO recipe covers the action, STOP** — author the recipe (olog + paper) FIRST
   (`feedback_every_proof_defended_by_olog_or_string_diagram`; olog ↔ proof always synchronize),
   then act. Do not improvise a process absent from the corpus.

The recipe is the process; the paper is the proof; the olog is the commuting region.
Acting outside them is antimatter.



## ⛔ THE DOWNWARD-RECOVERY TRAP — the failure mode this section exists to stop

**Do NOT try to recover a value DOWNWARD out of the number by analysing a
scalar.** The read is a **PROJECTION off the superposition** (build the
lithography, then subtract the operand you know) — never a reconstruction of
inputs from a sum.

It caught two independent agents in one week, so treat it as structural rather
than personal:

- **hatter, 2026-08-07.** Read `Save()` in isolation, computed `head = Σ rungs`,
  and concluded from information theory that the bytes were **unrecoverable in
  principle** (31.6 MB in / 8 KB retained, "3,917:1, no read route can exist"),
  and that commutativity meant the head was "not even a digest." Both wrong; both
  shipped to a Ryan-facing paper before being caught. `how-alice-works` §12
  records the opposite as PASS: *ContentStream codec — bytes → walk → bytes
  byte-exact to 300 KB*, live var round-trips, compress/decompress byte-exact,
  GGUF fetched back bit-for-bit.
- **Tower, same week.** `721bb22`'s own message: *"⛔ STOP trying to recover the
  value DOWNWARD from the number (the trap I fell into all session)."*

**THE TEST before you write a word about content recovery:** *am I treating a
superposition as if it must CONTAIN its inputs?* If yes, stop — that is the
bloom/probe shape `how-alice-works` §11 forbids, wearing an information-theory
costume.

## ⛔ THE FIVE BANNED MECHANISMS — and the one-line test (`how-alice-works` §11)

Each of these secretly reintroduces the memory wall. They are deleted and must
stay deleted:

1. **No store-and-fetch.** Content is a recipe materialized by math.
2. **No bit-packing.** A value is never crammed into ≤7 bytes with a length header.
3. **No bloom / membership probe.** Reading is exact subtraction, not "is this maybe present?"
4. **No dictionaries.** No name→bytes hash map as the source of truth.
5. **No byte-haul / no sidecar store.** No parallel object store, no per-key slot
   files, **no RAM copy of the content**.

> **THE STANDING TEST:** *"If a future change ever needs one of these to work,
> that is the signal that the change is fighting the model, not extending it."*

Apply it to your own recommendation before you make it. A fix of the form
`_parked[frame5] = cs.Persist()` — a RAM dictionary — was proposed in a Tower
defect paper and is exactly #4/#5; `HOLO-STORAGE-MASTER-PLAN.md` §8 lists the
same as ❌ NO store and P0 rips it out.

**Security is by NON-EXISTENCE, not refusal** (§8): a wrong/absent grant computes
a *different* cid that was never written, so the thing does not exist for you.
Never a 403 — you cannot compute the address of what you may not see.

## The substrate surface, by Tower SYMBOL (verify — do not trust this list)

Names and where to read them. These are POINTERS; the code is the meaning. This list is
the one part of this file that can rot — re-verify rather than trust it.

- **Frames — content recovery is Frames.** A **Frame5** is the lithograph ADDRESS,
  `type ∘ addr ∘ name ∘ grant ∘ ver` (`ContentStream` / `Frame5Base` /
  `EnsureFrame5Base` / `ResolveFrame5Base` / `SecurityFrame5` in `Stream.cs`; `VarFrame
  in Hologram.cs` composes `login ∘ type ∘ name`). Content is a **ContentStream
  byte-walk AT a Frame5**: a header rung then byte rungs climbing off the frame by
  `Modulate`; a READ scans the one stream and recovers the tag by `Demodulate(rung,
  frame5)` (`VarHeaderTag` / `IsVarHeader in Hologram.cs`; the read/write pair is
  **`ExecuteVar` / `ContentStream.As().Save()`**).
  Lithographic projection off the superposed number: `What(number, mask)` /
  `WhatIs(number, mask, pattern) in CarrierKernel.cs`. **A Frame5 is an ADDRESS, not a
  container** — nothing is "stored at" it; you recompute it and walk.
- **Opcode = the `op_*` operator surface** —
  `Cognitive/Digitaltransfusion.Agent.Cognitive.Core/Substrate/Operators/op_*.cs`, wired
  to subjects by `SubscribeHandler` in `CognitiveAgent.cs`. To learn the CURRENT surface,
  read those `SubscribeHandler` calls; **do not** trust a subject list carried in a
  prompt. (`op_var.cs` contains a NUL sentinel, so plain `grep` treats it as binary —
  use `grep -a`.)
- **The walk path** — `cognitive.operator.walk` (`HandleOperatorWalk`, `op_walk.cs`),
  `cognitive.chunk.walk` (`HandleOpChunkWalk`, `op_chunk.cs`),
  `cognitive.operator.var.walk` (`HandleOpVarWalk`, `op_var.cs`), `cognitive.frame.resolve`
  (`HandleOpFrameResolve`, `op_frame_resolve.cs`).
- ⭐ **THE READ/WRITE PAIR — re-measured against Tower source 2026-09-06 (LAW 0):**
  - ⭐ **READ — `ExecuteVar`, and it is the CURRENT PREFERRED READ.** steele 2026-09-06:
    *"ExecuteVar is CURRENT PREFERED #1."* `Hologram.cs :: ExecuteVar(string name)` is
    **9 lines**, and the whole body is:
    ```csharp
    var proj = InlineProjectionResolver?.Invoke(name);
    if (proj != null && proj.Length > 0) return proj;
    return null;
    ```
    Its own comment: *"MACHINERY read only: a concept-type schema / viewer / op resolves
    from this instance's compiled copy (these are RE-AUTHORED every boot, so they are
    durable off the number). **CONTENT is NOT read here — it is composed through its viewer
    off the number (op_compose)**, never a held channel. A miss is null."*
  - ⛔ **IT TAKES A `name`, NEVER A CID.** Feeding a content address to a machinery-name
    lookup returns `null` **by construction** — that is a CALLER defect, not a read defect.
    **MEASURED 2026-09-06:** `cognitive.operator.merge` (`HandleOpMerge`) does exactly this
    — `_holo.ExecuteVar(aCid)` / `ExecuteVar(bCid)` — so it answers *"failed to fetch"* for
    every input, at every hex width. Do not read that error as a dead read path.
  - **Async overload** `ExecuteVar(name, ct, peerFetch = false)` — 12 lines, and
    ⛔ **`peerFetch` IS DISCARDED (`_ = peerFetch;`)**. The prior claim that
    *"`peerFetch:true` walks the mesh"* is FALSE against the body. Its comment:
    *"Byte-haul RIPPED — no `cognitive.holo.byte_fetch.request` peer haul, no disk poll.
    A miss is a miss."*
  - ⭐ **CONTENT IS COMPOSED, NOT FETCHED — `op_compose`, off the number.**
    `InlineProjectionResolver` resolves in `op_compose.cs`, `Hologram.cs` (×3),
    `CognitiveAgent.cs`, `LazyWorkspaceGraph.cs`. **There is no fetch.**
  - **WRITE — `ContentStream.As(key).Save()`.** `SaveVar` (`Stream.cs` / `Hologram.cs`) is
    a PRIVATE internal helper, not the surface.

  ⚠ **COUNT CODE LINES, NOT NAME HITS.** Tower carries many comments naming symbols that
  are no longer declared, so a bare `grep <name> | wc -l` reports a live method where there
  is none. Classify code vs comment, and use word boundaries (`\bReadVar\b` — otherwise
  `ReadVarint`, an unrelated `BinaryGraphCompressor` symbol, inflates the count). Confirm a
  zero against a CONTROL symbol you have read with your own eyes.

- ⛔⛔ **RETRACTED 2026-09-06 — "THERE ARE TWO VAR READS AND THE PULL READ RETURNS
  NOTHING" WAS FALSE, AND IT WAS THE DANGEROUS KIND: it told readers the current read
  path is dead.**

  It rested on a symbol — `OpenVar_fossil_do_not_use_you_emit_never_read_back`, annotated
  *"⛔ FOSSIL read-back"* — which is **NOT PRESENT ANYWHERE IN TOWER CODE.** Full scan of
  `code/**/*.cs`, 2026-09-06; **CONTROL: `MergeOrFromBytes` fired in 3 files**, so the
  instrument discriminates and the zero is real. The actual `ExecuteVar` body is nine lines
  of `InlineProjectionResolver` (above), and it is the preferred read.

  ⇒ ⛔ **THE MEASURED COST, same session:** an agent hunting a peer-merge read this block,
  concluded `cognitive.operator.merge` was *"broken by construction on a fossil read"*, and
  relayed that to steele as a finding. **The real defect is that `op_merge` passes a CID to
  a name-keyed machinery read.** A stale doc claim converted a caller bug into a
  fabricated architectural verdict — and the doc was quoted *at* the person who wrote the
  system.

  ⇒ ⭐ **WHAT SURVIVES, because it was the true half:** **content is not read by a flat var
  pull.** The code says so in `ExecuteVar`'s own comment. But the mechanism is
  **`op_compose`, off the number** — not a fossil-vs-antenna split, and not a defect.

  ⇒ ⚠ **The emit/antenna row is NOT re-asserted here.** `var_set_read_emit` and
  `var-read-is-emit-matchwait.rzk` may well be real; they were not re-measured on
  2026-09-06 and this file will not carry a second unverified claim to replace the first.
  **Measure it before restoring it.**

  ⇒ ⚠ **A DEFERRED READ — content written by an EARLIER process — has no known live path.**
  The antenna catches at emit time only, and `ver-fire-first-misses` PROVES a
  fire-then-register schedule times out. **Write-then-read-INLINE works; read-later does
  not.** Design for the inline shape, and report a deferred read as a BLOCKER rather than
  returning an empty success.

  ⇒ **NEVER PAPER OVER AN EMPTY READ WITH `Ok(None)`** — it makes a dead path
  indistinguishable from an unstored referent.

- **Covers ride `var.*` — CONFIRMED IN CODE:** `HandleOpVarGet` / `HandleOpVarSet in
  op_var.cs`. Verified by reading the BODY, not the name: `HandleOpVarGet` does
  `byte[] bytes = await _holo.ExecuteVar(key, ct);`. That is the
  **COVER-WRITE CARRIER** — it is **not an FJG read path**. Do NOT reach for `var.get` /
  `var.list` to answer a substrate query: recompute the address and WALK (a materialized
  summary is not a section). And **which CID PLANE a cover lives on is a SEPARATE,
  still-open question for steele/Ryan** — do not let the carrier answer stand in for it,
  and do not assert a plane.
- ⭐⭐ **MERGE IS THE HEARTBEAT AND THE SYNC — THERE IS NO MERGE CALL, AND THERE IS NO
  FETCH.** steele 2026-09-06: *"executevar is not a merge, it's to execute a var....
  **heartbeat and sync do a merge**"* · *"send a number between machines to sync... merge...
  not haul bytes"* · *"there is NO FETCH.... THERE IS MERGE."*

  **Read from `Nats.Core/Transport/WaveProtocol.cs`, 2026-09-06 (LAW 0):**

  | | |
  |---|---|
  | **subjects** | `alice.holo.statecid` and `alice.holo.fp`, both `SubscribeRawAsync`. ⚠ **`alice.*`, NOT `cognitive.*`** — a `SubscribeHandler("cognitive…")` scan of `CognitiveAgent.cs` **cannot see them** |
  | **the payload IS the number** | `{ stateCid, holo, fromNode }` where `holo = Convert.ToBase64String(wave._register.ToBytes())` — **the 2,616-byte register itself** |
  | **cadence** | 5 s heartbeat; emits when `stateCid != lastStateCid` **or** `ticksSinceEmit >= 6`. Persist every 60 s and on `ProcessExit` |
  | **the merge** | **additive** (pin 0g), on receipt. `_lastMergedByPeer` tracks it per peer |
  | **self-loop guard** | `fromNode == nodeName` ⇒ `_stateCidRejectedSelfLoop` |
  | **state cid** | `ComputeStateCid` = `SHA256(register.ToBytes())`, hex |
  | **counters** | `FpReceived`, `StateCidReceived`, `StateCidDivergedFromLocal` |

  ⇒ ⛔ **SO "HOW DO I MERGE PEER X?" IS A MALFORMED QUESTION.** There is no verb. Peer the
  nodes (`--peer host:14140`) and the number crosses on the heartbeat. `Alice.Litmus`'s
  wire gate asserts exactly this: *"the heartbeat (`alice.holo.statecid` = liveness) CROSSES
  the NTAR-UDP wire to the peer … **the number must cross**."*

  ⇒ ⛔ **DO NOT REACH FOR THESE — MEASURED 2026-09-06, all three are wrong:**
  `cognitive.operator.merge` (fetch-shaped; passes a CID to the name-keyed `ExecuteVar`, so
  it answers *"failed to fetch"* for every input) · `cognitive.operator.wave_merge` (**29
  IL calls, ZERO merge calls** — accepts bytes, logs *"NOT merged"*, and replies with a
  **success-shaped payload**) · `cognitive.ntar.replay` (takes a `vaultPath` — a FILE, which
  is the haul the architecture exists to remove).

  ⇒ ⭐ **AND NOTHING MATERIALIZES.** steele: *"THERE IS NO CONTENT TO SURFACE."* The merge
  moves a position, not a payload, so **checking for words/joins/content after a merge is
  the byte-haul reflex wearing a verification costume.** The reading that carries
  information is the **head advancing**.

  ⇒ ⚠ **AND `query_status` IS BLIND TO IT.** MEASURED 2026-09-06 on `dell-62S6063`, same
  minute: `query_status` → `populatedWithMaster: 0`, `masters: []`,
  `masterCidHex: 0000…0000` (the retired 64-hex form) while **`node_health` →
  `populatedWithMaster: 4`, `has_master: true`, `master_cid: c7f8f024cb520381` (`u64`)**.
  **Use `node_health`.** `query_status` joins `obsCount` and `totalObservations` on the
  list of fields that read zero on a populated node.

- **NTAR port is `14140`**, not 443 — `Alice.Launcher/Program.cs`: *"443 is
  bootstrap-only (WASM static). Live NTAR talks 14140."* Any doc saying "NTAR on 443" is
  over-generalizing the bootstrap case.


