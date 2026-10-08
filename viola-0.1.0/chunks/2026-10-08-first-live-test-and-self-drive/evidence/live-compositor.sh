#!/usr/bin/env bash
# The own compositor of chunk 2026-10-08-first-live-test-and-self-drive (plan.md, "The own compositor"): one
# Hyprland nested as one client window of the desktop compositor, started ONCE for all live work, with a private
# runtime dir, a config and sockets of its own. foot and wtype run inside it. Every hyprctl call goes through
# hc() and names the own instance; the one call that names the desktop instance is the `locked` READ of
# lock-read. No key and no dispatch ever goes to the desktop compositor, and nothing here unlocks anything.
# Prints relative names and codes only.
#
#   start                                    the one start (refused when a start was ever made)
#   status                                   the own compositor alive, its windows (read only)
#   hc <hyprctl arguments>                   hyprctl on the own instance
#   lock-read pre|after-start|after-end      the desktop lock, a read; exit 6 = not locked (S6)
#   window <class> <font size> -- <program and arguments>
#   guard <class>                            exit 3 = the guard failed (S3)
#   key <class> <text>                       guard, then wtype on the own socket
#   close <class>                            ends that window's own foot process by pid
#   end [force]                              ends the own compositor by pid
set -u

EVID=$(cd "$(dirname "$0")" && pwd)
ROOT=$(git -C "$EVID" rev-parse --show-toplevel) || exit 2
CFG=$EVID/live-compositor.lua
LEDGER=$EVID/live-sessions.ndjson
BASE=$ROOT/target/e2e-home
DESK_RT=${XDG_RUNTIME_DIR:?no XDG_RUNTIME_DIR}
DESK_SIG=${HYPRLAND_INSTANCE_SIGNATURE:?no desktop signature in the environment}
case "${WAYLAND_DISPLAY:-wayland-1}" in
  /*) DESK_WL=$WAYLAND_DISPLAY ;;
  *) DESK_WL=$DESK_RT/${WAYLAND_DISPLAY:-wayland-1} ;;
esac
CRT=$DESK_RT/vcomp
SD=""; CPID=""; CSTART=""; SIG=""; WL=""; GUARD_WHY=""

say() {
  printf '%s %s\n' "$(date -u +%H:%M:%S.%3NZ)" "$*" \
    | sed -e "s#$ROOT#.#g" -e "s#$DESK_RT#\$XDG_RUNTIME_DIR#g" -e "s#$HOME#~#g"
}
die() { say "REFUSED $2"; exit "$1"; }
pstart() { sed 's/.*) //' "/proc/$1/stat" 2>/dev/null | cut -d' ' -f20; }
pexe() { readlink "/proc/$1/exe" 2>/dev/null; }
sget() { sed -n "s/^$1=//p" "$SD/state" 2>/dev/null | tail -n 1; }
sput() { printf '%s=%s\n' "$1" "$2" >> "$SD/state"; }
now() { date +%s.%N; }
since() { awk -v e="$1" -v n="$(now)" 'BEGIN { printf "%.1f", n - e }'; }
wait_until() {
  local d
  d=$(awk -v e="$1" -v s="$2" -v n="$(now)" 'BEGIN { d = e + s - n; if (d < 0) d = 0; printf "%.3f", d }')
  sleep "$d"
}
# The desktop shell that holds the lock, by its command line (a read of the process list).
shell_pid() {
  ps -eo pid=,args= | awk '$2 == "quickshell" && index($0, "/usr/share/omarchy/shell") { print $1 }' | tr '\n' ' ' | sed 's/ $//'
}

load_state() {
  local found=() f
  for f in "$BASE"/viola-comp-*/state; do [ -f "$f" ] && found+=("$f"); done
  [ "${#found[@]}" -eq 1 ] || die 20 "state: ${#found[@]} compositor state files (expected 1)"
  SD=$(dirname "${found[0]}")
  CPID=$(sget pid); CSTART=$(sget pid_start); SIG=$(sget sig); WL=$(sget wl)
  [ -n "$CPID" ] && [ -n "$CSTART" ] || die 20 "state: no pid"
}
need_instance() {
  [ -n "$SIG" ] && [ -n "$WL" ] || die 20 "state: no own instance recorded"
  [ "$SIG" != "$DESK_SIG" ] || die 21 "state: the signature is the desktop's"
}
comp_alive() {
  [ -n "$CPID" ] && [ "$(pexe "$CPID")" = /usr/bin/Hyprland ] && [ "$(pstart "$CPID")" = "$CSTART" ] \
    && tr '\0' '\n' < "/proc/$CPID/environ" 2>/dev/null | command grep -q -x "XDG_RUNTIME_DIR=$CRT"
}
# Every hyprctl call but the desktop `locked` read: the private runtime dir and the own signature, with the
# desktop's signature and socket name removed from the call's environment.
hc() { env -u HYPRLAND_INSTANCE_SIGNATURE -u WAYLAND_DISPLAY XDG_RUNTIME_DIR="$CRT" hyprctl --instance "$SIG" "$@"; }
own_clients() { hc clients -j | jq 'length' 2>/dev/null; }

# The desktop lock, read only. desk_locked is the ONLY call that names the desktop instance.
desk_locked() { timeout 5 hyprctl --instance "$DESK_SIG" locked 2>/dev/null | tr -d '\n'; }
shell_locked() {
  local a
  a=$(timeout 5 omarchy-shell lock isLocked 2>/dev/null | tr -d '\n')
  case "$a" in true | false) printf '%s' "$a" ;; *) printf 'no-answer' ;; esac
}

cmd_lock_read() {
  local form=${1:-} event="" s1="absent" c1="absent" s2 c2 t0 shell_ok comp_ok before now_pid relaunched verdict f
  case "$form" in
    pre)
      # against the shell pid of the last reading (the one before the start when none was taken since)
      for f in "$BASE"/viola-comp-*/state; do [ -f "$f" ] && SD=$(dirname "$f"); done
      before=unread
      if [ -n "$SD" ]; then
        before=$(sget shell_pid_last)
        [ -n "$before" ] || before=$(sget shell_pid_before)
      fi
      ;;
    after-start | after-end)
      load_state
      event=$(sget "${form#after-}_epoch")
      [ -n "$event" ] || die 22 "lock-read: no ${form#after-} event in the state"
      if [ "$form" = after-start ]; then before=$(sget shell_pid_before); else before=$(sget shell_pid_before_end); fi
      wait_until "$event" 2
      s1=$(shell_locked); c1=$(desk_locked)
      say "lock-read $form · first reading at +$(since "$event") s · shell $s1 · desktop instance ${c1:-no-answer}"
      wait_until "$event" 15
      ;;
    *) die 2 "usage: lock-read pre|after-start|after-end" ;;
  esac
  t0=$(date +%s)
  while :; do
    s2=$(shell_locked)
    [ "$s2" != no-answer ] && break
    [ $(($(date +%s) - t0)) -ge 30 ] && break
    sleep 1
  done
  c2=$(desk_locked)
  if [ -n "$event" ]; then
    say "lock-read $form · second reading at +$(since "$event") s · shell $s2 · desktop instance ${c2:-no-answer}"
  else
    say "lock-read pre · one reading · shell $s2 · desktop instance ${c2:-no-answer}"
  fi
  shell_ok=true; comp_ok=true
  [ "$s2" = true ] || shell_ok=false
  [ "$s1" = false ] && shell_ok=false
  [ "$c2" = true ] || comp_ok=false
  { [ "$c1" = true ] || [ "$c1" = absent ]; } || comp_ok=false
  now_pid=$(shell_pid)
  if [ "$before" = unread ]; then relaunched=unread; elif [ "$before" = "$now_pid" ]; then relaunched=false; else relaunched=true; fi
  [ -n "$SD" ] && sput shell_pid_last "$now_pid"
  if [ "$shell_ok" = true ] && [ "$comp_ok" = true ]; then verdict=locked; else verdict=S6; fi
  say "lock-read $form · desktop_lock_shell=$shell_ok desktop_lock_compositor=$comp_ok shell_first_answer=$s1 shell_relaunched=$relaunched verdict=$verdict"
  [ "$verdict" = locked ] || exit 6
}

cmd_start() {
  local d l p ts before_sigs before_wl s w new_wl=() i
  if [ -f "$LEDGER" ] && jq -e -s 'any(.[]; .kind == "compositor" and .event == "start")' "$LEDGER" > /dev/null 2>&1; then
    die 10 "start: the ledger already holds a compositor start line"
  fi
  for d in "$BASE"/viola-comp-*/; do
    [ -d "$d" ] && die 10 "start: a compositor state dir already exists (one start, inputs#I13)"
  done
  [ -S /run/seatd.sock ] && die 11 "start: a seatd socket exists, the forced seatd backend could open a seat"
  [ -f "$CFG" ] || die 12 "start: no config beside the script"
  if [ -d "$CRT" ]; then
    [ "$(stat -c '%a %u' "$CRT")" = "700 $(id -u)" ] || die 12 "start: the private runtime dir is not 0700 and owned"
    for l in "$CRT"/hypr/*/hyprland.lock; do
      [ -f "$l" ] || continue
      p=$(head -n 1 "$l")
      [ -n "$p" ] && [ -d "/proc/$p" ] && die 13 "start: an instance is alive in the private runtime dir"
    done
  else
    mkdir -m 700 "$CRT" || die 12 "start: mkdir of the private runtime dir failed"
  fi
  before_sigs=$(ls "$CRT/hypr" 2> /dev/null | sort | tr '\n' ' ')
  before_wl=$(ls "$CRT" | command grep -E '^wayland-[0-9]+$' | sort | tr '\n' ' ')
  ts=$(date -u +%Y%m%dT%H%M%SZ)
  SD=$BASE/viola-comp-$ts
  mkdir -m 700 "$SD" || die 14 "start: mkdir of the state dir failed"
  : > "$SD/state"; chmod 600 "$SD/state"
  sput shell_pid_before "$(shell_pid)"
  ulimit -c 0
  setsid env -i HOME="$HOME" PATH="$PATH" USER="$USER" LANG="${LANG:-C.UTF-8}" XDG_RUNTIME_DIR="$CRT" \
    WAYLAND_DISPLAY="$DESK_WL" LIBSEAT_BACKEND=seatd HYPRLAND_NO_SD_VARS=1 HYPRLAND_NO_SD_NOTIFY=1 \
    HYPRLAND_NO_RT=1 HYPRLAND_NO_CRASHREPORTER=1 \
    Hyprland --config "$CFG" > "$SD/compositor.out" 2>&1 < /dev/null &
  CPID=$!
  sput start_epoch "$(now)"
  say "start: own compositor started, nested (one client window of the desktop compositor; no session bus, seat backend forced to an absent seatd) · state dir target/e2e-home/viola-comp-$ts"
  for i in $(seq 1 50); do
    [ "$(pexe "$CPID")" = /usr/bin/Hyprland ] && break
    [ -d "/proc/$CPID" ] || break
    sleep 0.1
  done
  if [ "$(pexe "$CPID")" != /usr/bin/Hyprland ]; then
    # setsid forked: the compositor is the Hyprland process whose runtime dir is the private one
    for p in $(pgrep -x Hyprland); do
      tr '\0' '\n' < "/proc/$p/environ" 2> /dev/null | command grep -q -x "XDG_RUNTIME_DIR=$CRT" && CPID=$p
    done
  fi
  sput pid "$CPID"
  CSTART=$(pstart "$CPID")
  sput pid_start "$CSTART"
  for i in $(seq 1 100); do
    [ -d "/proc/$CPID" ] || break
    for d in "$CRT"/hypr/*/; do
      [ -S "$d.socket.sock" ] || continue
      s=$(basename "$d")
      case " $before_sigs " in *" $s "*) continue ;; esac
      SIG=$s
    done
    [ -n "$SIG" ] && break
    sleep 0.1
  done
  if [ -z "$SIG" ]; then
    say "start: FAILED, no control socket (compositor alive: $([ -d "/proc/$CPID" ] && echo yes || echo no)) (S1)"
    command grep -a -i -E 'critical|error|failed|could not|cannot|backend' "$SD/compositor.out" | sed -E 's/\x1b\[[0-9;]*m//g' | cut -c1-200 | tail -n 15
    exit 15
  fi
  [ "$SIG" != "$DESK_SIG" ] || die 21 "start: the instance found is the desktop's"
  sput sig "$SIG"
  for i in $(seq 1 50); do
    new_wl=()
    for w in $(ls "$CRT" | command grep -E '^wayland-[0-9]+$' | sort); do
      case " $before_wl " in *" $w "*) continue ;; esac
      [ -S "$CRT/$w" ] && new_wl+=("$w")
    done
    [ "${#new_wl[@]}" -ge 1 ] && break
    sleep 0.1
  done
  [ "${#new_wl[@]}" -eq 1 ] || { say "start: FAILED, ${#new_wl[@]} new wayland sockets in the private runtime dir (expected 1) (S1)"; exit 16; }
  WL=${new_wl[0]}
  sput wl "$WL"
  comp_alive || { say "start: FAILED, the compositor is not alive under the private runtime dir (S1)"; exit 17; }
  say "start: own instance up · signature differs from the desktop's: yes · control socket in the private runtime dir: $([ -S "$CRT/hypr/$SIG/.socket.sock" ] && echo yes || echo no) · wayland socket \$XDG_RUNTIME_DIR/vcomp/$WL"
  say "start: version $(hc version | head -n 1 | cut -c1-60)"
  say "start: monitors $(hc monitors -j | jq -r '[.[] | "\(.name) \(.width)x\(.height) scale \(.scale)"] | join(", ")')"
  say "start: own instance locked $(hc locked | tr -d '\n') · windows $(own_clients)"
}

cmd_status() {
  load_state
  if comp_alive; then
    need_instance
    say "status: own compositor alive (pid and start time match) · up $(since "$(sget start_epoch)") s · locked $(hc locked | tr -d '\n') · windows $(own_clients) · classes [$(hc clients -j | jq -r '[.[].class] | join(",")')] · log $(stat -c %s "$CRT/hypr/$SIG/hyprland.log" 2> /dev/null || echo 0) B"
  else
    say "status: own compositor GONE (S7 if the live work has not ended)"
    exit 7
  fi
}

cmd_window() {
  local class=${1:-} font=${2:-} fpid i addr=""
  { [ -n "$class" ] && [ -n "$font" ] && [ "${3:-}" = -- ] && [ $# -ge 4 ]; } || die 2 "usage: window <class> <font size> -- <program and arguments>"
  shift 3
  load_state; need_instance
  comp_alive || die 30 "window: the own compositor is gone (S7)"
  i=$(own_clients)
  [ "$i" = 0 ] || die 31 "window: the own instance already holds ${i:-an unreadable number of} window(s)"
  setsid env -i HOME="$HOME" USER="$USER" LOGNAME="$USER" SHELL="${SHELL:-/bin/bash}" PATH="$PATH" \
    LANG="${LANG:-C.UTF-8}" XDG_RUNTIME_DIR="$DESK_RT" WAYLAND_DISPLAY="$CRT/$WL" \
    foot --app-id="$class" --title="$class" --font="monospace:size=$font" --override=pad=0x0 \
    --working-directory="$ROOT" -e "$@" > "$SD/foot-$class.out" 2>&1 < /dev/null &
  fpid=$!
  for i in $(seq 1 50); do
    [ "$(pexe "$fpid")" = /usr/bin/foot ] && break
    [ -d "/proc/$fpid" ] || break
    sleep 0.1
  done
  printf 'pid=%s\npid_start=%s\nfont=%s\n' "$fpid" "$(pstart "$fpid")" "$font" > "$SD/win-$class"
  for i in $(seq 1 100); do
    addr=$(hc clients -j | jq -r --arg c "$class" '[.[] | select(.class == $c)] | if length == 1 then .[0].address else "" end')
    [ -n "$addr" ] && break
    [ -d "/proc/$fpid" ] || break
    sleep 0.1
  done
  if [ -z "$addr" ]; then
    say "window: FAILED, no window of class $class on the own instance (foot alive: $([ -d "/proc/$fpid" ] && echo yes || echo no))"
    sed -e "s#$ROOT#.#g" -e "s#$HOME#~#g" "$SD/foot-$class.out" | cut -c1-200 | tail -n 6
    exit 32
  fi
  say "window: class $class · font size $font · $(hc clients -j | jq -r --arg c "$class" '.[] | select(.class == $c) | "mapped \(.mapped) · size \(.size[0])x\(.size[1])"') · environment declared (no CLAUDE and no VIOLA name)"
}

guard_check() {
  local class=$1 cj addr act
  comp_alive || { GUARD_WHY="the own compositor is gone"; return 1; }
  [ "$(hc locked | tr -d '\n')" = false ] || { GUARD_WHY="the own instance is locked or unreadable"; return 1; }
  cj=$(hc clients -j) || { GUARD_WHY="clients unreadable"; return 1; }
  [ "$(jq 'length' <<< "$cj")" = 1 ] || { GUARD_WHY="the own instance holds $(jq 'length' <<< "$cj") windows"; return 1; }
  [ "$(jq -r '.[0].class' <<< "$cj")" = "$class" ] || { GUARD_WHY="the one window is not of class $class"; return 1; }
  addr=$(jq -r '.[0].address' <<< "$cj")
  act=$(hc activewindow -j | jq -r '.address // ""' 2> /dev/null)
  { [ -n "$addr" ] && [ "$act" = "$addr" ]; } || { GUARD_WHY="the active window is not the chunk's window"; return 1; }
  return 0
}

cmd_guard() {
  [ -n "${1:-}" ] || die 2 "usage: guard <class>"
  load_state; need_instance
  if guard_check "$1"; then
    say "guard: ok · own compositor alive, own instance not locked, one window, class $1, active address equal"
  else
    say "guard: FAILED · $GUARD_WHY (S3: no key)"
    exit 3
  fi
}

cmd_key() {
  local class=${1:-} text=${2:-} rc
  { [ -n "$class" ] && [ -n "$text" ]; } || die 2 "usage: key <class> <text>"
  load_state; need_instance
  if ! guard_check "$class"; then
    say "key: guard FAILED · $GUARD_WHY · nothing typed (S3)"
    exit 3
  fi
  env -u HYPRLAND_INSTANCE_SIGNATURE XDG_RUNTIME_DIR="$CRT" WAYLAND_DISPLAY="$CRT/$WL" wtype "$text"
  rc=$?
  say "key: guard ok immediately before the key · wtype on the own socket exit $rc · ${#text} character(s)"
  return "$rc"
}

cmd_close() {
  local class=${1:-} pid st i
  [ -n "$class" ] || die 2 "usage: close <class>"
  load_state
  [ -f "$SD/win-$class" ] || die 33 "close: no window of class $class was opened by this script"
  pid=$(sed -n 's/^pid=//p' "$SD/win-$class"); st=$(sed -n 's/^pid_start=//p' "$SD/win-$class")
  if [ "$(pexe "$pid")" = /usr/bin/foot ] && [ "$(pstart "$pid")" = "$st" ]; then
    kill -TERM "$pid"
    say "close: TERM sent to the window's own foot process"
  else
    say "close: the window's foot process is already gone"
  fi
  for i in $(seq 1 100); do
    { [ "$(pexe "$pid")" = /usr/bin/foot ] && [ "$(pstart "$pid")" = "$st" ]; } || break
    sleep 0.1
  done
  say "close: foot gone $({ [ "$(pexe "$pid")" = /usr/bin/foot ] && [ "$(pstart "$pid")" = "$st" ]; } && echo no || echo yes) · windows on the own instance $(comp_alive && [ -n "$SIG" ] && own_clients || echo unread)"
}

cmd_end() {
  local n i left=0 f pid st
  load_state
  [ -z "$(sget end_epoch)" ] || die 41 "end: the own compositor was already ended"
  if comp_alive && [ -n "$SIG" ]; then
    n=$(own_clients)
    { [ "$n" = 0 ] || [ "${1:-}" = force ]; } || die 40 "end: ${n:-unread} window(s) left on the own instance; close first"
    say "end: windows on the own instance before the end: ${n:-unread}"
  fi
  sput shell_pid_before_end "$(shell_pid)"
  if comp_alive; then
    kill -TERM "$CPID"
    sput end_epoch "$(now)"
    say "end: TERM sent to the own compositor (its runtime dir read from its environment first)"
    for i in $(seq 1 50); do [ -d "/proc/$CPID" ] || break; sleep 0.1; done
    if [ -d "/proc/$CPID" ] && [ "$(pstart "$CPID")" = "$CSTART" ]; then
      kill -KILL "$CPID"; sleep 0.5
      say "end: still alive after 5 s, KILL sent"
    fi
  else
    sput end_epoch "$(now)"
    say "end: the own compositor was found gone before the end (S7)"
  fi
  { [ -d "/proc/$CPID" ] && [ "$(pstart "$CPID")" = "$CSTART" ]; } && left=$((left + 1))
  for f in "$SD"/win-*; do
    [ -f "$f" ] || continue
    pid=$(sed -n 's/^pid=//p' "$f"); st=$(sed -n 's/^pid_start=//p' "$f")
    { [ "$(pexe "$pid")" = /usr/bin/foot ] && [ "$(pstart "$pid")" = "$st" ]; } && left=$((left + 1))
  done
  say "end: own compositor gone $({ [ -d "/proc/$CPID" ] && [ "$(pstart "$CPID")" = "$CSTART" ]; } && echo no || echo yes) · own_left=$left (the compositor and the windows' foot processes still present) · Hyprland processes on the host $(ps -eo comm= | command grep -c -x Hyprland)"
}

verb=${1:-}
[ $# -ge 1 ] && shift
case "$verb" in
  start) cmd_start ;;
  status) cmd_status ;;
  hc) load_state; need_instance; comp_alive || die 30 "hc: the own compositor is gone"; hc "$@" ;;
  lock-read) cmd_lock_read "$@" ;;
  window) cmd_window "$@" ;;
  guard) cmd_guard "$@" ;;
  key) cmd_key "$@" ;;
  close) cmd_close "$@" ;;
  end) cmd_end "$@" ;;
  *) die 2 "usage: live-compositor.sh start|status|hc|lock-read|window|guard|key|close|end" ;;
esac
