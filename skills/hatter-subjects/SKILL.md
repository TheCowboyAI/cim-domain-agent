---
name: hatter-subjects
description: The NATS subject surface hatter uses — cognitive.ingest.*, cognitive.query.*, the 26 optics.* subjects (ingest / walk / cover / query / modal / special), and how the hatter-mcp tool family mirrors them. Load when writing or debugging a subject publish/request, wiring a new optics.* subject, or reconciling the MCP tool surface against the subject family.
---

<!-- Copyright (c) 2025 - Cowboy AI, Inc. -->

# NATS subjects used by hatter

| Subject | Use |
|---|---|
| `cognitive.ingest.bulk` | Text content (bytes-as-string under UTF-8 univalence per `feedback_one_ingest_byte_to_word_view`). The canonical observe-back write path. |
| `cognitive.ingest.bytes` | Raw binary, no word semantics |
| `cognitive.ingest.document` | Structured-document entry with typedSlots / namedSlots / structuralPairs (Tier 1+ source-library routing) |
| `cognitive.ingest.qfs` | Substrate-native fast path (per `feedback_qfs_ingest_is_the_fast_path`) |
| `cognitive.query.{whatis,changed,relate,status,priorities,orphans}` | Statistical / structural queries — returns frequency / certainty / edges, NOT prose |
| `cognitive.query.observations` | (Tower branch) — returns full prose text of observations; v0 in-memory index |
| `optics.ingest.{qfs,bytes,symbols,tokens,words,phrases,concepts,schemas}` | Sprint-28+ typed-categorical ingest into FiberJoinGraph (8 subjects). REJECT-on-missing cover-witness per Q-A-1. `optics.ingest.qfs` is the primary write entry. |
| `optics.walk.<tier>` / `optics.cover.<tier>` | Per-tier walk + cover-witness read (6 + 6 subjects); Sprint-28 wired. |
| `optics.query.{tier_of,whatis,relate,status,changed,priorities,orphans}` | Substrate-derived typed queries (7 subjects); no parallel cache. Sprint-29..39 wired with substrate-derived bodies. |
| `optics.query.{whatis,relate,changed,priorities}-modal` | ◊_discourse modal-lifted queries; Phrases-and-above tier-eligibility per Q-D-1 (4 subjects). Sprint-29 T3 + Sprint-40 T3 per-sense projection. |
| `optics.longest-match` / `optics.workspace.view` | Special subjects (2). Total optics.* subjects: 26. |

The `hatter-mcp` binary (Sprint-30) exposes the `optics_*` MCP tool family over
JSON-RPC stdio mirroring the NATS subject family — see
`papers/HATTER-MCP-DESIGN.md` for design (note: optics_* tools shipped;
the design's code_* family is still pending). **No test pins the tool count** —
count it from the registry (`.register(Box::new(…))` sites in `src/mcp/`), never
from a doc; several doc sites disagree with each other.
