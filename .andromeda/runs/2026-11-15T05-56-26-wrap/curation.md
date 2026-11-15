# Curation — 2026-11-14-status-line-interception-removed

CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  none
  Tier 2 (.claude/rules/*):                   + testing.md: "A removal sweeps every test that reads the removed field, row or mode before its first gate run …" (confidence 0.8)
    Proof: implement's first full gate run read red 5 on tests the plan did not name (two `tests/cli_instance_state.rs` cases asserting the removed snapshot field, the `test_cases_checks.rs` verdict vector, two verify summaries), `.andromeda/runs/2026-11-15T05-28-03-implement/journal.md`; the hangup case's borrowed `statusline-echo` hook re-pointed at `/usr/bin/touch` (report, Deviations). Signals: measurement +0.4, specific detail +0.2, no-other-home +0.2.
  Tier 3 (.claude/docs/session-learnings.md): extended "A plan whose test and probe contradict: narrow the probe" + the minted-name facet
    Proof: the plan named five tests and five fixture keys with its own identifier probe's tokens; implement stopped twice (relay-1.md, relay-2.md of `.andromeda/runs/2026-11-15T05-14-17-implement/`); the overseer's rulings `inputs#I6` («never a split string, which would game the probe») and `inputs#I7`. Signals: the overseer's correction +0.4, measurement +0.4 (= 0.8).
  Filters: 0 dup · 4 task-specific · 0 conflict · 0 deferred (cap) · 1 recurrence (→ handoff)
  Extended: T3/session-learnings.md: "A plan whose test and probe contradict: narrow the probe" + "a name the plan mints that need not carry the token takes a neutral word, the probe kept"
  No-other-home: "A removal sweeps every test that reads the removed field, row or mode before its first gate run"
  CLAUDE.md size: 133/200 · T1 3.4 KB, 0 over 600 B

## Decisions & corrections, each entry's disposition
1. Plan names vs its own identifier probe → extended (T3, the entry above).
2. The overseer's ruling: neutral words, the probe exactly as planned, never a split string → extended (folded into 1's facet).
3. Check 4 (9)'s baseline on a tree mid-implement → rejected — Filter 4 (one-off, task-specific; 0.2).
4. A removal sweeps every test reading the removed field → applied (T2 testing.md).
5. A borrowed removed mode re-pointed at a plain command, never deleted → applied (folded into 4's entry).
6. A «writes nothing» witness over a home whose wrapper STOPPED → rejected — Filter 4 (no failure measured; 0.2).
7. Sweep hazard: the identifier probe is case-exact (`Statusline.1.json`) → rejected — Filter 2 (this chunk's fixture name alone).
8. Sweep hazard: the `status-line` pattern hits other meanings → rejected — Filter 2 (this wrap's own sweep).
9. Guard refusals → not a learning: the guard's own telemetry (evolve `guard-refused` signals).
10. A P5 card `none` on a revision and the take-up card's rows → not a learning: a pipeline letter's silence (recorded for the pipeline, not curated).
11. `pre-push` reproduced the narrower entries' reds → rejected — Filter 4 (a positive measurement of a mechanism; no new rule).
12. The operator's standing directions held → not a learning.
This window: an evolve record's `ts` typed ahead of the clock (refused by the tool, re-stamped from `date -u`) → recurrence-despite-learning of CLAUDE.md's «A timestamp written into any record … is read from `date -u`» (→ handoff, Deferred learnings).
