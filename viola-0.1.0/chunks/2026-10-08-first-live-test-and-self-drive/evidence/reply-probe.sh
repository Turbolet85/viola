#!/usr/bin/env bash
# The terminal-reply probe (inputs#I15): which answer of the terminal does the wheel's stdin classifier take for
# typing? One window per query on the chunk's own compositor: the product build wraps reply-probe-child.py, which
# sends one query and records the terminal's answer; the instance's log then shows whether a `wheel` record of
# cause `human-input` followed. No claude start, no key: nothing here calls `key`.
#   reply-probe.sh <diag dir (a 0700 dir under target/e2e-home/)> <query id>...
set -u
EVID=$(cd "$(dirname "$0")" && pwd)
ROOT=$(git -C "$EVID" rev-parse --show-toplevel) || exit 2
COMP=$EVID/live-compositor.sh
VIOLA=$ROOT/target/release-check/release/viola
CHILD=$EVID/reply-probe-child.py
D=$ROOT/${1:?usage: reply-probe.sh <diag dir> <query id>...}
shift
CLASS=viola.replyprobe
SIDE=$D/replies.ndjson
[ -d "$D" ] || { echo "no diag dir"; exit 2; }

for q in "$@"; do
  name=q-$(printf '%s' "$q" | tr -c 'a-z0-9' '-')
  ev=$D/home/instances/$name/events.ndjson
  for _ in $(seq 1 50); do
    [ "$(bash "$COMP" hc clients -j | jq 'length')" = 0 ] && break
    sleep 0.1
  done
  before=$(wc -l < "$SIDE" 2> /dev/null || echo 0)
  bash "$COMP" window "$CLASS" 8 -- "$VIOLA" --home "$D/home" run "$name" -- "$CHILD" "$q" "$SIDE" > "$D/window-$q.out" 2>&1
  wrc=$?
  for _ in $(seq 1 60); do
    [ "$(wc -l < "$SIDE" 2> /dev/null || echo 0)" -gt "$before" ] && break
    sleep 0.1
  done
  sleep 0.4
  moved=$(jq -c 'select(.kind == "wheel" and .data.cause == "human-input")' "$ev" 2> /dev/null | wc -l)
  kinds=$(jq -r '.kind' "$ev" 2> /dev/null | tr '\n' ' ')
  line=$(jq -c --arg q "$q" 'select(.id == $q)' "$SIDE" 2> /dev/null | tail -n 1)
  printf '%s window_exit=%s wheel_human_input=%s kinds=[%s] %s\n' "$q" "$wrc" "$moved" "${kinds% }" "${line:-no-side-line}"
  bash "$COMP" close "$CLASS" > /dev/null 2>&1
done
