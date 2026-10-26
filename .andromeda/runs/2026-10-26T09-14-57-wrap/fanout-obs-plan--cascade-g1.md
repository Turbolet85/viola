proposals:
  - detector: D-cascade
    severity: warning
    file: .andromeda/obs-plan-detail/obs-scope-summary.md
    near: 123
    section: '## 1. Obs Scope Summary'
    change: 'The OS-APIs entity reason should read: Instrument viola call sites (bind, connect, lock, rename, kill-fallback use, and the client peer-pid read and sysinfo start-time read of the server check, whose outcome is logged by the CLI and the hook only on a mismatch as `server-verify-failed`; MCP writes no line), not the OS internals.'
    sidecar: 'obs-scope-summary OS-APIs entity: the client server check (peer pid via `peer_creds`, start time via sysinfo) added to the instrumented call sites (chunk 2026-10-26-server-verification-before-any-frame).'
    rationale: 'Report Symbols / APIs: `Client::server_pid` reads `peer_creds().pid()` (Linux `SO_PEERCRED`, Windows `GetNamedPipeServerProcessId`) and `verify_server` reads the start time through `process_start_time`; Schema / config: the mismatch is logged as `process-exit` / `hook-decision` `server-verify-failed`, MCP writes no line.'
  - detector: D-cascade
    severity: warning
    file: .andromeda/obs-plan-detail/obs-scope-summary.md
    near: 267
    section: '## 1. Obs Scope Summary'
    change: 'The mirror line should read: `hook-invoked`, `hook-decision`: `corr` = `dialog_id` for dialog hooks once the wrapper has assigned one, otherwise null (a dialog hook that fails open before any `hook.dialog` response, the server-verification mismatch included, logs `corr` null).'
    sidecar: 'obs-scope-summary log-fields mirror: dialog-hook `corr` qualified to null when no `dialog_id` was assigned (the server-verification fail-open writes `corr:null`); binds with test-plan §3 Log format.'
    rationale: 'Report Schema / config: on the dialog path a mismatch writes `hook-invoked{corr:null}` then `hook-decision{corr:null, decision_emitted:false, detail:"server-verify-failed"}`, so a dialog hook now writes `corr` null, against «`corr` = `dialog_id` for dialog hooks». The block is the verbatim mirror of test-plan §3 Log format, so both sides bind.'
  - detector: D-cascade
    severity: warning
    file: .andromeda/obs-plan-detail/obs-scope-summary.md
    near: 327
    section: '## 1. Obs Scope Summary'
    change: 'The Dialog → `answer` required log fields list should name `server-verify-failed` beside `unknown-dialog` (exit 13) and `unverified-cli` (exit 12): exit 21 on CLI `answer` (`process-exit`, `during:"connect"`) and the dialog hook fail-open `hook-decision` (`corr` null, `decision_emitted:false`), with no pid, start time, path or endpoint.'
    sidecar: 'obs-scope-summary Dialog → answer path: `server-verify-failed` added to its refusal / detail codes (chunk 2026-10-26-server-verification-before-any-frame).'
    rationale: 'Report Symbols / APIs: `answer` goes through `client::connect` inside `answer.client`, a mismatch exit 21; `src/cmd/hook.rs` `ask` shares the hook `connect`; Schema / config: this build first writes `server-verify-failed` on `process-exit` and `hook-decision`.'
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/obs-scope-summary.md
    near: 454
    section: '## 1. Obs Scope Summary'
    change: 'Add, after the `ConPTY vs openpty` item, the item: peer-pid read for the client server check, Linux `SO_PEERCRED` vs Windows `GetNamedPipeServerProcessId` vs macOS none (`SERVER_PID_REPORTED` false, so the verdict is `Matches` with no comparison and no `server-verify-failed` line is written there, the recorded gap; the server-identity negatives compile out on macOS and its cases are owed to "Unix endpoint and home hardening").'
    sidecar: 'obs-scope-summary multi-platform trigger: the peer-pid platform difference and the macOS recorded gap added (F8).'
    rationale: 'Task New facts F8; report Symbols / APIs (`SERVER_PID_REPORTED` false on macOS; `peer_pid_reported == false` gives `Matches` with no comparison) and Retired (macOS stays on the recorded gap, owned by "Unix endpoint and home hardening").'
    fact: F8
  - detector: D-cascade
    severity: warning
    file: .andromeda/obs-plan-detail/obs-scope-summary.md
    near: 485
    section: '## 1. Obs Scope Summary'
    change: 'Append to the exit-21 / exit-1 bullet: As landed, the server-verify fail is written on Linux and Windows as `process-exit{exit_code:21, detail:"server-verify-failed", during:"connect"}` in `cli-<name>.ndjson` by `send`, `wait`, `last`, `answer`, `pause` and `release`, distinct from `instance-dead`, with no pid, start time, path or endpoint (chunk 2026-10-26-server-verification-before-any-frame); macOS writes none (no peer pid, the recorded gap).'
    sidecar: 'obs-scope-summary Founder Direction 4 trigger: as-landed qualifier for the written exit-21 `server-verify-failed` cause.'
    rationale: 'Report Schema / config: this build is the first that writes `server-verify-failed` on `process-exit` exit 21; Symbols / APIs: `Unreachable::detail()` gives `instance-dead` | `server-verify-failed`; Coverage: no pid, path or endpoint (asserted).'
  - detector: D-cascade
    severity: warning
    file: .andromeda/obs-plan-detail/telemetry-strategy.md
    near: 30
    section: '## 2. Telemetry Strategy'
    change: 'Append to the Vector 4 row: The server-verification failure (a serving process that is not the recorded wrapper) writes one `hook-decision{detail:"server-verify-failed"}` at WARN in `hook-<name>.ndjson` as landed, exit 0, empty streams, no frame (Linux and Windows; chunk 2026-10-26-server-verification-before-any-frame).'
    sidecar: 'telemetry-strategy trigger map, Vector 4 row: the written server-verification fail-open cause added as landed.'
    rationale: 'Report Schema / config (`hook-decision{detail:"server-verify-failed"}` at WARN into `hook-<name>.ndjson`) and Outcome (the hook `stop`, `pre-tool-use`, `permission-request` against the impostor: exit 0, empty streams, no frame, `< 1.0 s`).'
note: 'The sibling path entries (Confirmed send, wait / last, Human takes the wheel) were not picked though `server-verify-failed` exit 21 now reaches them too; the §4 rule that a dialog hook writes `hook-invoked` only after the `hook.dialog` response (D-07, span-trace-coverage) is reached by F3; the log-fields mirror edit binds with test-plan §3 Log format.'
