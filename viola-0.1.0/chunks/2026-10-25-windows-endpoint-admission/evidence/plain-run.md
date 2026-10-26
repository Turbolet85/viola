# The plain `cargo test --bin viola` reading (plan step 8, gate entry 10)

Fired in /implement's first full block run, 2026-10-25T23:56Z, run dir `.andromeda/runs/2026-10-25T23-49-29-implement/`,
Linux dev host, base `b6740d59` plus this chunk's tree. The entry asserts the two `ok` lines and no exit (`inputs#I5`
fork 4: plain `cargo test` is not a supported runner; nextest is the gate).

```
 10 unit        green · exit 101 · 6.32s · 102082 B → 10.log · cargo test --bin viola
```

The two asserted lines, from its log:

```
test run::send::budget::tests::send_budget_paused_is_refused_with_the_window_word_and_no_cursor::case_4_both_name_five_hour ... ok
test tests::scoped_capture_sees_a_line_whose_callsite_another_thread_registered_first ... ok
```

The run's failing list, verbatim (the registry prefix respelled `~`):

```
failures:

---- obs::tests::log_config_rejection_writes_one_warn_line stdout ----

thread 'obs::tests::log_config_rejection_writes_one_warn_line' (1918131) panicked at src/obs.rs:714:9:
assertion failed: lines[0].get("instance").is_none()

---- run::gate::tests::feed_panic_writes_one_parse_rejected_line_until_the_size_changes stdout ----

thread '<unnamed>' (1918339) panicked at ~/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/vt100-0.16.2/src/screen.rs:730:22:
attempt to subtract with overflow

thread 'run::gate::tests::feed_panic_writes_one_parse_rejected_line_until_the_size_changes' (1918333) panicked at src/run/gate.rs:438:9:
assertion `left == right` failed
  left: 2
 right: 1


failures:
    obs::tests::log_config_rejection_writes_one_warn_line
    run::gate::tests::feed_panic_writes_one_parse_rejected_line_until_the_size_changes

test result: FAILED. 1037 passed; 2 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.26s
```

## Known cause, no owner (`inputs#I5` fork 4)

Both reds are among the three the founder recorded with no owner. Each shares process-global state with other tests
of the one libtest process, and is written for nextest's process-per-test (research.md, Measurements):

- `run::gate::tests::feed_panic_writes_one_parse_rejected_line_until_the_size_changes` — its process-global subscriber
  (`src/run/gate.rs:429`, the binary's one global install) also takes another test's feed-thread `parse-rejected` line
  (`left: 2 right: 1`); 3 of 3 at P3, red here too.
- `obs::tests::log_config_rejection_writes_one_warn_line` — the process context (`ProcessCtx`) another test sets
  leaves `instance` on its line; 2 of 3 at P3, red here.
- `cmd::verify::stop::tests::install_turns_the_signal_into_the_stop_with_the_process_alive::case_2_sigterm` — the
  process's signal handler; 2 of 3 at P3, `ok` in this run.

Under nextest (gate entry 7, `run --unit`) all three read green.
