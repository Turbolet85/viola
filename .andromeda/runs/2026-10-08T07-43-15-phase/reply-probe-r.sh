#!/usr/bin/env bash
# The revision's addition to the terminal-reply probe: one window per query on the chunk's own, standing
# compositor; the product build wraps reply-probe-r-child.py. No claude start, no key: nothing here calls `key`.
# Every compositor call goes through the chunk's evidence/live-compositor.sh (the own instance only).
#   reply-probe-r.sh <diag dir (a 0700 dir under target/e2e-home/)> <query id>...
set -u
RUN=$(cd "$(dirname "$0")" && pwd)
ROOT=$(git -C "$RUN" rev-parse --show-toplevel) || exit 2
COMP=$ROOT/viola-0.1.0/chunks/2026-10-08-first-live-test-and-self-drive/evidence/live-compositor.sh
VIOLA=$ROOT/target/release-check/release/viola
CHILD=$RUN/reply-probe-r-child.py
D=$ROOT/${1:?usage: reply-probe-r.sh <diag dir> <query id>...}
shift
CLASS=viola.replyprobe
SIDE=$D/replies-r.ndjson
[ -d "$D" ] || { echo "no diag dir"; exit 2; }
touch "$SIDE"

for q in "$@"; do
  name=r-$(printf '%s' "$q" | tr -c 'a-z0-9' '-')
  ev=$D/home/instances/$name/events.ndjson
  for _ in $(seq 1 50); do
    [ "$(bash "$COMP" hc clients -j | jq 'length')" = 0 ] && break
    sleep 0.1
  done
  before=$(wc -l < "$SIDE")
  bash "$COMP" window "$CLASS" 8 -- "$VIOLA" --home "$D/home" run "$name" -- "$CHILD" "$q" "$SIDE" > "$D/window-r-$q.out" 2>&1
  wrc=$?
  for _ in $(seq 1 60); do
    [ "$(wc -l < "$SIDE")" -gt "$before" ] && break
    sleep 0.1
  done
  sleep 0.4
  moved=$(jq -c 'select(.kind == "wheel" and .data.cause == "human-input")' "$ev" 2> /dev/null | wc -l)
  line=$(jq -c --arg q "$q" 'select(.id == $q)' "$SIDE" 2> /dev/null | tail -n 1)
  printf '%s window_exit=%s wheel_human_input=%s %s\n' "$q" "$wrc" "$moved" "${line:-no-side-line}"
  bash "$COMP" close "$CLASS" > /dev/null 2>&1
done
