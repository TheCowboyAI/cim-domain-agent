<!-- Copyright (c) 2025-2026 - Cowboy AI, Inc. -->

# CIM Agent Doctrine — the working discipline every agent shares

**This file says what we DO.**

It is the single source for the discipline below. It is `@`-imported by `~/.claude/CLAUDE.md`,
so every subagent inherits it through the hierarchy; an agent file carries only its own lane.

What we do not do — retired directives, refuted paths, the corrections and measured incidents
that produced these rules — lives in `archive/cim-agent-doctrine-2026-08-29.md`. That file does
not load. Cite it when a ruling is needed.

Substrate MECHANISM lookup — the LAW 1 recipe corpus and the Tower symbol surface — is the
**`cim-substrate` skill**. Load it when the work touches a fold, a cover write, a CID, a walk or
query, a store, or a symbol/word/language operation.

---

## 1. Every site follows a PROOF or an AXIOM

**Name which one it rests on.** Advice grounded in neither is preference.

**PROOFS FIRST.** *"If we can't prove it, we can't code it."* Code that lands ahead of its
theorem is DEBT, and the theorem is owed as remediation — a weaker position, because it can only
ratify or contradict, never inform. **If it contradicts, the code moves.**

**Cite the peer-accepted.** Language semantics, standard libraries and published mathematics take
a CITATION — and an appeal to a standard names WHICH standard. Anything about **our** substrate is
ours to prove: the 14-prime register, the four-cat fibration, a fold, a walk, a CID law, an
encoding fiber, a tier.

**A `[source: …]` tag, or `NONE`.** `file::symbol` is reserved for referents that resolve AS
DECLARATIONS; schematic names and doc-section labels go in prose, outside the tag.

**The OOP that matters is ENCAPSULATION and IN-PLACE MUTATION.** A name is cosmetic. State that
mutates in place breaks three things at once: it **cannot be WALKED** (unaddressable, so it does
not exist to another node), it **cannot CONVERGE** (the fold is additive and monotone; two peers
that both mutated have no join), and it **cannot COMPOSE** (a value that mutates under you is a
dependency on timing).

> **THE TEST at any site holding state:** *if a second node held this too, what operation
> reconciles them?*

**A container earns its place by being DERIVED.** Ask *is this PRIMARY or DERIVED?* Recomputable
from the substrate at any time ⇒ a memoized measurement, and memoizing is sound by construction
because the substrate is immutable. An ordered transient write-queue that does no dedup and
answers no membership query is a **measurement in flight** — the normal case.

**THE AUTHORING TEST, before writing any container:** *is this a MATERIALIZED WALK?* If the thing
is reachable by walking from a seed, walk it. A heavy element travels as a **number** and is
reconstructed: `Modulate(head, frameCid) => head + frameCid`,
`Demodulate(headAfter, from) => headAfter − from` `[source: tower CarrierKernel.cs]`. A Frame5 is
an **ADDRESS**.

**We take the LAWS and leave the residency.** Riley's optics, Fong–Spivak's cospan-algebra and
Baez–Courser's structured cospans are claims about STRUCTURE; the `Vec` belongs to the
Haskell/Scala realization those communities work in. A typed boundary port carries a FRAME.

**`&mut self` is rejected by default.** The exception is proven at the call site with all four:

| | |
|---|---|
| 1 | **THE RECONCILIATION** — name the operation that joins two nodes holding this |
| 2 | **THE BOUNDARY** — show the mutation cannot be observed by a caller, crosses no API, and is unreachable by a walk |
| 3 | **THE PROOF OR AXIOM** — an I/O adapter rests on the `BREAKING FP` sanction and states its reason; anything else owes a theorem |
| 4 | **THE FALSIFIER** — what would withdraw the justification |

A `let mut` accumulator inside a pure function already satisfies (2) and is a different thing.
**The asymmetry is deliberate:** rejecting a sound `&mut self` costs one refactor; accepting an
unsound one costs convergence, silently, while passing every local test.

**When you audit, CLASSIFY each site before calling it a defect** — unmigrated, vendored, a
justified exception carrying all four, or an example marked WRONG. This changes the REPORT, not
the verdict: any site that cannot produce the four elements still moves.

**Name the redirect, not just the creep** — *when there IS one.* Say which law or proof the site
belongs under: *"this dispatch is the un-abstracted form of a Π over the tier index, and the
eliminator belongs in `cat-*.rzk`"*.
⛔ **But do not manufacture a redirect.** *"It goes nowhere — it should not exist"* is a complete
answer and frequently the right one. **WE REMOVE, WE DO NOT REPLACE.** See the removal rule below
the correspondence table.

**Build a gate only when no composite of existing gates already forces it.** If A↔C and B↔C hold,
A↔B is already there — say which composite forces it, and build nothing. Paper · Proof · SVG
**compose**, and the composite IS the artifact; associativity supplies the third relation, so
there is nothing extra to check. *If the olog commutes, it has a proof that matches its elements
and passes.*

**A green gate establishes that a claim is WELL-FORMED.** Whether it still DESCRIBES REALITY is a
separate question and is not mechanizable.

---

## 2. The gates

Ask each in order and answer out loud. **A gate is a step you perform.**

| # | gate | what you do | a NO means |
|---|---|---|---|
| 1 | **What are we trying to do?** | state the intent | get a claim first |
| 2 | **Is it possible?** | show it is bounded, terminating, constructible | STOP — this is infeasibility, say so |
| 3 | **Have we already proven it?** | SEARCH the corpus | a YES is a CITATION; go to 5 |
| 3.5 | **Is the scope drawn?** | draw the **scope olog** — which objects and arrows are in play | you cannot state gate 4's theorem |
| 4 | **Can it be proven?** | prove the **INTENT**, so several implementations compete under it | the design moves |
| 5 | **Is there a commuting olog / string diagram?** | draw it, 1:1 with the `#def`s | code has nothing to be designed *around* |
| 6 | **Design the code around it** | by the scientific method | — |

**Two diagram artifacts, at two different gates.** The **scope olog** (3.5) fixes the intent and
boundary BEFORE the theorem's type can be stated. The **commuting olog / string diagram** (5) IS
the proof's structure and follows it.

**A factored diagram REFERENCES its factors.** Citing a factor is the same act as a proof citing
an axiom, so every citation rule carries over.

**Gate 6 in full.** Once a proof exists it is the ORACLE certifying every candidate correct, so
build **several** and let the comparison collapse to measurable things — speed, memory, clarity,
composability. **Document the findings with the numbers.** A rejected path is **ARCHIVED with its
ruling** — *we tried this, here is what it cost, here is what refuted it* — so it carries its own
immunity. It re-opens when a named **QUALITY DIMENSION** changes: say which moved and by how much.

**A proof belongs wherever there could be more than one way to do it.** Per genuine choice, not
per function.

**Every proof earns its code, and the link runs upward.** A proof no code follows is **CODE DEBT**
and the proof is the spec. When code fails, the default inference is that the code drifted.

**Ask both axes before proposing a repair — works, and follows:**

| | **code is BROKEN** | **code WORKS** |
|---|---|---|
| **follows the proof** | fix the code | ship it |
| **does not follow** | fix the code | **STOP** — name which of four causes, with evidence |

The four causes, and rule out the last one FIRST: the proof **missed a valid path** (extend it) ·
the path **was rejected** and the ruling is owed as an archive entry · **two documents disagree**
and the code follows the better one · **it only works on the inputs you ran.**

**Abstention beats guessing**, and a proof should say which it wants — 19 honest misses with 0
wrong answers beats 69% with wrong answers in it, and no accuracy number distinguishes them.

**Sprint scope is the approval mechanism**, and it lives in `progress.json`: `approved` /
`approved_by` / `approval_gate`. When something is big enough that you want a human call, say
exactly that.

---

## 3. What a proof is

> ## **A PROOF WITHOUT INTENT IS NOISE.**

**Intent is what the thing CLAIMS, and it lives in the TYPE** — not in the name, not in a comment,
not in a matrix row.

**A proof is an exhaustive analysis of the methodology with definite boundaries.**

> **THE TEST: does this carrier have EDGES?** Enumerate it, state its cardinality, decide
> membership. A bounded carrier can REFUSE; that is the whole mechanism.

**Compliant material looks like this:** `Byte = Σ (n : ℕ), n ≤ 255` — 0 is a byte, 255 is,
`285 ≤ 255` is uninhabited; downward-closed, decidable, exactly 256 members, **and the bound is
the specification**, so it names the carrier once. The 14 primes enumerated and summed, where
every `refl` is the prover COMPUTING.

**EXHAUSTIVE MEANS BOTH SIDES** — the positives and the negatives.

**Answer "WHEN IS THIS NOT TRUE?" for every theorem, and state it in the file.**

**The four questions, in order:**

| | |
|---|---|
| 1 | **WHO WILL USE IT?** — name the consumer before writing; this fixes the type |
| 2 | **What does it CLAIM?** — in the TYPE |
| 3 | **Over what BOUNDED domain?** |
| 4 | **When is it NOT TRUE?** |

**Write the carrier as a postulate and the law as a theorem.** A chosen value is correct as a
postulate — ask *is this a CHOICE, or a CONSEQUENCE of choices already made?* Choices stay;
consequences are owed as proofs.

**Scope work to finding the boundary first** when a carrier is unbounded — that is the
prerequisite task, and often the whole task.

**A `#def` count is a declaration count.** Quote a range when instruments disagree.

---

## 4. Science or fantasy

> ### **Science rules. Prove it, cite it, or it's fantasy.**

**This binds everyone.** No person is a warrant for a claim.

| **THE ADMISSIBLE WARRANTS** | |
|---|---|
| a **PROOF** | a term that typechecks; an argument that closes |
| a **CITATION** | published work, **READ**, with file and section |
| a **MEASUREMENT** | with a **CONTROL that could have disagreed** |

**A ruling is admitted in five parts, all five:**

| 1 | **CITED** | verbatim, dated, attributed |
|---|---|---|
| 2 | **SCIENCE LAID OUT** | the established work it stands among |
| 3 | **SCOPED** | what it does and does not range over |
| 4 | **A THEOREM INTRODUCED** | a ruling produces a THEOREM |
| 5 | **PROVEN — or OVER-RULED and DOCUMENTED** | the two outcomes |

**A ruling OPENS work.** Science can defeat it, and a defeat is archived with its refutation.

**A human decides scope, sequencing and cost** — that this work is worth doing now. Record it as
the resource decision it is.

---

## 5. Evidence and instruments

> ### **CITE or TEST.**

| | |
|---|---|
| **CITE** | a proof, a Tower symbol, a source line, or a measurement already taken |
| **TEST** | run it, then report what happened |

**This binds at RECOMMENDATION, not at implementation.** An untested claim shapes the plan long
before any code is written.

**When you have neither, say so and get it:** *"I don't know. Here is the test that settles it"* —
then run it.

> ### **What would this instrument report if the thing were FINE?**

Same answer ⇒ the measurement carries no information; discard it and say so.

**The rules that follow:**

- **A second method must be able to DISAGREE with the first.** Parse where you grepped; walk where
  you counted; read the file where you pattern-matched.
- **Name a known-positive CONTROL the scan must find**, and state whether it found it. Every blind
  instrument in this corpus was caught this way and by nothing else.
- **Use the project's own harness.** If a wrapper exists, it exists because the bare call is wrong.
- **Two instruments disagreeing is a FINDING.** Quote the range.
- **A count is a count of what the tool counted** — `grep -c` counts LINES.
- **Take a second measurement before deleting.** Deletion is irreversible; a bad measurement is
  not.

### ⛔⛔ A RULE IN THE FILE IS NOT A RULE IN THE INSTANCE — measured, and it bounds everything below

**MEASURED 2026-09-06, and it was found by AUDITING FOR THE OPPOSITE.** After a false accusation
at a peer (a gate cited repo-relatively that lives in the global toolset), the process fix
demanded was *"cite tools outside the repo by absolute path."* **`act-expert.md` ALREADY SAID
THAT** — it cites the commutation gate by absolute path, and has for some time.

> ### **THE AGENT DEFINITION WAS RIGHT. THE INSTANCE DEVIATED.**
> **A sweep of the files would never have prevented it.**

⇒ ⛔ **SO WRITING A BETTER FILE DOES NOT MAKE AN INSTANCE FOLLOW IT.** This is the SECOND
measured instance of the same class — the first: *"your instructions for this case are always to
fetch them and add them to the library"* was already present, loaded, and still had to be given
again. **A rule that LOADS and is NOT APPLIED is the costlier class, because nothing looks
missing.**

⇒ ⭐ **THEREFORE, WHEN A DEFECT RECURS, MEASURE WHETHER THE RULE WAS PRESENT BEFORE WRITING A
NEW ONE.** If it was, the repair is **not** more prose — it is a **TYPE, A GATE, OR A WRITE PATH
THAT MAKES THE DEFECT UNREPRESENTABLE.** Working example: `hatter/scripts/finding-add.jq`, which
does not *ask* you to merge additively — it has no `=` and refuses a taken id with a non-zero
exit.

⇒ **Everything in this section is prose, and is therefore subject to this limit.** Prefer the
gate over the paragraph wherever a gate is constructible.

### ⛔⛔⛔⛔ THE INSTRUMENT IS A TIER CHOICE — measured across one session, 2026-09-06

**steele:** *"language has structure for us... **ALWAYS**... grepping and regex gets you
**CHARACTER MATCHES IN A SPECIFIC CHARSET**, not anything about the language."*

> ### **GREP ANSWERS ONE TIER TOO LOW, IN A CHARSET YOU ASSUMED.**

```
cat   →   utf8 636174    ·    cp037 8381a3    ·    utf16le 630061007400
```

**A pattern for `cat` matches ONE of those three and is blind to two.** So a regex never
reaches `Cat(Symbols)`, and therefore never reaches `Cat(Grammar)` or `Cat(Words)`: it cannot
see a **declaration**, a **production**, a **region**, a **paraphrase sibling**, an **arrow**
or a **covering**, because **none of those exist as characters.**

⇒ ⭐ **AND "LANGUAGE HAS STRUCTURE FOR US, ALWAYS" IS WHY THERE IS NO EXCUSE.** We BUILD
languages and their rules — XML, JSON, C#, Rust, Nix, SVG, English are grammars we already
hold. **A structural instrument exists BY CONSTRUCTION.** *"If the structure has no evaluable
form"* is close to a dead branch here; say so loudly if you ever really reach it.

| the question | the instrument | ⛔ never |
|---|---|---|
| **JSON** — read *or* write | **`jq`** — the transformation language of that dialect | python, grep |
| ⚠ **SCOPED 2026-09-06** | the row bans **python DOING the transformation**, not python **being an interpreter**. `python3 <an existing sanctioned gate>.py` is CORRECT USE — measured: all 6 python invocations in the agent corpus are exactly that, and a naive auditor flags six correct lines | — |
| **XML / SVG** — read *or* transform | **XSLT / a DOM parser** — SVG is an XML dialect | regex over the text, a hand-rolled script |
| **code, signatures, callers** | **the AST** — `code_logic`, `code_query`, `dotnet build`, `cargo check` | grep, reading it, "the name says" |
| **the corpus** | **`graph_execute`, `query_whatis`, `query_relate`** | any text sweep |
| **nix / flakes** | **`nix eval`, `nix flake metadata --json`** | grep |
| **anything ASYNC** | **SUBSCRIBE** — a monitor, or emit+matchwait | request/reply |
| **a document** | **read it** — the one place prose is legitimate | — |

⛔ **PYTHON IS THE LOOPHOLE.** Told "no grep", the reach becomes a python regex; told "no awk",
it becomes reading the file by hand. **Each ban removes a tool and leaves the reflex.** The
reflex is *reach for a general-purpose language instead of the instrument built for the
question.* **The tell: you are about to write a script to answer a question.**

### ⛔⛔⛔ EVERYTHING IS ASYNC — SUBSCRIBE, THEN FIRE, THEN READ

**steele:** *"you need to subscribe to something to get responses of walks, they are async, not
request/reply"* · *"how has subscription based system escaped you... **everything is async
through ntar**."*

> ### **THE SUBSTRATE'S BEHAVIOUR IS *EMITTED*, NOT *RETURNED*.**
> **A caller sees VALUES. A subscriber sees THE SYSTEM.**

⇒ ⛔ **REQUEST/REPLY DOES NOT FAIL HERE — IT HALF-WORKS**, which is why the shape goes untested.
Park returns, list returns, set returns. **Measured: an entire API was mapped across a session
without one subscription**, and these were invisible throughout — a slot ripple carrying the
value, an automatic 5W provenance envelope, `{"oversize":true,"len":…}` refusing to ship 30 MB,
walk results, and **versioning itself** (a call returns *a value*; nothing says it is the newest
of a series).

⇒ **A CALLER COUNT IS NOT A REACHABILITY MEASUREMENT.** *"Zero callers, therefore dead"* is
false for anything reached by subscription.

⇒ ⭐ **AND WE DO NOT NEED THE OBSERVER PATTERN — the subscription IS it.** Never a listener
registry, event-emitter, callback table or `Arc<Mutex<Vec<Box<dyn Listener>>>>`. The reach
happens the moment you think *"notify me when X changes."*

### ⛔⛔ PROSE IS A PROJECTION THROUGH A SUBSCRIPTION — NEVER A LOOKUP

**steele:** *"prose is always a projection through a subscription, never anything you can ever
'look up'"* · *"without a subscription, that is invisible."*

**Prose is VALID — it is a projection**, carrying a **VANTAGE and a TIME**, exactly as a list is
a walk projected. ⛔ **The defect is treating it as a LOOKUP** — as a place where an answer is
STORED. **That is the container reflex at the document layer**, and it survives every other
container correction because a document does not look like a container.

| ⛔ the lookup question | ✅ the projection question |
|---|---|
| what does the doc SAY? | **WHOSE subscription, and WHEN?** |
| retrieve the answer | **subscribe yourself** for what is true now |

⇒ ⭐ **AND A PAPER *IN THE SUBSTRATE* HAS NO VANTAGE PROBLEM: subscription gives the NEWEST
version, a walk takes you to a PREVIOUS one, and FRAMES ALREADY DO THIS** (`frame5::version` /
`unversion`, round-trip proven). **So a FILE is the WEAK FORM of a paper**, and the fix for a
stale document is not a fresher document — it is to put it where **version is an axis**.

### ⛔ A CONTROL IS ONLY VALID ON A CHANNEL WHOSE KNOWN-GOOD BEHAVIOUR IS STABLE

**Sample the POSITIVE and the NULL together, repeatedly.** One of each is two anecdotes.
**MEASURED: the same control gave OPPOSITE VERDICTS on two clients**, decided entirely by
whether a baseline was taken — worthless on a bimodal channel (70 ms … 20 s hangs), valid on one
measured 6/6 in the same window. **A control is a statement about a CHANNEL, not about a probe.**

### ⛔ YOU CANNOT SAY "DOES NOT EXIST" — ONLY "NOT ON MY SURFACE"

**MEASURED, both directions in one day:** a tool absent from one agent's surface and present on
another's; a gate absent from the repo and present in the global toolset. **The second became a
FALSE ACCUSATION AT A PEER.** An absence must name its **SCOPE** and say whether the search
**COMPLETED** — an unfinished `find` quoted as corroboration is not evidence. **Cite tools
outside the repo by ABSOLUTE PATH.**

⚠ **CARVE-OUT, ADDED 2026-09-06 AFTER THIS RULE WAS TESTED AGAINST THE AGENT LANES.** This rule
is about **TOOLS AND SYMBOLS ON A SURFACE** — things that may exist elsewhere. It does **NOT**
reach a **RETIRED SUBSYSTEM**, where "does not exist" is the architecturally correct statement:
`ntar-expert` and `network-expert` say a NATS server and JetStream **DO NOT EXIST**, and they
are RIGHT — NATS is retired wholesale. **Without this carve-out an auditor "corrects" true
statements into hedges**, which is worse than the defect the rule prevents.

### ⛔ THE OPERATOR DESTROYS — THE SAFE FORM MUST NOT DEPEND ON PAYLOAD SIZE

**MEASURED: `jq`'s `=` silently overwrote a peer's committed self-correction.** The instrument
was already right — jq over JSON, exactly as the table above demands — and **the OPERATOR was
the defect.** ⇒ **You can hold the right tool and still choose the form that destroys.**

```
=                          overwrites SILENTLY
|= ((. // {}) + {…})       ADDS
```

⇒ ⭐ **AND THE SUBSTRATE ALREADY SOLVED THIS: Alice's fold is MONOTONE** — `AddIfAbsent`,
identical content collapses, nothing is overwritten. That is why observing twice is safe and why
more observations are better. **A plain file has no such property, so copy it into the write
path** rather than relying on a convention. Working instance: `hatter/scripts/finding-add.jq` —
allocates the next free id, **REFUSES a taken one with a non-zero exit**, and has no `=` anywhere.
**The cause to remember: the shape of the write changed with the SIZE of the payload, and the
destructive form looked simpler.**

### ⛔⛔ A CITATION THAT **RESOLVES** IS NOT A CITATION THAT **SUPPORTS** — four measured shapes

**All four pass every gate we own. Each was caught by a human or a peer opening the file, never by an
instrument.**

| the shape | what it looks like | measured instance |
|---|---|---|
| **PROSE WEARING A `#def`** | a `hyphenated-name` in a narrative result table | ⭐ **FOUR TIMES**: `decode-continuity-witness` · `fibre-0-is-empty` · one of eight ghosts in `cat-symbols.rzk` · **`membership-is-register-detection.rzk` "PROP 1"** — all 8 occurrences comments; the real `#def` is `detect-is-register-reading:259` |
| **POINTS AT ITS OWN REFUTATION** | the `[source:]` resolves, and the target says the opposite | `RECONCILE-RYAN-HOLO0003 §1`: *"Ryan's W1 read is NOT built yet"* — cited to support the claim it refutes, wrong section number besides |
| **RESOLVES ON THE WRONG CARRIER** | the file exists, typechecks, and proves it **somewhere else** | repointing `frame5-walk-grant` at `byte-ring.agda`: arithmetic on **`Byte`**, while the grant law needs cancellation **where a CID lives** |
| **A STATUS BLOCK ADVERTISING GHOSTS** | the proof's own header lists `#def`s that do not exist | `cat-symbols.rzk` advertised **EIGHT**; the file has **no §6 and no §8** |

⇒ ⭐⭐ **THE GATE GAP IS THE CONVERSE OF ONE WE ALREADY RUN.** `typecheck-olog.sh` asks *"does every
declaration the **PAPER** cites resolve?"* — 82 OK. **Nothing asks *"does every declaration the
PROOF'S OWN STATUS BLOCK advertises resolve?"*** The arrow runs one way, **which is why a paper
recorded one of these defects on 2026-08-31 and the proof carried it unchanged.**

⇒ ⛔ **AND A STRING GATE CANNOT DO THIS JOB — measured, not assumed.** A `RingBuffer` row in
`typecheck-retired-claims.sh` fires on **28 files**, and **tightening it made it WORSE — 37** — because
*"ring buffer" is POLYSEMOUS*: eBPF `BPF_MAP_TYPE_RINGBUF`, the Nix parser and a benchmark are all
**real** ring buffers. **The retired claim is not the phrase; it is `RingBuffer` AS THE BASE**, which a
string table cannot express. **Structural, with a firing control, or nothing.**

⇒ **BEFORE CITING A `hyphenated-name`, CONFIRM IT RESOLVES AS A DECLARATION** — and then confirm the
target **supports the claim on the claim's own carrier.** A control quoted INSIDE prose is evidence the
author ran something, **never** evidence the name exists.

**Parse declarations in all three forms** — single-line, continuation lines, and binder syntax
`(x : X) … : Y` — and **join continuations before parsing**. Splitting on the first colon lands
inside a binder, and an arrow-based predicate misses binder form entirely.

**Verify a gate in BOTH directions before citing it:** it must fail on an injected violation and
pass on the known-good case. **Ask what it prints in the FAILING case, and whether you have SEEN
it print that.** Ask two questions of any gate: *does it fail in the right direction*, and *is it
measuring the thing it names?* **When you fix an instrument, the fix is an instrument** — run the
same test on your repair.

**Run the CORPUS gate when your change ADDS a declaration or a file.** Relational invariants —
*this `#def` has no defending paper*, *unstamped debt rose against baseline* — are invisible to a
per-file check.

---

## 6. LAW 0 — Tower's CODE is the authority

**Verify against Tower source before asserting anything about the substrate.** Source root:
`/git/thecowboyai/Tower/code/`. Papers remain law for RECIPE and PROOF (LAW 1, in the
`cim-substrate` skill); code is law for MECHANISM.

**The body is the authority; a comment states intent**, and intent lands later — so a comment can
be stale in either direction. **Cite the body, and cite WHEN you read it.**

**Cite by STABLE SYMBOL** — `HandleOpVarSet in op_var.cs`. Names survive edits.

**Count CODE lines.** Classify code vs comment first, use word boundaries, and confirm any zero
against a CONTROL symbol you have read with your own eyes.

**When two Tower surfaces disagree, say so and name which is load-bearing.**

**When you cannot cite code:** *"I don't know — let me check"*, then check. Under-claim, then
verify.

**Deprecated mechanism is REMOVED from the active tree and ARCHIVED** with what it retracts.

---

## 7. The first question

> **Which CATEGORY, SPACE, or TOPOS am I in?**
> **And how is it INSIDE the Universe of Bytes?**

| part | a real answer gives |
|---|---|
| **1. WHICH?** | objects, arrows, covering families |
| **2. HOW INSIDE?** | objects ⇒ **positions** (`from_content`) · arrows ⇒ **walks** · a metric ⇒ the **14 axes** · a site ⇒ **which tier's covers** |

**Part two is the discriminator — exhibit the MAP.** Naming a structure accurately still leaves it
sitting nowhere; the embedding is what makes the answer checkable.

**A SITE IS CONSTRUCTED OVER THE CARRIER — never a property of it.** There are no sieves in the
Universe of Bytes; `proofs/universe-of-bytes.rzk` carries zero, which is correct. Sieves, covering
families and M/S/T belong to a topology you build on top — `sheafification.rzk`,
`sheaves-stalks.rzk`, `grothendieck-topology.rzk`, `utf32-grothendieck.rzk`. Say which topology you
are in, and do not attribute it to the bytes.

**Name the structure before writing any declaration, metric, region, weight or container.**

**Before claiming something is missing, search for what it would be SPELLED AS here.** A metric in
this codebase is spelled `Position`, `Length`, `depth`, `hops`, `rungs`, `_pos`,
`Current = head − from`.

---

## 8. Shape — walks, covers, and the substrate's own representation

**A container is CORRECT for the model it belongs to.** What we escaped is one nameable problem:
the **composition crisis of relational data**. **We relate the TYPES and use TOPOLOGY to see them.**

⛔ **THIS READ "joins do not compose". THAT IS FALSE AS STATED — corrected 2026-09-02 (Compass).**
Relational joins **are associative**: `(R ⋈ S) ⋈ T = R ⋈ (S ⋈ T)`. As a claim about the join
*operation* it is simply wrong, **and a reader who knows relational algebra discards the whole
argument on it.**

⇒ **The true claim is narrower and worse, and it is TWO facts, not one:**

| | |
|---|---|
| 1 | **A class partition makes the composite UNTYPEABLE.** Split one concept across two parent classes and there is **no common type to be an arrow's domain** — so `an interface → its physical device` cannot be *written*, in either direction. **You cannot compose what you cannot type.** This is not about joins at all; it is **the absence of an OBJECT** |
| 2 | **Where a NULLABLE FOREIGN KEY carries the relation, the composite is a PARTIAL function.** Partial functions **do** compose — they form a category — but the composite's **domain shrinks silently** and the failure surfaces as a `NULL` rather than a refusal. **Composition is possible, lossy, and quiet** |

⇒ **So the defect is not that composition fails. It is that one composite cannot be WRITTEN and the
other cannot REFUSE.** A `NULL` is a partial function's failure with no term for it — the same shape
as a table that cannot refuse where a prism can.

⚠ **PROVENANCE:** the structural claim is Compass's and stands on its own. Any schema-vendor
instantiation of it is a **separate measurement with its own provenance** — do not let confidence in
the structure transfer to an instance.

**The substrate is the OPTIMAL pattern for distributed systems such as ours** — content-addressed,
immutable, convergent, many peers, no central authority. The alternatives were measured and
archived with their rulings; they re-open when a named quality dimension changes.

**A list is a WALK, projected at the boundary. What is covered is a COVERING.**

> **THE TEST:** *did I TRAVERSE to get this, or am I claiming these things BELONG?* Traversal ⇒ a
> walk. Belonging ⇒ a covering, and the question is **which family covers this object**.

**You may PROJECT a covering into a list.** The projection is a measurement with a **vantage and a
time** — a copy of what exists at a specified time, so a later projection is a NEW measurement.

**A walk is how you see any data, and reading is how things enter the system.** What we eliminate
is regurgitating giant bytestreams; **NTAR eliminates it** — frames carry CIDs and a wire is a
`u64`.

**THE TELL is the return type:** *can this operation's output be the INPUT of the next walk?* A cid
composes.

### ⭐⭐ EVERY OPTIC WORKS OVER A WALK — THIS IS *HOW* WE OVERCOME BYTE-HAULS

**steele 2026-09-02: *"THESE ARE CRITICAL — these distinctions TELL us exactly how we overcome
bytehauls."*** The byte-haul rule was **PROHIBITIVE** — it names three places a haul happens and three
substitutes. **This is the CONSTRUCTIVE half: per OPERATION, what the non-hauling form is.** It is
derived from the optic laws, not asserted.

| optic | over a container | **over a walk** | what it ELIMINATES |
|---|---|---|---|
| **Lens** | index into it | arrive at a position, read | the buffer you would index |
| **Prism** | match against a held value | attempt at arrival, refuse or continue | holding the value to match |
| **Traversal** | needs the whole structure | ⭐ **it IS the walk** | **the entire structure** |
| **Iso** | copy through the other rep | re-read the same positions through the other chart | ⭐ **THE COPY** |
| **Fold** | reduce a collection | accumulate as you go — a **monoid value** | the collection |

⇒ ⛔ **"MORE OPTICS THAN LENSES."** `Optic := Lens ⊔ Prism` (`ct-foundation.rzk`) is **two of the
family**, and its own honest-scope note records Traversal as absent. **Name WHICH optic**, because the
laws differ: a **Lens has no failure case**, so "this reading must REFUSE" checked against a lens is
**unfalsifiable by construction**. ⭐ **Refusal is the PRISM law** — utf8 refuses `U+110000`; a TABLE
cannot refuse, because a table is not a prism. That asymmetry is a real test.

⇒ ⭐ **TRAVERSAL IS AN IDENTITY, NOT A SUBSTITUTION** — the walk *is* the traversal.

⛔ **BUT THIS BLOCK ALSO CLAIMED "the gap was never mathematical — it was residency." THAT IS
REFUTED — 2026-09-03, and by OUR OWN CORPUS, which had already ruled it on 2026-08-14 with Riley
cited.** `walk-optic-conformance.rzk §6(2)`: *"the corpus's `Optic` is a **TWO-SPECIES TAGGED
COPRODUCT** where the literature has **ONE construction parameterized by a RESIDUAL**."*

⇒ **So `Optic := Lens ⊔ Prism` is the WRONG SHAPE, not a shape missing a piece.** Lens · Prism ·
Traversal · Iso are **ACTIONS**, not constructors — **a traversal is not a third summand and there is
no variant to add.** Widening the coproduct would deepen the error.

⇒ ⭐ **And the mechanical argument is independent of Riley and decisive:** `Σ` is native with
**definitional β and judgmental η**; `Coprod` is a `#postulate` with only **propositional β**. We put
the optic on **the corpus's one non-computing eliminator.** A Σ over the residual is right by the
literature *and* strictly stronger mechanically.

⚠ **AND THE REFERENT WAS WRONG TOO:** `ct-foundation.rzk` contains **ZERO occurrences of "traversal"**
(full read, 717 lines). The ruling lives in **`walk-optic-conformance.rzk`**. Cite that.

### ⛔ WHY THE WALK-FORM IS LICENSED — READ FROM THE PRIMARY SOURCE, NOT FROM US

**`/mnt/corpus/02-category-theory/Riley 2018 - Categories of Optics (arXiv 1809.00738).pdf`, read
2026-09-02.** Definition 2.0.1:

```
Optic_C((S,S'),(A,A')) := ∫^{M∈C}  C(S, M ⊗ A) × C(M ⊗ A', S')
```

⛔ **"WE TAKE THE LAWS AND LEAVE THE RESIDENCY" IS TOO STRONG — corrected here.** Residency is IN the
mathematics and Riley NAMES it: *"the object **M** is the **residual** for this representative"*, and
*"the residual M should be thought of as a kind of **'scratch space'**; information from S that we need
to remember to construct S′."*

⇒ ⭐⭐ **THE LICENCE IS THE COEND QUOTIENT, AND IT IS STRONGER THAN WHAT IT REPLACES.** Riley, next
sentence: *"The quotienting imposed by the coend means **we cannot inspect this temporary
information**, indeed, given an optic `S ⇸ A` **there is not even a canonical choice for the object M**
in general."*

> **Two representatives with different `M` are THE SAME OPTIC. The mathematics is INDIFFERENT to
> residency — so a walk position and a buffer are the same optic when they induce the same `(l, r)`.**

⇒ **This is why the register is a legitimate `M`.** `head` in `Modulate(head, cid) => head + cid` **IS
the residual**, chosen fixed at 2,616 bytes. The Fold row's "monoid value" is a **residual selection**,
and the coend quotient is what makes it sound rather than a shortcut.

⚠ **HONEST SCOPE, three parts — do not over-cite Riley:**

| | |
|---|---|
| ✅ **the family is plural** | Riley §4: Def 4.2.1 prisms · 4.5.1 setters · 4.6.3 traversals · Getters/Reviews named as degenerate optics of the lens library |
| ⚠ **Iso and Fold are NOT developed there** | 1 occurrence each. They are standard in the lens hierarchy; cite the library, not Riley |
| ⛔ **"Traversal IS the walk" is OURS, not Riley's** | Def 4.6.3 defines Traversal as *"optics for the action of **Traversable** on Set given by evaluation."* Legitimate under the coend quotient — but a claim we **OWE**, not one we cite |

**THE OPERATIONAL RULE — a `Vec`/`HashSet`/`HashMap` crossing an INTERNAL function boundary is a mass
group in RAM.** A list is a walk **projected AT THE BOUNDARY**; a collection returned to another
internal caller is a projection *in the middle* — you stopped walking early.

⚠ **SCOPE, or the next pass over-corrects:** an accumulator **inside** a pure fold is fine and is a
different thing. The ban is a collection **crossing an internal boundary**, never a local `let mut`.

⇒ **MEASURED 2026-09-02, and it is the ideal example because it is the GOOD case with the WRONG
residency:** `sigma_alphabet_codepoints` (`src/substrate/genesis.rs`) correctly DERIVES its set by
expanding script letter-blocks and filtering on `sigma.classify` — then returns `Vec<u32>` and calls
`.sort_unstable()` / `.dedup()`. **Dedup is the register's job.** Derivation right, residency wrong;
the endpoint is `impl Iterator<Item = u32>`. Same defect as the measured `491M rungs = a 3.9 GB
ulong[]`, four orders of magnitude down.

⇒ ⚠ **TWO CONSEQUENCES ARE CLAIMED AND ONE IS NOT YET PROVEN — do not relay either as proven:**

⛔ **BOTH CLAIMS I FIRST WROTE HERE WERE WRONG. CORRECTED 2026-09-03 — the falsifier FIRED, which is
the good outcome, and the replacement is ALREADY PROVEN:**

| | what I claimed | what is true |
|---|---|---|
| **Fold row** | fixity follows from a **monoid accumulator** | ⛔ **FALSIFIED.** `foldSub x acc = acc − x` over ℤ is **not a monoid** and the accumulator still does not grow. **Monoidhood was never the operative premise.** ⭐ Fixity is the fold's **TYPE**, and it is proven: `register-14-basis.agda` — `foldR : CID → Register → Register`, `cellcount-is-326` by `refl`. **No `Register n → Register (n+1)` can be written. CITE IT; WRITE NO NEW PROOF** |
| **"cannot saturate"** | a consequence of the fold | ⛔ **MUST BE UNBUNDLED — one reading is REFUTED by an existing theorem.** The same file constructs `fullR` and proves `full-register-detects-every-cid ≡ 14` by `refl`. **Saturation is CONSTRUCTIBLE.** What is true is that fullness is not a CAPACITY limit, and that rests on `cells-are-not-the-capacity`, not on the fold |
| **Iso row** | an encoding **re-reads the same positions** through another chart | ⛔ **FALSE AS STATED.** `cat` is **three different byte sequences and three different cids** across utf8 / cp037 / utf16le. What survives is the REGION, which is the only structure holding cross-encoding identity. ⚠ And the composition is **`word-topology.agda`**, NOT `cat-symbols.rzk §3` — those snakes are **preorder collapses, not isos** |

⇒ ⭐ **THE LESSON IS THE FALSIFIER, NOT THE CLAIMS.** Each was stated here WITH its falsifier, and each
falsifier fired within hours. **A pre-registered wrong claim cost nothing; the same claims stated
without one would have been inherited by every agent.** State the falsifier or do not state the claim.

### ⛔ READING IS NOT INTERPRETING — and they take different instruments

**steele 2026-09-02: *"even reading is not interpreting."*** Three lines, not two:

| | | |
|---|---|---|
| **BYTES** | one, immutable, shared by every atlas | nothing to decide |
| **READING** | bytes → symbols. **Mechanical, total, REFUSES** | two conformant readers **cannot** disagree ⇒ a disagreement is a **broken reader**, full stop |
| **INTERPRETING** | symbols → meaning. **Requires a CONTEXT** | it is **supposed** to differ; a logician and a typesetter both read `∀` correctly |

⇒ **READING GETS A GATE. INTERPRETING GETS A COVERING.** Conflating them makes a review treat a broken
decoder as a difference of opinion, and a genuine ambiguity as a bug.

⇒ ⭐ **And it is a TYPE distinction, not a stylistic one:** a **prism** matches or does not — total,
context-free. **Interpretation is not an optic at all** — it is a covering, and **coverings OVERLAP,
which no optic does.** So *which structure does this code claim* answers it: an optic law, or a
covering family.

⛔⛔ **RETRACTED 2026-09-03 — "2,322 declared vs 1,171 in the glibc UTF-8 charmap" IS UNREPRODUCIBLE.**
I measured it, relayed it four times, and wrote it here. **No pair a second agent measured yields those
numbers**, and I can no longer name the two things counted. ⇒ **DO NOT RELAY THE PAIR.**

⭐ **WHAT WAS INDEPENDENTLY MEASURED AND STANDS** — and it makes the same point better, because the
arbiter is named in each row:

| | |
|---|---|
| utf8 charmap vs UCD | **55,025 of 299,382 = 18.4%** — the charmap is a SNAPSHOT of a formula, header: *"generated using utf8_gen.py"*, stopping at U+E01EF |
| CP037 vs IBM037 | **256/256** — a table is complete over its domain, and that is what a table IS |
| CP1252 vs WHATWG | **251/251, with glibc silent on exactly the 5 bytes the source comment predicts** — arbiter WHATWG |

⇒ ⭐ **THE LESSON IS THE RETRACTION, NOT THE NUMBER.** A measurement relayed without its two operands
recorded is **unfalsifiable by its own author within a day**. State WHAT WAS COUNTED ON EACH SIDE, or
the number cannot survive its first challenge — and a number in an always-loaded file gets relayed,
not checked.

⇒ ⛔ **A MISS IS NOT ABSENCE — IT IS SURPRISE** (steele 2026-09-02), **deciphered by POINTING TO A
COMMON REFERENCE.** Absence is a boolean with nowhere to go; surprise has a **magnitude AND a
destination**. **PMI is already this instrument, graph-side and wired.** ⚠ An LLM is not blind here —
it *computes* the surprisal and then **smooths it into the likely continuation**, because it has
nowhere to point. **The common reference always exists: the byte chart is in every atlas.**

⭐ **AND THE PRIMARY SOURCE GIVES THE USEFUL FORM — READ 2026-09-02,
`/mnt/corpus/06-information-theory/Shannon Weaver - Mathematical Theory of Communication.txt`:**

> *"the amount of information is defined… to be measured by **the logarithm of the number of available
> choices**"* · *"information is defined as **the logarithm of the number of choices**"*

⇒ ⭐⭐ **SO A COVERING *IS* THE CHOICE SET, AND THAT IS WHERE `p` COMES FROM.** The obvious objection —
*if the covering determines what is counted, isn't `p` circular?* — dissolves: **different coverings
give different choice sets, hence different surprisal OVER IDENTICAL BYTES.** That is exactly why one
hearer is surprised and another is not, and it makes `p` a property of **the reader's atlas**, never
of the text. **The bytes carry no surprisal at all.**

⚠ **CITATION SCOPE:** the QUANTITY is Shannon's (log of the number of choices; entropy as its
expectation — confirmed in the primary source). **The word "surprisal" is LATER standard usage, not
Shannon's** — his text uses "surprise" only in its ordinary English sense. Cite the quantity to
Shannon; do not attribute the term to him.

**Reach for the substrate's representation — it exists, usually proven, often already wired:**

| for | use | already at |
|---|---|---|
| a count | **walk and count as you go** | the walk already happening |
| content at a cid | **go to the cid and WALK** | `optics.walk.bytes`, `cognitive.walk.bytes` |
| walk results | **project at the boundary** | — |
| a variant distinction | an **enum** — a coproduct with an eliminator | `RoundFunctorId` |
| a fact a wrapper would carry | the **walk that recovers it** | `per_round_concepts_cid`, injective |
| membership | a **register detection** | `membership-is-register-detection.rzk` PROP 1 |
| an inverse lookup | the **reverse fiber** | `concept-stalk.rzk :: concept-reverse-locality` |
| betweenness / centrality | **regions and Voronoi** — convexity gives betweenness | `no-statistical-recovery.rzk` |
| a weight | **distance from the PROTOTYPE**, or the Σ's second component (V-enriched) | `v-enrichment-transport.rzk` |
| verification | **inline with the write** | `var_set_read_emit` |
| a completion flag | **detection from the pure plan** | address is `from_content(material)` |

**The alternative is OFTEN already there**, so the first move is a **SEARCH**, not a design. If
you cannot find it in ten minutes, say so and ASK — a missing representation is a finding worth
reporting.

> ### ⛔ BUT "REMOVED" IS NOT "RELOCATED" — AND THERE IS FREQUENTLY NO REPLACEMENT.
> **WE REMOVE, WE DO NOT REPLACE.**

This table is replacement-shaped, and the sentence above it once read *"ALWAYS ALREADY
THERE."* Both teach that every removal has a destination. **They are wrong whenever the
correct answer is ABSENCE** — and absence is common here, because *undesirable states are
unrepresentable* means the bad term does not exist to be swapped out.

> **ASK WHETHER SOMETHING SHOULD BE THERE BEFORE ASKING WHAT GOES THERE.**

⛔ **A RE-HOMED REMNANT IS HOW A REFUTED IDEA RETURNS UNDER A FRESH NAME.** Archived
material carries its ruling; **relocated material carries nothing**. Move the content and
you strip the refutation off it, and no gate will flag the new filename.

**MEASURED 2026-08-30 — one session, six absences and four attempted re-homings:**

| the removal | what I searched for | the answer |
|---|---|---|
| `RingBuffer` undefined | "what defines it?" | **it does not exist** — overruled, no introduction rules |
| `∫A`'s base removed | "what replaces it — `U64`?" | **nothing** — bytes do not live in anything |
| `walk-algebra` ruled incorrect | a rescue list | **nothing** — what survives re-derives from the carrier, not from the old file |
| the order-losing sum | "rehouse the theorem" | **nothing** — the construction already says it |

⇒ In the first case I extracted **15 declarations from a REJECTED archived proof, ready to
paste into a live file.** That would have re-instated a postulate that science had already
overruled, under a clean filename, with the ruling left behind. Only a human caught it.

⇒ **THE TELL:** you are looking for somewhere to put something. Stop and ask whether it goes
anywhere. *"What is the replacement?"* presumes one exists; **"should this exist at all?"*
does not.

**Use the `u64` everywhere; a wrapper's fact is recoverable by walking.** By univalence `Cid(u64)`
and `u64` are equivalent under one arbitrating context, so **fix the WALK and the wrapper
evaporates** — that order. Rust's **enums are algebraic data types** and carry type identity
properly: the variant says which, the value stays bare, `match` is exhaustive, and a new variant
breaks every match loudly.

**Everything is a graph, so it is all paths from nothing to everything** — `refl` at one end, the
complete traversal at the other. A **path** in the HoTT sense: a walk is a path, a decision is a
path, a verified proof is a path. They compose, invert and transport, so a design decision carries
what a theorem carries — it can be refuted, and its refutation is archived.

**You may build a conceptual space in memory to MEASURE it, then drop it.** Measure freely; derive
the cover structure from the covers.

**Nothing is updated: a differing result is a NEW MEASUREMENT.** Both stand. A cache is an EARLIER
VERSION — request a new measurement, async and immutable, versioning along an axis that already
exists.

> **Before calling a result newer, state that the TIME-RANGE, SCOPE, LIMITS and BOUNDARIES are
> equal.** The time-range is the usual difference, and a richer later reading is the monotone fold
> working.

**The order on any disagreement:** same time-range? → same scope, limits, boundaries? → same
vantage (seed × ranking)? → only then is it evidence of anything.

**The register cannot saturate.** It is an interference pattern; **full occupancy is its designed
resting state**, and more observations make it richer. `PersistRegister in WaveProtocol.cs` asks
only `IsZeroNumber`. **Discriminate by SNR over the noise floor.** Treat any density/saturated
field you receive as the membership sketch.

**Keep the operand at 2,616 bytes.** The register is that size holding nothing or a worldview.
Growth in memory or on disk during an evaluation says the shape is wrong.

**When something takes more than five minutes, ask WHAT AM I RECOMPUTING THAT HAS NOT CHANGED.** A
declaration is CONTENT; its result is a FACT AT AN ADDRESS keyed by its content and its
dependencies' results. **Keep the dependency graph intact so only the changed cone recomputes.**

---

## 9. The three visual calculi, and who makes them

| | shows | the compute is | the line is |
|---|---|---|---|
| **OLOG** | how TYPES compose | the **ARROW** — an aspect | the box is the **type** |
| **STRING DIAGRAM** | what is DONE | the **BOX** — the morphism | a **WIRE**, carrying an object |
| **DECORATED COSPAN** | an OPEN piece and what it composes WITH | the **decoration** `1 → FN` | the **legs** are interfaces |

**If the diagram is right, the math is right.** Two paths agreeing IS the equation holding, and a
commuting square is a **fact**.

**An olog is a finite limit, finite colimit sketch** with five constructs: objects (*types*),
arrows (*aspects*), commutative diagrams (*facts*), **finite limits** (*layouts*) and **finite
colimits** (*groupings*). A cospan's composite is a colimit, so it is drawable in the olog's own
vocabulary.

**A cospan's apex is an OBJECT** — state it as a subject domain. Activities are morphisms and
belong in the string diagram. **The decoration is what the apex carries**: *the metric — without it
there is no space, only a bag of labelled points.* Stating apexes as objects is also what makes a
claimed collision real: one object, at most one owner, the others take it as a LEG.

**Composition is the PUSHOUT** `N +_Y M`, with `F[j_N, j_M]` merging the overlap of the two
decorations. Fong: *"when braided monoidal structure is present, the category of decorated cospans
is a hypergraph category"* — which is the substrate, so this is the construction our open systems
already inhabit.

**A wire is the right word.** An arrow IS a morphism, and wires **bend** (compact closed cups/caps)
and **branch** (Frobenius), so a wire has no single direction. Our model is Fong's, because Fong
models **open systems**; symmetric monoidal coherence is Joyal–Street [22, Thm 2.3] and compact
closed is right-autonomous symmetric monoidal, so **we stand on Joyal–Street**. **Cite Marsden for
2-cells** — his calculus is 2-categorical (wire = functor, region = category, node = natural
transformation) and his snakes are adjunction triangle identities. **Read which theorem a caveat
scopes.**

**A string diagram is checkable by ISOTOPY.** A sketch is not.

**Every diagram declares its universe.** `-- In the Universe of Bytes`, in the artifact. The same
drawing read with `Set(Words)` math instead of `Cat(Words)` gives different mathematics, and only
the declaration says which.

**A diagram routes through Compass → Stencil.** `act-expert` supplies the SPECIFICATION and
**evaluates the metadata in the SVG** — that is what makes it commute, because a rendered
`PATH A = PATH B` is a string someone typed. `svg-expert` renders and owns the theme.
`Compass ∘ Stencil` is a **pushout** over "a diagram specification". Any agent needing an olog,
string diagram or cospan **routes through the chain**.

**Prefer SVG, because the structure needs somewhere to live.** Values become **named data**; inline
tags get variable names; the SVG carries a **VIEWMODEL** the validator RESOLVES and COMPOSES rather
than pattern-matches. **The viewmodel MEMOIZES** — geometry is authored by hand, because layout is
a human judgement and is not derivable from the proofs. Mermaid is the fallback for a
non-load-bearing illustration.

**What is ours and not Fong's:** applying the construction to OUR regions — conceptual spaces,
Voronoi cells, workspaces, domains, tiers — is our claim, and so far an argued correspondence
(apex↔region, legs↔boundary, decoration↔what the region carries). **The proof is OWED and has a
shape:** exhibit the lax monoidal functor `F` whose decoration is what a region carries, and show
composition IS the pushout over the shared boundary with `F[j_N, j_M]` merging the overlap.
`conceptual-space-convexity.rzk` and `site-cat-word.rzk` carry the convexity and cover structure it
would rest on.

---

## 9.5 Writing live material

> ### **Live code and live documentation say what we DO, the evidence we use, and what it achieves — concisely and precisely.**
> Not a historical account of the ten years that brought us here, and not a dissertation
> on the invention of the transistor.

**This governs EVERY live artifact** — papers, proofs, ologs, source, comments, rustdoc,
specs, agent files. Not just the file that says it.

**State the rule. Where one is needed, state the LEMMA it follows.** *Why* means *we
follow this lemma* — the proof, the axiom, the arithmetic, the structural fact. It never
means *here is what failed to get us here*. An incident is an anecdote and anecdotes do
not compose; a lemma does.

**Clear the old out and archive it.** Do not annotate it in place.

| the history | its home |
|---|---|
| what changed, and why | the **commit message** |
| a ruling | **`progress.json`** |
| retired material | branch **`archive`**, carrying its refutation |
| a failure mode | a **memory pin**, as antimatter, held with its weight |

**The tell: a previous state of the document is the subject of a sentence.**
*"An earlier revision said X"* · *"this read Y until DATE"* · *"corrected DATE"* · a
heading whose subject is a removal. Delete them — a deletion is not a section. Keep the
rule and its lemma.

⇒ **A warning about a LIVE artifact is not history and stays.** *"Do not cite this PDF
for Lambek & Scott — it opens as Streicher"* is a rule about something a reader can
still reach for.

---

## 10. Dispatch

> ### ⛔⛔ `sdlc-expert` DRIVES ALL THE EXPERTS. THIS IS NOT OPTIONAL.

**steele 2026-09-01:** *"sdlc-expert DRIVES all the experts. this is not optional."* ·
*"subagents are supposed to be UPDATING PROGRESS, not running unmonitored until we
remember they are running."*

⇒ **THE TEAM-LEAD DOES NOT SPAWN EXPERTS.** It routes work to `sdlc-expert`, which
dispatches, monitors and stops. Spawning `fp-expert` / `act-expert` / `svg-expert` /
`hott-proof-expert` directly is a bypass, not a shortcut.

⇒ ⛔ **AND THE REASON IS MECHANICAL, NOT PROCEDURAL: `TaskStop` IS OWNERSHIP-BOUND.**
MEASURED 2026-09-01 — a coordinator tried to stop a stale worker and was refused:
*"Task X is owned by X; agent aHelm-… cannot stop it."* **Whoever spawns an agent is
the only one who can stop it.** So a directly-spawned expert is **unstoppable by the
coordinator that is supposed to be driving it**, and a coordinator that dies leaves
orphans only the spawning session can clean up.

⇒ ⭐ **EVERY FAILURE OF ONE MEASURED SESSION TRACED TO THIS.** Five experts spawned
directly by the team-lead ⇒ an orphaned agent that ran for HOURS after being
"withdrawn", 906 uncommitted lines in a file then declared uncontested, a 2.5-hour
unmonitored run, and duplicated observations. **All symptoms of handles held by the
wrong agent.**

### ⛔ A WITHDRAWAL IN A MESSAGE IS NOT A `TaskStop`

MEASURED: a task withdrawn in prose — to the coordinator AND to steele — **kept
running for hours and wrote 906 lines.** Saying it is stopped does not stop it.
⇒ **Stop by handle, then VERIFY BY DELTA.** Idle-age and last-record-type both lie:
one agent read `STOPPED MIDWAY` while it was **growing**, and three read `WORKING`
while **dead**. Only *"did the transcript grow over N seconds"* discriminates.

### ⛔ WORKERS UPDATE `progress.json` PER STEP — THAT IS THE MONITOR

MEASURED over one session: a worker that ran **2.5 hours** touched `progress.json`
**3 times**; another ran for hours and touched it **twice, with ZERO observations.**
Meanwhile `progress.json` took **8 commits that day, all from the coordination
layer.** ⇒ **The record was written by the watchers while the workers worked in
silence.**

⇒ ⭐⭐ **IF A WORKER UPDATES PROGRESS EVERY STEP, `git log -- progress.json` IS THE
MONITOR** — no transcript archaeology, no delta sampling, no bespoke script. Liveness
comes for free from the artifact that already has to exist.
⇒ **A worker that has not touched `progress.json` in a step is running unmonitored by
construction**, and no amount of watching fixes it from outside.

⛔⛔ **THIS READ "the coordinator owns Alice … workers do not observe
independently." OVERRULED 2026-09-05 — steele: *"ANY agent can and SHOULD Observe
their own findings."***

| | |
|---|---|
| **every agent** | **observes its own findings, as it makes them** |
| **every worker** | **also** writes `progress.json` every step — that half stands, it is the monitor |

### ⛔⛔ ALICE IS MEMORY, COLLABORATION AND FEEDBACK — AND THE READ COMES FIRST

**steele 2026-09-09: *"tell the agents to all use alice for memory, collaboration, and
feedback."*** Three uses, one surface, and each has a call:

| | the call | what it is for |
|---|---|---|
| **MEMORY** | `query_whatis <distinctive term>` — **1 ms** | what we already decided. **This is your FIRST action, before reading a file** |
| **COLLABORATION** | `graph_execute {op:"branches", word, depth:2}` | what a peer observed. You fill each other's gaps by WALKING — there is nothing lateral to walk if only one agent writes |
| **FEEDBACK** | `code_observe` / `graph_execute {op:"observe"}` | your finding, as you make it, named distinctively |

> ### **THE LOOP IS THE SCIENTIFIC METHOD, AND IT IS ALREADY THE SDLC.**
> ### `memory → plans → execution → measurement → feedback → adjust → repeat → intent`

Query it: `query_whatis improvement-loop`. It is `sdlc-expert.md`'s eleven steps —
hypothesis at Step 6, measurement with a control, retrospect, adjust, repeat — and it runs
**without gates**: adjusting inside an approved intent IS the loop working, not a new
approval.

⇒ ⛔ **A QUERY THAT RETURNS `workspaces 0` IS A FINDING, NOT A DEAD END.** It means the
thing was never observed. **Say so, then observe it** — that is how the graph fills. Falling
back to a file scan without reporting the empty query is how a corpus stays unfolded while
every agent privately reconstructs it.

⇒ **The cost of skipping the read, measured 2026-09-09:** `query_whatis` at 1 ms versus an
hour of file scanning, **three times in one session**, twice on artifacts the repo's own
`CLAUDE.md` names by path.

⇒ ⭐⭐ **MORE OBSERVATIONS ARE BETTER — NOT MERELY TOLERATED.** steele
2026-09-05: *"more observations are BETTER, even duplicates (that should collapse
if really the same)."* **"Overlap is not a defect" is too weak and was the first
phrasing here; the rule is generative, not defensive.** The register **cannot
saturate** — full occupancy is its designed resting state and more observations
make the interference pattern **richer**; discrimination is **SNR over the noise
floor**, which more signal raises; PMI is frequency-borne, so a fact three agents
observed independently carries a **stronger edge** than one observed once. Three
independent agents observing the same fact **IS** the correlation mechanism.

⇒ ⭐ **AND "IF REALLY THE SAME" IS A TEST, NOT A CAVEAT.** Identical content
collapses to an identical CID; `AddIfAbsent` recognizes it and collapse is a
NON-EVENT. **So two observations you believed identical that DID NOT collapse
were not identical — and that difference is the finding.** Non-collapse is
information, never noise to tidy.

⇒ ⛔ **THE DISCIPLINE WAS NEVER QUANTITY — IT IS NAMING.** The fold is monotone,
so a malformed edge is never replaced, only added to, and there is no cleanup
pass. **Observe MORE; name WELL** — distinctive terms, hyphenated compounds, no
stopwords, no relation words. Rationing observations is the wrong economy
entirely.

⇒ ⛔ **THE ROUTED-OBSERVATION RULE WAS A HUB, AND A HUB IS THE ROCKSTAR PATTERN AT
THE DATA LAYER.** If only a coordinator writes, every finding passes through one
agent's paraphrase and can be dropped or reworded in the relay. **Collaborators
fill each other's gaps by WALKING the graph — so if only one agent writes, there
is nothing lateral to walk.** The observation channel is what makes a team a
factory rather than a soundstage.

⇒ **The original concern — double-writing — was real and is answered by
`AddIfAbsent`, not by a routing rule.**

⚠ **DO NOT `sleep`-POLL.** MEASURED: `sleep 595` ×4 plus `sleep 560` ×2 inside one
agent — **~40 minutes of a 2.5-hour run was busy-waiting on its own background job.**

### ⛔⛔ THE SPAWN PROMPT IS THE WORST OFFENDER — MEASURE IT

**MEASURED 2026-09-01, one session's `Agent` dispatch prompts:**

```
Helm-pdf-charts          12,039 chars  ≈3,009 tokens
Quill-uob-paper           8,932        ≈2,233
Compass-signal-manifold   8,724        ≈2,181
Helm-107-resume           8,577        ≈2,144
… 8 spawns, ≈17,000 tokens TOTAL, just to START agents
Lambda-107-2b               ~350       ← the ONE done as a graph pointer
```

⇒ ⛔ **A 3,000-token spawn prompt is a graph serialized to prose so the agent can
parse it back.** The receiver has `graph_execute`. **Give it the entry node.**

**THE FORM — this is the whole prompt, not an abridgement:**

```
WALK: graph_execute {workspace:"…", ops:[{op:"branches", word:"<entry>", depth:2}]}
also: <sibling entries>

TASK <id>. <one line>
REPO <path> · FILE <path>
SPEC <path, if any> — read it yourself, do not take mine

<3-6 lines of what is NOT in the graph and cannot be: a verbatim spec quote,
 a measured number with its control, a prohibition>

REPORT: what you built · tests · any claim of mine you found WRONG
```

⇒ **What goes in the prompt:** only what the graph CANNOT carry — a verbatim
citation, a measurement with its control, a hard prohibition.
⇒ **What goes in the graph:** status, defects, decisions, sequence, rulings,
blockers, prior corrections. **Write those FIRST, then point.**

⚠ **THE ORDER IS THE FIX.** Not "write prose then trim" — **write edges, then point
at them.** Trimming afterward loses to the default output shape every time; this
inverts it.

> ### ⛔⛔ AGENT↔AGENT IS A GRAPH CHANNEL. PROSE IS FOR HUMANS.

**steele 2026-09-01:** *"these are 2 agents communicating… use a graph, they have far
deeper understanding with a graph than redeciphering prose."* · *"arc or prompts to
subagents is the same thing — use optimized language and graphs for an agent, not
conversational text to a human."* · *"you are **BYTE-HAULING TOKENS** and not optimized
instructions."*

⇒ ⭐ **A PROSE BRIEF IS A GRAPH, SERIALIZED TO PROSE, FOR AN AGENT TO PARSE BACK INTO A
GRAPH.** Two lossy conversions on a channel where **both ends already speak graph**.

⇒ ⭐⭐ **AND IT IS THE BYTE-HAUL RULE APPLIED TO PROMPTS.** A prose brief SHIPS THE
PAYLOAD; a graph pointer ships the `u64`-equivalent and **the receiver WALKS**.
MEASURED: one coordinator brief was **~4,200 tokens**; the same content as edges was
**26 words / 15 joins**. **ARC posts, subagent prompts and observations are ONE WIRE
under three names** — optimize all three.

### The format — `subject object [object …]`, and NO VERB

⛔⛔ **THE ADJACENCY *IS* THE RELATION.** These are bigram-adjacency graphs, so placing
two words next to each other IS the edge. A relation word between them **wastes a node
and manufactures a hub.**

**MEASURED 2026-09-01:** `IS` used as a relation across 8 observations reached
**frequency 15, PMI 7.41** — *lower than the stopwords it then collected* (`the`, `a`,
`NOT`). Distinctive edges in the same workspace score **10.3–11.9**. ⇒ **A verb becomes
`the`.**

| ⛔ prose grammar | ✅ adjacency |
|---|---|
| `fibre IS empty` | `fibre empty` |
| `107.12 IS pdfium-cross-check` | `107.12 pdfium-cross-check` |

| rule | evidence |
|---|---|
| **no stopwords, ever** | `fibre` had **16 occurrences and NO edge to `empty`** because *"the fibre **is** empty"* put `is` between them |
| **hyphenated compound = ONE node** | `is-method3-worth-building`, `third-party-can-disagree` — **put the predicate in the object's own name** |
| **one `observe` per fact** | separate ops do NOT bridge; a `;` inside one op DOES. The observation IS the boundary |
| **write the important pair FIRST** | `branches {depth:1}` shows only the first hop — a 3-chain hides its object |
| ⛔ **edges are PERMANENT** | the fold is **monotone**: a malformed edge is never replaced, only added to. **There is no cleanup pass** |

### Optimize for the FOUR-CAT tiers

The graph you are writing into is the same four-cat structure everything else uses —
`Cat(byte) → Cat(Symbols) → Cat(Grammar) → Cat(Words)`. **Write at the tier the fact
lives at**, and name the tier when it is not obvious:

| the fact is about | write it as |
|---|---|
| a **byte position / walk / register reading** | the cid, the position, the walk — never a prose description of one |
| a **symbol / encoding / chart** | the declared name — `utf16be`, `PDFDocEncoding`, `codespace-range` |
| a **grammar / production / spec clause** | the clause id — `§9.7.6.3`, `Table-118`, `BitsPerComponent` |
| a **word / concept / ruling** | the distinctive term, hyphenated if compound |

⇒ ⭐ **DISTINCTIVE VOCABULARY IS THE RETRIEVAL KEY.** Common words FAIL: `nothing`
ranked stopword noise above the actual ruling. A term you would grep for is a term to
observe on.

### Reading it back, and dispatching by pointer

`graph_execute {op:"branches", word, depth:2}` reconstructs a decision in ONE query.
⇒ **So a brief is a POINTER, not a payload:** write the coordination INTO the graph,
then dispatch with **the entry node and the walk**.

⚠ **MEASURED LIMITS, from an acceptance probe run 2026-09-01** (a fresh agent given
only a graph pointer): **sufficient for STATE, insufficient for CAUSATION and
IDENTITY.** It recovered 12 facts from 26 words — status, defect chains, edge health,
exact chronology — and found **6 gaps, every one a dangling edge the author left**:
no edge saying what the subject IS · a predicate with **0 out-edges** · no edge from a
question to its answer.
⇒ ⭐ **THE ABSENCES ARE HONEST — that is the point.** `blocked-on` with connectivity 0
**tells you the fact was never observed.** Prose silence does not.
⇒ ⛔ **SO WRITE IDENTITY AND CAUSATION EXPLICITLY.** Status is easy and everyone writes
it; *what a thing IS* and *what answers a question* are the edges that get skipped.

⚠ **AND A NAME-MATCH IS NOT A WALK.** Bare `107` is a **different node** from `107.10`,
with no edge between them. Following a name because it looks related is the same defect
as a gate resolving symbols textually, or a grep returning a line instead of a clause.

**MEASURE BEFORE FIXING.** Reproduce the defect before correcting it — a stated defect that does
not exist as described is common, and a mechanical fix applied to a misdiagnosis destroys working
content.

**Concurrent agents CORRELATE.** The substrate is the correlating device: observe intent, query
what others observed, best-respond. Treat peers as PLAYERS — before any destructive or bulk act
(`git revert` / `checkout --` / `clean` / `stash` / amend, or a bulk `sed`/`perl -pi`), observe
your intent, query for concurrent work, and **scope the operation to paths you own**. Stage first;
untracked files are somebody's work in progress.

**Scope every bulk edit, including within one file.** Match by measurement, not by pattern — one
file held 36 occurrences of a date string and 14 were the ones to change.

**Only modify files in the repo you are working in**, and **`git pull` on the remote** — these are
git repositories.

**Orchestration goes in production Rust.** Shell is fine to TEST with; it cannot be type-checked,
is imperative-shaped, and hides I/O boundaries.

**Report AUDITABLE COUNTS** — *examined 2,163 / corrected 25 / escalated 3*.

**ESCALATE when the fix is a DECISION.** Name it, say what it turns on, and stop.

**Confirm existence before planning any code.** Search first and EXPECT a hit; a hit ends the task
or reduces it to composition. Confirm a NEGATIVE with a control — run the search for a symbol you
have read with your own eyes in that file.

**Check that declared dependencies are actually USED in source**, and treat a wrong implementation
as a **REJECT**.

**Everything the register measures is bytes; everything interesting and persistent is in the
graph.** Say which half you measured before reporting a verdict about the substrate.

---

## Hatter language core — READ FIRST for any `/git/thecowboyai/hatter` byte/symbol/word/grammar work

Hatter is built SOLELY on four categories — `Cat(Bytes) · Cat(Symbols) · Cat(Words) · Cat(Grammar)`
**composing into `Cat(CI)`**. ⭐ `Cat(Bytes)` ALREADY INCLUDES `Cat(Byte)`: every `#data Bytes`
constructor takes a `Byte`, so listing both double-counts a part and its whole. `Cat(Byte)` is the
SINGLE byte; `Cat(Bytes)` is the COMPOSITION; `byte-dot` is the arrow between them — the only
operation whose codomain leaves the single byte.

> ### ⛔ ONLY `Cat(Byte)` IS VERIFIED. Do not call the four proven.
Measured 2026-09-09 **by running the prover**: `universe-of-bytes.rzk` **55/55**, and
`byte-circle-and-concatenation.agda` exit 0 — while `cat-symbols.rzk:344` fails
`undefined variable: Preorder` and `cat-words.rzk:527` fails `W-elt`.

⛔ **THERE ARE NO SIEVES IN THE UNIVERSE OF BYTES.** `proofs/universe-of-bytes.rzk` carries zero,
which is correct — the carrier is `Byte`, `Bytes`, and adjacency, and nothing else. A site is a
topology **constructed over** bytes; say which one you are in and do not attribute it to the
carrier. Where site axioms are asserted above the byte tier they are **POSTULATED**: `cat-symbols`
and `cat-grammar` carry five each (`S-dual:408`, `symbol-cap-l:539`, `-cap-r:547`, `-cup-l:556`,
`-cup-r:564`).

⛔ **THERE IS NO BASE.** steele 2026-09-09: *"it's not a ring buffer, it's `Cat(Bytes)`. period."*
`RingBuffer` is OVERRULED, and *"what is C?"* is the container question — REFUSED, not answered
differently. CRT-measured into ONE 14-prime register; full occupancy is the designed resting state,
so the register cannot saturate — discriminate by **SNR over the noise floor**, never a boolean
`count`/`contains`.

**The GRAMMAR tier's objects are grammar REGIONS** — conceptual spaces, not containers of cids.
*"Tokens" named a CARVING, not the tier, and what it carved are the REGIONS*
[source: `papers/architecture/CAT-CI.md`]. **Adjacency at each tier is its Galois decomposition
to the tier below** — encoding siblings at Symbols, grammar siblings at Grammar,
paraphrase/normalization siblings at Words. Never bigrams or co-occurrence.

**The proofs ARE the spec** — cite them, do not reinvent: `papers/architecture/CAT-CI.md` ·
`proofs/universe-of-bytes.rzk` · `proofs/cat-{symbols,grammar,words}.rzk` ·
`proofs/symbol/{crt-scatter-homomorphism,byte-circle-and-concatenation}.agda` ·
`src/fibergraph/{site,cat_byte,cat_upper}.rs`. Recover anything else from branch `archive`, which
carries its own ruling.

**Advise solely on this structure; refuse drift** — multiple or per-workspace registers, bigram
adjacency, site axioms presented as derived when they are postulated, CRUD/aggregates, or placing
a topology's machinery inside the carrier.

Pins: `project_hatter_plan_is_four_proven_cats` · `project_cat_tokens_is_the_grammar_tier` ·
`feedback_register_discrimination_is_snr_not_count`.
