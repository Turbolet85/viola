# obs · Span / Trace Coverage — category read

## Carried
- (.andromeda/obs-plan-detail/span-trace-coverage.md:18) A CLI verb's `--json` error must never carry the anyhow chain; the chain's only destination is the detail file.
  > Client-side `send-refused{side:"client"}`. The full anyhow chain goes to `detail-cli.ndjson` only
- (.andromeda/obs-plan-detail/span-trace-coverage.md:21) The channel error surface's log pair must keep this field list and never record `params` or result bodies, so any change that tells the `-32602` causes apart stays inside these rules.
  > `channel.request` (client) / `channel.dispatch` (server) spans. `channel-request` / `channel-response` with `method`, `corr`, `conn`, `result_class`, `error_code`, `sender`. Never `params` or result bodies
- (.andromeda/obs-plan-detail/span-trace-coverage.md:22) Every one of the five MCP tools must run inside its `mcp.tool_call` span, and a logged reject must be codes only.
  > `mcp.tool_call` span per tool (`send · wait · last · answer · list`). Codes-only `send-refused` / `channel-response` on rejects.
- (.andromeda/obs-plan-detail/span-trace-coverage.md:28) The CLI witness must match this outcome for a server that fails verification: the closed code on the exit line, with no `channel.request` span and no `channel-request` line.
  > A mismatch or a check that cannot be made opens no `channel.request` span, writes no `channel-request` line and ends `process-exit{subject:"self", exit_code:21, detail:"server-verify-failed", during:"connect"}`.
- (.andromeda/obs-plan-detail/span-trace-coverage.md:30) The refused instance check's `--json` document must stay byte-exact, and it must write no line, so the CLI enumeration names this exact body.
  > a refused check (exit 21, the instance pair on stderr, or under `--json` exactly `{"v":1,"error":"instance-unreachable","detail":null}`) opens no process log and writes no line.
- (.andromeda/obs-plan-detail/span-trace-coverage.md:32) MCP's `instance-unreachable` must stay codes-only with `detail: null`, no frame and no log line, on both the instance-check and server-check paths, and the check must stay inside `mcp.tool_call`.
  > the frame tools `send`, `wait`, `last` and `answer` first run `check_instance` on their instance, before `live_endpoint` (a refusal returns `instance-unreachable` with `detail: null`, no frame and no line), then verify the wrapper after `TokioClient::connect`, inside `mcp.tool_call` (`server_verified`, the start time read on `tokio::task::spawn_blocking`). A mismatch or a check that cannot be made returns the codes-only `isError` `instance-unreachable` with no frame and no log line, as the unreachable path does.
- (.andromeda/obs-plan-detail/span-trace-coverage.md:44) Folding or ratifying the `Refusal` enum must leave this refusal's decision point, its `batch-script-child` code and its no-child outcome as written.
  > the `.cmd`/`.bat` refusal (`batch-script-child`) is decided earlier, by viola-side program resolution (`resolve_program`, after obs init and before the strip plan and `pty.spawn`), and logged as `process-exit{subject:"self", exit_code:1, detail:"batch-script-child"}` with no child spawned and no `process-start{subject:"claude-child"}`;
- (.andromeda/obs-plan-detail/span-trace-coverage.md:50) A span on the start path must never hold a path, an endpoint name or an env value, which is the span-side half of the chunk's never-log floor.
  > Never the endpoint name, `pinned_bin` path or env values.
- (.andromeda/obs-plan-detail/span-trace-coverage.md:54) The cap-driving case owed on the child pipe drain, on its `claude --version` side, must keep this probe's closed subject and its fields when the read passes its cap.
  > `process-start` / `process-exit{subject:"version-probe", child_exit_status, duration_ms}`.
- (.andromeda/obs-plan-detail/span-trace-coverage.md:55) The start's failure `detail` must stay a closed code holding no path or pid.
  > On failure, `process-exit{subject:"self", exit_code:1, detail}` with `detail` ∈ `already-live|squatted-name|pinned-hash-mismatch|batch-script-child` (Founder Direction 4; no path or pid).
- (.andromeda/obs-plan-detail/span-trace-coverage.md:55) The CLI enumeration must name revive's `strict-modes-failed` document as an existing error body held to the closed form, with the revive start-refusal documents owed to the exit-cause catalogue left to that entry.
  > a refusal exits 1 with the home pair or the instance pair on stderr (`viola revive --json`: its `strict-modes-failed` document), opens no log, writes no line and starts no child, so it carries no `detail`.
- (.andromeda/obs-plan-detail/span-trace-coverage.md:55) The cap-driving case owed on the child pipe drain, on its `claude agents --json` side, must match this: one `parse-rejected` on an oversize answer, inside the probe pair, and the read passing on.
  > its `session-live` reading (skipped under `--fork`) writes one `process-start` / `process-exit{subject:"agents-probe"}` pair in `run-<name>.ndjson`, plus one `parse-rejected{parser:"claude-agents-json"}` on an oversize or malformed answer, and passes on a failed read;
- (.andromeda/obs-plan-detail/span-trace-coverage.md:75) This `-32603` site must be in the channel enumeration, answering the fixed form and issuing nothing.
  > A record that fails answers `-32603` with nothing issued and nothing typed, so no `send-issued` is written.
- (.andromeda/obs-plan-detail/span-trace-coverage.md:89) MCP and CLI `send` client refusals must carry only this closed pair, one line per deciding side and never the text.
  > client-side validation refusal, made before any frame: `send-refused{corr:null, refusal:"not-delivered", detail:"control-character"|"empty-text", side:"client"}`, with a mirror line on the wrapper side if the frame reached it.
- (.andromeda/obs-plan-detail/span-trace-coverage.md:91) The response line's error side must remain this field set, on both sides of the channel.
  > `channel-response{corr:<id>, conn, result_class, refusal, detail, duration_ms}` on both sides;
- (.andromeda/obs-plan-detail/span-trace-coverage.md:93) A sanitised handler fault must still reach the `Drop` guard's `channel-response` line, so an early return or error stays logged.
  > `channel-response` is emitted from a `Drop` guard in `channel.dispatch`, so an early return or error still logs it.
- (.andromeda/obs-plan-detail/span-trace-coverage.md:93) A CLI client's error exits must keep these closed codes, including `internal-error` on an unparseable reply, so a reply the client cannot read reaches no free text.
  > `send.client` closes with `process-exit{subject:"self", exit_code}` (0 / 10 / 11 / 12 / 13 / 14 / 20 / 21, or 1 `internal-error` on a reply with neither a parseable `result` nor an `error.code`); exit 20 carries `detail:"wrapper-fault"`, and exit 21 `detail:"instance-dead"` with `during` `connect` (no live snapshot, a replayed one included, or the connect failed) or `call` (the connection lost after it), or `detail:"server-verify-failed"` with `during:"connect"` (the serving process is not the recorded wrapper, or the check cannot be made; no frame sent; Linux and Windows, macOS on the recorded gap).
- (.andromeda/obs-plan-detail/span-trace-coverage.md:94) A witness of `send`'s refused instance check must read no span, no `process-exit` and no line, beside the document.
  > A `send` its instance check refuses (`client::refuse_untrusted`: exit 21, the instance pair, or under `--json` the `instance-unreachable` document) is refused before any role file opens: no `send.client` span, no `process-exit` and no line.
- (.andromeda/obs-plan-detail/span-trace-coverage.md:109) A closed or reset connection must surface as this one code on the client side, and no result body may reach either line.
  > the client side alone also writes `instance-unreachable` when the reply never arrived (a closed or reset connection). A `last` response carries no `outcome`, and no result body reaches either line;
- (.andromeda/obs-plan-detail/span-trace-coverage.md:110) `wait` and `last` must hold the same exact document and the same no-line MCP refusal on both surfaces.
  > a refused instance check exits 21 with the instance pair, or under `--json` exactly `{"v":1,"error":"instance-unreachable","detail":null}`, and sends no frame and writes no line, and MCP `wait` / `last` refused by `check_instance` return `instance-unreachable` with no line.
- (.andromeda/obs-plan-detail/span-trace-coverage.md:120) This `-32603` site must be in the channel enumeration, with the dialog held and no `dialog-answered` line.
  > A record that fails answers `-32603`: the answer is not taken, the dialog stays held and no `dialog-answered` line is written.
- (.andromeda/obs-plan-detail/span-trace-coverage.md:124) The cap-driving case owed on the dialog hold's `config.json` reader must leave this attribute reading the hold the slot was started with.
  > `run.dialog_await`: `deadline_ms` (the hold the slot was started with: `config.json`'s `dialog_hold_s` in ms
- (.andromeda/obs-plan-detail/span-trace-coverage.md:132) `answer`'s refusals must stay these closed codes and exits.
  > driver-side refusals: `channel-response{result_class:"refusal", refusal:"not-delivered", detail:"unknown-dialog"}` (exit 13), or `refusal:"unverified-cli"` (exit 12, answer withheld).
- (.andromeda/obs-plan-detail/span-trace-coverage.md:134) `answer`'s refused instance check must hold the `--json` document and the MCP code with `detail: null` and write no line.
  > a refused instance check on CLI `answer` (`client::refuse_untrusted`, before any role file opens): exit 21 with the instance pair, or under `--json` the `instance-unreachable` document, no `channel-request`, no `process-exit` and no line; MCP `answer` refused by `check_instance` returns `instance-unreachable` with `detail: null` and writes no line.
- (.andromeda/obs-plan-detail/span-trace-coverage.md:144) `pause` and `release` must answer only after the `wheel` record lands, and this `-32603` site must be in the channel enumeration.
  > answered only after the move's `wheel` record lands (`-32603` if it did not).
- (.andromeda/obs-plan-detail/span-trace-coverage.md:151) The channel enumeration must name this `error.data` form, a code under `reason`, beside the plain `data: null`, and this line must hold for any change that tells the `-32602` causes apart.
  > one `release-from-driver{corr:<rpc id>, conn, from (only when a valid name), from_trust:"self-reported"}` at INFO (D-01), beside the `-32602` `"invalid params"` reply with `data: {"reason":"release-from-driver"}`; a `from` of another type, or a non-bool `budget`, is a plain `-32602` (`data: null`) with no such line
- (.andromeda/obs-plan-detail/span-trace-coverage.md:174) The cap-driving case owed on the status-line command's stdout must leave this bound's outcome standing, and that stdout must reach no line.
  > `statusline.shell_out` closes on child exit or at the bound `STATUSLINE_DEADLINE` (5 s, PROVISIONAL, `src/cmd/hook.rs`); the bound ends in `hook-decision{detail:"deadline", deadline_hit:true}`. The user's stdout passes through unchanged and is never logged.
- (.andromeda/obs-plan-detail/span-trace-coverage.md:180) The planted `CLAUDE*` witness must read the strip as names only, on this one line.
  > `env_stripped_count`, `env_stripped_known` (the stripped names) and `env_kept` (the persistent-set names kept) on `process-start{subject:"claude-child"}`; names only, never values (D-34).
- (.andromeda/obs-plan-detail/span-trace-coverage.md:181) No stripped value may be read by the strip or any log, and no env map may reach a span or event, which is the floor the planted value tests.
  > no value is read by the strip, the registry reader or any log, and no env map is passed to a span or event (D-34).
- (.andromeda/obs-plan-detail/span-trace-coverage.md:193) The explicit `link` / `unlink` errors must be in the channel enumeration in exactly these forms.
  > a missing, non-string or invalid name, a `driven` that is not the serving wrapper's own name and a `driver` equal to `driven` are each `-32602` with `data: null` and no line written, a `record` / `remove` error is `-32603`, and neither method reads `from`.
- (.andromeda/obs-plan-detail/span-trace-coverage.md:226) `verify`'s refusal and fault output must stay these fixed forms.
  > Refusals go to stderr as the fixed `unable:` + `hint:` pair, and an internal fault prints exactly `error: internal error`.
- (.andromeda/obs-plan-detail/span-trace-coverage.md:227) `plugin install`'s two `--json` documents must hold no path, and the error one must be in the CLI enumeration.
  > a refused check opens no log and writes no line (exit 21: the stderr pair, or under `--json` the `instance-unreachable` document). Its outcome is the exit code (0 / 1 / 2 / 21) and, under `--json`, `{"v":1,"ok":{"marketplace":"viola","plugin":"viola@viola"}}`; no path in either document or any line.
- (.andromeda/obs-plan.md:327) Any span this chunk adds or moves must take this declared form, with a static name and an explicit field allow-list, and without `err` or `ret`.
  > Names are static strings; instance names, dialog ids and paths never appear in a span name. Every span is declared with `#[instrument(skip_all, name = "...", fields(...))]` or `info_span!` with an explicit field allow-list, and the `err` / `ret` options are banned (Section 11).
- (.andromeda/obs-plan.md:333) `rmcp` and `portable-pty` must stay OFF in the sink, so no upstream text reaches a line through them.
  > Every span is manual — no surface has an auto-instrumentation library viola uses, and the rmcp and portable-pty targets are OFF in the sink; channel lines never record `params` or result bodies.
- (.andromeda/obs-plan.md:345) A failure's code on the start path must carry no path or pid, and no span or line may carry the named paths and values.
  > Never the endpoint name, the `pinned_bin` path or env values in a span, and never the sideload companions' paths or hashes; the child's `--settings` argument, the statusline source path and the command appear in no line or span; a sideload outcome is never a refusal or an exit change; a failure's `detail` carries no path or pid;
- (.andromeda/obs-plan.md:690) A span field must never hold a payload or upstream text, because no later stage scrubs it.
  > NEVER include PII or content in span fields. Scrubbing at an exporter stage is too late, and v1 has no exporter stage at all.
- (.andromeda/obs-plan.md:691) Any instrumented function this chunk touches must use the `skip_all` form.
  > NEVER use a bare `#[instrument]`. It records every argument with `Debug` and leaks send text, `hook.dialog` payloads and tool `input`. Always use `#[instrument(skip_all, fields(...))]`.
- (.andromeda/obs-plan.md:692) A function returning an error that can hold a serde source must never record it through its span.
  > NEVER use `#[instrument(err)]` or `#[instrument(ret)]` on functions whose error or return can carry a serde / serde_path_to_error source or user content.

## Bindings
- **Channel `-32602` causes ↔ §5 Metric Coverage**: the response line's field list and the `release-from-driver` reply bind the cause-telling change, and `viola.compat.v_rejected` over-counts until the line tells them apart, (.andromeda/obs-plan-detail/span-trace-coverage.md:21), (.andromeda/obs-plan-detail/span-trace-coverage.md:151), (.andromeda/obs-plan.md:418)
- **Channel `-32603` sites ↔ §1 channel surface and §7 scrubbing layers**: each site here must answer the fixed form the body names, `"internal error"` with `data:null`, (.andromeda/obs-plan-detail/span-trace-coverage.md:75), (.andromeda/obs-plan-detail/span-trace-coverage.md:193), (.andromeda/obs-plan.md:123), (.andromeda/obs-plan.md:510)
- **Anyhow chain ↔ §7 and §11 Error Reporting**: the detail-file-only chain here binds with the fixed-message mapping and the ban on a serde-sourced chain reaching any of the three surfaces, (.andromeda/obs-plan-detail/span-trace-coverage.md:18), (.andromeda/obs-plan.md:511), (.andromeda/obs-plan.md:724)
- **MCP `instance-unreachable` ↔ §7 Error classes**: the no-line refusal here binds with the `isError` code as its only record, inside `mcp.tool_call`, (.andromeda/obs-plan-detail/span-trace-coverage.md:32), (.andromeda/obs-plan.md:537)
- **E2 strip ↔ §1 triggers and §11 PII Scrubbing**: the names-only line binds with the never-log floor across all sinks that the planted-value witness proves, (.andromeda/obs-plan-detail/span-trace-coverage.md:181), (.andromeda/obs-plan.md:210), (.andromeda/obs-plan.md:734)
- **`Refusal` fold ↔ §7 Platform pick and scrubbing layer 1**: the `batch-script-child` outcome here binds with the one thiserror type per crate and its fixed `Display`, (.andromeda/obs-plan-detail/span-trace-coverage.md:44), (.andromeda/obs-plan.md:496), (.andromeda/obs-plan.md:509)
- **Secret scan ↔ §8 Verification and §11 Universal**: `verify`'s home-level lines fall under the same scan that must prove the floor for `v1-47`, (.andromeda/obs-plan-detail/span-trace-coverage.md:231), (.andromeda/obs-plan.md:570), (.andromeda/obs-plan.md:759)
- **Oversize `viola mcp` stdin ↔ §1 `viola mcp` surface**: this category holds no span for the pre-`initialize` case, so the owed test reads against the surface line's session end, (.andromeda/obs-plan.md:125)
