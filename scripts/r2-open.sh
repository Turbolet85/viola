#!/usr/bin/env bash
# R2: ONE `viola open` end to end beside a live parent, at the operator's host (test-only; never run
# by a gate, fired once by the operator pass). It starts the parent `parent` (a `viola run` over
# /usr/bin/sleep, no claude) in a foot window of its own, waits for it to be live, runs the one
# `viola open child --parent parent --json -- claude` (the ONE claude start), waits for the child
# wrapper's agent to exist, reads the board and the compositor's clients, closes both windows on
# every exit path and prints one verdict line.
#
#   bash scripts/r2-open.sh <scratch home> <built viola>
#
# It reads no environment variable but PATH and XDG_RUNTIME_DIR. Verdict line:
#   r2: <green|red> placement=<word> workspace=<n|none> parent_workspace=<n|none>[ first=<reading>]
#       agent=<comm|none>[ left=<n>]
# `agent` and `left` are readings: neither turns a green red.
set -u

if [ "$#" -ne 2 ]; then
    echo "usage: bash scripts/r2-open.sh <scratch home> <built viola>" >&2
    exit 2
fi
home=$(realpath -- "$1")
viola=$(realpath -- "$2")
r2=${XDG_RUNTIME_DIR:?}/viola-r2
config=$r2/claude-config
work=$r2/work
live_bound=30
gone_bound=5
agent_bound=20

placement=none
workspace=none
parent_workspace=none
agent=none
parent_foot=
child_feet=
cleaned=

# Whether a pid still holds a window in the compositor's clients.
window_of_pid() {
    hyprctl -j clients | jq -e --argjson pid "$1" 'any(.[]; .pid == $pid)' >/dev/null 2>&1
}

# The windows of app-id viola.parent or viola.child still standing.
windows_left() {
    hyprctl -j clients 2>/dev/null |
        jq '[.[] | select(.class == "viola.parent" or .class == "viola.child")] | length' 2>/dev/null ||
        echo 0
}

# Ends a foot this script started (its whole tree goes with its terminal), then waits until its
# window is gone; KILL past the bound.
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

# Whether the NUL-separated argv of /proc/<pid>/cmdline holds `--home <home> run <name>` in a row.
runs_instance() {
    local pid=$1 name=$2 args
    mapfile -d '' -t args 2>/dev/null <"/proc/$pid/cmdline" || return 1
    local i
    for ((i = 0; i + 3 < ${#args[@]}; i++)); do
        if [ "${args[i]}" = --home ] && [ "${args[i + 1]}" = "$home" ] &&
            [ "${args[i + 2]}" = run ] && [ "${args[i + 3]}" = "$name" ]; then
            return 0
        fi
    done
    return 1
}

# The child's terminal found without the open's address: the compositor's windows of app-id
# viola.child, and the parent process of every process running `--home <home> run child`.
find_child_feet() {
    local found pid parent
    found=$(hyprctl -j clients 2>/dev/null |
        jq -r '.[] | select(.class == "viola.child") | .pid // empty' 2>/dev/null)
    for pid in /proc/[0-9]*; do
        pid=${pid#/proc/}
        if runs_instance "$pid" child; then
            parent=$(awk '/^PPid:/ {print $2}' "/proc/$pid/status" 2>/dev/null)
            if [ "$(cat "/proc/$parent/comm" 2>/dev/null)" = foot ]; then
                found="$found $parent"
            fi
        fi
    done
    echo "$found" | tr ' ' '\n' | sed '/^$/d' | sort -u | tr '\n' ' '
}

# A wrapper left by a closed terminal, by the pid its own snapshot records, only while that pid
# still runs the built viola.
end_wrapper() {
    local name=$1 pid exe
    pid=$(jq -r '.data.pid // empty' "$home/instances/$name/snapshot.json" 2>/dev/null)
    [ -n "$pid" ] || return 0
    exe=$(readlink "/proc/$pid/exe" 2>/dev/null) || return 0
    case "$exe" in
        */bin/*/viola | "$viola") kill -TERM "$pid" 2>/dev/null ;;
    esac
}

cleanup() {
    [ -z "$cleaned" ] || return 0
    cleaned=1
    local foot
    if [ -z "$child_feet" ]; then
        child_feet=$(find_child_feet)
    fi
    for foot in $child_feet; do
        close_foot "$foot"
    done
    close_foot "$parent_foot"
    end_wrapper child
    end_wrapper parent
}
trap cleanup EXIT

# Closes everything, reads what still stands, prints the one line and exits: 0 green, 1 red.
finish() {
    local word=$1 first=${2:-} left
    cleanup
    left=$(windows_left)
    local line="r2: $word placement=$placement workspace=$workspace parent_workspace=$parent_workspace"
    if [ -n "$first" ]; then
        line="$line first=$first"
    fi
    line="$line agent=$agent"
    if [ "${left:-0}" -gt 0 ] 2>/dev/null; then
        line="$line left=$left"
    fi
    echo "$line"
    if [ "$word" = green ]; then
        exit 0
    fi
    exit 1
}

# The child wrapper's agent: the wrapper's pid from the child's snapshot (`.data.pid`), then its
# own child process read from /proc/<pid>/task/<pid>/children, a hyprctl read left out; its comm,
# or none within the bound.
wait_agent() {
    local waited=0 wrapper kid
    while [ "$waited" -lt $((agent_bound * 10)) ]; do
        wrapper=$(jq -r '.data.pid // empty' "$home/instances/child/snapshot.json" 2>/dev/null)
        if [ -n "$wrapper" ]; then
            for kid in $(cat "/proc/$wrapper/task/$wrapper/children" 2>/dev/null); do
                local comm
                comm=$(cat "/proc/$kid/comm" 2>/dev/null)
                if [ -n "$comm" ] && [ "$comm" != hyprctl ]; then
                    agent=$comm
                    return 0
                fi
            done
        fi
        sleep 0.1
        waited=$((waited + 1))
    done
}

for tool in foot hyprctl jq; do
    if ! command -v "$tool" >/dev/null; then
        finish red "no-$tool"
    fi
done
mkdir -p "$work"

# The parent: its own foot window, its wrapper placing itself on workspace 1 (W1 (a)).
foot --app-id=viola.parent --title=parent "--working-directory=$work" \
    "$viola" --home "$home" run parent --origin cli --place 1 -- /usr/bin/sleep 900 \
    </dev/null >/dev/null 2>&1 &
parent_foot=$!

parent_live() {
    "$viola" --home "$home" list --json 2>/dev/null |
        jq -e 'any(.ok.items[]; .name == "parent" and .liveness == "live")' >/dev/null 2>&1
}
waited=0
until parent_live; do
    if [ "$waited" -ge $((live_bound * 10)) ] || [ ! -d "/proc/$parent_foot" ]; then
        finish red parent-not-live
    fi
    sleep 0.1
    waited=$((waited + 1))
done

# The ONE claude start.
opened=$(cd "$work" && env "CLAUDE_CONFIG_DIR=$config" "$viola" --home "$home" \
    open child --parent parent --json -- claude 2>/dev/null)
open_exit=$?

wait_agent

board=$("$viola" --home "$home" list --json 2>/dev/null)
clients=$(hyprctl -j clients 2>/dev/null)

placement=$(jq -r '.ok.placement // "none"' <<<"$opened" 2>/dev/null || echo none)
address=$(jq -r '.ok.window.address // empty' <<<"$opened" 2>/dev/null)
parent_address=$(jq -r '.ok.items[] | select(.name == "parent") | .window.address // empty' \
    <<<"$board" 2>/dev/null)
child_foot=
if [ -n "$address" ]; then
    child_foot=$(jq -r --arg a "$address" '.[] | select(.address == $a) | .pid // empty' \
        <<<"$clients" 2>/dev/null)
    workspace=$(jq -r --arg a "$address" \
        '[.[] | select(.address == $a) | .workspace.id][0] // "none"' <<<"$clients" 2>/dev/null)
fi
child_feet=$child_foot
if [ -n "$parent_address" ]; then
    parent_workspace=$(jq -r --arg a "$parent_address" \
        '[.[] | select(.address == $a) | .workspace.id][0] // "none"' <<<"$clients" 2>/dev/null)
fi

first=
if [ "$open_exit" -ne 0 ]; then
    first=open-exit-$open_exit
elif [ "$placement" != placed ]; then
    first=placement
elif [ -z "$child_foot" ]; then
    first=window-not-listed
elif [ "$(cat "/proc/$child_foot/comm" 2>/dev/null)" != foot ]; then
    first=window-not-foot
elif [ "$parent_workspace" = none ]; then
    first=parent-window
elif [ "$workspace" != "$parent_workspace" ]; then
    first=workspace
fi

if [ -z "$first" ]; then
    finish green
fi
finish red "$first"
