#!/usr/bin/env python3
# Copyright (c) 2025-2026 - Cowboy AI, Inc.
r"""
GATE — a diagram that CLAIMS commuting must be CHECKABLE, with the same rigor a
proof system applies to an equation.

In the Universe of Bytes.

⛔ WHY THIS EXISTS. steele 2026-08-22: "then these are pretty pictures. we need to
check commutation for the visual elements with the same rigor a proof system does
for math." Six SVGs were rendered that asserted `PATH A = PATH B` because a human
typed that string. Nothing could contradict it. That is `fn verify() -> bool
{ true }` in visual form.

WHAT A PROOF SYSTEM ACTUALLY DOES, and this mirrors it in two stages:

  STAGE 1 — WELL-FORMEDNESS (mechanical, decidable, done here in full)
    Before asking whether two terms are EQUAL, a prover asks whether they are
    even of the SAME TYPE. The diagram analogue:
      * every arrow's src/tgt resolves to a declared node
      * each path is a genuine COMPOSITION: arrow[i].tgt == arrow[i+1].src
      * the two paths are PARALLEL: same source, same target
      * the claimed from/to match the paths' actual endpoints
      * the codomain is NOT a declared TERMINAL / CONTRACTIBLE object
    A diagram failing the first four is not "unclear" — it is MALFORMED, and
    the equation it claims cannot even be STATED. Failing the fifth makes it
    VACUOUS: statable, trivially true, and carrying no information.

  STAGE 2 — TRUTH (NOT decidable here; discharged by CITATION)
    By CIM-19 a commuting square IS an equation IS a proof term. So truth is
    established by naming a #def whose TYPE is that equation and whose proof
    typechecks. This gate does not re-run the prover; it requires the citation
    to EXIST and resolve, and it FAILS any claim that has neither a proof nor an
    explicit exemption status.
    A REFUTED claim asserts the NEGATION, which is equally a positive claim, so
    it carries the SAME burden under a different attribute: witness=FILE#SYMBOL,
    resolved by the very same instrument.

⇒ SO GREEN MEANS: every commuting claim is WELL-FORMED and NON-VACUOUS, and
  every one is either backed by a resolving proof citation, or REFUTED with a
  resolving witness, or openly carries one of the three exemption statuses
  (UNPROVEN, UNPROVABLE-AS-STATED, DEFINITIONAL — the latter two with a stated
  reason). It does NOT mean the diagrams are true. Stage 2's verdict lives in
  the proof gate.

/!\ WHAT IT CANNOT SEE, stated so green is not over-read:
  * whether the cited #def's TYPE is really the equation drawn. The citation
    resolves; its content is not compared to the geometry.
  * whether the rendered picture matches the metadata. A diagram could declare
    one graph and draw another — see --strict for a partial check.
  * anything about a diagram carrying no metadata block at all, other than to
    report it as UNCHECKABLE (which is a finding, not a pass).
  * ⛔ AN UNDECLARED TERMINAL OBJECT. The vacuity rule added 2026-09-07 fires
    ONLY on a node the author marked terminal="true" / contractible="true". The
    gate cannot compute contractibility — it does not typecheck, it has no
    access to the object's definition, and there is no structural signature of
    terminality in a drawn graph (a sink is not a terminal object; every olog
    has sinks). ⇒ SO THE RULE IS HONOUR-BOUND ON DETECTION AND MECHANICAL ON
    CONSEQUENCE. Once declared, the vacuous square is UNREPRESENTABLE; until
    declared, it is invisible. This is stated rather than hidden because the
    defect that motivated the rule — `NoIntersection := Σ (witness : Point
    two), witness = witness`, which is contractible, so
    `vesica-d-gt-2r-yields-no-intersection` typechecks whether or not the
    circles intersect — was found by a human READING the type, not by any gate.
    The proof gate, not this one, is where contractibility could be decided.
  * whether a REFUTED claim's witness really WITNESSES non-commutation. The
    witness citation is resolved to a DECLARATION, exactly as a proof citation
    is; its TYPE is never compared to the negated equation.
  * ⛔ A KNOWN MODEL LIMIT, found 2026-08-23 by svg-expert while trying to
    declare a Frobenius claim: THIS GATE MODELS A PATH AS A LINEAR CHAIN OF
    BINARY ARROWS. That expresses composition squares fine, but it CANNOT
    directly express the two law-families our category actually has:
      - SPECIALNESS  (comultiply ; multiply = id) needs a SPLIT and a MERGE
      - the SNAKE    ((1 (x) cap);(cup (x) 1) = id) needs a bend and its partner
    Both are expressible only by declaring the intermediate TENSOR object as a
    single node (delta : A -> A(x)A, mu : A(x)A -> A), which is a modelling
    decision the author must make deliberately, not something the gate infers.
    ⇒ SO A GREEN RUN SAYS NOTHING ABOUT FROBENIUS OR COMPACT-CLOSED LAWS. The
    gate checks the fragment it can see, and the fragment it can see is the
    PROGRESSIVE one — which is precisely NOT our category (Selinger Caveat 3.2).
    Naming this rather than hiding it: a gate that silently ignores the laws
    that matter most here would be the measurement artifact again.

REPEATABLE RESULT (added 2026-08-23). steele: "a PROGRAM that can REPEAT the
results — deterministic, re-runnable, producing a saved artifact you can diff."
A printout is not a repeatable result; it cannot be diffed and it cannot be
carried to another machine. So the verdict is also emittable as a JSON artifact:

  --emit-result FILE   write the full verdict as deterministic JSON
  --check-result FILE  recompute from the SVGs and DIFF against a saved artifact;
                       exit 1 naming every field that drifted, 0 if identical

  Determinism, and each of these is a requirement not an implementation detail:
    * claims, defects and file lists carry a TOTAL SORT KEY — never dict or
      filesystem order
    * NO timestamps, hostnames, usernames, PIDs, randomness
    * paths are stored RELATIVE to the scanned root, so the artifact does not
      embed the machine it was produced on and diffs across machines
    * UTF-8, sort_keys, indent=2, trailing newline
  ⇒ Two runs over unchanged inputs produce a BYTE-IDENTICAL file.

/!\ WHAT EMISSION DOES NOT ESTABLISH. An artifact records the verdict; it does
  not make any claim TRUE. --check-result answers exactly one question — "does
  today's verdict match the recorded one?" — and a matching artifact is evidence
  of STABILITY, never of correctness. Both diffed verdicts can be wrong together.
  Everything under "WHAT IT CANNOT SEE" above is equally invisible to the JSON.

USAGE
  check-diagram-commutation.py <dir-or-file>...     gate
  check-diagram-commutation.py --list <...>         show every claim + status
  check-diagram-commutation.py --strict <...>       also require labels to appear
                                                    in rendered <text>
  check-diagram-commutation.py --emit-result F <...>   write JSON verdict to F
  check-diagram-commutation.py --check-result F <...>  diff verdict against F
Exit: 0 = all claims well-formed and accounted for; 1 = a defect (or, under
--check-result, drift); 2 = nothing to judge.
"""
import sys, os, re, glob, json
import xml.etree.ElementTree as ET

NS = "https://cowboy.ai/diagram/v1"
SVG = "http://www.w3.org/2000/svg"

ARTIFACT_KIND = "diagram-commutation-result"
# ⛔ BUMPED 1 -> 2 ON 2026-09-07, and the bump is NOT cosmetic. The claim record
# gained a `witness` field, `summary.by_status` gained a REFUTED key, and
# `stage1_verdict` gained a third value (VACUOUS). A schema change that does not
# move the version lets --check-result report drift with no explanation of WHY,
# which is the silent failure this artifact exists to prevent. Any artifact
# emitted under version 1 must be RE-EMITTED, not hand-edited.
ARTIFACT_VERSION = 2

# The statuses that are always reported, each with its own count, EVEN WHEN ZERO.
# ⛔ THEY ARE NEVER SUMMED. Collapsing them is the defect this gate's own comment
# at STAGE 2 warns against, and until 2026-08-23 the summary line performed
# exactly that collapse — printing "4 openly UNPROVEN" over 2 DEFINITIONAL and
# 2 UNPROVABLE-AS-STATED, i.e. reporting a real obligation as a non-obligation.
# ⛔ REFUTED ADDED 2026-09-07 AND IT IS NOT SUMMED EITHER. It is the LOUDEST of
# the five — it asserts the square does NOT commute — so folding it into any
# aggregate would repeat the 2026-08-23 regression in its most damaging form,
# printing a counterexample as though it were an ordinary open obligation.
CANONICAL_STATUSES = ("PROVEN", "UNPROVEN", "UNPROVABLE-AS-STATED",
                      "DEFINITIONAL", "REFUTED")

# A node may declare itself terminal under either spelling. In a category of
# types these are the SAME property: a contractible type is a terminal object,
# because isContr A makes (X -> A) contractible, hence any two maps into A are
# equal. Both spellings are accepted so an author writing in HoTT vocabulary and
# an author writing in categorical vocabulary reach the same gate.
TERMINAL_ATTRS = ("terminal", "contractible")


def find_meta(root):
    """Return the <diagram> element from <metadata>, or None."""
    for md in root.iter(f"{{{SVG}}}metadata"):
        for d in md:
            if d.tag.endswith("diagram"):
                return d
    return None


def rendered_text(root):
    out = []
    for t in root.iter():
        if t.tag in (f"{{{SVG}}}text", f"{{{SVG}}}tspan"):
            if t.text:
                out.append(" ".join(t.text.split()))
    return " | ".join(out)


def terminal_declaration(node):
    """Pure: an SVG <node> element -> (is_terminal, error_or_None).

    ⛔ A MISSPELLED VALUE MUST NOT READ AS ABSENCE. terminal="ture" is not a
    node that failed to declare terminality; it is a node whose declaration the
    gate could not read, and silently treating it as false is exactly how a
    real exemption disappears — the same failure shape as an unexplained
    DEFINITIONAL. So an unrecognised value is a DEFECT with its own message.

    On an unrecognised value the returned flag is False and the caller emits the
    error instead of the vacuity finding. That is a deliberate choice about
    CASCADE, not about strictness: the run is already RED, so nothing is
    silent, and the author sees ONE precise message ("I cannot read this
    attribute") rather than a downstream consequence derived from a guess.

    Two spellings that DISAGREE is also an error, not a precedence question.
    """
    seen = {}
    for attr in TERMINAL_ATTRS:
        raw = node.get(attr)
        if raw is None:
            continue
        v = raw.strip().lower()
        if v not in ("true", "false"):
            return (False,
                    f"declares {attr}='{raw}', which is neither 'true' nor "
                    f"'false'. An attribute the gate cannot read is not an "
                    f"absent declaration — fix the value; do not let a typo "
                    f"decide whether a claim is checked.")
        seen[attr] = (v == "true")
    if len(seen) == 2 and len(set(seen.values())) == 2:
        return (False,
                f"declares terminal='{str(seen['terminal']).lower()}' and "
                f"contractible='{str(seen['contractible']).lower()}' — these are "
                f"the SAME property (a contractible type IS a terminal object), "
                f"so they cannot disagree. Declare one.")
    return (any(seen.values()), None)


# --------------------------------------------------------------------------
# CITATION RESOLUTION — a symbol RESOLVES only if it is DECLARED
# --------------------------------------------------------------------------
# ⛔ FIXED 2026-09-03 (Compass, authorized by steele). The old test was
#     re.search(rf"(^|\s)(#def\s+|def\s+)?{sym}(\s|:|$)", body)
# and it was BLIND IN BOTH DIRECTIONS AT ONCE — a single line failing twice:
#
#   OVER-CERTIFYING — the declaration prefix was OPTIONAL `(...)?`, and the
#     search ran over the RAW body INCLUDING COMMENTS. So a symbol that appears
#     only in prose RESOLVED. MEASURED: `fjg-reconstruction-functor.rzk`
#     mentions `qfs-content-addresses` on two `--` comment lines that say it was
#     "struck as ANTIMATTER" and "STRUCK (E9) — REFUTED, then dead". The gate
#     certified a citation to a REFUTED postulate as PROVEN. That is the exact
#     failure the corpus already named: "a narrative result table reads exactly
#     like a citation", and a gate is the one instrument that must not repeat it.
#
#   UNDER-REPORTING — the prefix alternatives were only `#def` / `def`, so
#     `#define` and `#postulate` were unknown to it. MEASURED: `#define` is used
#     220 times across 20 proof files, two of which (`fjg-register-fold.rzk`,
#     `fjg-fiber-rederivation.rzk`) back live PROVEN claims. Matching `#def`
#     alone would call three real theorems "NOT A DECLARATION".
#
# ⇒ BOTH HALVES MATTER. Fixing only the over-certification would have made the
#   gate red on true claims; fixing only the keyword set would have left the
#   antimatter citation green. A gate wrong in two directions cannot be repaired
#   in one.
#
# THE RULE NOW: strip comments FIRST (per language), then require a genuine
# DECLARATION FORM. Presence is not resolution; a mention is not a definition.

# Agda/rzk: `--` opens a comment ONLY at line start or after whitespace, and
# only when not continued by a symbol character.
# ⚠ BOTH GUARDS ARE LOAD-BEARING, and each was found by measurement:
#   * the whitespace guard protects `val--ᵇ` (byte-ring.agda:229) — a real
#     identifier CONTAINING `--`. Naive stripping truncates that declaration to
#     `val` and the symbol vanishes.
#   * the symbol-char guard protects operator names like `-->`, which Agda
#     lexes as a token, not a comment.
_DASH_COMMENT = re.compile(r"(?:(?<=\s)|(?<=^))--+(?![!#$%&*+./<=>?@^|~\\:-])", re.M)
_SLASH_COMMENT = re.compile(r"//.*$", re.M)


def _strip_block(text, opener, closer, nested):
    """Pure: remove block comments, PRESERVING NEWLINES.

    Line structure must survive, because every declaration pattern is anchored
    with ^ — collapsing lines would silently merge a comment's tail onto the
    next declaration and change what matches.
    """
    out, i, depth = [], 0, 0
    while i < len(text):
        if (nested or not depth) and text.startswith(opener, i):
            depth += 1
            i += len(opener)
            continue
        if depth and text.startswith(closer, i):
            depth -= 1
            i += len(closer)
            continue
        ch = text[i]
        out.append(ch if not depth else ("\n" if ch == "\n" else ""))
        i += 1
    return "".join(out)


def _strip_dash_lines(text):
    """Pure: drop the `--` comment tail of each line, keeping the line."""
    keep = []
    for line in text.split("\n"):
        m = _DASH_COMMENT.search(line)
        keep.append(line[:m.start()] if m else line)
    return "\n".join(keep)


def strip_comments(body, ext):
    """Pure: return `body` with comments blanked, line structure preserved.

    Returns the body unchanged for extensions whose comment syntax we do not
    know. Under-stripping can only make the gate STRICTER — an unrecognised
    comment style still has to LOOK like a declaration to resolve — never laxer.
    """
    if ext in (".rzk", ".agda", ".lagda", ".hs"):
        return _strip_dash_lines(_strip_block(body, "{-", "-}", nested=True))
    if ext in (".rs", ".c", ".h", ".cs", ".cpp", ".java", ".js", ".ts", ".go"):
        return _SLASH_COMMENT.sub("", _strip_block(body, "/*", "*/", nested=False))
    return body


def declaration_patterns(ext, sym):
    """Pure: the regexes that constitute a DECLARATION of `sym` in this language.

    An empty list means: this file type has no declaration form we recognise, so
    no citation into it can RESOLVE. That is deliberate — a paper or a prose file
    contains mentions, never declarations, and certifying against one is the
    defect this function exists to stop.
    """
    s = re.escape(sym)
    if ext in (".rzk",):
        # the four declaration keywords MEASURED in the corpus:
        #   #def 4770 · #postulate 3932 · #define 220 · #data 2
        return [rf"^[ \t]*#(?:def|define|postulate|data)[ \t]+{s}(?![A-Za-z0-9_'\-])"]
    if ext in (".agda", ".lagda"):
        return [
            # a type signature, top-level or inside a postulate/where/record block
            rf"^[ \t]*{s}[ \t]*:(?!:)",
            rf"^[ \t]*(?:data|record|module|pattern)[ \t]+{s}(?![A-Za-z0-9_'\-])",
        ]
    if ext in (".rs",):
        vis = r"(?:pub(?:[ \t]*\([^)]*\))?[ \t]+)?"
        return [
            rf"^[ \t]*{vis}(?:default[ \t]+)?(?:async[ \t]+)?(?:unsafe[ \t]+)?"
            rf"(?:extern[ \t]+\"[^\"]*\"[ \t]+)?"
            rf"(?:fn|struct|enum|const|static|type|trait|mod|union)[ \t]+{s}\b",
            rf"^[ \t]*macro_rules![ \t]*{s}\b",
            # a PUBLIC struct/enum field. `pub` is required on purpose: a bare
            # `name: value,` line is indistinguishable from a struct-literal
            # INITIALIZER without parsing, and accepting those would re-open the
            # mention-as-declaration hole one language over.
            rf"^[ \t]*pub(?:[ \t]*\([^)]*\))?[ \t]+{s}[ \t]*:",
        ]
    return []


def resolve_symbol(body, sym, path):
    """Pure: (body, symbol, path) -> (VERDICT, detail).

    VERDICT is 'DECLARED' | 'SYMBOL NOT FOUND' | 'NOT A DECLARATION'
            | 'NO DECLARATION FORM FOR THIS FILE TYPE'.
    The three failing verdicts are DISTINCT on purpose: "absent", "mentioned but
    never defined" and "we cannot tell" are different findings, and collapsing
    them into one message is how a real obligation gets read as a typo.
    """
    ext = os.path.splitext(path)[1].lower()
    pats = declaration_patterns(ext, sym)
    stripped = strip_comments(body, ext)
    if not pats:
        return ("NO DECLARATION FORM FOR THIS FILE TYPE",
                f"'{ext or 'no extension'}' has no declaration syntax this gate "
                f"recognises, so the citation cannot RESOLVE. Cite a proof or a "
                f"source declaration, not a prose file.")
    for p in pats:
        if re.search(p, stripped, re.M):
            return ("DECLARED", "")
    # distinguish "never appears" from "appears only in prose" — different findings
    loose = rf"(?<![A-Za-z0-9_'\-]){re.escape(sym)}(?![A-Za-z0-9_'\-])"
    if re.search(loose, stripped, re.M):
        return ("NOT A DECLARATION",
                "It occurs in the file but never as a declaration. A MENTION IS "
                "NOT A DEFINITION — a name in prose, a struck row or a narrative "
                "result table reads exactly like a citation and is not one.")
    if re.search(loose, body, re.M):
        return ("NOT A DECLARATION",
                "Every occurrence is inside a COMMENT. Commented-out or "
                "narrative text cannot discharge a proof obligation; if the "
                "symbol was struck, the claim citing it is not PROVEN.")
    return ("SYMBOL NOT FOUND",
            "It does not occur in that file at all. A fabricated citation is "
            "worse than an absent one.")


def resolve_citation(cid, value, attr, cites_phrase):
    """Pure: (claim id, attribute value, attribute name, phrase) -> error | None.

    FACTORED OUT 2026-09-07 when REFUTED was added, and the factoring is the
    point rather than tidiness: `witness=` must be held to the IDENTICAL
    standard as `proof=`, because a refutation you cannot point at is the
    vacuity defect wearing a new costume. Two copies of this logic would drift,
    and the copy that drifts laxer is always the newer one.

    The message wording is parameterised so the PROVEN path's four messages are
    BYTE-IDENTICAL to what they were before the factoring — an instrument whose
    output changes when you refactor it cannot be diffed across the change.
    """
    m = re.match(r"^(.+?)#(.+)$", value)
    if not m:
        return f"claim '{cid}' {attr}='{value}' is not of the form FILE#SYMBOL"
    pf, sym = m.group(1), m.group(2)
    cands = [pf] + [os.path.join(r, pf) for r in
                    ("/git/thecowboyai/hatter", "/home/steele/.claude")]
    hit = next((x for x in cands if os.path.isfile(x)), None)
    if not hit:
        return (f"claim '{cid}' {cites_phrase} '{pf}' — FILE NOT FOUND. "
                f"A fabricated citation is worse than an absent one.")
    try:
        with open(hit, encoding="utf-8", errors="replace") as fh:
            body = fh.read()
    except OSError as e:
        # errors as VALUES: an unreadable citation target does not RESOLVE, and
        # saying so beats a traceback out of the analysis.
        return f"claim '{cid}' {cites_phrase} '{pf}' — UNREADABLE ({e})."
    verdict, detail = resolve_symbol(body, sym, hit)
    if verdict != "DECLARED":
        return f"claim '{cid}' {cites_phrase} symbol '{sym}' in '{pf}' — {verdict}. {detail}"
    return None


def check_file(path, rel, strict=False):
    """Return (claims, defects) for one SVG. Pure: inputs in, values out.

    `path` is what the caller named (used for stdout, so output is unchanged).
    `rel` is the path RELATIVE to the scanned root, and is the only one that
    reaches the artifact — an absolute path would embed this machine.

    A defect is a dict {display, rel, claim, stage, message}; `message` carries NO
    path prefix, because the prefix differs by how the gate was invoked and would
    make the artifact non-deterministic. The printer re-attaches it.
    A claim is a dict; see build_result for the emitted shape.
    """
    defects, claims = [], []

    def defect(msg, claim=None, stage=1, kind=None):
        # `kind` is INTERNAL — it never reaches the artifact, whose defect
        # records keep their four fields. It exists so build_result can tell a
        # VACUOUS claim (statable, trivially true) from a MALFORMED one (not
        # statable at all); collapsing those two would be the same species of
        # error as summing the statuses.
        defects.append({"display": path, "rel": rel, "claim": claim,
                        "stage": stage, "message": msg, "kind": kind})

    try:
        root = ET.parse(path).getroot()
    except ET.ParseError as e:
        defect(f"XML PARSE ERROR — {e}")
        return [], defects
    except OSError as e:
        # errors as VALUES: an unreadable file is a defect, not a traceback.
        defect(f"UNREADABLE — {e}")
        return [], defects

    d = find_meta(root)
    if d is None:
        defect("NO <metadata><diagram> BLOCK — the diagram is UNCHECKABLE. "
               "A picture asserting structure it does not declare cannot be verified.")
        return [], defects

    nodes, arrows, terminal_nodes = {}, {}, set()
    for n in d:
        tag = n.tag.split("}")[-1]
        if tag == "node":
            nid = n.get("id")
            if nid in nodes:
                defect(f"duplicate node id '{nid}'")
            nodes[nid] = n.get("label", "")
            is_terminal, terr = terminal_declaration(n)
            if terr:
                defect(f"node '{nid}' {terr}")
            elif is_terminal:
                terminal_nodes.add(nid)
        elif tag == "arrow":
            aid = n.get("id")
            if aid in arrows:
                defect(f"duplicate arrow id '{aid}'")
            arrows[aid] = (n.get("src"), n.get("tgt"), n.get("label", ""))

    # --- STAGE 1a: every arrow's endpoints are declared nodes -----------------
    for aid, (src, tgt, lbl) in arrows.items():
        for end, which in ((src, "src"), (tgt, "tgt")):
            if end not in nodes:
                defect(f"arrow '{aid}' has {which}='{end}' which is not a declared node "
                       f"— a DANGLING ARROW. Its type cannot be formed.")

    # --- the commuting claims -------------------------------------------------
    for c in d:
        if not c.tag.endswith("commutes"):
            continue
        cid = c.get("id", "?")
        frm, to = c.get("from"), c.get("to")
        pa = [x for x in (c.get("pathA") or "").split(";") if x]
        pb = [x for x in (c.get("pathB") or "").split(";") if x]
        proof = c.get("proof")
        status = (c.get("status") or "").upper()
        reason = c.get("reason")
        witness = c.get("witness")
        claims.append({"file": rel, "rel": rel, "id": cid, "from": frm, "to": to,
                       "pathA": pa, "pathB": pb, "status": status,
                       "proof": proof, "reason": reason, "witness": witness})

        if not pa or not pb:
            defect(f"claim '{cid}' names fewer than two paths — nothing to equate.", cid, 1)
            continue

        # --- STAGE 1b: each path must actually COMPOSE ------------------------
        def walk(pathids, name):
            """Return (start, end) or None, appending a defect on failure."""
            missing = [a for a in pathids if a not in arrows]
            if missing:
                defect(f"claim '{cid}' {name} references undeclared arrow(s) {missing}", cid, 1)
                return None
            for i in range(len(pathids) - 1):
                s1, t1, _ = arrows[pathids[i]]
                s2, t2, _ = arrows[pathids[i + 1]]
                if t1 != s2:
                    defect(
                        f"claim '{cid}' {name} DOES NOT COMPOSE — "
                        f"'{pathids[i]}' ends at '{t1}' but '{pathids[i+1]}' starts at '{s2}'. "
                        f"This is not a path; the equation cannot be stated.", cid, 1)
                    return None
            return arrows[pathids[0]][0], arrows[pathids[-1]][1]

        ea, eb = walk(pa, "pathA"), walk(pb, "pathB")
        if ea is None or eb is None:
            continue

        # --- STAGE 1c: the two paths must be PARALLEL -------------------------
        if ea[0] != eb[0] or ea[1] != eb[1]:
            defect(
                f"claim '{cid}' compares NON-PARALLEL paths — "
                f"pathA runs {ea[0]}→{ea[1]}, pathB runs {eb[0]}→{eb[1]}. "
                f"Two arrows with different domain or codomain are not of the same type, "
                f"so they cannot be equal. This is the diagram analogue of a type error.", cid, 1)
            continue
        if frm and ea[0] != frm:
            defect(f"claim '{cid}' declares from='{frm}' but its paths start at '{ea[0]}'", cid, 1)
        if to and ea[1] != to:
            defect(f"claim '{cid}' declares to='{to}' but its paths end at '{ea[1]}'", cid, 1)

        # --- STAGE 1d: the codomain must not be TERMINAL ----------------------
        # ⛔ ADDED 2026-09-07. Hom(X, 1) is a SINGLETON, so any two morphisms
        # into a terminal object are equal BY THE UNIVERSAL PROPERTY, before
        # anything the diagram draws is consulted. Such a square commutes for
        # free and carries ZERO information: it cannot fail, so it is not an
        # assertion. That is `fn verify() -> bool { true }` again — the very
        # thing this gate was built to stop — arriving through the codomain
        # rather than through a typed string.
        #
        # THE DEFECT THAT MOTIVATED IT, and it is a real one in the corpus:
        #   NoIntersection := Σ (witness : Point two), witness = witness
        # is CONTRACTIBLE, so `vesica-d-gt-2r-yields-no-intersection` typechecks
        # whether or not the circles intersect. The proof term is honest; the
        # TYPE says nothing. A diagram whose codomain is that object inherits
        # exactly the same emptiness.
        #
        # ⚠ THE DOMAIN IS NOT SYMMETRIC AND MUST NOT BE TREATED AS IF IT WERE.
        # Hom(1, X) is the POINTS of X — richly non-trivial. Only the CODOMAIN
        # trivialises, and a rule applied to both ends would reject sound work.
        #
        # ⚠ NOR IS AN INTERMEDIATE TERMINAL NODE REJECTED. A path factoring as
        # X -> 1 -> Y is the CONSTANT morphism, and "this composite is constant"
        # is a substantive, falsifiable claim. Rejecting it would be
        # over-application; the triviality lives strictly at the codomain.
        if ea[1] in terminal_nodes:
            defect(
                f"claim '{cid}' has codomain '{ea[1]}', a node DECLARED TERMINAL "
                f"/ CONTRACTIBLE. Hom(-, 1) is a singleton, so the two paths are "
                f"equal by the universal property and this square commutes "
                f"TRIVIALLY — it cannot fail, so it asserts nothing. This is not "
                f"a claim to prove; it is a claim to DELETE. If there is real "
                f"content here, it lives in a square whose codomain is not "
                f"terminal — restate it there.", cid, 1, kind="vacuous")
            continue

        # --- STAGE 2: truth is discharged by CITATION, or openly declared -----
        # ⛔ TWO STATUSES ADDED 2026-08-23 ON COMPASS'S RULING. UNPROVEN was the
        # only honest alternative to PROVEN, and it was WRONG for both real
        # claims, in OPPOSITE directions:
        #   * the olog's verdict-square is UNPROVABLE AS STATED — it quantifies
        #     over "an algebraic derivation", a carrier with no boundary, so no
        #     exhaustive statement can be made over it. UNPROVEN implies "not yet";
        #     this one is "not ever, in this form".
        #   * the cospan's pushout-square is DEFINITIONAL — commutativity is part
        #     of the DATA of a colimiting cocone, not a consequence of it. Writing
        #     a #def for it would be re-proving the peer-accepted.
        # Collapsing all three into UNPROVEN hides a real obligation behind a
        # non-obligation, which is the opposite of what this gate is for.
        #
        # ⛔ A FIFTH STATUS ADDED 2026-09-07 ON STEELE'S AUTHORISATION: REFUTED,
        # the claim that the square does NOT commute. The four existing statuses
        # could not express it. PROVEN is the opposite claim; UNPROVEN says "not
        # yet", which is FALSE of a square with a counterexample in hand;
        # UNPROVABLE-AS-STATED says the question cannot be posed, when in fact it
        # was posed and ANSWERED; DEFINITIONAL says it holds by construction.
        # Filing a counterexample under any of them loses the counterexample.
        #
        # ⛔ AND IT CARRIES THE HEAVIEST BURDEN OF THE FIVE, NOT THE LIGHTEST.
        # ¬P is a positive claim needing a term, so witness= is MANDATORY and is
        # resolved by the SAME instrument as proof=. A REFUTED with no witness is
        # REJECTED, never warned about: an unpointable refutation is precisely
        # `fn verify() -> bool { true }` with the sign flipped, and letting it
        # pass would install the vacuity defect inside the mechanism built to
        # catch it.
        #
        # ⇒ CONSEQUENCE THE AUTHOR MUST KNOW: a square cannot be marked REFUTED
        # before its witness declaration EXISTS. That ordering is proofs-first,
        # working as intended — not an obstacle to route around by downgrading
        # the claim to UNPROVEN.
        if status == "REFUTED":
            if proof:
                defect(f"claim '{cid}' is marked REFUTED yet cites proof='{proof}'. "
                       f"A refutation is not a proof of the square — put the "
                       f"counterexample in witness= and remove proof=.", cid, 2)
            if not witness:
                defect(
                    f"claim '{cid}' is marked REFUTED with no witness= attribute. "
                    f"REFUTED asserts the square does NOT commute, which is a "
                    f"POSITIVE claim requiring a term. A refutation you cannot "
                    f"point at cannot fail, so it is the vacuity defect in a new "
                    f"costume. Cite the counterexample as witness=FILE#SYMBOL.",
                    cid, 2)
                continue
            err = resolve_citation(cid, witness, "witness", "cites WITNESS")
            if err:
                defect(err, cid, 2)
            continue
        # a witness on anything else is a confusion of claim and counterclaim
        if witness:
            defect(f"claim '{cid}' has status='{status or 'MISSING'}' yet carries "
                   f"witness='{witness}'. A witness refutes; only REFUTED takes "
                   f"one. Pick which claim you are making.", cid, 2)
        if status in ("UNPROVEN", "UNPROVABLE-AS-STATED", "DEFINITIONAL"):
            if proof:
                defect(f"claim '{cid}' is marked {status} yet cites a proof — pick one.", cid, 2)
            if status in ("UNPROVABLE-AS-STATED", "DEFINITIONAL") and not reason:
                defect(
                    f"claim '{cid}' is marked {status} with no reason= attribute. "
                    f"Both statuses EXCUSE the claim from proof, so each must say WHY — "
                    f"an unexplained exemption is how a real obligation disappears.", cid, 2)
            continue
        if status != "PROVEN":
            defect(
                f"claim '{cid}' has status='{status or 'MISSING'}'. "
                f"Must be PROVEN (resolving citation), REFUTED (+resolving witness), "
                f"UNPROVEN, UNPROVABLE-AS-STATED (+reason) or DEFINITIONAL (+reason). "
                f"An unmarked claim is an assertion that cannot fail.", cid, 2)
            continue
        if not proof:
            defect(f"claim '{cid}' is marked PROVEN but cites no proof.", cid, 2)
            continue
        # citation must resolve: FILE#SYMBOL, file exists, symbol appears in it
        err = resolve_citation(cid, proof, "proof", "cites")
        if err:
            defect(err, cid, 2)

    # --- VACUITY GUARD -------------------------------------------------------
    # ⛔ FOUND 2026-08-23 by sdlc-expert, USING the gate: a <diagram> block with
    # zero nodes and zero claims produced no defects and exited 0 — so GREEN WAS
    # ACHIEVABLE BY DECLARING NOTHING, while the picture still drew "PATH A =
    # PATH B". That is this gate failing toward green, the defect it exists to
    # catch, in itself. A gate that can be satisfied by an empty declaration is
    # not a gate.
    txt_all = rendered_text(root)
    if not nodes:
        defect("<diagram> declares NO NODES — an empty declaration cannot be "
               "checked, and green here would mean nothing. Declare the graph the "
               "picture draws.")
    # the picture must not ASSERT a commuting claim the metadata does not DECLARE
    # ⛔ TIGHTENED IMMEDIATELY AFTER THE FIRST RUN. The first pattern was
    # /PATH A|PATH B|commut/ and it FALSE-POSITIVED on both string diagrams,
    # matching "SPECIAL COMMUTATIVE FROBENIUS" — a STRUCTURE NAME, not a claim.
    # A gate that flags the discipline working is worse than no gate. Match only
    # a real commuting-square ASSERTION: the path labels, the verb, or the noun
    # phrase "commutative diagram". The adjective in "commutative Frobenius
    # monoid" must NOT fire.
    asserts_commuting = re.search(
        r"PATH\s*[AB]\b|\bcommut(es|ing)\b|commutative\s+diagram", txt_all, re.I)
    declared = [c for c in d if c.tag.endswith("commutes")]
    if asserts_commuting and not declared:
        defect("the RENDERED PICTURE asserts commuting (text matches "
               "/PATH A|PATH B|commut/) but the metadata declares NO <commutes> claim. "
               "An undeclared claim is unverifiable — declare it, or stop drawing it.")

    # --- optional: do declared labels actually appear in the picture? ---------
    if strict:
        txt = rendered_text(root)
        for nid, lbl in nodes.items():
            if lbl and lbl not in txt:
                defect(f"node '{nid}' declares label '{lbl}' which appears in NO "
                       f"rendered text — the metadata and the picture disagree.")
    return claims, defects


# --------------------------------------------------------------------------
# collection + the deterministic artifact
# --------------------------------------------------------------------------

def collect_files(args):
    """Return [(path_as_named, path_relative_to_its_scanned_root)], sorted.

    The RELATIVE name is what the artifact stores. `agents/diagrams/x.svg`
    scanned from /home/steele/.claude and from any clone elsewhere both yield
    `x.svg`, so the artifact does not embed the machine that produced it.
    """
    out = []
    for a in args:
        if os.path.isdir(a):
            for f in sorted(glob.glob(os.path.join(a, "*.svg"))):
                out.append((f, os.path.relpath(f, a)))
        else:
            out.append((a, os.path.basename(a)))
    return sorted(out, key=lambda p: (p[1], p[0]))


def analyze(files, strict=False):
    """Pure: (files, strict) -> (claims, defects). No globals, no caching."""
    claims, defects = [], []
    for path, rel in files:
        c, d = check_file(path, rel, strict=strict)
        claims += c
        defects += d
    return claims, defects


def claim_key(c):
    return (c["rel"], c["id"], c["status"], c["from"] or "", c["to"] or "")


def defect_key(d):
    return (d["rel"], d["claim"] or "", d["stage"], d["message"])


def build_result(files, claims, defects, strict):
    """Pure: build the JSON-ready verdict. Total ordering on every list."""
    by_claim = {}
    for d in sorted(defects, key=defect_key):
        if d["claim"] is not None:
            by_claim.setdefault((d["rel"], d["claim"]), []).append(d)

    out_claims = []
    for c in sorted(claims, key=claim_key):
        mine = by_claim.get((c["rel"], c["id"]), [])
        s1 = [d["message"] for d in mine if d["stage"] == 1]
        s2 = [d["message"] for d in mine if d["stage"] == 2]
        # ⛔ THREE VERDICTS, NOT TWO, AND THEY ARE NOT ORDERED BY SEVERITY OF
        # TONE BUT BY WHAT THEY MEAN. MALFORMED = the equation cannot be STATED
        # (a type error). VACUOUS = it can be stated, is trivially true, and
        # asserts nothing. Collapsing VACUOUS into MALFORMED would tell an author
        # to fix the paths when the repair is to DELETE the claim; collapsing it
        # into WELL-FORMED would be the 2026-08-23 regression again, one field
        # over. MALFORMED wins where both apply — a claim that cannot be stated
        # has no codomain worth judging.
        vac = [d for d in mine if d.get("kind") == "vacuous"]
        hard = [d for d in mine if d["stage"] == 1 and d.get("kind") != "vacuous"]
        verdict = "MALFORMED" if hard else ("VACUOUS" if vac else "WELL-FORMED")
        out_claims.append({
            "file": c["rel"],
            "id": c["id"],
            "from": c["from"],
            "to": c["to"],
            "pathA": c["pathA"],
            "pathB": c["pathB"],
            "status": c["status"] or "MISSING",
            "proof": c["proof"],
            "reason": c["reason"],
            "witness": c["witness"],
            # STAGE 1 is the whole of what this gate DECIDES; stage 2 is
            # citation-resolution only, and neither is a truth verdict.
            "stage1_verdict": verdict,
            "stage1_defects": s1,
            "stage2_defects": s2,
        })

    by_status = {s: 0 for s in CANONICAL_STATUSES}
    for c in out_claims:
        by_status[c["status"]] = by_status.get(c["status"], 0) + 1

    return {
        "artifact": ARTIFACT_KIND,
        "version": ARTIFACT_VERSION,
        "options": {"strict": bool(strict)},
        "files": sorted(rel for _, rel in files),
        "claims": out_claims,
        "defects": [{"file": d["rel"], "claim": d["claim"],
                     "stage": d["stage"], "message": d["message"]}
                    for d in sorted(defects, key=defect_key)],
        "summary": {
            "files": len(files),
            "claims": len(out_claims),
            "defects": len(defects),
            "malformed_claims": sum(1 for c in out_claims
                                    if c["stage1_verdict"] == "MALFORMED"),
            # counted SEPARATELY, never added to malformed_claims
            "vacuous_claims": sum(1 for c in out_claims
                                  if c["stage1_verdict"] == "VACUOUS"),
            # ⛔ PER STATUS, NEVER SUMMED. See CANONICAL_STATUSES.
            "by_status": by_status,
        },
    }


def write_result(result, dest):
    """Return None on success, or an error STRING (errors as values)."""
    try:
        with open(dest, "w", encoding="utf-8") as fh:
            json.dump(result, fh, sort_keys=True, indent=2, ensure_ascii=False)
            fh.write("\n")
    except OSError as e:
        return f"could not write '{dest}': {e}"
    return None


def read_result(src):
    """Return (result, None) or (None, error STRING)."""
    try:
        with open(src, encoding="utf-8") as fh:
            return json.load(fh), None
    except OSError as e:
        return None, f"could not read '{src}': {e}"
    except json.JSONDecodeError as e:
        return None, f"'{src}' is not valid JSON — {e}"


def _fmt(v):
    return json.dumps(v, sort_keys=True, ensure_ascii=False)


def diff_result(expected, actual):
    """Pure: return a SORTED list of drift lines. Empty list = identical.

    Keyed by (file, claim id) rather than by position, so an inserted or removed
    claim reports as MISSING/EXTRA instead of shifting every later comparison.
    """
    drift = []

    for k in ("artifact", "version"):
        if expected.get(k) != actual.get(k):
            drift.append(f"{k}: expected {_fmt(expected.get(k))}, actual {_fmt(actual.get(k))}")
    if expected.get("options") != actual.get("options"):
        drift.append(f"options: expected {_fmt(expected.get('options'))}, "
                     f"actual {_fmt(actual.get('options'))}")

    ef, af = set(expected.get("files") or []), set(actual.get("files") or [])
    for f in sorted(ef - af):
        drift.append(f"files: '{f}' MISSING — it is in the artifact but was not scanned")
    for f in sorted(af - ef):
        drift.append(f"files: '{f}' EXTRA — it was scanned but is not in the artifact")

    ec = {(c.get("file"), c.get("id")): c for c in (expected.get("claims") or [])}
    ac = {(c.get("file"), c.get("id")): c for c in (actual.get("claims") or [])}
    for k in sorted(ec.keys() - ac.keys()):
        drift.append(f"claims[{k[0]}#{k[1]}]: MISSING — recorded, but no such claim now")
    for k in sorted(ac.keys() - ec.keys()):
        drift.append(f"claims[{k[0]}#{k[1]}]: EXTRA — present now, but not recorded")
    for k in sorted(ec.keys() & ac.keys()):
        e, a = ec[k], ac[k]
        for field in sorted(set(e.keys()) | set(a.keys())):
            if e.get(field) != a.get(field):
                drift.append(f"claims[{k[0]}#{k[1]}].{field}: "
                             f"expected {_fmt(e.get(field))}, actual {_fmt(a.get(field))}")

    def dkey(d):
        return (d.get("file"), d.get("claim") or "", d.get("stage"), d.get("message"))
    ed = {dkey(d) for d in (expected.get("defects") or [])}
    ad = {dkey(d) for d in (actual.get("defects") or [])}
    for d in sorted(ed - ad):
        drift.append(f"defects: RESOLVED — recorded for {d[0]} but no longer reported: {d[3]}")
    for d in sorted(ad - ed):
        drift.append(f"defects: NEW — reported for {d[0]} but not in the artifact: {d[3]}")

    es, as_ = expected.get("summary") or {}, actual.get("summary") or {}
    for field in sorted(set(es.keys()) | set(as_.keys())):
        if es.get(field) != as_.get(field):
            drift.append(f"summary.{field}: expected {_fmt(es.get(field))}, "
                         f"actual {_fmt(as_.get(field))}")

    return sorted(drift)


# --------------------------------------------------------------------------

def parse_argv(argv):
    """Pure: argv -> (positionals, flags, opts). Unknown --flags stay ignored."""
    args, flags, opts, i = [], set(), {}, 0
    while i < len(argv):
        a = argv[i]
        if a.startswith("--"):
            name, _, inline = a.partition("=")
            if name in ("--emit-result", "--check-result"):
                if inline:
                    opts[name] = inline
                elif i + 1 < len(argv):
                    opts[name] = argv[i + 1]
                    i += 1
                else:
                    opts[name] = None
            else:
                flags.add(a)
        else:
            args.append(a)
        i += 1
    return args, flags, opts


def main():
    args, flags, opts = parse_argv(sys.argv[1:])
    for name in ("--emit-result", "--check-result"):
        if name in opts and not opts[name]:
            print(f"{name} needs a FILE argument")
            return 2
    if not args:
        print(__doc__.strip().split("USAGE")[1]); return 2

    files = collect_files(args)
    if not files:
        print("no .svg found — NOTHING TO JUDGE"); return 2

    strict = "--strict" in flags
    all_claims, all_defects = analyze(files, strict=strict)
    result = build_result(files, all_claims, all_defects, strict)

    # --- diff mode: recompute and compare against the saved artifact ----------
    if "--check-result" in opts:
        saved, err = read_result(opts["--check-result"])
        if err:
            print(f"⛔ {err}")
            return 1
        drift = diff_result(saved, result)
        if not drift:
            print(f"IDENTICAL — {len(result['claims'])} claim(s) across "
                  f"{len(files)} file(s) match '{opts['--check-result']}'.")
            print("  /!\\ Matching means the verdict is STABLE, not that it is TRUE.")
            return 0
        print(f"⛔ DRIFT against '{opts['--check-result']}' — {len(drift)} difference(s):")
        for line in drift:
            print(f"  ⛔ {line}")
        return 1

    # emission happens BEFORE any reporting branch, so --emit-result produces the
    # same bytes whatever else was asked for. The artifact is a function of the
    # inputs, never of the display mode.
    emit_err = None
    if "--emit-result" in opts:
        emit_err = write_result(result, opts["--emit-result"])

    if "--list" in flags:
        for c in sorted(all_claims, key=claim_key):
            # a REFUTED claim's citation lives in witness=, so --list must show
            # it — a listing that prints an empty citation column for the one
            # status carrying the heaviest burden reads as though it had none.
            cite = c["proof"] or (f"witness {c['witness']}" if c["witness"] else "")
            print(f"  {os.path.basename(c['rel']):42s} {c['id']:14s} "
                  f"{c['from']}→{str(c['to']):22s} {c['status']:9s} {cite}")
        print(f"  {len(all_claims)} commuting claim(s) across {len(files)} file(s)")
        if "--emit-result" in opts:
            print(f"  wrote {opts['--emit-result']}" if not emit_err else f"  ⛔ {emit_err}")
        return 1 if emit_err else 0

    print(f"commutation gate: {len(files)} diagram(s), {len(all_claims)} commuting claim(s)")
    if not all_defects:
        s = result["summary"]
        print(f"OK — {s['claims']} claim(s), {s['malformed_claims']} malformed, "
              f"{s['vacuous_claims']} vacuous. "
              f"Status breakdown (NEVER summed):")
        # ⛔ EACH STATUS WITH ITS OWN COUNT. The old line printed
        # "{n} openly UNPROVEN" for every non-PROVEN claim, which reported 2
        # DEFINITIONAL + 2 UNPROVABLE-AS-STATED as "4 openly UNPROVEN" — the
        # exact collapse the STAGE 2 comment above forbids, and it fails in the
        # direction of looking ordinary.
        for st in list(CANONICAL_STATUSES) + sorted(
                k for k in s["by_status"] if k not in CANONICAL_STATUSES):
            note = {"PROVEN": "with resolving citations",
                    "UNPROVEN": "obligation open — not yet proven",
                    "UNPROVABLE-AS-STATED": "exempt, with a stated reason",
                    "DEFINITIONAL": "exempt, with a stated reason",
                    "REFUTED": "asserts the square does NOT commute, "
                               "with a resolving witness"}.get(st, "unrecognised status")
            print(f"      {st:22s} {s['by_status'][st]:3d}   ({note})")
        print()
        print("  /!\\ GREEN MEANS: every claim is WELL-FORMED (paths compose, paths are")
        print("      parallel, the codomain is not a DECLARED terminal object) and is")
        print("      either cited, or REFUTED with a resolving witness, or openly carries")
        print("      one of the three exemption statuses. It does NOT mean the diagrams")
        print("      are TRUE — that verdict is the proof gate's, and this gate never")
        print("      re-runs the prover. It also does NOT mean no codomain is terminal:")
        print("      the vacuity rule fires only on a DECLARED one and cannot compute")
        print("      contractibility. See WHAT IT CANNOT SEE in the module docstring.")
        if "--emit-result" in opts:
            print()
            print(f"  wrote {opts['--emit-result']}" if not emit_err else f"  ⛔ {emit_err}")
        return 1 if emit_err else 0
    print()
    # stdout keeps PRODUCTION order (already deterministic: files are scanned
    # sorted). The artifact is what carries the total sort key; re-ordering the
    # printout would change existing gate output for no gain.
    for d in all_defects:
        print(f"  ⛔ {d['display']}: {d['message']}")
    print(f"\n{len(all_defects)} defect(s).")
    if "--emit-result" in opts:
        print(f"  wrote {opts['--emit-result']}" if not emit_err else f"  ⛔ {emit_err}")
    return 1


if __name__ == "__main__":
    sys.exit(main())
