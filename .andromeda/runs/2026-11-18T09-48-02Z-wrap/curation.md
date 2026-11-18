# Curation — 2026-11-18-trust-record-row-read-live (wrap run 2026-11-18T09-48-02Z-wrap)

CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  none
  Tier 2 (.claude/rules/*):                   + testing.md: "A test-support helper in a crate's `src/` that spawns an instrumented binary under `env_clear()` runs it with a scratch cwd, or re-adds `LLVM_PROFILE_FILE`" (confidence 0.8) · + testing-src.md: its pointer line
  Tier 3 (.claude/docs/session-learnings.md): none
  Filters: 3 dup · 4 task-specific · 0 conflict (→ handoff) · 0 deferred (→ handoff) · 2 recurrence-despite-learning (→ handoff)
  No-other-home: "an instrumented child spawned under env_clear() from src/ runs in a scratch cwd"
  CLAUDE.md size: 134/200 · T1 3.9 KB, 0 over 600 B

## The applied entry
- testing.md `## Session Additions` 2026-11-18 (with the testing-src.md pointer, one learning).
  Proof: the operator pass's first `bash scripts/agent-run.sh pre-push` read red on
  `rig_keeps_no_credential_or_host_path_in_any_record` («the tree's status moved», three `crates/viola-e2e/default_*.profraw`),
  cause measured in `crates/viola-e2e/src/rig/mod.rs` `cli_version` (spawned under `env_clear()` with no `current_dir`),
  fix the scratch cwd, re-fired green (`evidence/operator-pass.md` §Entry 16). Signals: verified by a real gate failure
  +0.4 · specific technical detail +0.2 · no other durable home +0.2 (the test-plan add was rejected at check 2 as
  curation-shaped, nothing on the route) = 0.8. Filter 1: the generated body `testing.md` «`.env_clear()` must re-add
  `LLVM_PROFILE_FILE`» matches (dedup-reject-only) — the facet it lacks is its scope: the rule does not load on `src/`,
  where the rig lives, so the facet lands as a NEW Session Additions entry; the recurrence is logged too (handoff).

## Each Decisions & corrections entry, dispositioned
Implement's:
- The founder's caps held (R6 on S0R / S0R-2 / S0R-3, S0C's own ledger, «1 + 1» spending S0B only) — not a learning:
  the directive's and the founder's per-chunk word, recorded in the report.
- «Риг сейчас, окно — отдельной записью» (inputs#I17 item 1) — not a learning: a ruling of this chunk, carried to P5
  as the new route entry.
- Recurrence of «a scripted write fires no hook» (run 01-02-09Z, `cargo fmt` red) — rejected (Filter 1, a dedup of a
  DEFECT record kept in the pipeline's own implement letter): → handoff as `recurrence-despite-learning`.
- The host CLI moved to 2.1.326 / 2.1.327, starts kept on 2.1.325 by path — rejected (Filter 1, duplicate of
  verification-harness.md 2026-10-10, extended 2026-11-13).
- Guard refusals (two `cd` moves, one `rec.py render | grep`) — rejected: the guard itself refuses the act
  (`guard-refused`, no learning).
- Sweep hazard: a fixed-string hunt for the retired case name hits run dirs' `control-1.toml` and outcome history —
  rejected (Filter 4: 0.2 − 0.3, a one-off of this chunk's retired name).
Earlier runs' outcome files (the report's outcome basis):
- `cargo fmt` red from python-scripted edits (run 01-02-09Z) — the same recurrence as above, one event.
- Step 15's fake oracles copied from the first run — rejected (Filter 1, duplicate of testing.md 2026-10-16).
- The rstest `case_5` → `case_05_` padding of a control's case name (run 01-39-23Z) — rejected (Filter 4: 0.2, caught
  before the control fired, one-off).
- `Place` for clippy's `too_many_arguments` (run 08-10-59Z) — rejected (Filter 2, task-specific to one helper).
- The respelled `fold-review.py` path on the overseer's word (inputs#I15) — rejected (Filter 2, one file's ruling).
- The recursive `rm` of the plant clone refused (run 01-02-09Z) — rejected: host-linux.md 2026-10-07 holds it
  (Filter 1).
`## After implement`'s:
- The pass: no correction or ruling — nothing to weigh.
- The wrap directive's item 5 departures: the `.profraw` fix — APPLIED above; `verify_started`'s `Place`, the
  `fold-review.py` respell and the rehearsal oracles from the fake's first run — rejected as above (Filters 2 · 1).
- The widening on the answer card — not a learning: the masters' (P2, Q1).
- F2 not confirmed — not a learning: the route's (P5).
This window:
- The Bash guard refused three `cd` moves into a reference or run dir — rejected: the guard holds the act.
- The `.env_clear()` rule in testing.md did not load where the defect was written — `recurrence-despite-learning:
  testing.md «.env_clear() must re-add LLVM_PROFILE_FILE»` → handoff (its scope is the remedy's, the pipeline owners').
