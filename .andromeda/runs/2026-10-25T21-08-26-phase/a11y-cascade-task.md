# Task — a11y · 2026-10-25-windows-endpoint-admission

## The chunk
Windows named-pipe admission in `viola-channel`: new Windows CI assertions that the wrapper's listener rejects a remote (network) pipe client, that a second listener on a name already held fails (first-pipe-instance), and that an open by a principal outside the owning user and SYSTEM is denied, where a Windows runner can measure it.
It re-verifies, never rebuilds, what HEAD already asserts: the protected user-and-SYSTEM DACL read back, non-inheritable handles on both ends, and the `viola run` squatted-name exit-1 start refusal (its test-file coordinate included).
It re-verifies the cause of, and closes, the flaky `send_budget_paused_is_refused_with_the_window_word_and_no_cursor` case_4 under plain `cargo test` (three green runs, or its log capture fixed).
No wire contract, exit code or refusal word changes (`squatted-name`, exit 1 stand); no boundary is widened.

## Reached files
- crates/viola-channel/src/server.rs · edits the doc comment naming the `PIPE_REJECT_REMOTE_CLIENTS` default
- crates/viola-channel/src/server/win.rs · edits its unit tests (`owner_only_sddl_is_protected_user_and_system`, `bound_pipe_dacl_reads_back_protected_user_and_system`, `pipe_handles_on_both_ends_are_not_inheritable`, plus the new remote-reject, first-instance and DACL-effect cases)
- crates/viola-channel/src/endpoint.rs
- tests/security_negatives_channel.rs
- src/cmd/run.rs · edits `refuse_squatted`
- tests/path1_run_start.rs
- tests/cli_instance_state.rs
- src/run/send/budget.rs · edits the test `send_budget_paused_is_refused_with_the_window_word_and_no_cursor` and its log-capture helper

## Needs
- The wrapper IPC channel (the named-pipe endpoint) is the chunk's main surface; the a11y scope classes it as not-assertable (machine-to-machine, no UI surface), so the admission work itself owes no a11y assertion.
- Pipe admission gates automation clients of the channel only: it must never block, refuse or delay a human keystroke in `viola run`; the human input path through the PTY stays outside every admission check.
- The `viola run` squatted-name start refusal is a human-mode CLI refusal: if `src/cmd/run.rs` is touched it keeps `unable <reason> <detail>` on stderr, `hint:` as the last stderr line, and its typed exit code 1.
- That refusal's text is a pinned fixed CLI string (trycmd or literal): the chunk changes no copy, so its pinned form stays green unchanged.
- The refusal word stays beside the plain word `unable`, is never coloured, carries no SGR under non-TTY, `NO_COLOR`, `TERM=dumb` or `--json`, and is never a stack trace or a bare status number.
- Any new diagnostic for a failed bind, a squatted name or a rejected client goes to the instance's `diagnostics/`, never to the terminal: during `viola run` viola writes no terminal byte of its own.
- The budget-paused `send` refusal (path P2, send refused) keeps its window word and typed exit code: fixing the flaky case's log capture changes no refusal text or code.
- Zero-flakiness rule (`retries: 0`, `#[ignore]` banned, a flake keeps the chunk red until its root cause is fixed) applies to the flaky budget case and to every new Windows test: none is retried, skipped or ignored to reach green.
- New Windows tests run inside the per-OS `test` job's single `coverage` suite on `windows-2025`, under `gate --require coverage,doctest` with no skip allowed; no new `suite` value and no second driver outside `scripts/agent-run.* run`.
- A DACL-effect check that a Windows runner cannot measure without a second interactive user goes to the Windows-only live measurements entry, not to a manual-only or skipped test.

## Not needed
- `viola list` board rules: the `CAPTION` literal and caption order, amber `DIALOG` / dim stale / bold NAME SGR, the unwrapped separator, NAME truncation and the owed dim-row witness; the chunk does not touch `list`.
- Path P4 wheel cases: `pause` / `release`, the `human-typing` refusal, focus, mouse and resize sequences and the Windows win32-input-mode mouse report (F-W3); the chunk changes no wheel behaviour.
- Path P3 `answer` and the `unverified-cli` withholding; path P6 `wait` / `last` / `verify` step lines and `waiting:` line.
- Cognitive timeouts: the chunk adds and changes no time bound (wait timeout, MCP `wait` cap, dialog hold, readiness gate, confirmation windows, status-line deadline, agents read, budget pause).
- `--help` group labels and the owed `handoff:` row; the `viola revive --json` owed document; `viola plugin install` output.
- Manual NVDA / VoiceOver / Orca passes and any WCAG conformance claim; the vendor `claude` TUI content (never parsed).
- The deferred 0.2.0 frontend: ARIA landmarks, keyboard navigation and visual design for a page.
- The `supply-chain` red (RUSTSEC-2026-0386 tokio lock bump): owned by its own route entry, no a11y bearing.
