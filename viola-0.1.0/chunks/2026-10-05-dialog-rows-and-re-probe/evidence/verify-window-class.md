# Verify-driven tests join the `verify_window_` class — CI round 4

**Authority:** the overseer's decision, founder-delegated (2026-10-05, relayed by the operator), on the measurement in
`ci-rounds.md`: "neither verify change. Run C/D in parallel adds concurrent live claude only to fit a test bound, and fewer
settles is unsafe. Instead, move the verify-driven tests into the existing verify_window_ class from :84: no test-side
bound, a nextest per-test kill override sized from the measured floor plus a 3x tail (e.g. 20 s), so a hang is still
caught. The 7 s rule stays for every other test. This reverses my earlier 'not option 3' only because the measurement
shows a designed floor, not a regression; record that in evidence. Keep the round-1 settle fix. Then CI, and read it
twice green before the wrap (the red was intermittent)."

## The reversal, recorded
The earlier decision ruled the test bound out ("NOT option 3: the bound stays"), following the testing rule that a timing
red is never fixed by raising a timeout or a test bound. The measurement (`ci-rounds.md`) shows the red is not a
regression and not contention: verify's four interactive runs make a **designed floor** of seven 300 ms settles
(about 1.0 s → 2.1 s median on the ubuntu coverage leg), and both reds fall in runs with a runner-wide 2.5-3x slow tail
that the tests driving no verify show too. The overseer reversed the earlier ruling for that reason alone. The 7 s
test-side rule (`WITHIN`) stays for every other wait and every other test.

## What changed
- `tests/support/verify.rs`: `verify` / `verify_without_dialogs` (and through them the `stamped_home` and
  `dialogless_home` fixtures) spawn `viola verify` with no test-side bound (`viola_unbounded`, the class's existing form:
  `verify_without_screens` already used it).
- `.config/nextest.toml`, profile `ci` (the harness and every CI leg run it):
  - `test(/verify_window_/)`: kill at 45 s (15 s × 3). These tests wait out the gate's 5 s maximum four times by design,
    about 21 s measured. This entry comes first, because the first matching override wins.
  - the twelve binaries that drive verify, directly or through the stamped-home fixture: kill at 20 s (10 s × 2). That
    is the measured floor plus a 3x tail. The longest other test there, a `send_window_` one, read 11.25 s on the
    slowest measured run.
- The `mutants` profile is unchanged: its 10 s kill (the `verify_window_` override at 30 s) stays the stricter bound, and
  a verify against the fake takes about 2.7 s on the dev host.
- Kept: the round-1 settle before the Run C / Run D kill (`run-kill-settle.md`).

## Controls (2026-10-05, the dev host)
| control | form | result |
|---|---|---|
| a hang is still caught | a planted `sleep(40 s)` after `verify` in `verify_kills_the_dialog_and_plan_runs_only_after_their_last_stop_hook`, `cargo nextest run --profile ci` | `SLOW [> 10.000s]`, then `TIMEOUT [20.003s]`: killed by the new override |
| the same, without the override | the same planted sleep, the default profile (no kill) | `PASS [43.346s]`: nothing bounds it, so the override is what catches a hang |
| verify past 7 s now passes | not forceable without a new seam: the only verify-side hold, `--stop-receipt-hold-ms`, must stay under the 300 ms quiet period or it defeats the Run C / Run D settle it exists to witness (a 1 000 ms hold read `left: 4` `right: 6` under both forms, about 6.5 s, never the bound) | recorded as not forced |

Both planted edits were reverted. `git diff` shows `tests/cli_verify.rs` unchanged by them.
