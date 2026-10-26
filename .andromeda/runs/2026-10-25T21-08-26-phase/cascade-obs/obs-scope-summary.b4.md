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
### obs-scope-summary#28.3 · values · paths · ## 1. Obs Scope Summary · lines 309–309
- **Required log fields:** `event`, `process`, `instance`, `corr` (the JSON-RPC `id` on channel lines; the send `cursor` on send lines), `from` (self-reported, not an identity), `confirmed`, `refusal`, `detail` (`input-not-ready` / `no-prompt-submitted` / `control-character` / `empty-text` / `turn-running`), `duration_ms`. Sent text never appears.
### obs-scope-summary#28.4 · values · paths · ## 1. Obs Scope Summary · lines 310–310
- **Source:** tests excerpt, Critical Path 2; arch hint 2; design excerpt, Readback (send → readback latency span); Founder Direction 5; creator must-work "First live test"
### obs-scope-summary#29.1 · values · paths · ## 1. Obs Scope Summary · lines 316–316
- **Surfaces involved:** cli or ipc-internal (mcp) → ipc-internal (channel) → cli (`run`) + persistent store (`events.ndjson` tail)
### obs-scope-summary#29.2 · values · paths · ## 1. Obs Scope Summary · lines 317–317
- **Must-trace spans:** `wait.block` (client) › `channel.request` › `run.wait_dispatch`. Log events: `channel-request` and `channel-response` with the outcome.
### obs-scope-summary#29.3 · values · paths · ## 1. Obs Scope Summary · lines 318–318
- **Required log fields:** `event`, `process`, `instance`, `corr` (JSON-RPC `id`), `after` (byte offset), `timeout_ms`, `outcome` (woken kind / `timed_out` / `instance-unreachable`), `exit_code` (21 when the wrapper is killed mid-wait), `duration_ms`. The `last_assistant_message` content never appears.
### obs-scope-summary#29.4 · values · paths · ## 1. Obs Scope Summary · lines 319–319
- **Source:** tests excerpt, Critical Path 3; arch hint 3; design excerpt, CLI `viola wait` (span with outcome attribute); creator must-work "Waking without polling"
### obs-scope-summary#30.1 · values · paths · ## 1. Obs Scope Summary · lines 325–325
- **Surfaces involved:** cli (`hook` PreToolUse / PermissionRequest) → ipc-internal (`hook.dialog`) → cli (`run`) ← cli or ipc-internal (mcp) `answer`
### obs-scope-summary#30.2 · values · paths · ## 1. Obs Scope Summary · lines 326–326
- **Must-trace spans:** `hook.handle` › `channel.request(hook.dialog)` › `run.dialog_register` › `run.dialog_await` › `hook.decision_emit`; on the driver side, `answer.client` › `channel.request(answer)`. Log events: `hook-invoked`, `dialog-raised`, `dialog-answered`, `hook-decision` (decision null on deadline expiry).
### obs-scope-summary#30.3 · values · paths · ## 1. Obs Scope Summary · lines 327–327
- **Required log fields:** `event`, `process`, `instance`, `corr` (`dialog_id`), `dialog_kind`, `refusal` / `detail` (`unknown-dialog` exit 13, `unverified-cli` exit 12), `deadline_hit`, `duration_ms`. The question, answer and tool `input` text never appear.
### obs-scope-summary#30.4 · values · paths · ## 1. Obs Scope Summary · lines 328–328
- **Source:** tests excerpt, Critical Path 4 and Critical Path 7 (unverified CLI gate folded in as the answer-withheld branch); arch hint 4; creator must-work "Dialog answering"
### obs-scope-summary#31.1 · values · paths · ## 1. Obs Scope Summary · lines 334–334
- **Surfaces involved:** cli (`run`, key input) + `events.ndjson` (`wheel`) → cli or ipc-internal (mcp) `send` refused → cli `release` → ipc-internal
### obs-scope-summary#31.2 · values · paths · ## 1. Obs Scope Summary · lines 335–335
- **Must-trace spans:** `run.wheel_transition` › `channel.request(send)` refused › `pause.client` / `release.client` › `channel.request(pause|release)`. Log events: `send-refused` and `channel-request` / `channel-response` for `pause` and `release`. `release-from-driver` is logged as its own event (Founder Direction 4), admitted by `schemas/diag-line.v1.json`.
