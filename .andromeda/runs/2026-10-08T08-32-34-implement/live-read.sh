#!/usr/bin/env bash
# Readings of a live session's records, as codes and counts only (no prompt, assistant or payload text).
#   live-read.sh <home> <instance> start      wait for session-start (30 s), then 5 s, then the snapshot reading
#   live-read.sh <home> <instance> kinds      the event kinds in order, with cause / holder where the kind has one
#   live-read.sh <home> <instance> wheel      the wheel records (ts, holder, cause) and their count
set -u
H=${1:?home}; N=${2:?instance}; OP=${3:?op}
EV=$H/instances/$N/events.ndjson
SNAP=$H/instances/$N/snapshot.json
kinds() {
  jq -r '[.kind, (.data.cause // .data.holder // "")] | map(select(. != "")) | join(":")' "$EV" 2> /dev/null | tr '\n' ' '
}
case "$OP" in
  start)
    for _ in $(seq 1 300); do
      [ -f "$EV" ] && jq -e -s 'any(.[]; .kind == "session-start")' "$EV" > /dev/null 2>&1 && break
      sleep 0.1
    done
    if ! { [ -f "$EV" ] && jq -e -s 'any(.[]; .kind == "session-start")' "$EV" > /dev/null 2>&1; }; then
      echo "START no session-start within 30 s (S4 if a modal stands) · kinds so far: $(kinds)"
      exit 4
    fi
    echo "START first records: $(jq -r '[.kind, (.data.cause // "")] | map(select(. != "")) | join(":")' "$EV" | head -n 3 | tr '\n' ' ')"
    echo "START record times: $(jq -r '[.kind, .ts] | join("@")' "$EV" | head -n 4 | tr '\n' ' ')"
    sleep 5
    echo "SNAPSHOT cli_version=$(jq -r '.data.cli_version' "$SNAP") cli_verified=$(jq -r '.data.cli_verified' "$SNAP") wheel=$(jq -r '.data.wheel' "$SNAP") dialog_pending=$(jq -r '.data.dialog_pending' "$SNAP")"
    echo "WHEEL human records: $(jq -c 'select(.kind == "wheel" and .data.holder == "human")' "$EV" | wc -l) · all kinds: $(kinds)"
    CP=$(jq -r '.data.child_pid // empty' "$SNAP")
    if [ -n "$CP" ] && [ -d "/proc/$CP" ]; then
      PTS=$(readlink "/proc/$CP/fd/0")
      echo "CHILD alive=yes exe=$(basename "$(readlink "/proc/$CP/exe")") tty_kind=$(case "$PTS" in /dev/pts/*) echo pts ;; *) echo other ;; esac) size(rows cols)=$(stty size < "$PTS" 2> /dev/null)"
      echo "CHILD CLAUDE names: $(tr '\0' '\n' < "/proc/$CP/environ" | cut -d= -f1 | command grep -E '^CLAUDE' | sort | tr '\n' ' ')"
      echo "CHILD VIOLA names: $(tr '\0' '\n' < "/proc/$CP/environ" | cut -d= -f1 | command grep -E '^VIOLA' | sort | tr '\n' ' ')"
      echo "CHILD fds: $(ls "/proc/$CP/fd" | sort -n | tr '\n' ' ')"
    else
      echo "CHILD alive=no"
    fi
    ;;
  kinds) echo "KINDS $(kinds)" ;;
  wheel)
    jq -r 'select(.kind == "wheel") | [.ts, .data.holder, .data.cause] | join(" ")' "$EV"
    echo "WHEEL count $(jq -c 'select(.kind == "wheel")' "$EV" | wc -l)"
    ;;
  *) echo "usage"; exit 2 ;;
esac
