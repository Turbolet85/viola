#!/usr/bin/env bash
# Step 0's reading (plan.md step 0): the key probe over the fake agent, inside the one start of the chunk's own
# compositor. No live claude start. Every compositor call goes through live-compositor.sh, so every hyprctl call
# names the own instance. Prints relative names and codes only.
#   key-probe-own.sh <font size> [<key, one ASCII character; default x>]
# exit 9: the terminal is under the floor of 90 columns by 30 rows at this font size (the window is closed);
# exit 5: the window did not take focus by itself (S1); exit 3: the guard failed, no key (S3).
set -u
EVID=$(cd "$(dirname "$0")" && pwd)
ROOT=$(git -C "$EVID" rev-parse --show-toplevel) || exit 2
COMP=$EVID/live-compositor.sh
FONT=${1:?usage: key-probe-own.sh <font size> [<key>]}
KEY=${2:-x}
[ "${#KEY}" -eq 1 ] || { echo "usage: the key is one character"; exit 2; }
KEYHEX=$(printf '%s' "$KEY" | od -An -tx1 | tr -d ' \n')
VIOLA=$ROOT/target/harness/debug/viola
FAKE=$ROOT/target/harness/debug/viola-fake-agent
TS=$(date -u +%Y%m%dT%H%M%SZ)
REL=target/e2e-home/viola-keyprobe-$TS
BASE=$ROOT/$REL
HOME_DIR=$BASE/home
RECEIPT=$BASE/receipt.ndjson
EVENTS=$HOME_DIR/instances/keyprobe/events.ndjson
SNAP=$HOME_DIR/instances/keyprobe/snapshot.json
CLASS=viola.keyprobe

say() { printf '%s %s\n' "$(date -u +%H:%M:%S.%3NZ)" "$*" | sed -e "s#$ROOT#.#g" -e "s#$HOME#~#g"; }
wheel_lines() { [ -f "$EVENTS" ] && jq -c 'select(.kind == "wheel")' "$EVENTS" 2> /dev/null | wc -l || echo 0; }
wrapper_pid() { [ -f "$SNAP" ] && jq -r '.data.pid // empty' "$SNAP" 2> /dev/null; }
close_window() {
  local wp i
  wp=$(wrapper_pid)
  bash "$COMP" close "$CLASS"
  for i in $(seq 1 100); do
    { [ -n "$wp" ] && [ -d "/proc/$wp" ]; } || break
    sleep 0.1
  done
  say "wrapper gone: $({ [ -n "$wp" ] && [ -d "/proc/$wp" ]; } && echo no || echo yes)"
}

mkdir -m 700 "$BASE" || { say "ABORT mkdir of the probe base"; exit 2; }
say "probe home $REL/home (not yet existing) · font size $FONT · harness viola over the fake agent, no live start"
bash "$COMP" window "$CLASS" "$FONT" -- "$VIOLA" --home "$HOME_DIR" run keyprobe -- \
  "$FAKE" --cli-version 2.1.287 --fixtures "$ROOT/fixtures/claude" --screens --trusted-root "$ROOT" --receipt "$RECEIPT" \
  || { say "ABORT the window did not open"; exit 4; }

for _ in $(seq 1 150); do
  [ -f "$EVENTS" ] && [ "$(wc -l < "$EVENTS")" -ge 3 ] && break
  sleep 0.1
done
[ -f "$EVENTS" ] || { say "ABORT no events file after 15 s"; close_window; exit 4; }
say "start records: $(jq -r '[.kind, (.data.cause // .data.source // "")] | join(":")' "$EVENTS" | head -n 3 | tr '\n' ' ')"

for _ in $(seq 1 50); do
  [ -f "$RECEIPT" ] && command grep -q '"kind":"size"' "$RECEIPT" && break
  sleep 0.1
done
sleep 0.5
COLS=$(jq -c 'select(.kind == "size")' "$RECEIPT" 2> /dev/null | tail -n 1 | jq -r '.cols // empty')
ROWS=$(jq -c 'select(.kind == "size")' "$RECEIPT" 2> /dev/null | tail -n 1 | jq -r '.rows // empty')
say "size receipt at font size $FONT: cols ${COLS:-unread} · rows ${ROWS:-unread} (floor 90 by 30)"
if [ -z "$COLS" ] || [ -z "$ROWS" ] || [ "$COLS" -lt 90 ] || [ "$ROWS" -lt 30 ]; then
  say "under the floor at font size $FONT: the window is closed, the next font size is tried"
  close_window
  echo "RESULT floor=false font=$FONT cols=${COLS:-null} rows=${ROWS:-null}"
  exit 9
fi

CJ=$(bash "$COMP" hc clients -j)
ADDR=$(jq -r --arg c "$CLASS" '[.[] | select(.class == $c)] | if length == 1 then .[0].address else "" end' <<< "$CJ")
MAPPED=$(jq -r --arg c "$CLASS" '.[] | select(.class == $c) | .mapped' <<< "$CJ")
say "the window listed on the own instance: class $CLASS · mapped $MAPPED"
[ "$MAPPED" = true ] || { say "ABORT the probe window is not mapped; no key (S1)"; close_window; exit 5; }
W0=$(wheel_lines)
sleep 1.5
ACT=$(bash "$COMP" hc activewindow -j | jq -r '.address // ""')
FOCUS=false
[ -n "$ADDR" ] && [ "$ACT" = "$ADDR" ] && FOCUS=true
W1=$(wheel_lines)
say "it holds focus by itself (the own instance's active address equals its address): $FOCUS · wheel records before $W0 and after $W1 (delta $((W1 - W0)))"
[ "$FOCUS" = true ] || { say "ABORT the window did not take focus by itself; no key (S1)"; close_window; exit 5; }

bash "$COMP" key "$CLASS" "$KEY"
KRC=$?
if [ "$KRC" -ne 0 ]; then
  say "ABORT the key was not typed (exit $KRC)"
  close_window
  exit 3
fi
for _ in $(seq 1 50); do
  [ "$(wheel_lines)" -gt "$W1" ] && break
  sleep 0.1
done
W2=$(wheel_lines)
HOLDER=$(jq -c 'select(.kind == "wheel")' "$EVENTS" | tail -n 1 | jq -r '.data.holder')
CAUSE=$(jq -c 'select(.kind == "wheel")' "$EVENTS" | tail -n 1 | jq -r '.data.cause')
say "wheel records after the key: $W2 (new $((W2 - W1))) · the last: holder $HOLDER · cause $CAUSE"
KEYS=$(jq -r 'select(.kind == "key") | .hex' "$RECEIPT" 2> /dev/null | tr '\n' ' ')
say "the fake agent's receipt key lines (hex): ${KEYS:-none}"
case " $KEYS " in *" $KEYHEX "*) KEYSEEN=true ;; *) KEYSEEN=false ;; esac

printf 'probe' | "$VIOLA" --home "$HOME_DIR" send keyprobe > "$BASE/send.out" 2> "$BASE/send.err"
SRC=$?
ISSUED=$(jq -c 'select(.kind == "send-issued")' "$EVENTS" | wc -l)
say "driver send after the key: exit $SRC · stderr: $(tr '\n' '|' < "$BASE/send.err" | sed -e "s#$ROOT#.#g" -e "s#$HOME#~#g") · send-issued records $ISSUED"

close_window
echo "RESULT floor=true font=$FONT cols=$COLS rows=$ROWS mapped=$MAPPED focus_self=$FOCUS focus_wheel_delta=$((W1 - W0)) guard=true new_wheel=$((W2 - W1)) wheel_holder=$HOLDER wheel_cause=$CAUSE key=$KEY key_hex=$KEYHEX key_hex_seen=$KEYSEEN receipt_keys=$(printf '%s' "$KEYS" | tr ' ' ',' | sed 's/,$//') send_exit=$SRC send_issued=$ISSUED home=$REL"
