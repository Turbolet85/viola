# obs · Telemetry Strategy — category read

## Carried
- (.andromeda/obs-plan-detail/telemetry-strategy.md:10) Any span the chunk adds or touches on the `viola-channel` server (the listener bind, its admission checks) must be declared in the one span form, skip_all with a static `<area>.<operation>` name, and stays unserialized in v1.
  > | Spans / Traces | The 6 must-trace paths (Section 4) and every crate seam: `viola-pty` spawn/write/wait/kill, `viola-channel` client call and server dispatch, `viola-state` snapshot/heartbeat/tail, `viola-agent-claude` hook parse, `viola-mcp` tool calls | Declared in code with tracing 0.1.44 `#[instrument(skip_all, name = "<area>.<operation>", fields(...))]`. They are **not serialized** in v1, because the output sets `with_current_span(false)` + `with_span_list(false)`.
- (.andromeda/obs-plan-detail/telemetry-strategy.md:11) A remote-client rejection, a held-name bind failure or the squatted-name start refusal must surface, where it is logged at all, as a closed `event` value in the role's process log, never as free text.
  > | Structured Logs | Every boundary call, state transition, refusal, fault/heal and process lifecycle, as the closed `event` enum (Section 3) | **The primary signal.** tracing-subscriber 0.3.23 JSON-per-line, one file per process role under `<home>/diagnostics/`
- (.andromeda/obs-plan-detail/telemetry-strategy.md:12) Any new admission error type or variant in `viola-channel` must carry a fixed `Display`, and the start refusal it causes must end in a `process-exit` with a distinct `detail` code; no external reporter is added.
  > | Errors | Panics (a one-line `event:"panic"` record), typed thiserror 2.0.20 errors with fixed `Display`, exit codes plus distinct `detail` codes on `process-exit` | No external reporter: security says "no outbound network calls".
- (.andromeda/obs-plan-detail/telemetry-strategy.md:13) The chunk must add no in-process metric instrument or exporter for admission outcomes; any count is derived by `jq` over the ndjson.
  > | Metrics | `duration_ms` samples, counts of `event`/`detail` values, `skipped` counts | No in-process instruments and no exporter. Metrics are derived by `jq` over the ndjson
- (.andromeda/obs-plan-detail/telemetry-strategy.md:28) The pipe listener is the security Vector 1 (IPC) surface; whatever the chunk logs on it must meet the channel field allow-list it routes to.
  > | logging-sensitive Vector 1 (IPC) | §4 surface table (channel), §6 channel field allow-list, §8 table |
- (.andromeda/obs-plan-detail/telemetry-strategy.md:39) The Windows-only listener work must keep the same file sink and line schema as the other OSes; the endpoint shape reaches telemetry only as the closed `endpoint_kind` field.
  > | multi-platform-exporter-compat | §3 Log file location (0600/0700 vs home DACL), §3 Logging stack (concurrent-append check on windows-2025 / macos-latest / ubuntu-latest), §4 Scenario 1 (`endpoint_kind`, `pty_backend`), §9 per-OS artifacts |
- (.andromeda/obs-plan.md:123) Admission logging on the channel server goes to the log file of the process on that side, and the client's self-reported `from` is never logged as an authenticated identity.
  > propagation is the `corr` field in log lines, since channel `params` declare no trace-context field, and a merged-log join needs `process`, `instance` and a connection discriminator, a gap for Phase 3; `-32603` is `"internal error"` with `data:null`, and the self-reported `from` is never logged as an authenticated identity.
- (.andromeda/obs-plan.md:161) The squatted-name exit-1 start refusal (and any new listener-bind failure at start) keeps a distinct `detail` code free of path and pid.
  > - **Required log fields:** exit-1 causes use a distinct `detail` code, with no path or pid in it.
- (.andromeda/obs-plan.md:190) Lines the channel writes around an admitted or rejected connection never carry `params` or result bodies.
  > - **Required telemetry signal:** channel lines never carry `params` or result bodies, and `from` is logged as unauthenticated.
- (.andromeda/obs-plan.md:239) The Windows-only admission assertions must not introduce an OS-specific sink or field.
  > - **Required telemetry signal:** the same file sink and line schema on every OS, with identical fields and no OS-specific exporter.
- (.andromeda/obs-plan.md:280) The chunk's telemetry stays inside the v1 pyramid: structured logs primary, spans unserialized, joins by `corr` + `conn`, no reporter, no instruments.
  > Structured logs are the primary signal, one JSON-per-line file per process role; spans are declared in code but not serialized in v1, and cross-process joins use `corr` + `conn`; errors have no external reporter; metrics have no in-process instruments and no exporter and are derived by `jq` over the ndjson
- (.andromeda/obs-plan.md:286) Every new assertion the chunk lands (remote-reject mode read-back, second-listener failure, DACL effect) must be a machine verdict that sets an exit code.
  > - No human-review-gated log analysis. Every verdict is a `jq` / harness assertion that sets an exit code.
- (.andromeda/obs-plan.md:290) Every cross-surface channel call keeps `corr` plus `conn`; admission hardening must not drop the discriminator, and adds no W3C context.
  > - Every cross-surface call carries correlation through the tests `corr` field plus the additive `conn` discriminator (Section 3, Trace context propagation). W3C `traceparent` is not used in v1: no consumer exists and channel `params` declare no trace-context field.
- (.andromeda/obs-plan.md:293) A span added on the listener or start path takes the `channel` or `run` area and a static snake_case name.
  > - **Span naming:** `<area>.<operation>` in snake_case. `<area>` is one of `run`, `pty`, `channel`, `state`, `hook`, `statusline`, `send`, `wait`, `last`, `answer`, `pause`, `release`, `cli`, `mcp`.
- (.andromeda/obs-plan.md:294) Any event the chunk emits is a value of the closed `ObsEvent` enum; it coins no ad hoc event string.
  > - **Event naming:** the `event` field is a closed kebab-case enum (the tests list plus the Section 12 extensions). It is compiled in viola-core as `ObsEvent` with a kebab-case `Display`.
- (.andromeda/obs-plan.md:296) Fields the chunk adds follow the snake_case field / kebab-case value conventions.
  > - **Log fields:** snake_case field names. Enum values are kebab-case (arch JSON / enum naming).
- (.andromeda/obs-plan.md:311) If `src/cmd/run.rs`'s squatted-endpoint start refusal is touched, it keeps exactly two fixed stderr lines through `human::refuse`, no path or pid, and nothing else on the terminal.
  > a start refusal (`.cmd`/`.bat` child, live or stale name, tampered pinned copy, squatted endpoint) writes exactly two fixed stderr lines (e.g. `unable: <name>'s command is a .cmd or .bat script` / `hint: pass the real executable, not a .cmd or .bat shim`, no path or pid) through `src/human.rs` `refuse` → `write_refusal`, one `write_all` on the locked stderr, a closed pipe swallowed
- (.andromeda/obs-plan.md:345) The pipe's endpoint name never enters a span or a `detail` code on the start path the chunk hardens.
  > - Never the endpoint name, the `pinned_bin` path or env values in a span, and never the sideload companions' paths or hashes;
- (.andromeda/obs-plan.md:470) `interprocess` (the listener's crate) stays at `OFF` in the sink: a listener or admission failure is captured at the viola seam as a viola event.
  > | `rmcp`, `portable_pty`, `sysinfo`, `interprocess` | `OFF` (D-11; failures are captured at the viola seam as viola events) |
- (.andromeda/obs-plan.md:479) The channel's boundary events stay `channel-request` / `channel-response` with codes only and no argument digest.
  > - **IPC invocation** (viola-channel): `channel-request` / `channel-response` with method, `corr`, `conn`, result class, error code and `sender`. **No argument digest**: `params` and result bodies are never recorded, not even hashed.
- (.andromeda/obs-plan.md:557) The endpoint (pipe name) is never logged in any home-level line.
  > Home-level lines stay path-free; `endpoint` / `pinned_bin` are never logged;
- (.andromeda/obs-plan.md:645) The CARRY's log-capture fix for `send_budget_paused_is_refused_with_the_window_word_and_no_cursor` must wait on the `send-refused` line deterministically, never by a sleep.
  > - **Deterministic emission:** events key on hook events and `events.ndjson` offsets. Nothing is derived from screen content, and no log-based wait relies on sleeps (creator explicit).
- (.andromeda/obs-plan.md:683) The `viola run` start sequence is a must-trace path; hardening the listener inside it must not drop its spans or events.
  > - NEVER skip spans or events on the 6 must-trace paths (Section 4). The agent loses its ability to debug.
- (.andromeda/obs-plan.md:684) Admission telemetry carries the closed `event` and `corr`, never free text.
  > - NEVER let the signal pyramid collapse into free text. Every line carries the closed `event` enum and `corr`.
- (.andromeda/obs-plan.md:709) The CARRY's capture fix must read the `send-refused` line as `obs_event!` writes it; no raw tracing macro is added to make the line observable.
  > - NEVER call `tracing::{event,info,warn,error,debug,trace}!` directly outside `viola_core::obs`. Only `obs_event!` guarantees `event`, `process` and `instance`; the caller sets `corr` explicitly as a typed field on the events that carry one.
- (.andromeda/obs-plan.md:715) A new `event` value for a remote-reject or bind-failure outcome needs a Decisions Log entry; existing fields are not renamed.
  > - NEVER rename tests fields (for example `corr` → `correlation_id`, `event` → `event_type`) or add `event` values without a Decisions Log entry.
- (.andromeda/obs-plan.md:735) Any `detail` code the chunk adds or keeps (`squatted-name` included) holds no path or pid.
  > - NEVER put paths or pids inside `detail` codes (Founder Direction 4). Paths appear only in instance detail files.
- (.andromeda/obs-plan.md:748) The CARRY is closed by fixing the capture or by three plain `cargo test` greens as evidence, never by adding a retry policy to the flaky assertion.
  > - NEVER add retry-once policies for panics or flaky telemetry assertions.

## Bindings
- **Spans on `viola-channel` server dispatch ↔ obs Span / Trace Coverage (§4 Scenario `viola run` start sequence)**: the strategy's span form (.andromeda/obs-plan-detail/telemetry-strategy.md:10) and the scenario's ban on the endpoint name in a span (.andromeda/obs-plan.md:345) bind the same listener-bind step the chunk hardens.
- **Refusals as closed events ↔ obs Log Coverage (§6 `detail` catalog) and Decisions Log**: a refusal is a closed `event` (.andromeda/obs-plan-detail/telemetry-strategy.md:11) with a distinct path-free `detail` (.andromeda/obs-plan.md:161, .andromeda/obs-plan.md:735); a new `event` value needs a Decisions Log entry (.andromeda/obs-plan.md:715).
- **Vector 1 (IPC) trigger ↔ security-plan §Authentication & Authorization (IPC access control (Windows))**: the security clause the chunk asserts (owner+SYSTEM DACL, `accept_remote(false)`, first-instance) is the telemetry trigger routed to the channel field allow-list (.andromeda/obs-plan-detail/telemetry-strategy.md:28); `from` is never an authenticated identity in a line (.andromeda/obs-plan.md:190).
- **multi-platform-exporter-compat ↔ arch CI/CD (windows-2025 leg)**: Windows-only assertions keep one sink and schema across OSes (.andromeda/obs-plan-detail/telemetry-strategy.md:39, .andromeda/obs-plan.md:239).
- **Start refusal stderr ↔ obs Observability Harness Contract (§3) and arch exit codes**: the squatted-endpoint refusal's two fixed lines (.andromeda/obs-plan.md:311) bind with the scope's "no exit code and no refusal word changes (`squatted-name`, exit 1 stand)".
- **CARRY log capture ↔ test-plan §3 harness / zero-flakiness**: deterministic, sleep-free log waits (.andromeda/obs-plan.md:645) and no retry policy (.andromeda/obs-plan.md:748) bind the `send-refused` capture fix the chunk owes; the line itself is `obs_event!`'s (.andromeda/obs-plan.md:709).
