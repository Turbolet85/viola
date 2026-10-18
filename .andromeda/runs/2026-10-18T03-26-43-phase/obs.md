# obs extract

## Relevance
partial — the chunk changes the confirmation leg of a must-trace path (Confirmed `send`, CL-1) and the shape of `viola verify`'s lines, but it adds no new instrumented surface, event or process role.

## Constraints
- A `/compact` send confirmed by its post-condition stays on the CL-1 must-trace chain `send.client` › `channel.request` → `channel.dispatch` › `run.readiness_gate` › `pty.paste_write` › `run.confirm_window`, with `run.confirm_window` carrying `window_ms`. For `/compact`, `window_ms` must be the compact post-condition's own bound, not `CONFIRM_WINDOW_FALLBACK`. Whether the code records `window_ms` per post-condition today is research's question (per obs-plan §4 Scenario: Confirmed `send` (CL-1) from driver to readback).
- The CL-1 Cleanup rule names only the `/clear` close: a `session-start` of cause `clear` whose `agent_session_id` differs from the remembered id. The `/compact` close (cause `compact`, the SAME id) is a new close condition, so the plan text needs amending. Window expiry stays `send-refused{detail:"no-prompt-submitted"}` with no new detail, and no `prompt-submitted` claims a post-condition send. A listed command on an unverified CLI still opens no `run.confirm_window` (per obs-plan §4 Scenario: Confirmed `send` (CL-1) from driver to readback).
- The required log fields stay `send-issued{corr:<cursor>, from, text_bytes}`, then `hook-invoked{hook_event:"session-start"}`, then `send-confirmed{corr:<cursor>, confirmed, duration_ms}` (`confirmed:false` only for `unconfirmable`) or `send-refused`. Wrapper `send-*` lines carry `conn` / `rpc_id`. Neither the sent text nor a session id appears in any of them (per obs-plan §4 Scenario: Confirmed `send` (CL-1) from driver to readback).
- A new `event` value, `process-exit` `detail`, `parse-rejected` `detail` or `subject` value needs a Decisions Log entry and a `schemas/diag-line.v1.json` change. `send-refused.detail` values are the arch `RefusalReason` details verbatim (per obs-plan §3 Log format JSON schema; per obs-plan §6 `detail` code catalog).
- `verify` must keep exactly one `[NN/MM] <row id> <row words>  pass|fail` step line per relied row, with `MM` = `LedgerRow::relied()`. A 24th row relied on every OS moves `MM` to 24 on Unix and 21 on Windows. The last line is `stamped <version>  N pass  N fail`, and the two counts sum to `MM`. Today `verify` logs eight `process-start` / `process-exit{subject, child_exit_status, duration_ms}` spawn pairs. A compaction probe placed inside Run B keeps eight. A new interactive start adds a ninth `verify-pty-probe` pair, which the plan's count must then name. No line may carry a prompt, a session id, a row's text or a path (per obs-plan §4 Edge flows → `verify` / `plugin install`).
- The hidden `hook <event> --capture` arm that would file a compact-cause `SessionStart.<k>.json` capture stays uninstrumented: no `VIOLA_*` read, no obs init, no role file, no `hook-invoked` / `hook-decision` (per obs-plan §4 Edge flows → `verify` / `plugin install`).
- Confirmation keys on hook events only (the SessionStart cause and session id), never on screen content or a sleep (per obs-plan §10 Standard+ invariants, Deterministic emission). A home stamped before the new row reads `cli_verified:false` on `process-start{subject:"claude-child"}` through `state.ledger_read` (per obs-plan §4 Edge flows → Capability ledger).

## Patterns to follow
- The `/clear` `NewSession` arm is the template for the new arm: same spans and same line sequence, with the `session-start` hook awaited in place of `prompt-submitted` and the same expiry detail (per obs-plan §4 Scenario: Confirmed `send` (CL-1) from driver to readback).
- Emit only through `obs_event!` under `#[instrument(skip_all, name = "...", fields(...))]`, with an explicit field allow-list and a static `<area>.<operation>` span name (per obs-plan §4 Span naming convention; per obs-plan §11 Logs).
- `verify` logs spawn pairs at the call site with a closed `subject`. Run E's reuse of `verify-pty-probe` is the precedent for adding a start without a new `subject` value (per obs-plan §6 Boundary-call wrappers → Child / shell spawns).
- `viola.send.confirm_ms` is derived from `send-confirmed` lines whose `confirmed` is not `false`. Confirmed `/compact` sends join that histogram, and their samples run up to the new window. Its grouping keys stay the closed enums (per obs-plan §5 Per-surface / per-path metrics; per obs-plan §5 Metric label cardinality discipline).

## Anti-patterns to avoid
- A bare `#[instrument]`, or `err` / `ret`, on the new post-condition or claim code. Never log `agent_session_id`, the session id or a path in a span field or process-log line (per obs-plan §11 Spans / Traces; per obs-plan §11 PII Scrubbing).
- Minting a new `detail` code (for example a compact-specific timeout) or a new `event` value without a Decisions Log entry (per obs-plan §11 Logs).
- Deriving compaction completion from the vt100 screen (per obs-plan §11 Telemetry Strategy).

## Contract bindings
- obs ↔ tests §3 Log format: `schemas/diag-line.v1.json` closes `event`, `subject` and the `process-exit` / `parse-rejected` `detail` enums, and G4 validates every line the new tests write (per obs-plan §3 Log format JSON schema).
- obs ↔ design-system §Streams: the step line count and the `stamped` summary line are what an agent reads from `verify` with no instance (per obs-plan §4 Edge flows → `verify` / `plugin install`).
- obs ↔ arch Event line: the `session-start` with cause `compact` lives in `events.ndjson` (the product stream), not in a process log (per obs-plan §3 Log format JSON schema).
- obs ↔ security NEVER-log floor: no session id, prompt, path or `CLAUDE*` value in any home-level line, which the secret scan enforces (per obs-plan §11 PII Scrubbing).

## Acceptance criteria contributions
- On a stamped fake-agent home, a `/compact` send writes `send-issued` then `send-confirmed{confirmed:true, duration_ms}` with `corr` = the cursor once the same-id `session-start{cause:"compact"}` arrives. A send whose post-condition never arrives writes `send-refused{detail:"no-prompt-submitted"}` at the compact window's expiry. On an unstamped home the send writes `send-confirmed{confirmed:false}` and opens no window (per obs-plan §4 Scenario: Confirmed `send` (CL-1) from driver to readback).
- `viola verify` on the fake agent prints `MM` = 24 on Unix and 21 on Windows, and `stamped … N pass  N fail` sums to `MM`. Under a `VIOLA_NAME`, `cli-<name>.ndjson` holds the spawn-pair count the chosen probe placement implies, and no line carries a session id, prompt or path (per obs-plan §4 Edge flows → `verify` / `plugin install`).
- G2 (zero `event:"panic"` lines), G4 (schema conformance) and the secret scan pass over the homes the new cases write (per obs-plan §10 Error budget).
