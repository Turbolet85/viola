# obs · Span / Trace Coverage — category read

## Carried
- (.andromeda/obs-plan-detail/span-trace-coverage.md:8) Every client this chunk verifies runs under one of these CLIENT spans; the server check must sit inside the existing client span (before its `channel.request`), adding no unlisted span kind or name.
  > - `CLIENT`: `send.client`, `wait.block`, `last.client`, `answer.client`, `pause.client`, `release.client`, `channel.request` (client side), `statusline.shell_out`, `pty.spawn`.
- (.andromeda/obs-plan-detail/span-trace-coverage.md:18) The CLI verbs' verification refusal must close under the verb's dispatch span with the `process-exit{exit_code, detail}` line into `cli-<name>.ndjson` when an instance resolves.
  > A dispatch span per verb (`send.client`, `wait.block`, `answer.client`, `pause.client`, `release.client`, `cli.<verb>`). `process-start` / `process-exit{exit_code, detail}` into `cli-<name>.ndjson` when an instance resolves (D-06);
- (.andromeda/obs-plan-detail/span-trace-coverage.md:20) The hook client's verification must sit inside `hook.handle` ahead of `channel.request`, keep the `hook-invoked` / `hook-decision` pair, stay inside the `catch_unwind` that keeps exit 0, and leave the capture arms uninstrumented.
  > `hook.handle` › `channel.request` › `hook.decision_emit`. `hook-invoked` / `hook-decision` (except the hidden capture arms, `hook <event> --capture` and `hook statusline --capture`: no obs init, no lines, §4 Edge flows). `std::panic::catch_unwind` around dispatch keeps exit 0. The writer falls back to `std::io::sink` on open failure
- (.andromeda/obs-plan-detail/span-trace-coverage.md:21) A verification step added in or beside `viola-channel`'s client must keep the `channel.request` span and the `channel-request` / `channel-response` field set, never recording `params` or result bodies.
  > `channel.request` (client) / `channel.dispatch` (server) spans. `channel-request` / `channel-response` with `method`, `corr`, `conn`, `result_class`, `error_code`, `sender`. Never `params` or result bodies
- (.andromeda/obs-plan-detail/span-trace-coverage.md:22) The MCP frame tools' verification refusal must stay under `mcp.tool_call` and be logged codes-only, the writer explicit.
  > `mcp.tool_call` span per tool (`send · wait · last · answer · list`). Codes-only `send-refused` / `channel-response` on rejects. The writer is always explicit (file; stderr only as a fallback)
- (.andromeda/obs-plan-detail/span-trace-coverage.md:59) The must-trace chain for `send` (CLI or MCP) and for `hook.event` keeps its order: the server check comes before the client's `channel.request`, so a refused check opens no `channel.request` and no wrapper span.
  > **Must-trace spans:** `send.client` (CLIENT; `cli` or `mcp.tool_call`) › `channel.request` (CLIENT) → `channel.dispatch` (SERVER, `run`) › `run.readiness_gate` › `pty.paste_write` › `run.confirm_window`. An unconfirmable send ends at `pty.paste_write` and opens no `run.confirm_window`. In the hook process: `hook.handle` › `channel.request(hook.event)`
- (.andromeda/obs-plan-detail/span-trace-coverage.md:61) The `channel.request` span attributes stay `method` and `conn`; a verification outcome adds no field to them unless the catalog names it.
  > `channel.request` / `channel.dispatch`: `method="send"`, `conn`;
- (.andromeda/obs-plan-detail/span-trace-coverage.md:75) `send.client`'s closing `process-exit` exit-code set and the exit 21 `detail` / `during` shape are what a mismatch or an unmakeable check must map onto (exit 21), with a detail distinct from `instance-dead` where the cause differs.
  > `send.client` closes with `process-exit{subject:"self", exit_code}` (0 / 10 / 11 / 12 / 13 / 14 / 20 / 21, or 1 `internal-error` on a reply with neither a parseable `result` nor an `error.code`); exit 20 carries `detail:"wrapper-fault"`, and exit 21 `detail:"instance-dead"` with `during` `connect` (no live snapshot, or the connect failed) or `call` (the connection lost after it).
- (.andromeda/obs-plan-detail/span-trace-coverage.md:89) The client-side `instance-unreachable` outcome on `channel-response` belongs to a reply that never arrived; a verification refusal before any frame must not be logged as this outcome on a `channel-response` the call never made.
  > the client side alone also writes `instance-unreachable` when the reply never arrived (a closed or reset connection).
- (.andromeda/obs-plan-detail/span-trace-coverage.md:90) This is the dated-gap text the chunk closes: once verification lands, `server-verify-failed` (and `strict-modes-failed` per the boundary ruling) can occur on `wait` / `last` as exit 21 at WARN, and the sentence is amended at the wrap.
  > client `process-exit{subject:"self", exit_code, detail}` at WARN, where exit 21 carries `detail:"instance-dead"` plus `during` ∈ `connect|call` (endpoint vanished mid-wait = `during:"call"`). `wait` / `last`, like `send`, use the liveness-only pre-check, so `strict-modes-failed` / `server-verify-failed` cannot occur on them until client-side server verification lands (the dated gap owned by the Epoch 6 route entries "Server verification before any frame" and "Home and code-bearing file integrity").
- (.andromeda/obs-plan-detail/span-trace-coverage.md:100) The `hook.dialog` and `answer` chains keep their order, with the server check placed before `channel.request(hook.dialog)` and `channel.request(answer)`.
  > **Must-trace spans:** `hook.handle` (SERVER) › `channel.request(hook.dialog)` → `channel.dispatch` › `run.dialog_register` › `run.dialog_await` › `hook.decision_emit`. Driver side: `answer.client` › `channel.request(answer)` → `channel.dispatch`.
- (.andromeda/obs-plan-detail/span-trace-coverage.md:111) A hook whose server check fails fails open before any `dialog_id` exists, so it must log the corr-less fail-open pair with a Section 6 fail-open `detail` code.
  > Fail-open before `dialog_id` gives `hook-invoked{corr:null}` + `hook-decision{corr:null, detail}` (Section 6 fail-open codes).
- (.andromeda/obs-plan-detail/span-trace-coverage.md:119) `pause` / `release` verification sits inside `pause.client` / `release.client` before `channel.request(pause|release)`.
  > `pause.client` / `release.client` (CLIENT) › `channel.request(pause|release)` → `channel.dispatch(pause|release)`
- (.andromeda/obs-plan-detail/span-trace-coverage.md:167) The client and hook snapshot readers named here stay on `read_snapshot` with no `state-recovered` line unless the held CARRY 7 fork is ruled by the founder; a move to the classified read changes this E5 sentence and its spans.
  > The five other snapshot readers (`src/cmd/client.rs`, `run`'s collision check, `src/cmd/hook.rs` twice, `src/cmd/hook/statusline.rs`; the rule: `read_snapshot(` under `src/`, outside test modules) still call `read_snapshot` and write no `state-recovered` line.
- (.andromeda/obs-plan.md:253) A verification failure behind exit 21 is its own cause, so it needs a distinct `detail` code carrying no path or pid.
  > each cause behind exit 21 and exit 1 gets a distinct `detail` code with no path or pid.
- (.andromeda/obs-plan.md:327) Any span added for the check has a static `<area>.<operation>` name, is declared with `skip_all` and an explicit field allow-list, and uses no `err` / `ret`.
  > Names are static strings; instance names, dialog ids and paths never appear in a span name. Every span is declared with `#[instrument(skip_all, name = "...", fields(...))]` or `info_span!` with an explicit field allow-list, and the `err` / `ret` options are banned (Section 11).
- (.andromeda/obs-plan.md:337) The check's lines carry the common fields and correlate by `corr` + `conn` only; no `trace_id` / `span_id` is added.
  > `trace_id` / `span_id` are **not** emitted (D-12); correlation is `corr` + `conn`.
- (.andromeda/obs-plan.md:361) The body's broad line for `wait` / `last` names this entry as the owner of the gap; it is amended with the detail line at the wrap.
  > `strict-modes-failed` / `server-verify-failed` cannot occur on `wait` / `last` until client-side server verification lands, owned by the route entries "Server verification before any frame" and "Home and code-bearing file integrity".
- (.andromeda/obs-plan.md:388) The hook's verification must leave E1 unchanged: without `VIOLA_NAME` no check runs, no file and no line.
  > - **E1** (unwrapped hook no-op): no `VIOLA_NAME` means no file, no line, exit 0 and empty stdout/stderr, asserted by absence (D-16).
- (.andromeda/obs-plan.md:394) The body's E5 line holds the other readers line-free; it stands unless the held fork moves a client to the replay.
  > one line per call; the other snapshot readers write no `state-recovered` line.
- (.andromeda/obs-plan.md:694) The MCP tools' Tokio client and the hook must pass `corr` / `conn` / `instance` explicitly to the check's lines, not through span context.
  > NEVER rely on span context to carry `corr` / `conn` / `instance` across threads or tasks. Spans are not serialized, so pass these as explicit values.
- (.andromeda/obs-plan.md:735) The verification `detail` code (e.g. `server-verify-failed`) carries neither the server pid, the recorded pid nor the endpoint path.
  > NEVER put paths or pids inside `detail` codes (Founder Direction 4). Paths appear only in instance detail files.

## Bindings
- **exit 21 verification detail ↔ §6 Log Coverage `detail` catalog**: the new cause behind exit 21 (`server-verify-failed`, and `strict-modes-failed` if the strict-modes half is taken) must be a closed catalog code and admitted by `schemas/diag-line.v1.json` (G4); (.andromeda/obs-plan-detail/span-trace-coverage.md:90), (.andromeda/obs-plan.md:253), (.andromeda/obs-plan.md:735)
- **hook fail-open on mismatch ↔ §6 fail-open codes and the hook contract (exit 0, no stdout/stderr)**: the corr-less `hook-invoked` / `hook-decision` pair needs a fail-open `detail` for a refused server check; (.andromeda/obs-plan-detail/span-trace-coverage.md:111), (.andromeda/obs-plan-detail/span-trace-coverage.md:20)
- **MCP `instance-unreachable` ↔ security-plan error handling (codes only)**: the tool's refusal is a codes-only `isError` mirrored by a codes-only line under `mcp.tool_call`; (.andromeda/obs-plan-detail/span-trace-coverage.md:22)
- **E5 snapshot readers ↔ the held CARRY 7 (founder's fork) and security-plan's strict-modes half ("Home and code-bearing file integrity")**: the readers' line-free state stands until ruled; (.andromeda/obs-plan-detail/span-trace-coverage.md:167), (.andromeda/obs-plan.md:394)
- **dated-gap sentences ↔ wrap master amendment (security.md exceptions closed)**: both the detail line and the body's broad line are rewritten when the gap closes; (.andromeda/obs-plan-detail/span-trace-coverage.md:90), (.andromeda/obs-plan.md:361)
