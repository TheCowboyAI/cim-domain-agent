<!-- Copyright (c) 2025-2026 - Cowboy AI, Inc. -->

# CIM Diagram Theme — the token set every diagram draws from

*In the Universe of Bytes.*

**This file exists so diagrams are CONSISTENT rather than individually pretty.** A palette
chosen per-diagram makes two diagrams of the same structure look like different structures,
which is a semantic defect and not a cosmetic one. Draw from these tokens or state why not.

---

## 0. ⛔⛔ WHAT THIS FILE IS — A PROJECTION. **THE THEME IS A FUNCTOR.**

**steele 2026-09-06:** *"theme.md is a functor for layout hiding in an md file, who makes
'themes' in markdown, not a single library I know of."* · *"is css another projection? I think
the language we use for themes is **xml+xslt**"* · *"same with gtk... projection from something
else."*

**A THEME MAPS SEMANTIC ROLES TO VISUAL TOKENS AND IS STRUCTURE-PRESERVING** — which is the
paragraph directly above, stated as prose. **That is functoriality.** The tell is in this file:
its tokens are **already CSS custom-property names** (`--type`, `--aspect`, `--frobenius`,
`--antimatter`). It is a stylesheet written as a document about a stylesheet.

| | |
|---|---|
| **the FUNCTOR** | the token set + its transformation. For an XML dialect that is **XSLT** — structure-to-structure, composable, and written in the artifact's own grammar |
| **PROJECTIONS of it** | **this markdown** · CSS (and CSS is itself emitted — SCSS compiles to it) · GTK css/`.theme` · Qt stylesheets · design-token JSON |
| ⇒ **so** | **ONE FUNCTOR, MANY PROJECTIONS.** Author the functor; project to each target. **Never author at the egress end** |

⛔ **THIS FILE IS A PROJECTION, NOT THE AUTHORITY.** Read it for the token *meanings*; do not
hand-transcribe it into an artifact. **A theme written as markdown is narration where a
structural instrument already exists** — see `~/.claude/memory/instrument-discipline.md`.

⚠ **THE XSLT FORM IS OWED AND DOES NOT YET EXIST** — stated so no one cites a file that isn't
there. Until it does, `papers/diagram-theme.css` (in hatter) is the nearest machine-consumable
form and is already recorded as superseding any inline `<style>`.

⭐ **AND IT MATTERS MOST FOR THE TWO RENDERERS.** Antenna is C#→WASM (DOM/CSS); Wonderland is
Rust→native. **Two renderers means two PROJECTIONS**, and the reach will be to author a theme
in each. They then drift, and the same structure looks like different structures **across
renderers** — this file's own stated defect, relocated where it is harder to see.
**Before writing any theme code: am I AUTHORING a theme, or PROJECTING one?**

---

## 1. The calculus determines the colour — colour is SEMANTIC, never decoration

| element | token | hex | used for |
|---|---|---|---|
| **type box** (olog) | `--type` | `#ffffff` fill, `#1a1a1a` stroke | boxes are TYPES; neutral, because a type is not an action |
| **aspect** (olog arrow) | `--aspect` | `#1a1a1a` | a functional relationship |
| **fact region** (olog) | `--fact` | `#f4f7fb` fill, `#9db4d0` stroke | a commutative diagram — a region, so the commuting is visible |
| **layout** (finite limit) | `--layout` | `#fbf4f4` fill, `#d09d9d` stroke | products, pullbacks, fibre products |
| **grouping** (finite colimit) | `--grouping` | `#f7f2fb` fill, `#b393d3` stroke | coproducts, quotients, **PUSHOUTS** |
| **morphism box** (string diagram) | `--morphism` | `#eef4fa` fill, `#1a1a1a` stroke | boxes are MORPHISMS — tinted, because it acts |
| **wire** (string diagram) | `--wire` | `#1a6ea8` | carries an OBJECT. Never drawn with an arrowhead |
| **Frobenius node** | `--frobenius` | `#c1272d` fill, `#7d1418` stroke | where a wire SPLITS or MERGES |
| **cup / cap** (compact closed) | `--bend` | `#6a3d9a` | a wire that BENDS; must satisfy the snake equation |
| **apex** (cospan) | `--apex` | `#eef7f1` fill, `#1a7a45` stroke | the open system itself |
| **leg** (cospan) | `--leg` | `#1a1a1a` | a MAP INTO the apex |
| **decoration** (cospan) | `--decoration` | `#f7fbf8` fill, `#1a7a45` dashed | `1 → F N`; dashed, because it is carried and not contained |
| **law / caveat panel** | `--law` | `#fbfbf4` fill, `#c8c8a8` stroke | what must hold, and what the diagram cannot show |
| **antimatter** | `--antimatter` | `#6a3d9a` | a refuted path, kept with its ruling |

⛔ **AN ARROWHEAD IS A SEMANTIC COMMITMENT.** Put one on an olog aspect and on a cospan leg —
both are genuine morphisms with a domain and codomain. **Never put one on a string-diagram
wire**: in a hypergraph category a morphism is indexed by a finite SET of objects, not a
domain/codomain pair (Fong–Spivak arXiv:1806.08304), so an arrowhead asserts structure the
category does not have.

---

## 2. Type scale and geometry

| | value |
|---|---|
| font stack | `DejaVu Sans, Verdana, sans-serif` (present on NixOS; no web fetch) |
| title | 15px bold |
| subtitle / legend | 11.5px `#555` |
| box label | 13px |
| edge label | 11.5px `#333` |
| caveat | 10.5–11px `#777`, italic |
| corner radius | 4px boxes, 6px regions |
| stroke | 1.4px boxes, 2px wires and legs, 2.2px emphasis |
| background | `#ffffff` — always explicit, never transparent |

---

## 3. Animation is ANTENNA-TIER — and that rule reaches into the SVG

**steele, recorded in `CLAUDE.md`:** the substrate emits SEMANTIC STATE; the antenna renders,
animates and interpolates ALL locally. *"Anything time-driven — animation, interpolation,
easing, frame pacing — is antenna-tier."*

⇒ **So animation belongs INSIDE the SVG, driven by the viewer's own clock** — SMIL
(`<animate>`, `<animateTransform>`) or CSS `@keyframes` in a `<style>` block. Both run in the
renderer.

⛔ **NEVER generate a frame sequence, a per-frame export, or a diagram whose content depends
on wall-clock at authoring time.** That is putting a clock in the fold, and it is the exact
design the antenna split exists to forbid.

**What animation is FOR here — it must carry meaning, or it is noise:**

| animate | to show |
|---|---|
| a wire's `stroke-dashoffset` | flow along a wire — which way an object travels |
| the two paths of a commuting square, in sequence | **that they arrive at the same place** — the commuting IS the animation |
| a yank on a bent wire | the snake equation: the bend pulls straight to the identity |
| a split/merge at a Frobenius node | that the object really does branch |
| a pushout assembling from two cospans | composition over the shared boundary |

⇒ **Default is STATIC.** Animate only when motion shows something a still frame cannot, and
always leave the diagram readable with animation disabled — `prefers-reduced-motion` must not
destroy the content.

---

## 4. Non-negotiables

1. **Copyright within the first 10 lines**: `<!-- Copyright (c) 2025-2026 - Cowboy AI, Inc. -->`
   — the **one comment that survives**; everything else load-bearing goes in the tree.
2. ⛔ **RETIRED 2026-09-06 — this read "a header comment naming the calculus and the primary
   it follows."** ✅ **REPLACED BY: the calculus and its primary are ELEMENTS in `<metadata>`.**
   **A comment carries no load.** A validator must recover every claim **without reading a
   comment** — if a fact exists only as English inside the file, it is not in the diagram.
   *Why it was mis-formed:* a rule demanding a header **comment** only exists while the theme
   itself is prose (§0). A functor realised as tokens has no prose non-negotiables to retire —
   it has token definitions.
3. **Well-formed XML.** Parse it before shipping; a corrupted attribute renders as nothing.
4. **`viewBox` always**, so it scales.
5. **No external fetches** — no web fonts, no remote images. A diagram must render offline.
6. **A string diagram is never mermaid** (`typecheck-diagram-kind.sh` gates it).
7. **State what the diagram CANNOT show** in a caveat panel. A diagram that claims completeness
   it does not have is the measurement artifact in visual form.

---

## 5. DARK THEME — soft neon, high contrast, easy on the eyes

**steele 2026-08-22:** *"we also want a DARK theme with near neon glow effects on borders.
text should be READABLE (high contrast) but not glaring on our eyes, a softer than New Tokyo
Dark or Dracula theme."*

**THE TWO CONSTRAINTS PULL AGAINST EACH OTHER, AND THAT IS THE DESIGN PROBLEM.** Neon wants
saturation; comfort wants restraint. The resolution is: **carry the neon in the GLOW, not in
the text.** Borders and wires glow; type stays a soft off-white that never reaches `#ffffff`.

⛔ **NEVER pure white on pure black.** `#ffffff` on `#000000` is ~21:1 and is precisely the
glare being complained about. Our background is a soft blue-black and our text is a warm
off-white — **12.11:1 measured**, well past WCAG AAA (7:1) while visibly softer than Dracula
(`#f8f8f2` on `#282a36`) or Tokyo Night.

### Surfaces

| token | hex | note |
|---|---|---|
| `--bg` | `#151820` | soft blue-black. Never `#000` |
| `--panel` | `#1c2029` | law / caveat panels |
| `--panel-edge` | `#3a4152` | panel borders — muted, they are not the subject |
| `--text` | `#cdd6e4` | primary. Soft off-white, ~12:1 on `--bg` |
| `--text-dim` | `#8b95a7` | legends, captions — **5.88:1 measured** |
| `--text-faint` | `#7c8598` | caveats. **4.79:1 — measured, not eyeballed.** The first draft used `#6b7488`, which is 3.78:1 and FAILS WCAG AA; a theme that ships failing contrast is the measurement artifact in a palette |

### Semantic colours — same MEANING as light, retuned for glow

| element | light | **dark** | glow |
|---|---|---|---|
| type box (olog) | `#ffffff` / `#1a1a1a` | `#1c2029` / **`#9db8dc`** | soft |
| aspect (arrow) | `#1a1a1a` | **`#b9c6da`** | none |
| fact region | `#f4f7fb` / `#9db4d0` | `#18212e` / **`#5f86b5`** | soft |
| **layout** (limit) | `#fbf4f4` / `#d09d9d` | `#241b1e` / **`#d98d8d`** | soft |
| **grouping** (colimit) | `#f7f2fb` / `#b393d3` | `#201a2c` / **`#b491e8`** | soft |
| morphism box | `#eef4fa` / `#1a1a1a` | `#17242f` / **`#5fb3e0`** | medium |
| wire | `#1a6ea8` | **`#5fc9f0`** | medium |
| Frobenius node | `#c1272d` | **`#ff6b74`** | **strong** — it is the branch point |
| cup / cap (bend) | `#6a3d9a` | **`#c79bf0`** | **strong** — it is the bend |
| apex (cospan) | `#eef7f1` / `#1a7a45` | `#152420` / **`#4fd99b`** | medium |
| leg | `#1a1a1a` | **`#b9c6da`** | none |
| decoration | `#f7fbf8` / `#1a7a45` dashed | `#14201c` / **`#4fd99b`** dashed | soft |
| law panel | `#fbfbf4` / `#c8c8a8` | `#1e1d17` / **`#8f8a68`** | none |
| antimatter | `#6a3d9a` | **`#c79bf0`** | strong |

⇒ **GLOW IS SEMANTIC TOO.** Strong glow marks where the STRUCTURE lives — the Frobenius node
and the bend, the two things an interchange-only reading misses. Panels never glow; they are
context, not content.

### The glow filter — put this in `<defs>`

```xml
<filter id="glow-soft" x="-40%" y="-40%" width="180%" height="180%">
  <feGaussianBlur stdDeviation="1.6" result="b"/>
  <feMerge><feMergeNode in="b"/><feMergeNode in="SourceGraphic"/></feMerge>
</filter>
<filter id="glow-med" x="-60%" y="-60%" width="220%" height="220%">
  <feGaussianBlur stdDeviation="2.6" result="b"/>
  <feMerge><feMergeNode in="b"/><feMergeNode in="b"/><feMergeNode in="SourceGraphic"/></feMerge>
</filter>
<filter id="glow-strong" x="-80%" y="-80%" width="260%" height="260%">
  <feGaussianBlur stdDeviation="3.4" result="b"/>
  <feMerge><feMergeNode in="b"/><feMergeNode in="b"/><feMergeNode in="b"/><feMergeNode in="SourceGraphic"/></feMerge>
</filter>
```

⛔⛔ **A PERCENTAGE FILTER REGION ERASES AXIS-ALIGNED `<line>` ELEMENTS. SILENTLY.**
Found 2026-08-22 by rendering to PNG and LOOKING — the XML parsed clean and **every straight
wire in the string diagram had vanished**: both inputs, `a workspace state`, `a register
reading`, `a verdict`, `an epoch`.

**The mechanism:** an axis-aligned `<line>` has a bounding box of ZERO height (or zero width).
The filters above give their region in bbox PERCENTAGES (`x="-60%" width="220%"`), and a
percentage of zero is zero — so the filter region collapses and the element renders as
**nothing at all**. Not misdrawn. Absent.

**Confirmed with a control before fixing:** two identical lines, one filtered each way; only
the `userSpaceOnUse` one drew.

**The fix — a fourth filter, in user space, for straight wires:**

```xml
<filter id="glow-med-line" filterUnits="userSpaceOnUse" x="-2000" y="-2000"
        width="6000" height="6000">
  <feGaussianBlur stdDeviation="2.6" result="b"/>
  <feMerge><feMergeNode in="b"/><feMergeNode in="b"/><feMergeNode in="SourceGraphic"/></feMerge>
</filter>
```

⇒ **Use `glow-med-line` on every `<line>`.** Curved `<path>` wires have a two-dimensional bbox
and keep `glow-med`.

⇒ **AND THE LESSON IS THE ONE THIS CORPUS KEEPS RELEARNING:** XML validity is Test 1
(well-formed), never Test 2 (does it DO what is claimed). **A diagram that parses can still be
blank.** Render it and look at it, every time — the parser reports the identical "OK" whether
the wires drew or not, which makes it a measurement artifact if it is the only check.

⛔ **APPLY GLOW TO STROKES AND SHAPES ONLY — NEVER TO TEXT.** Blurred text is the single
biggest legibility killer in neon themes, and it is what makes a "cool" diagram unreadable at
100%. Text is crisp, unfiltered, and never glows.

⇒ **Strokes go up ~0.2–0.4px in dark**, because a glowing thin line reads thinner than it is.

### ⭐ THIS REPO RENDERS DARK — a CHOICE, not a deletion

**steele 2026-08-23:** *"we only need the dark versions, not both"* — then, correcting my
over-application: *"that didn't mean take the light version out of the theme, **we are simply
choosing the dark theme for this repo**."*

⇒ **THE THEME CARRIES BOTH. §1 (light) and §5 (dark) are EQUAL, COMPLETE token sets**, and both
remain fully usable. **This repo selects DARK**, so:

| | |
|---|---|
| what to render here | the **dark** token set (§5) |
| file naming | plain `X.svg` — **no `-dark` suffix**, since there is no twin to distinguish from |
| parity checking | **not needed** — one artifact per diagram |
| §1, the light set | **KEPT, EQUAL, AND USABLE.** It is the theme's other half, not a legacy section — a different repo or an embedding that needs a light ground renders from it |

⚠ **A CHOICE IS NOT A DEPRECATION.** Recording it as one is the same over-correction as
*"no member set is ever stored"* and *"the substrate has no metric"* — an over-stated rule that
destroys a valid thing to make a point. §1 defines what each token MEANS; §5 retunes those
meanings for a dark ground. **Neither supersedes the other.**

### Naming

- Dark variant of `X.svg` is **`X-dark.svg`**, same directory.
- **Same geometry, same labels, same caveats.** A dark diagram that says something different
  from its light twin is two diagrams, not a theme.
- Both must remain readable printed in greyscale — which is why structure is carried by
  DASH PATTERN and SHAPE as well as colour.

### Contrast is MEASURED, never eyeballed

Run this before shipping a palette change. Every ratio in this file came out of it:

```python
def lum(h):
    h=h.lstrip('#'); c=[int(h[i:i+2],16)/255 for i in (0,2,4)]
    c=[(v/12.92 if v<=0.03928 else ((v+0.055)/1.055)**2.4) for v in c]
    return 0.2126*c[0]+0.7152*c[1]+0.0722*c[2]
def ratio(a,b):
    la,lb=lum(a),lum(b); hi,lo=max(la,lb),min(la,lb); return (hi+0.05)/(lo+0.05)
```

| pair | ratio | verdict |
|---|---|---|
| `--text` on `--bg` | **12.11:1** | AAA |
| `--text-dim` on `--bg` | **5.88:1** | AA |
| `--text-faint` on `--bg` | **4.79:1** | AA |
| *Dracula reference* | *13.36:1* | ours is deliberately softer |
| *pure white on black* | *21.00:1* | the glare we are avoiding |
