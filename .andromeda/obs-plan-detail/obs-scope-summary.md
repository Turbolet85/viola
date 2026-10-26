# obs-plan · Obs Scope Summary
Read whole. The master's broad view is .andromeda/obs-plan.md; this file holds its Obs Scope Summary detail.

## 1. Obs Scope Summary

<!-- detail: values · tier -->

- 5 telemetry surface entries across 4 process roles (`run` / `hook` / `mcp` / `cli`) and 2 product parts;
- 6 instrumentable crates;
- 6 must-trace paths plus 4 edge flows;
- 18 telemetry triggers, including cross-surface propagation through `corr`, perf budgets, chaos fault telemetry, multi-OS compat, and binding Founder Directions.

<!-- /detail -->

<!-- detail: pending · tier -->

- the `diagnostics/` directory root (tests `<home>/diagnostics/` vs arch and security `instances/<name>/diagnostics/`);
- `mcp` logging to stderr when `VIOLA_DIR` is unset — reconciled by D-09 and landed (chunk 2026-10-17-mcp-server-for-drivers): `mcp` always writes `mcp.ndjson`, stderr JSON only when that file cannot be opened;
- the `event` enum extensions (`release-from-driver` and the chaos signals);
- `hook-invoked` `corr` timing relative to `dialog_id` assignment.

<!-- /detail -->

<!-- detail: values · scope-entities -->

- **Source:** arch excerpt, Workspace / Modules; Stack (clap 4.6.7, vt100 0.16.2, thiserror/anyhow)
- **Instrumentability:** instrumentable
- **Reason if not fully instrumentable:** N/A. This is the only crate that uses anyhow, so it is the error-capture boundary. Arch Stack says "context chains only at dispatch". Security Vector 6 says a chain that contains a serde source must become a fixed message before it reaches stderr. The full chain goes to `diagnostics/` only.

<!-- /detail -->

<!-- detail: values · scope-entities -->

- **Source:** arch excerpt, Workspace / Modules
- **Instrumentability:** not-instrumentable
- **Reason if not fully instrumentable:** It is a pure type crate with no I/O and no runtime behaviour of its own. It supplies the kebab-case values used as log fields (`refusal`, `detail`, event kinds) but emits nothing. Excluded from sections 2 to 4.

<!-- /detail -->

<!-- detail: values · scope-entities -->

- **Source:** arch excerpt, Stack (portable-pty) and Workspace
- **Instrumentability:** instrumentable
- **Reason if not fully instrumentable:** N/A for the seam. The ConPTY and openpty OS APIs behind it are boundary-only (see the OS PTY entry). The PTY byte stream is user content and is never logged (security Data Classifications, user content; brief §3.2 "Nobody parses the rendered screen for content").

<!-- /detail -->

<!-- detail: values · scope-entities -->

- **Source:** arch excerpt, Stack (interprocess 2.4.4), Workspace, Standard Contracts (IPC methods `send · wait · last · answer · pause · release · link · unlink · hook.dialog · hook.event`)
- **Instrumentability:** instrumentable
- **Reason if not fully instrumentable:** N/A. Stack forbids Tokio in this crate's graph unless its feature is enabled, so instrumentation must be sync.

<!-- /detail -->

<!-- detail: values · scope-entities -->

- **Source:** arch excerpt, Stack (ndjson + atomic-write-file, sysinfo 0.39.6) and Workspace
- **Instrumentability:** instrumentable
- **Reason if not fully instrumentable:** N/A. Stack bans Tokio here, so instrumentation must be sync.

<!-- /detail -->

<!-- detail: values · scope-entities -->

- **Source:** arch excerpt, Workspace; Stack (serde_path_to_error drift reports)
- **Instrumentability:** instrumentable
- **Reason if not fully instrumentable:** N/A. Its outputs are the most sensitive in the product:
  - Drift reports go to `diagnostics/` only (security Vector 4).
  - R8-stripped `CLAUDE*` values are never logged anywhere (security Vector 6).

<!-- /detail -->

<!-- detail: values · scope-entities -->

- **Source:** arch excerpt, Stack (rmcp) and Workspace
- **Instrumentability:** instrumentable
- **Reason if not fully instrumentable:** N/A. stdout is reserved for MCP frames (arch Stack and Obs-Relevant Conventions), so no telemetry may go to stdout.

<!-- /detail -->

<!-- detail: values · scope-entities -->

- **Source:** arch excerpt, Stack (portable-pty; "hosts the unmodified `claude`"); Project Intent
- **Instrumentability:** boundary-only
- **Reason if not fully instrumentable:** It is a third-party binary run unmodified. Instrument only the boundary:
  - spawn and exit (`process-start` / `process-exit`), with exit detected on the process handle, never on EOF (Stack; brief §4.1);
  - paste writes;
  - the vt100 readiness-gate outcome.

  Its internal state and transcripts are never read as a contract (creator Obs Anti-Patterns, brief §4).

<!-- /detail -->

<!-- detail: values · scope-entities -->

- **Source:** arch excerpt, Standard Contracts (Hook contract; MCP tools)
- **Instrumentability:** boundary-only
- **Reason if not fully instrumentable:** The caller is Claude Code, so viola sees only the inbound stdin payload and its own response. Instrument `hook-invoked` / `hook-decision` and MCP tool-call handling. No trace context arrives from Claude Code.

<!-- /detail -->

<!-- detail: values · scope-entities -->

- **Source:** arch excerpt, Stack (sysinfo + `claude agents --json`); security Vector 8
- **Instrumentability:** boundary-only
- **Reason if not fully instrumentable:** External CLI. Instrument the spawn/exit and the parse outcome. A parse failure becomes `"unknown"` plus a counter, and the output rows are untrusted.

<!-- /detail -->

<!-- detail: values · scope-entities -->

- **Source:** arch excerpt, Standard Contracts (`viola hook statusline`, "the only shell-out in viola")
- **Instrumentability:** boundary-only
- **Reason if not fully instrumentable:** A user-written shell command. Instrument the shell-out duration and exit status only. The command string is never logged outside `diagnostics/` (security Data Classifications, `statusline_command`).

<!-- /detail -->

<!-- detail: values · scope-entities -->

- **Source:** arch excerpt, Stack
- **Instrumentability:** boundary-only
- **Reason if not fully instrumentable:** Instrument viola's call sites (bind, connect, lock, rename, kill-fallback use, and the server check's peer-pid and start-time reads, whose failed check the CLI and the hook log as `server-verify-failed` and MCP logs by no line), not the OS internals.

<!-- /detail -->

<!-- detail: values · scope-entities -->

- **Source:** arch excerpt, Stack (ndjson + snapshots), Standard Contracts (event line, snapshot envelope, heartbeat); security Data Classifications
- **Instrumentability:** instrumentable (through the `viola-state` call sites)
- **Reason if not fully instrumentable:** N/A. Scrub rules per store (security Data Classifications):
  - `events.ndjson` is itself the product audit stream (brief §7, "the event log as the audit trail"). Its payload content is never copied into process-log lines.
  - Absolute paths in `snapshot.json` (`endpoint`, `pinned_bin`) and in config values are scrubbed from external output. They are allowed in `diagnostics/`.
  - Operational metadata in `budget.json` (percentages, `resets_at`) may be logged as structured fields.

<!-- /detail -->

<!-- detail: values · scope-entities -->

- **Source:** arch excerpt, Observability Hints (Config management); security Data Classifications (config values)
- **Instrumentability:** not-instrumentable
- **Reason if not fully instrumentable:** Static configuration and data. Only the parse at load time is observable, and it is logged through `viola-state` / root bin (skipped-key counts).

<!-- /detail -->

<!-- detail: values · scope-entities -->

- **Source:** arch excerpt, CI/CD Platform
- **Instrumentability:** not-instrumentable
- **Reason if not fully instrumentable:** Not product runtime. Fixtures are scrub-required data (security Data Classifications, recorded fixtures), not a telemetry source. Excluded from sections 2 to 4.

<!-- /detail -->

<!-- detail: values · scope-surfaces -->

- **Backend instrumentation hooks:** Rust `tracing` facade with `#[tracing::instrument]` spans on the dispatch functions, plus a tracing-subscriber JSON structured logger (tests excerpt, Log format: `tracing-subscriber 0.3.23 fmt().json().flatten_event(true).with_current_span(false)`). Each process gets its own panic hook: a `std::panic::set_hook` replacement that writes one line. No HTTP framework is involved, so there is no auto-instrumentation.
- **Frontend instrumentation hooks:** N/A. The only output is terminal stdout/stderr (design-system.md cli surface: static fixed-column text, with no interactive prompt, TUI or pager).
- **Exporter:** A file sink only: JSON-per-line to `diagnostics/` when an instance is resolved (security Vector 6: "write the full error only to `diagnostics/` when an instance is resolved"). Without an instance there is no process-log file, and human stderr carries only fixed-message lines (design excerpt, CLI faults). Network OTLP is excluded: security says "viola makes no outbound network calls". The stdout exporter is excluded because stdout carries `--json` results. `viola revive <name> --list` logs as `cli` into the caller's `cli-<VIOLA_NAME>.ndjson` once its strict-modes check has passed (none with no `VIOLA_NAME`; a refused check opens none; chunk 2026-10-17-cli-machine-contract). `viola revive`'s start arm is not a short-lived verb: it is the wrapper surface below (`process="run"`, `diagnostics/run-<name>.ndjson`).
- **Service identity:** `service.name` is hardcoded `"viola"`, the binary name. `service.version` is compile-time `env!("CARGO_PKG_VERSION")`, the same value already used as `sender` and `writer`. The process role is the `process` field. The instance is `ViolaName` from an explicit argument or `VIOLA_NAME`. `deployment.environment` is N/A: local-only, with no environments (arch Obs-Relevant Conventions, Service identity; arch Surfaces, "no hosting").
- **Notes:**
  - The instance comes from `VIOLA_NAME` or an argument, never from the working directory (arch Instance identity rule).
  - Refusal and fault lines on stderr are fixed strings with no paths, pids, chains or upstream text (design excerpt, CLI refusals / CLI faults).
  - Exit codes 0, 1, 2, 10–14, 20 and 21 are the machine-readable outcome and should be a log field (arch Standard Contracts, CLI `--json` + exit codes).
  - Founder Direction 4: each cause behind exit 21 and exit 1 gets a distinct detail code.

<!-- /detail -->

<!-- detail: values · scope-surfaces -->

- **Backend instrumentation hooks:** `tracing` spans (`#[tracing::instrument]`) on the start sequence, the `viola-pty` seam operations, the `viola-channel` sync server handlers, vt100 readiness-gate outcomes and `viola-state` writes, all written through the tracing-subscriber JSON logger. Instrumentation must be sync: std threads, no Tokio (arch Stack, Hybrid concurrency; `cargo deny` ban).
- **Frontend instrumentation hooks:** N/A. The terminal carries the child's screen.
- **Exporter:** A file sink only, to `diagnostics/run-<name>.ndjson` (tests excerpt, Log format). The terminal is forbidden: "`run` writes nothing to the terminal while the child runs" (arch Observability Hints).
- **Service identity:** as for the short-lived verbs, with `process="run"` and `instance=<ViolaName>`.
- **Notes:**
  - Rust's default panic hook prints to stderr, which is the child's terminal. It must be replaced before the child spawns.
  - The heartbeat file, touched every 1 s, is the existing liveness tick (arch Standard Contracts, Heartbeat).
  - Child exit is detected on the process handle, so `process-exit` for the child must come from the handle wait, not from reader EOF (arch Stack; brief §4.1 Windows exit detection).

<!-- /detail -->

<!-- detail: values · scope-surfaces -->

- **Backend instrumentation hooks:** `tracing` spans around stdin parse → channel call (`hook.dialog` / `hook.event`) → decision emit, through the tracing-subscriber JSON logger. `hook-invoked` / `hook-decision` lines are required, and panics are caught with `catch_unwind` and emitted as one `level:"ERROR", event:"panic"` line (tests excerpt, Log format, Agent parsing).
- **Frontend instrumentation hooks:** N/A.
- **Exporter:** A file sink only, to `diagnostics/hook-<name>.ndjson`. stderr is forbidden on both. stdout carries only the decision body for `viola hook <event>`, and for `viola hook statusline` only the user's status-line command's output (arch Standard Contracts, Hook contract); the two hidden `--capture` arms (`verify`'s probe capture) write no sink and print nothing, except `--answers`' probe decision body. A failed log write is swallowed, and the hook still exits 0 (tests excerpt, Log format Constraints).
- **Service identity:** as for the short-lived verbs, with `process="hook"`. If `VIOLA_NAME` is missing, `instance` is null and the hook is a silent no-op (tests excerpt, E1 R5 unwrapped session hooks; Coverage Triggers, Vector 4 "missing `VIOLA_NAME`").
- **Notes:**
  - Many hook processes can append to the same `hook-<name>.ndjson` at once. Each line must be one `write` call on an append-mode handle (arch Stack, atomic-write-file / `OpenOptions::append`).
  - The latency budget is `max < 1.0 s` per hook (tests excerpt, Quality Gates). Logger init must not add blocking I/O beyond one file open and append. `viola hook statusline` is outside that budget: its shell-out of the user's command is bounded by the 5 s PROVISIONAL `STATUSLINE_DEADLINE`, and no `run --perf` row times it.
  - Where no instance directory resolves, file logging is impossible. That case gets no log and no stderr, by contract.

<!-- /detail -->

<!-- detail: values · scope-surfaces -->

- **Backend instrumentation hooks:** language-native `tracing` spans on client call and server dispatch. Propagation uses the `corr` field in log lines (JSON-RPC `id` for `channel-request` / `channel-response`; `dialog_id`; send `cursor`), per the tests excerpt Log format. Channel `params` declare no trace-context field (arch Standard Contracts, `send`: "No trace-context field is declared"). Readers skip unknown fields (arch Mixed-version tolerance), so an additive envelope field would be wire-compatible, but none is required by upstream.
- **Frontend instrumentation hooks:** N/A.
- **Exporter:** The log file of whichever process is on each side (`run-<name>`, `hook-<name>`, `mcp`, or none for short-lived CLI verbs without an instance).
- **Service identity:** Each side logs its own `process`. The peer version is available as `params.sender` (`CARGO_PKG_VERSION`).
- **Notes:**
  - JSON-RPC ids are "integers, monotonic per connection" (arch Standard Contracts, JSON-RPC frame shapes). `corr` for `channel-request` / `channel-response` is therefore not unique across connections or processes, and a merged-log join needs `process`, `instance` and a connection discriminator. This is a gap for Phase 3; the tests contract allows additive fields.
  - `-32603` must be `"internal error"` with `data:null` (security Vector 1).
  - `from` is self-reported and must not be logged as an authenticated identity (security Vector 1).

<!-- /detail -->

<!-- detail: values · scope-surfaces -->

- **Backend instrumentation hooks:** `tracing` spans per MCP tool call (`send · wait · last · answer · list`) wrapping the `viola-channel` Tokio client, through the tracing-subscriber JSON logger. stdio has no HTTP-framework auto-instrumentation.
- **Frontend instrumentation hooks:** N/A.
- **Exporter:** Always the file sink `<home>/diagnostics/mcp.ndjson` (tests excerpt; D-09), the home resolved through `resolved_home` like every verb's and checked with its `diagnostics/` by strict-modes before the sink opens; stderr JSON only when that file cannot be opened (as landed, chunk 2026-10-17-mcp-server-for-drivers). stdout is forbidden because it carries MCP frames. tracing-subscriber `fmt()` writes to stdout unless a writer is configured, so the writer must be set explicitly.
- **Service identity:** as for the short-lived verbs, with `process="mcp"`. `instance` is the server's own `VIOLA_NAME` when set, which is also the `from` it adds.
- **Notes:**
  - `isError` results carry codes only: `refusal` / `detail`, `instance-unreachable`, `wrapper-fault`, `invalid-params` (its `detail` a closed field word; chunk 2026-10-17-mcp-server-for-drivers) (security Vector 5). Log lines follow the same codes-only rule for rejected calls (tests excerpt, Coverage Triggers, MCP).
  - The `mcp` sink follows tests (`mcp.ndjson`), reconciled by D-09 and landed (stderr JSON only when the file cannot be opened).

<!-- /detail -->

<!-- detail: values · strategy -->

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

<!-- /detail -->

<!-- detail: values · logs -->

- **Logging stack:** `tracing` with tracing-subscriber 0.3.23, configured as `fmt().json().flatten_event(true).with_current_span(false)` (tests excerpt, verbatim). Version 0.3.23 satisfies the security Vector 9 floor of `>=0.3.20` (RUSTSEC-2025-0055). Sink rules:
  - one file per process role under `diagnostics/`;
  - no stdout sink on any role;
  - stderr JSON only for `mcp`, and only when `mcp.ndjson` cannot be opened (D-09).

  The writer must emit each formatted line as a single `write` on an append-mode handle (arch Crash-safe disk writes). Concurrent `hook` processes share one file.

  The level filter is not read from `RUST_LOG`. Arch says "environment variables are not a configuration channel", and security Anti-Patterns ban env, flag or config values that disable any control. Any verbosity control therefore comes from `--home/config.json` or a CLI flag and must never bypass redaction. There is no separate agent-mode flag: JSON-to-file is the only mode.

  Timestamps must be RFC 3339 UTC with milliseconds and `Z` (tests excerpt, `timestamp`; arch Timestamps). Phase 2 must verify whether the tracing-subscriber default timer meets millisecond precision, or whether a chrono-based `FormatTime` (`to_rfc3339_opts(SecondsFormat::Millis, true)`) is needed.

<!-- /detail -->

<!-- detail: mirror · log-fields -->

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

<!-- /detail -->

<!-- detail: values · logs -->

- **Log file location:** Tests require `<home>/diagnostics/{run-<name>,hook-<name>,mcp,cli-<name>}.ndjson`. That differs from two upstream statements:
  - arch Obs-Relevant Conventions (Diagnostics location): `<viola home>/instances/<ViolaName>/diagnostics/`;
  - security Vector 2 and Data Classifications: only `instances/<name>/diagnostics/` may hold user content.

  Obs aligns to tests (tests is upstream) on file basenames and on the `logs` wrapper `{"src":"diag","file":<basename>,"record":<line>}`. The directory root (home-level or instance-level) must be reconciled in Phase 3/5 with a Decisions Log entry. If `mcp.ndjson` sits at home level, it must never carry user content.

  `mcp` always writes `<home>/diagnostics/mcp.ndjson`, as tests assume (D-09; stderr JSON only when the file cannot be opened; landed at chunk 2026-10-17-mcp-server-for-drivers).

  Every file is 0600 inside a 0700 directory (Windows: home DACL), and the tests secret-scan test enforces it. Retention is the lifetime of the session home (tests excerpt, `logs`); there is no rotation in v1 (security Vector 7).

<!-- /detail -->

<!-- detail: values · paths -->

- **Surfaces involved:** cli (`run`) + ipc-internal (channel bind) + persistent stores (snapshot, heartbeat, `events.ndjson`)
- **Must-trace spans:** `run.start` › `run.collision_check` › `run.pin_copy` › `run.version_gate` › `channel.bind` › `run.budget_eval` › `state.snapshot_write` › `state.heartbeat_start` › `pty.spawn`. Log events: `process-start` (run), `process-start` for the child, and `process-exit` on failure. The product event order to verify in `events.ndjson` is `wheel{cause:"start"}` → `budget-gate` → `session-start`.
- **Required log fields:** `event`, `process="run"`, `instance`, `corr=null`, `cli_version`, `cli_verified`, `exit_code`. Exit-1 causes use a distinct `detail` code: already-live, squatted-name, sha-256 mismatch, `.cmd`/`.bat` child, and for a start made by `viola revive` also no-session, session-live and cwd-missing. A revive whose instance check refuses writes no line: the check precedes the log (chunk 2026-10-10-budget-governor). No path or pid appears in the detail.
- **Source:** tests excerpt, Critical Path 1; arch Critical paths hint 1; Founder Direction 4

<!-- /detail -->

<!-- detail: values · paths -->

- **Surfaces involved:** cli or ipc-internal (mcp) → ipc-internal (channel) → cli (`run`, PTY paste) → cli (`hook` UserPromptSubmit; `hook` SessionStart for a `/clear` confirmed by its new session or a `/compact` confirmed by its same session (verified CLI); no hook for an unconfirmable send) → `events.ndjson`
- **Must-trace spans:** `send.client` (cli/mcp) › `channel.request` › `run.readiness_gate` (vt100) › `pty.paste_write` › `run.confirm_window` (not opened for an unconfirmable send). Log events: `channel-request`, then `send-issued`, then `hook-invoked` (UserPromptSubmit, or SessionStart for a `/clear` confirmed by its new session or a `/compact` confirmed by its same session (verified CLI); none awaited for an unconfirmable send), then `send-confirmed` or `send-refused`, then `channel-response`.
- **Required log fields:** `event`, `process`, `instance`, `corr` (the JSON-RPC `id` on channel lines; the send `cursor` on send lines), `from` (self-reported, not an identity), `confirmed`, `refusal`, `detail` (`input-not-ready` / `no-prompt-submitted` / `control-character` / `empty-text` / `turn-running`), `duration_ms`. Sent text never appears.
- **Source:** tests excerpt, Critical Path 2; arch hint 2; design excerpt, Readback (send → readback latency span); Founder Direction 5; creator must-work "First live test"

<!-- /detail -->

<!-- detail: values · paths -->

- **Surfaces involved:** cli or ipc-internal (mcp) → ipc-internal (channel) → cli (`run`) + persistent store (`events.ndjson` tail)
- **Must-trace spans:** `wait.block` (client) › `channel.request` › `run.wait_dispatch`. Log events: `channel-request` and `channel-response` with the outcome.
- **Required log fields:** `event`, `process`, `instance`, `corr` (JSON-RPC `id`), `after` (byte offset), `timeout_ms`, `outcome` (woken kind / `timed_out` / `instance-unreachable`), `exit_code` (21 when the wrapper is killed mid-wait), `duration_ms`. The `last_assistant_message` content never appears.
- **Source:** tests excerpt, Critical Path 3; arch hint 3; design excerpt, CLI `viola wait` (span with outcome attribute); creator must-work "Waking without polling"

<!-- /detail -->

<!-- detail: values · paths -->

- **Surfaces involved:** cli (`hook` PreToolUse / PermissionRequest) → ipc-internal (`hook.dialog`) → cli (`run`) ← cli or ipc-internal (mcp) `answer`
- **Must-trace spans:** `hook.handle` › `channel.request(hook.dialog)` › `run.dialog_register` › `run.dialog_await` › `hook.decision_emit`; on the driver side, `answer.client` › `channel.request(answer)`. Log events: `hook-invoked`, `dialog-raised`, `dialog-answered`, `hook-decision` (decision null on deadline expiry).
- **Required log fields:** `event`, `process`, `instance`, `corr` (`dialog_id`), `dialog_kind`, `refusal` / `detail` (`unknown-dialog` exit 13, `unverified-cli` exit 12, `server-verify-failed` on Linux and Windows: exit 21 on CLI `answer` with `during:"connect"`, fail-open on the dialog hook with `corr` null and `decision_emitted:false`, no pid, start time, path or endpoint), `deadline_hit`, `duration_ms`. The question, answer and tool `input` text never appear.
- **Source:** tests excerpt, Critical Path 4 and Critical Path 7 (unverified CLI gate folded in as the answer-withheld branch); arch hint 4; creator must-work "Dialog answering"

<!-- /detail -->

<!-- detail: values · paths -->

- **Surfaces involved:** cli (`run`, key input) + `events.ndjson` (`wheel`) → cli or ipc-internal (mcp) `send` refused → cli `release` → ipc-internal
- **Must-trace spans:** `run.wheel_transition` › `channel.request(send)` refused › `pause.client` / `release.client` › `channel.request(pause|release)`. Log events: `send-refused` and `channel-request` / `channel-response` for `pause` and `release`. `release-from-driver` is logged as its own event (Founder Direction 4), admitted by `schemas/diag-line.v1.json`.
- **Required log fields:** `event`, `process`, `instance`, `corr`, `refusal="human-typing"`, `detail` (null / `manual-pause`), `wheel` holder, `exit_code` (10). Typed keys never appear.
- **Source:** tests excerpt, Critical Path 5; arch hint 5; creator must-work "Wheel take-over" and "R2 One wheel"

<!-- /detail -->

<!-- detail: values · paths -->

- **Surfaces involved:** cli (`hook statusline`, shell-out) → persistent store (`budget.json`) → cli (`run`, gate evaluation at start, on `send` and at `turn-ended`) → `events.ndjson` (`budget-gate`)
- **Must-trace spans:** `hook.statusline` › `state.budget_write` › `statusline.shell_out`, then `run.budget_eval`. Log events: `hook-invoked`, `hook-decision` for statusline, and `send-refused` with `budget-paused`.
- **Required log fields:** `event`, `process`, `instance`, `corr`, `window` (`five-hour` / `seven-day`), `used_percentage`, `resets_at`, `paused`, `refusal="budget-paused"`, `exit_code` (11), `shell_exit_status`, `duration_ms`. The `statusline_command` string never appears outside `diagnostics/`.
- **Source:** tests excerpt, Critical Path 6; arch hint 6; creator must-work "Budget governor"; brief §7 policy

<!-- /detail -->

<!-- detail: values · triggers -->

- **Required telemetry signal:**
  - A scrub rule on `channel-request` / `channel-response`: method, `id` / `corr`, result class (`ok` / `refusal` / `error` code) and `sender` version only, never `params` or result bodies.
  - `-32603` is logged as the fixed "internal error".
  - `from` is logged as unauthenticated.

<!-- /detail -->

<!-- detail: values · triggers -->

- **Required telemetry signal:**
  - A never-log rule for `send` text on stderr, `--json`, MCP and every process-log field outside `instances/<name>/diagnostics/`.
  - `send-refused` with detail `control-character` or `empty-text` is emitted by each deciding side, client and wrapper, without the text (tests excerpt, Coverage Triggers, Vector 2).

<!-- /detail -->

<!-- detail: values · triggers -->

- **Required telemetry signal:**
  - Fail-open paths are logged to `hook-<name>.ndjson` only: oversize stdin, malformed JSON, channel failure, server-verify mismatch, strict-modes failure, missing `VIOLA_NAME`. As landed a strict-modes failure is not logged: `viola hook statusline` is the one arm that runs an instance check, and a refused check leaves no line (chunk 2026-10-10-budget-governor; the founder's own answer, 2026-10-13, relayed verbatim by the Viola overseer, the operator's answer at its planning superseded 2026-10-13).
  - A server-verify mismatch (Linux and Windows) is one `hook-decision{detail:"server-verify-failed"}` at WARN with no frame sent — on a dialog hook `hook-invoked{corr:null}` then `hook-decision{corr:null, decision_emitted:false, detail:"server-verify-failed"}` — and no pid, start time, path or endpoint in the line; macOS compares no peer pid (the recorded gap).
  - Exit is 0 and stderr stays empty.
  - Drift reports go to `diagnostics/` only.

<!-- /detail -->

<!-- detail: values · triggers -->

- **Required telemetry signal:** Rejected tool calls are logged with codes only (`refusal` / `detail` / `instance-unreachable` / `wrapper-fault` / `invalid-params`, whose `detail` is a closed field word `target · text · after · timeout_ms · dialog_id · response`; chunk 2026-10-17-mcp-server-for-drivers), with no upstream text, absolute paths or type names.

<!-- /detail -->

<!-- detail: values · triggers -->

- **Required telemetry signal:**
  - A never-log floor for `CLAUDE_CODE_MESSAGING_TOKEN`, `CLAUDE_CODE_MESSAGING_SOCKET` and every R8-stripped `CLAUDE*` value, across all sinks and fixtures.
  - The R8 strip is logged as a count or variable-name list only, never values. Phase 3 must confirm names are admissible.
  - An anyhow chain containing a serde source becomes a fixed message before reaching stderr.
  - No env, flag or config may disable redaction.

<!-- /detail -->

<!-- detail: values · triggers -->

- **Required telemetry signal:**
  - `diagnostics/` files are 0600 in 0700 directories (Windows: home DACL), enforced by the tests secret-scan test.
  - Refusal telemetry for strict-modes failures, pinned-exe hash mismatches and ignored symlinks. As landed a strict-modes failure has a line only on the `ledger-stamps` read in `run`'s version gate. Four strict-modes refusals leave no line: `viola revive`'s instance check, `viola hook statusline`'s instance check, the checked read of `budget.json` (`read_budget_stamped`, which the budget gate takes through `read_budget` and `viola list` takes directly, where a refused file is no reading) (chunk 2026-10-10-budget-governor), and `viola list`'s scan of the home, `instances/` and each instance, which refuses the whole board with exit 21 (chunk 2026-10-11-the-board-viola-list).
  - `skipped.torn_lines` counts.

<!-- /detail -->

<!-- detail: values · triggers -->

- **Required telemetry signal:**
  - `process-start` / `process-exit` for the `claude` child and for `claude agents --json` / `--version` probes.
  - Parse failures surface as `unknown` plus a counter.
  - C0/C1 controls are escaped in any human terminal rendering of logged rows.

<!-- /detail -->

<!-- detail: values · triggers -->

- **Required telemetry signal:** The logger dependency is tracing-subscriber `>=0.3.20` (0.3.23 pinned by tests). Obs dependencies stay Tokio-free in the sync crates and add no C-building crates, and they pass `cargo deny` on all three targets.

<!-- /detail -->

<!-- detail: values · triggers -->

- **Required telemetry signal:**
  - `corr` = send `cursor` joins `send-issued` / `send-confirmed` / `send-refused` across CLI, MCP and wrapper.
  - `corr` = `dialog_id` joins hook → wrapper → driver.
  - The per-connection JSON-RPC `id` needs a discriminator for a merged-log join (section 2).

<!-- /detail -->

<!-- detail: values · triggers -->

- **Required telemetry signal:**
  - `duration_ms` on `hook-decision`, or on `process-exit` for `process="hook"`, per hook event. The budget is `session-end` `max < 1.0 s`, and spine hooks and `pre-tool-use` on an unverified CLI must stay under the spine deadline (provisionally 1.0 s).
  - The heartbeat staleness flip happens at >5 s (4.9 s live / 5.1 s stale or gone).

  Aggregation into p95/p99 is done by the harness or `jq` over the ndjson, not by an in-process metrics pipeline (no listener, no exporter).

<!-- /detail -->

<!-- detail: values · triggers -->

- **Required telemetry signal:** Fault telemetry per class:
  - torn ndjson line healed, with file and offset;
  - corrupt or unsupported-`v` snapshot → replay recovery;
  - child exit detected on the process handle without EOF (ConPTY);
  - endpoint vanished during `wait` (exit 21 / `instance-unreachable`);
  - stale heartbeat vs live pid.

  These need `event` values or an additive `detail` field (section 3, gap b).

<!-- /detail -->

<!-- detail: values · triggers -->

- **Required telemetry signal:** The same file sink and line schema on every OS. Platform differences must still yield identical fields:
  - permission application: Windows DACL vs Unix 0600/0700;
  - append atomicity with concurrent hook writers: Windows append vs POSIX `O_APPEND`;
  - named pipe vs Unix socket in channel logs;
  - ConPTY vs openpty in `pty` spans.
  - the server check's peer-pid read: Linux `SO_PEERCRED` vs Windows `GetNamedPipeServerProcessId` vs macOS none (`SERVER_PID_REPORTED` false, so the verdict is `Matches` with no comparison and no `server-verify-failed` line is written there: the recorded gap, owned by "Unix endpoint and home hardening").

  No OS-specific exporter.

<!-- /detail -->

<!-- detail: values · triggers -->

- **Required telemetry signal:** A zero-unlogged-panics invariant for all four process roles. Each panic produces exactly one single-line `event:"panic"` record and no multi-line backtrace, and `hook` still exits 0. No formal latency or availability SLO beyond the tests perf gates.

<!-- /detail -->

<!-- detail: values · triggers -->

- **Required telemetry signal:** No rotation of `events.ndjson` or `diagnostics/*.ndjson` in v1. Any future rotation must keep 0600/0700 permissions and keep `wait after` and `send cursor` offsets valid. The trigger is operational, not compliance: security tier is Minimal and there are no compliance triggers.

<!-- /detail -->

<!-- detail: values · triggers -->

- **Required telemetry signal:**
  - Structured JSON process logs from every process role (`run` / `hook` / `mcp` / `cli`) under `diagnostics/`, merged by harness `logs` with `events.ndjson`.
  - `send-issued` and `send-refused` are logged events.
  - The tests field and event list is adopted verbatim.

<!-- /detail -->

<!-- detail: values · triggers -->

- **Required telemetry signal:**
  - `release-from-driver` is its own logged event, not a generic -32602 wrapper fault.
  - Each cause behind exit 21 (dead instance, strict-mode fail, server-verify fail) and exit 1 (already live, squatted name, SHA-256 mismatch, `.cmd`/`.bat` child, and `viola revive`'s no logged session and missing recorded directory) gets a distinct `detail` code with no path or pid. `viola revive`'s strict-mode fail carries no logged `detail` as landed (no log line): its output is the stderr pair, or under `--json` the `{"v":1,"error":"strict-modes-failed","detail":null}` document (chunk 2026-10-17-cli-machine-contract), exit 1 (chunk 2026-10-10-budget-governor).
  - The server-verify fail is written on Linux and Windows as `process-exit{subject:"self", exit_code:21, detail:"server-verify-failed", during:"connect"}` in `cli-<name>.ndjson` by `send`, `wait`, `last`, `answer`, `pause` and `release`, distinct from `instance-dead`, with no pid, start time, path or endpoint; macOS writes none (no peer pid, the recorded gap).

<!-- /detail -->

<!-- detail: values · triggers -->

- **Required telemetry signal:**
  - Counters for unknown event kinds, unknown fields and torn lines (`skipped`).
  - The `RefusalReason` `unknown` fallback (exit 14).
  - Higher-`v` rejections (-32602; the code also carries `send`'s invalid params and `release` carrying `from`, so it is not a higher-`v` count alone).
  - Per-parser parse-failure counts: paste validation, channel framing at `MAX_FRAME`, hook stdin, `claude agents --json`, vt100 under `catch_unwind`.

<!-- /detail -->

<!-- detail: values · triggers -->

- **Required telemetry signal:**
  - Events are emitted deterministically, keyed on hook events and `events.ndjson` offsets.
  - No telemetry is derived from screen content: vt100 is used for gate outcomes only.
  - No log-based waits on sleeps.

<!-- /detail -->
