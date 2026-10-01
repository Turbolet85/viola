#!/usr/bin/env bash
# C1 mutation driver — one unit at a time, resumable by artifact presence (c-mutation-{unit}.json).
# Operator disk guard (2026-10-01): before each unit read C: free; < 60 GB -> -j 2; < 30 GB -> stop and report.
# cargo-mutants' copied trees go to this session's scratchpad (TMP/TEMP) so a leftover is this run's alone,
# and is removed when the unit ends (operator direction).
cd /d/dev/projects/viola || exit 9
R=.andromeda/runs/2026-10-01T09-18-50-code-audit
MT=/c/Users/turbo/AppData/Local/Temp/claude/D--dev-projects-viola/c8f1daeb-6af5-4376-85ea-48b63a994182/scratchpad/mtmp
MTW='C:\Users\turbo\AppData\Local\Temp\claude\D--dev-projects-viola\c8f1daeb-6af5-4376-85ea-48b63a994182\scratchpad\mtmp'
LOG="$R/mut-driver.log"
CAP=14400
for unit in "$@"; do
  if [ -f "$R/c-mutation-$unit.json" ]; then echo "$(date -u +%FT%TZ) $unit already summarized - skip" >> "$LOG"; continue; fi
  free=$(df -BG --output=avail /c | tail -1 | tr -dc '0-9')
  if [ "$free" -lt 30 ]; then echo "$(date -u +%FT%TZ) STOP: C: free ${free}G < 30G before $unit" >> "$LOG"; exit 3; fi
  jobs=4; [ "$free" -lt 60 ] && jobs=2
  feat=""; case "$unit" in viola|viola-e2e) feat="--features fake-agent";; esac
  out="$R/mutants-$unit"
  if [ -e "$out" ]; then out="$R/mutants-$unit-$(date -u +%H%M%S)"; fi
  mkdir -p "$MT"
  echo "$(date -u +%FT%TZ) START $unit jobs=$jobs free=${free}G out=$out" >> "$LOG"
  TMP="$MTW" TEMP="$MTW" NEXTEST_PROFILE=mutants timeout $CAP cargo mutants -p "$unit" $feat --test-tool=nextest -j $jobs --output "$out" > "$out.stdout.txt" 2> "$out.stderr.txt"
  rc=$?
  echo "$(date -u +%FT%TZ) END $unit rc=$rc" >> "$LOG"
  python -X utf8 "$R/mutsum.py" "$unit" "$out" "$rc" "$jobs" >> "$LOG" 2>&1
  left=$(ls -A "$MT" 2>/dev/null | wc -l)
  echo "$(date -u +%FT%TZ) scratch leftovers after $unit: $left" >> "$LOG"
  [ "$left" -gt 0 ] && rm -rf "$MT"/* && echo "$(date -u +%FT%TZ) scratch leftovers removed" >> "$LOG"
done
echo "$(date -u +%FT%TZ) DRIVER DONE" >> "$LOG"
