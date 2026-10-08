#!/usr/bin/env bash
# The close of a live session (plan.md step 7, Close): the child's fd numbers and CLAUDE / VIOLA names (names
# only) while it is alive, then `close <class>`, with a poller reading when the wrapper and the child are gone
# (by pid and start time) and when a session-end line landed. Codes and numbers only.
#   live-close.sh <home> <instance> <class>
set -u
H=${1:?home}; N=${2:?instance}; CLASS=${3:?class}
EVID=viola-0.1.0/chunks/2026-10-08-first-live-test-and-self-drive/evidence
EV=$H/instances/$N/events.ndjson
SNAP=$H/instances/$N/snapshot.json
pstart() { sed 's/.*) //' "/proc/$1/stat" 2> /dev/null | cut -d' ' -f20; }
WP=$(jq -r '.data.pid' "$SNAP"); CP=$(jq -r '.data.child_pid' "$SNAP")
WS=$(pstart "$WP"); CS=$(pstart "$CP")
echo "BEFORE wrapper alive=$([ -n "$WS" ] && echo yes || echo no) child alive=$([ -n "$CS" ] && echo yes || echo no) child exe=$(basename "$(readlink "/proc/$CP/exe" 2> /dev/null)")"
echo "CHILD fds: $(ls "/proc/$CP/fd" 2> /dev/null | sort -n | tr '\n' ' ')"
echo "CHILD fd count: $(ls "/proc/$CP/fd" 2> /dev/null | wc -l)"
echo "CHILD CLAUDE names: $(tr '\0' '\n' < "/proc/$CP/environ" 2> /dev/null | cut -d= -f1 | command grep -E '^CLAUDE' | sort | tr '\n' ' ')"
echo "CHILD VIOLA names: $(tr '\0' '\n' < "/proc/$CP/environ" 2> /dev/null | cut -d= -f1 | command grep -E '^VIOLA' | sort | tr '\n' ' ')"
SE0=$(jq -c 'select(.kind == "session-end")' "$EV" | wc -l)
T0=$(date +%s.%N)
(
  wg=""; cg=""; se=""
  for _ in $(seq 1 1000); do
    now=$(date +%s.%N)
    [ -z "$wg" ] && [ "$(pstart "$WP")" != "$WS" ] && wg=$(awk -v a="$T0" -v b="$now" 'BEGIN { printf "%.2f", b - a }')
    [ -z "$cg" ] && [ "$(pstart "$CP")" != "$CS" ] && cg=$(awk -v a="$T0" -v b="$now" 'BEGIN { printf "%.2f", b - a }')
    [ -z "$se" ] && [ "$(command grep -c '"kind":"session-end"' "$EV")" -gt "$SE0" ] && se=$(awk -v a="$T0" -v b="$now" 'BEGIN { printf "%.2f", b - a }')
    [ -n "$wg" ] && [ -n "$cg" ] && break
    sleep 0.02
  done
  sleep 0.3
  [ -z "$se" ] && [ "$(command grep -c '"kind":"session-end"' "$EV")" -gt "$SE0" ] && se=late
  echo "POLL wrapper_gone_s=${wg:-never} child_gone_s=${cg:-never} session_end_line_s=${se:-none} (seconds after the close call began)"
) &
bash "$EVID/live-compositor.sh" close "$CLASS"
wait
echo "AFTER wrapper gone=$([ "$(pstart "$WP")" != "$WS" ] && echo yes || echo no) child gone=$([ "$(pstart "$CP")" != "$CS" ] && echo yes || echo no) · last record kind: $(tail -n 1 "$EV" | jq -r .kind) source: $(tail -n 1 "$EV" | jq -r .source)"
echo "AFTER session-end ts: $(jq -r 'select(.kind == "session-end") | .ts' "$EV" | tail -n 1)"
