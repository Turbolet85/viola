### obs-scope-summary#1.4 · values · tier · ## 1. Obs Scope Summary · lines 11–11 · add-to
- 18 telemetry triggers, including cross-surface propagation through `corr`, perf budgets, chaos fault telemetry, multi-OS compat, and binding Founder Directions.
### obs-scope-summary#6.1 · values · scope-entities · ## 1. Obs Scope Summary · lines 50–50 · add-to
- **Source:** arch excerpt, Stack (interprocess 2.4.4), Workspace, Standard Contracts (IPC methods `send · wait · last · answer · pause · release · link · unlink · hook.dialog · hook.event`)
### obs-scope-summary#11.3 · values · scope-entities · ## 1. Obs Scope Summary · lines 99–99 · add-to
- **Reason if not fully instrumentable:** The caller is Claude Code, so viola sees only the inbound stdin payload and its own response. Instrument `hook-invoked` / `hook-decision` and MCP tool-call handling. No trace context arrives from Claude Code.
### obs-scope-summary#14.3 · values · scope-entities · ## 1. Obs Scope Summary · lines 123–123 · add-to
- **Reason if not fully instrumentable:** Instrument viola's call sites (bind, connect, lock, rename, kill-fallback use, and the server check's peer-pid and start-time reads, whose failed check the CLI and the hook log as `server-verify-failed` and MCP logs by no line), not the OS internals.
### obs-scope-summary#15.3 · values · scope-entities · ## 1. Obs Scope Summary · lines 131–134 · add-to
- **Reason if not fully instrumentable:** N/A. Scrub rules per store (security Data Classifications):
  - `events.ndjson` is itself the product audit stream (brief §7, "the event log as the audit trail"). Its payload content is never copied into process-log lines.
  - Absolute paths in `snapshot.json` (`endpoint`, `pinned_bin`) and in config values are scrubbed from external output. They are allowed in `diagnostics/`.
  - Operational metadata in `budget.json` (percentages, `resets_at`) may be logged as structured fields.
### obs-scope-summary#18.3 · values · scope-surfaces · ## 1. Obs Scope Summary · lines 158–158 · add-to
- **Exporter:** A file sink only: JSON-per-line to `diagnostics/` when an instance is resolved (security Vector 6: "write the full error only to `diagnostics/` when an instance is resolved"). Without an instance there is no process-log file, and human stderr carries only fixed-message lines (design excerpt, CLI faults). Network OTLP is excluded: security says "viola makes no outbound network calls". The stdout exporter is excluded because stdout carries `--json` results. `viola revive <name> --list` logs as `cli` into the caller's `cli-<VIOLA_NAME>.ndjson` once its strict-modes check has passed (none with no `VIOLA_NAME`; a refused check opens none; chunk 2026-10-17-cli-machine-contract). `viola link` / `viola unlink` log as `cli` into the driven instance's `cli-<driven>.ndjson` (the instance their `<driven>` argument names, never the caller's `VIOLA_NAME`) once their instance check of it has passed; a refused check opens none. `viola revive`'s start arm is not a short-lived verb: it is the wrapper surface below (`process="run"`, `diagnostics/run-<name>.ndjson`).
### obs-scope-summary#18.5 · values · scope-surfaces · ## 1. Obs Scope Summary · lines 160–164 · contradict
- **Notes:**
  - The instance comes from `VIOLA_NAME` or an argument, never from the working directory (arch Instance identity rule).
  - Refusal and fault lines on stderr are fixed strings with no paths, pids, chains or upstream text (design excerpt, CLI refusals / CLI faults).
  - Exit codes 0, 1, 2, 10–14, 20 and 21 are the machine-readable outcome and should be a log field (arch Standard Contracts, CLI `--json` + exit codes).
  - Founder Direction 4: each cause behind exit 21 and exit 1 gets a distinct detail code.
### obs-scope-summary#19.3 · values · scope-surfaces · ## 1. Obs Scope Summary · lines 172–172 · add-to
- **Exporter:** A file sink only, to `diagnostics/run-<name>.ndjson` (tests excerpt, Log format). The terminal is forbidden: "`run` writes nothing to the terminal while the child runs" (arch Observability Hints).
### obs-scope-summary#20.1 · values · scope-surfaces · ## 1. Obs Scope Summary · lines 183–183 · add-to
- **Backend instrumentation hooks:** `tracing` spans around stdin parse → channel call (`hook.dialog` / `hook.event`) → decision emit, through the tracing-subscriber JSON logger. `hook-invoked` / `hook-decision` lines are required, and panics are caught with `catch_unwind` and emitted as one `level:"ERROR", event:"panic"` line (tests excerpt, Log format, Agent parsing).
### obs-scope-summary#20.3 · values · scope-surfaces · ## 1. Obs Scope Summary · lines 185–185 · add-to
- **Exporter:** A file sink only, to `diagnostics/hook-<name>.ndjson`. stderr is forbidden on both. stdout carries only the decision body for `viola hook <event>`, and for `viola hook statusline` only the user's status-line command's output (arch Standard Contracts, Hook contract); the two hidden `--capture` arms (`verify`'s probe capture) write no sink and print nothing, except `--answers`' probe decision body. A failed log write is swallowed, and the hook still exits 0 (tests excerpt, Log format Constraints).
### obs-scope-summary#20.4 · values · scope-surfaces · ## 1. Obs Scope Summary · lines 186–186 · add-to
- **Service identity:** as for the short-lived verbs, with `process="hook"`. If `VIOLA_NAME` is missing, `instance` is null and the hook is a silent no-op (tests excerpt, E1 R5 unwrapped session hooks; Coverage Triggers, Vector 4 "missing `VIOLA_NAME`").
### obs-scope-summary#20.5 · values · scope-surfaces · ## 1. Obs Scope Summary · lines 187–190 · add-to
- **Notes:**
  - Many hook processes can append to the same `hook-<name>.ndjson` at once. Each line must be one `write` call on an append-mode handle (arch Stack, atomic-write-file / `OpenOptions::append`).
  - The latency budget is `max < 1.0 s` per hook (tests excerpt, Quality Gates). Logger init must not add blocking I/O beyond one file open and append. `viola hook statusline` is outside that budget: its shell-out of the user's command is bounded by the 5 s PROVISIONAL `STATUSLINE_DEADLINE`, and no `run --perf` row times it.
  - Where no instance directory resolves, file logging is impossible. That case gets no log and no stderr, by contract.
### obs-scope-summary#22.3 · values · scope-surfaces · ## 1. Obs Scope Summary · lines 211–211 · stale
- **Exporter:** Always the file sink `<home>/diagnostics/mcp.ndjson` (tests excerpt; D-09), the home resolved through `resolved_home` like every verb's and checked with its `diagnostics/` by strict-modes before the sink opens; stderr JSON only when that file cannot be opened (as landed, chunk 2026-10-17-mcp-server-for-drivers). stdout is forbidden because it carries MCP frames. tracing-subscriber `fmt()` writes to stdout unless a writer is configured, so the writer must be set explicitly.
### obs-scope-summary#22.5 · values · scope-surfaces · ## 1. Obs Scope Summary · lines 213–215 · add-to
- **Notes:**
  - `isError` results carry codes only: `refusal` / `detail`, `instance-unreachable`, `wrapper-fault`, `invalid-params` (its `detail` a closed field word; chunk 2026-10-17-mcp-server-for-drivers) (security Vector 5). Log lines follow the same codes-only rule for rejected calls (tests excerpt, Coverage Triggers, MCP).
  - The `mcp` sink follows tests (`mcp.ndjson`), reconciled by D-09 and landed (stderr JSON only when the file cannot be opened).
### obs-scope-summary#23.1 · values · strategy · ## 1. Obs Scope Summary · lines 221–237 · add-to
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
### obs-scope-summary#24.1 · values · logs · ## 1. Obs Scope Summary · lines 243–252 · stale
- **Logging stack:** `tracing` with tracing-subscriber 0.3.23, configured as `fmt().json().flatten_event(true).with_current_span(false)` (tests excerpt, verbatim). Version 0.3.23 satisfies the security Vector 9 floor of `>=0.3.20` (RUSTSEC-2025-0055). Sink rules:
  - one file per process role under `diagnostics/`;
  - no stdout sink on any role;
  - stderr JSON only for `mcp`, and only when `mcp.ndjson` cannot be opened (D-09).

  The writer must emit each formatted line as a single `write` on an append-mode handle (arch Crash-safe disk writes). Concurrent `hook` processes share one file.

  The level filter is not read from `RUST_LOG`. Arch says "environment variables are not a configuration channel", and security Anti-Patterns ban env, flag or config values that disable any control. Any verbosity control therefore comes from `--home/config.json` or a CLI flag and must never bypass redaction. There is no separate agent-mode flag: JSON-to-file is the only mode.

  Timestamps must be RFC 3339 UTC with milliseconds and `Z` (tests excerpt, `timestamp`; arch Timestamps). Phase 2 must verify whether the tracing-subscriber default timer meets millisecond precision, or whether a chrono-based `FormatTime` (`to_rfc3339_opts(SecondsFormat::Millis, true)`) is needed.
