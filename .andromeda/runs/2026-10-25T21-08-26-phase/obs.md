# obs extract

## Relevance
partial — the chunk is security assertion work on the Windows pipe listener; obs binds only where the start sequence logs the squat refusal and the bind, where the CARRY'd budget case reads a wrapper `send-refused` line, and where new test homes feed the CI gates.

## Constraints
- The start sequence's `run.collision_check` span carries `outcome` ∈ `free|already-live|squatted-name`, and a squatted endpoint ends as one `process-exit{subject:"self", exit_code:1, detail:"squatted-name"}` in `run-<name>.ndjson` with no child spawned; obs-plan §4 Scenario: `viola run` start sequence to child spawn requires both, and whether the code already writes them on the Windows leg is research's question.
- `channel.bind` carries `endpoint_kind` (`named-pipe|unix-socket`) and `process-start{subject:"self"}` carries `endpoint_kind`; obs-plan §4 Scenario: `viola run` start sequence to child spawn requires that the endpoint name, the `pinned_bin` path and env values never appear in any span or line — the pipe name, its SDDL string and the user SID included.
- `process-exit` `detail` is a closed code catalog (13 values in `schemas/diag-line.v1.json`); obs-plan §6 Log Coverage (`detail` code catalog) requires that `squatted-name` stays the exit-1 word and that no path or pid enters a `detail` code — the chunk's scope states no code changes, so any new code (e.g. for a rejected remote client or a second-listener failure) is outside the catalog.
- The `event` field is a closed kebab-case enum; obs-plan §3 Log format JSON schema requires a Decisions Log entry for any new value, so a rejected remote client or a refused open by a principal outside the DACL gets no new `event` value in this chunk.
- `interprocess` is `OFF` in every sink and its failures are captured at viola's seam from returned `Result`s; obs-plan §6 Log Coverage (Per-module log levels) requires that a bind failure on a held name surface as viola's own `squatted-name` refusal, never as an `interprocess` record.
- A start refusal before any child writes exactly the two fixed stderr lines (`unable:` / `hint:`) through `src/human.rs` `refuse`, no path or pid; obs-plan §3 Observability Harness Contract (the `run` terminal rule) requires this for the squatted-endpoint refusal.
- The wrapper-side `send-refused` for `budget-paused` carries `refusal`, `detail`, `window`, `side:"wrapper"`, `conn`/`srv_conn` and `rpc_id`, with `corr` = the target's `events.ndjson` end offset; obs-plan §3 Trace context propagation (Send cursor on early refusals) defines that line, which the CARRY'd `send_budget_paused_is_refused_with_the_window_word_and_no_cursor` case_4 reads — whether its capture misses the line under libtest threads is research's question.

## Patterns to follow
- Refusal telemetry as one codes-only `process-exit{subject:"self", exit_code:1, detail}` at `error` (obs-plan §6 Log Coverage, Log levels mapping), the same shape `already-live` and `pinned-hash-mismatch` already use.
- Every log line through `obs_event!` under `#[instrument(skip_all, name = "<area>.<operation>", fields(...))]` with an explicit allow-list (obs-plan §4 Span / Trace Coverage); a new test-only span or line in `viola-channel` follows the `channel.<operation>` naming.
- Every integration-test viola home under `target/e2e-home/`, so G2, G4, the secret scan and the `diag-<os>` upload cover it on `windows-2025` (obs-plan §9 CI Integration, Integration tests row).
- Unit tests that assert emitted lines parse them from an in-memory `Vec<u8>` writer against `schemas/diag-line.v1.json` (obs-plan §9 CI Integration, Unit tests row) — the form a fix of the CARRY'd budget case's log capture takes if it is a capture fault.

## Anti-patterns to avoid
- Logging the pipe name, the SDDL text, a SID, a peer pid or any path in a span field, `detail` code or line (obs-plan §11 Obs Anti-Patterns § PII Scrubbing; § Spans / Traces).
- Adding an `event` value or a `process-exit` `detail` code for the remote-client reject or the second-listener failure without a Decisions Log entry (obs-plan §11 Obs Anti-Patterns § Logs).
- Closing the CARRY by a retry or a re-run instead of a cause (obs-plan §11 Obs Anti-Patterns § SLO: no retry-once policies for flaky telemetry assertions).

## Contract bindings
- obs ↔ tests: the `squatted-name` exit-1 line and the start refusal are asserted through test-plan §3's log format, which obs-plan §3 Log format JSON schema reproduces as binding; `tests/path1_run_start.rs` and `tests/cli_instance_state.rs` read them.
- obs ↔ security: the codes-only refusal and the never-log-the-endpoint rule bind to security-plan §Error Handling and §Authentication & Authorization (IPC access control, Windows); obs-plan §8 PII Scrubbing & Compliance keeps `endpoint` off every home-level line.
- obs ↔ CI: new Windows test homes are inside G2 / G4 / the secret scan through obs-plan §9 CI Integration's `target/e2e-home/` scope.

## Acceptance criteria contributions
- A squatted pipe name at `viola run` start writes exactly one `process-exit{subject:"self", exit_code:1, detail:"squatted-name"}` in `run-<name>.ndjson`, no `process-start{subject:"claude-child"}`, and no line carrying the pipe name, SDDL text or a SID (per obs-plan §4 Scenario: `viola run` start sequence to child spawn).
- Every line the chunk's tests produce on `windows-2025` passes G4 against `schemas/diag-line.v1.json` with the `process-exit` `detail` enum unchanged, and G2 counts zero panics (per obs-plan §9 CI Integration).
- The CARRY'd budget case_4 finds its wrapper-side `send-refused{refusal, detail, window, side:"wrapper"}` line on three plain `cargo test` runs, or its capture is fixed with the cause named (per obs-plan §3 Trace context propagation).
