<!-- Copyright (c) 2025 - Cowboy AI, Inc. -->

# Retired agents — DO NOT DISPATCH

These nine agent definitions are **retired**. They were removed from the live
set at `~/.claude/agents/` and are kept here for history only.

> **steele 2026-08-02:** *"archive all these we don't want to use them by
> mistake"*

## Why they are in a subdirectory rather than deleted

Deleting them would lose the history of how the agent roster evolved. Leaving
them in `agents/` risked the failure this archive exists to prevent: a
directory-level copy back into `~/.claude/agents/` would **resurrect all nine
as dispatchable agents**. The live directory is the authority for what can be
dispatched; `agents/` mirrors it; `agents/retired/` is outside that mirror.

**A sync that copies `agents/*.md` into `~/.claude/agents/` must not recurse
into this directory.**

## What is here

| file | note |
|---|---|
| `sage.md` | the only one with path references elsewhere in this repo — see below |
| `domain-expert.md` | superseded by the domain-specific experts |
| `cim-domain-expert.md` | superseded |
| `domain-ontologist-researcher.md` | superseded |
| `subject-expert.md` | superseded |
| `cim-ui-layer-expert.md` | UI-layer experts retired as a group |
| `egui-ui-expert.md` | " |
| `iced-ui-expert.md` | " |
| `act-expert.v2.md` | a variant of `act-expert.md`, which remains **live** |

## Dangling references, recorded not silently fixed

Three files referenced `agents/sage.md` by path before this move. They are
**left as-is** rather than rewritten, because two are historical records and
the third is a deployment example whose correct value is a decision, not a
cleanup:

- `AGENT_REFERENCE_THEORY.md:454` — an illustrative code comment
- `SUBJECT_MIGRATION_GUIDE.md:49` — an illustrative code comment
- `nix/DEPLOYMENT.md:193` — `agentFile = "${cim-domain-agent}/agents/sage.md"`
  in a deployment example. **This path is now wrong.** Whoever owns that
  deployment should point it at a live agent or at
  `agents/retired/sage.md` deliberately.

The other eight had **zero** path references anywhere in the repo.

## Verification at the time of archival

- live `~/.claude/agents/` — 34 agents
- `agents/` after the move — 34 agents + 7 repo docs
- the two agent sets are **identical, byte-for-byte** (verified with `cmp`)
