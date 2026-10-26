proposals:
  - detector: D-retired
    severity: warning
    file: .andromeda/obs-plan-detail/span-trace-coverage.md
    near: 90
    section: '## 4. Span / Trace Coverage › #### Scenario: `wait` / `last` event-driven readback'
    change: '  - client `process-exit{subject:"self", exit_code, detail}` at WARN, where exit 21 carries `detail:"instance-dead"` plus `during` ∈ `connect|call` (endpoint vanished mid-wait = `during:"call"`), or, on Linux and Windows, `detail:"server-verify-failed"` with `during:"connect"` when the serving process is not the recorded wrapper (no `channel.request` span, no `channel-request` line; as landed, chunk 2026-10-26-server-verification-before-any-frame; macOS stays on the recorded gap). `wait` / `last` take the liveness-only pre-check and then the server check before their frame; `strict-modes-failed` still cannot occur on them until the strict-modes half lands (owned by the route entry "Home and code-bearing file integrity").'
    sidecar: 'span-trace-coverage wait/last: the «server-verify-failed cannot occur until client-side server verification lands» clause retired; exit 21 server-verify-failed (during connect) added on Linux and Windows; strict-modes-failed still owed to "Home and code-bearing file integrity".'
    rationale: 'Report Retired names the wait / last dated gap (server-identity half) as closed on Linux and Windows; Spec claims disproved states this line false for server-verify-failed (tests/security_negatives_server_identity.rs reads it on wait and last) while strict-modes-failed still cannot occur; Schema / config: this build writes process-exit{exit_code:21, detail:"server-verify-failed", during:"connect"} at WARN.'
    basis: viola-0.1.0/chunks/2026-10-26-server-verification-before-any-frame/report.md:55
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/span-trace-coverage.md
    near: 10
    section: '## 4. Span / Trace Coverage'
    change: '- Pre-frame server check (as landed, chunk 2026-10-26-server-verification-before-any-frame; Linux and Windows, macOS on the recorded gap): every frame-writing client verifies the serving process (the connected stream''s peer pid and its start time against the one snapshot read) before its first frame, with no span of its own. The six CLI frame verbs run it inside their dispatch span (`send.client`, `wait.block`, `last.client`, `answer.client`, `pause.client`, `release.client`), after their own exit-13 refusals; a mismatch or an unmakeable check opens no `channel.request` span, writes no `channel-request` line and ends `process-exit{subject:"self", exit_code:21, detail:"server-verify-failed", during:"connect"}` at WARN in `cli-<name>.ndjson`. `viola hook` runs it inside `hook.handle` for both `hook.event` and `hook.dialog`: no `channel.request`, `hook-decision{detail:"server-verify-failed"}` at WARN in `hook-<name>.ndjson` (dialog path: `hook-invoked{corr:null}` then `hook-decision{corr:null, decision_emitted:false, detail:"server-verify-failed"}`), exit 0 with empty streams; a SessionEnd whose check fails takes the direct append. `viola mcp`''s four frame tools run it inside `mcp.tool_call` after the connect and write no line for the mismatch, as on their unreachable path. No pid, start time, path or endpoint reaches any line. The strict-modes half (`strict-modes-failed`) stays owed to "Home and code-bearing file integrity".'
    sidecar: 'span-trace-coverage §4 spans: new item for the pre-frame server check — where it runs (six CLI dispatch spans, hook.handle, mcp.tool_call), its outcome lines (cli process-exit 21 server-verify-failed during connect; hook-decision server-verify-failed; MCP no line), no channel.request on a mismatch.'
    rationale: 'Report Symbols / APIs (src/cmd/{send,wait,last,answer,pause,release}.rs check inside the dispatch span; src/cmd/hook.rs shared connect for deliver and ask; crates/viola-mcp/src/tools.rs server_verified), Schema / config (first build to write server-verify-failed, no pid/path in any line), Coverage of new surfaces and the must-trace check in Decisions & corrections (no channel.request span or channel-request line, since no frame is sent).'
    basis: viola-0.1.0/chunks/2026-10-26-server-verification-before-any-frame/report.md:21
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/span-trace-coverage.md
    near: 75
    section: '## 4. Span / Trace Coverage › #### Scenario: Confirmed `send` (CL-1) from driver to readback'
    change: 'In the Cleanup bullet, the exit-21 clause reads: exit 20 carries `detail:"wrapper-fault"`, and exit 21 `detail:"instance-dead"` with `during` `connect` (no live snapshot, or the connect failed) or `call` (the connection lost after it), or, on Linux and Windows, `detail:"server-verify-failed"` with `during:"connect"` (the serving process is not the recorded wrapper: no `channel.request` span, no `channel-request` line, no `send-issued`; as landed, chunk 2026-10-26-server-verification-before-any-frame).'
    sidecar: 'span-trace-coverage Confirmed send cleanup: exit 21 gains its second detail, server-verify-failed (during connect), beside instance-dead.'
    rationale: 'The line names instance-dead as exit 21''s only detail; the report''s Symbols / APIs (send verifies inside send.client, a mismatch is exit 21) and Schema / config (process-exit{subject:"self", exit_code:21, detail:"server-verify-failed", during:"connect"} at WARN, written by this build) add a second.'
    basis: viola-0.1.0/chunks/2026-10-26-server-verification-before-any-frame/report.md:15
  - detector: D-obs-instrumentation
    severity: warning
    file: .andromeda/obs-plan-detail/span-trace-coverage.md
    near: 167
    section: '## 4. Span / Trace Coverage › #### Scenario: Budget governor'
    change: 'First sentence reads: The three other snapshot readers (`run`''s collision check, `src/cmd/hook.rs` once — its event and dialog paths share one read since chunk 2026-10-26-server-verification-before-any-frame —, `src/cmd/hook/statusline.rs`; the rule: `read_snapshot(` under `src/`, outside test modules; the CLI verbs'' and `viola mcp`''s read is `viola_state::liveness::live_endpoint`, outside that rule) still call `read_snapshot` and write no `state-recovered` line. The rest of the item is unchanged.'
    sidecar: 'span-trace-coverage E5: the other snapshot readers five → three under the rule `read_snapshot(` under src/ outside test modules (src/cmd/client.rs holds none; hook.rs one read).'
    rationale: 'Report Counts / qualifiers moved (pattern readers): the rule reads 3 product callers at this tree — src/cmd/hook.rs:204, src/cmd/hook/statusline.rs:142, src/cmd/run.rs:379; src/cmd/client.rs holds no read_snapshot( call; this chunk moved hook.rs''s two reads to one; this site listed as changed.'
    basis: viola-0.1.0/chunks/2026-10-26-server-verification-before-any-frame/report.md:25
note: 'D-obs-pii and D-obs-stack read clean (no pid/path in any new line, asserted; no dependency moved); the MCP mismatch writing no line matches its existing unreachable path, but obs-plan.md:205 (Vector 5: rejected tool calls logged with codes only) does not hold for it — a body-owner question, not proposed here.'
