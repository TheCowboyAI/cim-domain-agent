---
name: cim-substrate
description: Tower substrate mechanism reference for CIM work — the LAW 1 paper/recipe corpus (SUBSTRATE.md, FOUR-CATS.md, papers/ologs), and the Tower symbol surface (Frames/Frame5, the op_* opcode surface, walk paths, var.* cover carrier, NTAR port). Load this when an action touches a fold, a cover write, a CID, a walk or query, a store, or a symbol/word/language operation — i.e. whenever you need to cite a Tower symbol or follow a governing recipe.
---

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
   - **The register IS the storage.** Content folds into the one number and returns by
     SPINE WALK — literally `Demodulate(headAfter, from) => headAfter - from` in
     `CarrierKernel.cs`, inverse of `Modulate(head, frameCid) => head + frameCid`. There
     is no separate content-addressed side rail.
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
   (`feedback_every_proof_defended_by_paper_with_commuting_olog`; olog ↔ proof always synchronize),
   then act. Do not improvise a process absent from the corpus.

The recipe is the process; the paper is the proof; the olog is the commuting region.
Acting outside them is antimatter.



## The substrate surface, by Tower SYMBOL (verify — do not trust this list)

Names and where to read them. These are POINTERS; the code is the meaning. This list is
the one part of this file that can rot — re-verify rather than trust it.

- **Frames — content recovery is Frames.** A **Frame5** is the lithograph ADDRESS,
  `type ∘ addr ∘ name ∘ grant ∘ ver` (`ContentStream` / `Frame5Base` /
  `EnsureFrame5Base` / `ResolveFrame5Base` / `SecurityFrame5` in `Stream.cs`; `VarFrame
  in Hologram.cs` composes `login ∘ type ∘ name`). Content is a **ContentStream
  byte-walk AT a Frame5**: a header rung then byte rungs climbing off the frame by
  `Modulate`; a READ scans the one stream and recovers the tag by `Demodulate(rung,
  frame5)` (`VarHeaderTag` / `IsVarHeader` / `ReadVar` / `WriteVar in Hologram.cs`).
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
- **Covers ride `var.*` — CONFIRMED IN CODE:** `HandleOpVarGet` / `HandleOpVarSet in
  op_var.cs` call the live `_holo.ReadVar` / `_holo.WriteVar in Hologram.cs`. That is the
  **COVER-WRITE CARRIER** — it is **not an FJG read path**. Do NOT reach for `var.get` /
  `var.list` to answer a substrate query: recompute the address and WALK (a materialized
  summary is not a section). And **which CID PLANE a cover lives on is a SEPARATE,
  still-open question for steele/Ryan** — do not let the carrier answer stand in for it,
  and do not assert a plane.
- **NTAR port is `14140`**, not 443 — `Alice.Launcher/Program.cs`: *"443 is
  bootstrap-only (WASM static). Live NTAR talks 14140."* Any doc saying "NTAR on 443" is
  over-generalizing the bootstrap case.


