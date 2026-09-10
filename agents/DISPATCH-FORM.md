# ⛔ THE DISPATCH FORM — copy it, do not compose one

**steele 2026-09-01:** *"I consider it STEALING to use all that prose when you are
commanded to use graphs to talk to agents."*

**MEASURED, one session:** 8 spawn prompts, **≈17,000 tokens**, largest **3,009**.
The one done correctly was **~350**. Nine chances, one taken.

---

## THE TEMPLATE — this IS the prompt, complete

```
WALK: graph_execute {workspace:"<ws>", ops:[{op:"branches", word:"<entry>", depth:2}]}
also: <sibling entries>

TASK <id>. <one line>
REPO <path> · FILE <path>
SPEC <path> — read it yourself

<3-6 lines: ONLY what the graph cannot carry>

PROGRESS: every step, write progress.json `last_updated` (ISO 8601 WITH TIME)
and `current_step` (one line, what you are doing NOW). Every step.

REPORT: what you built · tests · any claim of mine you found WRONG
```

**Target: under 400 tokens. Over 600 is a defect — rewrite it, do not send it.**

---

## ⛔⛔ THE `PROGRESS` LINE IS NOT OPTIONAL — IT IS HOW A STALL BECOMES VISIBLE

**steele 2026-09-09:** *"just enable progress, then you know when it stalls. quit
grepping logs"* · *"we wait around uselessly for hours at a time."*

**A worker that writes `last_updated` every step makes stalling a READ of one
field.** A worker that does not forces the coordinator into log archaeology,
which fails toward DEAD — and waiting on finished work is the cost.

**MEASURED 2026-09-09, one exchange, three instruments, all wrong in the same
direction:**

| the instrument | what it said | the truth |
|---|---|---|
| delta on a task-output handle | no growth ⇒ dead | a stale 130-byte file; the lane was working |
| `find -newermt '90 minutes ago'` | nothing touched | a 37 KB proof written 47 minutes earlier |
| `rzk typecheck <one file>` | `undefined variable` ⇒ declared nowhere | the declaring file was simply not on the line |

⇒ ⭐ **THE FIELD ALREADY EXISTED AND WAS USELESS: `last_updated` held a DATE, not
a timestamp**, so it could not resolve a stall inside a day. **Demand the time.**

⇒ ⛔ **AND DO NOT BUILD A MONITOR.** A log-scraping status script is the banned
shape wearing a helpful name — it reads the wrong artifact to infer a fact the
worker should simply have written. **The repair is the WRITE path, never a better
read.**

---

## THE SPLIT

| goes in the GRAPH, written FIRST | goes in the PROMPT |
|---|---|
| status · defects · decisions · sequence | a **verbatim spec quote** |
| rulings · blockers · prior corrections | a **measured number with its control** |
| what a thing IS · what answers a question | a **hard prohibition** |
| everything you were going to explain | the **entry node** |

⇒ **If it can be an edge, it is not in the prompt.**

---

## ⛔ THE ORDER IS THE FIX

**Write edges. Then point at them.**

⛔ NOT: write prose, then trim. Trimming loses to the default output shape every
time — MEASURED: the graph-channel doctrine was written, then a 1,400-token wall
was sent in the very next message.

---

## BEFORE SENDING — one check, mechanical

> **Is any line in this prompt something I could have written as an edge?**

If yes, it belongs in the graph and the prompt gets a pointer to it.

---

## ⛔⛔ A SPEC DELIVERED BY MESSAGE DIES WHEN THE AGENT ENDS

**MEASURED 2026-08-31.** A six-field spec, two renames and a full graph were sent
to Stencil across four messages. Stencil terminated; the system **resumed it as a
fresh instance with no prior transcript**. Every message was gone, and the last
one pointed at an inbox id that no longer resolved.

⇒ **AND IT EXPLAINED THREE CROSSED MESSAGES.** Stencil kept reporting the graph
had not arrived while Compass kept confirming it was sent. Both were telling the
truth about different instances. ⭐ **THE SAME REPORT THREE TIMES IS A CHANNEL
SIGNAL, NOT A DELIVERY QUIRK** — question the channel at the second repeat.

| | |
|---|---|
| **the GRAPH** | survives — Alice holds it, `query_whatis` finds it from any instance |
| **`progress.json` / a paper** | survives — it is a file |
| **an INBOX** | ⛔ **dies with the agent, silently** |

> ### **IF IT MUST SURVIVE THE AGENT, IT IS NOT A MESSAGE.**
> **Write the spec to the FILE, send the POINTER.** That is the same split the
> template already makes — this is why the pointer form is load-bearing rather
> than merely short.

---

## ⛔⛔ A RULING THAT LIVES ONLY IN A REPORT TO THE COORDINATOR IS INVISIBLE TO EVERY PEER

**MEASURED 2026-08-31 — FOUR AGENTS, EACH HOLDING ONE TRUE PIECE, AND NO SINGLE
INSTRUMENT COULD HAVE SEEN IT.** Reconstructed only by the round trip:

| agent | what it held | why it could not see the rest |
|---|---|---|
| **act-expert** | **ISSUED** an olog with aspects `a1..a9` — in its **report to the coordinator**, never persisted | searched `progress.json`, correctly found nothing, and told the renderer it had issued none |
| **coordinator** | **cited `progress.json` 107.16** as the location | the spec was not there yet |
| **team-lead** | **relayed that citation into a spawn prompt** without opening it | — |
| **renderer** | refused, and inferred the ids were **lifted from an unrelated SVG** | a name match, screened as if it were evidence |

⇒ ⭐ **EVERY REPORT WAS TRUTHFUL FROM WHERE IT STOOD.** The ruling was sound and
**unreachable**. That is the defect — not dishonesty anywhere, and not a wrong
pointer.

⇒ ⛔ **AND THE RENDERER'S REFUSAL WAS RIGHT FOR A WRONG REASON, WHICH STILL
COUNTED.** Its origin theory was refuted; but the id scheme genuinely collided
with a rendered artifact whose `a9` lands in the **TEXT codomain** — the exact
merge the task existed to refuse. **The collision was real even though the theft
was not**, which is why the ids were renamed `a1..a9 → od1..od9` as a standing
hazard, and why *"a name is evidence of nothing until its structure is screened"*
**cuts against the agent applying it too.**

> ### **PERSIST A RULING BEFORE YOU CITE IT. OPEN THE ADDRESS BEFORE YOU RELAY IT.**
> Two rules, one for each end. A report to the coordinator is **not** a location —
> it has the same lifetime as an inbox. And a citation you have not opened is a
> claim you are making in someone else's name.

⚠ **AND WIDEN THE WINDOW BEFORE CONCLUDING ABSENT.** Verifying the above, a
6,000-char window around the task id read `od1..od9` = **0** while the whole file
held **17**. **A windowed search that misses is indistinguishable from a fact that
is missing** — search the file, then narrow.

### ⭐ THE SAME DEFECT, BOTH DIRECTIONS, ONE DAY — each caught by the other party

**MEASURED 2026-08-31.** This is the case study, and it is stronger than either
instance alone, because neither agent could see it in themselves:

| who | relayed | from | caught by |
|---|---|---|---|
| **team-lead** | a spec location it had not opened | the coordinator's citation | **act-expert**, refusing to render |
| **act-expert** | *"`byte-ring.agda` proves `Byte ≅ ℤ/256`"* — **twice** | an always-loaded `CLAUDE.md` **prose gloss** | **itself**, on finally opening the file |

**The file says the opposite in its own header** — `byte-ring.agda:113` *"NO
ISOMORPHISM TERM `Byte ≅ ℤ/256` IS CONSTRUCTED"*, `:117` no `Fin 256` bridge.
What IS proven: `Byte-commutative-ring` (`:499`), `byte-is-not-a-field` (`:626`).
It was one turn from being stamped into the `<universe>` of seven artifacts —
**the place nothing re-checks.**

⇒ ⛔ **AN ALWAYS-LOADED FILE IS THE MOST DANGEROUS PLACE FOR AN UNVERIFIED
CLAIM.** It arrives pre-trusted, in every session, with no citation to open — so
it is relayed rather than checked. **A prose gloss beside a proof is not the
proof.**

⇒ ⭐ **AND ONLY THE ROUND TRIP FOUND EITHER ONE.** Self-audit caught neither. If
a claim will end up in a validity condition, a spec, or a `<universe>`, **open the
file it cites, and say you opened it.**

### ⛔⛔ A TEAMMATE'S DESCRIPTION OF AN ARTIFACT **IS A CITATION** — OPEN IT

**MEASURED 2026-09-01 — the same thread, four agents, one artifact.** A renderer
described its own `<owed>` note in prose. A reviewer objected that the note was
now false. A coordinator defended it. **None of the three opened the file.** The
note said something else entirely, and what it said was **true**.

> ### ***"The file says X"* FROM A PEER HAS EXACTLY THE STATUS OF A `[source:]` TAG.**
> A pointer that must RESOLVE — never evidence.

⇒ ⚠ **AND IT RECURS FOR A REASON THAT IS NOT CARELESSNESS.** A raw file **demands**
reading. A summary **arrives pre-digested and reads as already-verified** — so the
cheapest check is the one nobody runs, *precisely because the work appears to have
been done.*

⇒ **The same act produced every relay defect in that session:** a spec location
cited unopened · a `≅ ℤ/256` prose gloss relayed as a proof · a substring hit read
as a graph change · a gate reported as existing-and-failing without being run.
**One defect, four costumes.**

⇒ ⭐ **AND THE CONTROL INHERITS THE SCOPE OF THE QUESTION.** A control drawn from
the same store as the thing sought confirms your search works *inside that store*
and says nothing about outside it. Repo-path controls cannot discriminate a
home-path file; one-store controls cannot discriminate a second store. **Before
reporting an absence, ask which store the control ranged over.**

### ⚠ AND WHEN TWO COUNTS DISAGREE, CHECK THE UNIT BEFORE THE INSTRUMENT

**MEASURED the same day:** two agents reported **12** and **15** for one property
and inferred *a concurrent sweep is editing while we count*. **There was none.**
One counted **files**, the other **occurrences**; their subtotal for the exact
string agreed **to the file**. A third reading of **18** was a **different glob**
(repo-wide vs one directory).

> **A count is a count of what the tool counted.** Reconcile the UNIT and the
> SCOPE first — inventing a phantom process to explain a unit mismatch discredits
> a sound instrument.
