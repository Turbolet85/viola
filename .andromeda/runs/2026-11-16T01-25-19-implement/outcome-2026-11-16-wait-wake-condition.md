# Outcome — 2026-11-16-wait-wake-condition · run 2026-11-16T01-25-19-implement

**Outcome: surfaced** — a plan entry's predicate (entry 9) contradicts the measured class; stopped before the controls,
the rest of the block and the operator pass, on the operator's direction ("A red that moves a plan entry: stop and
say so BEFORE the operator pass").

## The gap
- **Entry 9** (`role = 'probe'`) asserts `after_tests == before_tests and after_max_test_s < 20 and after_wall_s <
  before_wall_s`; plan step 8 predicted `after_max below 20 s`.
- **Measured** (`.andromeda/cache/chunks/2026-11-16-wait-wake-condition/evidence/class-wall.json`, produced by
  `class-wall.sh` at 2026-11-16T01:35Z, both sides exit 0, 321 tests each, no failure):
  before wall 56.069 s → after 48.381 s (holds) · tests 321 = 321 (holds) · max test 43.164 s → 43.358 s (fails).
- The max is `runs::verify_window_without_screens_fails_every_interactive_row` on both sides: the `verify_window_`
  class, which waits out the gate's 8.5 s maximum four times by design under its own 60 s `ci` kill
  (`.config/nextest.toml`, the first `[[profile.ci.overrides]]`). The stamped-home fixture cannot move it.
- Entry 9's line: `9 probe red · last line 'True' ✗ (last: 'False')`.
- Split of the same JUnits (`class-wall-split.py`, read-only): outside the 3 `verify_window_` tests the slowest is a
  `send_window_` test (10.996 s → 10.913 s), under 20 s on both sides; the carried red's own test
  `path1_run_start_one_home_in_order` 6.593 s → 2.385 s; summed test time 840.077 s → 475.522 s.
- What would close it (the operator's / phase's call, not implement's — the plan is not edited here): entry 9's max
  read over the stamped-home tests outside the `verify_window_` class (its own kill), or the class's
  `path1_run_start_one_home_in_order` time, or `max < 60` for that class. Implement authored none of these.

## Files changed
- `src/run/wait.rs` — test module: `wait_a_turn_end_wakes_whatever_its_background_tasks` (rows zero · one · three ·
  null), `wait_armed_at_a_task_free_end_parks_through_a_notification_and_a_harness_turn`.
- `fixtures/fake-scripts/wait-held-turn.json` (new) — the five gated steps.
- `tests/cli_wait_last.rs` — `boot_held`, `turn_ends`, `path3_wait_wakes_on_a_turn_end_holding_background_tasks`,
  `path3_wait_armed_at_a_task_free_turn_end_parks_through_a_notification_and_a_harness_turn`.
- `tests/support/home.rs` — `stamped_home` copies one stamp per build (`stamp_dir`, `stamp_key`, `file_sha256`,
  `copy_tree`, `take_stamp`, `remove_stamp_dirs`, `sweep_other_stamps`).
- `tests/cli_verify.rs` — the two stamp-isolation tests and `tree_facts` (see Deviations).
- `.config/nextest.toml` — the verify-driven override's comment only.

## Deviations
- The two step-6 tests live at `tests/cli_verify.rs`'s root, not `tests/cli_verify/stamps.rs`: entry 7's
  `test(=name)` filter matches no test inside the `stamps` module (first firing: exit 4, `no tests to run`).
  Scope record: ``- `tests/cli_verify.rs` · in-intent · serves step 6 · self``.
- `tests/support/verify.rs` (listed) is unchanged: the stamp reaches its tmp dir through the existing `verify::verify`.
- The fixture also sweeps other builds' stamp dirs whose lock it can take without waiting (`try_lock`), so per-build
  96 MB stamps do not pile up on the tmpfs backing; a stamp another process holds stays.
- Step 8's after side leaves out the chunk's two new stamp-isolation tests (`-E "(<filter>) - (<the two>)"`), so both
  sides run the same 321 tests. nextest writes JUnit under the workspace's own `target/nextest/ci/`, not under
  `CARGO_TARGET_DIR`; the script's first firing failed on that and was fixed and re-run whole.

## Gates as of the stop
- Green: 1 fmt · 2 clippy · 3 check · 4 wait unit rows · 5 viola-core · 6 the two booted wait cases (`2 tests run:
  2 passed`) · 7 the two stamp tests (second firing) · 8 the writer binaries (44.98 s).
- Red: 9.
- Not run: 10 (the guard probe), 11 `run --unit`, 12 `pre-push`; the six controls of step 9 (file written,
  `control-1.toml`, not fired); the operator entries 13–15. No matrix ref set; no chunk report (a surfaced run writes
  none).

## Process hygiene
| process | started by | final state |
|---|---|---|
| nextest / viola / fake agent of the gate entries and `class-wall.sh` | this run | terminated (census at 2026-11-16T01:38Z: no process of this repository's `target/`; the 7 live `viola` are the bridge's and the lab's builds) |

Left on disk (gitignored, the operator's desk): `target/class-wall/` (the clone at df3bad61 and two target dirs);
the stamp dir `target/e2e-home/viola-stamp-*` on the tmpfs.

## Decisions & corrections
- Bash guard refused a `cd` out of the root (Setup) and a `cat` heredoc append (P1); each re-issued once in the
  passing form.
- A journal stamp was first written estimated (`01:33Z`) and corrected to the `date -u` reading — the standing
  timestamp learning, recurred.
- The operator's direction for this run: no live start, no mutation run, journal each long run before a wait, stop
  on a plan-moving red before the operator pass — followed.
