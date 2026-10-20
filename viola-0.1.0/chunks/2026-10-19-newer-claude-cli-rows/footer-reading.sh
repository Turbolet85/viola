#!/usr/bin/env bash
# Plan step 8 of chunk 2026-10-19-newer-claude-cli-rows: the 2.1.302 footer reading, one live start
# (the founder's Fork 2 option 1, inputs#I6). Only the pane's bottom three rows are written, scrubbed.
set -euo pipefail

root="$(pwd -P)"
cdir="$HOME/.local/share/mise/installs/claude/2.1.302"
claude="$cdir/claude"
viola="$root/target/debug/viola"
evidence="$root/viola-0.1.0/chunks/2026-10-19-newer-claude-cli-rows/evidence/footer-2.1.302.md"
session=vf302
user="$(id -un)"
cwd="$root/.viola-footer-$$"
base="$root/target/e2e-home/viola-footer-$(date -u +%Y%m%dT%H%M%SZ)"
home="$base/vhome"
started=0

cleanup() {
    if [ "$started" = 1 ]; then
        tmux kill-session -t "$session" 2>/dev/null || true
    fi
    for i in $(seq 1 50); do
        left=""
        for p in $(pgrep -x claude || true); do
            [ "$(readlink "/proc/$p/cwd" 2>/dev/null || true)" = "$cwd" ] && left="$left $p"
        done
        [ -z "$left" ] && break
        if [ "$i" = 30 ]; then kill $left 2>/dev/null || true; fi
        sleep 0.1
    done
    case "$cwd" in
        "$root"/.viola-footer-*) rm -rf -- "$cwd" ;;
    esac
}
trap cleanup EXIT

scrub() {
    local s="$1" p
    for p in "$(realpath -m "$home")" "$home" "$(realpath -m "$base")" "$base" "$cwd" "$root"; do
        s="${s//"$p"/<scrubbed>}"
    done
    s="${s//"$HOME"/\~}"
    s="${s//"$user"/<user>}"
    printf '%s\n' "$s"
}

footer() {
    local label="$1" at rows hint
    at="$(date -u +%FT%TZ)"
    rows="$(tmux capture-pane -p -t "$session" -S 21 -E 23)"
    hint=no
    case "$rows" in *"for agents"*) hint=yes ;; esac
    {
        printf '\n## %s — %s\n\n```text\n' "$label" "$at"
        while IFS= read -r line; do scrub "$line"; done <<<"$rows"
        printf '```\n\n`for agents` on these rows: %s\n' "$hint"
    } >>"$evidence"
}

cargo build -q
version="$("$claude" --version)"
case "$version" in
    2.1.302*) ;;
    *) echo "footer-reading: wrong claude version" >&2; exit 1 ;;
esac

mkdir -m 0700 "$cwd"
mkdir -p "$base"
chmod 0700 "$base"

t0="$(date +%s)"
start_at="$(date -u +%FT%TZ)"
tmux new-session -d -s "$session" -x 80 -y 24 -c "$cwd" \
    env -u VIOLA_NAME -u VIOLA_DIR -u VIOLA_BIN PATH="$cdir:$PATH" "$viola" --home "$home" run m302 -- "$claude"
started=1

sleep 2
list_at="$(date -u +%FT%TZ)"
list_rc=0
list_out="$(env -u VIOLA_NAME -u VIOLA_DIR -u VIOLA_BIN PATH="$cdir:$PATH" "$viola" --home "$home" list --json 2>/dev/null)" || list_rc=$?
list_row="$(printf '%s' "$list_out" | python3 -c '
import json, sys
try:
    items = json.load(sys.stdin)["ok"]["items"]
except Exception:
    print("unreadable")
    sys.exit(0)
rows = [i for i in items if i.get("name") == "m302"]
print(" · ".join("{}={}".format(k, rows[0].get(k)) for k in ("wrapped", "liveness", "status")) if rows else "no m302 row")
')"

{
    printf '# Footer reading — claude 2.1.302 (plan step 8)\n\n'
    printf 'Written by `footer-reading.sh`. One `viola run` start of `%s` under a tmpfs `--home`, its cwd a fresh\n' "$version"
    printf '`.viola-footer-<pid>/` under the repository root, in a detached tmux pane 80×24, with no key typed\n'
    printf "(the founder's Fork 2 option 1, \`inputs#I6\`). Only the pane's bottom three rows are kept, scrubbed of the\n"
    printf 'home and the user word.\n\n'
    printf -- '- Start: %s.\n' "$start_at"
    printf -- '- `viola list --json` at ~2 s: %s · exit %s · the m302 row: %s.\n' "$list_at" "$list_rc" "$list_row"
} >"$evidence"

until_s() {
    local wait=$(($1 - ($(date +%s) - t0)))
    [ "$wait" -gt 0 ] && sleep "$wait"
    return 0
}

until_s 15
footer "~15 s"
until_s 25
footer "~25 s"

tmux kill-session -t "$session"
started=0
printf '\n- Killed: %s, with no key typed.\n' "$(date -u +%FT%TZ)" >>"$evidence"
echo "footer-reading: written"
