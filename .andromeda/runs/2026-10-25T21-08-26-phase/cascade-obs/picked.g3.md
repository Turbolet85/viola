### telemetry-strategy#1.1 · values · strategy · ## 2. Telemetry Strategy · lines 8–10
| Signal | Coverage focus | Tier-specific depth (viola v1) |
|--------|---------------|---------------------|
| Spans / Traces | The 6 must-trace paths (Section 4) and every crate seam: `viola-pty` spawn/write/wait/kill, `viola-channel` client call and server dispatch, `viola-state` snapshot/heartbeat/tail, `viola-agent-claude` hook parse, `viola-mcp` tool calls | Declared in code with tracing 0.1.44 `#[instrument(skip_all, name = "<area>.<operation>", fields(...))]`. They are **not serialized** in v1, because the output sets `with_current_span(false)` + `with_span_list(false)`. They give the code its structure and are ready for the deferred tracing-opentelemetry 0.34.0 bridge. Cross-process joins use `corr` + `conn`, not W3C trace IDs |
### telemetry-strategy#1.2 · values · strategy · ## 2. Telemetry Strategy · lines 11–11
| Structured Logs | Every boundary call, state transition, refusal, fault/heal and process lifecycle, as the closed `event` enum (Section 3) | **The primary signal.** tracing-subscriber 0.3.23 JSON-per-line, one file per process role under `<home>/diagnostics/`, merged with `events.ndjson` by `agent-run logs` |
### telemetry-strategy#1.3 · values · strategy · ## 2. Telemetry Strategy · lines 12–12
| Errors | Panics (a one-line `event:"panic"` record), typed thiserror 2.0.20 errors with fixed `Display`, exit codes plus distinct `detail` codes on `process-exit` | No external reporter: security says "no outbound network calls". A hand-written `std::panic::set_hook`, plus `std::panic::catch_unwind` at the main-thread catch site of every role, around `hook` dispatch, in the `run` PTY pump and handle-wait threads, and around the vt100 feed (Section 7) |
### telemetry-strategy#1.4 · values · strategy · ## 2. Telemetry Strategy · lines 13–13
| Metrics | `duration_ms` samples, counts of `event`/`detail` values, `skipped` counts | No in-process instruments and no exporter. Metrics are derived by `jq` over the ndjson; the perf gates run on hyperfine 1.20.0 `--export-json` (Section 5) |
### telemetry-strategy#1.5 · values · strategy · ## 2. Telemetry Strategy · lines 14–14
| Audit Logs | `instances/<name>/events.ndjson` is the product audit stream (arch-owned, never rotated) | Not a compliance trail (security Minimal, no compliance triggers). Obs never copies event payloads into process logs |
### telemetry-strategy#2.1 · values · strategy · ## 2. Telemetry Strategy · lines 20–20
- Every panic after init step 4 produces exactly one single-line structured record in its home-level role file: location, thread, role and instance. The bounded exemptions (the pre-init window, `cli` without an instance, `hook` without `VIOLA_NAME`, a failed step-4 role-file open, the no-obs-init hidden capture arms (`hook <event> --capture` and `hook statusline --capture`), and a refused instance check on either `viola revive` arm (start or `--list`) or on `viola hook statusline`, or a refused `viola list` scan, which opens none either) are listed in Section 10. The payload (one escaped JSON string) and the backtrace (a one-line JSON array of frames) are written only to the instance-scoped detail file (Section 7).
### telemetry-strategy#3.1 · values · triggers · ## 2. Telemetry Strategy · lines 26–28
| Trigger | Covered in |
|---------|-----------|
| logging-sensitive Vector 1 (IPC) | §4 surface table (channel), §6 channel field allow-list, §8 table |
### telemetry-strategy#3.2 · values · triggers · ## 2. Telemetry Strategy · lines 29–29
| logging-sensitive Vector 2 (paste text) | §4 Scenario 2 (`send-refused` `side`), §8 |
### telemetry-strategy#3.5 · values · triggers · ## 2. Telemetry Strategy · lines 32–32
| logging-sensitive Vector 6 (CLI/env) | §4 Scenario 1 (`env_stripped_*`), §7 fixed messages, §8 |
### telemetry-strategy#3.6 · values · triggers · ## 2. Telemetry Strategy · lines 33–33
| logging-sensitive Vector 7 (filesystem) | §3 Log file location (0600/0700), §6 `state-recovered` / `skipped`, §6 detail catalog Filesystem refusals (`strict-modes-failed`, `pinned-hash-mismatch`, `symlink-ignored` via `parse-rejected{parser:"state-entry"}`), §9 secret-scan-before-upload |
### telemetry-strategy#3.7 · values · triggers · ## 2. Telemetry Strategy · lines 34–34
| logging-sensitive Vector 8 (child spawn) | §4 Scenario 1 (`subject` values), §6 `parse-rejected` |
### telemetry-strategy#3.8 · values · triggers · ## 2. Telemetry Strategy · lines 35–35
| logging-sensitive Vector 9 (supply chain) | §3 Logging stack (feature set), §9 cargo-deny gates, §11 |
### telemetry-strategy#3.11 · values · triggers · ## 2. Telemetry Strategy · lines 38–38
| chaos-instrumentation | §6 `state-recovered` / `process-exit` `exit_source` + `during`, §10 |
### telemetry-strategy#3.12 · values · triggers · ## 2. Telemetry Strategy · lines 39–39
| multi-platform-exporter-compat | §3 Log file location (0600/0700 vs home DACL), §3 Logging stack (concurrent-append check on windows-2025 / macos-latest / ubuntu-latest), §4 Scenario 1 (`endpoint_kind`, `pty_backend`), §9 per-OS artifacts |
### telemetry-strategy#3.13 · values · triggers · ## 2. Telemetry Strategy · lines 40–40
| error-budget-SLO | §7, §10 zero-unlogged-panics |
### telemetry-strategy#3.15 · values · triggers · ## 2. Telemetry Strategy · lines 42–42
| creator-explicit (Founder Directions 1/2/5) | §3 schema verbatim, §6 |
### telemetry-strategy#3.16 · values · triggers · ## 2. Telemetry Strategy · lines 43–43
| creator-explicit (Founder Direction 4) | §6 detail code catalog, §12 D-01 |
