#!/usr/bin/env bash
# Focus diagnosis: opens ONE plain foot window (class viola.focusprobe, running sleep) on an empty
# workspace 9, dispatches focus to it by address, and saves what the compositor logged meanwhile.
# Types no key. Never dispatches to any other window.
set -u
S=$(dirname "$0")
LOG=$(ls -d "$XDG_RUNTIME_DIR"/hypr/*/ | head -n 1)hyprland.log
CLASS=viola.focusprobe
say() { printf '%s %s\n' "$(date -u +%H:%M:%S.%3NZ)" "$*"; }
active() { hyprctl activewindow -j | python3 -c 'import json,sys
try:
    w=json.load(sys.stdin); print(w.get("address",""), w.get("class",""))
except Exception:
    print("", "")'; }
dpms() { hyprctl monitors -j | python3 -c 'import json,sys
print(" ".join(m["name"]+"="+str(m.get("dpmsStatus"))+"/ws"+str(m["activeWorkspace"]["name"])+"/f"+str(m["focused"]) for m in json.load(sys.stdin)))'; }
field() { hyprctl clients -j | python3 -c 'import json,sys
c=[w for w in json.load(sys.stdin) if w.get("class")==sys.argv[1]]
print(c[0].get(sys.argv[2],"") if len(c)==1 else "")' "$CLASS" "$1"; }

OFF0=$(wc -c < "$LOG")
say "log offset at start $OFF0"
say "monitors at start: $(dpms)"
say "dpms on: $(hyprctl dispatch 'hl.dsp.dpms("on")' 2>&1 | tr '\n' ' ')"
sleep 1
say "monitors: $(dpms)"
say "before: active class $(active | cut -d' ' -f2)"
say "workspace 9: $(hyprctl dispatch 'hl.dsp.focus({ workspace = 9 })' 2>&1 | tr '\n' ' ')"
sleep 0.5
say "active workspace: $(hyprctl activeworkspace -j | python3 -c 'import json,sys
w=json.load(sys.stdin); print(w.get("name"), w.get("windows"), w.get("monitor"))')"
OFF1=$(wc -c < "$LOG")
say "exec: $(hyprctl dispatch "hl.dsp.exec_cmd(\"foot --app-id=$CLASS --title=viola-focusprobe -e sleep 900\")" 2>&1 | tr '\n' ' ')"
for _ in $(seq 1 50); do [ -n "$(field address)" ] && break; sleep 0.1; done
ADDR=$(field address)
[ -n "$ADDR" ] || { say "ABORT no probe window"; exit 5; }
say "probe window listed: mapped $(field mapped) · focusHistoryID $(field focusHistoryID) · at $(field at) size $(field size)"
sleep 1.2
say "1.2 s later: active class $(active | cut -d' ' -f2) · probe focusHistoryID $(field focusHistoryID)"
OFF2=$(wc -c < "$LOG")
say "focus dispatch by address: $(hyprctl dispatch "hl.dsp.focus({ window = \"address:$ADDR\" })" 2>&1 | tr '\n' ' ')"
sleep 0.8
read -r ACT ACLASS <<<"$(active)"
say "after: active class $ACLASS · matches probe: $([ "$ACT" = "$ADDR" ] && echo yes || echo no) · cursor $(hyprctl cursorpos)"
OFF3=$(wc -c < "$LOG")
say "monitors: $(dpms)"
tail -c +"$((OFF0 + 1))" "$LOG" | head -c "$((OFF1 - OFF0))" | command grep -v 'from aquamarine' > "$S/diag-log-a-workspace.txt"
tail -c +"$((OFF1 + 1))" "$LOG" | head -c "$((OFF2 - OFF1))" | command grep -v 'from aquamarine' > "$S/diag-log-b-open.txt"
tail -c +"$((OFF2 + 1))" "$LOG" | head -c "$((OFF3 - OFF2))" | command grep -v 'from aquamarine' > "$S/diag-log-c-focus.txt"
say "log lines kept: workspace $(wc -l < "$S/diag-log-a-workspace.txt") · open $(wc -l < "$S/diag-log-b-open.txt") · focus $(wc -l < "$S/diag-log-c-focus.txt")"
say "probe window left open at $ADDR (pid $(field pid))"
