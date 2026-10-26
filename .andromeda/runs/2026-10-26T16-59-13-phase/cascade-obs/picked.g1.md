### obs-scope-summary#1.4 · values · tier · ## 1. Obs Scope Summary · lines 11–11
- 18 telemetry triggers, including cross-surface propagation through `corr`, perf budgets, chaos fault telemetry, multi-OS compat, and binding Founder Directions.
### obs-scope-summary#2.1 · pending · tier · ## 1. Obs Scope Summary · lines 17–17
- the `diagnostics/` directory root (tests `<home>/diagnostics/` vs arch and security `instances/<name>/diagnostics/`);
### obs-scope-summary#2.2 · pending · tier · ## 1. Obs Scope Summary · lines 18–18
- `mcp` logging to stderr when `VIOLA_DIR` is unset — reconciled by D-09 and landed (chunk 2026-10-17-mcp-server-for-drivers): `mcp` always writes `mcp.ndjson`, stderr JSON only when that file cannot be opened;
### obs-scope-summary#3.2 · values · scope-entities · ## 1. Obs Scope Summary · lines 27–27
- **Instrumentability:** instrumentable
### obs-scope-summary#3.3 · values · scope-entities · ## 1. Obs Scope Summary · lines 28–28
- **Reason if not fully instrumentable:** N/A. This is the only crate that uses anyhow, so it is the error-capture boundary. Arch Stack says "context chains only at dispatch". Security Vector 6 says a chain that contains a serde source must become a fixed message before it reaches stderr. The full chain goes to `diagnostics/` only.
### obs-scope-summary#6.1 · values · scope-entities · ## 1. Obs Scope Summary · lines 50–50
- **Source:** arch excerpt, Stack (interprocess 2.4.4), Workspace, Standard Contracts (IPC methods `send · wait · last · answer · pause · release · link · unlink · hook.dialog · hook.event`)
### obs-scope-summary#7.2 · values · scope-entities · ## 1. Obs Scope Summary · lines 59–59
- **Instrumentability:** instrumentable
### obs-scope-summary#7.3 · values · scope-entities · ## 1. Obs Scope Summary · lines 60–60
- **Reason if not fully instrumentable:** N/A. Stack bans Tokio here, so instrumentation must be sync.
### obs-scope-summary#9.2 · values · scope-entities · ## 1. Obs Scope Summary · lines 77–77
- **Instrumentability:** instrumentable
### obs-scope-summary#10.2 · values · scope-entities · ## 1. Obs Scope Summary · lines 85–85
- **Instrumentability:** boundary-only
### obs-scope-summary#10.3 · values · scope-entities · ## 1. Obs Scope Summary · lines 86–91
- **Reason if not fully instrumentable:** It is a third-party binary run unmodified. Instrument only the boundary:
  - spawn and exit (`process-start` / `process-exit`), with exit detected on the process handle, never on EOF (Stack; brief §4.1);
  - paste writes;
  - the vt100 readiness-gate outcome.

  Its internal state and transcripts are never read as a contract (creator Obs Anti-Patterns, brief §4).
### obs-scope-summary#11.2 · values · scope-entities · ## 1. Obs Scope Summary · lines 98–98
- **Instrumentability:** boundary-only
### obs-scope-summary#11.3 · values · scope-entities · ## 1. Obs Scope Summary · lines 99–99
- **Reason if not fully instrumentable:** The caller is Claude Code, so viola sees only the inbound stdin payload and its own response. Instrument `hook-invoked` / `hook-decision` and MCP tool-call handling. No trace context arrives from Claude Code.
### obs-scope-summary#13.3 · values · scope-entities · ## 1. Obs Scope Summary · lines 115–115
- **Reason if not fully instrumentable:** A user-written shell command. Instrument the shell-out duration and exit status only. The command string is never logged outside `diagnostics/` (security Data Classifications, `statusline_command`).
### obs-scope-summary#14.2 · values · scope-entities · ## 1. Obs Scope Summary · lines 122–122
- **Instrumentability:** boundary-only
### obs-scope-summary#14.3 · values · scope-entities · ## 1. Obs Scope Summary · lines 123–123
- **Reason if not fully instrumentable:** Instrument viola's call sites (bind, connect, lock, rename, kill-fallback use, and the server check's peer-pid and start-time reads, whose failed check the CLI and the hook log as `server-verify-failed` and MCP logs by no line), not the OS internals.
### obs-scope-summary#15.2 · values · scope-entities · ## 1. Obs Scope Summary · lines 130–130
- **Instrumentability:** instrumentable (through the `viola-state` call sites)
### obs-scope-summary#15.3 · values · scope-entities · ## 1. Obs Scope Summary · lines 131–134
- **Reason if not fully instrumentable:** N/A. Scrub rules per store (security Data Classifications):
  - `events.ndjson` is itself the product audit stream (brief §7, "the event log as the audit trail"). Its payload content is never copied into process-log lines.
  - Absolute paths in `snapshot.json` (`endpoint`, `pinned_bin`) and in config values are scrubbed from external output. They are allowed in `diagnostics/`.
  - Operational metadata in `budget.json` (percentages, `resets_at`) may be logged as structured fields.
### obs-scope-summary#18.1 · values · scope-surfaces · ## 1. Obs Scope Summary · lines 156–156
- **Backend instrumentation hooks:** Rust `tracing` facade with `#[tracing::instrument]` spans on the dispatch functions, plus a tracing-subscriber JSON structured logger (tests excerpt, Log format: `tracing-subscriber 0.3.23 fmt().json().flatten_event(true).with_current_span(false)`). Each process gets its own panic hook: a `std::panic::set_hook` replacement that writes one line. No HTTP framework is involved, so there is no auto-instrumentation.
### obs-scope-summary#18.2 · values · scope-surfaces · ## 1. Obs Scope Summary · lines 157–157
- **Frontend instrumentation hooks:** N/A. The only output is terminal stdout/stderr (design-system.md cli surface: static fixed-column text, with no interactive prompt, TUI or pager).
### obs-scope-summary#18.3 · values · scope-surfaces · ## 1. Obs Scope Summary · lines 158–158
- **Exporter:** A file sink only: JSON-per-line to `diagnostics/` when an instance is resolved (security Vector 6: "write the full error only to `diagnostics/` when an instance is resolved"). Without an instance there is no process-log file, and human stderr carries only fixed-message lines (design excerpt, CLI faults). Network OTLP is excluded: security says "viola makes no outbound network calls". The stdout exporter is excluded because stdout carries `--json` results. `viola revive <name> --list` logs as `cli` into the caller's `cli-<VIOLA_NAME>.ndjson` once its strict-modes check has passed (none with no `VIOLA_NAME`; a refused check opens none; chunk 2026-10-17-cli-machine-contract). `viola link` / `viola unlink` log as `cli` into the driven instance's `cli-<driven>.ndjson` (the instance their `<driven>` argument names, never the caller's `VIOLA_NAME`) once their instance check of it has passed; a refused check opens none. `viola revive`'s start arm is not a short-lived verb: it is the wrapper surface below (`process="run"`, `diagnostics/run-<name>.ndjson`).
### obs-scope-summary#18.4 · values · scope-surfaces · ## 1. Obs Scope Summary · lines 159–159
- **Service identity:** `service.name` is hardcoded `"viola"`, the binary name. `service.version` is compile-time `env!("CARGO_PKG_VERSION")`, the same value already used as `sender` and `writer`. The process role is the `process` field. The instance is `ViolaName` from an explicit argument or `VIOLA_NAME`. `deployment.environment` is N/A: local-only, with no environments (arch Obs-Relevant Conventions, Service identity; arch Surfaces, "no hosting").
### obs-scope-summary#18.5 · values · scope-surfaces · ## 1. Obs Scope Summary · lines 160–164
- **Notes:**
  - The instance comes from `VIOLA_NAME` or an argument, never from the working directory (arch Instance identity rule).
  - Refusal and fault lines on stderr are fixed strings with no paths, pids, chains or upstream text (design excerpt, CLI refusals / CLI faults).
  - Exit codes 0, 1, 2, 10–14, 20 and 21 are the machine-readable outcome and should be a log field (arch Standard Contracts, CLI `--json` + exit codes).
  - Founder Direction 4: each cause behind exit 21 and exit 1 gets a distinct detail code.
### obs-scope-summary#19.1 · values · scope-surfaces · ## 1. Obs Scope Summary · lines 170–170
- **Backend instrumentation hooks:** `tracing` spans (`#[tracing::instrument]`) on the start sequence, the `viola-pty` seam operations, the `viola-channel` sync server handlers, vt100 readiness-gate outcomes and `viola-state` writes, all written through the tracing-subscriber JSON logger. Instrumentation must be sync: std threads, no Tokio (arch Stack, Hybrid concurrency; `cargo deny` ban).
