#!/usr/bin/env bash
# Compositor-key probe: does a wtype key typed into a foot window running `viola run` over the
# fake agent move the wheel? No live claude start. Prints relative names and codes only.
set -u
ROOT=$(git rev-parse --show-toplevel)
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
FORBIDDEN_A=viola.viola-builder
FORBIDDEN_B=overseer.viola-overseer

say() { printf '%s %s\n' "$(date -u +%H:%M:%S.%3NZ)" "$*"; }
active() { hyprctl activewindow -j | python3 -c 'import json,sys
try:
    w=json.load(sys.stdin); print(w.get("address",""), w.get("class",""))
except Exception:
    print("", "")'; }
wheel_lines() { [ -f "$EVENTS" ] && command grep -c '"kind":"wheel"' "$EVENTS" || echo 0; }
close_probe() {
  # End the probe's own foot process by pid; no close dispatcher (it acts on the active window).
  local fp
  fp=$(hyprctl clients -j | python3 -c 'import json,sys
c=[w for w in json.load(sys.stdin) if w.get("class")==sys.argv[1]]
print(c[0]["pid"] if len(c)==1 else "")' "$CLASS")
  if [ -n "$fp" ] && [ "$(readlink "/proc/$fp/exe" 2>/dev/null)" = "/usr/bin/foot" ]; then
    kill -TERM "$fp"
  fi
}

dpms() { hyprctl monitors -j | python3 -c 'import json,sys
print(" ".join(m["name"]+"="+str(m.get("dpmsStatus")) for m in json.load(sys.stdin)))'; }
restore() {
  close_probe
  sleep 0.5
  say "restore workspace 7 on the second monitor: $(hyprctl dispatch 'hl.dsp.focus({ workspace = 7 })' 2>&1 | tr '\n' ' ')"
  say "DPMS off: $(hyprctl dispatch 'hl.dsp.dpms("off")' 2>&1 | tr '\n' ' ')"
  sleep 0.8
  say "end state: dpms $(dpms) · active window class $(active | cut -d' ' -f2) · probe windows $(hyprctl clients -j | python3 -c 'import json,sys
print(len([w for w in json.load(sys.stdin) if w.get("class")==sys.argv[1]]))' "$CLASS")"
}
trap restore EXIT

say "dpms at start: $(dpms)"
hyprctl dispatch 'hl.dsp.dpms("on")' >/dev/null 2>&1
sleep 0.8
case "$(dpms)" in *=False*) say "ABORT DPMS did not come on: $(dpms)"; exit 8;; esac
say "dpms on: $(dpms)"
say "probe home $REL/home (not yet existing)"
mkdir -m 700 "$BASE" || { say "ABORT mkdir base"; exit 3; }
say "before: active window class $(active | cut -d' ' -f2)"

say "workspace switch: $(hyprctl dispatch 'hl.dsp.focus({ workspace = 9 })' 2>&1 | tr '\n' ' ')"
sleep 0.5
WS=$(hyprctl activeworkspace -j | python3 -c 'import json,sys
w=json.load(sys.stdin); print(w.get("name"), w.get("windows"))')
if [ "$WS" != "9 0" ]; then
  say "ABORT the active workspace is not an empty workspace 9 (name and window count: $WS); nothing launched"
  exit 7
fi
say "active workspace 9, empty"
say "exec: $(hyprctl dispatch "hl.dsp.exec_cmd(\"foot --app-id=$CLASS --title=viola-keyprobe --working-directory=$ROOT -e $VIOLA --home $HOME_DIR run keyprobe -- $FAKE --cli-version 2.1.287 --fixtures $ROOT/fixtures/claude --screens --trusted-root $ROOT --receipt $RECEIPT\")" 2>&1 | tr '\n' ' ')"

for _ in $(seq 1 150); do
  [ -f "$EVENTS" ] && [ "$(wc -l < "$EVENTS")" -ge 3 ] && break
  sleep 0.1
done
if ! [ -f "$EVENTS" ]; then
  say "ABORT no events file after 15 s"
  [ -f "$HOME_DIR/diagnostics/run-keyprobe.ndjson" ] && python3 -c 'import json,sys
for l in open(sys.argv[1]):
    try:
        d=json.loads(l); print("  run-log", d.get("event"), d.get("detail"), d.get("subject"))
    except Exception: pass' "$HOME_DIR/diagnostics/run-keyprobe.ndjson"
  exit 4
fi
say "events lines at start: $(wc -l < "$EVENTS")"

ADDR=$(hyprctl clients -j | python3 -c 'import json,sys
c=[w for w in json.load(sys.stdin) if w.get("class")==sys.argv[1]]
print(c[0]["address"] if len(c)==1 else "")' "$CLASS")
[ -n "$ADDR" ] || { say "ABORT probe window not found (or not unique)"; exit 5; }
say "probe window found, class $CLASS, workspace $(hyprctl clients -j | python3 -c 'import json,sys
print([w["workspace"]["name"] for w in json.load(sys.stdin) if w.get("address")==sys.argv[1]][0])' "$ADDR")"

MAPPED=$(hyprctl clients -j | python3 -c 'import json,sys
print([w.get("mapped") for w in json.load(sys.stdin) if w.get("address")==sys.argv[1]][0])' "$ADDR")
say "probe window mapped (read before any focus or key): $MAPPED"
[ "$MAPPED" = "True" ] || { say "ABORT the probe window is not mapped; no focus, no key"; exit 9; }
W0=$(wheel_lines)
say "wheel records at window discovery: $W0"
sleep 1.5
read -r ACT ACLASS <<<"$(active)"
say "the new window took focus by itself: active class $ACLASS, address matches probe: $([ "$ACT" = "$ADDR" ] && echo yes || echo no)"
if [ "$ACT" != "$ADDR" ]; then
  say "focus dispatch on the probe window by address: $(hyprctl dispatch "hl.dsp.focus({ window = \"address:$ADDR\" })" 2>&1 | tr '\n' ' ')"
  sleep 0.8
  read -r ACT ACLASS <<<"$(active)"
  say "after the focus dispatch: active class $ACLASS, address matches probe: $([ "$ACT" = "$ADDR" ] && echo yes || echo no)"
fi
W1=$(wheel_lines)
say "wheel records after focus, before any key: $W1"

read -r ACT ACLASS <<<"$(active)"
if [ "$ACT" != "$ADDR" ] || [ "$ACLASS" = "$FORBIDDEN_A" ] || [ "$ACLASS" = "$FORBIDDEN_B" ]; then
  say "ABORT guard: active window is not the probe window (class $ACLASS); no key typed"
  exit 6
fi
wtype x
say "wtype x sent (guard passed: active address == probe address)"

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
    print("  event", d.get("kind"), d.get("source"), json.dumps(d.get("data"), sort_keys=True))' "$EVENTS"
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
say "driver send after the key: exit $? · stderr: $(tr '\n' '|' < "$BASE/send.err")"
"$VIOLA" --home "$HOME_DIR" release keyprobe > "$BASE/release.out" 2> "$BASE/release.err"
say "release from this session (VIOLA_NAME set in its environment): exit $? · stderr: $(tr '\n' '|' < "$BASE/release.err")"
say "wheel records after the release attempt: $(wheel_lines)"

PID=$(python3 -c 'import json,sys
print(json.load(open(sys.argv[1]))["data"]["pid"])' "$SNAP" 2>/dev/null)
close_probe
say "TERM sent to the probe window's own foot process"
for _ in $(seq 1 100); do
  [ -n "$PID" ] && [ -d "/proc/$PID" ] || break
  sleep 0.1
done
if [ -n "$PID" ] && [ -d "/proc/$PID" ]; then
  EXE=$(readlink "/proc/$PID/exe" 2>/dev/null)
  if [ "$EXE" = "$VIOLA" ]; then kill -TERM "$PID"; say "wrapper still alive after 10 s: TERM sent by pid"; else say "pid reused by another exe; nothing sent"; fi
else
  say "wrapper process gone"
fi
sleep 0.5
say "probe windows left: $(hyprctl clients -j | python3 -c 'import json,sys
print(len([w for w in json.load(sys.stdin) if w.get("class")==sys.argv[1]]))' "$CLASS")"
say "after: active window class $(active | cut -d' ' -f2)"
say "home kept at $REL (tmpfs behind the link)"
