#!/usr/bin/env bash
# Copyright (c) 2025-2026 - Cowboy AI, Inc.
#
# Archive Claude Code conversation history to pve-shared for long-term storage.
#
# This archive is AUTHORITATIVE for transcripts — the ~/.claude git repo
# deliberately ignores them (see .gitignore header). Everything the harness
# writes per-conversation is mirrored here: transcripts, subagent transcripts,
# tool-results, the sessions index, and the typed-prompt history.
#
# Additive by design: no --delete. A transcript that disappears locally (harness
# cleanup, disk pressure, a rotated project dir) stays in the archive. This is
# long-term storage, so it accumulates rather than tracking local state.
#
# Usage:  archive-history.sh [--dry-run]

set -euo pipefail

SRC="${HOME}/.claude"
DEST="/mnt/pve-shared/claude-history"
HOST="$(hostname -s)"
STAMP="$(date -Iseconds)"
DAY="$(date -I)"

DRY=()
[[ "${1:-}" == "--dry-run" ]] && DRY=(--dry-run)

# The archive is per-machine: two hosts both write ~/.claude/projects with
# overlapping session UUIDs, so a shared flat namespace would collide.
TARGET="${DEST}/${HOST}"

if ! mountpoint -q /mnt/pve-shared; then
  echo "FAIL: /mnt/pve-shared is not mounted — refusing to write into the mountpoint stub." >&2
  echo "      Writing there would fill the local disk and be invisible once the NFS mount returns." >&2
  exit 1
fi

mkdir -p "${TARGET}/projects"

echo "==> archiving conversation history"
echo "    from: ${SRC}"
echo "    to:   ${TARGET}"

# Transcripts, subagent transcripts and tool-results. Includes the memory/
# backups too — harmless duplication of what git tracks, and it means the
# archive is a complete standalone restore point.
# --info=stats1 only: progress2 emits a carriage-return progress line per file,
# which is ~170 KB of noise for this file count and useless in a log.
rsync -a --info=stats1 "${DRY[@]}" \
  --prune-empty-dirs \
  --include='*/' \
  --include='*.jsonl' \
  --include='*.jsonl.bak' \
  --include='**/subagents/**' \
  --include='**/tool-results/**' \
  --include='sessions-index.json' \
  --include='**/memory/**' \
  --exclude='*' \
  "${SRC}/projects/" "${TARGET}/projects/"

# Typed-prompt history across all projects. Append-only and rewritten in place,
# so it is snapshotted per-day rather than mirrored — keeps prior days intact.
if [[ -f "${SRC}/history.jsonl" ]]; then
  rsync -a "${DRY[@]}" "${SRC}/history.jsonl" "${TARGET}/history-${DAY}.jsonl"
fi

if [[ ${#DRY[@]} -eq 0 ]]; then
  {
    echo "${STAMP} host=${HOST}"
    echo "  transcripts: $(find "${TARGET}/projects" -name '*.jsonl' | wc -l) files, $(du -sh "${TARGET}/projects" | cut -f1)"
  } >> "${TARGET}/archive.log"
fi

echo "==> done"
du -sh "${TARGET}" 2>/dev/null || true
