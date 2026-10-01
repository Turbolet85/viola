#!/usr/bin/env bash
# C1 viola-e2e re-invocation: the per-unit form (-p viola-e2e) never builds the ROOT package's bins in the copied
# tree, and the harness tests spawn target/debug/viola-fake-agent.exe (baseline.log: os error 2). Testing the root
# package too makes cargo build its bins (fake-agent feature) beside viola-e2e's tests. Same cap and disk guard.
cd /d/dev/projects/viola || exit 9
R=.andromeda/runs/2026-10-01T09-18-50-code-audit
MTW='C:\Users\turbo\AppData\Local\Temp\claude\D--dev-projects-viola\c8f1daeb-6af5-4376-85ea-48b63a994182\scratchpad\mtmp'
LOG="$R/mut-driver.log"
unit=viola-e2e
free=$(df -BG --output=avail /c | tail -1 | tr -dc '0-9')
if [ "$free" -lt 30 ]; then echo "$(date -u +%FT%TZ) STOP: C: free ${free}G < 30G before $unit (re-invocation)" >> "$LOG"; exit 3; fi
jobs=4; [ "$free" -lt 60 ] && jobs=2
out="$R/mutants-viola-e2e-c"
echo "$(date -u +%FT%TZ) START $unit (re-invocation, --test-workspace=true) jobs=$jobs free=${free}G out=$out" >> "$LOG"
TMP="$MTW" TEMP="$MTW" NEXTEST_PROFILE=mutants timeout 14400 cargo mutants -p viola-e2e --features fake-agent --test-workspace=true --test-tool=nextest -j $jobs --output "$out" > "$out.stdout.txt" 2> "$out.stderr.txt"
rc=$?
echo "$(date -u +%FT%TZ) END $unit rc=$rc" >> "$LOG"
python -X utf8 "$R/mutsum.py" "$unit" "$out" "$rc" "$jobs" >> "$LOG" 2>&1
echo "$(date -u +%FT%TZ) E2E DONE" >> "$LOG"
