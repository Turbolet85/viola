# obs extract

## Relevance
relevant. The chunk adds the `mcp` process role that obs-plan names throughout: its own sink, catch site, span area and panic invariant. CARRY 3, the `mcp` role in `role_of`, is an obs-plan §7 mandate.

## Constraints
- The `mcp` role's sink is always `<home>/diagnostics/mcp.ndjson`. stderr JSON is allowed only when that file cannot be opened. Content-bearing records (chains, drift reports, panic payload and backtrace) go only to `instances/<name>/diagnostics/detail-mcp.ndjson`, and only when an instance resolves. `mcp.ndjson` has concurrent writers (one `viola mcp` per Claude session), so each line is one `write_all` on an append handle. Per obs-plan §3 Logging stack (D-09). Its wording supersedes §1's "stderr when `VIOLA_DIR` is unset", per §1's closing note.
- Init order for `mcp`, per obs-plan §3 OTel SDK init (Init order steps 1–6 and the per-role anchor `mcp`):
  - Init steps 1–6 complete before the rmcp stdio transport starts.
  - The instance source is `VIOLA_NAME` only.
  - No file under the home is opened before the home strict-modes check passes.
  - The writer is set explicitly with `log_internal_errors(false)`.
  - `process-start{subject:"self"}` is the first line.

  Whether the chunk's `[inferred]` home resolution for `viola mcp` matches step 3 (`--home` → grandparent of `VIOLA_DIR` → `~/.viola/`) is research's question.
- Role classification: §7's Main-thread catch site names `mcp` as a role subcommand that `role_of` must classify from argv before clap runs, `--home <dir> mcp` included. Per obs-plan §7 Panic hooks:
  - A main-thread panic writes one `event:"panic"` line, then `process-exit{subject:"self", exit_code:1, detail:"internal-error"}`, then exits 1, with nothing on stdout.
  - A panic inside a spawned Tokio task is logged by the hook and contained at the task boundary, and the process continues.

  Whether `src/main.rs` `role_of` already does this is research's question (scope says it files `mcp` under `cli` at HEAD).
- Spans, per obs-plan §4 Instrumentation per surface (ipc-internal viola-mcp row) and §4 Span kinds:
  - One `mcp.tool_call` span (SERVER) per tool: `send · wait · last · answer · list`.
  - The client spans it wraps are `send.client`, `wait.block`, `last.client` and `answer.client`, each › `channel.request`.
  - Required span attributes follow the §4 scenarios: `wait.block` carries `after` and `timeout_ms`. `channel.request` carries `method` and `conn`.
- Rejects: on rejects, each tool logs codes-only `send-refused` / `channel-response` lines that mirror the `isError` codes. Per obs-plan §7 Error classes captured and §4 (ipc-internal viola-mcp row):
  - The `channel-response` client side writes `instance-unreachable` when the reply never arrived.
  - The wrapper's `budget-paused` refusal is `send-refused{refusal:"budget-paused", detail:"five-hour"|"seven-day", window, used_percentage, resets_at, paused:true}`, per obs-plan §4 Scenario: Budget governor. The MCP form of that refusal (CARRY 4) carries the same closed codes.
- Fields: every line carries the common fields with `process:"mcp"` and `instance` set to the server's `VIOLA_NAME`, which is also the `from` it sends. No field outside the §6 Additive field catalog may be logged (default-deny, G4 against `schemas/diag-line.v1.json`). Per obs-plan §6 Required fields (every log line). Whether the schema's `process` enum and the `ObsProcess::Mcp` mapping already admit `mcp` is research's question.
- The client generates `conn` as `mcp-<pid>-<t0>-<n>` and sends it in `params`. `corr`, `conn` and `instance` are passed as explicit values into every Tokio task and never recovered from span context. Spawned futures use `.instrument(span)` / `.in_current_span()`. Per obs-plan §3 Trace context propagation (IPC boundaries, Internal async boundaries).

## Patterns to follow
- Emit through `obs_event!` with explicit typed fields. `corr` is a plain typed `key = value` field, never `?`/`%`, and absent when null. Per obs-plan §3 Logging stack.
- `#[instrument(skip_all, name = "mcp.tool_call", fields(...))]` with a static name. No instance, dialog id or path goes in a name. Per obs-plan §4 Span naming convention.
- `channel-response` is emitted from a `Drop` guard, so an early return or an error still logs it, as the wrapper's `channel.dispatch` does. Per obs-plan §4 Scenario: Confirmed `send` (CL-1), Cleanup.
- Error layer: a fixed-message thiserror `Display` on `McpError`. At dispatch, an anyhow chain with a serde source becomes `internal error` before it reaches `isError`. The full chain goes as `chain:[...]` to `detail-mcp.ndjson` through the root-bin `viola::obs` detail writer. Per obs-plan §7 `before_send` scrubbing pattern (layers 1–3).
- Payload types get veil `#[derive(Redact)]` with `#[redact]` on content fields (`SendParams.text`, `AnswerParams.response`, `LastResult.last_assistant_message`). Only `text_bytes` is logged. Per obs-plan §8 PII Scrubbing (data classification table, Integration points 2).

## Anti-patterns to avoid
- On stdout and stderr in the `mcp` code path:
  - No telemetry on stdout, and no `print!` / `println!` / `eprint!` / `eprintln!` / `dbg!` (clippy `print_stdout` / `print_stderr` / `dbg_macro` = deny).
  - stdout carries MCP frames only.
  - `with_writer` is always explicit.

  Per obs-plan §11 Logs.
- rmcp's native `tracing` target stays `OFF` in the `filter::Targets`. Never install `tracing_log::LogTracer`, `EnvFilter` or `RUST_LOG`. rmcp failures are captured at viola's seam from the returned `Result`s. Per obs-plan §6 Per-module log levels (D-11) and §11 Logs.
- No bare `#[instrument]`, and no `err` / `ret` on functions whose error or return can carry a serde source or user content: send text, the answer `response` and `last_assistant_message` never reach a span field. No heartbeat tick line for `mcp`, which is request-driven. Per obs-plan §11 Spans / Traces and §3 Heartbeat ticks.

## Contract bindings
- obs ↔ tests. Per obs-plan §9 Pipeline integration (Integration tests row), §3 Logging stack and §8 Integration points 5:
  - The new MCP integration tests' homes must live under `target/e2e-home/`, so that G2, G4, the secret scan and the `diag-<os>` upload cover `mcp.ndjson` and `detail-mcp.ndjson`.
  - The tests-owned concurrent-append check must cover `mcp.ndjson` as a shared file.
  - The content canary rule applies to MCP `send` / `answer` inputs.
- obs ↔ security, per obs-plan §4 Scenario: `wait` / `last` event-driven readback (`process-exit` bullet) and §8 PII Scrubbing:
  - security Vector 5 (MCP) codes-only `isError` is mirrored by codes-only log lines.
  - The boundary fork (verification before the first frame, or an eighth liveness-only exception) decides whether `strict-modes-failed` / `server-verify-failed` can occur on the `mcp` client. Under a liveness-only pre-check they cannot, as for `wait` / `last`.
- obs ↔ architecture (channel), per obs-plan §3 Trace context propagation:
  - The `(conn, corr)` join key and `(conn, rpc_id)` on the wrapper's `send-*` lines (D-25, D-30) link the `mcp` side to the wrapper side.
  - Each side logs its own `instance`.
- obs ↔ list (CARRY gap for research). §6 Boundary-call wrappers (Child / shell spawns) names the writers of the `agents-probe` pair as `viola list` (into `cli-<name>.ndjson`), `revive` and verify's Run B. It does not name the `mcp` `list` tool. If that tool reuses `run::agents::read_logged`, the pair lands in `mcp.ndjson` as a writer the plan does not record. Per obs-plan §6 Boundary-call wrappers.

## Acceptance criteria contributions
- A fake-agent integration run of `viola mcp` (all five tools, including at least one refusal of each class) meets all of the following. Per obs-plan §3 Logging stack and §9 Gate commands (G4).
  - It writes nothing to stdout but MCP frames.
  - Its lines are in `<home>/diagnostics/mcp.ndjson` (0600) with `process:"mcp"`, `instance` set to the server's `VIOLA_NAME`, and one `mcp.tool_call`-backed `channel-request` / `channel-response` pair per call.
  - Every line passes G4 against `schemas/diag-line.v1.json`.
- `role_of` classifies `viola mcp` and `viola --home <dir> mcp` as `mcp`. A forced main-thread panic leaves exactly one `event:"panic"` line in `mcp.ndjson`, then `process-exit{subject:"self", exit_code:1, detail:"internal-error"}`, with exit 1 and empty stdout. Per obs-plan §7 Panic hooks and §10 Always-required SLO invariant.
- A synthetic canary sent as MCP `send` text, or as an `answer` response, never appears in `mcp.ndjson` or any home-level `diagnostics/*.ndjson`; it may appear only in `detail-*.ndjson`. Every `isError` result and its mirror line carry closed codes only, with no path, upstream text or serde chain. Per obs-plan §8 PII Scrubbing (Integration points 5) and §7 `before_send` scrubbing pattern.
- An MCP `send` refused on a paused budget logs the wrapper's `send-refused{refusal:"budget-paused", detail:"five-hour"|"seven-day", …, paused:true}`. The tool's `isError` carries the same `refusal` / `detail` pair, with no cursor and no hint naming `release`. Per obs-plan §4 Scenario: Budget governor.
