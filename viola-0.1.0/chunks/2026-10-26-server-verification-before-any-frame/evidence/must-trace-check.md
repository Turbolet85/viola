# The operator's check — the must-trace paths keep their spans and events on the new refusal paths

The operator's ask at invocation: obs-plan's rule that the six must-trace paths (obs-plan §4, detail
`.andromeda/obs-plan-detail/span-trace-coverage.md`) never skip their spans or events — confirm the new pre-frame refusal
paths keep both, and record what is found.

## The six paths, and which the chunk reaches

| Scenario (obs-plan §4) | New refusal on it? | Root span kept | Lines on the refusal |
|---|---|---|---|
| `viola run` start sequence | no — `run` writes no frame and holds no client (`run.collision_check` reads the snapshot only) | unchanged | unchanged |
| Confirmed `send` (CL-1) | CLI `send`, MCP `send`, and the spine hooks' `hook.event` | `send.client` · `mcp.tool_call` · `hook.handle` | CLI: `process-exit{exit_code:21, detail:"server-verify-failed", during:"connect"}` at WARN; hook: `hook-invoked` then `hook-decision{detail:"server-verify-failed"}` at WARN; MCP: none (as its unreachable path today, the plan's Constraints) |
| `wait` / `last` readback | CLI `wait` / `last`, MCP `wait` / `last` | `wait.block` · `last.client` · `mcp.tool_call` | CLI: the same `process-exit` |
| Dialog → `answer` | the dialog hook's `hook.dialog`; CLI and MCP `answer` | `hook.handle` · `answer.client` · `mcp.tool_call` | hook: `hook-invoked{corr:null}` and `hook-decision{corr:null, decision_emitted:false, detail:"server-verify-failed"}` at WARN; CLI: the same `process-exit` |
| Wheel (`pause` / `release`) | CLI `pause` / `release` | `pause.client` · `release.client` | CLI: the same `process-exit` |
| Budget governor | no — `hook statusline` writes no frame | unchanged | unchanged |

On every refused path no `channel.request` span opens and no `channel-request` line is written: the check runs before
the first frame, so there is no request to trace. That is the scenario's own rule for a call that never reached the
channel (the client-side `send-refused` precedent), not a skipped span.

## The witnesses — measured, not inferred

- **Spans, in process** (spans are not serialised to the role files — `with_current_span(false)`, `with_span_list(false)` —
  so a captured layer is the only reader): `request_refuses_an_unverified_server_inside_its_dispatch_span` in each of
  `src/cmd/{send,wait,last,answer,pause,release}.rs` (shared check `cmd::client::refused_inside::assert_refused_inside`)
  reads exactly `["span <verb span>"]` — the dispatch span opened, nothing else: no `channel.request` span, no line —
  and no frame at the server; `cmd::hook::tests::hook_refuses_an_unverified_server_inside_hook_handle` reads, in order,
  `span hook.handle` · `event "hook-invoked" in hook.handle` · `event "hook-decision" in none` (the event path's decision
  is written by the caller after `handle` returns, as on every fail-open path before this chunk) · `span hook.handle` ·
  `event "hook-invoked" in hook.handle` · `event "hook-decision" in hook.handle` (the dialog path). Each is red with the
  check removed (`red-before-green.md`, second reading: 7 run, 7 failed).
- **Lines, on disk** (the role files a real process wrote): `tests/security_negatives_server_identity.rs` reads one
  `process-exit{level:"WARN", subject:"self", exit_code:21, detail:"server-verify-failed", during:"connect"}` and no
  `channel-request` in `cli-builder.ndjson` for each of the six verbs in both modes, and no recorded pid, home path or
  endpoint in any output; `tests/hook_fail_open.rs`'s three impostor rows read `hook-invoked` then `hook-decision` at
  WARN with `detail:"server-verify-failed"`, both lines schema-valid against `schemas/diag-line.v1.json`.
- **MCP:** `frame` runs inside `call`, the `#[instrument(name = "mcp.tool_call")]` future (`crates/viola-mcp/src/tools.rs`),
  so the span holds by construction; the start-time read runs on `spawn_blocking` and writes no line. No span capture was
  added in `viola-mcp` (it carries no `tracing-subscriber` dev-dependency, and adding one is outside the plan's lists);
  `handlers_frame_tools_write_no_frame_to_a_server_that_is_not_the_recorded_wrapper` witnesses the result and the absent
  frame.

## Finding

The new refusal paths keep their scenario's root span and write their scenario's outcome line. One text in obs-plan §4
no longer holds and is the wrap's to amend (named in the plan's expected amendments): the `wait` / `last` scenario's
«`strict-modes-failed` / `server-verify-failed` cannot occur on `wait` / `last` until client-side server verification
lands» — `server-verify-failed` now occurs there on Linux and Windows. No span was skipped and no event dropped.
