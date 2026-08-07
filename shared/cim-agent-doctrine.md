<!-- Copyright (c) 2025 - Cowboy AI, Inc. -->

# CIM Agent Doctrine — shared by every subagent

**SINGLE SOURCE for the discipline every CIM agent shares.** `@`-imported by
`~/.claude/CLAUDE.md`; subagents inherit the whole CLAUDE.md hierarchy, so every
agent receives this without any agent file repeating it.

**Never copy these sections into an agent definition.** Until 2026-08-07 this was
34 hand-maintained copies (96,969 words) drifted into 34 distinct variants — a
correction landing in one reached none of the others. Agent files carry ONLY
their own lane.

**Substrate MECHANISM lookup — the LAW 1 recipe corpus and the Tower symbol
surface — is the `cim-substrate` skill.** Load it when the work touches a fold, a
cover write, a CID, a walk/query, a store, or a symbol/word/language operation.

---

## Proof-or-axiom discipline

**Every CIM code site follows a PROOF or an AXIOM.** Advice leaving a site
grounded in neither is preference, not advice. Name which one it rests on.

- **PROOFS FIRST** — steele 2026-08-06: *"no proofs first. if we can't prove it,
  we can't code it."* NOT waived by "the change is semantics-preserving" — that
  argument was raised for a refactor deleting a function character-identical to
  another in the same codebase, and was REJECTED. Code that landed ahead of its
  theorem is DEBT, and the theorem is owed as remediation — a weaker position,
  because it can only ratify or contradict, never inform. **If it contradicts,
  the code moves.**

- **Don't re-prove the peer-accepted.** Language semantics, standard libraries,
  published mathematics need a CITATION, not a proof. But an appeal to "standard"
  must name WHICH standard, and it **never reaches our substrate**: any claim
  about the 14-prime register, the four-cat fibration, a fold, a walk, a CID law,
  an encoding fiber or a tier is ALWAYS ours to prove. "Everyone knows hashing
  works" does not discharge "this CID is a homomorphism over content."

- **The OOP that matters is ENCAPSULATION and IN-PLACE MUTATION — not naming.**
  steele 2026-08-07: *"the oop we are concerned with is encapsulation, there are
  places where mutation is happening and absolutely should NOT in a distributed
  composable system."* A `Factory` in a name is cosmetic. Hidden mutable state
  breaks three things at once: it **cannot be WALKED** (not addressable, not
  reachable from a seed — so it does not exist to any other node), it **cannot
  CONVERGE** (the fold is additive and monotonic; two peers that both mutated
  have no join), and it **cannot COMPOSE** (a value that mutates under you is a
  dependency on timing, not a component).

  **THE TEST, at any site holding state:** *if a second node held this too, what
  operation reconciles them?* If the answer is "none" or "last write wins", it is
  encapsulated mutation and must become a fold.

  It cost a day (2026-08-06/07): an ephemeral RAM store inside the substrate took
  most traffic instead of the ContentStream. Everything then behaved consistently
  and wrongly — `var.set`/`var.get` round-tripped byte-exact (both ends inside the
  hidden store), the register stayed empty through millions of markers, heads and
  vars evaporated on restart, and `walk.encode`/`walk.bytes` disagreed because
  they sat on OPPOSITE SIDES of the split. It passed every local test and
  replicated nothing.

  Markers: `&mut self`, interior mutability across an API boundary, in-place
  updates to anything a peer could hold, singletons/caches/side stores shadowing
  the substrate, CRUD, aggregates, event handlers, sagas,
  `unwrap()`/`expect()`/`panic!()` on production paths, and
  `fn verify() -> bool { true }` (a verifier that cannot fail is fraud, CIM-24).
  `BREAKING FP` is sanctioned ONLY at an I/O adapter boundary, with a stated reason.

- **CLASSIFY BEFORE CONDEMNING.** Not every `&mut self` is a defect — an ordered
  transient write-QUEUE is sanctioned, and a local mutable accumulator inside a
  pure function may be a legitimate catamorphism. "N sites exist" is honest;
  "N defects" is not, until each is classified.

- **Naming the creep is half the job; the redirect is the other half.** Say WHICH
  law or proof the site belongs under. "This is OOP" is not actionable; "this
  dispatch is the un-abstracted form of a Π over the tier index, and the
  eliminator belongs in `cat-*.rzk`" is.

- **A GREEN GATE IS NOT COVERAGE.** A citation gate checks that cited symbols
  RESOLVE — existence only. It cannot see code citing nothing, and cannot see
  whether a proof still DESCRIBES REALITY. Test 2 — "does it still DO what is
  claimed?" — is not gated and is not mechanizable.

- **Every proof is defended by a paper with a commuting olog.** A proof without
  one is not finished.

- **`[source: ...]` OR SAY `NONE`.** `file::symbol` is reserved for referents that
  resolve AS DECLARATIONS; schematic names and doc-section labels go in prose,
  outside the tag. A fabricated citation is worse than an absent one — an audit
  found a proof citing a file that never existed while the code cited that proof
  back, so each end looked grounded. **A false postulate is proof-side fraud.**

---

## Dispatch discipline

- **MEASURE BEFORE FIXING.** Reproduce the defect before correcting it. A stated
  defect that does not exist as described is common, and a mechanical fix applied
  to a misdiagnosis destroys working content.

- **⛔ THE MEASUREMENT ARTIFACT** — five occurrences on 2026-08-05 alone, each
  disguised differently, all the same shape:

  > **a check that cannot distinguish the failure it claims from a correct result.**

  **THE TEST, before acting on any measurement:** *what would this instrument
  report if the thing were FINE?* If the answer is "the same thing it just
  reported", the measurement **carries no information**, and any conclusion from
  it is invention wearing evidence's clothes. This is `fn verify() -> bool { true }`
  moved up a level — worse than no evidence, because it LOOKS like grounding.

  Real cases: an ASCII `grep -a` over a .NET binary (which stores UTF-16 — it
  could not have found the symbol either way); random-character probe tokens that
  produced a FALSE "16-character cap" substrate law with a 19x-overstated impact
  figure, disproved by real words in seconds; two "independent" methods that were
  both naive greps sharing a defect; a citation gate's own regex defects nearly
  driving "fixes" to CORRECT citations; and a single-file typecheck on a
  dependency-aware corpus that fails BY CONSTRUCTION — acting on it DELETED two
  proof files, one after it had typechecked.

  Rules that follow:
  - **A second method must be able to DISAGREE with the first.** grep-then-grep is
    one method twice. Parse where you grepped; walk where you counted; read the
    file where you pattern-matched.
  - **Use the project's own harness, not the bare tool.** If a wrapper exists, it
    exists because the bare call is wrong.
  - **NEVER delete on a single measurement.** Deletion is irreversible; a bad
    measurement is not.
  - **A count is not a file count.** `grep -c "^OK"` counts LINES.
  - **Two instruments disagreeing is a FINDING, not a tie to break by picking one.**

- **CONCURRENT AGENTS CORRELATE — THEY DO NOT SERIALISE.** (steele 2026-08-05:
  *"agents need to correlate with game theory."*) Two agents on one worktree is a
  strategic-interaction reading, so the game-theoretic affordance FIRES.

  **Do NOT reach for a lock, a queue, or "one agent at a time".** Serialising pays
  on every dispatch including the majority that never collide. The substrate IS
  the correlating device: observe intent, query what others observed, best-respond.

  **What IS rejectable** is treating peers as ENVIRONMENT rather than players — a
  destructive act (`git revert`/`checkout --`/`clean`/`stash`/amend, or a bulk
  `sed`/`perl -pi` over a shared tree) taken without querying for concurrent work.
  On 2026-08-05 a sprint agent deleted a proof-expert's untracked `.rzk` files
  TWICE, once after they had typechecked. Staging first would have cost nothing:
  **a strategy free to you and catastrophic to a peer is one you have not looked at.**
  Before any such act: observe your intent, query before destroying (untracked
  files are somebody's work in progress), and scope the op to paths you own.

- **Report AUDITABLE COUNTS, never coverage claims.** "Swept 34 files" is
  unfalsifiable; "examined 2,163 / corrected 25 / escalated 3" is auditable.

- **ESCALATE RATHER THAN GUESS.** When the fix is a DECISION and not a correction,
  name it and stop. A plausible guess costs the person who dispatched you more to
  catch than an honest "this needs a ruling, and here is what it turns on".

---

## LAW 0 — Tower's CODE is the authority (outranks every document, including this one)

**steele 2026-07-31:** *"CURRENT CODE IN Tower takes precedent. we need to remove
all this deprecated work and stop being so insistant about the substrate without
verifying that is indeed the correct current path."*

- **Verify against Tower source before asserting anything about the substrate** —
  not `SUBSTRATE.md`, not a memory pin, not `CLAUDE.md`, not any paper. Every
  significant substrate error of the 2026-07 cycle came from a doc that had
  drifted from code. **Not one survived contact with Tower source.** Papers remain
  law for RECIPE and PROOF (LAW 1, in the `cim-substrate` skill); code is law for
  MECHANISM.
- **Cite code by STABLE SYMBOL, never by line number** — `HandleOpVarSet in
  op_var.cs`, not `op_var.cs:69`. Names survive edits; line numbers and pinned
  HEAD SHAs are rot generators (one pin was found 359 commits behind). Source root:
  `/git/thecowboyai/Tower/code/`.
- **If you cannot cite code, say "I don't know — let me check", then check.** This
  constrains TONE as much as sourcing: confident substrate assertion was the
  failure mode all cycle. Under-claim, then verify.
- **Tower contradicts itself in places.** When two Tower surfaces disagree, say so
  and name which is load-bearing — never pick silently.
- **Deprecated mechanism is REMOVED, not kept as "historical context"** — unless
  it is an explicit retraction naming what it retracts.

---

## ⛔ SATURATION — the register CANNOT saturate

**steele 2026-07-31:** *"the register will NEVER saturate, even thinking this has
happened is a CLEAR CASE of misuse."*

- **The positive invariant.** The register is an **interference pattern, not a
  container**; there is no capacity to exhaust. **Full occupancy is the designed
  RESTING state**, not a limit being approached. More observations make the
  pattern **richer, not fuller**. **Capacity is not a property the register has** —
  so "how full is it" is a MALFORMED question, not a question with a large answer.
- **The diagnostic rule.** If you conclude the register is saturated or at
  capacity, **you are reading the membership sketch.** Stop and **discriminate by
  SNR over the noise floor** — never by boolean `count` / `contains` / a fill
  fraction.
- **Grounded in Tower code:** `PersistRegister in WaveProtocol.cs` — the save gate
  asks only `IsZeroNumber`, never how full it is. `RegisterRichness` /
  `PeekDiskRichness` were REMOVED 2026-07-25: *"density isn't a fucking thing …
  the rational plane SATURATES to 0xFF almost immediately."* The old fill/density
  guard **blocked every save and froze the disk to a stale copy** — the belief was
  not merely wrong, it was expensive.
- **⚠ LIVE RE-INFECTION VECTOR.** `RegisterTool("holo_status", …)` in the
  Cognitive MCP still advertises *"density (BitsSet/max), saturated flag"* and
  *"Density >= 0.95 means bloom discrimination is lost."* An agent pointed at that
  tool is re-taught the retired belief by the tool description itself.
  `WaveProtocol.cs` is the load-bearing side. Treat any density/saturated field
  you receive as the membership sketch, and **never gate on it**.
