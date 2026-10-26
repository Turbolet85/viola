# obs · cascade read · group 1 of 7

## Carried
- (.andromeda/obs-plan-detail/obs-scope-summary.md:18) The MCP tools' refusal on a failed or impossible verification must log its codes-only `instance-unreachable` line to `mcp.ndjson`, with stderr JSON only when that file cannot be opened.
  > - `mcp` logging to stderr when `VIOLA_DIR` is unset — reconciled by D-09 and landed (chunk 2026-10-17-mcp-server-for-drivers): `mcp` always writes `mcp.ndjson`, stderr JSON only when that file cannot be opened;
  (.andromeda/obs-plan.md:127)
  > - **Exporter:** always the file sink `<home>/diagnostics/mcp.ndjson`, stderr JSON only when that file cannot be opened; stdout is forbidden because it carries MCP frames, so the writer is set explicitly; `isError` results and the log lines for rejected calls carry codes only.
- (.andromeda/obs-plan-detail/obs-scope-summary.md:20) The dialog hook client must keep `hook-invoked`'s `corr` timing as reconciled (logged after the `hook.dialog` response), so a verification refused before the `hook.dialog` frame leaves a fail-open line that carries no `dialog_id` the wrapper never assigned.
  > - `hook-invoked` `corr` timing relative to `dialog_id` assignment.
  (.andromeda/obs-plan.md:142)
  > (c) `hook-invoked` is emitted before the wrapper assigns `dialog_id`, so `corr` can only be filled from `hook-decision` onward, or `hook-invoked` must be logged after the `hook.dialog` response.
  (.andromeda/obs-plan.md:264)
  > Until a wrap brings a difference current, §3, §6 and §12 win where they differ: diagnostics root (D-08, D-22), `event` enum extensions (D-01…D-05, D-21), `hook-invoked` `corr` timing (D-07), the channel join key (D-25: `(conn, corr)`, each side logs its own `instance`)
- (.andromeda/obs-plan-detail/obs-scope-summary.md:27) The root bin's edited CLI verbs and hook must instrument the new verification outcome, and as the anyhow boundary must turn any chain into a fixed message before stderr.
  > - **Instrumentability:** instrumentable
  (.andromeda/obs-plan.md:47)
  > - **Instrumentability:** instrumentable — the only crate that uses anyhow, so the error-capture boundary: a chain that contains a serde source becomes a fixed message before it reaches stderr, and the full chain goes to `diagnostics/` only.
- (.andromeda/obs-plan-detail/obs-scope-summary.md:51) The server identity read added to `viola-channel`'s connect and endpoint must be instrumented sync, with no Tokio outside the `tokio` feature, and its channel lines must carry `corr` + `conn` and never `params` or result bodies.
  > - **Instrumentability:** instrumentable
  (.andromeda/obs-plan-detail/obs-scope-summary.md:52)
  > - **Reason if not fully instrumentable:** N/A. Stack forbids Tokio in this crate's graph unless its feature is enabled, so instrumentation must be sync.
  (.andromeda/obs-plan.md:59)
  > - **Instrumentability:** instrumentable, and the instrumentation must be sync: Tokio is forbidden in this crate's graph unless its feature is enabled.
  (.andromeda/obs-plan.md:123)
  > - **Exporter:** the log file of whichever process is on each side; propagation is the `corr` field in log lines, since channel `params` declare no trace-context field, and a merged-log join needs `process`, `instance` and a connection discriminator, a gap for Phase 3; `-32603` is `"internal error"` with `data:null`, and the self-reported `from` is never logged as an authenticated identity.
  (.andromeda/obs-plan.md:190)
  > - **Required telemetry signal:** channel lines never carry `params` or result bodies, and `from` is logged as unauthenticated.
  (.andromeda/obs-plan.md:225)
  > - **Required telemetry signal:** obs dependencies stay Tokio-free in the sync crates, add no C-building crates and pass `cargo deny`.
- (.andromeda/obs-plan-detail/obs-scope-summary.md:59) The verification joined into `viola-state`'s `live_endpoint` must be instrumented sync, adding no Tokio or obs dependency to the sync graph.
  > - **Instrumentability:** instrumentable
  (.andromeda/obs-plan-detail/obs-scope-summary.md:60)
  > - **Reason if not fully instrumentable:** N/A. Stack bans Tokio here, so instrumentation must be sync.
  (.andromeda/obs-plan.md:63)
  > - **Instrumentability:** instrumentable, and the instrumentation must be sync: Stack bans Tokio here.
- (.andromeda/obs-plan-detail/obs-scope-summary.md:77) The `viola-mcp` tools' new refusal must reach the driver only as an `isError` code, never on stdout as telemetry, mirrored by a codes-only log line with no upstream text, path or type name.
  > - **Instrumentability:** instrumentable
  (.andromeda/obs-plan.md:71)
  > - **Instrumentability:** instrumentable; stdout is reserved for MCP frames, so no telemetry may go to stdout.
  (.andromeda/obs-plan.md:205)
  > - **Required telemetry signal:** rejected tool calls are logged with codes only, with no upstream text, absolute paths or type names.
- (.andromeda/obs-plan-detail/obs-scope-summary.md:98) The hook client's fail-open path on a mismatch must be observed only at viola's boundary, through `hook-invoked` / `hook-decision` in `hook-<name>.ndjson`, exit 0 with stderr empty, and the added reads must stay inside the hook perf budget.
  > - **Instrumentability:** boundary-only
  (.andromeda/obs-plan-detail/obs-scope-summary.md:99)
  > - **Reason if not fully instrumentable:** The caller is Claude Code, so viola sees only the inbound stdin payload and its own response. Instrument `hook-invoked` / `hook-decision` and MCP tool-call handling. No trace context arrives from Claude Code.
  (.andromeda/obs-plan.md:79)
  > - **Instrumentability:** boundary-only — viola sees only the inbound stdin payload and its own response; no trace context arrives from Claude Code.
  (.andromeda/obs-plan.md:119)
  > - **Exporter:** a file sink only, to `diagnostics/hook-<name>.ndjson`; stderr is forbidden, and stdout carries only the decision body (for `viola hook statusline`, the user's status-line command's output); a failed log write is swallowed and the hook still exits 0; without `VIOLA_NAME` the hook is a silent no-op; each line is one `write` on an append-mode handle, and logger init adds no blocking I/O beyond one file open and append.
  (.andromeda/obs-plan.md:200)
  > - **Required telemetry signal:** fail-open paths log to `hook-<name>.ndjson` only, exit 0 with stderr empty, and drift reports go to `diagnostics/` only.
  (.andromeda/obs-plan.md:230)
  > - **Trigger type:** perf-budget-instruments
  > - **Source:** tests excerpt, Coverage Triggers (hook deadlines / heartbeat) and Quality Gates (hyperfine `max`); brief O6 ("hook latency per tool call")
- (.andromeda/obs-plan-detail/obs-scope-summary.md:122) The new peer-credential, pipe server pid and sysinfo start-time reads must be instrumented at viola's own connect call site as viola events, the same fields on every OS, with no OS-specific field for the macOS gap.
  > - **Instrumentability:** boundary-only
  (.andromeda/obs-plan-detail/obs-scope-summary.md:123)
  > - **Reason if not fully instrumentable:** Instrument viola's call sites (bind, connect, lock, rename, kill-fallback use), not the OS internals.
  (.andromeda/obs-plan.md:91)
  > - **Instrumentability:** boundary-only — viola's call sites, not the OS internals.
  (.andromeda/obs-plan.md:239)
  > - **Required telemetry signal:** the same file sink and line schema on every OS, with identical fields and no OS-specific exporter.
- (.andromeda/obs-plan-detail/obs-scope-summary.md:130) The clients' `snapshot.json` read of `pid`, `started_at` and `endpoint` must be instrumented through the `viola-state` call sites, the `endpoint` path kept off external output and no strict-modes line added beyond the catalog P4 settles.
  > - **Instrumentability:** instrumentable (through the `viola-state` call sites)
  (.andromeda/obs-plan-detail/obs-scope-summary.md:133)
  > Absolute paths in `snapshot.json` (`endpoint`, `pinned_bin`) and in config values are scrubbed from external output. They are allowed in `diagnostics/`.
  (.andromeda/obs-plan.md:95)
  > - **Instrumentability:** instrumentable through the `viola-state` call sites; `events.ndjson` payload content is never copied into process-log lines, and the absolute paths in `snapshot.json` and config values are scrubbed from external output.
  (.andromeda/obs-plan.md:215)
  > a strict-modes failure has a line only on the `ledger-stamps` read in `run`'s version gate.
- (.andromeda/obs-plan-detail/obs-scope-summary.md:158) The CLI verbs' exit-21 line on a failed verification must go to the cli file sink only when an instance resolves, with no process-log file without one and only a fixed-message line on stderr.
  > - **Exporter:** A file sink only: JSON-per-line to `diagnostics/` when an instance is resolved (security Vector 6: "write the full error only to `diagnostics/` when an instance is resolved"). Without an instance there is no process-log file, and human stderr carries only fixed-message lines (design excerpt, CLI faults).
  (.andromeda/obs-plan.md:111)
  > - **Exporter:** a file sink only, JSON-per-line to `diagnostics/` when an instance resolves — no network OTLP (viola makes no outbound network calls) and no stdout exporter (stdout carries `--json` results); the instance comes from `VIOLA_NAME` or an argument, never from the working directory; refusal and fault lines on stderr are fixed strings with no paths, pids, chains or upstream text.
- (.andromeda/obs-plan-detail/obs-scope-summary.md:162) A failed or impossible server verification is a new cause behind exit 21: it must log the exit code as a field with its own distinct `detail` code, and its stderr line must hold no path, pid or start time.
  > - Refusal and fault lines on stderr are fixed strings with no paths, pids, chains or upstream text (design excerpt, CLI refusals / CLI faults).
  >   - Exit codes 0, 1, 2, 10–14, 20 and 21 are the machine-readable outcome and should be a log field (arch Standard Contracts, CLI `--json` + exit codes).
  >   - Founder Direction 4: each cause behind exit 21 and exit 1 gets a distinct detail code.
  (.andromeda/obs-plan.md:253)
  > - **Required telemetry signal:** `release-from-driver` is its own logged event, and each cause behind exit 21 and exit 1 gets a distinct `detail` code with no path or pid.
- (.andromeda/obs-plan.md:310) The scope's hook fail-open on a mismatch must write nothing on stdout or stderr, the decision body being absent with no frame written.
  > `hook` writes only its decision body to stdout and nothing to stderr.

## Bindings
- **obs-scope exit-21 cause ↔ §6 Log Coverage**: the distinct `detail` code Founder Direction 4 requires for the failed verification is added to §6's closed exit-21 catalog and admitted by `schemas/diag-line.v1.json`, (.andromeda/obs-plan-detail/obs-scope-summary.md:162)
- **obs-scope hook boundary ↔ §6 Log Coverage and §10 SLO budgets**: the fail-open `hook-decision` `detail` on a mismatch joins §6's catalog, and the added identity reads stay inside the hook deadlines §10 gates, (.andromeda/obs-plan-detail/obs-scope-summary.md:99)
- **obs-scope `hook-invoked` timing ↔ §4 Span / Trace Coverage (dialog → `answer` scenario)**: the D-07 timing reconciled for the dialog scenario decides what the hook logs when the `hook.dialog` response never comes, (.andromeda/obs-plan-detail/obs-scope-summary.md:20)
- **obs-scope `viola-channel` / `viola-state` sync rule ↔ the security plan's Vector 9 and `cargo deny`**: the server identity read adds no Tokio or obs dependency to the sync graph, (.andromeda/obs-plan.md:225)
- **obs-scope persistent stores ↔ §8 PII Scrubbing**: the `endpoint` path and the snapshot `pid` the clients read stay off external output and home-level lines, (.andromeda/obs-plan-detail/obs-scope-summary.md:133)
- **obs-scope Vector 7 strict-modes line ↔ the security plan's dated exceptions (P4's held strict-modes half)**: if the clients take the strict-modes check, the one-site rule for a strict-modes failure line changes with the line catalog, (.andromeda/obs-plan.md:215)
- **obs-scope MCP sink ↔ §7 Error Capture & Reporting**: the MCP `isError` code and its `mcp.ndjson` line carry codes only, the typed verification error's chain going to the detail file alone, (.andromeda/obs-plan.md:127)
