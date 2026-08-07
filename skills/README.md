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

## Known gap

`~/.claude/skills/cim-substrate/` (user scope, loads in every project) is **not
mirrored here** and is untracked everywhere. It is referenced by the shared agent
doctrine as the substrate-mechanism lookup, so losing it would be expensive.
