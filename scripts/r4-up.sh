#!/usr/bin/env bash
# R4: `viola up` read live at the operator's host (test-only; never run by a gate, fired by the
# operator's own entries).
#
#   bash scripts/r4-up.sh <scratch root> <built viola> windows <ws-parent> <ws-child> [--rehearse]
#   bash scripts/r4-up.sh <scratch root> <built viola> after <home> --granted
#   bash scripts/r4-up.sh <scratch root> <built viola> after --rehearse
#   bash scripts/r4-up.sh <scratch root> <built viola> plant
#
# windows (R4a): a planted fleet over the stand-in agent, no claude: a root opened to <ws-parent>,
# its child opened to <ws-child>, a hand-started root run in a foot this script spawns. Each wrapper
# is ended, its heartbeat set before this boot, then ONE `viola up` re-opens the three; each new
# window's workspace is read by its app-id, and the three are closed on every exit path.
#
# after (R4b) reads a fleet after a reboot and the operator's own `viola up`. Without --granted it
# refuses as its first act, before any root, PATH or home read. Granted, it starts, copies and types
# nothing: it reads only the board of <home> (with no claude on PATH) and each listed instance's
# own window lines under <home>/instances/<name>/events.ndjson. It needs PRESENT outside the tree
# <home> an existing directory, hyprctl and jq on PATH, the desktop unlocked; and ABSENT the scratch
# root, which holds only an empty no-claude dir and a scratch HOME for the board read.
#
# after --rehearse plants the R4a rehearsal's fleet over the stand-ins (workspaces 7 and 8), ends
# it as a reboot would, runs the ONE `viola up` itself, then makes the same reading of the scratch
# home before it closes every window it opened.
#
# plant refuses as its first act: a planted real pair needs a credential, which is never copied.
#
# --rehearse runs over the stand-in foot, hyprctl and agent (workspaces inside the fakes only), with
# every viola process's PATH the scratch bin alone, so nothing it starts can reach a compositor or a
# claude. The compositor reading is the instance's own window record there, the fake listing no
# app-id.
#
# It strips every VIOLA_* name from its own environment first, reads no other variable but PATH and
# HOME, and every board read runs with no claude on its PATH. A verdict line holds words and counts
# only: no name, address, pid, workspace number or path.
#
# Verdict lines:
#   r4: <green|red> phase=windows order=<parents-first|child-first|unread> placed=<n>/2 lines=<matched|mismatched>
#   r4: <green|red> phase=after live=<n> placed=<p>/<w> order=<parents-first|child-first|unread> gone=<g>
#   r4: red precondition=<word>
#   r4: rehearsed — not a live reading        (after the verdict, under --rehearse)
set -u

case "${3:-}" in
    plant)
        echo "r4: red precondition=r4b-not-granted"
        exit 1
        ;;
esac

rehearse=
granted=
words=()
for word in "$@"; do
    case $word in
        --rehearse) rehearse=1 ;;
        --granted) granted=1 ;;
        *) words+=("$word") ;;
    esac
done
phase=${words[2]:-}
if [ "$phase" = after ] && [ -z "$rehearse" ] && [ -z "$granted" ]; then
    echo "r4: red precondition=r4b-not-granted"
    exit 1
fi
usage() {
    echo "usage: bash scripts/r4-up.sh <scratch root> <built viola> windows <ws-parent> <ws-child> [--rehearse]" >&2
    echo "       bash scripts/r4-up.sh <scratch root> <built viola> after <home> --granted | after --rehearse" >&2
    exit 2
}
case "$phase" in
    windows)
        [ "${#words[@]}" -eq 5 ] && [ -z "$granted" ] || usage
        ;;
    after)
        if [ -n "$rehearse" ]; then
            [ "${#words[@]}" -eq 3 ] && [ -z "$granted" ] || usage
        else
            [ "${#words[@]}" -eq 4 ] || usage
        fi
        ;;
    *) usage ;;
esac
for name in $(compgen -e); do
    case $name in
        VIOLA_*) unset "$name" ;;
    esac
done
root=$(realpath -m -- "${words[0]}")
viola=$(realpath -- "${words[1]}")
if [ "$phase" = windows ]; then
    ws_parent=${words[3]}
    ws_child=${words[4]}
else
    ws_parent=7
    ws_child=8
fi
case "$ws_parent$ws_child" in
    *[!0-9-]* | '')
        echo "usage: the two workspaces are integers" >&2
        exit 2
        ;;
esac
repo=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." && pwd)
export LC_ALL=C

stamp=$(date -u +%Y%m%dt%H%M%S)
parent=r4p$stamp
child=r4c$stamp
hand=r4h$stamp
home=$root/vhome
bin=$root/bin
no_claude=$root/no-claude
work=$root/work
user=$root/user
live_bound=30
gone_bound=5
session_bound=20

if [ -n "$rehearse" ]; then
    agent=$(dirname -- "$viola")/viola-fake-agent
    hyprctl=$bin/hyprctl
    foot=$bin/foot
    children_path=$bin
else
    agent=$repo/target/r4-agent/debug/viola-fake-agent
    hyprctl=hyprctl
    foot=foot
    children_path=$bin:/usr/bin:/bin
fi

order=unread
placed=0
lines=mismatched
cleaned=
fleet=
after_live=0
after_placed=0
after_with=0
after_order=unread
after_gone=0

# The verdict on a failed precondition: nothing of the fleet was started.
refuse() {
    echo "r4: red precondition=$1"
    if [ -n "$rehearse" ]; then
        echo "r4: rehearsed — not a live reading"
    fi
    exit 1
}

# One viola process with the children's environment: PATH the scratch bin first, HOME the scratch
# user, no VIOLA_* name.
v() {
    env "PATH=$children_path" "HOME=$user" "$viola" --home "$home" "$@"
}

# The board, read with no claude on PATH.
board_json() {
    env "PATH=$no_claude" "HOME=$user" "$viola" --home "$home" list --json 2>/dev/null
}

# R4b's reading of <home>, shared by both after arms; it reads and starts nothing else.
# live: the board's live items. w: the live instances whose window line before their newest one
# recorded a workspace; p: those of them whose newest window line reads placed on that same
# workspace. order: over each live child whose recorded parent is live, parents-first when every
# parent's newest window line is earlier than its child's, child-first when one is not, unread with
# no such edge. gone: the board's items not live.
read_after() {
    local h=$1 board name reading was_set was_placed newest parent_name kid
    local -A newest_of=()
    board=$(board_json) || board=
    after_live=0
    after_placed=0
    after_with=0
    after_order=unread
    after_gone=$(jq -r '[.ok.items[]? | select(.liveness != "live")] | length' <<<"$board" 2>/dev/null)
    case "$after_gone" in '' | *[!0-9]*) after_gone=0 ;; esac
    while IFS= read -r name; do
        case "$name" in '' | */* | .*) continue ;; esac
        after_live=$((after_live + 1))
        reading=$(jq -n -r -R '
            [inputs | fromjson? | select(type == "object" and .kind == "window")] as $w
            | ($w | length) as $n
            | if $n == 0 then "0 0 -"
              else $w[$n - 1] as $new
              | (if $n >= 2 then ($w[$n - 2].data.workspace // null) else null end) as $was
              | "\(if $was != null then 1 else 0 end) \(if $was != null and $new.data.placement == "placed" and $new.data.workspace == $was then 1 else 0 end) \($new.ts // "-")"
              end' "$h/instances/$name/events.ndjson" 2>/dev/null) || reading="0 0 -"
        read -r was_set was_placed newest <<<"$reading"
        [ "$was_set" = 1 ] && after_with=$((after_with + 1))
        [ "$was_placed" = 1 ] && after_placed=$((after_placed + 1))
        newest_of[$name]=${newest:--}
    done < <(jq -r '.ok.items[]? | select(.liveness == "live") | .name' <<<"$board" 2>/dev/null)
    while read -r parent_name kid; do
        [ -n "$kid" ] || continue
        if [ "${newest_of[$parent_name]:--}" != - ] && [ "${newest_of[$kid]:--}" != - ] &&
            [[ "${newest_of[$parent_name]}" < "${newest_of[$kid]}" ]]; then
            [ "$after_order" = child-first ] || after_order=parents-first
        else
            after_order=child-first
        fi
    done < <(jq -r '(.ok.items // []) as $i
        | [$i[] | select(.liveness == "live") | .name] as $l
        | $i[] | select(.liveness == "live" and (.parent | type) == "string")
        | select(.parent as $p | any($l[]; . == $p))
        | .parent + " " + .name' <<<"$board" 2>/dev/null)
}

# The after verdict, from the last reading.
after_verdict() {
    local word=red
    if [ "$after_live" -ge 1 ] && [ "$after_placed" -eq "$after_with" ] &&
        [ "$after_order" = parents-first ]; then
        word=green
    fi
    echo "r4: $word phase=after live=$after_live placed=$after_placed/$after_with order=$after_order gone=$after_gone"
    [ "$word" = green ]
}

# The live after arm: preconditions in order, each failure reading and starting nothing; then the
# one reading of the operator's home.
if [ "$phase" = after ] && [ -z "$rehearse" ]; then
    [ ! -e "$root" ] || refuse root-present
    command -v hyprctl >/dev/null || refuse no-hyprctl
    [ "$(hyprctl locked 2>/dev/null | tr -d '[:space:]')" = false ] || refuse locked
    command -v jq >/dev/null || refuse no-jq
    [ -d "${words[3]}" ] || refuse no-home
    home=$(realpath -- "${words[3]}")
    mkdir -p -m 0700 "$root" "$no_claude" "$user" || refuse root-present
    read_after "$home"
    after_verdict
    exit $?
fi

# The pid a name's snapshot records, while that pid still runs this build or its pinned copy.
wrapper_of() {
    local pid exe
    pid=$(jq -r '.data.pid // empty' "$home/instances/$1/snapshot.json" 2>/dev/null)
    [ -n "$pid" ] || return 1
    exe=$(readlink "/proc/$pid/exe" 2>/dev/null) || return 1
    exe=${exe% (deleted)}
    case "$exe" in
        "$home"/bin/*/viola | "$viola") echo "$pid" ;;
        *) return 1 ;;
    esac
}

# The pids of the terminals of app-id viola.<name> the compositor lists (the fake lists none).
terminals_of() {
    "$hyprctl" -j clients 2>/dev/null |
        jq -r --arg c "viola.$1" '.[] | select(.class == $c) | .pid // empty' 2>/dev/null
}

# Ends a pid and waits for it to be gone; KILL past the bound.
end_pid() {
    local pid=$1 waited=0
    [ -d "/proc/$pid" ] || return 0
    kill -TERM "$pid" 2>/dev/null
    while [ -d "/proc/$pid" ] && [ "$waited" -lt $((gone_bound * 10)) ]; do
        sleep 0.1
        waited=$((waited + 1))
    done
    if [ -d "/proc/$pid" ]; then
        kill -KILL "$pid" 2>/dev/null
    fi
}

# Ends the window of <name>: its wrapper by the pid its snapshot records, else its terminal found
# by its app-id.
close_instance() {
    local pid
    if pid=$(wrapper_of "$1"); then
        end_pid "$pid"
        return 0
    fi
    for pid in $(terminals_of "$1"); do
        if [ "$(cat "/proc/$pid/comm" 2>/dev/null)" = foot ]; then
            end_pid "$pid"
        fi
    done
}

cleanup() {
    [ -n "$fleet" ] || return 0
    [ -z "$cleaned" ] || return 0
    cleaned=1
    local name pid
    for name in "$child" "$parent" "$hand"; do
        close_instance "$name"
    done
    if [ -n "$rehearse" ] && [ -f "$bin/foot-spawns.ndjson" ]; then
        for pid in $(jq -r '.pid' "$bin/foot-spawns.ndjson" 2>/dev/null); do
            if [ "$(cat "/proc/$pid/comm" 2>/dev/null)" = foot ]; then
                end_pid "$pid"
            fi
        done
    fi
}
trap cleanup EXIT

finish() {
    local word=red
    cleanup
    if [ "$phase" = after ]; then
        after_verdict
        word=$?
        echo "r4: rehearsed — not a live reading"
        exit "$word"
    fi
    if [ "$order" = parents-first ] && [ "$placed" -eq 2 ] && [ "$lines" = matched ]; then
        word=green
    fi
    echo "r4: $word phase=windows order=$order placed=$placed/2 lines=$lines"
    if [ -n "$rehearse" ]; then
        echo "r4: rehearsed — not a live reading"
    fi
    if [ "$word" = green ]; then
        exit 0
    fi
    exit 1
}

# Whether <name>'s log holds a session-start, within the bound.
session_logged() {
    local waited=0
    until jq -e -s 'any(.[]; .kind == "session-start")' "$home/instances/$1/events.ndjson" \
        >/dev/null 2>&1; do
        [ "$waited" -lt $((session_bound * 10)) ] || return 1
        sleep 0.1
        waited=$((waited + 1))
    done
}

# Whether <name> reads live on the board, within the bound.
board_live() {
    local waited=0
    until board_json | jq -e --arg n "$1" 'any(.ok.items[]; .name == $n and .liveness == "live")' \
        >/dev/null 2>&1; do
        [ "$waited" -lt $((live_bound * 10)) ] || return 1
        sleep 0.1
        waited=$((waited + 1))
    done
}

# Ends <name>'s wrapper as a host going down would, and waits until it is gone.
end_wrapper() {
    local pid
    pid=$(wrapper_of "$1") || return 1
    end_pid "$pid"
}

# The workspace <name>'s window stands on: the compositor's, by app-id; under --rehearse the
# instance's own window record.
workspace_of() {
    if [ -n "$rehearse" ]; then
        jq -r '.data.window.workspace // "none"' "$home/instances/$1/snapshot.json" 2>/dev/null ||
            echo none
        return 0
    fi
    "$hyprctl" -j clients 2>/dev/null |
        jq -r --arg c "viola.$1" '[.[] | select(.class == $c) | .workspace.id][0] // "none"' \
            2>/dev/null || echo none
}

# Preconditions, in order; each failure starts nothing.
[ ! -e "$root" ] || refuse root-present
mkdir -p -m 0700 "$root" || refuse root-present
if [ -z "$rehearse" ] && git -C "$root" rev-parse --git-dir >/dev/null 2>&1; then
    refuse root-in-repository
fi
mkdir -p -m 0700 "$bin" "$no_claude" "$user" "$work/$parent" "$work/$child" "$work/$hand"
if [ -n "$rehearse" ]; then
    [ -x "$agent" ] || refuse stand-in-agent
    for name in foot hyprctl; do
        cp -- "$agent" "$bin/$name" && chmod 0700 "$bin/$name" || refuse "no-$name"
    done
    mkdir -p -m 0700 "$bin/hyprctl-set"
    cp -- "$repo"/fixtures/hyprland/0.56.2/*.json "$bin/hyprctl-set/" || refuse no-hyprctl
    for ws in "$ws_parent" "$ws_child"; do
        sed "s/workspace = 1,/workspace = $ws,/" \
            "$repo/fixtures/hyprland/0.56.2/MoveToWorkspace.recorded-1.json" \
            >"$bin/hyprctl-set/MoveToWorkspace.rehearse-$ws.json"
    done
    echo terminal >"$bin/hyprctl-mode"
else
    command -v foot >/dev/null || refuse no-foot
    command -v hyprctl >/dev/null || refuse no-hyprctl
    [ "$(hyprctl locked 2>/dev/null | tr -d '[:space:]')" = false ] || refuse locked
fi
for tool in jq sha256sum; do
    command -v "$tool" >/dev/null || refuse "no-$tool"
done
[ -x "$agent" ] || refuse stand-in-agent
cp -- "$agent" "$bin/claude" && chmod 0700 "$bin/claude" || refuse stand-in-agent
printf '%s\n' --fixtures "$repo/fixtures/claude" --screens --trusted-root "$work" >"$bin/fake-args"
# Every stand-in session holds the one recorded session id: a malformed agents answer (the stand-in's
# own mode) keeps a revive from reading another revived instance as that session open elsewhere.
echo malformed >"$bin/agents-mode"
"$bin/claude" --version 2>/dev/null | grep -q . || refuse stand-in-agent
found=$(env "PATH=$children_path" "$BASH" -c 'command -v claude')
if [ "$found" != "$bin/claude" ] || [ -e /usr/bin/claude ] || [ -e /bin/claude ]; then
    refuse real-claude-reachable
fi
echo "r4: build=$(sha256sum -- "$viola" | cut -c1-16)"
fleet=1

# The fleet: a root and its child opened to their workspaces, a root started by hand.
(cd "$work" && v open "$parent" --cwd "$work/$parent" --workspace "$ws_parent" -- claude) \
    >/dev/null 2>&1 || finish
(cd "$work" && v open "$child" --parent "$parent" --cwd "$work/$child" --workspace "$ws_child" \
    -- claude) >/dev/null 2>&1 || finish
env "PATH=$children_path" "HOME=$user" "$foot" "--app-id=viola.$hand" "--title=$hand" \
    "--working-directory=$work/$hand" "$viola" --home "$home" run "$hand" -- claude \
    </dev/null >/dev/null 2>&1 &
board_live "$hand" || finish
for name in "$parent" "$child" "$hand"; do
    session_logged "$name" || finish
    echo "r4 hello" | timeout 30 env "PATH=$children_path" "HOME=$user" "$viola" --home "$home" \
        send "$name" >/dev/null 2>&1
done

# The reboot's leavings: every wrapper ended with no clean exit, each beat set before this boot.
btime=$(awk '/^btime / {print $2}' /proc/stat)
for name in "$parent" "$child" "$hand"; do
    end_wrapper "$name" || finish
    touch -d "@$((btime - 60))" "$home/instances/$name/heartbeat" || finish
done

# The ONE up.
opened=$(cd "$root" && v up --json 2>/dev/null)
up_exit=$?

if [ "$phase" = after ]; then
    read_after "$home"
    finish
fi

mapfile -t live < <(jq -r '.ok.instances[] | select(.outcome == "live") | .name' <<<"$opened" 2>/dev/null)
at_parent=-1
at_child=-1
for i in "${!live[@]}"; do
    [ "${live[i]}" = "$parent" ] && at_parent=$i
    [ "${live[i]}" = "$child" ] && at_child=$i
done
if [ "$at_parent" -ge 0 ] && [ "$at_child" -ge 0 ]; then
    if [ "$at_parent" -lt "$at_child" ]; then
        order=parents-first
    else
        order=child-first
    fi
fi
for pair in "$parent:$ws_parent" "$child:$ws_child"; do
    name=${pair%%:*}
    ws=${pair#*:}
    word=$(jq -r --arg n "$name" '.ok.instances[] | select(.name == $n) | .placement // "none"' \
        <<<"$opened" 2>/dev/null)
    if [ "$word" = placed ] && [ "$(workspace_of "$name")" = "$ws" ]; then
        placed=$((placed + 1))
    fi
done
outcomes=$(jq -r '[.ok.instances[] | .name + "=" + .outcome] | sort | join(" ")' <<<"$opened" 2>/dev/null)
want=$(printf '%s\n' "$child=live" "$hand=live" "$parent=live" | sort | paste -sd ' ')
if [ "$up_exit" -eq 0 ] && [ "$outcomes" = "$want" ]; then
    lines=matched
fi
finish
