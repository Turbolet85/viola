# Curation — 2026-11-14-budget-gate-removed

CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  none
  Tier 2 (.claude/rules/*):                   + testing.md: "A wheel move taken through `human_input` writes its record unwaited … take the move through a call that waits for its record" (confidence 0.8)
  Tier 3 (.claude/docs/session-learnings.md): none
  Filters: 3 dup (one by master home) · 4 task-specific / not a learning · 0 conflict · 2 deferred (→ handoff) · 1 recurrence (→ handoff)
  No-other-home: "A wheel move taken through `human_input` writes its record unwaited"
  Extended: 2/testing.md: "A test that counts `events.ndjson` lines after `Wrapper::stop` …" + "find the tests a start-record count change moves by their counted literals, never by the removed kind's name"
  Extended: 2/testing.md: "No gate builds the separate `fuzz/` workspace …" + "prune `fuzz/Cargo.lock` without the nightly by `cargo metadata --offline`, read back by a `--locked` `cargo tree`"
  CLAUDE.md size: 133/200 · T1 3.4 KB, 0 over 600 B

Landed: `rec.py harness land .claude/rules/testing.md` rev 20 · 1 new · 2 revised · TX 1229.

## Applied and extended

- **New — `human_input` unwaited (T2 testing.md, 0.8):** verified by a real gate failure +0.4 · specific detail +0.2 ·
  no other home +0.2 (no master row, route annotation, rule or contract states it).
  Proof: implement firing 1, `release_carrying_a_budget_key_is_invalid_params_and_moves_no_wheel::case_1_budget_true`
  red (`left: Driver right: Human`, and an empty record list), green at firing 2 taking the move through `pause`.
- **Extended — start-record count (T2 testing.md, 0.9):** gate failure +0.4 · detail +0.2 · repeated over five test
  files +0.3. Facet over the 2026-10-04 entry: how to FIND the moved tests (by the counted literals, over the whole test
  tree), which the matched entry lacks. Merges the report's «Research's file list missed five companions» entry.
  Proof: implement firing 1's integration reds — `links_survive_a_wrapper_restart` index 8 out of bounds,
  `hook_session_end_appends_directly_only_while_the_log_is_free` 2 vs 3, two `timed out waiting for the hook events`.
- **Extended — fuzz lock prune (T2 testing.md, 0.8):** measured +0.4 · detail +0.2 · no other home +0.2. Facet over the
  2026-11-13 fuzz entry: the lock refresh, not the compile check. Proof: implement P1 — `cargo metadata … --offline`
  rewrote `fuzz/Cargo.lock` (−223 lines, `chrono` and its dependents), the `--locked` `cargo tree` gate entry green.

## Every Decisions & corrections entry

- Q1 ratified by the founder's own word — not a learning (a ruling, landed in architecture by P2).
- The probe-clash answer is technical, not PROVISIONAL — not a learning (a correction of the outcome file's label).
- This wrap's operator directive — not a learning (a directive this wrap executes).
- The two-window form — not a learning (a run fact).
- A probe that forbids a word and a test that must spell it cannot share a tree — rejected, Filter 1 by master home:
  test-plan `dec-n5cm3kti` «Retired words kept as committed fixtures» carries it (the leaf comparer cited it).
- A wheel move through `human_input` is recorded unwaited — applied (above).
- A start-record count change moves every test that counts the log — extended (above).
- The fake agent's `statusline-echo` marker appends one line per run — deferred (Filter 5 cap; 0.8 with no-other-home)
  → handoff.
- Research's file list missed five companions — merged into the start-record extension (the same lesson: sweep the
  test tree for the old literal).
- The fuzz lock refreshes without the nightly — extended (above).
- Sweep hazard `(?i)budget` mixes other budgets — rejected, Filter 2 task-specific (the gate is gone; no later sweep
  keys on it).
- Sweep hazard — a count the masters never state — rejected, Filter 1 duplicate (the site-sweep rule already makes the
  refusal a zero's basis).
- `gate.py control` with rstest cases — deferred (Filter 5 cap; 0.8 with no-other-home) → handoff.
- Hook false positive — stamp-ahead on a clock-time literal — rejected, Filter 4 (one-off, 0.2).
- A timestamp written by estimate, corrected — recurrence-despite-learning of the Tier-1 «A timestamp written into any
  record … is read from `date -u`» → handoff.
- Guard refusals — not a learning (the standing rules held).
- The operator's run directive held — not a learning (a run fact).
- The operator pass ran as usual — not a learning (a run fact).
- This window: the cascade's bare-number pattern terms — not curated (a pipeline-tool defect, carried by the friction
  record and the owner's ruling, inputs#I9).
