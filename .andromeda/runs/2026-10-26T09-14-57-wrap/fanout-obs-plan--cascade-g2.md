proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/span-trace-coverage.md
    near: 18
    section: '## 4. Span / Trace Coverage'
    change: 'In the cli (short-lived verbs) row the dispatch-span list gains `last.client`, and after it add: «Each of the six frame verbs (`send`, `wait`, `last`, `answer`, `pause`, `release`) verifies the wrapper through `cmd::client::connect` (the connected stream''s peer pid and its OS start time against the one snapshot read, the closed `ServerVerdict`) inside its dispatch span, after its own exit-13 refusals and before any frame; a mismatch or an unmakeable check opens no `channel.request` span, writes no `channel-request` line and ends `process-exit{exit_code:21, detail:"server-verify-failed", during:"connect"}` (as landed, chunk 2026-10-26-server-verification-before-any-frame; Linux and Windows, macOS on the recorded gap)».'
    sidecar: 'span-trace-coverage cli row: `last.client` named; the six frame verbs'' pre-frame server check inside their dispatch spans added.'
    rationale: 'Report Symbols / APIs: each verb''s `request` goes through `client::connect` inside `send.client`, `wait.block`, `last.client`, `answer.client`, `pause.client`, `release.client`, the client''s own refusals first; Decisions (must-trace check): no `channel.request` span or `channel-request` line, since no frame is sent. The row lists five of the six dispatch spans and no server check.'
    fact: F1
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/span-trace-coverage.md
    near: 18
    section: '## 4. Span / Trace Coverage'
    change: 'After the F1 clause in the cli (short-lived verbs) row add: «The refusal''s place is asserted by captured-span cases, one per CLI frame verb (`request_refuses_an_unverified_server_inside_its_dispatch_span` in each verb''s module) and one for the hook (`hook_refuses_an_unverified_server_inside_hook_handle`), each through the shared test-only helper `cmd::client::refused_inside` (`assert_refused_inside`): the server refusal happens inside its scenario''s root span, and each case reads red with the check removed (chunk 2026-10-26-server-verification-before-any-frame)».'
    sidecar: 'span-trace-coverage cli row: the captured-span cases asserting the server refusal inside each root span named, with the shared `refused_inside` helper.'
    rationale: 'Report Symbols / APIs (`refused_inside`, test-only, shared by the six verbs'' cases), New text (each verb''s `request_refuses_an_unverified_server_inside_its_dispatch_span`, hook.rs `hook_refuses_an_unverified_server_inside_hook_handle`) and outcome deviation 6 (added on the operator''s check, each red with the check removed). Functions named, no case count, per the Tier 1 rule.'
    fact: F6
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/span-trace-coverage.md
    near: 20
    section: '## 4. Span / Trace Coverage'
    change: 'In the cli (`viola hook`) row, after `hook.handle` › `channel.request` › `hook.decision_emit` add: «The event and dialog paths share one `connect` inside `hook.handle` (`read_snapshot`, then `connect_by`, then `verify_server`) before any frame; a server that is not the recorded wrapper, or an unmakeable check, sends no frame, opens no `channel.request` and fails open (exit 0, empty streams) with `hook-decision{detail:"server-verify-failed"}`; a SessionEnd whose check fails takes the direct `append_session_end` (as landed, chunk 2026-10-26-server-verification-before-any-frame; Linux and Windows, macOS on the recorded gap)».'
    sidecar: 'span-trace-coverage hook row: the shared pre-frame `connect` (read, connect, verify) and its fail-open outcome added; SessionEnd falls back to the direct append.'
    rationale: 'Report Symbols / APIs for `src/cmd/hook.rs`: `enum NotSent` (`server-verify-failed`), a shared `connect` used by `deliver` and `ask`, a SessionEnd whose check fails takes `append_session_end`; Outcome: the hook against the impostor exits 0, empty streams, no frame, `hook-decision` `server-verify-failed`.'
    fact: F4
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/span-trace-coverage.md
    near: 22
    section: '## 4. Span / Trace Coverage'
    change: 'In the ipc-internal (viola-mcp) row add: «The frame tools `send`, `wait`, `last` and `answer` verify the wrapper after `TokioClient::connect`, inside `mcp.tool_call` (`server_verified`, the start time read on `tokio::task::spawn_blocking`); a mismatch returns the codes-only `isError` `instance-unreachable` with no frame and no log line, as the unreachable path does (as landed, chunk 2026-10-26-server-verification-before-any-frame; Linux and Windows)».'
    sidecar: 'span-trace-coverage mcp row: the frame tools'' server check after connect, inside `mcp.tool_call`, mismatch codes-only with no line.'
    rationale: 'Report Symbols / APIs for `crates/viola-mcp/src/tools.rs` (`frame` verifies after `TokioClient::connect` through `server_verified`, start time on `spawn_blocking`; a mismatch returns `result::unreachable()` with no frame, no new log line) and Coverage (instrumentation n/a by plan, inside `mcp.tool_call`).'
    fact: F5
  - detector: D-cascade
    severity: warning
    file: .andromeda/obs-plan-detail/span-trace-coverage.md
    near: 59
    section: '## 4. Span / Trace Coverage › #### Scenario: Confirmed `send` (CL-1) from driver to readback'
    change: 'In the Must-trace spans, after `send.client` (CLIENT; `cli` or `mcp.tool_call`) add: «, whose server check (the peer pid and start time against the one snapshot read) runs before `channel.request`: a server that is not the recorded wrapper opens no `channel.request` and ends the path at `send.client` (chunk 2026-10-26-server-verification-before-any-frame)».'
    sidecar: 'span-trace-coverage Confirmed send: the pre-frame server check inside `send.client` placed on the must-trace chain.'
    rationale: 'Second occurrence of the F1 fact on the send scenario: report Symbols / APIs (send''s request goes through `client::connect` inside `send.client`; MCP `frame` verifies after connect) and the must-trace check (no `channel.request` span when no frame is sent).'
    dependent-of: D-new-detail
  - detector: D-cascade
    severity: warning
    file: .andromeda/obs-plan-detail/span-trace-coverage.md
    near: 72
    section: '## 4. Span / Trace Coverage › #### Scenario: Confirmed `send` (CL-1) from driver to readback'
    change: 'After the client-side validation refusal bullet add: «  - client-side server-verification refusal, made after the client''s own refusals and before any frame (Linux and Windows, chunk 2026-10-26-server-verification-before-any-frame): no `channel-request` / `channel-response` line on either side; CLI `send` ends `process-exit{subject:"self", exit_code:21, detail:"server-verify-failed", during:"connect"}` at WARN, MCP `send` writes no line;».'
    sidecar: 'span-trace-coverage Confirmed send log fields: the server-verification refusal''s lines (none on the channel, the client process-exit) added.'
    rationale: 'Report Schema / config: this build writes `process-exit{subject:"self", exit_code:21, detail:"server-verify-failed", during:"connect"}` at WARN into `cli-<name>.ndjson`, MCP no line; Outcome (obs): no `channel-request`. The log-field list names every other client refusal but not this one.'
    dependent-of: D-new-detail
  - detector: D-cascade
    severity: warning
    file: .andromeda/obs-plan-detail/span-trace-coverage.md
    near: 75
    section: '## 4. Span / Trace Coverage › #### Scenario: Confirmed `send` (CL-1) from driver to readback'
    change: 'In the Cleanup, the exit-21 clause reads «exit 21 `detail:"instance-dead"` with `during` `connect` (no live snapshot, or the connect failed) or `call` (the connection lost after it), or `detail:"server-verify-failed"` with `during:"connect"` (the serving process is not the recorded wrapper, or the check cannot be made; no frame sent; Linux and Windows since chunk 2026-10-26-server-verification-before-any-frame, macOS on the recorded gap)».'
    sidecar: 'span-trace-coverage Confirmed send cleanup: exit 21 gains `server-verify-failed` (during connect) beside `instance-dead`.'
    rationale: 'Report Schema / config: `server-verify-failed` is first written by this build as `process-exit{exit_code:21, detail:"server-verify-failed", during:"connect"}`; the Cleanup lists `instance-dead` as exit 21''s only detail, now stale.'
  - detector: D-cascade
    severity: warning
    file: .andromeda/obs-plan-detail/span-trace-coverage.md
    near: 83
    section: '## 4. Span / Trace Coverage › #### Scenario: `wait` / `last` event-driven readback'
    change: 'In the Must-trace spans add: «The server check runs inside `wait.block` / `last.client`, after the client''s own exit-13 refusals and before `channel.request`; a mismatch opens no `channel.request` (chunk 2026-10-26-server-verification-before-any-frame; Linux and Windows)».'
    sidecar: 'span-trace-coverage wait/last: the pre-frame server check inside `wait.block` / `last.client` placed on the must-trace chain.'
    rationale: 'Second occurrence of the F1 fact on the wait / last scenario: report Symbols / APIs (`wait.block`, `last.client` dispatch spans; the client''s own refusals stay first) and the must-trace check.'
    dependent-of: D-new-detail
  - detector: D-retired
    severity: warning
    file: .andromeda/obs-plan-detail/span-trace-coverage.md
    near: 90
    section: '## 4. Span / Trace Coverage › #### Scenario: `wait` / `last` event-driven readback'
    change: 'The bullet reads «client `process-exit{subject:"self", exit_code, detail}` at WARN, where exit 21 carries `detail:"instance-dead"` plus `during` ∈ `connect|call` (endpoint vanished mid-wait = `during:"call"`), or `detail:"server-verify-failed"` with `during:"connect"` when the serving process is not the recorded wrapper or the check cannot be made (Linux and Windows since chunk 2026-10-26-server-verification-before-any-frame; macOS on the recorded gap). `wait` / `last` take the liveness-only pre-check and then the server check; `strict-modes-failed` cannot occur on them until the strict-modes half lands (owned by the route entry "Home and code-bearing file integrity")».'
    sidecar: 'span-trace-coverage wait/last: «server-verify-failed cannot occur» retired; exit 21 `server-verify-failed` (during connect) added; `strict-modes-failed` still owed to "Home and code-bearing file integrity".'
    rationale: 'Report Retired names the wait / last gap''s server-identity half as closed on Linux and Windows; Spec claims disproved states this line false for `server-verify-failed` (`tests/security_negatives_server_identity.rs` reads it on `wait` and `last`) while `strict-modes-failed` still cannot occur; the owner "Server verification before any frame" is closed by this chunk.'
    basis: .andromeda/obs-plan-detail/span-trace-coverage.md:90
  - detector: D-retired
    severity: warning
    file: .andromeda/obs-plan.md
    near: 361
    section: '## 4. Span / Trace Coverage › #### Scenario: `wait` / `last` event-driven readback'
    change: 'The broad line''s last clause reads «`server-verify-failed` (exit 21, `during:"connect"`) occurs on `wait` / `last` on Linux and Windows since chunk 2026-10-26-server-verification-before-any-frame; `strict-modes-failed` cannot occur on them until the strict-modes half lands, owned by the route entry "Home and code-bearing file integrity"».'
    sidecar: 'obs-plan §4 wait/last broad line: the «cannot occur until client-side server verification lands» clause retired for `server-verify-failed`.'
    rationale: 'The body''s broad line restates the detail sentence the report disproves (Spec claims disproved, obs-plan.md:361); same retirement as the detail entry.'
    basis: .andromeda/obs-plan.md:361
    dependent-of: D-retired
  - detector: D-cascade
    severity: warning
    file: .andromeda/obs-plan-detail/span-trace-coverage.md
    near: 92
    section: '## 4. Span / Trace Coverage › #### Scenario: `wait` / `last` event-driven readback'
    change: 'The Cleanup''s first sentence reads «`wait.block` closes on the response, on the connection error, or on a server-verification refusal made before any frame (no `channel.request` opened; the client''s `process-exit` its one line)».'
    sidecar: 'span-trace-coverage wait/last cleanup: `wait.block` also closes on a pre-frame server-verification refusal.'
    rationale: 'Report Symbols / APIs: a mismatch inside `wait.block` is exit 21 before any frame; the Cleanup names only the response and the connection error as its closers.'
    dependent-of: D-new-detail
  - detector: D-cascade
    severity: warning
    file: .andromeda/obs-plan-detail/span-trace-coverage.md
    near: 100
    section: '## 4. Span / Trace Coverage › #### Scenario: Dialog → `answer` (question / permission / plan), including the unverified-CLI branch'
    change: 'In the Must-trace spans add: «A dialog hook whose server check fails (a server that is not the recorded wrapper, or an unmakeable check) opens no `channel.request(hook.dialog)` and emits no decision (fail open); driver side, `answer.client` runs the same check before `channel.request(answer)`, a mismatch opening none (chunk 2026-10-26-server-verification-before-any-frame; Linux and Windows)».'
    sidecar: 'span-trace-coverage Dialog → answer: the pre-frame server check on the hook''s `hook.dialog` and on `answer.client` placed on the must-trace chain.'
    rationale: 'Report Symbols / APIs: `ask` uses the shared hook `connect`, `Asked::ServerUnverified`; `answer`''s request goes through `client::connect` inside `answer.client`; Outcome: `pre-tool-use` / `permission-request` against the impostor send no frame.'
    dependent-of: D-new-detail
  - detector: D-cascade
    severity: warning
    file: .andromeda/obs-plan-detail/span-trace-coverage.md
    near: 108
    section: '## 4. Span / Trace Coverage › #### Scenario: Dialog → `answer` (question / permission / plan), including the unverified-CLI branch'
    change: 'After «fail-open paths before `dialog_id` emit `hook-invoked{corr:null}` immediately» add: «— among them a failed server check, which writes `hook-invoked{corr:null}` then `hook-decision{corr:null, decision_emitted:false, detail:"server-verify-failed"}`, with no `hook.dialog` frame and so no `dialog-raised` (chunk 2026-10-26-server-verification-before-any-frame)».'
    sidecar: 'span-trace-coverage Dialog → answer: the server-verification fail-open named among the paths before `dialog_id`, with its two lines.'
    rationale: 'Report Schema / config gives the dialog path''s order, `hook-invoked{corr:null}` then `hook-decision{corr:null, decision_emitted:false, detail:"server-verify-failed"}`. The D-07 rule still holds (it is a fail-open path before `dialog_id`); the entry gains the named case.'
    dependent-of: D-new-detail
  - detector: D-cascade
    severity: warning
    file: .andromeda/obs-plan-detail/span-trace-coverage.md
    near: 109
    section: '## 4. Span / Trace Coverage › #### Scenario: Dialog → `answer` (question / permission / plan), including the unverified-CLI branch'
    change: 'After the driver-side refusals add: «; a server-verification refusal on CLI `answer`, made before any frame: no `channel-request` / `channel-response` line, `process-exit{exit_code:21, detail:"server-verify-failed", during:"connect"}` at WARN (MCP `answer`: no line)».'
    sidecar: 'span-trace-coverage Dialog → answer: the driver-side server-verification refusal''s exit 21 line added.'
    rationale: 'Report Symbols / APIs: `answer_of` maps `CallError::ServerUnverified` to `Unreachable::ServerUnverified`, exit 21; Schema / config: the `process-exit` line at WARN, MCP no line. The driver-side list names exit 13 and 12 only.'
    dependent-of: D-new-detail
  - detector: D-cascade
    severity: warning
    file: .andromeda/obs-plan-detail/span-trace-coverage.md
    near: 119
    section: '## 4. Span / Trace Coverage › #### Scenario: Human takes the wheel, automation is refused, `release` returns it'
    change: 'In the Must-trace spans add: «`pause.client` / `release.client` run the server check before `channel.request(pause|release)`; a mismatch opens no `channel.request` and sends no frame (chunk 2026-10-26-server-verification-before-any-frame; Linux and Windows)».'
    sidecar: 'span-trace-coverage wheel scenario: the pre-frame server check inside `pause.client` / `release.client` placed on the must-trace chain.'
    rationale: 'Second occurrence of the F1 fact on the wheel scenario: report Symbols / APIs (`pause.client`, `release.client` go through `client::connect`; `release` reaches the wrapper through `client::connect` as the others do).'
    dependent-of: D-new-detail
  - detector: D-cascade
    severity: warning
    file: .andromeda/obs-plan-detail/span-trace-coverage.md
    near: 123
    section: '## 4. Span / Trace Coverage › #### Scenario: Human takes the wheel, automation is refused, `release` returns it'
    change: 'After the `channel-request` / `channel-response{method:"release"}` bullet add: «  - a server-verification refusal on `pause` / `release`, made before any frame: no `channel-request` / `channel-response` line, the client''s `process-exit{exit_code:21, detail:"server-verify-failed", during:"connect"}` at WARN (chunk 2026-10-26-server-verification-before-any-frame);».'
    sidecar: 'span-trace-coverage wheel scenario log fields: the pause/release server-verification refusal''s exit 21 line added.'
    rationale: 'Report Symbols / APIs (a mismatch is exit 21 for each of the six verbs, `pause` and `release` among them) and Schema / config (the `process-exit` line, WARN, `during:"connect"`).'
    dependent-of: D-new-detail
  - detector: D-cascade
    severity: warning
    file: .andromeda/obs-plan-detail/span-trace-coverage.md
    near: 167
    section: '## 4. Span / Trace Coverage › #### Scenario: Budget governor'
    change: 'The sentence''s readers list reads «The three other snapshot readers (`run`''s collision check in `src/cmd/run.rs`, `src/cmd/hook.rs` once — its event and dialog paths share one read since chunk 2026-10-26-server-verification-before-any-frame — and `src/cmd/hook/statusline.rs`; the rule: `read_snapshot(` under `src/`, outside test modules; the CLI frame verbs and `viola mcp` read through `viola_state::liveness::live_endpoint`, outside the rule''s `src/`)»; the rest of the sentence stands.'
    sidecar: 'span-trace-coverage E5: the other snapshot readers re-counted, five to three by the stated rule; `src/cmd/client.rs` dropped, hook.rs once.'
    rationale: 'Report Counts / qualifiers moved: pattern `readers` measures three product callers of `read_snapshot(` under `src/` outside test modules (hook.rs, hook/statusline.rs, run.rs); `src/cmd/client.rs` holds no call since the CLI read moved into `live_endpoint`; this chunk joined hook.rs''s two reads. This site is listed among the five changed.'
    basis: .andromeda/obs-plan-detail/span-trace-coverage.md:167
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/log-coverage.md
    near: 28
    section: '## 6. Log Coverage'
    change: 'After «`process-exit` exit 21: `instance-dead`, `strict-modes-failed`, `server-verify-failed`.» add: «As landed (chunk 2026-10-26-server-verification-before-any-frame), this build writes `server-verify-failed` as `process-exit{subject:"self", exit_code:21, detail:"server-verify-failed", during:"connect"}` at WARN into `cli-<name>.ndjson`, from the six CLI frame verbs on a mismatch or an unmakeable server check (Linux and Windows); no pid, start time, path or endpoint».'
    sidecar: 'log-coverage process-exit exit 21: `server-verify-failed` marked written by this build, with its line and sink.'
    rationale: 'Report Schema / config: `server-verify-failed` was already in the schema''s `process-exit` enum and this build is the first that writes it, `during:"connect"`, WARN, `cli-<name>.ndjson`, no pid / start time / path / endpoint (asserted).'
    basis: .andromeda/obs-plan-detail/log-coverage.md:28
    fact: F2
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/log-coverage.md
    near: 30
    section: '## 6. Log Coverage'
    change: 'After the fail-open code list add: «`server-verify-failed` is written by this build (chunk 2026-10-26-server-verification-before-any-frame): on a server that is not the recorded wrapper, `viola hook` writes `hook-decision{detail:"server-verify-failed"}` at WARN into `hook-<name>.ndjson`; on the dialog path the order is `hook-invoked{corr:null}` then `hook-decision{corr:null, decision_emitted:false, detail:"server-verify-failed"}` (Linux and Windows)».'
    sidecar: 'log-coverage hook-decision fail-open codes: `server-verify-failed` marked written by this build, with the dialog path''s line order.'
    rationale: 'Report Schema / config: `hook-decision{detail:"server-verify-failed"}` at WARN into `hook-<name>.ndjson`, the dialog path `hook-invoked{corr:null}` then `hook-decision{corr:null, decision_emitted:false, ...}`; first written by this build.'
    basis: .andromeda/obs-plan-detail/log-coverage.md:30
    fact: F3
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/pii-scrubbing-compliance.md
    near: 36
    section: '## 8. PII Scrubbing & Compliance › ### PII Scrubbing'
    change: 'After the pre-push document bullet add: «   - A server-verification refusal carries no pid, start time, path or endpoint: `tests/security_negatives_server_identity.rs` scans every output and the `cli-<name>.ndjson` line with `digit_runs` (whole numbers only, itself tested by `digit_runs_reads_whole_numbers_only`), Linux and Windows (chunk 2026-10-26-server-verification-before-any-frame)».'
    sidecar: 'pii-scrubbing verification: the server-identity negatives'' `digit_runs` scan of every output and the cli line named.'
    rationale: 'Report Outcome (obs): no pid / start time / path / endpoint, `digit_runs` scans every output; New text lists `digit_runs` and `digit_runs_reads_whole_numbers_only` in the new test file, compiled out on macOS.'
    fact: F7
note: 'F8 (obs-scope-summary, the macOS recorded gap) is no category of this group; the hook perf rows'' 2.2–2.4 ms belong to SLO Invariants, not metric-coverage; D-obs-stack and D-obs-pii read clean (no dependency moved, the new lines codes only, asserted).'
