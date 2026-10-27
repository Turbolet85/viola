### obs-scope-summary#17.1 · values · scope-entities · ## 1. Obs Scope Summary · lines 148–148
- **Source:** arch excerpt, CI/CD Platform
### obs-scope-summary#17.2 · values · scope-entities · ## 1. Obs Scope Summary · lines 149–149
- **Instrumentability:** not-instrumentable
### obs-scope-summary#17.3 · values · scope-entities · ## 1. Obs Scope Summary · lines 150–150
- **Reason if not fully instrumentable:** Not product runtime. Fixtures are scrub-required data (security Data Classifications, recorded fixtures), not a telemetry source. Excluded from sections 2 to 4.
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
### obs-scope-summary#20.2 · values · scope-surfaces · ## 1. Obs Scope Summary · lines 184–184
- **Frontend instrumentation hooks:** N/A.
### obs-scope-summary#20.3 · values · scope-surfaces · ## 1. Obs Scope Summary · lines 185–185
- **Exporter:** A file sink only, to `diagnostics/hook-<name>.ndjson`. stderr is forbidden on both. stdout carries only the decision body for `viola hook <event>`, and for `viola hook statusline` only the user's status-line command's output (arch Standard Contracts, Hook contract); the two hidden `--capture` arms (`verify`'s probe capture) write no sink and print nothing, except `--answers`' probe decision body. A failed log write is swallowed, and the hook still exits 0 (tests excerpt, Log format Constraints).
### obs-scope-summary#20.4 · values · scope-surfaces · ## 1. Obs Scope Summary · lines 186–186
- **Service identity:** as for the short-lived verbs, with `process="hook"`. If `VIOLA_NAME` is missing, `instance` is null and the hook is a silent no-op (tests excerpt, E1 R5 unwrapped session hooks; Coverage Triggers, Vector 4 "missing `VIOLA_NAME`").
### obs-scope-summary#20.5 · values · scope-surfaces · ## 1. Obs Scope Summary · lines 187–190
- **Notes:**
  - Many hook processes can append to the same `hook-<name>.ndjson` at once. Each line must be one `write` call on an append-mode handle (arch Stack, atomic-write-file / `OpenOptions::append`).
  - The latency budget is `max < 1.0 s` per hook (tests excerpt, Quality Gates). Logger init must not add blocking I/O beyond one file open and append. `viola hook statusline` is outside that budget: its shell-out of the user's command is bounded by the 5 s PROVISIONAL `STATUSLINE_DEADLINE`, and no `run --perf` row times it.
  - Where no instance directory resolves, file logging is impossible. That case gets no log and no stderr, by contract.
### obs-scope-summary#21.1 · values · scope-surfaces · ## 1. Obs Scope Summary · lines 196–196
- **Backend instrumentation hooks:** language-native `tracing` spans on client call and server dispatch. Propagation uses the `corr` field in log lines (JSON-RPC `id` for `channel-request` / `channel-response`; `dialog_id`; send `cursor`), per the tests excerpt Log format. Channel `params` declare no trace-context field (arch Standard Contracts, `send`: "No trace-context field is declared"). Readers skip unknown fields (arch Mixed-version tolerance), so an additive envelope field would be wire-compatible, but none is required by upstream.
### obs-scope-summary#21.2 · values · scope-surfaces · ## 1. Obs Scope Summary · lines 197–197
- **Frontend instrumentation hooks:** N/A.
### obs-scope-summary#21.3 · values · scope-surfaces · ## 1. Obs Scope Summary · lines 198–198
- **Exporter:** The log file of whichever process is on each side (`run-<name>`, `hook-<name>`, `mcp`, or none for short-lived CLI verbs without an instance).
### obs-scope-summary#21.4 · values · scope-surfaces · ## 1. Obs Scope Summary · lines 199–199
- **Service identity:** Each side logs its own `process`. The peer version is available as `params.sender` (`CARGO_PKG_VERSION`).
### obs-scope-summary#21.5 · values · scope-surfaces · ## 1. Obs Scope Summary · lines 200–203
- **Notes:**
  - JSON-RPC ids are "integers, monotonic per connection" (arch Standard Contracts, JSON-RPC frame shapes). `corr` for `channel-request` / `channel-response` is therefore not unique across connections or processes, and a merged-log join needs `process`, `instance` and a connection discriminator. This is a gap for Phase 3; the tests contract allows additive fields.
  - `-32603` must be `"internal error"` with `data:null` (security Vector 1).
  - `from` is self-reported and must not be logged as an authenticated identity (security Vector 1).
### obs-scope-summary#22.1 · values · scope-surfaces · ## 1. Obs Scope Summary · lines 209–209
- **Backend instrumentation hooks:** `tracing` spans per MCP tool call (`send · wait · last · answer · list`) wrapping the `viola-channel` Tokio client, through the tracing-subscriber JSON logger. stdio has no HTTP-framework auto-instrumentation.
### obs-scope-summary#22.2 · values · scope-surfaces · ## 1. Obs Scope Summary · lines 210–210
- **Frontend instrumentation hooks:** N/A.
