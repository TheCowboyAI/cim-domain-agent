---
name: feed-ingest
description: How `hatter ingest-feeds` works — the two independent projections over fetched bytes (Tier-1 recognizer fibration, always; four-cat cascade, opt-in via --cascade), Σ selection from the feed's declared language, provenance on the delta, and the non-fatal cascade policy. Load when working on src/substrate/ingest_feeds.rs, running or debugging ingest-feeds, or touching feed/RSS/Atom fetch paths.
---

<!-- Copyright (c) 2025 - Cowboy AI, Inc. -->

# Feed ingest — `hatter ingest-feeds` (two projections, one optional)

`src/substrate/ingest_feeds.rs`. HTTP fetch → bytes → recognizer fibration.
**TWO independent projections** over the same fetched bytes:

1. **Tier-1 recognizer fibration (always).**
   `format_dispatch::dispatch_recognizable_file` → `cognitive.ingest.bytes`
   (source CID) + recognized FIELD VALUES as observations via
   `cognitive.ingest.bulk`. **This path does NOT carve words** — it emits whole
   field values and Tower folds them.
2. **Four-cat cascade (opt-in, `--cascade`).** `inspect_feed_content` extracts
   the feed's PROSE (the whitelisted semantic values — staging raw XML would
   fold `rss`/`channel`/`title` as WORDS), stages it, and runs the QFS-inspector
   cascade under the Σ the feed DECLARED.

Discipline that is load-bearing here:

- **Σ is SELECTED, not assumed** — `select_alphabet` reads RSS `<language>` off
  the recognizer's own `RssSymbol::Channel` output, and Atom `xml:lang` by a
  BOUNDED scan of the root open tag only (an entry-level `xml:lang` must not
  masquerade as the feed's). `SelectedAlphabet` keeps `declared` alongside the
  alphabet so "declared `en`" and "defaulted to `en`" stay distinguishable.
- **Provenance rides the delta** — `declaredLanguage` / `alphabet` /
  `alphabetCid` / `alphabetDeclared` on the domain event + `SourceReport`.
- **The cascade is NON-FATAL** — at step 3b the Tier-1 fold has ALREADY landed,
  so a cascade fault warns and records `cascade: None` rather than failing the
  source (same policy as `master.create`). No prose ⇒ `Ok(None)`, not an error.
- **Prose is STAGED to a file, not folded from memory** — the cascade entry is
  `inspect_qfs(ContentPath)` and hatter PROJECTS OVER QFS-RESIDENT BYTES
  (`[[feedback_no_ingest_project_over_qfs]]`). An `inspect_bytes` bypass was
  considered and rejected for that reason.
- **`--cascade` opens a SECOND Alice connection** (`--cascade-alice-url`,
  default `ws://localhost:14140/`, `ALICE_URL` overrides). The inspector needs
  an `Arc<NtarClient>`; main's Tier-1 path holds the `Transport` enum, which
  does not expose one, and `NtarClient` is not `Clone`. Named, not hidden. A
  connect failure FAILS the command rather than silently degrading to Tier-1.

**NOT yet live-verified:** `--cascade` has never run against a real Alice with a
real feed. Everything is build + test only. Smoke it on dell before trusting it.
