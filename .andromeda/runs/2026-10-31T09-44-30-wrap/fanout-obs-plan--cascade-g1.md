proposals:
  - detector: D-cascade
    severity: warning
    file: .andromeda/obs-plan-detail/obs-scope-summary.md
    near: 310
    section: '## 1. Obs Scope Summary'
    change: '- **Required log fields:** `event`, `process`, `instance`, `corr` (the JSON-RPC `id` on channel lines; the send `cursor` on send lines), `from` (self-reported, not an identity), `confirmed`, `refusal`, `detail` (`input-not-ready` / `no-prompt-submitted` / `control-character` / `empty-text` / `turn-running`; on a `channel-response` answering `-32602`, `invalid-params` or `unsupported-version`), `duration_ms`. Sent text never appears.'
    sidecar: 'obs-scope-summary send path: the `detail` list gains the two closed `channel-response` words a `send` can answer on `-32602`.'
    rationale: 'Report Changes, Symbols / APIs: `ResponseLine`''s `Drop` writes `detail = fault.and_then(ProtocolError::cause)` on both sides; Schema / config closes the field to three words at `schemas/diag-line.v1.json:42`; this list spans channel lines (its `corr` clause says so) and enumerates `detail` values without them.'
    basis: schemas/diag-line.v1.json:42
  - detector: D-cascade
    severity: warning
    file: .andromeda/obs-plan-detail/obs-scope-summary.md
    near: 336
    section: '## 1. Obs Scope Summary'
    change: '- **Must-trace spans:** `run.wheel_transition` › `channel.request(send)` refused › `pause.client` / `release.client` › `channel.request(pause|release)`. Log events: `send-refused` and `channel-request` / `channel-response` for `pause` and `release`. `release-from-driver` is logged as its own event (Founder Direction 4), admitted by `schemas/diag-line.v1.json`; the same word is also a `detail` value of `channel-response`, on the `-32602` a `release` from a driver answers, and does not replace the event.'
    sidecar: 'obs-scope-summary wheel path: `release-from-driver` named as a `channel-response` `detail` word as well as its own event.'
    rationale: 'Report Changes, Symbols / APIs: `ProtocolError::cause` gives `release-from-driver` for one of the three `-32602` faults; Schema / config: the only schema edit is `channel-response`''s `detail` at `schemas/diag-line.v1.json:42`, so the event value stands and the one spelling now has two meanings on this path.'
    basis: schemas/diag-line.v1.json:42
  - detector: D-cascade
    severity: warning
    file: .andromeda/obs-plan-detail/obs-scope-summary.md
    near: 354
    section: '## 1. Obs Scope Summary'
    change: '  - A scrub rule on `channel-request` / `channel-response`: method, `id` / `corr`, result class (`ok` / `refusal` / `error` code), `sender` version and, on a `channel-response` answering `-32602`, the closed `detail` word (`unsupported-version` / `invalid-params` / `release-from-driver`) only, never `params` or result bodies.'
    sidecar: 'obs-scope-summary Vector 1 trigger: the channel scrub rule''s field list admits `channel-response`''s closed `detail` word.'
    rationale: 'The rule says «only» and the session adds a field to the line: report Changes, `ResponseLine`''s `Drop` writes `detail`, closed by `schemas/diag-line.v1.json:42`; Coverage of new surfaces reads `channel-response.detail` «PII no param value or name», so the scrub holds and the list is what moves.'
    basis: schemas/diag-line.v1.json:42
  - detector: D-cascade
    severity: warning
    file: .andromeda/obs-plan-detail/telemetry-strategy.md
    near: 12
    section: '## 2. Telemetry Strategy'
    change: '| Errors | Panics (a one-line `event:"panic"` record), typed thiserror 2.0.20 errors with fixed `Display`, exit codes plus distinct `detail` codes on `process-exit` (the `mcp` role writes no self `process-exit`, Section 7) | No external reporter: security says "no outbound network calls". A hand-written `std::panic::set_hook`, plus `std::panic::catch_unwind` at the main-thread catch site of every role, around `hook` dispatch, in the `run` PTY pump and handle-wait threads, and around the vt100 feed (Section 7) |'
    sidecar: 'telemetry-strategy Errors row: `process-exit` `detail` codes carved out for the `mcp` role, which writes no self line.'
    rationale: 'Report, Spec claims disproved: `src/obs.rs:232` `internal_error_exit_line` writes for `Run` and `Cli` only, and `ObsEvent::ProcessExit` shows no self `process-exit` on the `mcp` role; the row states the `process-exit` codes for every role, unqualified.'
    basis: src/obs.rs:232
  - detector: D-cascade
    severity: warning
    file: .andromeda/obs-plan-detail/telemetry-strategy.md
    near: 11
    section: '## 2. Telemetry Strategy'
    change: '| Structured Logs | Every boundary call, state transition, refusal, fault/heal and process lifecycle (the `mcp` role writes no self `process-exit`, Section 7), as the closed `event` enum (Section 3) | **The primary signal.** tracing-subscriber 0.3.23 JSON-per-line, one file per process role under `<home>/diagnostics/`, merged with `events.ndjson` by `agent-run logs` |'
    sidecar: 'telemetry-strategy Structured Logs row: «every process lifecycle» carved out for `mcp`''s absent self `process-exit`.'
    rationale: 'Same measurement as the Errors row, report Spec claims disproved, pattern `mcp-exit-line`: no self `process-exit` is written on the `mcp` role, so «every … process lifecycle» overstates that role''s end.'
    basis: src/obs.rs:232
    dependent-of: D-cascade
note: 'obs-scope-summary#22.3 stays true and states no exit, F6''s exit 1 being error-capture-reporting''s to place; obs-scope-summary#20.5 gives 2.5 / 2.4 / 2.4 / 2.4 / 3.3 ms as `max` at its 2026-10-26 gate file while the report reads the same five as `mean`, a label difference the report does not resolve and no value is proposed for it; D-obs-instrumentation, D-obs-stack and D-obs-pii fired nothing on this report.'
