#!/usr/bin/env bash
# Key probe in a compositor of the chunk's own: a headless Hyprland started here with a minimal config, in a
# private runtime dir, with foot and wtype inside it. Does a wtype key typed into a foot window running
# `viola run` over the fake agent move the wheel? No live claude start. Every hyprctl call names the own
# instance through hc(); the desktop compositor is never asked and never dispatched to. Prints relative names
# and codes only.
set -u
ROOT=$(git rev-parse --show-toplevel)
RUN=$(cd "$(dirname "$0")" && pwd)
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
DESK_RT=$XDG_RUNTIME_DIR
DESK_SIG=${HYPRLAND_INSTANCE_SIGNATURE:-}
CRT=$DESK_RT/vcomp
CFG=$RUN/comp-min.lua
CPID=""; FPID=""; SIG=""; WL=""

say() { printf '%s %s\n' "$(date -u +%H:%M:%S.%3NZ)" "$*"; }
clean() { sed -e "s#$ROOT#.#g" -e "s#$HOME#~#g"; }
hc() { XDG_RUNTIME_DIR="$CRT" hyprctl --instance "$SIG" "$@"; }
active() { hc activewindow -j | python3 -c 'import json,sys
try:
    w=json.load(sys.stdin); print(w.get("address",""), w.get("class",""))
except Exception:
    print("", "")'; }
field() { hc clients -j | python3 -c 'import json,sys
c=[w for w in json.load(sys.stdin) if w.get("class")==sys.argv[1]]
print(c[0].get(sys.argv[2],"") if len(c)==1 else "")' "$CLASS" "$1"; }
wheel_lines() { [ -f "$EVENTS" ] && command grep -c '"kind":"wheel"' "$EVENTS" || echo 0; }
# The desktop's witnesses, read with no hyprctl: the shell's lock answer, the user manager's environment,
# the desktop's runtime entries and its compositor process count.
desk() { printf 'shell-locked=%s sd-env=%s hypr-dirs=%s wayland-sockets=%s compositors=%s' \
  "$(omarchy-shell lock isLocked 2>/dev/null | tr -d '\n')" \
  "$(systemctl --user show-environment | command grep -E '^(WAYLAND_DISPLAY|HYPRLAND_INSTANCE_SIGNATURE|XDG_CURRENT_DESKTOP)=' | sha256sum | cut -c1-12)" \
  "$(ls "$DESK_RT/hypr" | wc -l)" \
  "$(ls "$DESK_RT" | command grep -c -E '^wayland-[0-9]+$')" \
  "$(ps -eo comm= | command grep -c -x Hyprland)"; }

teardown() {
  if [ -n "$FPID" ] && [ "$(readlink "/proc/$FPID/exe" 2>/dev/null)" = /usr/bin/foot ]; then
    kill -TERM "$FPID"; say "TERM sent to the probe's own foot process"
  fi
  local wp=""
  [ -f "$SNAP" ] && wp=$(python3 -c 'import json,sys
print(json.load(open(sys.argv[1]))["data"]["pid"])' "$SNAP" 2>/dev/null)
  for _ in $(seq 1 100); do
    { [ -n "$FPID" ] && [ -d "/proc/$FPID" ]; } || { [ -n "$wp" ] && [ -d "/proc/$wp" ]; } || break
    sleep 0.1
  done
  say "foot gone: $([ -n "$FPID" ] && [ -d "/proc/$FPID" ] && echo no || echo yes) · wrapper gone: $([ -n "$wp" ] && [ -d "/proc/$wp" ] && echo no || echo yes)"
  if [ -n "$CPID" ] && [ "$(readlink "/proc/$CPID/exe" 2>/dev/null)" = /usr/bin/Hyprland ] \
     && tr '\0' '\n' < "/proc/$CPID/environ" | command grep -q -x "XDG_RUNTIME_DIR=$CRT"; then
    kill -TERM "$CPID"; say "TERM sent to the own compositor (its runtime dir read from its environment first)"
    for _ in $(seq 1 50); do [ -d "/proc/$CPID" ] || break; sleep 0.1; done
    if [ -d "/proc/$CPID" ]; then kill -KILL "$CPID"; sleep 0.3; say "own compositor still alive after 5 s: KILL sent"; fi
  fi
  say "own compositor gone: $([ -n "$CPID" ] && [ -d "/proc/$CPID" ] && echo no || echo yes)"
  say "desktop after:  $(desk)"
}
trap teardown EXIT

say "desktop before: $(desk)"
[ -S /run/seatd.sock ] && { say "ABORT a seatd socket exists: the forced seatd backend could open a seat"; exit 11; }
if [ -d "$CRT" ]; then
  [ "$(stat -c '%a %u' "$CRT")" = "700 $(id -u)" ] || { say "ABORT the private runtime dir is not 0700 and owned"; exit 12; }
  for l in "$CRT"/hypr/*/hyprland.lock; do
    [ -f "$l" ] || continue
    p=$(head -n 1 "$l")
    [ -n "$p" ] && [ -d "/proc/$p" ] && { say "ABORT an instance is alive in the private runtime dir"; exit 13; }
  done
else
  mkdir -m 700 "$CRT" || { say "ABORT mkdir private runtime dir"; exit 12; }
fi
BEFORE_SIGS=$(ls "$CRT/hypr" 2>/dev/null | sort | tr '\n' ' ')
mkdir -m 700 "$BASE" || { say "ABORT mkdir base"; exit 3; }
say "probe home $REL/home (not yet existing) · private runtime dir \$XDG_RUNTIME_DIR/vcomp"

MODE=${1:-nested}
ulimit -c 0
if [ "$MODE" = headless ]; then
  env -i HOME="$HOME" PATH="$PATH" USER="$USER" LANG="${LANG:-C.UTF-8}" XDG_RUNTIME_DIR="$CRT" \
    LIBSEAT_BACKEND=seatd HYPRLAND_NO_SD_VARS=1 HYPRLAND_NO_SD_NOTIFY=1 HYPRLAND_NO_RT=1 HYPRLAND_NO_CRASHREPORTER=1 \
    Hyprland --config "$CFG" > "$BASE/compositor.out" 2>&1 &
  CPID=$!
  say "own compositor started, mode headless (no WAYLAND_DISPLAY, no session bus, seat backend forced to an absent seatd)"
else
  # Nested: the own compositor is one client window of the desktop compositor (its socket named by absolute
  # path); it takes no seat, no session bus, and its own sockets live in the private runtime dir.
  env -i HOME="$HOME" PATH="$PATH" USER="$USER" LANG="${LANG:-C.UTF-8}" XDG_RUNTIME_DIR="$CRT" \
    WAYLAND_DISPLAY="$DESK_RT/${WAYLAND_DISPLAY:-wayland-1}" \
    LIBSEAT_BACKEND=seatd HYPRLAND_NO_SD_VARS=1 HYPRLAND_NO_SD_NOTIFY=1 HYPRLAND_NO_RT=1 HYPRLAND_NO_CRASHREPORTER=1 \
    Hyprland --config "$CFG" > "$BASE/compositor.out" 2>&1 &
  CPID=$!
  say "own compositor started, mode nested (one client window of the desktop compositor; no session bus, seat backend forced to an absent seatd)"
fi
for _ in $(seq 1 100); do
  [ -d "/proc/$CPID" ] || break
  for d in "$CRT"/hypr/*/; do
    [ -S "$d.socket.sock" ] || continue
    s=$(basename "$d")
    case " $BEFORE_SIGS " in *" $s "*) continue;; esac
    SIG=$s
  done
  [ -n "$SIG" ] && break
  sleep 0.1
done
if [ -z "$SIG" ]; then
  say "ABORT the own compositor gave no control socket (alive: $([ -d "/proc/$CPID" ] && echo yes || echo no))"
  command grep -a -i -E 'critical|error|failed|could not|cannot|session|backend|headless' "$BASE/compositor.out" | clean | sed -E 's/\x1b\[[0-9;]*m//g' | cut -c1-200 | tail -n 25
  exit 20
fi
[ "$SIG" != "$DESK_SIG" ] || { say "ABORT the instance found is the desktop's"; exit 21; }
say "own instance up: signature differs from the desktop's: yes · control socket in the private runtime dir"
for _ in $(seq 1 50); do
  WL=$(ls "$CRT" | command grep -E '^wayland-[0-9]+$' | head -n 1)
  [ -n "$WL" ] && break
  sleep 0.1
done
[ -n "$WL" ] || { say "ABORT no wayland socket in the private runtime dir"; exit 22; }
say "wayland socket: \$XDG_RUNTIME_DIR/vcomp/$WL"
command grep -a -i -E 'failed to open a session|backend|start-hyprland|headless' "$BASE/compositor.out" | clean | sed -E 's/\x1b\[[0-9;]*m//g' | cut -c1-180 | head -n 12
say "version: $(hc version | head -n 1 | cut -c1-60)"
MON=$(hc monitors -j | python3 -c 'import json,sys
print(" ".join(m["name"]+":"+str(m["width"])+"x"+str(m["height"])+":dpms="+str(m.get("dpmsStatus")) for m in json.load(sys.stdin)))')
say "monitors: ${MON:-none}"
if [ -z "$MON" ]; then
  say "output create headless: $(hc output create headless 2>&1 | tr '\n' ' ')"
  sleep 0.5
  MON=$(hc monitors -j | python3 -c 'import json,sys
print(" ".join(m["name"]+":"+str(m["width"])+"x"+str(m["height"])+":dpms="+str(m.get("dpmsStatus")) for m in json.load(sys.stdin)))')
  say "monitors: ${MON:-none}"
fi
say "own instance locked: $(hc locked | tr -d '\n') · windows: $(hc clients -j | python3 -c 'import json,sys
print(len(json.load(sys.stdin)))')"

env -i HOME="$HOME" PATH="$PATH" USER="$USER" LOGNAME="$USER" SHELL="${SHELL:-/bin/bash}" LANG="${LANG:-C.UTF-8}" \
  XDG_RUNTIME_DIR="$DESK_RT" WAYLAND_DISPLAY="$CRT/$WL" \
  foot --app-id="$CLASS" --title=viola-keyprobe --working-directory="$ROOT" -e "$VIOLA" --home "$HOME_DIR" run keyprobe -- \
  "$FAKE" --cli-version 2.1.287 --fixtures "$ROOT/fixtures/claude" --screens --trusted-root "$ROOT" --receipt "$RECEIPT" \
  > "$BASE/foot.out" 2>&1 &
FPID=$!
say "foot started on the own compositor (a declared environment: no CLAUDE and no VIOLA name)"

for _ in $(seq 1 150); do
  [ -f "$EVENTS" ] && [ "$(wc -l < "$EVENTS")" -ge 3 ] && break
  sleep 0.1
done
if ! [ -f "$EVENTS" ]; then
  say "ABORT no events file after 15 s (foot alive: $([ -d "/proc/$FPID" ] && echo yes || echo no))"
  clean < "$BASE/foot.out" | cut -c1-200 | tail -n 8
  exit 4
fi
say "events lines at start: $(wc -l < "$EVENTS") · kinds: $(python3 -c 'import json,sys
print(" ".join(json.loads(l).get("kind","?") for l in open(sys.argv[1])))' "$EVENTS")"
for _ in $(seq 1 50); do ADDR=$(field address); [ -n "$ADDR" ] && break; sleep 0.1; done
[ -n "$ADDR" ] || { say "ABORT probe window not found (or not unique) on the own instance"; exit 5; }
MAPPED=$(field mapped)
say "probe window listed on the own instance: class $CLASS · mapped $MAPPED · size $(field size)"
[ "$MAPPED" = "True" ] || { say "ABORT the probe window is not mapped; no key"; exit 9; }
W0=$(wheel_lines)
say "wheel records at window discovery: $W0"
sleep 1.5
read -r ACT ACLASS <<<"$(active)"
say "the window took focus by itself: active class ${ACLASS:-none}, address matches probe: $([ "$ACT" = "$ADDR" ] && echo yes || echo no)"
if [ "$ACT" != "$ADDR" ]; then
  say "focus dispatch on the own instance: $(hc dispatch "hl.dsp.focus({ window = \"address:$ADDR\" })" 2>&1 | tr '\n' ' ')"
  sleep 0.8
  read -r ACT ACLASS <<<"$(active)"
  say "after the dispatch: active class ${ACLASS:-none}, address matches probe: $([ "$ACT" = "$ADDR" ] && echo yes || echo no)"
fi
W1=$(wheel_lines)
say "wheel records with the window focused, before any key: $W1 (delta $((W1 - W0)))"

read -r ACT ACLASS <<<"$(active)"
LOCKED=$(hc locked | tr -d '\n')
NWIN=$(hc clients -j | python3 -c 'import json,sys
print(len(json.load(sys.stdin)))')
if [ "$ACT" != "$ADDR" ] || [ "$ACLASS" != "$CLASS" ] || [ "$LOCKED" != "false" ] || [ "$NWIN" != "1" ] || [ "$SIG" = "$DESK_SIG" ]; then
  say "ABORT guard: active class ${ACLASS:-none}, own instance locked $LOCKED, windows $NWIN; no key typed"
  exit 6
fi
WAYLAND_DISPLAY="$CRT/$WL" XDG_RUNTIME_DIR="$CRT" wtype x
say "wtype x sent to the own compositor: exit $? (guard passed: its active window is the probe, it holds 1 window, it is not locked)"

for _ in $(seq 1 50); do
  [ "$(wheel_lines)" -gt "$W1" ] && break
  sleep 0.1
done
W2=$(wheel_lines)
say "wheel records after the key: $W2"
python3 -c 'import json,sys
for l in open(sys.argv[1]):
    try: d=json.loads(l)
    except Exception: continue
    if d.get("kind")=="wheel": print("  wheel", json.dumps(d.get("data"), sort_keys=True))
    else: print("  event", d.get("kind"))' "$EVENTS"
if [ -f "$RECEIPT" ]; then
  python3 -c 'import json,sys
for l in open(sys.argv[1]):
    try: d=json.loads(l)
    except Exception: continue
    if d.get("kind")=="key": print("  receipt key", d.get("hex"))' "$RECEIPT"
else
  say "no receipt file"
fi

printf 'probe' | "$VIOLA" --home "$HOME_DIR" send keyprobe > "$BASE/send.out" 2> "$BASE/send.err"
say "driver send after the key: exit $? · stderr: $(tr '\n' '|' < "$BASE/send.err" | clean)"
say "send-issued records: $(command grep -c '"kind":"send-issued"' "$EVENTS" || true) · wheel records: $(wheel_lines)"
say "home kept at $REL (tmpfs behind the link)"
