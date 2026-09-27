# viola-pty watch — the recurrence (OPEN) and the recorder folded in (overseer direction, founder-delegated)

## The recurrence — recorded OPEN

- Watch: `viola-pty tests::spawn_runs_a_raw_child_that_sees_its_size_a_resize_and_its_own_exit_code` (scope item 7, CARRY 4).
- **Recurred once, the first local red:** 2026-09-27 18:02 local (16:02:58Z end of the operator pre-push, entry 35), in the windows
  leg's cargo-mutants unmutated baseline under the nextest `mutants` profile: `TIMEOUT [10.107s]`, 363 tests in parallel. Details
  are in `operator-pass.md`.
- **Capture absent:** the profile's 10 s kill preempted the test's own 10 s `lines()` bound, so the child report never printed. Its
  stdout held only `running 1 test` / `(test timed out)`.
- **Status: OPEN.** A green re-run does not close it (testing.md 2026-09-27: a green proves no cause). The cause stays unknown until a
  recurrence arrives with its report.
- **Expiry counter reset to 0.** The watch retires only after 3 consecutive green CI runs with no recurrence of either watched name,
  counted from this recurrence.
- Red-to-first-action for overseer1: **36 s at most** (red 16:02:58Z; `date -u` two calls after the first read = 16:03:34Z), as
  in `operator-pass.md`.

## The recorder (not a fix: the test's behaviour and assertions are unchanged)

In `crates/viola-pty/src/lib.rs`'s tests, shared by all four real-PTY tests (the watched one, its forced-window twin, `kill_ends_…`,
`terminate_ends_…`), which use one set of helpers (`spawn_child_entry`, `lines`, `wait_exit`, `CHILD_WITHIN`):

1. **`CHILD_WITHIN` 10 s → 7 s**, below the `mutants` profile's kill (`slow-timeout = { period = "5s", terminate-after = 2 }` =
   10 s). This lowers a bound; plan step 2 forbids raising it. A stuck wait now fails the test itself, with `child report stopped at
   [...]` in its message, before the runner kills it (testing.md 2026-09-24, extended 2026-09-27).
2. **The report streams to a known file.** The child already appends each report line to its file as the step happens. That file
   moved from a random tempdir to `<temp dir>/viola-pty-watch/<test name>.report` (e.g. `tests.spawn_runs_a_raw_child_….report`).
   A runner kill never runs cleanup, so the file stays. A panicking test keeps it, and only a passing test removes it (`impl Drop for
   Child`). Under the pre-push legs, `<temp dir>` is the mutation scratch (`TMP`/`TEMP` on the host, `TMPDIR` in the distro), so a
   leg's red leaves the report there until the next run's wipe.
3. The same `Drop` stops a PTY child still running when a test ends early, so a failed test leaks no child process.

## Remove-the-guard pair (temporary control test, removed after; `grep -c zz_watch_control` = 0)

Control: `tests::zz_watch_control` spawned a `block` child (it reports `start …` and never reads) and waited for its second line.
Command: `cargo nextest run -p viola-pty --profile mutants -E 'test(=tests::zz_watch_control)'`, this host, 2026-09-27 ~16:06–16:08Z.
Each edit of `CHILD_WITHIN` was confirmed on disk (`grep -n`) before its run.

| `CHILD_WITHIN` | nextest | test output | `viola-pty-watch/tests.zz_watch_control.report` |
|---|---|---|---|
| **7 s (the guard)** | `FAIL [7.021s]` | `child report stopped at ["start pid=14276 raw=true size=80x24"]` | kept: `start pid=14276 raw=true size=80x24` |
| 10 s (restored) | `TERMINATING [> 10.000s]` → `TIMEOUT [10.021s]` | `child report stopped at [...]` **and** `(test timed out)`: the bound and the kill race, and this time the dump got out; at 16:02Z it did not | kept |
| 12 s (past the kill) | `TERMINATING [> 10.000s]` → `TIMEOUT [10.016s]` | only `(test timed out)`: **the report is lost from the output**, exactly as in the 16:02Z red | kept: `start pid=18208 raw=true size=80x24` |

- So at 10 s the loss is a race, not a certainty: the pre-push red lost it and this control run did not. Past the kill it is certain.
  At 7 s the test's own dump always wins.
- The file recorder kept the report in every row, including the killed ones.
- The blocked children (pids 14276, 63600, 18208) were all gone after each run (`Get-Process -Id` count 0).
- Restored to 7 s, control removed. Then: `cargo fmt --all --check` 0, workspace clippy `-D warnings` 0, gate entries 16 (the three
  split crates' units) and 17 (the forced-window test 20×) green. `viola-pty-watch/` was empty after them (passing tests remove
  their reports).

## Siblings outside this chunk (a finding, not changed)

Root-package tests also wait on a child with a 10 s bound equal to the `mutants` kill, and they run in a baseline whenever the
diff touches the root package: `tests/support/fake.rs` `WAIT_WITHIN`, `tests/support/home.rs` `READY_WITHIN`,
`tests/support/outer_pty.rs` `EXIT_WITHIN`, `tests/run_cli.rs` `READY_WITHIN` and `:262`, `tests/cli_instance_state.rs:220`,
`tests/contract_diag_schema.rs:244`/`:264`. They share the race, but none is a watched name, and none is in this chunk's files.
This is the operator's call.
