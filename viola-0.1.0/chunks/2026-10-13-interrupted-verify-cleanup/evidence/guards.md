# Remove-the-guard controls — 2026-10-13-interrupted-verify-cleanup

The run's journal line `remove-the-guard controls fired` dates them
(`.andromeda/runs/2026-10-13T18-48-52-implement/journal.md`). Each guard was neutralised in the worktree, the stop
table run alone through `bash scripts/agent-run.sh run --integration --filter 'test(/verify_stopped_by_a_signal/)'`,
and the file restored byte-for-byte in a `finally`, its SHA-256 read back equal. Driver and logs stayed in the session
scratchpad (`$TMPDIR/{scratchpad}/guards.py`, `$TMPDIR/{scratchpad}/guards/`).

| guard neutralised | the edit | reading (both cases, `case_1_sigint` and `case_2_sigterm`) | restored |
|---|---|---|---|
| the handler's install, `src/cmd/verify.rs` `measure` | `stop::install();` commented out | red, 0 passed 2 failed: `tests/cli_verify.rs:880` `left: None right: Some(1)` — the signal's default action ended verify | yes |
| the stop read before each interactive run, `src/cmd/verify/typed.rs` `measure` | `if (inputs.stop)()` → `if false` | red, 0 passed 2 failed: `tests/cli_verify.rs:919` the log shape holds four `verify-pty-probe` pairs (Runs C and D started) where two are asserted | yes |
| the stop read in every wait, `src/cmd/verify/typed.rs` `poll_until` | `if stopped()` → `if false` | red, 0 passed 2 failed: `tests/support/watch.rs:54` `viola never exited` — Run B kept waiting for its Stop past the case's bound | yes |

With every guard in place the same table read green in the P2 block run (entry 7: `cli_verify` 72 passed, the two
stop cases among them).
