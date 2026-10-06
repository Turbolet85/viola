# The light gate's first run: one red, `pre-push`, on truncated coverage profiles

**First run of the block, 2026-10-06T22:07:09Z to 22:09:27Z: `entries 28 · green 22 · red 1 (25) · not-run 5`.
The red is `bash scripts/agent-run.sh pre-push`: all 1 638 tests passed, then the coverage merge failed.**

## The reading
- The entry's document: `"cmd":"pre-push","ok":false,"stage":"linux-tests"`; suite `coverage`
  `"passed":1638,"failed":2`, failures `["llvm-cov-exit-1","llvm-cov-summary-missing"]`. nextest's own summary:
  `1638 tests run: 1638 passed (5 slow), 0 skipped`.
- `llvm-profdata merge` refused three raw profiles of 4 709, each `invalid instrumentation profile data (file
  header is corrupt)`, and ended `no profile can be merged`: sizes 57 344, 4 096 and 95 904 bytes, written at
  22:09:08Z, 22:09:08Z and 22:09:11Z. No profile is empty.
- No test of this chunk failed and no bound fired.

## What else the host was doing (measured)
- Inside the entry's window, 22:08:37Z to 22:09:28Z, another project's cargo build on the same volume
  (`~/dev/projects/escher/target`) wrote 793 files, 10 781 MB (by modification time, 22:08:47Z to 22:09:24Z); in
  the block's earlier 88 s it wrote 561 MB in 5 s. A `cargo test --workspace --locked` of that project was running
  when the entry ended, and the 1-minute load read 105 on 32 cores.
- The same entry read green four times on this tree earlier in the day with no such write under way (the two
  full blocks of `/andromeda-implement`, its run before the operator pass, and the CI coverage leg of
  ci#37534758441).

## What is known and what is not
- Known: the red coincides with a 10.8 GB write storm from another session's build; the failing step is the
  profile merge, not a test.
- Not known: which process wrote each truncated profile and what cut it short. A raw profile is written at a
  process's exit. Hypothesis, not measured: under that load a process still writing its profile was ended by its
  terminal's hang-up (verify's Runs A, C and D end their child by a kill after a 300 ms quiet screen; a hook
  process still exiting then loses its profile part-written), the class `verification-harness.md` already names
  for the fake agent's own exit.
- The operator's standing disposition for this host's contention reds (relayed by the overseer at this wrap):
  environment, relayed to overseer1 (its F167); no bound or test moves.

## The second run of the block, 2026-10-06T22:11:15Z to 22:14:01Z: one red, another entry
- Started after 30 s with no other build running (four polls). Summary:
  `entries 28 · green 22 · red 1 (19) · not-run 5`. `pre-push` read green this time (42.2 s).
- The red is `bash scripts/agent-run.sh run`, the default selector: unit `1312 passed`; integration
  `326 tests run: 287 passed (11 slow), 25 failed, 14 timed out` in 54.4 s against its usual 29 s to 35 s. The 39
  are spread over ten binaries, most of them untouched by this chunk (`cli_wait_last`, `run_cli`, `hook_events`,
  `hook_fail_open`, `cli_wheel`, `contract_diag_schema`, `cli_version_gate`, `cli_instance_state`, `cli_verify`,
  `contract_fake_agent_drift`): 7 s test-side bounds and 20 s kills, the stalled-start shape of
  `evidence/block-reds-host-contention.md`.
- The other project's build wrote 191 large files, 23 001 MB, to the same volume during this run: the 30 s of
  quiet was a gap between two of its builds.
- So across the two runs every entry of the block read green at least once, and neither run is a clean block.

## The third run, 2026-10-06T22:15:51Z to 22:19:01Z: one red, a third entry
- Started after 45 s with no build running and a 1-minute load under 20. Summary:
  `entries 28 · green 22 · red 1 (18) · not-run 5`: the wrapped-send case, `FAIL [  17.932s]`,
  `wrapper builder not ready`, `starts w1 c0`, the boot fixture before any send.
- Other projects' builds wrote 95 large files, 11 607 MB, during the run.

## The fourth run, 2026-10-06T22:20:50Z to 22:23:01Z: the verdict
- The operator's answer at this wrap (the overseer, 2026-10-06): the other builders are held for the window,
  overseer1 holding escher-builder at its seam; "run the whole block once".
- Read before the start: 1-minute load 10.3, no `cargo`, `rustc` or linker process on the host.
- Summary: **`entries 28 · green 23 · red 0 · recorded 0 · timeout 0 · not-run 5`** (the round's two legs and
  the three operator entries). `pre-push` 42.8 s, the default run 35.0 s, the five binaries 22.0 s, the
  wrapped-send case 1.1 s. No other build wrote a large file during the run.

## Two-sided reading over the four runs
| run (UTC) | other builds' large writes during it | summary |
|---|---|---|
| 22:07:09 to 22:09:27 | 10 781 MB inside `pre-push`'s window | red 1: `pre-push` (3 truncated profiles) |
| 22:11:15 to 22:14:01 | 23 001 MB | red 1: the default run (39 stalled starts) |
| 22:15:51 to 22:19:01 | 11 607 MB | red 1: the wrapped-send case (boot not ready) |
| 22:20:50 to 22:23:01 | 0 MB, the builders held | red 0 |

## What follows
- The fourth listing is the light gate's verdict: green.
- The three reds are not closed by it. Their common cause is measured as far as "another session's build
  writing to the same volume"; the mechanism of the truncated profiles is a hypothesis. The `pre-push` red is
  pinned as a `WATCH:` on the next markerless entry, "Local-command send outcomes" (route-resolve's
  recurrence-watch row), origin this chunk, 0 green runs so far; the stalled-start class is the one the
  operator dispositioned as environment (relayed to overseer1, its F167), with no bound or test moved.
