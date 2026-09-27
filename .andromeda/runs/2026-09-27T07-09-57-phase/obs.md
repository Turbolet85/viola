# obs extract

## Relevance
Relevant. This chunk adds the first `viola-channel` spans and events (`channel-request` / `channel-response`), the additive `conn` field, the `channel.bind` start-sequence span, and folds in the `corr`-required schema CARRY and the Scenario 1 span CARRY.

## Constraints
- **`conn` format and fallback.** The client builds `conn = "<process>-<pid>-<t0>-<n>"`. `t0` is the process start instant in Unix epoch ms, captured once at init. `n` is a per-process connection counter. `conn` goes in `params` beside `v` / `sender` / `from` and is logged on both sides. A server that gets a frame without `conn` logs `srv_conn = "srv-<accept counter>"` instead. `conn` is for log correlation only and must never be treated as identity (per obs-plan §3 Trace context propagation; §12 D-10, D-32). The merged-log join key is `(conn, corr)` with `corr` = the JSON-RPC `id`. `instance` is deliberately left out of the key (per obs-plan §3 Trace context propagation, D-25).
- **Codes-only channel lines.** `channel-request` may carry `method`, `conn`/`srv_conn`, `from` + `from_trust:"self-reported"`, `sender`, `v` (plus `after` / `timeout_ms` for later methods). `channel-response` may carry `method`, `conn`/`srv_conn`, `result_class` (`ok|refusal|error`), `error_code` (`-32700|-32600|-32601|-32602|-32603`), `refusal`, `detail`, `outcome`, `duration_ms`. `params` and result bodies must never be recorded, not even as a hash (per obs-plan §6 Additive field catalog and Boundary-call wrappers; §4 surface table, ipc-internal viola-channel row).
- **Levels.** `-32600` / `-32602` responses (this includes the newer-peer "unsupported protocol version" refusal) log at `warn`. `-32603` logs at `error`. Per-frame framing detail is `debug` only. A frame over `MAX_FRAME` is logged as `parse-rejected{parser:"channel-frame", detail:"oversize"}` at `warn` (per obs-plan §6 Log levels mapping and `detail` code catalog).
- **Sync-only instrumentation.** `viola-channel` takes `tracing = "0.1.44"` and no Tokio. Server worker threads take a cloned `Span` and run inside `span.in_scope(...)`. `corr`, `conn` and `instance` are passed as explicit values to each `obs_event!` and are never read back from span context. The `interprocess` target stays `OFF`; its failures are captured at viola's seam from the returned `Result`s (per obs-plan §3 Logging stack / Trace context propagation → Internal async boundaries; §6 Per-module log levels; §12 D-11).
- **Span names and kinds.** `channel.request` is CLIENT and `channel.dispatch` is SERVER. Both use `#[instrument(skip_all, name = "<area>.<operation>", fields(...))]`; the `err` and `ret` options are banned. The Scenario 1 span chain `run.start` › `run.collision_check` › `run.pin_copy` › `run.version_gate` › `channel.bind` › `state.snapshot_write` › `state.heartbeat_start` › `pty.spawn` has these required attributes:
  - `channel.bind`: `endpoint_kind` (`named-pipe|unix-socket`)
  - `pty.spawn`: `pty_backend`, `env_stripped_count`
  - `run.collision_check`: `outcome`
  - `state.snapshot_write`: `v`

  The endpoint name and the `pinned_bin` path are never logged (per obs-plan §4 Span naming / Span kinds / Scenario 1; §8 Data classification, the `endpoint` / `pinned_bin` row).
- **Error surface.** `ChannelError` is thiserror 2.0.20 with a fixed `Display`. `-32603` is always `"internal error"` with `data:null`. A panic in one connection worker is contained: the `channel.dispatch` `Drop` guard logs `channel-response{result_class:"error", error_code:-32603}`, that connection closes, and `run` continues (per obs-plan §7 Platform pick, Scrubbing layers, Panic hooks → `run` worker threads).
- **Bind-loser exit.** The loser of the exclusive bind logs `process-exit{subject:"self", exit_code:1, detail}` at `error`. `detail` comes from the closed catalog (`already-live|squatted-name|…`) and never contains a path or pid (per obs-plan §4 Scenario 1 Required log fields; §6 `detail` code catalog). Which catalog code the bind-arbiter loser maps to is a P4 decision.

## Patterns to follow
- Emit every line through `obs_event!`. It attaches `event`, `process` and `instance`. The caller passes `corr` as a typed `key = value` field, never `?corr` or `%corr` (per obs-plan §3 Logging stack).
- Emit `channel-response` from a `Drop` guard inside `channel.dispatch`, so early returns and errors still log it (per obs-plan §4 Scenario: Confirmed `send`, Cleanup).
- Put veil 0.3.0 `#[derive(Redact)]` on the channel payload/envelope types, and ban `toggle` in cargo-deny. This is the second layer behind `skip_all` (per obs-plan §8 Scrubbing libraries; Integration points 1–2).
- Put content-bearing error chains only in the instance's `detail-<process>.ndjson`, and only when an instance resolves. The detail line carries the same `event`, `corr` and `conn` as its home-level line (per obs-plan §3 Sink; §7 Error → span correlation).
- Log `process-start{subject:"self", …, endpoint_kind}` on `run` (per obs-plan §6 Additive field catalog, `process-start` row; §4 Scenario 1).

## Anti-patterns to avoid
- A bare `#[instrument]`, or `#[instrument(err|ret)]`, on channel functions. Also any direct `tracing::{info,warn,error,debug,trace,event}!` outside `viola_core::obs`: the lint-probes grep and clippy `disallowed-macros` will fail the build (per obs-plan §11 Spans / Traces; §11 Logs).
- Logging `params`, result bodies, the endpoint name/path, or `from` as if it were an authenticated identity (per obs-plan §11 PII Scrubbing; §3 Trace context propagation).
- Using `print!`/`eprintln!` in the `run` path's channel server, or relying on span context to carry `corr`/`conn` across std threads (per obs-plan §11 Logs; §11 Spans / Traces).

## Contract bindings
- **obs ↔ tests §3 Log format / `schemas/diag-line.v1.json` (G4).**
  - `channel-request` / `channel-response` need `corr` = the JSON-RPC `id`.
  - `conn` / `srv_conn` must be in the per-event allow-list, which is default-deny via top-level `unevaluatedProperties: false` (per obs-plan §3 Log format JSON schema; §8 Default-deny posture).
  - The CARRY makes `corr` `required` on every corr-bearing event.
- **obs ↔ security Vector 1 (IPC).** `-32603` is fixed as `"internal error"` with `data:null`, and `from` is unauthenticated (per obs-plan §1 Telemetry triggers; §7).
- **obs ↔ arch Mixed-version tolerance.** `conn` is an additive `params` field. Readers skip unknown fields. The D-10 arch acknowledgment covers the D-32 format (per obs-plan §12 D-10, D-32).
- **obs ↔ tests secret-scan / multi-OS.**
  - Same line schema on named pipe and on Unix socket.
  - `diagnostics/` files are 0600/0700 (DACL on Windows) (per obs-plan §1 Telemetry triggers, multi-platform-exporter-compat; §8 Integration point 6).

## Acceptance criteria contributions
- A `diag-line.v1.json` negative test: a `channel-request` / `channel-response` (and every other corr-bearing event) line without `corr` is rejected. A line with `conn` or `srv_conn` validates, and a literal `null` `corr` is rejected (per obs-plan §3 Log format JSON schema, Null encoding; §8 Default-deny posture).
- For one end-to-end channel call, both sides' role files contain `channel-request` + `channel-response` with an equal `(conn, corr)`. `conn` matches `^[a-z]+-\d+-\d+-\d+$`. A frame without `conn` produces `srv_conn:"srv-<n>"` on the server line (per obs-plan §3 Trace context propagation; §12 D-10, D-32).
- A newer-`v` request produces `channel-response{error_code:-32602, result_class:"error"}` at `warn`. An unknown method produces `error_code:-32601`. A frame over `MAX_FRAME` produces `parse-rejected{parser:"channel-frame", detail:"oversize"}`. No channel line contains `params`/result content, an endpoint path, or a pid inside `detail` (per obs-plan §6 Log levels mapping, `detail` code catalog, Boundary-call wrappers).
- G1 finds no bare `#[instrument]`, `lint-probes.sh` finds no raw `event!`, and `cargo check` of `viola-channel` without its `tokio` feature passes. The `run` start sequence emits `process-start{subject:"self", endpoint_kind}`, and the Scenario 1 spans carry their required attributes (per obs-plan §3 Bootstrap phases → logger-stack-install / obs-ci-gate-wire; §4 Scenario 1).

## Relevant amendment history
- **2026-09-24-diagnostics-plane:** §3/§11 were amended to match what shipped: `obs_event!` has no `corr` arm, so the caller supplies `corr`. The operator closed the trade-off with a CARRY on this chunk: make `corr` `required` in `diag-line` for corr-bearing events, with a negative test. §8 changed the default-deny mechanism to top-level `unevaluatedProperties: false`.
- **2026-09-24-observability-gates:** the raw-tracing ban is now split. clippy `disallowed-macros` bans the level macros, and a fail-closed `scripts/lint-probes.sh` grep catches raw `event!`. Both apply to the new `viola-channel` crate. The reason is that no inner allow can exempt `obs_event!`'s expansion.
- **2026-09-25-pty-wrapper-on-windows:** a proposal to make `viola-pty` tracing-free was rejected as a sequencing change. Obs §3/§4 require seam spans, and "Wrapper channel" owns the first spans. A CARRY names `pty.spawn` and the open question of where the seam spans live. D-34 (`env_kept`, names only) shapes the claude-child `process-start` fields.
- **2026-09-27-instance-state-and-start-order:** `already-live` covers both `live` and `stale` holders, and a `gone` holder is taken over (`run.collision_check` outcome `free`, no exit-1 line). This matters for how the bind-arbiter loser is logged. Proposal O1 (a "spans not yet declared" note under §4 Scenario 1) was rejected; instead the Scenario 1 spans are CARRY'd to this chunk.
- **2026-09-24-log-redaction-and-never-log-floor:** anyhow scope is the root-bin dispatch edge plus `viola::obs::report_internal_error`. The `chain:[...]` field goes only to detail files. The first redaction subjects (veil, `#[instrument(skip_all)]`, fixed-`Display` `ChannelError`) are CARRY'd to this chunk.
