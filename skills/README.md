<!-- Copyright (c) 2025 - Cowboy AI, Inc. -->

# CIM Skill Definitions — tracked mirror

Sibling to `agents/`, and the same replication contract: the **live** copies are
the ones Claude Code loads; the copies here are the **tracked** ones that survive
a fresh clone. Both are plain files, kept in sync by copy — there is no symlink
and no automatic sync, so they can drift (the `agents/` mirror already has).

## Why this directory exists

Project skills live at `<repo>/.claude/skills/<name>/SKILL.md`, and several CIM
repos gitignore `.claude/` wholesale (`hatter/.gitignore:9`). Content migrated
out of a checked-in `CLAUDE.md` into a project skill would otherwise be lost on
clone. Mirroring here keeps it under version control without un-ignoring
`.claude/`.

## Layout

Each skill is a directory containing `SKILL.md`, whose YAML front-matter carries
`name` and `description`. Only the `description` stays resident in context; the
body loads when the skill is invoked — that is the point of migrating a section
out of an always-loaded `CLAUDE.md`.

| Skill | Live location | Covers |
|---|---|---|
| `feed-ingest` | `hatter/.claude/skills/feed-ingest/` | `hatter ingest-feeds` — the two projections, Σ selection, non-fatal cascade |
| `hatter-subjects` | `hatter/.claude/skills/hatter-subjects/` | the `cognitive.*` / `optics.*` NATS subject surface |

## Replicating

Copy in the direction of whichever side you edited, then commit here:

```sh
cp -r /git/thecowboyai/hatter/.claude/skills/<name> /git/thecowboyai/cim-domain-agent/skills/
```


## What else from `~/.claude` is mirrored here

Per steele 2026-08-07 — *"all the things you see in .claude get replicated to
../cim-domain-agents for persistence"* — this repo is the durable form of the
user-scope Claude configuration, none of which lives in a repo of its own
(`~/.claude` is not a git repository).

| Mirror here | Source | Notes |
|---|---|---|
| `agents/` | `~/.claude/agents/` | pre-existing; already drifted (act-expert differs at line 94) |
| `skills/` | `~/.claude/skills/` + project `.claude/skills/` | this directory, incl. `cim-substrate` (user-scope, loads in every project) |
| `shared/cim-agent-doctrine.md` | `~/.claude/shared/` | `@`-imported by the global CLAUDE.md; loads in every session |
| `commands/` | `~/.claude/commands/` | 19 slash commands |
| `CLAUDE.global.md` | `~/.claude/CLAUDE.md` | **deliberately NOT named `CLAUDE.md`** — that filename auto-loads as a memory file when working in this repo, which would duplicate the global that is already loaded. `.global` keeps it inert here while still being the backup. |

Same copy-based contract throughout: no symlinks, no sync step, so any of
these can drift from its source. Re-copy in whichever direction you edited.
