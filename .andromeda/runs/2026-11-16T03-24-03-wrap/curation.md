# Curation — 2026-11-16-wait-wake-condition

CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  none
  Tier 2 (.claude/rules/*):                   none
  Tier 3 (.claude/docs/session-learnings.md): extended «2026-11-13 — A remove-the-guard control can go red without running the guard's own case»
  Filters: 0 dup · 0 task-specific · 0 conflict (→ handoff) · 0 deferred (→ handoff) · 1 below confidence
  Extended: T3/.claude/docs/session-learnings.md: «A remove-the-guard control can go red without running the guard's own case» + «a scope-joined waiter with no deadline hangs a whole-module control past the guard's FAIL: fire it on a filtered entry, with --timeout»
  CLAUDE.md size: 133/200 · T1 3.4 KB, 0 over 600 B

## Applied
- Extension (T3), confidence 0.9 (measured +0.4 · repeated across three runs +0.3 · technical detail +0.2).
  Proof: `.andromeda/cache/chunks/2026-11-16-wait-wake-condition/evidence/control-2.txt` (`held-turn` timeout at
  120.01 s on the whole-module entry, the two older tests SLOW past their panic) → `evidence/control-3.txt` (`holds`
  in 2.21 s on entry 5, filtered to the guard's four cases); run 01-45-25's two 1800 s timeouts (no `--timeout`).

## Dispositions of every Decisions & corrections entry
Implement's:
- The operator's direction across the four runs (no live start, no mutation run, journal, stop on a plan-moving red) —
  not a learning: this chunk's direction, followed.
- A remove-the-guard control on a park test needs a bounded wait and an entry that runs only the guard's cases —
  extended (above).
- A control fired on an entry with no per-test kill waits out the 1800 s default; give `--timeout` — extended, folded
  into the same facet (above).
- Planlint did not catch a 17-char control id — not a learning: a pipeline tool defect, the friction stream's.
- nextest writes JUnit under the workspace's `target/nextest/ci/`, not under `CARGO_TARGET_DIR` — rejected, Filter 4:
  confidence 0.3 (measured +0.4 · detail +0.2 · one-off -0.3).
- Guard refusals re-issued once — not a learning: `guard-refused`, the friction stream's.
- The timestamp estimated then corrected to `date -u` — → handoff, `recurrence-despite-learning` (CLAUDE.md's
  timestamp learning).
After implement's:
- inputs#I16 items 1–4 (F1 PROVISIONAL, the carry pinned, `session_crons` a candidate, the CI carry discharged) — not
  learnings: this chunk's rulings, carried by the report, the route and founder-forks.md.
- inputs#I16 item 6 — the leaves' «all twenty-four» relied rows against the code's 21: → handoff and the wrap card
  (the directive asks no edit); the cascade cannot reach it (no master states the count).
- inputs#I16 item 6 — `rec.py put` landed plan rev 3 with an `UNPARSED` line and exit 3 — not a learning: a pipeline
  tool behaviour, the friction stream's.
- The operator pass record: none.
