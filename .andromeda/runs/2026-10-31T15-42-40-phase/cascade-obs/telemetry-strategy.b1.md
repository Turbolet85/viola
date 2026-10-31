### telemetry-strategy#1.1 · values · strategy · ## 2. Telemetry Strategy · lines 8–10
| Signal | Coverage focus | Tier-specific depth (viola v1) |
|--------|---------------|---------------------|
| Spans / Traces | The 6 must-trace paths (Section 4) and every crate seam: `viola-pty` spawn/write/wait/kill, `viola-channel` client call and server dispatch, `viola-state` snapshot/heartbeat/tail, `viola-agent-claude` hook parse, `viola-mcp` tool calls | Declared in code with tracing 0.1.44 `#[instrument(skip_all, name = "<area>.<operation>", fields(...))]`. They are **not serialized** in v1, because the output sets `with_current_span(false)` + `with_span_list(false)`. They give the code its structure and are ready for the deferred tracing-opentelemetry 0.34.0 bridge. Cross-process joins use `corr` + `conn`, not W3C trace IDs |
### telemetry-strategy#1.2 · values · strategy · ## 2. Telemetry Strategy · lines 11–11
| Structured Logs | Every boundary call, state transition, refusal, fault/heal and process lifecycle (the `mcp` role writes no self `process-exit`, Section 7), as the closed `event` enum (Section 3) | **The primary signal.** tracing-subscriber 0.3.23 JSON-per-line, one file per process role under `<home>/diagnostics/`, merged with `events.ndjson` by `agent-run logs` |
### telemetry-strategy#1.3 · values · strategy · ## 2. Telemetry Strategy · lines 12–12
| Errors | Panics (a one-line `event:"panic"` record), typed thiserror 2.0.20 errors with fixed `Display`, exit codes plus distinct `detail` codes on `process-exit` (the `mcp` role writes no self `process-exit`, Section 7) | No external reporter: security says "no outbound network calls". A hand-written `std::panic::set_hook`, plus `std::panic::catch_unwind` at the main-thread catch site of every role, around `hook` dispatch, in the `run` PTY pump and handle-wait threads, and around the vt100 feed (Section 7) |
### telemetry-strategy#1.4 · values · strategy · ## 2. Telemetry Strategy · lines 13–13
| Metrics | `duration_ms` samples, counts of `event`/`detail` values, `skipped` counts | No in-process instruments and no exporter. Metrics are derived by `jq` over the ndjson; the perf gates run on hyperfine 1.20.0 `--export-json` (Section 5) |
### telemetry-strategy#1.5 · values · strategy · ## 2. Telemetry Strategy · lines 14–14
| Audit Logs | `instances/<name>/events.ndjson` is the product audit stream (arch-owned, never rotated) | Not a compliance trail (security Minimal, no compliance triggers). Obs never copies event payloads into process logs |
### telemetry-strategy#2.1 · values · strategy · ## 2. Telemetry Strategy · lines 20–20
- Every panic after init step 4 produces exactly one single-line structured record in its home-level role file: location, thread, role and instance. The bounded exemptions (the pre-init window, `cli` without an instance, `hook` without `VIOLA_NAME`, a failed step-4 role-file open (a `diagnostics/` folder or file its strict-modes check refuses included), the no-obs-init hidden capture arms (`hook <event> --capture` and `hook statusline --capture`), and a refused instance check on either `viola revive` arm (start or `--list`) or on `viola hook statusline`, or a refused `viola list` scan, which opens none either, or a refused instance check on `viola link` / `viola unlink`, made before their role file opens, or a refused instance check on the six frame verbs (`send`, `wait`, `last`, `answer`, `pause`, `release`), a refused or absent instance on `viola hook`'s event and dialog paths (a `VIOLA_DIR` naming no existing instance directory included), or a refused home tree on `viola run`, `viola revive`'s start or `viola verify`, each made before its role file opens) are listed in Section 10. The payload (one escaped JSON string) and the backtrace (a one-line JSON array of frames) are written only to the instance-scoped detail file (Section 7).
### telemetry-strategy#3.1 · values · triggers · ## 2. Telemetry Strategy · lines 26–28
| Trigger | Covered in |
|---------|-----------|
| logging-sensitive Vector 1 (IPC) | §4 surface table (channel), §6 channel field allow-list, §8 table |
### telemetry-strategy#3.2 · values · triggers · ## 2. Telemetry Strategy · lines 29–29
| logging-sensitive Vector 2 (paste text) | §4 Scenario 2 (`send-refused` `side`), §8 |
### telemetry-strategy#3.3 · values · triggers · ## 2. Telemetry Strategy · lines 30–30
| logging-sensitive Vector 4 (hook stdin) | §3 Logging stack (hook sink), §4 Scenario 4, §6 `hook-decision` `detail` codes. The "missing `VIOLA_NAME`" item is **overridden**: no file and no line, asserted by absence (D-16, §4 E1). The "strict-modes failure" item writes no line as landed either: `viola hook statusline`'s refused instance check leaves no trace, asserted by absence (chunk 2026-10-10-budget-governor). The event and dialog paths' refused or absent instance (a `VIOLA_DIR` naming no existing instance directory included) leaves none either: exit 0 at once, no `config.json` read, no role file, no stdin read, no line, no frame, no SessionEnd append. The "server-verify mismatch" item writes one `hook-decision{detail:"server-verify-failed"}` at WARN on Linux and Windows (macOS compares no peer pid, the recorded gap) |
### telemetry-strategy#3.4 · values · triggers · ## 2. Telemetry Strategy · lines 31–31
| logging-sensitive Vector 5 (MCP) | §4 surface table (mcp), §6 |
### telemetry-strategy#3.5 · values · triggers · ## 2. Telemetry Strategy · lines 32–32
| logging-sensitive Vector 6 (CLI/env) | §4 Scenario 1 (`env_stripped_*`), §7 fixed messages, §8 |
### telemetry-strategy#3.6 · values · triggers · ## 2. Telemetry Strategy · lines 33–33
| logging-sensitive Vector 7 (filesystem) | §3 Log file location (0600/0700), §6 `state-recovered` / `skipped`, §6 detail catalog Filesystem refusals (`strict-modes-failed`, `pinned-hash-mismatch`, `symlink-ignored` via `parse-rejected{parser:"state-entry"}`), §9 secret-scan-before-upload, §4 Scenario 1 / §6 `process-start.sideload_fallback` `strict-modes-failed` (a `conpty/` companion its owner/DACL check refuses: the child on the inbox ConPTY, exit unchanged), §10 exemptions (a strict-modes check made before a read or open — the six frame verbs, `viola hook`'s event and dialog paths, `viola mcp`'s frame tools, `verify`, `run` / `revive`'s start, the `diagnostics/` opens — writes no line) |
### telemetry-strategy#3.7 · values · triggers · ## 2. Telemetry Strategy · lines 34–34
| logging-sensitive Vector 8 (child spawn) | §4 Scenario 1 (`subject` values), §6 `parse-rejected` |
### telemetry-strategy#3.8 · values · triggers · ## 2. Telemetry Strategy · lines 35–35
| logging-sensitive Vector 9 (supply chain) | §3 Logging stack (feature set), §9 cargo-deny gates, §11 |
### telemetry-strategy#3.9 · values · triggers · ## 2. Telemetry Strategy · lines 36–36
| cross-surface-trace-propagation | §3 Trace context propagation (`corr` + `conn`), §4 Scenarios 2/4 |
### telemetry-strategy#3.10 · values · triggers · ## 2. Telemetry Strategy · lines 37–37
| perf-budget-instruments | §5, §10 budget table |
### telemetry-strategy#3.11 · values · triggers · ## 2. Telemetry Strategy · lines 38–38
| chaos-instrumentation | §6 `state-recovered` / `process-exit` `exit_source` + `during`, §10 |
### telemetry-strategy#3.12 · values · triggers · ## 2. Telemetry Strategy · lines 39–39
| multi-platform-exporter-compat | §3 Log file location (0600/0700 vs home DACL), §3 Logging stack (concurrent-append check on windows-2025 / macos-latest / ubuntu-latest), §4 Scenario 1 (`endpoint_kind`, `pty_backend`), §9 per-OS artifacts |
### telemetry-strategy#3.13 · values · triggers · ## 2. Telemetry Strategy · lines 40–40
| error-budget-SLO | §7, §10 zero-unlogged-panics |
### telemetry-strategy#3.14 · values · triggers · ## 2. Telemetry Strategy · lines 41–41
| audit-log-retention | §3 Log file location (no rotation), §10 |
### telemetry-strategy#3.15 · values · triggers · ## 2. Telemetry Strategy · lines 42–42
| creator-explicit (Founder Directions 1/2/5) | §3 schema verbatim, §6 |
### telemetry-strategy#3.16 · values · triggers · ## 2. Telemetry Strategy · lines 43–43
| creator-explicit (Founder Direction 4) | §6 detail code catalog, §12 D-01; a strict-modes refusal made before any process log opens (the six frame verbs and `link` / `unlink`, exit 21; `run`, `revive`'s start and `verify`, exit 1) writes no line and carries no `detail` code: its machine-readable form is the exit code, and under `--json` the closed `instance-unreachable` document (the frame verbs) or `strict-modes-failed` (`revive`) |
### telemetry-strategy#3.17 · values · triggers · ## 2. Telemetry Strategy · lines 44–44
| creator-explicit (installed-CLI measured) | §5 counters, §6 `parse-rejected` / `skipped` |
### telemetry-strategy#3.18 · values · triggers · ## 2. Telemetry Strategy · lines 45–45
| creator-explicit (no screen scraping) | §4 Scenario 2 (vt100 gate outcome only), §11 |
