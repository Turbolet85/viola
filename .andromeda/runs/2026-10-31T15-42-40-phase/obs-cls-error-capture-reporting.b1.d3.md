# Classified — obs · error-capture-reporting.b1

## Answers
error-capture-reporting#1.1 · no · the panic hook's separate `event:"panic"` line and raw frames are neither written nor moved by the chunk; its `internal-error` line is the process-exit through `internal_error_exit_line`, which the hook does not write
error-capture-reporting#1.2 · yes · the chunk re-reads that `revive`'s strict-modes check runs before its log opens, and every pre-log refusal's no-line rule rests on the `check_diagnostics` open gate
error-capture-reporting#2.1 · yes · the catch-site map binds: `revive`'s start arm and its refused checks before its log, `revive --list` as `cli`, `internal-error` via `internal_error_exit_line` for `Run` and `Cli` only, `mcp` writing no self `process-exit`, and `no-session` in `cli-<VIOLA_NAME>.ndjson`
error-capture-reporting#3.1 · yes · the pre-log refusals the chunk gives codes or no-line rules (`revive` start instance and home-tree checks, `mcp` home check, `link`/`unlink` instance check, the six frame verbs' `refuse_untrusted`, `verify` home tree, `run` and `revive` home tree) all sit in this window
error-capture-reporting#4.1 · no · the vt100 feed thread's `parse-rejected` lines; the scope puts the feed thread under Not needed and no exit-cause code arises there
error-capture-reporting#4.2 · no · the passthrough of `hint:` and other words from the child's terminal output on the vt100 feed path; the scope puts that path under Not needed and the chunk's `hint:` lines are viola's stderr, not passthrough
error-capture-reporting#5.1 · yes · `viola mcp` frame tools refuse on `check_instance` with `instance-unreachable` and `detail: null` as the `isError` code, the only record, in `crates/viola-mcp/src/tools.rs`, which the chunk reaches whole
error-capture-reporting#5.2 · yes · the stdin cut before `initialize` is an exit-1 cause of `mcp` whose one record is `parse-rejected`, not a process-exit form, so the exit-1 catalogue must name it or its absence
error-capture-reporting#5.3 · no · the witness test of the same pre-initialize cut re-checks the stdin reader and streams the chunk does not edit; the test's assertions are not the moved document, so it is a re-check every change owes
error-capture-reporting#6.1 · yes · `sanitised_fault` fixes which peer codes read which `message` and `data`; the new codes cross `of_reply` in `crates/viola-mcp/src/result.rs` (reached whole) and the channel error read in `viola-channel/src/client.rs`
error-capture-reporting#6.2 · yes · the new codes must pass `viola_core::is_code_word`, which `reply_of` and `of_reply` apply, and `crates/viola-core/src/lib.rs` is reached whole
error-capture-reporting#6.3 · yes · the chunk's `revive --json` gap names `batch-script-child`, an `AgentError` variant with a fixed `Display` message that the new `--json` document must carry
error-capture-reporting#6.4 · yes · `NOT_ERROR_BODIES` in `tests/contract_error_bodies.rs`, a census the chunk edits, names these exit-2 writers as not error bodies, and the census must keep them recorded as the new documents land
