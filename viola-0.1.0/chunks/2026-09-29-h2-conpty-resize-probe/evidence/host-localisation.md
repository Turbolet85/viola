# H2 host localisation — this host, never runner evidence (plan step 7)

Measured at /implement, 2026-09-29, on the recorder of this chunk (`crates/viola-pty/src/lib.rs`, tests module).
Host builds (research.md): Windows `ver` 10.0.26200.9457, `conhost.exe` 10.0.26100.8875. No result here stands for
the windows-2025 runner (test-plan §11 Test Strategy); the runner reading is the operator pass's (plan Stage B).

## The loop gate (plan Test Commands entry 4)
`for i in $(seq 20); do bash scripts/agent-run.sh run --unit --filter "package(viola-pty) & test(/resize/)" > /dev/null || exit 1; done`
— gate run `implement-2026-09-29T05-24-12`, entry 4: **green, exit 0, 52.0 s**.

- Iterations 20 · passes 20 · failures 0. Each iteration started 5 tests (`Starting 5 tests` ×20; the P5 baseline's 4
  plus this chunk's new `spawn_reports_a_resize_to_a_child_that_reads_no_key`), and the log reads PASS ×20 for each:
  - `tests::spawn_runs_a_raw_child_that_sees_its_size_a_resize_and_its_own_exit_code` (the red test)
  - `tests::spawn_delivers_a_key_written_right_after_a_resize_to_a_child_not_yet_reading` (the `hold` sibling)
  - `tests::spawn_reports_a_resize_to_a_child_that_reads_no_key` (the key-free witness)
  - `pump::tests::pump_forwards_a_resize_that_lands_before_its_first_look`
  - `pump::tests::pump_does_not_resize_while_the_size_is_unchanged`
- Kept reports: **none** — `<temp dir>/viola-pty-watch/` held no file after the loop (every iteration passed, and a
  pass removes both reports). No host loss occurred, so no host reading is classified by `localisation-rule.md`.

## The overseer's first question, on this host
A resize reaches a child that reads no key: `spawn_reports_a_resize_to_a_child_that_reads_no_key` waits for the
watcher's `size 120x40` line with the child parked and never reading stdin, and it passed 20/20 in the loop above
(and in entry 3, the package gate, and entry 5, the default `run`). On this host the resize applies without a key.

## The `dsr-cpr` reading
A debug read, done once and removed before this stage ended: `Child`'s `Drop` was edited to keep the test-side report
on a pass (the `remove_file` of `test_report` commented out), one `run --unit --filter 'package(viola-pty) &
test(/resize/)'` ran (exit 0, 5 tests), the three kept `.test.report` files were read, the edit was restored and
confirmed gone (`grep -c TEMP-H2` → 0), and the files were deleted.

| test | test-side report |
|---|---|
| `spawn_runs_a_raw_child_that_sees_its_size_a_resize_and_its_own_exit_code` | `resize-returned` · `key-written` · `key-flushed` · `dsr-cpr 0` |
| `spawn_delivers_a_key_written_right_after_a_resize_to_a_child_not_yet_reading` | `resize-returned` · `key-written` · `key-flushed` · `dsr-cpr 0` |
| `spawn_reports_a_resize_to_a_child_that_reads_no_key` | `resize-returned` · `dsr-cpr 0` |

**This host's conhost wrote no `ESC [ 6 n` after a resize**, counted up to the verdict: for the two key tests, after
`byte 79 size=120x40` arrived; for the key-free witness, after the watcher saw `size 120x40`. So hypothesis H2-CPR
(microsoft/terminal PR #19535) does not show on conhost 10.0.26100.8875 within those windows. Whether the runner's
conhost writes one is for the runner reading.

The counter itself is covered by `count_dsr_counts_a_request_split_across_reads_once` (a request whole in one read,
split 2+2, split 1+2+1, two in one read, and a near miss `ESC [ 6 m`), green in entries 3 and 5.
