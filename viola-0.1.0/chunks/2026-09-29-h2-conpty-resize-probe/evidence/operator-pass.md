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

## Pre-CI commit `3d04d1c` (plan step 9)
- **Entry 15** `git diff --quiet && git diff --cached --quiet && git push origin HEAD` — exit 0 ·
  `90aba7c..3d04d1c  HEAD -> build/viola-0.1.0`.
- **CI read** (entry 18's form, `ci.py conclusion --sha HEAD --wait 1800`) — exit 0 ·
  `3d04d1cac3ac verdict: green · checks 15/15 · wall 280 s · runs ci#36527341834 completed/success`. The recorder's
  first run on all three OSes: green.

## Measurement pushes (plan step 10)
The loop step of `h2-loop-step.md`, inserted verbatim after `Coverage and doctest (sh shim)` by a byte-mode script
(unique anchor asserted once, LF only, both junctions read back, +38 lines exactly); `yaml.safe_load` parses the file
and the `test` job's step order reads `… Coverage and doctest (sh shim) → H2 loop (measurement only) → Harness
lifecycle (pwsh shim) …`.
