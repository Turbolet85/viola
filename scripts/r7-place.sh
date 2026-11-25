#!/usr/bin/env bash
# R7: one placing `viola open` read at the operator's host for the active workspace (test-only; never
# run by a gate, fired once by the operator pass on his word). It opens ONE terminal window, starts NO
# claude (a stand-in program this script writes itself answers the wrapper as `claude`), and unsets
# every inherited `VIOLA_*` name before it starts anything. It reads the focused monitor's active
# workspace, opens `r7place` onto another workspace, reads the active workspace again, and closes the
# window; the view may flash once while the window step moves the window and focuses the view back.
#
#   bash scripts/r7-place.sh <scratch root> <built viola> [--rehearse]
#
# `--rehearse` runs the same steps over the fake agent standing as `foot` and `hyprctl` (terminal
# mode) in a scratch bin dir that is every child's whole PATH, with the same stand-in as `claude`,
# under a scratch user home and runtime dir: it reaches no real compositor and starts no claude.
#
# Lines:
#   r7: build <sha256 of the built viola>                         (right before the start)
#   r7: red precondition=<word>                                   (nothing started)
#   r7: <green|red> placement=<word> active=<unchanged|moved|unread>  (the one verdict line)
#   r7: rehearsed — not a live reading                            (last, under --rehearse only)
set -u

rehearse=
if [ "$#" -eq 3 ] && [ "$3" = --rehearse ]; then
    rehearse=1
elif [ "$#" -ne 2 ]; then
    echo "usage: bash scripts/r7-place.sh <scratch root> <built viola> [--rehearse]" >&2
    exit 2
fi
for name in $(compgen -e); do
    case $name in
        VIOLA_*) unset "$name" ;;
    esac
done
repo=$(cd "$(dirname -- "$0")/.." && pwd)
viola=$(realpath -- "$2")
root=$1
gone_bound=5

placement=unread
active=unread
foot_pid=
cleaned=

precondition() {
    echo "r7: red precondition=$1"
    [ -z "$rehearse" ] || echo "r7: rehearsed — not a live reading"
    exit 1
}

# Preconditions, each read before anything starts.
[ -e "$root" ] && precondition scratch
command -v jq >/dev/null || precondition jq
mkdir -p -m 0700 -- "$root"
scratch=$(realpath -- "$root")
home=$scratch/home
work=$scratch/work
bin=$scratch/bin
stand_in=$bin/claude
mkdir -p -m 0700 "$work" "$bin"

if [ -n "$rehearse" ]; then
    child_path=$bin
    HYPRCTL=$bin/hyprctl
    FOOT=$bin/foot
else
    child_path=$bin:$PATH
    HYPRCTL=$(command -v hyprctl || true)
    FOOT=$(command -v foot || true)
fi

# A command started with the run's PATH (and, rehearsed, the scratch user home and runtime dir).
started() {
    if [ -n "$rehearse" ]; then
        env "PATH=$child_path" "HOME=$scratch/user" "XDG_RUNTIME_DIR=$scratch/run" "$@"
    else
        env "PATH=$child_path" "$@"
    fi
}

# The focused monitor's active workspace, or empty when it cannot be read.
active_workspace() {
    "$HYPRCTL" -j monitors 2>/dev/null |
        jq -r '[.[] | select(.focused == true) | .activeWorkspace.id][0] // empty' 2>/dev/null
}

# Whether a pid still holds a window in the compositor's clients; the fake lists its stand-in's
# window from a file whatever runs, so a rehearsal reads the process alone.
window_of_pid() {
    [ -z "$rehearse" ] || return 1
    "$HYPRCTL" -j clients 2>/dev/null | jq -e --argjson pid "$1" 'any(.[]; .pid == $pid)' \
        >/dev/null 2>&1
}

# The wrapper the open started, by the pid its own snapshot records, only while that pid still
# runs the built viola or its pinned copy.
wrapper_pid() {
    local pid exe
    pid=$(jq -r '.data.pid // empty' "$home/instances/r7place/snapshot.json" 2>/dev/null)
    [ -n "$pid" ] || return 0
    exe=$(readlink "/proc/$pid/exe" 2>/dev/null) || return 0
    case "${exe% (deleted)}" in
        */bin/*/viola | "$viola") echo "$pid" ;;
    esac
}

# The terminal `open` spawned, by its own start line, while that pid still runs `foot`.
logged_foot() {
    local pid
    pid=$(jq -r 'select(.event == "process-start" and .subject == "terminal") | .child_pid' \
        "$home/diagnostics/cli-r7place.ndjson" 2>/dev/null | tail -n 1)
    [ -n "$pid" ] || return 0
    [ "$(cat "/proc/$pid/comm" 2>/dev/null)" = foot ] && echo "$pid"
}

# The wrapper ended by TERM and its terminal waited gone; with no wrapper standing, the terminal
# `open` spawned is ended by TERM instead. A terminal outliving the bound is killed.
cleanup() {
    local wrapper waited=0
    [ -z "$cleaned" ] || return 0
    cleaned=1
    wrapper=$(wrapper_pid)
    if [ -n "$wrapper" ]; then
        foot_pid=$(awk '/^PPid:/ {print $2}' "/proc/$wrapper/status" 2>/dev/null)
        kill -TERM "$wrapper" 2>/dev/null
    else
        foot_pid=$(logged_foot)
        [ -z "$foot_pid" ] || kill -TERM "$foot_pid" 2>/dev/null
    fi
    [ -n "$foot_pid" ] || return 0
    [ "$(cat "/proc/$foot_pid/comm" 2>/dev/null)" = foot ] || return 0
    while { [ -d "/proc/$foot_pid" ] || window_of_pid "$foot_pid"; } &&
        [ "$waited" -lt $((gone_bound * 10)) ]; do
        sleep 0.1
        waited=$((waited + 1))
    done
    if [ -d "/proc/$foot_pid" ] && [ "$(cat "/proc/$foot_pid/comm" 2>/dev/null)" = foot ]; then
        kill -TERM "$foot_pid" 2>/dev/null
        sleep 1
        [ -d "/proc/$foot_pid" ] && kill -KILL "$foot_pid" 2>/dev/null
    fi
}
trap cleanup EXIT

finish() {
    cleanup
    if [ "$placement" = placed ] && [ "$active" = unchanged ]; then
        echo "r7: green placement=$placement active=$active"
        [ -z "$rehearse" ] || echo "r7: rehearsed — not a live reading"
        exit 0
    fi
    echo "r7: red placement=$placement active=$active"
    [ -z "$rehearse" ] || echo "r7: rehearsed — not a live reading"
    exit 1
}

if [ -n "$rehearse" ]; then
    fake=$(dirname -- "$viola")/viola-fake-agent
    [ -x "$fake" ] || precondition fake-agent
    mkdir -p -m 0700 "$bin/hyprctl-set" "$scratch/user" "$scratch/run"
    printf '{}\n' >"$scratch/user/.claude.json"
    chmod 0600 "$scratch/user/.claude.json"
    cp -- "$repo"/fixtures/hyprland/0.56.2/*.json "$bin/hyprctl-set/"
    printf 'terminal' >"$bin/hyprctl-mode"
    for tool in foot hyprctl; do
        cp -- "$fake" "$bin/$tool"
    done
fi
[ -n "$FOOT" ] && [ -x "$FOOT" ] || precondition foot
[ -n "$HYPRCTL" ] && [ -x "$HYPRCTL" ] || precondition hyprctl
# The lock is the real compositor's; the fake answers no `locked`.
if [ -z "$rehearse" ] && [ "$("$HYPRCTL" locked 2>/dev/null)" != false ]; then
    precondition locked
fi

# The program `open` starts as `claude`: the wrapper hands every child its own leading arguments
# ahead of the program's words, and its version probe runs `<program> --version` alone. This
# stand-in answers that probe with a line no version parse reads, so the CLI stays unverified (no
# trust record written, no automatic verify), ignores whatever else it is handed, and sleeps.
printf '%s\n' '#!/bin/sh' \
    'if [ "$#" -eq 1 ] && [ "$1" = --version ]; then' \
    '    echo "r7 stand-in"' \
    '    exit 0' \
    'fi' \
    'exec /usr/bin/sleep 900' >"$stand_in"
chmod 0700 "$stand_in"
[ "$("$stand_in" --version 2>/dev/null)" = "r7 stand-in" ] || precondition stand-in

# The reading: the active workspace A, then a target N other than A, the lowest of 1-9 holding
# no window, else the lowest that is not A.
before=$(active_workspace)
target=
held=$("$HYPRCTL" -j workspaces 2>/dev/null | jq -r '.[] | select(.windows > 0) | .id' 2>/dev/null)
for n in 1 2 3 4 5 6 7 8 9; do
    [ "$n" = "$before" ] && continue
    if ! grep -qx -- "$n" <<<"$held"; then
        target=$n
        break
    fi
done
if [ -z "$target" ]; then
    for n in 1 2 3 4 5 6 7 8 9; do
        [ "$n" = "$before" ] || {
            target=$n
            break
        }
    done
fi
if [ -n "$rehearse" ] && [ "$target" != 1 ]; then
    sed "s/workspace = 1,/workspace = $target,/" \
        "$repo/fixtures/hyprland/0.56.2/MoveToWorkspace.recorded-1.json" \
        >"$bin/hyprctl-set/MoveToWorkspace.rehearse-$target.json"
fi

echo "r7: build $(sha256sum -- "$viola" | cut -d' ' -f1)"
opened=$(cd "$work" && started "$viola" --home "$home" open r7place --workspace "$target" \
    --json -- claude 2>/dev/null)
open_exit=$?
if [ "$open_exit" -eq 0 ]; then
    placement=$(jq -r '.ok.placement // "unread"' <<<"$opened" 2>/dev/null || echo unread)
else
    placement=$(jq -r '.data.window.placement // "unread"' \
        "$home/instances/r7place/snapshot.json" 2>/dev/null || echo unread)
fi
[ -n "$placement" ] || placement=unread

after=$(active_workspace)
if [ -z "$before" ] || [ -z "$after" ]; then
    active=unread
elif [ "$after" = "$before" ]; then
    active=unchanged
else
    active=moved
fi
finish
