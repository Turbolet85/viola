# obs · cascade read · group 4 of 7

## Carried
- (.andromeda/obs-plan-detail/telemetry-strategy.md:10) The chunk edits the `viola-channel` client call (the sync and Tokio connects, the server identity read), so that seam and every verification span the chunk adds must be declared `<area>.<operation>` under an existing area, unserialized in v1, with cross-process joins by `corr` + `conn` only.
  > | Spans / Traces | The 6 must-trace paths (Section 4) and every crate seam: `viola-pty` spawn/write/wait/kill, `viola-channel` client call and server dispatch, `viola-state` snapshot/heartbeat/tail, `viola-agent-claude` hook parse, `viola-mcp` tool calls | Declared in code with tracing 0.1.44 `#[instrument(skip_all, name = "<area>.<operation>", fields(...))]`. They are **not serialized** in v1, because the output sets `with_current_span(false)` + `with_span_list(false)`. They give the code its structure and are ready for the deferred tracing-opentelemetry 0.34.0 bridge. Cross-process joins use `corr` + `conn`, not W3C trace IDs |
  (.andromeda/obs-plan.md:293)
  > `<area>` is one of `run`, `pty`, `channel`, `state`, `hook`, `statusline`, `send`, `wait`, `last`, `answer`, `pause`, `release`, `cli`, `mcp`.
- (.andromeda/obs-plan-detail/telemetry-strategy.md:11) A failed server verification is a new refusal at a boundary call, so every client that takes it must write it as a closed `event` line in its own role file under `<home>/diagnostics/`, codes only, with content going only to the instance's detail file.
  > | Structured Logs | Every boundary call, state transition, refusal, fault/heal and process lifecycle, as the closed `event` enum (Section 3) | **The primary signal.** tracing-subscriber 0.3.23 JSON-per-line, one file per process role under `<home>/diagnostics/`, merged with `events.ndjson` by `agent-run logs` |
  (.andromeda/obs-plan.md:280)
  > Structured logs are the primary signal, one JSON-per-line file per process role;
  (.andromeda/obs-plan.md:312)
  > Every role's codes-only process log goes to its home-level role file under `diagnostics/`, and content-bearing detail goes only to the instance's `diagnostics/detail-<role>.ndjson`.
- (.andromeda/obs-plan-detail/telemetry-strategy.md:12) The chunk's new typed verification error must carry a fixed `Display`, and its exit-21 outcome a `detail` code on `process-exit` distinct from the liveness causes; the hook's verification path must stay inside the `catch_unwind` around `hook` dispatch so a panic still fails open.
  > | Errors | Panics (a one-line `event:"panic"` record), typed thiserror 2.0.20 errors with fixed `Display`, exit codes plus distinct `detail` codes on `process-exit` | No external reporter: security says "no outbound network calls". A hand-written `std::panic::set_hook`, plus `std::panic::catch_unwind` at the main-thread catch site of every role, around `hook` dispatch, in the `run` PTY pump and handle-wait threads, and around the vt100 feed (Section 7) |
- (.andromeda/obs-plan-detail/telemetry-strategy.md:13) The hook perf budget the chunk re-measures with the added peer-credential and start-time read must be read from the hyperfine `--export-json` perf gates and `duration_ms` samples, with no in-process instrument added for the verification's timing.
  > | Metrics | `duration_ms` samples, counts of `event`/`detail` values, `skipped` counts | No in-process instruments and no exporter. Metrics are derived by `jq` over the ndjson; the perf gates run on hyperfine 1.20.0 `--export-json` (Section 5) |
  (.andromeda/obs-plan.md:286)
  > No human-review-gated log analysis. Every verdict is a `jq` / harness assertion that sets an exit code.
- (.andromeda/obs-plan-detail/telemetry-strategy.md:14) The chunk rules whether a verification mismatch in the hook reads as not delivered and so runs `SessionEnd`'s direct append to `events.ndjson`, the product audit stream; the hook's process line naming the outcome taken must not copy the event payload.
  > | Audit Logs | `instances/<name>/events.ndjson` is the product audit stream (arch-owned, never rotated) | Not a compliance trail (security Minimal, no compliance triggers). Obs never copies event payloads into process logs |
  (.andromeda/obs-plan.md:280)
  > `events.ndjson` is the product audit stream, not a compliance trail, and obs never copies event payloads into process logs.
- (.andromeda/obs-plan-detail/telemetry-strategy.md:28) The chunk edits the IPC client's connect and adds the server identity read, so the channel surface's coverage and its field allow-list (no `params` or result body, no `endpoint` value) bind every line on that path.
  > | logging-sensitive Vector 1 (IPC) | §4 surface table (channel), §6 channel field allow-list, §8 table |
- (.andromeda/obs-plan-detail/telemetry-strategy.md:29) A `send` (CLI or MCP) refused before its frame by a failed verification must log codes only, never the paste text.
  > | logging-sensitive Vector 2 (paste text) | §4 Scenario 2 (`send-refused` `side`), §8 |
- (.andromeda/obs-plan-detail/telemetry-strategy.md:30) The hook's fail-open-on-mismatch path must write only to the hook sink with a fail-open `hook-decision` `detail` code, nothing on stdout or stderr; if the strict-modes half comes with the chunk, the hook's refused-instance precedent is no line at all, asserted by absence, and the plan must say which of the two the hook client takes.
  > | logging-sensitive Vector 4 (hook stdin) | §3 Logging stack (hook sink), §4 Scenario 4, §6 `hook-decision` `detail` codes. The "missing `VIOLA_NAME`" item is **overridden**: no file and no line, asserted by absence (D-16, §4 E1). The "strict-modes failure" item writes no line as landed either: `viola hook statusline`'s refused instance check leaves no trace, asserted by absence (chunk 2026-10-10-budget-governor) |
  (.andromeda/obs-plan.md:310)
  > `hook` writes only its decision body to stdout and nothing to stderr.
- (.andromeda/obs-plan-detail/telemetry-strategy.md:31) The MCP tools' `instance-unreachable` on a failed verification must be an `isError` code only, mirrored by a codes-only `mcp` line, with nothing but MCP frames on stdout.
  > | logging-sensitive Vector 5 (MCP) | §4 surface table (mcp), §6 |
  (.andromeda/obs-plan.md:313)
  > `mcp` writes only MCP frames to stdout.
- (.andromeda/obs-plan-detail/telemetry-strategy.md:33) The snapshot the verification reads is a filesystem input: if P4 takes the strict-modes half, a client refusal must be the catalogued `strict-modes-failed`; under the founder's hold the five readers stay on `read_snapshot` and write no `state-recovered` line.
  > | logging-sensitive Vector 7 (filesystem) | §3 Log file location (0600/0700), §6 `state-recovered` / `skipped`, §6 detail catalog Filesystem refusals (`strict-modes-failed`, `pinned-hash-mismatch`, `symlink-ignored` via `parse-rejected{parser:"state-entry"}`), §9 secret-scan-before-upload |
- (.andromeda/obs-plan-detail/telemetry-strategy.md:35) The sysinfo start-time and peer-credential reads the chunk adds to the sync client crates must keep the logging stack's feature set and the cargo-deny gates as they stand: no new obs dependency and no Tokio in the sync graph.
  > | logging-sensitive Vector 9 (supply chain) | §3 Logging stack (feature set), §9 cargo-deny gates, §11 |
- (.andromeda/obs-plan-detail/telemetry-strategy.md:36) Every channel call a verified client makes must carry `corr` + `conn`, and a frame the verification stops writes no `channel-request` line; no trace-context field is added to `params`.
  > | cross-surface-trace-propagation | §3 Trace context propagation (`corr` + `conn`), §4 Scenarios 2/4 |
  (.andromeda/obs-plan.md:290)
  > Every cross-surface call carries correlation through the tests `corr` field plus the additive `conn` discriminator (Section 3, Trace context propagation). W3C `traceparent` is not used in v1: no consumer exists and channel `params` declare no trace-context field.
- (.andromeda/obs-plan-detail/telemetry-strategy.md:37) The added verification read must keep the hook paths inside their §10 budget-table rows, re-measured through the §5 perf gate.
  > | perf-budget-instruments | §5, §10 budget table |
- (.andromeda/obs-plan-detail/telemetry-strategy.md:38) The chunk's exit-21 `process-exit` on a failed verification, and the exiting-wrapper window it re-reads, must keep `exit_source` + `during` on `process-exit`, and `state-recovered` stays off the clients under the hold.
  > | chaos-instrumentation | §6 `state-recovered` / `process-exit` `exit_source` + `during`, §10 |
- (.andromeda/obs-plan-detail/telemetry-strategy.md:39) The macOS euid-plus-dir gap and the per-OS server pid read (Unix `peer_creds`, Windows the pipe's server process id) must keep one file sink, line schema and field set on every OS, with no OS-specific field for the gap.
  > | multi-platform-exporter-compat | §3 Log file location (0600/0700 vs home DACL), §3 Logging stack (concurrent-append check on windows-2025 / macos-latest / ubuntu-latest), §4 Scenario 1 (`endpoint_kind`, `pty_backend`), §9 per-OS artifacts |
- (.andromeda/obs-plan-detail/telemetry-strategy.md:42) Each new `detail` value the chunk adds must be admitted by the §3 line schema kept verbatim, as kebab-case under snake_case fields.
  > | creator-explicit (Founder Directions 1/2/5) | §3 schema verbatim, §6 |
  (.andromeda/obs-plan.md:296)
  > **Log fields:** snake_case field names. Enum values are kebab-case (arch JSON / enum naming).
- (.andromeda/obs-plan-detail/telemetry-strategy.md:43) The chunk adds `server-verify-failed` (and `strict-modes-failed` if the strict-modes half comes) and the hook's fail-open values to the closed `detail` code catalog, Founder Direction 4: no path, pid or start time in a code.
  > | creator-explicit (Founder Direction 4) | §6 detail code catalog, §12 D-01 |

## Bindings
- **telemetry-strategy Errors ↔ log-coverage detail catalog**: the distinct exit-21 `detail` code a failed verification carries on `process-exit` is the catalog entry Founder Direction 4 closes, and the line schema must admit it (.andromeda/obs-plan-detail/telemetry-strategy.md:12), (.andromeda/obs-plan-detail/telemetry-strategy.md:43), (.andromeda/obs-plan-detail/telemetry-strategy.md:42)
- **telemetry-strategy Vector 4 ↔ span-trace-coverage Scenario 4 (dialog → `answer`)**: the hook's fail-open on a mismatch writes no `hook.dialog` frame, so the scenario's hook lines and the hook sink's `hook-decision` code move together; whether the strict-modes half writes a line or none follows the no-trace precedent (.andromeda/obs-plan-detail/telemetry-strategy.md:30), (.andromeda/obs-plan.md:310)
- **telemetry-strategy trace propagation ↔ span-trace-coverage Scenarios 2/3/5 client side**: a client stopped by the verification makes no channel call, so `corr` + `conn` and `channel-request` appear only on calls made (.andromeda/obs-plan-detail/telemetry-strategy.md:36), (.andromeda/obs-plan.md:290)
- **telemetry-strategy Vector 7 ↔ security strict-modes half (P4) and the founder's snapshot hold**: a strict-modes refusal in the clients is `strict-modes-failed`, and `state-recovered` stays off the five readers unless his word moves them (.andromeda/obs-plan-detail/telemetry-strategy.md:33), (.andromeda/obs-plan-detail/telemetry-strategy.md:38)
- **telemetry-strategy Audit Logs ↔ the hook's `SessionEnd` fallback (the Windows exiting-wrapper CARRY)**: whether a mismatch runs the direct append to `events.ndjson` is the product decision, and the hook's process line names the outcome without the payload (.andromeda/obs-plan-detail/telemetry-strategy.md:14)
- **telemetry-strategy Metrics / perf trigger ↔ slo-invariants-telemetry-budgets budget table**: the added read is measured by the hyperfine perf gate against the hook rows, never by an instrument (.andromeda/obs-plan-detail/telemetry-strategy.md:13), (.andromeda/obs-plan-detail/telemetry-strategy.md:37)
- **telemetry-strategy Vector 9 ↔ the sync-crate Tokio ban**: the start-time and peer-credential reads keep `viola-channel` and `viola-state` sync with no new obs dependency (.andromeda/obs-plan-detail/telemetry-strategy.md:35)
- **telemetry-strategy multi-platform ↔ the macOS recorded gap**: one line schema on every OS (.andromeda/obs-plan-detail/telemetry-strategy.md:39)
