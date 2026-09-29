# Operator pass — the `leg = 'operator'` entries, as fired (plan Stage B)

Operator's word, 2026-09-29: "run the operator pass now, entries 14-18, with the fixed count (3 R/K losses or 3
pushes) and ci.yml restored byte for byte." Each entry below is the plan's exact `run`, its exit and its atoms.

## Before the pre-CI commit
- **Entry 14** `python -X utf8 C:/Users/turbo/.claude/skills/andromeda-phase/../andromeda-tools/scripts/gate.py hygiene`
  — exit 0 · `hygiene: clean — read 30 (runs 26 · evidence 4) · trails 11 not read · binary 0 not read by P1` ·
  atoms `exit 0` ✓ `contains hygiene: clean` ✓.
- **Entry 8 re-run** (its note: before every push) `bash scripts/agent-run.sh pre-push` — exit 0 · `"ok":true` ·
  linux coverage 919 passed 0 failed, browser 1 passed, gate no breaches · windows coverage 934 passed 0 failed, gate
  no breaches.
