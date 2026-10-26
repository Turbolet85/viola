# obs-plan · Telemetry Strategy
Read whole. The master's broad view is .andromeda/obs-plan.md; this file holds its Telemetry Strategy detail.

## 2. Telemetry Strategy

<!-- detail: values · strategy -->

| Signal | Coverage focus | Tier-specific depth (viola v1) |
|--------|---------------|---------------------|
| Spans / Traces | The 6 must-trace paths (Section 4) and every crate seam: `viola-pty` spawn/write/wait/kill, `viola-channel` client call and server dispatch, `viola-state` snapshot/heartbeat/tail, `viola-agent-claude` hook parse, `viola-mcp` tool calls | Declared in code with tracing 0.1.44 `#[instrument(skip_all, name = "<area>.<operation>", fields(...))]`. They are **not serialized** in v1, because the output sets `with_current_span(false)` + `with_span_list(false)`. They give the code its structure and are ready for the deferred tracing-opentelemetry 0.34.0 bridge. Cross-process joins use `corr` + `conn`, not W3C trace IDs |
| Structured Logs | Every boundary call, state transition, refusal, fault/heal and process lifecycle, as the closed `event` enum (Section 3) | **The primary signal.** tracing-subscriber 0.3.23 JSON-per-line, one file per process role under `<home>/diagnostics/`, merged with `events.ndjson` by `agent-run logs` |
| Errors | Panics (a one-line `event:"panic"` record), typed thiserror 2.0.20 errors with fixed `Display`, exit codes plus distinct `detail` codes on `process-exit` | No external reporter: security says "no outbound network calls". A hand-written `std::panic::set_hook`, plus `std::panic::catch_unwind` at the main-thread catch site of every role, around `hook` dispatch, in the `run` PTY pump and handle-wait threads, and around the vt100 feed (Section 7) |
| Metrics | `duration_ms` samples, counts of `event`/`detail` values, `skipped` counts | No in-process instruments and no exporter. Metrics are derived by `jq` over the ndjson; the perf gates run on hyperfine 1.20.0 `--export-json` (Section 5) |
| Audit Logs | `instances/<name>/events.ndjson` is the product audit stream (arch-owned, never rotated) | Not a compliance trail (security Minimal, no compliance triggers). Obs never copies event payloads into process logs |

<!-- /detail -->

<!-- detail: values · strategy -->

- Every panic after init step 4 produces exactly one single-line structured record in its home-level role file: location, thread, role and instance. The bounded exemptions (the pre-init window, `cli` without an instance, `hook` without `VIOLA_NAME`, a failed step-4 role-file open, the no-obs-init hidden capture arms (`hook <event> --capture` and `hook statusline --capture`), and a refused instance check on either `viola revive` arm (start or `--list`) or on `viola hook statusline`, or a refused `viola list` scan, which opens none either, or a refused instance check on `viola link` / `viola unlink`, made before their role file opens) are listed in Section 10. The payload (one escaped JSON string) and the backtrace (a one-line JSON array of frames) are written only to the instance-scoped detail file (Section 7).

<!-- /detail -->

<!-- detail: values · triggers -->

| Trigger | Covered in |
|---------|-----------|
| logging-sensitive Vector 1 (IPC) | §4 surface table (channel), §6 channel field allow-list, §8 table |
| logging-sensitive Vector 2 (paste text) | §4 Scenario 2 (`send-refused` `side`), §8 |
| logging-sensitive Vector 4 (hook stdin) | §3 Logging stack (hook sink), §4 Scenario 4, §6 `hook-decision` `detail` codes. The "missing `VIOLA_NAME`" item is **overridden**: no file and no line, asserted by absence (D-16, §4 E1). The "strict-modes failure" item writes no line as landed either: `viola hook statusline`'s refused instance check leaves no trace, asserted by absence (chunk 2026-10-10-budget-governor). The "server-verify mismatch" item writes one `hook-decision{detail:"server-verify-failed"}` at WARN on Linux and Windows (macOS compares no peer pid, the recorded gap) |
| logging-sensitive Vector 5 (MCP) | §4 surface table (mcp), §6 |
| logging-sensitive Vector 6 (CLI/env) | §4 Scenario 1 (`env_stripped_*`), §7 fixed messages, §8 |
| logging-sensitive Vector 7 (filesystem) | §3 Log file location (0600/0700), §6 `state-recovered` / `skipped`, §6 detail catalog Filesystem refusals (`strict-modes-failed`, `pinned-hash-mismatch`, `symlink-ignored` via `parse-rejected{parser:"state-entry"}`), §9 secret-scan-before-upload |
| logging-sensitive Vector 8 (child spawn) | §4 Scenario 1 (`subject` values), §6 `parse-rejected` |
| logging-sensitive Vector 9 (supply chain) | §3 Logging stack (feature set), §9 cargo-deny gates, §11 |
| cross-surface-trace-propagation | §3 Trace context propagation (`corr` + `conn`), §4 Scenarios 2/4 |
| perf-budget-instruments | §5, §10 budget table |
| chaos-instrumentation | §6 `state-recovered` / `process-exit` `exit_source` + `during`, §10 |
| multi-platform-exporter-compat | §3 Log file location (0600/0700 vs home DACL), §3 Logging stack (concurrent-append check on windows-2025 / macos-latest / ubuntu-latest), §4 Scenario 1 (`endpoint_kind`, `pty_backend`), §9 per-OS artifacts |
| error-budget-SLO | §7, §10 zero-unlogged-panics |
| audit-log-retention | §3 Log file location (no rotation), §10 |
| creator-explicit (Founder Directions 1/2/5) | §3 schema verbatim, §6 |
| creator-explicit (Founder Direction 4) | §6 detail code catalog, §12 D-01 |
| creator-explicit (installed-CLI measured) | §5 counters, §6 `parse-rejected` / `skipped` |
| creator-explicit (no screen scraping) | §4 Scenario 2 (vt100 gate outcome only), §11 |

<!-- /detail -->
