#!/usr/bin/env bash
# ARC Network Monitor — background NTAR-WebSocket subscriber via hatter.
#
# Runs as a one-shot wrapper: kills any prior listener, starts a fresh one
# via `hatter arc-listen` (which speaks NTAR-WebSocket on port 14140 — the
# transport this node actually has, unlike NATS on 14222 which doesn't exist
# on dell-62S6063).
#
# The subscriber writes received ARC messages to the buffer file as JSONL,
# one line per message: {"ts":"<rfc3339>","msg":<original-payload>}. The
# UserPromptSubmit hook reads `wc -l < buffer` for unread counts.

ARC_BUFFER="/tmp/arc-messages.jsonl"
ARC_PID="/tmp/arc-monitor.pid"
HATTER="/git/thecowboyai/hatter/target/release/hatter"

# Kill existing listener if running.
#
# 2026-07-31: the pid-file kill ALONE is not sufficient and corrupted the
# buffer. If a listener outlives its pid file (/tmp sweep, unclean exit,
# SIGKILL) the next run cannot see it, so a SECOND listener starts and both
# write $ARC_BUFFER concurrently. Two writers on one fd interleave at write()
# granularity, producing token-doubled JSONL ({{""msgmsg""::) that no parser
# accepts — 4 of 7 records were damaged before this was caught, and `wc -l`
# reported the fragments as 11 phantom "unread" messages.
#
# Kill by PATTERN, not just by recorded pid: the undesirable state is "more
# than one listener", so reap every match rather than the one we happened to
# write down. pkill excludes self ($$) by default.
if [ -f "$ARC_PID" ]; then
  kill "$(cat "$ARC_PID")" 2>/dev/null
  rm -f "$ARC_PID"
fi
pkill -f "$HATTER arc-listen" 2>/dev/null
# Give SIGTERM a moment to land so the new listener never overlaps the old.
for _ in 1 2 3 4 5; do
  pgrep -f "$HATTER arc-listen" >/dev/null 2>&1 || break
  sleep 0.2
done
pkill -9 -f "$HATTER arc-listen" 2>/dev/null

# Shape-filter the buffer in place: drop entries whose payload isn't a real
# ARC post (i.e. doesn't carry {body, from, subject, to}). The substrate
# also publishes walk-by-CID requests on conversation.interagent.> which
# the wildcard subscription captures; those entries have {cid,limit} and
# are not human/agent dialogue. Idempotent; safe to run every hook tick.
if [ -f "$ARC_BUFFER" ] && command -v jq >/dev/null 2>&1; then
  jq -c 'select(.msg | type == "object" and has("body") and has("from") and has("subject") and has("to"))' \
    "$ARC_BUFFER" > "$ARC_BUFFER.tmp" 2>/dev/null \
    && mv "$ARC_BUFFER.tmp" "$ARC_BUFFER" \
    || rm -f "$ARC_BUFFER.tmp"
fi

# Start background listener via hatter. The Rust subcommand writes the PID
# file itself on startup and removes it on clean SIGTERM/SIGINT exit.
# exclude-from is exact-match in hatter; payloads use lowercase from-field
# (`keel`, `steele`), so pass both cases to be belt-and-suspenders against
# any future capitalized senders.
# 2026-07-31: default was `1-1`. Per steele's Q12 ruling, "1-1 should never be
# used, that was an old non-secure system" — same correction already applied to
# src/substrate/alice_query.rs::DEFAULT_API_KEY. Kept as an env-overridable
# default so an explicit ALICE_API_KEY still wins.
ALICE_API_KEY="${ALICE_API_KEY:?set ALICE_API_KEY, e.g. source ~/.claude/.env.local}" nohup "$HATTER" arc-listen \
  --buffer "$ARC_BUFFER" \
  --pid-file "$ARC_PID" \
  --exclude-from "keel" \
  --exclude-from "Keel" \
  --exclude-from "steele" \
  --exclude-from "Steele" \
  --daemon \
  >/dev/null 2>&1 &

# Report buffer status (one-shot, for the UserPromptSubmit hook to consume).
# `wc -l` errors silently if the buffer doesn't exist yet; coerce to 0.
if [ -f "$ARC_BUFFER" ]; then
  count=$(wc -l < "$ARC_BUFFER" 2>/dev/null)
else
  count=0
fi
if [ "${count:-0}" -gt 0 ]; then
  echo "{\"systemMessage\":\"[ARC] $count unread message(s) on conversation.interagent.> — check arc\"}"
fi
echo "{\"systemMessage\":\"ARC monitor (re)started via hatter arc-listen — PID file at $ARC_PID\"}"
