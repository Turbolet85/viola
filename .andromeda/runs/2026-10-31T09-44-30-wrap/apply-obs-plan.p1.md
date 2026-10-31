# Apply — obs-plan · part 1 of 2

## Verdicts
- fanout-obs-plan--cascade-g3.md 2 · applied · .andromeda/obs-plan.md:418#1
- fanout-obs-plan.md 1 · folded into fanout-obs-plan--cascade-g3.md 2
- fanout-obs-plan--cascade-g3.md 7 · applied · .andromeda/obs-plan.md:527#1
- fanout-obs-plan.md 3 · folded into fanout-obs-plan--cascade-g3.md 7
- fanout-obs-plan--cascade-g3.md 8 · applied · .andromeda/obs-plan.md:496#1
- fanout-obs-plan.md 2 · folded into fanout-obs-plan--cascade-g3.md 8
- fanout-obs-plan--cascade-g1.md 1 · applied · .andromeda/obs-plan-detail/obs-scope-summary.md:310#1
- fanout-obs-plan--cascade-g1.md 2 · applied · .andromeda/obs-plan-detail/obs-scope-summary.md:336#1
- fanout-obs-plan--cascade-g1.md 3 · applied · .andromeda/obs-plan-detail/obs-scope-summary.md:354#1
- cascade-compare-obs-plan.md 1 · applied · .andromeda/obs-plan-detail/obs-scope-summary.md:499#1
- cascade-compare-obs-plan.md 2 · applied · .andromeda/obs-plan-detail/obs-scope-summary.md:215+#1
- cascade-compare-obs-plan.md 3 · applied · .andromeda/obs-plan-detail/obs-scope-summary.md:212#1
- fanout-obs-plan--cascade-g1.md 4 · applied · .andromeda/obs-plan-detail/telemetry-strategy.md:12#1
- fanout-obs-plan--cascade-g1.md 5 · applied · .andromeda/obs-plan-detail/telemetry-strategy.md:11#1
- fanout-obs-plan--cascade-g2.md 1 · applied · .andromeda/obs-plan-detail/span-trace-coverage.md:21#1, .andromeda/obs-plan-detail/span-trace-coverage.md:31+#1
- fanout-obs-plan--cascade-g2.md 2 · applied · .andromeda/obs-plan-detail/span-trace-coverage.md:91#1
- fanout-obs-plan--cascade-g2.md 3 · applied · .andromeda/obs-plan-detail/span-trace-coverage.md:132#1
- fanout-obs-plan--cascade-g2.md 4 · applied · .andromeda/obs-plan-detail/span-trace-coverage.md:151#1
- fanout-obs-plan--cascade-g2.md 5 · applied · .andromeda/obs-plan-detail/span-trace-coverage.md:194#1
- cascade-compare-obs-plan.md 6 · applied · .andromeda/obs-plan-detail/span-trace-coverage.md:32+#1
- fanout-obs-plan--cascade-g3.md 1 · applied · .andromeda/obs-plan-detail/metric-coverage.md:28#1
- fanout-obs-plan--cascade-g3.md 3 · applied · .andromeda/obs-plan-detail/log-coverage.md:13#1

## Edits
- line: .andromeda/obs-plan.md:418
  old: «over-counts: `-32602` also answers `send`'s invalid params, `release` carrying `from` and a bad `link` / `unlink` name or pair, until the line tells the causes apart.»
  new: «counts version rejections alone: it reads `channel-response{error_code:-32602, detail:"unsupported-version"}`, and a `-32602` for invalid params on `send`, `release` carrying `from` or a bad `link` / `unlink` name or pair carries another `detail` word.»
- line: .andromeda/obs-plan.md:527
  old: «The process writes `process-exit{subject:"self", exit_code:1, detail:"internal-error"}` and then exits 1.»
  new: «The process writes no `process-exit` line (`obs::internal_error_exit_line` covers `Run` and `Cli` only) and exits 1.»
- line: .andromeda/obs-plan.md:496
  old: «thiserror 2.0.20 on `ChannelError`, `StateError`, `AgentError`, `McpError` and `CoreError`.»
  new: «thiserror 2.0.20 on `AgentError`, `ChannelError`, `StateError`, `PinError`, `StampsError`, `StampsReadError`, `Refused`, `McpError` and `ServeError`.»
- line: .andromeda/obs-plan-detail/obs-scope-summary.md:310
  old: «`empty-text` / `turn-running`), `duration_ms`.»
  new: «`empty-text` / `turn-running`; on a `channel-response` answering `-32602`, `invalid-params` or `unsupported-version`), `duration_ms`.»
- line: .andromeda/obs-plan-detail/obs-scope-summary.md:336
  old: «admitted by `schemas/diag-line.v1.json`.»
  new: «admitted by `schemas/diag-line.v1.json`; the same word is also a `detail` value of `channel-response`, on the `-32602` that answers a `release` carrying `from`, and does not replace the event.»
- line: .andromeda/obs-plan-detail/obs-scope-summary.md:354
  old: «result class (`ok` / `refusal` / `error` code) and `sender` version only, never `params` or result bodies.»
  new: «result class (`ok` / `refusal` / `error` code), `sender` version and, on a `channel-response` answering `-32602`, the closed `detail` word (`unsupported-version` / `invalid-params` / `release-from-driver`) only, never `params` or result bodies.»
- line: .andromeda/obs-plan-detail/obs-scope-summary.md:499
  old: «(-32602; the code also carries `send`'s invalid params, `release` carrying `from`, and a `link` / `unlink` whose `driver` or `driven` is missing, not a string or no valid name, whose `driven` is not the serving wrapper's own name, or whose `driver` equals `driven`, so it is not a higher-`v` count alone).»
  new: «(`channel-response{error_code:-32602, detail:"unsupported-version"}`; the code also carries `detail:"invalid-params"` (`send`'s invalid params, and a `link` / `unlink` whose `driver` or `driven` is missing, not a string or no valid name, whose `driven` is not the serving wrapper's own name, or whose `driver` equals `driven`) and `detail:"release-from-driver"` (`release` carrying `from`), so the count reads `detail` and not the code).»
- after: .andromeda/obs-plan-detail/obs-scope-summary.md:215
  anchor: «  - `isError` results carry codes only: `refusal` / `detail`, `instance-unreachable`, `wrapper-fault`, `invalid-params` (its `detail` a closed field word; chunk 2026-10-17-mcp-server-for-drivers) (security Vector 5). Log lines follow the same codes-only rule for rejected calls (tests excerpt, Coverage Triggers, MCP).»
  add: «  - A peer's reply is screened before it reaches a result (`of_reply`): a `refusal` that is no code word (`viola_core::is_code_word`) reads `unknown`, a string `detail` that is no code word reads `null`, and `wrapper-fault`'s `detail` is the closed object `viola_channel::sanitised_fault` reads (the fixed `message` of the five codes, `data` null or closed values; a code outside the five reads `message` `wrapper fault`, `data` null). A code word outside the closed list stays visible, its exit `unknown`'s 14.»
- line: .andromeda/obs-plan-detail/obs-scope-summary.md:212
  old: «its one record the §6 `parse-rejected{parser:"mcp-stdin", detail:"oversize"}` line.»
  new: «its one record the §6 `parse-rejected{parser:"mcp-stdin", detail:"oversize"}` line. Such a line before `initialize` exits the process 1 with both streams empty and that same one record; 11 is the unit child's own code, not `viola mcp`'s exit.»
- line: .andromeda/obs-plan-detail/telemetry-strategy.md:12
  old: «distinct `detail` codes on `process-exit` |»
  new: «distinct `detail` codes on `process-exit` (the `mcp` role writes no self `process-exit`, Section 7) |»
- line: .andromeda/obs-plan-detail/telemetry-strategy.md:11
  old: «fault/heal and process lifecycle, as the closed `event` enum (Section 3)»
  new: «fault/heal and process lifecycle (the `mcp` role writes no self `process-exit`, Section 7), as the closed `event` enum (Section 3)»
- line: .andromeda/obs-plan-detail/span-trace-coverage.md:21
  old: «`result_class`, `error_code`, `sender`. Never `params` or result bodies»
  new: «`result_class`, `error_code`, `sender` and, on a `-32602`, `detail` (below). Never `params` or result bodies»
- after: .andromeda/obs-plan-detail/span-trace-coverage.md:31
  anchor: «- **cli (`viola hook`), the server check:** the event and dialog paths share one `connect` inside `hook.handle` (`read_snapshot_or_replay`, its span `state.snapshot_recover` under `hook.handle`, then `connect_by`, then `verify_server`) before any frame. Only a snapshot read whole goes on: a replayed snapshot (it carries no `endpoint`, `pid` or `started_at`) or an absent one is unreachable, so no frame is sent and the hook fails open with `hook-decision{detail:"channel-unreachable"}`, a SessionEnd taking the direct `append_session_end`. A server that is not the recorded wrapper, or a check that cannot be made, gets no frame: no `channel.request` opens, and the hook fails open (exit 0, empty streams) with `hook-decision{detail:"server-verify-failed"}`; a SessionEnd whose check fails takes the direct `append_session_end`. Linux and Windows; macOS on the recorded gap.»
  add: «- **ipc-internal (viola-channel), `channel-response`'s `detail`:** on a `-32602` the line carries the closed word of §6, written on both the server's and the client's line by `ResponseLine`'s `Drop` from `ProtocolError::cause` (`None` for the other four faults, so their lines carry none); the client's `fault_of` reads the reply's `data` (`supported`, `reason` or neither) to tell which fault it holds.»
- line: .andromeda/obs-plan-detail/span-trace-coverage.md:91
  old: «`channel-response{corr:<id>, conn, result_class, refusal, detail, duration_ms}` on both sides;»
  new: «`channel-response{corr:<id>, conn, result_class, refusal, duration_ms}` on both sides, plus `detail` on a `-32602` only (a word of the closed `unsupported-version|invalid-params|release-from-driver`), absent on every other `channel-response` line, a refusal's included, whose own `detail` stays on `send-refused`;»
- line: .andromeda/obs-plan-detail/span-trace-coverage.md:132
  old: «`channel-response{result_class:"refusal", refusal:"not-delivered", detail:"unknown-dialog"}` (exit 13),»
  new: «`channel-response{result_class:"refusal", refusal:"not-delivered"}` with no `detail` on the line (exit 13; `unknown-dialog` is the refusal's own detail, no word of the line's closed three),»
- line: .andromeda/obs-plan-detail/span-trace-coverage.md:151
  old: «reply with `data: {"reason":"release-from-driver"}`; a `from` of another type, or a non-bool `budget`, is a plain `-32602` (`data: null`) with no such line, and a plain `release`»
  new: «reply with `data: {"reason":"release-from-driver"}`, whose `channel-response` carries `detail:"release-from-driver"` on both sides at WARN; a `from` of another type, or a non-bool `budget`, is a plain `-32602` (`data: null`) with no such line, its `channel-response` carrying `detail:"invalid-params"` on both sides at WARN, and a plain `release`»
- line: .andromeda/obs-plan-detail/span-trace-coverage.md:194
  old: «which holds neither the `driver` nor the `driven` name, then the product line»
  new: «which holds neither the `driver` nor the `driven` name nor any param (on a `-32602` the `channel-response` also carries `detail:"invalid-params"`), then the product line»
- after: .andromeda/obs-plan-detail/span-trace-coverage.md:32
  anchor: «- **ipc-internal (viola-mcp), the server check:** the frame tools `send`, `wait`, `last` and `answer` first run `check_instance` on their instance, before `live_endpoint` (a refusal returns `instance-unreachable` with `detail: null`, no frame and no line), then verify the wrapper after `TokioClient::connect`, inside `mcp.tool_call` (`server_verified`, the start time read on `tokio::task::spawn_blocking`). A mismatch or a check that cannot be made returns the codes-only `isError` `instance-unreachable` with no frame and no log line, as the unreachable path does. Linux and Windows; macOS on the recorded gap.»
  add: «- **The clients, a peer reply:** CLI `reply_of` and MCP `of_reply` screen a reply through `viola_core::is_code_word` and `viola_channel::sanitised_fault`, adding no span and no line.»
- line: .andromeda/obs-plan-detail/metric-coverage.md:28
  old: «`viola.compat.v_rejected` ← `channel-response{error_code:-32602}` — over-counts: `-32602` also answers `send`'s invalid params (`ProtocolError::InvalidParams`, `"invalid params"`), `release` carrying `from` and a `link` / `unlink` with a missing, non-string or invalid name, a `driven` that is not the serving wrapper or a `driver` equal to `driven`, and the client's `fault_of` labels any `-32602` unsupported-version, until the line tells the causes apart»
  new: «`viola.compat.v_rejected` ← `channel-response{error_code:-32602, detail:"unsupported-version"}`, version rejections alone: invalid params on `send` and a bad `link` / `unlink` name or pair read `detail` `invalid-params`, and `release` carrying `from` reads `release-from-driver`»
- line: .andromeda/obs-plan-detail/log-coverage.md:13
  old: «`refusal`, `detail`, `error_code` (`-32700|-32600|-32601|-32602|-32603`)»
  new: «`refusal`, `detail` (`unsupported-version` / `invalid-params` / `release-from-driver`, closed by `schemas/diag-line.v1.json`: the three `-32602` faults, on the server line and the client line alike, and absent on every other `channel-response` line), `error_code` (`-32700|-32600|-32601|-32602|-32603`)»

## Patterns
[[pattern]]
id = "obs1-01"
fixed = 'over-counts: `-32602` also answers'

[[pattern]]
id = "obs1-02"
fixed = 'writes `process-exit{subject:"self", exit_code:1, detail:"internal-error"}` and then exits 1'

[[pattern]]
id = "obs1-03"
fixed = '`McpError` and `CoreError`'

[[pattern]]
id = "obs1-04"
fixed = 'so it is not a higher-`v` count alone'

[[pattern]]
id = "obs1-05"
fixed = '`fault_of` labels any `-32602` unsupported-version'

[[pattern]]
id = "obs1-06"
fixed = 'refusal:"not-delivered", detail:"unknown-dialog"'

[[pattern]]
id = "obs1-07"
fixed = 'result_class, refusal, detail, duration_ms}'

[[pattern]]
id = "obs1-08"
fixed = 'and `sender` version only'
