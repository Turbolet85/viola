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
### obs-scope-summary#28.2 · values · paths · ## 1. Obs Scope Summary · lines 308–308
- **Must-trace spans:** `send.client` (cli/mcp) › `channel.request` › `run.readiness_gate` (vt100) › `pty.paste_write` › `run.confirm_window` (not opened for an unconfirmable send). Log events: `channel-request`, then `send-issued`, then `hook-invoked` (UserPromptSubmit, or SessionStart for a `/clear` confirmed by its new session or a `/compact` confirmed by its same session (verified CLI); none awaited for an unconfirmable send), then `send-confirmed` or `send-refused`, then `channel-response`.
### obs-scope-summary#28.3 · values · paths · ## 1. Obs Scope Summary · lines 309–309
- **Required log fields:** `event`, `process`, `instance`, `corr` (the JSON-RPC `id` on channel lines; the send `cursor` on send lines), `from` (self-reported, not an identity), `confirmed`, `refusal`, `detail` (`input-not-ready` / `no-prompt-submitted` / `control-character` / `empty-text` / `turn-running`), `duration_ms`. Sent text never appears.
### obs-scope-summary#32.1 · values · paths · ## 1. Obs Scope Summary · lines 343–343
- **Surfaces involved:** cli (`hook statusline`, shell-out) → persistent store (`budget.json`) → cli (`run`, gate evaluation at start, on `send` and at `turn-ended`) → `events.ndjson` (`budget-gate`)
### obs-scope-summary#32.2 · values · paths · ## 1. Obs Scope Summary · lines 344–344
- **Must-trace spans:** `hook.statusline` › `state.budget_write` › `statusline.shell_out`, then `run.budget_eval`. Log events: `hook-invoked`, `hook-decision` for statusline, and `send-refused` with `budget-paused`.
### obs-scope-summary#32.3 · values · paths · ## 1. Obs Scope Summary · lines 345–345
- **Required log fields:** `event`, `process`, `instance`, `corr`, `window` (`five-hour` / `seven-day`), `used_percentage`, `resets_at`, `paused`, `refusal="budget-paused"`, `exit_code` (11), `shell_exit_status`, `duration_ms`. The `statusline_command` string never appears outside `diagnostics/`.
### obs-scope-summary#33.1 · values · triggers · ## 1. Obs Scope Summary · lines 352–355
- **Required telemetry signal:**
  - A scrub rule on `channel-request` / `channel-response`: method, `id` / `corr`, result class (`ok` / `refusal` / `error` code) and `sender` version only, never `params` or result bodies.
  - `-32603` is logged as the fixed "internal error".
  - `from` is logged as unauthenticated.
### obs-scope-summary#34.1 · values · triggers · ## 1. Obs Scope Summary · lines 361–363
- **Required telemetry signal:**
  - A never-log rule for `send` text on stderr, `--json`, MCP and every process-log field outside `instances/<name>/diagnostics/`.
  - `send-refused` with detail `control-character` or `empty-text` is emitted by each deciding side, client and wrapper, without the text (tests excerpt, Coverage Triggers, Vector 2).
### obs-scope-summary#37.1 · values · triggers · ## 1. Obs Scope Summary · lines 384–388
- **Required telemetry signal:**
  - A never-log floor for `CLAUDE_CODE_MESSAGING_TOKEN`, `CLAUDE_CODE_MESSAGING_SOCKET` and every R8-stripped `CLAUDE*` value, across all sinks and fixtures.
  - The R8 strip is logged as a count or variable-name list only, never values. Phase 3 must confirm names are admissible.
  - An anyhow chain containing a serde source becomes a fixed message before reaching stderr.
  - No env, flag or config may disable redaction.
### obs-scope-summary#40.1 · values · triggers · ## 1. Obs Scope Summary · lines 412–412
- **Required telemetry signal:** The logger dependency is tracing-subscriber `>=0.3.20` (0.3.23 pinned by tests). Obs dependencies stay Tokio-free in the sync crates and add no C-building crates, and they pass `cargo deny` on all three targets.
### obs-scope-summary#41.1 · values · triggers · ## 1. Obs Scope Summary · lines 418–421
- **Required telemetry signal:**
  - `corr` = send `cursor` joins `send-issued` / `send-confirmed` / `send-refused` across CLI, MCP and wrapper.
  - `corr` = `dialog_id` joins hook → wrapper → driver.
  - The per-connection JSON-RPC `id` needs a discriminator for a merged-log join (section 2).
### obs-scope-summary#44.1 · values · triggers · ## 1. Obs Scope Summary · lines 450–456
- **Required telemetry signal:** The same file sink and line schema on every OS. Platform differences must still yield identical fields:
  - permission application: Windows DACL vs Unix 0600/0700;
  - append atomicity with concurrent hook writers: Windows append vs POSIX `O_APPEND`;
  - named pipe vs Unix socket in channel logs;
  - ConPTY vs openpty in `pty` spans.

  No OS-specific exporter.
### obs-scope-summary#45.1 · values · triggers · ## 1. Obs Scope Summary · lines 462–462
- **Required telemetry signal:** A zero-unlogged-panics invariant for all four process roles. Each panic produces exactly one single-line `event:"panic"` record and no multi-line backtrace, and `hook` still exits 0. No formal latency or availability SLO beyond the tests perf gates.
### obs-scope-summary#47.1 · values · triggers · ## 1. Obs Scope Summary · lines 474–477
- **Required telemetry signal:**
  - Structured JSON process logs from every process role (`run` / `hook` / `mcp` / `cli`) under `diagnostics/`, merged by harness `logs` with `events.ndjson`.
  - `send-issued` and `send-refused` are logged events.
  - The tests field and event list is adopted verbatim.
### obs-scope-summary#48.1 · values · triggers · ## 1. Obs Scope Summary · lines 483–485
- **Required telemetry signal:**
  - `release-from-driver` is its own logged event, not a generic -32602 wrapper fault.
  - Each cause behind exit 21 (dead instance, strict-mode fail, server-verify fail) and exit 1 (already live, squatted name, SHA-256 mismatch, `.cmd`/`.bat` child, and `viola revive`'s no logged session and missing recorded directory) gets a distinct `detail` code with no path or pid. `viola revive`'s strict-mode fail carries no logged `detail` as landed (no log line): its output is the stderr pair, or under `--json` the `{"v":1,"error":"strict-modes-failed","detail":null}` document (chunk 2026-10-17-cli-machine-contract), exit 1 (chunk 2026-10-10-budget-governor).
### obs-scope-summary#50.1 · values · triggers · ## 1. Obs Scope Summary · lines 501–504
- **Required telemetry signal:**
  - Events are emitted deterministically, keyed on hook events and `events.ndjson` offsets.
  - No telemetry is derived from screen content: vt100 is used for gate outcomes only.
  - No log-based waits on sleeps.
