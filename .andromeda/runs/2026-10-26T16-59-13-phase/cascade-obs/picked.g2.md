### obs-scope-summary#19.2 · values · scope-surfaces · ## 1. Obs Scope Summary · lines 171–171
- **Frontend instrumentation hooks:** N/A. The terminal carries the child's screen.
### obs-scope-summary#19.3 · values · scope-surfaces · ## 1. Obs Scope Summary · lines 172–172
- **Exporter:** A file sink only, to `diagnostics/run-<name>.ndjson` (tests excerpt, Log format). The terminal is forbidden: "`run` writes nothing to the terminal while the child runs" (arch Observability Hints).
### obs-scope-summary#19.4 · values · scope-surfaces · ## 1. Obs Scope Summary · lines 173–173
- **Service identity:** as for the short-lived verbs, with `process="run"` and `instance=<ViolaName>`.
### obs-scope-summary#19.5 · values · scope-surfaces · ## 1. Obs Scope Summary · lines 174–177
- **Notes:**
  - Rust's default panic hook prints to stderr, which is the child's terminal. It must be replaced before the child spawns.
  - The heartbeat file, touched every 1 s, is the existing liveness tick (arch Standard Contracts, Heartbeat).
  - Child exit is detected on the process handle, so `process-exit` for the child must come from the handle wait, not from reader EOF (arch Stack; brief §4.1 Windows exit detection).
### obs-scope-summary#20.1 · values · scope-surfaces · ## 1. Obs Scope Summary · lines 183–183
- **Backend instrumentation hooks:** `tracing` spans around stdin parse → channel call (`hook.dialog` / `hook.event`) → decision emit, through the tracing-subscriber JSON logger. `hook-invoked` / `hook-decision` lines are required, and panics are caught with `catch_unwind` and emitted as one `level:"ERROR", event:"panic"` line (tests excerpt, Log format, Agent parsing).
### obs-scope-summary#20.3 · values · scope-surfaces · ## 1. Obs Scope Summary · lines 185–185
- **Exporter:** A file sink only, to `diagnostics/hook-<name>.ndjson`. stderr is forbidden on both. stdout carries only the decision body for `viola hook <event>`, and for `viola hook statusline` only the user's status-line command's output (arch Standard Contracts, Hook contract); the two hidden `--capture` arms (`verify`'s probe capture) write no sink and print nothing, except `--answers`' probe decision body. A failed log write is swallowed, and the hook still exits 0 (tests excerpt, Log format Constraints).
### obs-scope-summary#20.4 · values · scope-surfaces · ## 1. Obs Scope Summary · lines 186–186
- **Service identity:** as for the short-lived verbs, with `process="hook"`. If `VIOLA_NAME` is missing, `instance` is null and the hook is a silent no-op (tests excerpt, E1 R5 unwrapped session hooks; Coverage Triggers, Vector 4 "missing `VIOLA_NAME`").
### obs-scope-summary#20.5 · values · scope-surfaces · ## 1. Obs Scope Summary · lines 187–190
- **Notes:**
  - Many hook processes can append to the same `hook-<name>.ndjson` at once. Each line must be one `write` call on an append-mode handle (arch Stack, atomic-write-file / `OpenOptions::append`).
  - The latency budget is `max < 1.0 s` per hook (tests excerpt, Quality Gates). Logger init must not add blocking I/O beyond one file open and append. `viola hook statusline` is outside that budget: its shell-out of the user's command is bounded by the 5 s PROVISIONAL `STATUSLINE_DEADLINE`, and no `run --perf` row times it.
  - Where no instance directory resolves, file logging is impossible. That case gets no log and no stderr, by contract.
### obs-scope-summary#21.3 · values · scope-surfaces · ## 1. Obs Scope Summary · lines 198–198
- **Exporter:** The log file of whichever process is on each side (`run-<name>`, `hook-<name>`, `mcp`, or none for short-lived CLI verbs without an instance).
### obs-scope-summary#22.1 · values · scope-surfaces · ## 1. Obs Scope Summary · lines 209–209
- **Backend instrumentation hooks:** `tracing` spans per MCP tool call (`send · wait · last · answer · list`) wrapping the `viola-channel` Tokio client, through the tracing-subscriber JSON logger. stdio has no HTTP-framework auto-instrumentation.
### obs-scope-summary#22.3 · values · scope-surfaces · ## 1. Obs Scope Summary · lines 211–211
- **Exporter:** Always the file sink `<home>/diagnostics/mcp.ndjson` (tests excerpt; D-09), the home resolved through `resolved_home` like every verb's and checked with its `diagnostics/` by strict-modes before the sink opens; stderr JSON only when that file cannot be opened (as landed, chunk 2026-10-17-mcp-server-for-drivers). stdout is forbidden because it carries MCP frames. tracing-subscriber `fmt()` writes to stdout unless a writer is configured, so the writer must be set explicitly.
### obs-scope-summary#22.4 · values · scope-surfaces · ## 1. Obs Scope Summary · lines 212–212
- **Service identity:** as for the short-lived verbs, with `process="mcp"`. `instance` is the server's own `VIOLA_NAME` when set, which is also the `from` it adds.
### obs-scope-summary#22.5 · values · scope-surfaces · ## 1. Obs Scope Summary · lines 213–215
- **Notes:**
  - `isError` results carry codes only: `refusal` / `detail`, `instance-unreachable`, `wrapper-fault`, `invalid-params` (its `detail` a closed field word; chunk 2026-10-17-mcp-server-for-drivers) (security Vector 5). Log lines follow the same codes-only rule for rejected calls (tests excerpt, Coverage Triggers, MCP).
  - The `mcp` sink follows tests (`mcp.ndjson`), reconciled by D-09 and landed (stderr JSON only when the file cannot be opened).
### obs-scope-summary#23.1 · values · strategy · ## 1. Obs Scope Summary · lines 221–237
- **OTel SDK init:** v1 initialises no OTel SDK exporter. The `tracing` facade plus the tracing-subscriber JSON formatter is the binding pipeline (tests excerpt, Log format; Founder Direction 1: "tests owns them"). The constraints ruling out an exporter:
  - no outbound network calls (security Error reporting);
  - a new listener, including a local OTLP receiver or metrics endpoint, needs a Security Decisions Log entry (security Anti-Patterns);
  - stdout is reserved on `hook`, `mcp` and `run` (arch Obs-Relevant Conventions);
  - `cargo deny` bans Tokio in the sync crates' graph and bans C-building crates (arch Observability Hints, Build policy).

  An OTel bridge layer is optional. It may only feed an in-process or file sink, and it must stay Tokio-free on the sync crates. Phase 2 evaluates whether it adds anything over the tests-fixed format.

  Init order per process:
  1. As the first statement in `main`, install a replacement panic hook that writes one JSON line and nothing to stderr. The default hook's stderr output breaks the hook contract and the `run` terminal.
  2. Parse clap.
  3. Resolve the home (`--home`, then the grandparent of `VIOLA_DIR`, then the default) and the instance (`VIOLA_NAME` or an argument).
  4. Open `diagnostics/<process file>` in append mode with 0600 permissions inside a 0700 directory.
  5. Install the subscriber with an explicit writer (never stdout).
  6. Emit `process-start`.

  For `run`, this completes before the start sequence, so the collision check and version gate are logged. For `mcp`, it completes before the rmcp stdio transport starts.
### obs-scope-summary#24.1 · values · logs · ## 1. Obs Scope Summary · lines 243–252
- **Logging stack:** `tracing` with tracing-subscriber 0.3.23, configured as `fmt().json().flatten_event(true).with_current_span(false)` (tests excerpt, verbatim). Version 0.3.23 satisfies the security Vector 9 floor of `>=0.3.20` (RUSTSEC-2025-0055). Sink rules:
  - one file per process role under `diagnostics/`;
  - no stdout sink on any role;
  - stderr JSON only for `mcp`, and only when `mcp.ndjson` cannot be opened (D-09).

  The writer must emit each formatted line as a single `write` on an append-mode handle (arch Crash-safe disk writes). Concurrent `hook` processes share one file.

  The level filter is not read from `RUST_LOG`. Arch says "environment variables are not a configuration channel", and security Anti-Patterns ban env, flag or config values that disable any control. Any verbosity control therefore comes from `--home/config.json` or a CLI flag and must never bypass redaction. There is no separate agent-mode flag: JSON-to-file is the only mode.

  Timestamps must be RFC 3339 UTC with milliseconds and `Z` (tests excerpt, `timestamp`; arch Timestamps). Phase 2 must verify whether the tracing-subscriber default timer meets millisecond precision, or whether a chrono-based `FormatTime` (`to_rfc3339_opts(SecondsFormat::Millis, true)`) is needed.
### obs-scope-summary#25.0 · mirror · log-fields · ## 1. Obs Scope Summary · lines 258–278
```markdown
- **Format:** JSON-per-line. There are two streams:
  - **Event stream:** product events in `instances/<name>/events.ndjson`, exactly the arch Event line `{"v":1,"ts","instance","kind","source":"hook|wrapper|cli","data"}`.
  - **Process logs:** emitted by tracing-subscriber 0.3.23 `fmt().json().flatten_event(true).with_current_span(false)`, writing to `<home>/diagnostics/{run-<name>,hook-<name>,mcp,cli-<name>}.ndjson`. Files are 0600, dirs 0700, with one `write` per line.
- **Required fields:**
  - For process logs: `timestamp` (RFC 3339 UTC with ms and `Z`), `level` (`DEBUG|INFO|WARN|ERROR`), `target`, and `fields.message` (or the flattened `message`).
  - `event`, a closed kebab-case enum. A new value needs a Decisions Log entry, like the §3 closed enums. Each value fixes what its `corr` holds:
    - `channel-request`, `channel-response`: `corr` = the JSON-RPC request `id`
    - `dialog-raised`, `dialog-answered`: `corr` = `dialog_id`
    - `hook-invoked`, `hook-decision`: `corr` = `dialog_id` for dialog hooks, otherwise null
    - `send-issued`, `send-confirmed`, `send-refused`: `corr` = the send `cursor`
    - `process-start`, `process-exit`, `panic`: `corr` = null
  - `process` (`run|hook|mcp|cli`) and `instance` (a `ViolaName` or null).
  - `corr` is copied unchanged as a JSON number or string. It is never renamed (for example to `correlation_id`).

  obs-plan may add fields but must not rename or remove these. The harness greps on them.
- **Constraints:**
  - The hook trace is the `hook-<name>.ndjson` file. A failed write there is swallowed, so the hook still exits 0 and never writes to stderr.
  - No line may contain a `Cookie` header or any stripped `CLAUDE*` value. This is enforced by the secret-scan test in §6.
- **Agent parsing:** lines parseable with `jq -c 'select(.event=="dialog-raised")'` or equivalent; NEVER multi-line stack traces. A panic in a hook is caught and logged as one `level:"ERROR", event:"panic"` line.
```
### obs-scope-summary#26.1 · values · logs · ## 1. Obs Scope Summary · lines 284–292
- **Log file location:** Tests require `<home>/diagnostics/{run-<name>,hook-<name>,mcp,cli-<name>}.ndjson`. That differs from two upstream statements:
  - arch Obs-Relevant Conventions (Diagnostics location): `<viola home>/instances/<ViolaName>/diagnostics/`;
  - security Vector 2 and Data Classifications: only `instances/<name>/diagnostics/` may hold user content.

  Obs aligns to tests (tests is upstream) on file basenames and on the `logs` wrapper `{"src":"diag","file":<basename>,"record":<line>}`. The directory root (home-level or instance-level) must be reconciled in Phase 3/5 with a Decisions Log entry. If `mcp.ndjson` sits at home level, it must never carry user content.

  `mcp` always writes `<home>/diagnostics/mcp.ndjson`, as tests assume (D-09; stderr JSON only when the file cannot be opened; landed at chunk 2026-10-17-mcp-server-for-drivers).

  Every file is 0600 inside a 0700 directory (Windows: home DACL), and the tests secret-scan test enforces it. Retention is the lifetime of the session home (tests excerpt, `logs`); there is no rotation in v1 (security Vector 7).
### obs-scope-summary#27.1 · values · paths · ## 1. Obs Scope Summary · lines 298–298
- **Surfaces involved:** cli (`run`) + ipc-internal (channel bind) + persistent stores (snapshot, heartbeat, `events.ndjson`)
### obs-scope-summary#27.2 · values · paths · ## 1. Obs Scope Summary · lines 299–299
- **Must-trace spans:** `run.start` › `run.collision_check` › `run.pin_copy` › `run.version_gate` › `channel.bind` › `run.budget_eval` › `state.snapshot_write` › `state.heartbeat_start` › `pty.spawn`. Log events: `process-start` (run), `process-start` for the child, and `process-exit` on failure. The product event order to verify in `events.ndjson` is `wheel{cause:"start"}` → `budget-gate` → `session-start`.
### obs-scope-summary#27.3 · values · paths · ## 1. Obs Scope Summary · lines 300–300
- **Required log fields:** `event`, `process="run"`, `instance`, `corr=null`, `cli_version`, `cli_verified`, `exit_code`. Exit-1 causes use a distinct `detail` code: already-live, squatted-name, sha-256 mismatch, `.cmd`/`.bat` child, and for a start made by `viola revive` also no-session, session-live and cwd-missing. A revive whose instance check refuses writes no line: the check precedes the log (chunk 2026-10-10-budget-governor). No path or pid appears in the detail.
### obs-scope-summary#27.4 · values · paths · ## 1. Obs Scope Summary · lines 301–301
- **Source:** tests excerpt, Critical Path 1; arch Critical paths hint 1; Founder Direction 4
### obs-scope-summary#28.1 · values · paths · ## 1. Obs Scope Summary · lines 307–307
- **Surfaces involved:** cli or ipc-internal (mcp) → ipc-internal (channel) → cli (`run`, PTY paste) → cli (`hook` UserPromptSubmit; `hook` SessionStart for a `/clear` confirmed by its new session or a `/compact` confirmed by its same session (verified CLI); no hook for an unconfirmable send) → `events.ndjson`
### obs-scope-summary#28.2 · values · paths · ## 1. Obs Scope Summary · lines 308–308
- **Must-trace spans:** `send.client` (cli/mcp) › `channel.request` › `run.readiness_gate` (vt100) › `pty.paste_write` › `run.confirm_window` (not opened for an unconfirmable send). Log events: `channel-request`, then `send-issued`, then `hook-invoked` (UserPromptSubmit, or SessionStart for a `/clear` confirmed by its new session or a `/compact` confirmed by its same session (verified CLI); none awaited for an unconfirmable send), then `send-confirmed` or `send-refused`, then `channel-response`.
