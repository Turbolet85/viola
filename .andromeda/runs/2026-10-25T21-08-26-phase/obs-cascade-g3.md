# obs · cascade read · group 3 of 5

## Carried
- (.andromeda/obs-plan-detail/telemetry-strategy.md:10) Any span the chunk adds on the `viola-channel` listener bind or `run`'s start sequence must be declared `#[instrument(skip_all, name = "<area>.<operation>", fields(...))]`, stays unserialized in v1, and must not carry the cross-process join, which goes by `corr` + `conn`.
  > | Spans / Traces | The 6 must-trace paths (Section 4) and every crate seam: `viola-pty` spawn/write/wait/kill, `viola-channel` client call and server dispatch, `viola-state` snapshot/heartbeat/tail, `viola-agent-claude` hook parse, `viola-mcp` tool calls | Declared in code with tracing 0.1.44 `#[instrument(skip_all, name = "<area>.<operation>", fields(...))]`. They are **not serialized** in v1, because the output sets `with_current_span(false)` + `with_span_list(false)`. They give the code its structure and are ready for the deferred tracing-opentelemetry 0.34.0 bridge. Cross-process joins use `corr` + `conn`, not W3C trace IDs |
  (.andromeda/obs-plan.md:293)
  > - **Span naming:** `<area>.<operation>` in snake_case. `<area>` is one of `run`, `pty`, `channel`, `state`, `hook`, `statusline`, `send`, `wait`, `last`, `answer`, `pause`, `release`, `cli`, `mcp`.
- (.andromeda/obs-plan-detail/telemetry-strategy.md:11) The squatted-name start refusal and the CARRY case_4 `send-refused` line must be read back from the JSON-per-line role file under `<home>/diagnostics/`, so the case_4 capture fix must observe the line through that pipeline, never through another channel.
  > | Structured Logs | Every boundary call, state transition, refusal, fault/heal and process lifecycle, as the closed `event` enum (Section 3) | **The primary signal.** tracing-subscriber 0.3.23 JSON-per-line, one file per process role under `<home>/diagnostics/`, merged with `events.ndjson` by `agent-run logs` |
  (.andromeda/obs-plan-detail/telemetry-strategy.md:42)
  > | creator-explicit (Founder Directions 1/2/5) | §3 schema verbatim, §6 |
  (.andromeda/obs-plan.md:280)
  > Structured logs are the primary signal, one JSON-per-line file per process role; spans are declared in code but not serialized in v1, and cross-process joins use `corr` + `conn`;
  (.andromeda/obs-plan.md:294)
  > - **Event naming:** the `event` field is a closed kebab-case enum (the tests list plus the Section 12 extensions). It is compiled in viola-core as `ObsEvent` with a kebab-case `Display`.
- (.andromeda/obs-plan-detail/telemetry-strategy.md:12) Any new or changed `ChannelError` variant from a remote-client or first-instance refusal must keep a fixed thiserror `Display`, and the re-verified squatted-name exit-1 start refusal must keep its distinct `detail` code on `process-exit`, with no external reporter.
  > | Errors | Panics (a one-line `event:"panic"` record), typed thiserror 2.0.20 errors with fixed `Display`, exit codes plus distinct `detail` codes on `process-exit` | No external reporter: security says "no outbound network calls". A hand-written `std::panic::set_hook`, plus `std::panic::catch_unwind` at the main-thread catch site of every role, around `hook` dispatch, in the `run` PTY pump and handle-wait threads, and around the vt100 feed (Section 7) |
  (.andromeda/obs-plan-detail/telemetry-strategy.md:38)
  > | chaos-instrumentation | §6 `state-recovered` / `process-exit` `exit_source` + `during`, §10 |
  (.andromeda/obs-plan-detail/telemetry-strategy.md:43)
  > | creator-explicit (Founder Direction 4) | §6 detail code catalog, §12 D-01 |
  (.andromeda/obs-plan-detail/telemetry-strategy.md:32)
  > | logging-sensitive Vector 6 (CLI/env) | §4 Scenario 1 (`env_stripped_*`), §7 fixed messages, §8 |
  (.andromeda/obs-plan.md:311)
  > `run` writes nothing to the terminal except the child's own output, with one exception before any child is spawned: a start refusal (`.cmd`/`.bat` child, live or stale name, tampered pinned copy, squatted endpoint) writes exactly two fixed stderr lines
- (.andromeda/obs-plan-detail/telemetry-strategy.md:20) A panic in the new Windows listener tests or in `run`'s start must leave exactly one single-line record in its home-level role file, so G2 reads it on the Windows leg; payload and backtrace go only to the instance detail file.
  > - Every panic after init step 4 produces exactly one single-line structured record in its home-level role file: location, thread, role and instance.
  (.andromeda/obs-plan-detail/telemetry-strategy.md:40)
  > | error-budget-SLO | §7, §10 zero-unlogged-panics |
  (.andromeda/obs-plan.md:288)
  > - Every panic after init step 4 produces exactly one single-line structured record in its home-level role file: location, thread, role and instance, except the bounded exemptions listed in Section 10; the payload and the backtrace are written only to the instance-scoped detail file (Section 7).
- (.andromeda/obs-plan-detail/telemetry-strategy.md:28) The chunk works on the IPC vector's Windows listener, so any line added on the bind or refusal path must keep the channel field allow-list and record no `params`, pipe name or result body.
  > | logging-sensitive Vector 1 (IPC) | §4 surface table (channel), §6 channel field allow-list, §8 table |
- (.andromeda/obs-plan-detail/telemetry-strategy.md:29) The CARRY case_4 asserts the `send-refused` line this vector covers, which must carry its refusal fields and no sent text.
  > | logging-sensitive Vector 2 (paste text) | §4 Scenario 2 (`send-refused` `side`), §8 |
- (.andromeda/obs-plan-detail/telemetry-strategy.md:35) Any Windows API feature or test dependency the plan adds to `viola-channel` to read the pipe mode back must pass the cargo-deny gates and leave the logging stack's feature set as it stands.
  > | logging-sensitive Vector 9 (supply chain) | §3 Logging stack (feature set), §9 cargo-deny gates, §11 |
- (.andromeda/obs-plan-detail/telemetry-strategy.md:39) The Windows-only pipe assertions must keep one file sink and line schema on every OS, keep `endpoint_kind` on the start scenario's line, and leave their homes in the per-OS diagnostics artifacts.
  > | multi-platform-exporter-compat | §3 Log file location (0600/0700 vs home DACL), §3 Logging stack (concurrent-append check on windows-2025 / macos-latest / ubuntu-latest), §4 Scenario 1 (`endpoint_kind`, `pty_backend`), §9 per-OS artifacts |

## Bindings
- **telemetry-strategy spans ↔ span-trace-coverage**: a span added on the listener bind takes a static `channel.<op>` name with `skip_all` and no `err` / `ret`, inside the `viola run` start scenario whose surfaces include the channel bind, (.andromeda/obs-plan-detail/telemetry-strategy.md:10) ↔ (.andromeda/obs-plan.md:327) and (.andromeda/obs-plan.md:343).
- **telemetry-strategy structured logs and errors ↔ log-coverage**: the re-verified `process-exit` and `send-refused` lines keep the verbatim schema, the closed `event` enum and the `detail` catalog, which grows only through the plan, (.andromeda/obs-plan-detail/telemetry-strategy.md:43) ↔ (.andromeda/obs-plan.md:432).
- **telemetry-strategy errors ↔ error-capture-reporting**: a new `ChannelError` variant takes a fixed thiserror `Display` and no reporter, (.andromeda/obs-plan-detail/telemetry-strategy.md:12) ↔ (.andromeda/obs-plan.md:496).
- **telemetry-strategy panics ↔ slo-invariants-telemetry-budgets**: one `event:"panic"` line per panic is what G2 counts over the new tests' home-level role files, (.andromeda/obs-plan-detail/telemetry-strategy.md:20) ↔ (.andromeda/obs-plan.md:637).
- **telemetry-strategy triggers ↔ pii-scrubbing-compliance**: the IPC and paste-text vectors keep content out of home-level lines on the listener path and the `send-refused` line, (.andromeda/obs-plan-detail/telemetry-strategy.md:28) ↔ (.andromeda/obs-plan.md:555).
- **telemetry-strategy multi-platform ↔ ci-integration**: the Windows-only assertions reach the per-OS diagnostics artifacts through the existing test job, (.andromeda/obs-plan-detail/telemetry-strategy.md:39) ↔ (.andromeda/obs-plan.md:596).
