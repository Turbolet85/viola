# Implement outcome — 2026-10-13-epoch-4-survivors-ii

**Outcome: green + smoke.** All 16 `typed.rs` survivors from the Epoch 4 audit are disposed. Each one has a
behaviour-preserving restatement in `src/cmd/verify/typed.rs` and a named case in its `mod tests`. Each case read
red under its mutation, applied by hand at the restated site, and green again after the restore. The P2 block was
green on its first run: 18 green, 0 red, 3 operator entries not run. The operator pass then ran on the operator's
word, recorded in `evidence/operator.md`:
- entry 18 (pre-push) re-fired green;
- entry 19 (hygiene) read clean;
- the pre-CI commit `94f0be906ceb` was made;
- entry 20 pushed it;
- entry 21 read `verdict: green · checks 15/15`, `ci#38442894573`, across windows-2025, macos-latest and
  ubuntu-latest.

Next → `/andromeda-wrap-session`.

## Files changed
- `src/cmd/verify/typed.rs` (the one listed file):
  - `added_turn_ended`, `local_command_answered` and `probe_until` beside `framing_turns`. All six
    `clock.now() + PROBE_DEADLINE` sites now go through `probe_until`.
  - `SharedPty<P>` with `impl<P: Pty> Pty for SharedPty<P>`.
  - `ctrl_c_ending` and `ended_by` as plain functions. `Run::wait_pump` is removed. `Run::end_by_ctrl_c` and
    `Run::finish` call the new functions.
  - 8 new test functions (21 cases with the rstest rows), plus the test-only `FixedClock` and `Recording`.
- Chunk records: `viola-0.1.0/chunks/2026-10-13-epoch-4-survivors-ii/evidence/survivors.ndjson` (16 rows, each
  with `form: restatement` and both `file_at_head` and `by` filled) and `evidence/survivors.md` (the ledger, plus
  the 14 red/green pairs).
- No other file. The preservation guard (entry 10) and the removed-lines guard (entry 9, last line 0) are both
  green, and `scope` reads `clean — changed 1 · listed 1`.

## Kills
There were 14 hand applications (m01–m14) for the 16 audit rows. `:286:29`, `:651:45` and `:671:53` are the one
`+` in `probe_until` (m03). Each mutation was:
1. applied as one exact replacement, with `rustfmt` in the same process and the edit confirmed by a byte
   comparison;
2. read through `bash scripts/agent-run.sh run --unit --filter 'test(/(^|::)tests::(<case>)(::|$)/)'`;
3. reverted by restoring the saved bytes (confirmed byte-equal), then read green with the same filter.

Every red came from the case's own assertion. None was a compile error or a panic outside an assertion. m03's
`Instant - 120 s` did not panic because the host's monotonic clock was past 120 s. The readings are in
`evidence/survivors.md`. The Epoch 5 boundary audit is the reader that confirms them.

## Gates (P2, firing 1)
```
  1 lint        green · exit 0 · 0.82s · cargo fmt --all --check
  2 lint        green · exit 0 · 1.51s · cargo clippy --workspace --all-targets --features fake-agent -- -D warnings
  3 lint        green · exit 0 · 1.73s · CARGO_TARGET_DIR=target/wincheck cargo clippy … --target x86_64-pc-windows-msvc
  4 lint        green · exit 1 · 0.02s · G1 rg (no bare instrument)
  5 lint        green · exit 0 · 0.63s · bash scripts/lint-probes.sh
  6 unit        green · exit 0 · 6.61s · bash scripts/agent-run.sh run --unit
  7 integration green · exit 0 · 35.27s · run --integration --filter 'binary(cli_verify)'
  8 probe       green · exit 0 · the survivor ledger (last line 1)
  9 probe       green · exit 0 · the removed-lines guard (last line 0)
 10 probe       green · exit 0 · the preservation guard
 11 probe       green · exit 0 · cargo deny check
 12–17          green · the harness smoke: cleanup, boot, status, G2, G4, cleanup
 18 probe       green · exit 0 · 67.61s · bash scripts/agent-run.sh pre-push
entries 21 · green 18 · red 0 · recorded 0 · timeout 0 · not-run 3
```
There were no deferrals. The changed language is Rust, and every Rust gate ran. No `watch:` subject was named.
The matrix has no capability claimed by this chunk (`matrix.py show --chunk`: 0), so there was no ref write.

## Smoke (P3)
The smoke was recorded from P2 and not re-driven. Entries 12–17 are the plan's `role = 'smoke'` entries. Cleanup
read `processes_gone` and `endpoint_gone` true. Run B's end-to-end path is witnessed by entry 7 (`cli_verify`, over
the fake agent).

## Deviations from plan
- **The kill filter.** Step 4 writes `test(/(^|::)tests::<name>$/)`. An rstest table case is named
  `…::<name>::case_<n>_<label>`, so `$` selects none of the four table functions. Every hand-applied run used
  `(::|$)` in its place, which selects the plain case or every row of a table. The plan was not edited.
- **The m03 run selects more.** It ran the `ctrl_c_ending` table beside `probe_until`, so the shared `+` is read
  through both, as step 4e asks. That made 5 reds where the plan named one.
- **Scope record.** None was written: no edit was made outside research's list.

## Surfaced
None.

## Process hygiene
| process | started by | final state |
|---|---|---|
| the smoke wrapper `p-e4s2-smoke` (boot, entry 13) | a P2 gate entry (this run) | terminated: cleanup entry 17 read `processes_gone` and `endpoint_gone` true; the host's process list at 2026-10-13T17:55:13Z showed no process whose exe is under this repository |
| the test binaries of entries 6, 7 and 18, and of the 28 hand-mutant harness runs | this run | terminated: absent from the same process list |

The `viola` processes on the host's list are all `…/viola-lab/prototype/target/debug/viola`, the overseer's bridge.
They are another tree's build, not this run's. They were left running and are not this run's to stop.

## Decisions & corrections
- The operator's direction (this run's arguments): no live `claude` start, no mutation run, witness run or workflow
  dispatch, and each kill shown by a mutation applied by hand. Every time written into a record is read from
  `date -u` in the same call: the evolve records and the `survivors.md` stamp are read from the clock inside the
  writing process. Pipeline-tool output goes to a file that is read whole. Each long run is journaled in
  `journal.md` before the wait. The operator pass runs as usual.
- Kill-filter recurrence: none. The `(::|$)` form above is a plan-text gap for rstest tables, not a recurrence of
  the 2026-09-27 filter learning.
- Guard or hook refusals: none.
