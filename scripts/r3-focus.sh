#!/usr/bin/env bash
# R3: `viola focus` read end to end at the operator's host (test-only; never run by a gate, fired
# once by the operator pass on his word). Two arms, in order:
#   R3a  one foot window running a hand-started `viola run r3hand` over a stand-in program this script
#        writes itself (no claude), then ONE `viola focus r3hand` and ONE active-window read;
#   R3b  only after a green R3a: ONE `viola open r3open --json -- claude` — the ONE capped claude start
#        of R3, no spare — then ONE `viola focus r3open` and ONE active-window read.
# A red R3a ends the script at once: no open, no claude. Both windows are closed and waited gone on
# every exit path; the focus held before the run is not restored.
#
#   bash scripts/r3-focus.sh <scratch root> <built viola> [--rehearse]
#
# `--rehearse` runs the same steps over the fake agent standing as `foot`, `hyprctl` and `claude` in a
# scratch bin dir that is every child's whole PATH, under a scratch user home and runtime dir: it
# reaches no real compositor and starts no real claude. There a focus reads `focused` when it exited 0
# and the fake's last `hyprctl-asked` line is the window dispatch on the stand-in foot's address.
#
# It reads no environment variable but PATH, XDG_RUNTIME_DIR and HOME, and unsets every `VIOLA_*`
# name first, so nothing it starts takes a name from the session that fired it. Every `viola list` it
# makes runs with PATH set to an empty dir, so the board's `claude agents --json` read finds no claude.
#
# Lines:
#   r3: build <sha256 of the built viola>                     (right before the first start)
#   r3: red precondition=<word>                               (nothing started)
#   r3: <green|red> hand=<word> opened=<word>                 (the one verdict line)
#   r3: rehearsed — not a live reading                        (last, under --rehearse only)
set -u

rehearse=
if [ "$#" -eq 3 ] && [ "$3" = --rehearse ]; then
    rehearse=1
elif [ "$#" -ne 2 ]; then
    echo "usage: bash scripts/r3-focus.sh <scratch root> <built viola> [--rehearse]" >&2
    exit 2
fi
for name in $(compgen -e); do
    case $name in
        VIOLA_*) unset "$name" ;;
    esac
done
repo=$(cd "$(dirname -- "$0")/.." && pwd)
viola=$(realpath -- "$2")
mkdir -p -m 0700 -- "$1"
scratch=$(realpath -- "$1")
hand_home=$scratch/hand
opened_home=$scratch/opened
config=$scratch/claude-config
work=$scratch/work
no_claude=$work/no-claude
stand_in=$work/r3-stand-in
live_bound=30
gone_bound=5

hand=unfired
opened=unfired
hand_foot=
opened_foot=
cleaned=

if [ -n "$rehearse" ]; then
    bin=$scratch/bin
    child_path=$bin
    HYPRCTL=$bin/hyprctl
    FOOT=$bin/foot
else
    child_path=$PATH
    HYPRCTL=$(command -v hyprctl || true)
    FOOT=$(command -v foot || true)
fi

# A command started with the arm's PATH (and, rehearsed, the scratch user home and runtime dir);
# leading NAME=VALUE words are added to its environment.
started() {
    if [ -n "$rehearse" ]; then
        env "PATH=$child_path" "HOME=$scratch/user" "XDG_RUNTIME_DIR=$scratch/run" "$@"
    else
        env "PATH=$child_path" "$@"
    fi
}

# The board of a home, read with no claude on PATH.
board_json() {
    local home=$1
    if [ -n "$rehearse" ]; then
        env "PATH=$no_claude" "HOME=$scratch/user" "XDG_RUNTIME_DIR=$scratch/run" \
            "$viola" --home "$home" list --json 2>/dev/null
    else
        PATH=$no_claude "$viola" --home "$home" list --json 2>/dev/null
    fi
}

live() {
    board_json "$1" | jq -e --arg n "$2" 'any(.ok.items[]; .name == $n and .liveness == "live")' \
        >/dev/null 2>&1
}

# Whether a pid still holds a window in the compositor's clients; the fake lists its stand-in's
# window from a file, whatever runs, so a rehearsal reads the process alone.
window_of_pid() {
    [ -z "$rehearse" ] || return 1
    "$HYPRCTL" -j clients | jq -e --argjson pid "$1" 'any(.[]; .pid == $pid)' >/dev/null 2>&1
}

close_foot() {
    local pid=$1 waited=0
    [ -n "$pid" ] || return 0
    [ -d "/proc/$pid" ] || return 0
    kill -TERM "$pid" 2>/dev/null
    while { [ -d "/proc/$pid" ] || window_of_pid "$pid"; } && [ "$waited" -lt $((gone_bound * 10)) ]; do
        sleep 0.1
        waited=$((waited + 1))
    done
    if [ -d "/proc/$pid" ]; then
        kill -KILL "$pid" 2>/dev/null
    fi
}

# A wrapper and its program left by a closed terminal, by the pid the wrapper's own snapshot
# records, only while that pid still runs the built viola or its pinned copy.
end_wrapper() {
    local home=$1 name=$2 pid exe kids
    pid=$(jq -r '.data.pid // empty' "$home/instances/$name/snapshot.json" 2>/dev/null)
    [ -n "$pid" ] || return 0
    exe=$(readlink "/proc/$pid/exe" 2>/dev/null) || return 0
    case "$exe" in
        */bin/*/viola | "$viola")
            kids=$(cat "/proc/$pid/task/$pid/children" 2>/dev/null)
            kill -TERM "$pid" 2>/dev/null
            for kid in $kids; do
                kill -TERM "$kid" 2>/dev/null
            done
            ;;
    esac
}

# The open's terminal, found by the wrapper the open started: the parent of its snapshot's pid.
opened_terminal() {
    local pid
    pid=$(jq -r '.data.pid // empty' "$opened_home/instances/r3open/snapshot.json" 2>/dev/null)
    [ -n "$pid" ] || return 0
    awk '/^PPid:/ {print $2}' "/proc/$pid/status" 2>/dev/null
}

cleanup() {
    [ -z "$cleaned" ] || return 0
    cleaned=1
    if [ -z "$opened_foot" ]; then
        opened_foot=$(opened_terminal)
    fi
    close_foot "$opened_foot"
    close_foot "$hand_foot"
    end_wrapper "$opened_home" r3open
    end_wrapper "$hand_home" r3hand
}
trap cleanup EXIT

finish() {
    local word=$1
    cleanup
    echo "r3: $word hand=$hand opened=$opened"
    [ -z "$rehearse" ] || echo "r3: rehearsed — not a live reading"
    [ "$word" = green ] && exit 0
    exit 1
}

precondition() {
    echo "r3: red precondition=$1"
    [ -z "$rehearse" ] || echo "r3: rehearsed — not a live reading"
    exit 1
}

# What one focus read: `focused`, else the focus's `--json` error code, else `unread`. Live, the
# active window must be the window at `address` after a focus that exited 0; rehearsed, the fake's
# last dispatch must be the window focus on `address`.
focus_word() {
    local home=$1 name=$2 address=$3 out code active want last
    out=$(cd "$work" && started "$viola" --home "$home" focus "$name" --json 2>/dev/null)
    code=$?
    if [ "$code" -ne 0 ]; then
        jq -r '.error // "unread"' <<<"$out" 2>/dev/null || echo unread
        return
    fi
    if [ -n "$rehearse" ]; then
        want=$(jq -cn --arg a "$address" '["dispatch", "hl.dsp.focus({ window = \"address:\($a)\" })"]')
        last=$(tail -n 1 "$bin/hyprctl-asked" 2>/dev/null)
        [ -n "$address" ] && [ "$last" = "$want" ] && echo focused && return
    else
        active=$("$HYPRCTL" -j activewindow 2>/dev/null | jq -r '.address // empty' 2>/dev/null)
        [ -n "$address" ] && [ "$active" = "$address" ] && echo focused && return
    fi
    echo unread
}

# The window address of a pid in the compositor's clients, or empty.
address_of_pid() {
    "$HYPRCTL" -j clients 2>/dev/null |
        jq -r --argjson pid "$1" '[.[] | select(.pid == $pid) | .address][0] // empty' 2>/dev/null
}

# Preconditions, each read before anything starts.
if [ -e "$hand_home" ] || [ -e "$opened_home" ]; then
    precondition scratch
fi
command -v jq >/dev/null || precondition jq
mkdir -p -m 0700 "$work" "$no_claude"
if [ -n "$rehearse" ]; then
    fake=$(dirname -- "$viola")/viola-fake-agent
    [ -x "$fake" ] || precondition fake-agent
    mkdir -p -m 0700 "$bin/hyprctl-set" "$scratch/user" "$scratch/run"
    printf '{}\n' >"$scratch/user/.claude.json"
    chmod 0600 "$scratch/user/.claude.json"
    cp -- "$repo"/fixtures/hyprland/0.56.2/*.json "$bin/hyprctl-set/"
    printf 'terminal' >"$bin/hyprctl-mode"
    printf -- '--fixtures\n%s\n--trust-config\n%s\n' "$repo/fixtures/claude" \
        "$scratch/user/.claude.json" >"$bin/fake-args"
    for tool in foot hyprctl claude; do
        cp -- "$fake" "$bin/$tool"
    done
fi
[ -n "$FOOT" ] && [ -x "$FOOT" ] || precondition foot
[ -n "$HYPRCTL" ] && [ -x "$HYPRCTL" ] || precondition hyprctl
# The lock is the real compositor's; the fake answers no `locked`.
if [ -z "$rehearse" ] && [ "$("$HYPRCTL" locked 2>/dev/null)" != false ]; then
    precondition locked
fi

# R3a's program: the wrapper hands every child `--plugin-dir <dir>`, then `--model <m> --effort <e>`,
# ahead of the program's own words, and its version probe runs `<program> --version` alone. This
# stand-in answers that probe, ignores whatever else it is handed, and sleeps.
printf '%s\n' '#!/bin/sh' \
    'if [ "$#" -eq 1 ] && [ "$1" = --version ]; then' \
    '    echo "0.0.0 (r3 stand-in)"' \
    '    exit 0' \
    'fi' \
    'exec /usr/bin/sleep 900' >"$stand_in"
chmod 0700 "$stand_in"
[ "$("$stand_in" --version 2>/dev/null)" = "0.0.0 (r3 stand-in)" ] || precondition stand-in

echo "r3: build $(sha256sum -- "$viola" | cut -d' ' -f1)"

# R3a: a hand-started wrapper in its own terminal, no --origin.
(cd "$work" && started "$FOOT" --app-id=viola.r3hand --title=r3hand "--working-directory=$work" \
    "$viola" --home "$hand_home" run r3hand -- "$stand_in" </dev/null >/dev/null 2>&1) &
hand_foot=$!
waited=0
until live "$hand_home" r3hand; do
    if [ "$waited" -ge $((live_bound * 10)) ]; then
        # Never live, so no focus was fired: nothing was read.
        hand=unread
        finish red
    fi
    sleep 0.1
    waited=$((waited + 1))
done
# The subshell's pid is the terminal's parent; the terminal is the wrapper's parent.
wrapper=$(jq -r '.data.pid // empty' "$hand_home/instances/r3hand/snapshot.json" 2>/dev/null)
terminal=$(awk '/^PPid:/ {print $2}' "/proc/$wrapper/status" 2>/dev/null)
hand_foot=${terminal:-$hand_foot}
hand=$(focus_word "$hand_home" r3hand "$(address_of_pid "$hand_foot")")
[ "$hand" = focused ] || finish red

# R3b, only after a green R3a: the ONE claude start.
opened_out=$(cd "$work" && started "CLAUDE_CONFIG_DIR=$config" "$viola" --home "$opened_home" \
    open r3open --json -- claude 2>/dev/null)
open_exit=$?
opened_foot=$(opened_terminal)
address=$(jq -r '.ok.window.address // empty' <<<"$opened_out" 2>/dev/null)
if [ "$open_exit" -ne 0 ]; then
    opened=open-$(jq -r '.error // "unread"' <<<"$opened_out" 2>/dev/null || echo unread)
    finish red
fi
if [ -z "$address" ]; then
    opened=open-$(jq -r '.ok.placement // "unread"' <<<"$opened_out" 2>/dev/null || echo unread)
    finish red
fi
opened=$(focus_word "$opened_home" r3open "$address")
if [ "$hand" = focused ] && [ "$opened" = focused ]; then
    finish green
fi
finish red
