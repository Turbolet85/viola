#!/usr/bin/env bash
# P5 check 4 (9): baseline every new, non-leg entry on the untouched tree; one log per entry.
cd /d/dev/projects/viola || exit 9
D=.andromeda/runs/2026-09-26T21-29-01-phase/p5
b() { local n=$1; shift; bash -o pipefail -c "$1" > "$D/$n.log" 2>&1; echo "exit=$?" >> "$D/$n.log"; printf '%s: %s | last: %s\n' "$n" "$(tail -1 "$D/$n.log")" "$(tail -2 "$D/$n.log" | head -1 | cut -c1-160)"; }
b 04 "bash scripts/agent-run.sh run --unit --filter 'test(/pre_push_|chunk_base_of_an_uncommitted_promotion/)'"
b 07 "MSYS2_ARG_CONV_EXCL=* wsl.exe -d Ubuntu --exec /usr/bin/cc --version"
b 08 "MSYS2_ARG_CONV_EXCL=* wsl.exe -d Ubuntu --cd /mnt/d/dev/projects/viola --exec /usr/bin/bash scripts/wsl-provision.sh"
b 09 "MSYS2_ARG_CONV_EXCL=* wsl.exe -d Ubuntu --cd /mnt/d/dev/projects/viola --exec /usr/bin/bash scripts/wsl-provision.sh --probe"
b 10 "CLAUDE_CODE_MESSAGING_TOKEN=canary-prepush-7f3a MSYS2_ARG_CONV_EXCL=* wsl.exe -d Ubuntu --exec /usr/bin/env | grep -c canary-prepush-7f3a"
b 10c "WSLENV=CLAUDE_CODE_MESSAGING_TOKEN CLAUDE_CODE_MESSAGING_TOKEN=canary-prepush-7f3a MSYS2_ARG_CONV_EXCL=* wsl.exe -d Ubuntu --exec /usr/bin/env | grep -c canary-prepush-7f3a"
b 11 "bash scripts/agent-run.sh pre-push"
b 12 "grep -c -E '/home/|/mnt/|\"[A-Za-z]:' target/agent-run/artifacts/mutants-verdict-ubuntu-latest.json"
b 13 "grep -c pre-push scripts/agent-run.ps1"
