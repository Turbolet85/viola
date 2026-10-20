# obs extract

## Relevance
partial — the chunk changes ledger probes/oracles in `crates/viola-agent-claude/src/ledger.rs` and Run B's pastes in `src/cmd/verify.rs`, and may touch the `send` confirm path; obs binds to `verify`'s spawn-pair lines, the confirmed-send line set and the closed log vocabulary, not to the row semantics themselves.

## Constraints
- `viola verify`'s logged shape is fixed at eight `process-start` / `process-exit` spawn pairs (`version-probe`, `verify-probe`, one `agents-probe`, five `verify-pty-probe`); Run B's six compiled pastes add no pair or subject, and a changed or reordered paste (e.g. a fix to the `tag-escaping`, `paste-newline`, `/compact` or `/clear` paste) must keep that count and the closed `subject` enum unchanged (per obs-plan §4 Edge flows → `verify` / `plugin install`).
- No `verify` line may carry the probe command, a paste or prompt text, a status line payload, a session id, row text or a path; `verify`'s stdout step lines stay `[NN/MM] <row id> <row words>  pass|fail` with `MM` = `LedgerRow::relied()` (24 on Unix) and the last line `stamped <version>  N pass  N fail` (per obs-plan §4 Edge flows → `verify` / `plugin install`).
- The hidden `hook <event> --capture` and `hook statusline --capture` arms stay uninstrumented by design (no `VIOLA_*`, no obs init, no role or detail file, no `hook-invoked` / `hook-decision`); a probe fix that reads captures differently must not add instrumentation to them (per obs-plan §4 Edge flows → `verify` / `plugin install`).
- If the `send` confirm path changes (the exact match, the `/clear` new-session or `/compact` same-session post-condition), the line set stays `send-issued{text_bytes}` → `hook-invoked{hook_event}` → `send-confirmed{confirmed, duration_ms}` or `send-refused{detail:"no-prompt-submitted"}` on expiry, with `run.confirm_window`'s `window_ms` the send's own bound (40 000 for `/compact`, else 10 000) and no new event, detail or field (per obs-plan §4 Scenario: Confirmed `send` (CL-1) from driver to readback).
- `text_bytes` on `pty.paste_write` and `send-issued` is the typed text's length (every CR LF and other CR as one LF, trailing CR/LF removed); a `paste-newline` resolution that changes how an inner LF is typed must keep this definition or amend it explicitly (per obs-plan §6 Log Coverage, Additive field catalog).
- The `process-exit` `detail` enum (13 values in `schemas/diag-line.v1.json`), `parse-rejected.detail` and the `subject` enum are closed; a row fix that needs a new code (e.g. a new refusal cause for an unrelied local command) is a schema and Decisions Log change, never an ad hoc string (per obs-plan §6 Log Coverage, `detail` code catalog).
- A 2.1.302 stamp's effect on `run` is observed only through `cli_verified` on `process-start{subject:"claude-child"}` read inside `run.version_gate` › `state.ledger_read`; stamp contents and the stamps path never appear in a line (per obs-plan §4 Edge flows → Capability ledger).

## Patterns to follow
- New or changed functions on the verify / confirm path use `#[instrument(skip_all, fields(..))]` and log only via `obs_event!` with typed codes-only fields; existing span names `cli.verify`, `cli.verify_step`, `state.ledger_write`, `run.confirm_window` are reused, not renamed (per obs-plan §4 Edge flows → `verify` / `plugin install`; §11 Spans / Traces).
- `verify` logs its spawns at the call site, with `run_bounded` / `run_capped` unlogged; any added probe step reuses an existing subject (as Run E reuses `verify-pty-probe` and Run B's agents read reuses `agents-probe`) (per obs-plan §6 Log Coverage, Boundary-call wrappers → Child / shell spawns).
- Recorded `fixtures/claude/2.1.302/` captures are scrub-required data, not a telemetry source: obs adds no fixture content (per obs-plan §8 PII Scrubbing, Data classification rules → Recorded fixtures).
- Stall detection for the folded macOS readiness flake reads the heartbeat/snapshot through the harness status, not a per-tick line (per obs-plan §3 Heartbeat ticks).

## Anti-patterns to avoid
- NEVER derive telemetry from screen content: a re-measured oracle may read the vt100 screen for a gate outcome, but no log field or event is built from screen text (per obs-plan §11 Telemetry Strategy).
- NEVER log send text, paste text or `last_assistant_message` in any home-level file, and NEVER add an `event` value or rename a field without a Decisions Log entry (per obs-plan §11 PII Scrubbing; §11 Logs).
- NEVER add a retry-once policy for a flaky assertion: the macOS flake's attempt-2 green is no fix (per obs-plan §11 SLO).

## Contract bindings
- obs ↔ tests: CI runs `viola verify` only against the fake agent at the recorded version 2.1.287 (`tests/cli_verify.rs`, harness `boot` step 4, the `stamped_home` fixture); a changed probe must keep that replay green, and verify's home-level lines under a `VIOLA_NAME` fall under G2, G4 and the secret scan (per obs-plan §4 Edge flows → `verify` / `plugin install`; §9 CI Integration).
- obs ↔ tests: G4 checks every home-level line against `schemas/diag-line.v1.json` (obs-owned schema, tests-owned check body) (per obs-plan §9 CI Integration, Gate commands → G4).
- obs ↔ tests: harness readiness/stall reading (`watch.rs`'s `snapshot` / `beat`) binds to the heartbeat contract and test-plan §3 Status endpoint shape (per obs-plan §3 Heartbeat ticks).
- obs ↔ security: recorded fixtures and probe captures follow the NEVER-log floor and the `--record` scrub (per obs-plan §8 PII Scrubbing, Data classification rules).

## Acceptance criteria contributions
- An uninterrupted `viola verify` under a `VIOLA_NAME` writes exactly eight spawn pairs into `cli-<name>.ndjson`, every `subject` in the closed enum, and no line carries a paste text, payload, session id or path (per obs-plan §4 Edge flows → `verify` / `plugin install`).
- Every home-level line written by the changed verify and `send` paths in the fake-agent tests passes G4 against `schemas/diag-line.v1.json` with no new `event`, `subject` or `detail` value unless a Decisions Log entry and schema change land with it (per obs-plan §9 CI Integration, Gate commands → G4).
- If the confirm path changes, a confirmed `/clear` and a confirmed `/compact` each still log `send-confirmed{confirmed:true}`, and `run.confirm_window`'s `window_ms` reads 10 000 and 40 000 respectively; whether the code already does so on the changed path is research's question (per obs-plan §4 Scenario: Confirmed `send` (CL-1) from driver to readback).
