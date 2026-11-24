#!/usr/bin/env bash
# R5: `viola adopt` read live at the operator's host (test-only; never run by a gate, fired by the
# operator's own entries on his word), and its rehearsal over the fake agent.
#
#   bash scripts/r5-adopt.sh <base> <viola> <home> a <id> <model> <effort>   R5a: adopt a real ended plain session
#   bash scripts/r5-adopt.sh <base> <viola> <home> b <id>                    R5b: a real live one is refused
#   bash scripts/r5-adopt.sh <base> <viola> --rehearse                       both arms over the fake agent
#
# <base> must be absent; the script makes it and names it on stdout, removing nothing. <id> must
# have the session-id shape. Every inherited VIOLA_* name is unset first.
#
# `adopt` holds the terminal it is typed in, so ONE function, host_adopt, hosts it in a pseudo-
# terminal of the script's own (util-linux `script`), its input a FIFO the script holds open and
# never writes to: no key reaches it. ONE close_host, run by the trap on EXIT, INT and TERM and by
# the normal end alike, ends only the pids this run recorded: the adopted wrapper (by its snapshot's
# pid, else the one child of the recorded `script` pid, each acted on only while its exe is
# <viola>) and the `script` it started, TERM then KILL past 10 s while the start time is unchanged.
#
# --rehearse puts a fake-only PATH first (<base>/bin holding the fake agent as claude, foot and
# hyprctl, then /usr/bin:/bin), stamps its own home with `viola verify` against that fake under a
# scratch user home and runtime dir, and runs both arms through the same host_adopt and close_host.
#
# Verdict lines:
#   r5a: <green|red> resumed=<yes|no> lineage=<yes|no> pair=<model>/<effort>|unread turn=<ended|none|unfired> ended=<yes|no>
#   r5b: <green|red> refused=<session-live|<code>|none> started=<none|child> ended=<yes|no>
#   r5: rehearsed — not a live reading        (last, under --rehearse)
set -u

usage() {
    echo "usage: bash scripts/r5-adopt.sh <base> <viola> <home> a <id> <model> <effort>" >&2
    echo "       bash scripts/r5-adopt.sh <base> <viola> <home> b <id>" >&2
    echo "       bash scripts/r5-adopt.sh <base> <viola> --rehearse" >&2
    exit 2
}

precondition() {
    echo "r5: precondition $1" >&2
    exit 2
}

id_shaped() {
    printf '%s\n' "$1" | grep -Exq '[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}'
}

for name in $(compgen -e); do
    case $name in
        VIOLA_*) unset "$name" ;;
    esac
done
export LC_ALL=C

[ "$#" -ge 3 ] || usage
base_given=$1
viola_given=$2
rehearse=
arm=
id=
model=
effort=
if [ "$3" = --rehearse ]; then
    [ "$#" -eq 3 ] || usage
    rehearse=1
else
    [ "$#" -ge 5 ] || usage
    home=$3
    arm=$4
    id=$5
    case "$arm" in
        a)
            [ "$#" -eq 7 ] || usage
            model=$6
            effort=$7
            ;;
        b) [ "$#" -eq 5 ] || usage ;;
        *) usage ;;
    esac
    id_shaped "$id" || usage
fi

[ ! -e "$base_given" ] || precondition base-present
for tool in jq script mkfifo; do
    command -v "$tool" >/dev/null || precondition "no-$tool"
done
viola=$(realpath -- "$viola_given") || precondition no-viola
[ -x "$viola" ] || precondition no-viola
mkdir -p -m 0700 "$base_given" || precondition base-present
base=$(realpath -- "$base_given")
repo=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." && pwd)
no_claude=$base/no-claude
mkdir -m 0700 "$no_claude"
echo "r5: base=$base"

if [ -n "$rehearse" ]; then
    home=$base/home
    bin=$base/bin
    user=$base/user
    runtime=$base/run
    children_path=$bin:/usr/bin:/bin
    agent=$(dirname -- "$viola")/viola-fake-agent
    [ -x "$agent" ] || precondition stand-in-agent
    mkdir -m 0700 "$bin" "$user" "$runtime" "$base/verify-cwd"
    for name in claude foot hyprctl; do
        cp -- "$agent" "$bin/$name" && chmod 0700 "$bin/$name" || precondition stand-in-agent
    done
    printf '{}\n' >"$user/.claude.json"
    chmod 0600 "$user/.claude.json"
    found=$(env "PATH=$children_path" "$BASH" -c 'command -v claude; command -v hyprctl; command -v foot')
    [ "$found" = "$bin/claude"$'\n'"$bin/hyprctl"$'\n'"$bin/foot" ] || precondition real-tool-reachable
    # The stamp, as the harness boot takes it: the fake agent's flags after the dashes.
    (cd "$base/verify-cwd" && env "PATH=$children_path" "HOME=$user" "XDG_RUNTIME_DIR=$runtime" \
        "$viola" --home "$home" verify -- "$bin/claude" --cli-version 2.1.328 \
        --fixtures "$repo/fixtures/claude" --screens --turn-stop --dialogs --framing --transcript \
        --trusted-root "$base/verify-cwd" --trust-config "$user/.claude.json") \
        </dev/null >/dev/null 2>&1 || precondition stamp-failed
    # The adopted child's own flags: adopt hands it none of the caller's words.
    printf '%s\n' --cli-version 2.1.328 --fixtures "$repo/fixtures/claude" --screens --turn-stop \
        --trusted-root "$base/typed-in" >"$bin/fake-args"
    run_env=("PATH=$children_path" "HOME=$user" "XDG_RUNTIME_DIR=$runtime")
else
    command -v claude >/dev/null || precondition no-claude
    run_env=("PATH=$PATH")
fi

typed_in=$base/typed-in
mkdir -m 0700 "$typed_in"
fifo=$base/pty-in
script_pid=
script_start=
wrapper_pid=
wrapper_start=
instance=

# The OS start time of a pid (field 22 of its stat), empty when it is gone.
start_of() {
    local stat
    stat=$(cat "/proc/$1/stat" 2>/dev/null) || return 0
    stat=${stat##*) }
    # shellcheck disable=SC2086
    set -- $stat
    echo "${20}"
}

is_viola() {
    local exe
    exe=$(readlink "/proc/$1/exe" 2>/dev/null) || return 1
    [ "${exe% (deleted)}" = "$viola" ]
}

# Hosts `viola adopt <args…>` in a pseudo-terminal of its own, typed in <typed_in>.
host_adopt() {
    local cmd
    cmd=$(printf '%q ' env "${run_env[@]}" "$viola" --home "$home" adopt "$@")
    mkfifo -m 0600 "$fifo" || return 1
    exec 3<>"$fifo"
    (cd "$typed_in" && SHELL=/bin/sh exec script -qfec "$cmd" /dev/null <"$fifo" >/dev/null 2>&1) &
    script_pid=$!
    script_start=$(start_of "$script_pid")
}

# The adopted wrapper's pid: its snapshot's, else the one child of `script` running <viola>.
record_wrapper() {
    local pid child
    [ -z "$wrapper_pid" ] || return 0
    pid=$(jq -r '.data.pid // empty' "$home/instances/$instance/snapshot.json" 2>/dev/null)
    if [ -z "$pid" ] && [ -n "$script_pid" ]; then
        for child in $(cat "/proc/$script_pid/task/$script_pid/children" 2>/dev/null); do
            is_viola "$child" && pid=$child
        done
    fi
    if [ -n "$pid" ] && is_viola "$pid"; then
        wrapper_pid=$pid
        wrapper_start=$(start_of "$pid")
    fi
}

# TERM to a recorded pid, its wait up to 10 s, then KILL while it is still the same process.
end_recorded() {
    local pid=$1 started=$2 waited=0
    [ -n "$pid" ] && [ -n "$started" ] || return 0
    [ "$(start_of "$pid")" = "$started" ] || return 0
    kill -TERM "$pid" 2>/dev/null
    while [ "$(start_of "$pid")" = "$started" ] && [ "$waited" -lt 100 ]; do
        sleep 0.1
        waited=$((waited + 1))
    done
    if [ "$(start_of "$pid")" = "$started" ]; then
        kill -KILL "$pid" 2>/dev/null
    fi
}

closed=
close_host() {
    [ -z "$closed" ] || return 0
    closed=1
    record_wrapper
    if [ -n "$wrapper_pid" ] && is_viola "$wrapper_pid"; then
        end_recorded "$wrapper_pid" "$wrapper_start"
    fi
    end_recorded "$script_pid" "$script_start"
    if [ -n "$script_pid" ]; then
        wait "$script_pid" 2>/dev/null
    fi
    exec 3>&-
    rm -f -- "$fifo"
}
trap close_host EXIT
trap 'close_host; exit 130' INT
trap 'close_host; exit 143' TERM

# Whether both recorded pids are gone.
ended_word() {
    if { [ -z "$wrapper_start" ] || [ "$(start_of "$wrapper_pid")" != "$wrapper_start" ]; } &&
        { [ -z "$script_start" ] || [ "$(start_of "$script_pid")" != "$script_start" ]; }; then
        echo yes
    else
        echo no
    fi
}

v() {
    env "PATH=$no_claude" "${run_env[@]:1}" "$viola" --home "$home" "$@"
}

# R5a: adopt <id> as adopt-probe under r5-parent, read the start, drive one turn.
arm_a() {
    local id=$1 model=$2 effort=$3 waited=0 line lines sent cursor woke
    local resumed=no lineage=no pair=unread turn=unfired ended word=red
    instance=adopt-probe
    closed=
    wrapper_pid=
    wrapper_start=
    host_adopt "$id" --name adopt-probe --parent r5-parent --model "$model" --effort "$effort"
    lines=$home/instances/adopt-probe/events.ndjson
    until jq -e -s 'any(.[]; .kind == "session-start")' "$lines" >/dev/null 2>&1; do
        [ "$waited" -lt 1100 ] || break
        [ "$(start_of "$script_pid")" = "$script_start" ] || break
        sleep 0.1
        waited=$((waited + 1))
    done
    record_wrapper
    line=$(jq -c -s '[.[] | select(.kind == "session-start")][0] // empty' "$lines" 2>/dev/null)
    if [ -n "$line" ] && [ "$(jq -r '.data.cause' <<<"$line")" = resume ] &&
        [ "$(jq -r '.data.agent_session_id' <<<"$line")" = "$id" ]; then
        resumed=yes
    fi
    if [ "$(jq -r '.data.lineage.parent // empty' "$home/instances/adopt-probe/snapshot.json" 2>/dev/null)" = r5-parent ]; then
        lineage=yes
    fi
    local m e
    m=$(jq -r '.data.launch.model // empty' "$home/instances/adopt-probe/snapshot.json" 2>/dev/null)
    e=$(jq -r '.data.launch.effort // empty' "$home/instances/adopt-probe/snapshot.json" 2>/dev/null)
    if [ -n "$m" ] && [ -n "$e" ]; then
        pair=$m/$e
    fi
    if [ "$resumed" = yes ]; then
        sent=$(printf '%s' 'Reply with the single word: adopted' | v send adopt-probe --json 2>/dev/null)
        cursor=$(jq -r '.ok.cursor // empty' <<<"$sent" 2>/dev/null)
        if [ -n "$cursor" ]; then
            woke=$(v wait adopt-probe --after "$cursor" --timeout-ms 300000 --json 2>/dev/null)
            if [ "$(jq -r '.ok.event.kind // empty' <<<"$woke" 2>/dev/null)" = turn-ended ]; then
                turn=ended
            else
                turn=none
            fi
        fi
    fi
    close_host
    ended=$(ended_word)
    if [ "$resumed" = yes ] && [ "$lineage" = yes ] && [ "$pair" = "$model/$effort" ] &&
        [ "$turn" = ended ] && [ "$ended" = yes ]; then
        word=green
    fi
    echo "r5a: $word resumed=$resumed lineage=$lineage pair=$pair turn=$turn ended=$ended"
    [ "$word" = green ]
}

# R5b: adopt <id> as adopt-live under --json; a live session is refused and nothing starts.
arm_b() {
    local id=$1 code refused=none started=none ended word=red
    instance=adopt-live
    closed=
    wrapper_pid=
    wrapper_start=
    host_adopt "$id" --name adopt-live --json
    # A refused adopt ends by itself (exit 1 through `script -e`).
    wait "$script_pid" 2>/dev/null
    code=$?
    record_wrapper
    refused=$(jq -r -s '[.[] | select(.event == "process-exit" and .subject == "self") | .detail // empty] | last // empty' \
        "$home/diagnostics/run-adopt-live.ndjson" 2>/dev/null)
    [ -n "$refused" ] || refused=none
    if [ -n "$(jq -r '.data.child_pid // empty' "$home/instances/adopt-live/snapshot.json" 2>/dev/null)" ] ||
        jq -e -s 'any(.[]; .event == "process-start" and .subject == "claude-child")' \
            "$home/diagnostics/run-adopt-live.ndjson" >/dev/null 2>&1; then
        started=child
    fi
    close_host
    ended=$(ended_word)
    if [ "$code" -eq 1 ] && [ "$refused" = session-live ] && [ "$started" = none ] && [ "$ended" = yes ]; then
        word=green
    fi
    echo "r5b: $word refused=$refused started=$started ended=$ended"
    [ "$word" = green ]
}

if [ -n "$rehearse" ]; then
    id_a=$(cat /proc/sys/kernel/random/uuid)
    id_b=$(cat /proc/sys/kernel/random/uuid)
    jq -n --arg id "$id_b" '[{"cwd": "~/elsewhere", "kind": "interactive", "name": "another session",
        "pid": 4294967295, "sessionId": $id, "startedAt": 1760000000000, "status": "idle"}]' \
        >"$bin/agents-rows.json"
    status=0
    arm_a "$id_a" haiku low || status=1
    arm_b "$id_b" || status=1
    echo "r5: rehearsed — not a live reading"
    exit "$status"
fi

case "$arm" in
    a) arm_a "$id" "$model" "$effort" ;;
    b) arm_b "$id" ;;
esac
