### obs-scope-summary#3.2 · values · scope-entities · ## 1. Obs Scope Summary · lines 27–27
- **Instrumentability:** instrumentable
### obs-scope-summary#3.3 · values · scope-entities · ## 1. Obs Scope Summary · lines 28–28
- **Reason if not fully instrumentable:** N/A. This is the only crate that uses anyhow, so it is the error-capture boundary. Arch Stack says "context chains only at dispatch". Security Vector 6 says a chain that contains a serde source must become a fixed message before it reaches stderr. The full chain goes to `diagnostics/` only.
### obs-scope-summary#4.3 · values · scope-entities · ## 1. Obs Scope Summary · lines 36–36
- **Reason if not fully instrumentable:** It is a pure type crate with no I/O and no runtime behaviour of its own. It supplies the kebab-case values used as log fields (`refusal`, `detail`, event kinds) but emits nothing. Excluded from sections 2 to 4.
### obs-scope-summary#6.2 · values · scope-entities · ## 1. Obs Scope Summary · lines 51–51
- **Instrumentability:** instrumentable
### obs-scope-summary#6.3 · values · scope-entities · ## 1. Obs Scope Summary · lines 52–52
- **Reason if not fully instrumentable:** N/A. Stack forbids Tokio in this crate's graph unless its feature is enabled, so instrumentation must be sync.
### obs-scope-summary#8.3 · values · scope-entities · ## 1. Obs Scope Summary · lines 68–70
- **Reason if not fully instrumentable:** N/A. Its outputs are the most sensitive in the product:
  - Drift reports go to `diagnostics/` only (security Vector 4).
  - R8-stripped `CLAUDE*` values are never logged anywhere (security Vector 6).
### obs-scope-summary#9.3 · values · scope-entities · ## 1. Obs Scope Summary · lines 78–78
- **Reason if not fully instrumentable:** N/A. stdout is reserved for MCP frames (arch Stack and Obs-Relevant Conventions), so no telemetry may go to stdout.
### obs-scope-summary#14.2 · values · scope-entities · ## 1. Obs Scope Summary · lines 122–122
- **Instrumentability:** boundary-only
### obs-scope-summary#14.3 · values · scope-entities · ## 1. Obs Scope Summary · lines 123–123
- **Reason if not fully instrumentable:** Instrument viola's call sites (bind, connect, lock, rename, kill-fallback use), not the OS internals.
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
- **Exporter:** A file sink only: JSON-per-line to `diagnostics/` when an instance is resolved (security Vector 6: "write the full error only to `diagnostics/` when an instance is resolved"). Without an instance there is no process-log file, and human stderr carries only fixed-message lines (design excerpt, CLI faults). Network OTLP is excluded: security says "viola makes no outbound network calls". The stdout exporter is excluded because stdout carries `--json` results. `viola revive <name> --list` logs as `cli` into the caller's `cli-<VIOLA_NAME>.ndjson` once its strict-modes check has passed (none with no `VIOLA_NAME`; a refused check opens none; chunk 2026-10-17-cli-machine-contract). `viola revive`'s start arm is not a short-lived verb: it is the wrapper surface below (`process="run"`, `diagnostics/run-<name>.ndjson`).
### obs-scope-summary#18.5 · values · scope-surfaces · ## 1. Obs Scope Summary · lines 160–164
- **Notes:**
  - The instance comes from `VIOLA_NAME` or an argument, never from the working directory (arch Instance identity rule).
  - Refusal and fault lines on stderr are fixed strings with no paths, pids, chains or upstream text (design excerpt, CLI refusals / CLI faults).
  - Exit codes 0, 1, 2, 10–14, 20 and 21 are the machine-readable outcome and should be a log field (arch Standard Contracts, CLI `--json` + exit codes).
  - Founder Direction 4: each cause behind exit 21 and exit 1 gets a distinct detail code.
### obs-scope-summary#19.1 · values · scope-surfaces · ## 1. Obs Scope Summary · lines 170–170
- **Backend instrumentation hooks:** `tracing` spans (`#[tracing::instrument]`) on the start sequence, the `viola-pty` seam operations, the `viola-channel` sync server handlers, vt100 readiness-gate outcomes and `viola-state` writes, all written through the tracing-subscriber JSON logger. Instrumentation must be sync: std threads, no Tokio (arch Stack, Hybrid concurrency; `cargo deny` ban).
### obs-scope-summary#19.3 · values · scope-surfaces · ## 1. Obs Scope Summary · lines 172–172
- **Exporter:** A file sink only, to `diagnostics/run-<name>.ndjson` (tests excerpt, Log format). The terminal is forbidden: "`run` writes nothing to the terminal while the child runs" (arch Observability Hints).
### obs-scope-summary#19.4 · values · scope-surfaces · ## 1. Obs Scope Summary · lines 173–173
- **Service identity:** as for the short-lived verbs, with `process="run"` and `instance=<ViolaName>`.
### obs-scope-summary#19.5 · values · scope-surfaces · ## 1. Obs Scope Summary · lines 174–177
- **Notes:**
  - Rust's default panic hook prints to stderr, which is the child's terminal. It must be replaced before the child spawns.
  - The heartbeat file, touched every 1 s, is the existing liveness tick (arch Standard Contracts, Heartbeat).
  - Child exit is detected on the process handle, so `process-exit` for the child must come from the handle wait, not from reader EOF (arch Stack; brief §4.1 Windows exit detection).
### obs-scope-summary#21.1 · values · scope-surfaces · ## 1. Obs Scope Summary · lines 196–196
- **Backend instrumentation hooks:** language-native `tracing` spans on client call and server dispatch. Propagation uses the `corr` field in log lines (JSON-RPC `id` for `channel-request` / `channel-response`; `dialog_id`; send `cursor`), per the tests excerpt Log format. Channel `params` declare no trace-context field (arch Standard Contracts, `send`: "No trace-context field is declared"). Readers skip unknown fields (arch Mixed-version tolerance), so an additive envelope field would be wire-compatible, but none is required by upstream.
### obs-scope-summary#21.3 · values · scope-surfaces · ## 1. Obs Scope Summary · lines 198–198
- **Exporter:** The log file of whichever process is on each side (`run-<name>`, `hook-<name>`, `mcp`, or none for short-lived CLI verbs without an instance).
### obs-scope-summary#21.4 · values · scope-surfaces · ## 1. Obs Scope Summary · lines 199–199
- **Service identity:** Each side logs its own `process`. The peer version is available as `params.sender` (`CARGO_PKG_VERSION`).
### obs-scope-summary#21.5 · values · scope-surfaces · ## 1. Obs Scope Summary · lines 200–203
- **Notes:**
  - JSON-RPC ids are "integers, monotonic per connection" (arch Standard Contracts, JSON-RPC frame shapes). `corr` for `channel-request` / `channel-response` is therefore not unique across connections or processes, and a merged-log join needs `process`, `instance` and a connection discriminator. This is a gap for Phase 3; the tests contract allows additive fields.
  - `-32603` must be `"internal error"` with `data:null` (security Vector 1).
  - `from` is self-reported and must not be logged as an authenticated identity (security Vector 1).
