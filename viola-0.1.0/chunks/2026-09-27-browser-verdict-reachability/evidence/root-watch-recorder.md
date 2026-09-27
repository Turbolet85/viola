# Root-watch recorder: the remove-the-guard pair (plan steps 12–13; overseer direction, not a CARRY)

## The recorder (not a fix: each loop's predicate and assertion text are unchanged)

`tests/support/watch.rs`, `pub mod watch;` in `tests/support/mod.rs`:
- `WITHIN` = 7 s, strictly below the nextest `mutants` profile's kill (`slow-timeout = { period = "5s", terminate-after = 2 }` = 10 s).
- `Watch::start(label)` creates (truncates) `<std::env::temp_dir()>/viola-root-watch/<thread name, :: → .>.<label>.report`.
- `note(line)` appends a line only when it differs from the last one noted, so the report streams the polled state as it changes.
- `deadline_check(deadline, what)` panics past the deadline with `what`, then `watch report:` and every line so far.
- `Drop` removes the report unless the thread is panicking. A runner kill never runs `Drop`, so the file stays.

The 8 sites, each on `watch::WITHIN`. `grep -rnE 'from_secs\(10\)' tests` prints nothing:

| site | label | line noted |
|---|---|---|
| `tests/support/fake.rs` `wait_for` (was `WAIT_WITHIN`) | `receipt` | `receipt lines <n> kinds <kinds>` |
| `tests/support/home.rs` `wait_ready` (was `READY_WITHIN`) | `ready` | `starts w<a> c<b> r<c> snapshot <bool> beat <bool>` |
| `tests/support/outer_pty.rs` `wait_exit` (`EXIT_WITHIN` = `WITHIN`) | `exit` | `exit none` / `exit <code>` + `output bytes <n>` |
| `tests/run_cli.rs` `wait_raw` (was `READY_WITHIN`) | `raw` | `start receipt <bool> exited <bool>` |
| `tests/run_cli.rs` (was :262) | `claude-child` | `claude-child line <bool> role bytes <n>` |
| `tests/cli_instance_state.rs` (was :220, unix) | `stat` | `stat <first char>` |
| `tests/contract_diag_schema.rs` (was :244) | `raw` | `start receipt <bool> exited <bool>` |
| `tests/contract_diag_schema.rs` (was :264) | `exit` | `exited <bool>` |

`<kinds>` is the receipt's `kind` codes in first-seen order. No line carries file or screen content. The dir is under the temp dir,
outside every `target/e2e-home/**/diagnostics/` (obs-plan §9 G4).

## Remove-the-guard pair (temporary control test, removed after; `grep -rl zz_root_watch_control tests` prints nothing)

The control: `zz_root_watch_control` in `tests/run_cli.rs` waited through `Watch::start("control")` on a predicate that is never
true. It noted `elapsed s <n>` and called `deadline_check(started + WITHIN, "the control never became true")`.

Command: `cargo nextest run --features viola/fake-agent --profile mutants -E 'test(=zz_root_watch_control)'`, this Windows host,
2026-09-27, both runs just before 19:04:58Z. Each `WITHIN` edit was confirmed on disk (`grep -n 'from_secs' tests/support/watch.rs`
→ line 14) before its run.

| `WITHIN` | nextest | test output | `viola-root-watch/zz_root_watch_control.control.report` |
|---|---|---|---|
| **7 s (the guard)** | `FAIL [7.018s]` | `the control never became true` / `watch report:` / `elapsed s 0` … `elapsed s 7`: the report is in the message | kept: `elapsed s 0` … `elapsed s 7` (8 lines) |
| 12 s (past the kill) | `TERMINATING [> 10.000s]` → `TIMEOUT [10.010s]` | only `(test timed out)`: **the output is lost** | kept: `elapsed s 0` … `elapsed s 9` (10 lines) |

- At 7 s the test's own dump always wins. Past the kill, the output is lost, but the streamed file still holds the state up to the
  kill.
- The thread name under nextest is the bare test name, so the report file is `zz_root_watch_control.control.report`.
- Then `WITHIN` was restored to 7 s (line 14 reads `Duration::from_secs(7)`), and the control and its report file were removed.
- No test process outlived either run. The host process list held only another project's `viola.exe` (`viola-lab/prototype`), which
  this run did not start.
