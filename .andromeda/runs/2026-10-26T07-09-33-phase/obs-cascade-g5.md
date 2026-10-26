# obs · cascade read · group 5 of 7

## Carried
- (.andromeda/obs-plan-detail/span-trace-coverage.md:8) The server verification must run inside these CLIENT-kind verb spans and before the client-side `channel.request` opens, and any new client-side verification span is recorded with its kind.
  > - `CLIENT`: `send.client`, `wait.block`, `last.client`, `answer.client`, `pause.client`, `release.client`, `channel.request` (client side), `statusline.shell_out`, `pty.spawn`.
  (.andromeda/obs-plan.md:329)
  > **Span kinds.** Recorded in this plan per span.
- (.andromeda/obs-plan-detail/span-trace-coverage.md:9) The MCP tools' and the hook's verification must run inside `mcp.tool_call` and `hook.handle`, the SERVER-kind spans those edited clients open.
  > - `SERVER`: `channel.dispatch` (wrapper side of every `channel.request`), `mcp.tool_call`, `hook.handle`, `hook.statusline`.
- (.andromeda/obs-plan-detail/span-trace-coverage.md:10) A verification span added in `viola-state` (beside `live_endpoint`) must be a `state.*` INTERNAL span, and the hook's fail-open emit stays `hook.decision_emit`.
  > - `INTERNAL`: `run.*`, `state.*`, `hook.decision_emit`, `pty.paste_write`.
- (.andromeda/obs-plan-detail/span-trace-coverage.md:18) Each edited CLI verb must keep its dispatch span and write its mismatch outcome as `process-exit{exit_code, detail}` into `cli-<name>.ndjson`, with any typed error's chain only in `detail-cli.ndjson`.
  > | cli (short-lived verbs) | none (clap 4.6.7 has no tracing integration) | A dispatch span per verb (`send.client`, `wait.block`, `answer.client`, `pause.client`, `release.client`, `cli.<verb>`). `process-start` / `process-exit{exit_code, detail}` into `cli-<name>.ndjson` when an instance resolves (D-06);
  (.andromeda/obs-plan-detail/span-trace-coverage.md:18)
  > Client-side `send-refused{side:"client"}`. The full anyhow chain goes to `detail-cli.ndjson` only |
- (.andromeda/obs-plan-detail/span-trace-coverage.md:20) The hook's verification must sit inside `hook.handle` before its `channel.request`, and its mismatch path must keep exit 0 under the `catch_unwind` and the sink fallback.
  > | cli (`viola hook`) | none | `hook.handle` › `channel.request` › `hook.decision_emit`. `hook-invoked` / `hook-decision` (except the hidden capture arms, `hook <event> --capture` and `hook statusline --capture`: no obs init, no lines, §4 Edge flows). `std::panic::catch_unwind` around dispatch keeps exit 0. The writer falls back to `std::io::sink` on open failure |
- (.andromeda/obs-plan-detail/span-trace-coverage.md:21) The sync and Tokio clients' edited connect must keep the client `channel.request` span and its `channel-request` / `channel-response` fields, never `params` or bodies, and interprocess emits nothing of its own, so a failed peer-credential read is logged at viola's seam.
  > | ipc-internal (viola-channel) | none (interprocess 2.4.4 emits no `log`/`tracing`) | `channel.request` (client) / `channel.dispatch` (server) spans. `channel-request` / `channel-response` with `method`, `corr`, `conn`, `result_class`, `error_code`, `sender`. Never `params` or result bodies |
  (.andromeda/obs-plan.md:333)
  > Every span is manual — no surface has an auto-instrumentation library viola uses, and the rmcp and portable-pty targets are OFF in the sink; channel lines never record `params` or result bodies.
  (.andromeda/obs-plan.md:337)
  > `trace_id` / `span_id` are **not** emitted (D-12); correlation is `corr` + `conn`.
- (.andromeda/obs-plan-detail/span-trace-coverage.md:22) The MCP tools' `instance-unreachable` on a mismatch must stay inside `mcp.tool_call` and be mirrored by a codes-only line, with rmcp's own target still OFF.
  > | ipc-internal (viola-mcp) | rmcp 3.4.1 emits native `tracing`, but its target is OFF in the sink (records lack required fields and can carry frame content) | `mcp.tool_call` span per tool (`send · wait · last · answer · list`). Codes-only `send-refused` / `channel-response` on rejects. The writer is always explicit (file; stderr only as a fallback) |
- (.andromeda/obs-plan-detail/span-trace-coverage.md:33) `run`'s collision check, whose `read_snapshot` call the chunk re-reads, must keep this outcome mapping, its unreadable-or-newer snapshot still reading as absent under the founder's hold.
  > - `run.collision_check`: `outcome` (`free|already-live|squatted-name`) — a `live` or `stale` holder is `already-live`; a `gone` one is `free` (taken over, its `events.ndjson` appended to);
  (.andromeda/obs-plan-detail/span-trace-coverage.md:45)
  > On failure, `process-exit{subject:"self", exit_code:1, detail}` with `detail` ∈ `already-live|squatted-name|pinned-hash-mismatch|batch-script-child` (Founder Direction 4; no path or pid).
- (.andromeda/obs-plan-detail/span-trace-coverage.md:59) The confirmed send's client spans (cli or `mcp.tool_call`) and the hook's `channel.request(hook.event)` are the calls the verification must precede, and a refused verification opens no `channel.request`.
  > - **Must-trace spans:** `send.client` (CLIENT; `cli` or `mcp.tool_call`) › `channel.request` (CLIENT) → `channel.dispatch` (SERVER, `run`) › `run.readiness_gate` › `pty.paste_write` › `run.confirm_window`. An unconfirmable send ends at `pty.paste_write` and opens no `run.confirm_window`. In the hook process: `hook.handle` › `channel.request(hook.event)`, the user-prompt-submit hook for an ordinary send and the session-start hook for a `/clear` confirmed by its new session or a `/compact` confirmed by its same session (verified CLI).
  (.andromeda/obs-plan.md:351)
  > - **Surfaces involved:** cli (`send`) or ipc-internal (mcp) → ipc-internal (channel) → cli (`run`, PTY paste) → cli (`hook` user-prompt-submit;
- (.andromeda/obs-plan-detail/span-trace-coverage.md:60) The client `channel.request` attributes must be recorded only on a call the verification has let through.
  > - **Required span attributes:**
  >   - `channel.request` / `channel.dispatch`: `method="send"`, `conn`;
- (.andromeda/obs-plan-detail/span-trace-coverage.md:67) A send refused by a failed verification must write no `channel-request` line (no frame was written), and a call that is made keeps `corr` + `conn`.
  > - `channel-request{corr:<id>, conn, method, from, from_trust:"self-reported", sender}`;
  (.andromeda/obs-plan-detail/span-trace-coverage.md:72)
  > - client-side validation refusal, made before any frame: `send-refused{corr:null, refusal:"not-delivered", detail:"control-character"|"empty-text", side:"client"}`, with a mirror line on the wrapper side if the frame reached it.
  (.andromeda/obs-plan-detail/span-trace-coverage.md:73)
  > - `channel-response{corr:<id>, conn, result_class, refusal, detail, duration_ms}` on both sides;
- (.andromeda/obs-plan-detail/span-trace-coverage.md:75) `send.client`'s exit-21 close is where the mismatch outcome lands: its new closed `detail` (`server-verify-failed`, path-, pid- and start-time-free) must sit beside `instance-dead` and its `during` reading.
  > `send.client` closes with `process-exit{subject:"self", exit_code}` (0 / 10 / 11 / 12 / 13 / 14 / 20 / 21, or 1 `internal-error` on a reply with neither a parseable `result` nor an `error.code`); exit 20 carries `detail:"wrapper-fault"`, and exit 21 `detail:"instance-dead"` with `during` `connect` (no live snapshot, or the connect failed) or `call` (the connection lost after it).
- (.andromeda/obs-plan-detail/span-trace-coverage.md:83) The `wait` / `last` verification must sit inside `wait.block` / `last.client` before their `channel.request`.
  > - **Must-trace spans:** `wait.block` (CLIENT) › `channel.request` → `channel.dispatch` › `run.wait_dispatch` (INTERNAL). For `last`: `last.client` › `channel.request`.
- (.andromeda/obs-plan-detail/span-trace-coverage.md:90) This chunk is the entry the note names, so the `wait` / `last` exit-21 line must gain `server-verify-failed` (and `strict-modes-failed` only if P4 takes the strict-modes half) and the "cannot occur until" note must be amended.
  > client `process-exit{subject:"self", exit_code, detail}` at WARN, where exit 21 carries `detail:"instance-dead"` plus `during` ∈ `connect|call` (endpoint vanished mid-wait = `during:"call"`). `wait` / `last`, like `send`, use the liveness-only pre-check, so `strict-modes-failed` / `server-verify-failed` cannot occur on them until client-side server verification lands (the dated gap owned by the Epoch 6 route entries "Server verification before any frame" and "Home and code-bearing file integrity").
  (.andromeda/obs-plan.md:361)
  > `strict-modes-failed` / `server-verify-failed` cannot occur on `wait` / `last` until client-side server verification lands, owned by the route entries "Server verification before any frame" and "Home and code-bearing file integrity".
- (.andromeda/obs-plan-detail/span-trace-coverage.md:92) `wait.block` must also close on a refused verification, and that refusal is logged once, by the client.
  > - **Cleanup:** `wait.block` closes on the response or on the connection error. A mid-call disconnect is logged once, by the client.
- (.andromeda/obs-plan-detail/span-trace-coverage.md:100) The `hook.dialog` client and `answer.client` must verify before their `channel.request`, a refused `hook.dialog` verification ending the hook's path before any dispatch.
  > - **Must-trace spans:** `hook.handle` (SERVER) › `channel.request(hook.dialog)` → `channel.dispatch` › `run.dialog_register` › `run.dialog_await` › `hook.decision_emit`. Driver side: `answer.client` › `channel.request(answer)` → `channel.dispatch`.
  (.andromeda/obs-plan.md:367)
  > - **Surfaces involved:** cli (`hook` pre-tool-use / permission-request) → ipc-internal (`hook.dialog`) → cli (`run`) ← cli or ipc-internal (mcp) `answer`.
- (.andromeda/obs-plan-detail/span-trace-coverage.md:104) The hook's fail-open on a mismatch must record `decision_emitted` false on `hook.decision_emit`, with no decision body.
  > - `hook.decision_emit`: `decision_emitted`.
- (.andromeda/obs-plan-detail/span-trace-coverage.md:107) A `hook.dialog` refused by verification never gets a response or a `dialog_id`, so its `hook-invoked` must be the immediate `corr:null` line of a fail-open path.
  > - hook: `hook-invoked{corr:<dialog_id>, hook_event, stdin_bytes, invoked_at}`, emitted once `dialog_id` is known (D-07), then `hook-decision{corr:<dialog_id>, hook_event, decision_emitted, deadline_hit, detail, duration_ms}`;
  (.andromeda/obs-plan-detail/span-trace-coverage.md:108)
  > - A dialog hook's `hook-invoked` is written only after the `hook.dialog` response, so its `invoked_at` records the hook's true start instant; non-dialog hooks and fail-open paths before `dialog_id` emit `hook-invoked{corr:null}` immediately, and the wrapper's `dialog-raised` gives live visibility during the wait (D-07).
  (.andromeda/obs-plan.md:369)
  > a dialog hook's `hook-invoked` is written only after the `hook.dialog` response.
- (.andromeda/obs-plan-detail/span-trace-coverage.md:111) The hook's verification mismatch is a fail-open before `dialog_id`, so it must write this pair with a closed fail-open `detail` from the Section 6 codes.
  > Fail-open before `dialog_id` gives `hook-invoked{corr:null}` + `hook-decision{corr:null, detail}` (Section 6 fail-open codes).
- (.andromeda/obs-plan-detail/span-trace-coverage.md:119) The `pause` / `release` verification must sit inside `pause.client` / `release.client` before their `channel.request`.
  > `pause.client` / `release.client` (CLIENT) › `channel.request(pause|release)` → `channel.dispatch(pause|release)`, answered only after the move's `wheel` record lands (`-32603` if it did not).
- (.andromeda/obs-plan-detail/span-trace-coverage.md:123) A `pause` / `release` refused by verification must write no `channel-request` / `channel-response` of its own call.
  > - `channel-request` / `channel-response{method:"release"}`;
- (.andromeda/obs-plan-detail/span-trace-coverage.md:166) The five readers the chunk re-reads must stay on `read_snapshot` and write no `state-recovered` line unless the founder's live word moves them to the replay.
  > - **E5** (snapshot corruption): `state-recovered{detail:"snapshot-replayed"|"snapshot-unsupported-v", file:"snapshot.json", v_seen}` (D-03). As landed (chunk 2026-10-10-self-healing-state) the line is written by `viola_state::replay::read_snapshot_or_replay`, at WARN with no `corr`: `snapshot-replayed` for an unreadable snapshot, `snapshot-unsupported-v` with `v_seen` for a newer one, one line per call;
  (.andromeda/obs-plan-detail/span-trace-coverage.md:167)
  > The five other snapshot readers (`src/cmd/client.rs`, `run`'s collision check, `src/cmd/hook.rs` twice, `src/cmd/hook/statusline.rs`; the rule: `read_snapshot(` under `src/`, outside test modules) still call `read_snapshot` and write no `state-recovered` line.
  (.andromeda/obs-plan.md:394)
  > the other snapshot readers write no `state-recovered` line.

## Bindings
- **span-trace-coverage (exit-21 close) ↔ log-coverage**: the new `server-verify-failed` `detail` on `send.client` / `wait.block` / the other verbs' exit-21 close (.andromeda/obs-plan-detail/span-trace-coverage.md:75) (.andromeda/obs-plan-detail/span-trace-coverage.md:90) is written at WARN by the level mapping (.andromeda/obs-plan.md:460) and must be added to the closed catalog and the `diag-line.v1.json` schema.
- **span-trace-coverage (hook fail-open) ↔ log-coverage and the hook's stderr rule**: the `hook-invoked{corr:null}` + `hook-decision{corr:null, detail}` pair (.andromeda/obs-plan-detail/span-trace-coverage.md:111) carries a fail-open `detail` at WARN (.andromeda/obs-plan.md:460), logged to `hook-<name>.ndjson` only with exit 0 and stderr empty (.andromeda/obs-plan.md:200).
- **span-trace-coverage (viola-channel / viola-mcp rows) ↔ obs-anti-patterns (spans)**: the Tokio client's verification inside `mcp.tool_call` (.andromeda/obs-plan-detail/span-trace-coverage.md:22) must pass `corr` / `conn` / `instance` as explicit values, never through span context across tasks (.andromeda/obs-plan.md:694).
- **span-trace-coverage (hook path) ↔ slo-invariants-telemetry-budgets**: the peer-credential and start-time read added inside `hook.handle` (.andromeda/obs-plan-detail/span-trace-coverage.md:20) stays inside the hooks' `max` budgets gated by `gate --require perf` (.andromeda/obs-plan.md:649).
- **span-trace-coverage (E5) ↔ the founder's hold on the self-healing CARRY**: the five readers' `read_snapshot` with no `state-recovered` line (.andromeda/obs-plan-detail/span-trace-coverage.md:167) is the outcome the hold keeps; a move to `read_snapshot_or_replay` changes this line and is his to rule.
