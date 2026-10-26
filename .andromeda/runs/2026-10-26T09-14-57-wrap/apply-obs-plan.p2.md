# Apply — obs-plan · part 2 of 3

## Verdicts
- fanout-obs-plan--cascade-g2.md 1 · applied · .andromeda/obs-plan-detail/span-trace-coverage.md:18#1, .andromeda/obs-plan-detail/span-trace-coverage.md:24+#1
- fanout-obs-plan--cascade-g2.md 2 · folded into fanout-obs-plan--cascade-g2.md 1
- fanout-obs-plan--cascade-g2.md 3 · folded into fanout-obs-plan--cascade-g2.md 1
- fanout-obs-plan--cascade-g2.md 4 · folded into fanout-obs-plan--cascade-g2.md 1
- fanout-obs-plan--cascade-g2.md 5 · applied · .andromeda/obs-plan-detail/span-trace-coverage.md:59+#1
- fanout-obs-plan--cascade-g2.md 6 · applied · .andromeda/obs-plan-detail/span-trace-coverage.md:72+#1
- fanout-obs-plan--cascade-g2.md 7 · applied · .andromeda/obs-plan-detail/span-trace-coverage.md:75#1
- fanout-obs-plan--cascade-g2.md 8 · applied · .andromeda/obs-plan-detail/span-trace-coverage.md:83+#1
- fanout-obs-plan--cascade-g2.md 9 · applied · .andromeda/obs-plan-detail/span-trace-coverage.md:90#1
- fanout-obs-plan--cascade-g2.md 11 · applied · .andromeda/obs-plan-detail/span-trace-coverage.md:92#1
- fanout-obs-plan--cascade-g2.md 12 · applied · .andromeda/obs-plan-detail/span-trace-coverage.md:100+#1
- fanout-obs-plan--cascade-g2.md 13 · applied · .andromeda/obs-plan-detail/span-trace-coverage.md:108+#1
- fanout-obs-plan--cascade-g2.md 14 · applied · .andromeda/obs-plan-detail/span-trace-coverage.md:109+#1
- fanout-obs-plan--cascade-g2.md 15 · applied · .andromeda/obs-plan-detail/span-trace-coverage.md:119+#1
- fanout-obs-plan--cascade-g2.md 16 · applied · .andromeda/obs-plan-detail/span-trace-coverage.md:123+#1
- fanout-obs-plan--cascade-g2.md 17 · applied · .andromeda/obs-plan-detail/span-trace-coverage.md:167#1
- fanout-obs-plan--cascade-g2.md 18 · applied · .andromeda/obs-plan-detail/log-coverage.md:28+#1
- fanout-obs-plan--cascade-g2.md 19 · applied · .andromeda/obs-plan-detail/log-coverage.md:30+#1
- fanout-obs-plan--cascade-g2.md 20 · applied · .andromeda/obs-plan-detail/pii-scrubbing-compliance.md:36+#1
- cascade-compare-obs-plan.md 1 · applied · .andromeda/obs-plan-detail/metric-coverage.md:32+#1

## Edits
- line: .andromeda/obs-plan-detail/span-trace-coverage.md:18
  old: «A dispatch span per verb (`send.client`, `wait.block`, `answer.client`,»
  new: «A dispatch span per verb (`send.client`, `wait.block`, `last.client`, `answer.client`,»
- after: .andromeda/obs-plan-detail/span-trace-coverage.md:24
  anchor: «<!-- /detail -->»
  add: «»
  add: «<!-- detail: values · spans -->»
  add: «»
  add: «- **cli (short-lived verbs), the server check:** each of the six frame verbs (`send`, `wait`, `last`, `answer`, `pause`, `release`) verifies the wrapper through `cmd::client::connect` inside its dispatch span, after its own client-side refusals and before any frame: the connected stream's peer pid and that process's OS start time against the one snapshot read, a closed `ServerVerdict` (`Matches|Mismatch|Unverifiable`). A mismatch or a check that cannot be made opens no `channel.request` span, writes no `channel-request` line and ends `process-exit{subject:"self", exit_code:21, detail:"server-verify-failed", during:"connect"}`. Linux and Windows; macOS reports no peer pid and stays on the recorded gap.»
  add: «- The refusal's place is asserted by captured-span cases, one per CLI frame verb (`request_refuses_an_unverified_server_inside_its_dispatch_span` in each verb's module) and one for the hook (`hook_refuses_an_unverified_server_inside_hook_handle`), each through the shared test-only helper `cmd::client::refused_inside` (`assert_refused_inside`): the refusal happens inside its scenario's root span, and each case reads red with the check removed.»
  add: «- **cli (`viola hook`), the server check:** the event and dialog paths share one `connect` inside `hook.handle` (`read_snapshot`, then `connect_by`, then `verify_server`) before any frame. A server that is not the recorded wrapper, or a check that cannot be made, gets no frame: no `channel.request` opens, and the hook fails open (exit 0, empty streams) with `hook-decision{detail:"server-verify-failed"}`; a SessionEnd whose check fails takes the direct `append_session_end`. Linux and Windows; macOS on the recorded gap.»
  add: «- **ipc-internal (viola-mcp), the server check:** the frame tools `send`, `wait`, `last` and `answer` verify the wrapper after `TokioClient::connect`, inside `mcp.tool_call` (`server_verified`, the start time read on `tokio::task::spawn_blocking`). A mismatch or a check that cannot be made returns the codes-only `isError` `instance-unreachable` with no frame and no log line, as the unreachable path does. Linux and Windows; macOS on the recorded gap.»
  add: «»
  add: «<!-- /detail -->»
- after: .andromeda/obs-plan-detail/span-trace-coverage.md:59
  anchor: «- **Must-trace spans:** `send.client` (CLIENT; `cli` or `mcp.tool_call`) › `channel.request` (CLIENT) → `channel.dispatch` (SERVER, `run`) › `run.readiness_gate` › `pty.paste_write` › `run.confirm_window`. An unconfirmable send ends at `pty.paste_write` and opens no `run.confirm_window`. In the hook process: `hook.handle` › `channel.request(hook.event)`, the user-prompt-submit hook for an ordinary send and the session-start hook for a `/clear` confirmed by its new session or a `/compact` confirmed by its same session (verified CLI). One operation on this path carries no span and no line of its own, as landed (chunk 2026-10-10-session-links): the implicit link record (E3 below). After the second wheel and running-turn reads and directly before the send is issued, a `send` whose `from` is a name other than the wrapper's own records the link, so a new pair's `link` product line stands directly before that send's `send-issued` in `events.ndjson`, whose `cursor` is still the offset its own line starts at; a send refused at any earlier rung, one with no `from` and one under the wrapper's own name record nothing. A record that fails answers `-32603` with nothing issued and nothing typed, so no `send-issued` is written. `driver` and `driven` reach no process-log line.»
  add: «  - The server check runs before `channel.request`, inside `send.client` (CLI) or `mcp.tool_call` (MCP): the peer pid and its start time against the one snapshot read. A server that is not the recorded wrapper, or a check that cannot be made, opens no `channel.request`, and the path ends there (Linux and Windows; macOS on the recorded gap).»
- after: .andromeda/obs-plan-detail/span-trace-coverage.md:72
  anchor: «  - client-side validation refusal, made before any frame: `send-refused{corr:null, refusal:"not-delivered", detail:"control-character"|"empty-text", side:"client"}`, with a mirror line on the wrapper side if the frame reached it. `empty-text` is an empty typed text, read after `control-character` and ahead of both wheel reads; the wrapper, deciding it on a frame sent straight to it, writes its own `send-refused{side:"wrapper"}` line with the same pair and `corr`, `conn` and `rpc_id`, and no `send-issued`. One line per deciding side, never the text;»
  add: «  - client-side server-verification refusal, made after the client's own refusals and before any frame (Linux and Windows): no `channel-request` / `channel-response` line on either side; CLI `send` ends `process-exit{subject:"self", exit_code:21, detail:"server-verify-failed", during:"connect"}` at WARN, and MCP `send` writes no line;»
- line: .andromeda/obs-plan-detail/span-trace-coverage.md:75
  old: «or `call` (the connection lost after it).»
  new: «or `call` (the connection lost after it), or `detail:"server-verify-failed"` with `during:"connect"` (the serving process is not the recorded wrapper, or the check cannot be made; no frame sent; Linux and Windows, macOS on the recorded gap).»
- after: .andromeda/obs-plan-detail/span-trace-coverage.md:83
  anchor: «- **Must-trace spans:** `wait.block` (CLIENT) › `channel.request` → `channel.dispatch` › `run.wait_dispatch` (INTERNAL). For `last`: `last.client` › `channel.request`.»
  add: «  - The server check runs inside `wait.block` / `last.client`, after the client's own refusals and before `channel.request`; a server that is not the recorded wrapper, or a check that cannot be made, opens no `channel.request` (Linux and Windows; macOS on the recorded gap).»
- line: .andromeda/obs-plan-detail/span-trace-coverage.md:90
  old: «(endpoint vanished mid-wait = `during:"call"`). `wait` / `last`, like `send`, use the liveness-only pre-check, so `strict-modes-failed` / `server-verify-failed` cannot occur on them until client-side server verification lands (the dated gap owned by the Epoch 6 route entries "Server verification before any frame" and "Home and code-bearing file integrity").»
  new: «(endpoint vanished mid-wait = `during:"call"`), or `detail:"server-verify-failed"` with `during:"connect"` when the serving process is not the recorded wrapper or the check cannot be made (Linux and Windows; macOS on the recorded gap). `wait` / `last`, like `send`, take the liveness-only pre-check and then the server check; `strict-modes-failed` cannot occur on them, the strict-modes half owned by the route entry "Home and code-bearing file integrity".»
- line: .andromeda/obs-plan-detail/span-trace-coverage.md:92
  old: «`wait.block` closes on the response or on the connection error.»
  new: «`wait.block` closes on the response, on the connection error, or on a server-verification refusal made before any frame (no `channel.request` opened; the client's `process-exit` its outcome line).»
- after: .andromeda/obs-plan-detail/span-trace-coverage.md:100
  anchor: «- **Must-trace spans:** `hook.handle` (SERVER) › `channel.request(hook.dialog)` → `channel.dispatch` › `run.dialog_register` › `run.dialog_await` › `hook.decision_emit`. Driver side: `answer.client` › `channel.request(answer)` → `channel.dispatch`. One operation on the driver side carries no span and no line of its own, as landed (chunk 2026-10-10-session-links): the implicit link record in the wrapper's `answer` handling (E3 below). Once every refusal has passed and the response's kind matched the pending dialog, and before the `dialog-answered` line and the answer being taken, an `answer` whose `from` is a name other than the wrapper's own records the link (one `link` product line for a new pair, none for a pair the set holds); a refused answer, one with no `from` and one under the wrapper's own name record nothing. A record that fails answers `-32603`: the answer is not taken, the dialog stays held and no `dialog-answered` line is written.»
  add: «  - The server check comes before each frame of this path: the dialog hook's inside `hook.handle`, before `channel.request(hook.dialog)`, and the driver's inside `answer.client`, before `channel.request(answer)`. A server that is not the recorded wrapper, or a check that cannot be made, opens neither: the dialog hook emits no decision (fail open), and `answer` sends no frame (Linux and Windows; macOS on the recorded gap).»
- after: .andromeda/obs-plan-detail/span-trace-coverage.md:108
  anchor: «  - A dialog hook's `hook-invoked` is written only after the `hook.dialog` response, so its `invoked_at` records the hook's true start instant; non-dialog hooks and fail-open paths before `dialog_id` emit `hook-invoked{corr:null}` immediately, and the wrapper's `dialog-raised` gives live visibility during the wait (D-07).»
  add: «  - A failed server check is one of those fail-open paths before `dialog_id`: `hook-invoked{corr:null}` then `hook-decision{corr:null, decision_emitted:false, detail:"server-verify-failed"}`, with no `hook.dialog` frame and so no `dialog-raised`.»
- after: .andromeda/obs-plan-detail/span-trace-coverage.md:109
  anchor: «  - driver-side refusals: `channel-response{result_class:"refusal", refusal:"not-delivered", detail:"unknown-dialog"}` (exit 13), or `refusal:"unverified-cli"` (exit 12, answer withheld).»
  add: «  - a server-verification refusal on CLI `answer`, made before any frame: no `channel-request` / `channel-response` line, `process-exit{subject:"self", exit_code:21, detail:"server-verify-failed", during:"connect"}` at WARN; MCP `answer` writes no line.»
- after: .andromeda/obs-plan-detail/span-trace-coverage.md:119
  anchor: «- **Must-trace spans:** `run.wheel_transition` (INTERNAL) › `channel.dispatch(send)` refused; `pause.client` / `release.client` (CLIENT) › `channel.request(pause|release)` → `channel.dispatch(pause|release)`, answered only after the move's `wheel` record lands (`-32603` if it did not).»
  add: «  - `pause.client` / `release.client` run the server check before `channel.request(pause|release)`; a server that is not the recorded wrapper, or a check that cannot be made, opens no `channel.request` and sends no frame (Linux and Windows; macOS on the recorded gap).»
- after: .andromeda/obs-plan-detail/span-trace-coverage.md:123
  anchor: «  - `channel-request` / `channel-response{method:"release"}`;»
  add: «  - a server-verification refusal on `pause` / `release`, made before any frame: no `channel-request` / `channel-response` line, the client's `process-exit{subject:"self", exit_code:21, detail:"server-verify-failed", during:"connect"}` at WARN;»
- line: .andromeda/obs-plan-detail/span-trace-coverage.md:167
  old: «The five other snapshot readers (`src/cmd/client.rs`, `run`'s collision check, `src/cmd/hook.rs` twice, `src/cmd/hook/statusline.rs`; the rule: `read_snapshot(` under `src/`, outside test modules)»
  new: «The three other snapshot readers (`run`'s collision check in `src/cmd/run.rs`, `src/cmd/hook.rs` once, its event and dialog paths sharing the one read, and `src/cmd/hook/statusline.rs`; the rule: `read_snapshot(` under `src/`, outside test modules, as measured at viola-0.1.0/chunks/2026-10-26-server-verification-before-any-frame/report.md §Counts; the CLI frame verbs and `viola mcp` read through `viola_state::liveness::live_endpoint`, outside the rule's `src/`)»
- after: .andromeda/obs-plan-detail/log-coverage.md:28
  anchor: «- `process-exit` exit 21: `instance-dead`, `strict-modes-failed`, `server-verify-failed`.»
  add: « `server-verify-failed` is written by the six CLI frame verbs on a server that is not the recorded wrapper, or a check that cannot be made (Linux and Windows): `process-exit{subject:"self", exit_code:21, detail:"server-verify-failed", during:"connect"}` at WARN into `cli-<name>.ndjson`, with no pid, start time, path or endpoint.»
- after: .andromeda/obs-plan-detail/log-coverage.md:30
  anchor: «- `hook-decision` fail-open codes (security Vector 4): `oversize-stdin`, `malformed-json`, `channel-unreachable`, `server-verify-failed`, `strict-modes-failed`, `unverified-cli`, `deadline`. `viola hook statusline` uses three of the seven (the rule: the `detail` codes the arm's fail-open paths write): `oversize-stdin`, `malformed-json`, and `deadline`, which on that arm is the 5 s PROVISIONAL bound on the user's command, not a dialog deadline. Its refused instance check writes no `hook-decision` and no other line (chunk 2026-10-10-budget-governor; before it the arm wrote `strict-modes-failed`, four of the seven).»
  add: « `server-verify-failed` is written by `viola hook` on a server that is not the recorded wrapper, or a check that cannot be made (Linux and Windows): `hook-decision{detail:"server-verify-failed"}` at WARN into `hook-<name>.ndjson`; on the dialog path the order is `hook-invoked{corr:null}` then `hook-decision{corr:null, decision_emitted:false, detail:"server-verify-failed"}`.»
- after: .andromeda/obs-plan-detail/pii-scrubbing-compliance.md:36
  anchor: «   - The local `viola-harness pre-push` document is never uploaded and carries no absolute path, only codes and counts. The gate runs natively on the Linux host in the working tree itself (no clone), so neither the repository root nor the passwd home (`<home>`, `<home>/.cargo/bin`) reaches it. `pre_push_document_carries_no_absolute_path` asserts it, red at the Linux tests and green (chunks 2026-09-26-local-linux-pre-push-gate and 2026-10-03-mutation-scoring-completion).»
  add: «   - A server-verification refusal carries no pid, start time, path or endpoint: `tests/security_negatives_server_identity.rs` scans every output and the `cli-<name>.ndjson` line with `digit_runs` (whole numbers only, itself tested by `digit_runs_reads_whole_numbers_only`), on Linux and Windows (the file compiles out on macOS).»
- after: .andromeda/obs-plan-detail/metric-coverage.md:32
  anchor: «<!-- /detail -->»
  add: «»
  add: «<!-- detail: values · metrics -->»
  add: «»
  add: «- `viola.wait.outcome` counts only the `wait` calls answered by a `channel-response`: a `wait` refused before any frame writes none. The CLI's server-verification refusal (Linux and Windows) is one `process-exit{subject:"self", exit_code:21, detail:"server-verify-failed", during:"connect"}` at WARN in `cli-<name>.ndjson`, counted by the Counter row's `detail` grouping; MCP's `wait` tool writes no line for it.»
  add: «»
  add: «<!-- /detail -->»

## Patterns
[[pattern]]
id = "obs2-01"
fixed = 'server-verify-failed` cannot occur'

[[pattern]]
id = "obs2-02"
fixed = 'five other snapshot readers'

[[pattern]]
id = "obs2-03"
fixed = '`src/cmd/hook.rs` twice'
