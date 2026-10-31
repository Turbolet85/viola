### obs-scope-summary#15.3 · values · scope-entities · ## 1. Obs Scope Summary · lines 131–135
- **Reason if not fully instrumentable:** N/A. Scrub rules per store (security Data Classifications):
  - `events.ndjson` is itself the product audit stream (brief §7, "the event log as the audit trail"). Its payload content is never copied into process-log lines.
  - Absolute paths in `snapshot.json` (`endpoint`, `pinned_bin`) and in config values are scrubbed from external output. They are allowed in `diagnostics/`.
  - A `snapshot.json` the replay rebuilds from `events.ndjson` (`read_snapshot_or_replay`) carries no `endpoint`, `pid`, `started_at` or `statusline_command`: `live_endpoint` over it is `None` (unreachable), hook `connect` is unreachable (fail open, `channel-unreachable`), `hook statusline` runs and prints nothing, and `run`'s collision check reads the heartbeat alone.
  - Operational metadata in `budget.json` (percentages, `resets_at`) may be logged as structured fields.
### obs-scope-summary#16.3 · values · scope-entities · ## 1. Obs Scope Summary · lines 143–143
- **Reason if not fully instrumentable:** Static configuration and data. Only the parse at load time is observable, and it is logged through `viola-state` / root bin (skipped-key counts).
### obs-scope-summary#17.3 · values · scope-entities · ## 1. Obs Scope Summary · lines 151–151
- **Reason if not fully instrumentable:** Not product runtime. Fixtures are scrub-required data (security Data Classifications, recorded fixtures), not a telemetry source. Excluded from sections 2 to 4.
### obs-scope-summary#18.1 · values · scope-surfaces · ## 1. Obs Scope Summary · lines 157–157
- **Backend instrumentation hooks:** Rust `tracing` facade with `#[tracing::instrument]` spans on the dispatch functions, plus a tracing-subscriber JSON structured logger (tests excerpt, Log format: `tracing-subscriber 0.3.23 fmt().json().flatten_event(true).with_current_span(false)`). Each process gets its own panic hook: a `std::panic::set_hook` replacement that writes one line. No HTTP framework is involved, so there is no auto-instrumentation.
### obs-scope-summary#18.3 · values · scope-surfaces · ## 1. Obs Scope Summary · lines 159–159
- **Exporter:** A file sink only: JSON-per-line to `diagnostics/` when an instance is resolved (security Vector 6: "write the full error only to `diagnostics/` when an instance is resolved"). Without an instance there is no process-log file, and human stderr carries only fixed-message lines (design excerpt, CLI faults). Network OTLP is excluded: security says "viola makes no outbound network calls". The stdout exporter is excluded because stdout carries `--json` results. `viola revive <name> --list` logs as `cli` into the caller's `cli-<VIOLA_NAME>.ndjson` once its strict-modes check has passed (none with no `VIOLA_NAME`; a refused check opens none; chunk 2026-10-17-cli-machine-contract). `viola link` / `viola unlink` log as `cli` into the driven instance's `cli-<driven>.ndjson` (the instance their `<driven>` argument names, never the caller's `VIOLA_NAME`) once their instance check of it has passed; a refused check opens none. `send`, `wait`, `last`, `answer`, `pause` and `release` log as `cli` once their instance check (`refuse_untrusted`: `check_instance` on the home and `instances/<name>`) has passed; a refused check exits 21 with the instance pair, or under `--json` the `{"v":1,"error":"instance-unreachable","detail":null}` document, opens nothing under the home and writes no line. `viola revive`'s start arm is not a short-lived verb: it is the wrapper surface below (`process="run"`, `diagnostics/run-<name>.ndjson`). `viola verify` logs as `cli` (with an instance) once its home-tree check (`check_home_tree` on the home, `ledger/`, `bin/` and `plugin/`) has passed; a refused check exits 1 with the home pair, opens no log and writes no line, and a refused `update_stamps` (`StampsError::StrictModes`) exits 1 with the same pair.
### obs-scope-summary#18.4 · values · scope-surfaces · ## 1. Obs Scope Summary · lines 160–160
- **Service identity:** `service.name` is hardcoded `"viola"`, the binary name. `service.version` is compile-time `env!("CARGO_PKG_VERSION")`, the same value already used as `sender` and `writer`. The process role is the `process` field. The instance is `ViolaName` from an explicit argument or `VIOLA_NAME`. `deployment.environment` is N/A: local-only, with no environments (arch Obs-Relevant Conventions, Service identity; arch Surfaces, "no hosting").
### obs-scope-summary#18.5 · values · scope-surfaces · ## 1. Obs Scope Summary · lines 161–165
- **Notes:**
  - The instance comes from `VIOLA_NAME` or an argument, never from the working directory (arch Instance identity rule).
  - Refusal and fault lines on stderr are fixed strings with no paths, pids, chains or upstream text (design excerpt, CLI refusals / CLI faults).
  - Exit codes 0, 1, 2, 10–14, 20 and 21 are the machine-readable outcome and should be a log field (arch Standard Contracts, CLI `--json` + exit codes).
  - Founder Direction 4: each cause behind exit 21 and exit 1 gets a distinct detail code, except a strict-modes check refused before any log opens, which writes no line: its cause is carried only by the fixed stderr pair or the `--json` document (`send`, `wait`, `last`, `answer`, `pause`, `release`, `link` and `unlink`, exit 21; `run`, `viola revive` and `viola verify`, exit 1).
### obs-scope-summary#19.1 · values · scope-surfaces · ## 1. Obs Scope Summary · lines 171–171
- **Backend instrumentation hooks:** `tracing` spans (`#[tracing::instrument]`) on the start sequence, the `viola-pty` seam operations, the `viola-channel` sync server handlers, vt100 readiness-gate outcomes and `viola-state` writes, all written through the tracing-subscriber JSON logger. Instrumentation must be sync: std threads, no Tokio (arch Stack, Hybrid concurrency; `cargo deny` ban).
### obs-scope-summary#19.3 · values · scope-surfaces · ## 1. Obs Scope Summary · lines 173–173
- **Exporter:** A file sink only, to `diagnostics/run-<name>.ndjson` (tests excerpt, Log format). The terminal is forbidden: "`run` writes nothing to the terminal while the child runs" (arch Observability Hints).
### obs-scope-summary#19.4 · values · scope-surfaces · ## 1. Obs Scope Summary · lines 174–174
- **Service identity:** as for the short-lived verbs, with `process="run"` and `instance=<ViolaName>`.
### obs-scope-summary#20.3 · values · scope-surfaces · ## 1. Obs Scope Summary · lines 186–186
- **Exporter:** A file sink only, to `diagnostics/hook-<name>.ndjson`. stderr is forbidden on both. stdout carries only the decision body for `viola hook <event>`, and for `viola hook statusline` only the user's status-line command's output (arch Standard Contracts, Hook contract); the two hidden `--capture` arms (`verify`'s probe capture) write no sink and print nothing, except `--answers`' probe decision body. A failed log write is swallowed, and the hook still exits 0 (tests excerpt, Log format Constraints).
### obs-scope-summary#20.4 · values · scope-surfaces · ## 1. Obs Scope Summary · lines 187–187
- **Service identity:** as for the short-lived verbs, with `process="hook"`. If `VIOLA_NAME` is missing, `instance` is null and the hook is a silent no-op (tests excerpt, E1 R5 unwrapped session hooks; Coverage Triggers, Vector 4 "missing `VIOLA_NAME`"). The event and dialog paths resolve the instance directory (`canonical`) and run its strict-modes check (`trusted`: `check_instance`) before `read_diagnostics_level` and obs init: a refused or absent instance is a silent exit 0 with no `config.json` read, no role file, no stdin read, no line, no frame and no SessionEnd append. The two `--capture` arms run ahead of the check; the `FAKE_AGENT_HOOK_PANIC` seam runs after obs init.
### obs-scope-summary#20.5 · values · scope-surfaces · ## 1. Obs Scope Summary · lines 188–191
- **Notes:**
  - Many hook processes can append to the same `hook-<name>.ndjson` at once. Each line must be one `write` call on an append-mode handle (arch Stack, atomic-write-file / `OpenOptions::append`).
  - The latency budget is `max < 1.0 s` per hook (tests excerpt, Quality Gates). Logger init must not add blocking I/O beyond one file open and append; the instance check ahead of it (`canonical` + `trusted`: stat and, on Windows, owner and DACL reads) is in the timed path, and `run --perf` keeps the five hook rows under `SPINE_DEADLINE` with it, max 2.5 / 2.4 / 2.4 / 2.4 / 3.3 ms (as measured at `.andromeda/runs/2026-10-26T19-02-07-implement/gate-2026-10-26-home-and-code-bearing-file-integrity.json`). `viola hook statusline` is outside that budget: its shell-out of the user's command is bounded by the 5 s PROVISIONAL `STATUSLINE_DEADLINE`, and no `run --perf` row times it.
  - Where no instance directory resolves, file logging is impossible. That case gets no log and no stderr, by contract. A `VIOLA_DIR` naming an instance directory that does not exist resolves none, so the hook writes nothing.
### obs-scope-summary#21.1 · values · scope-surfaces · ## 1. Obs Scope Summary · lines 197–197
- **Backend instrumentation hooks:** language-native `tracing` spans on client call and server dispatch. Propagation uses the `corr` field in log lines (JSON-RPC `id` for `channel-request` / `channel-response`; `dialog_id`; send `cursor`), per the tests excerpt Log format. Channel `params` declare no trace-context field (arch Standard Contracts, `send`: "No trace-context field is declared"). Readers skip unknown fields (arch Mixed-version tolerance), so an additive envelope field would be wire-compatible, but none is required by upstream.
### obs-scope-summary#21.3 · values · scope-surfaces · ## 1. Obs Scope Summary · lines 199–199
- **Exporter:** The log file of whichever process is on each side (`run-<name>`, `hook-<name>`, `mcp`, or none for short-lived CLI verbs without an instance).
### obs-scope-summary#21.4 · values · scope-surfaces · ## 1. Obs Scope Summary · lines 200–200
- **Service identity:** Each side logs its own `process`. The peer version is available as `params.sender` (`CARGO_PKG_VERSION`).
### obs-scope-summary#21.5 · values · scope-surfaces · ## 1. Obs Scope Summary · lines 201–204
- **Notes:**
  - JSON-RPC ids are "integers, monotonic per connection" (arch Standard Contracts, JSON-RPC frame shapes). `corr` for `channel-request` / `channel-response` is therefore not unique across connections or processes, and a merged-log join needs `process`, `instance` and a connection discriminator. This is a gap for Phase 3; the tests contract allows additive fields.
  - `-32603` must be `"internal error"` with `data:null` (security Vector 1).
  - `from` is self-reported and must not be logged as an authenticated identity (security Vector 1).
### obs-scope-summary#22.1 · values · scope-surfaces · ## 1. Obs Scope Summary · lines 210–210
- **Backend instrumentation hooks:** `tracing` spans per MCP tool call (`send · wait · last · answer · list`) wrapping the `viola-channel` Tokio client, through the tracing-subscriber JSON logger. stdio has no HTTP-framework auto-instrumentation.
### obs-scope-summary#22.3 · values · scope-surfaces · ## 1. Obs Scope Summary · lines 212–212
- **Exporter:** Always the file sink `<home>/diagnostics/mcp.ndjson` (tests excerpt; D-09), the home resolved through `resolved_home` like every verb's and checked with its `diagnostics/` by strict-modes before the sink opens; stderr JSON only when that file cannot be opened (as landed, chunk 2026-10-17-mcp-server-for-drivers). stdout is forbidden because it carries MCP frames. tracing-subscriber `fmt()` writes to stdout unless a writer is configured, so the writer must be set explicitly. rmcp reads stdin only through viola's line cap (`CappedLines` at `MAX_FRAME`): a line of at most `MAX_FRAME` bytes, its newline counted, passes whole; a longer line is refused whole, never truncated or skipped, and ends the session as a closed stdin does, stderr empty, its one record the §6 `parse-rejected{parser:"mcp-stdin", detail:"oversize"}` line.
### obs-scope-summary#22.4 · values · scope-surfaces · ## 1. Obs Scope Summary · lines 213–213
- **Service identity:** as for the short-lived verbs, with `process="mcp"`. `instance` is the server's own `VIOLA_NAME` when set, which is also the `from` it adds.
### obs-scope-summary#22.5 · values · scope-surfaces · ## 1. Obs Scope Summary · lines 214–216
- **Notes:**
  - `isError` results carry codes only: `refusal` / `detail`, `instance-unreachable`, `wrapper-fault`, `invalid-params` (its `detail` a closed field word; chunk 2026-10-17-mcp-server-for-drivers) (security Vector 5). Log lines follow the same codes-only rule for rejected calls (tests excerpt, Coverage Triggers, MCP).
  - The `mcp` sink follows tests (`mcp.ndjson`), reconciled by D-09 and landed (stderr JSON only when the file cannot be opened).
### obs-scope-summary#23.1 · values · strategy · ## 1. Obs Scope Summary · lines 222–238
- **OTel SDK init:** v1 initialises no OTel SDK exporter. The `tracing` facade plus the tracing-subscriber JSON formatter is the binding pipeline (tests excerpt, Log format; Founder Direction 1: "tests owns them"). The constraints ruling out an exporter:
  - no outbound network calls (security Error reporting);
  - a new listener, including a local OTLP receiver or metrics endpoint, needs a Security Decisions Log entry (security Anti-Patterns);
  - stdout is reserved on `hook`, `mcp` and `run` (arch Obs-Relevant Conventions);
  - `cargo deny` bans Tokio in the sync crates' graph and bans C-building crates (arch Observability Hints, Build policy).

  An OTel bridge layer is optional. It may only feed an in-process or file sink, and it must stay Tokio-free on the sync crates. Phase 2 evaluates whether it adds anything over the tests-fixed format.

  Init order per process:
  1. As the first statement in `main`, install a replacement panic hook that writes one JSON line and nothing to stderr. The default hook's stderr output breaks the hook contract and the `run` terminal.
  2. Parse clap.
  3. Resolve the home (`--home`, then the grandparent of `VIOLA_DIR`, then the default) and the instance (`VIOLA_NAME` or an argument). Where the role checks the home before its first open, that strict-modes check runs here — every `viola hook` arm but the two `--capture` arms (`canonical` + `trusted`); `send`, `wait`, `last`, `answer`, `pause`, `release`, `link` and `unlink` (`refuse_untrusted`); `verify` (`check_home_tree`); `run` and `viola revive`'s start (`checked_exe` + `check_home_tree`) — and a refusal ends the process with nothing under the home opened and no line.
  4. Open `diagnostics/<process file>` in append mode with 0600 permissions inside a 0700 directory.
  5. Install the subscriber with an explicit writer (never stdout).
  6. Emit `process-start`.

  For `run`, this completes before the start sequence, so the collision check and version gate are logged. For `mcp`, it completes before the rmcp stdio transport starts.
### obs-scope-summary#24.1 · values · logs · ## 1. Obs Scope Summary · lines 244–253
- **Logging stack:** `tracing` with tracing-subscriber 0.3.23, configured as `fmt().json().flatten_event(true).with_current_span(false)` (tests excerpt, verbatim). Version 0.3.23 satisfies the security Vector 9 floor of `>=0.3.20` (RUSTSEC-2025-0055). Sink rules:
  - one file per process role under `diagnostics/`;
  - no stdout sink on any role;
  - stderr JSON only for `mcp`, and only when `mcp.ndjson` cannot be opened (D-09).

  The writer must emit each formatted line as a single `write` on an append-mode handle (arch Crash-safe disk writes). Concurrent `hook` processes share one file.

  The level filter is not read from `RUST_LOG`. Arch says "environment variables are not a configuration channel", and security Anti-Patterns ban env, flag or config values that disable any control. Any verbosity control therefore comes from `--home/config.json` or a CLI flag and must never bypass redaction. There is no separate agent-mode flag: JSON-to-file is the only mode.

  Timestamps must be RFC 3339 UTC with milliseconds and `Z` (tests excerpt, `timestamp`; arch Timestamps). Phase 2 must verify whether the tracing-subscriber default timer meets millisecond precision, or whether a chrono-based `FormatTime` (`to_rfc3339_opts(SecondsFormat::Millis, true)`) is needed.
### obs-scope-summary#25.0 · mirror · log-fields · ## 1. Obs Scope Summary · lines 259–279
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
