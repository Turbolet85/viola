# Cascade compare — obs-plan · wrap · 2026-10-26-server-verification-before-any-frame
E items 10 · held 6 · lost at classify 2 · lost at strong 0 · beside a placed fact 2 · cascade only 17 · listed - · body 0 · unplaced 0

## Lost — the full read carried these and the cascade did not; the wrap validates each, marked «(sampler)»

### metric-coverage#2.5 · lost at classify · .andromeda/obs-plan-detail/metric-coverage.md:24
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/metric-coverage.md
    near: 24
    section: "## 5. Metric Coverage"
    change: "In the `wait` outcome row's Derived metric cell, after `channel-response{method:\"wait\"}.outcome`, add: «— a `wait` refused before any frame has no `channel-response` and is not in this count: the CLI's server-verification mismatch (Linux and Windows, as built at chunk 2026-10-26-server-verification-before-any-frame) is one `process-exit{subject:\"self\", exit_code:21, detail:\"server-verify-failed\", during:\"connect\"}` at WARN in `cli-<name>.ndjson`, counted under the Counter row's `detail` grouping; MCP's `wait` tool writes no line for it»."
    sidecar: "metric-coverage §5: the `wait` outcome counter's source scoped — a pre-frame server-verification refusal writes no `channel-response`; it counts as `process-exit` `server-verify-failed` (CLI) or not at all (MCP) (chunk 2026-10-26-server-verification-before-any-frame)."
    rationale: "Report Changes › Schema / config: this build is the first to WRITE `server-verify-failed`, as `process-exit{subject:\"self\", exit_code:21, detail:\"server-verify-failed\", during:\"connect\"}` in `cli-<name>.ndjson`, and MCP writes no line for the mismatch; Decisions & corrections (inputs#I6 must-trace check): no `channel.request` span or `channel-response` line since no frame is sent; Symbols bullet: `wait` goes through `client::connect` inside `wait.block`. So `viola.wait.outcome`, derived from `channel-response{method:\"wait\"}`, silently omits the new refusal cause — the row needs the qualifier, in the style of the `viola.compat.v_rejected` over-count note at :28."
    basis: .andromeda/obs-plan-detail/metric-coverage.md:24

### obs-scope-summary#35.1 · lost at classify · beside a placed fact F8 (near 454, obs-scope-summary#44.1) · .andromeda/obs-plan-detail/obs-scope-summary.md:370
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/obs-scope-summary.md
    near: 370
    section: '## 1. Obs Scope Summary'
    change: 'Append to the fail-open bullet: As landed (chunk 2026-10-26-server-verification-before-any-frame) a server-verify mismatch is logged on Linux and Windows as one `hook-decision{detail:"server-verify-failed"}` at WARN in `hook-<name>.ndjson` (a dialog hook writes `hook-invoked{corr:null}` then `hook-decision{corr:null, decision_emitted:false, detail:"server-verify-failed"}`), exit 0, streams empty, no frame, and no pid, start time, path or endpoint in the line; macOS compares no peer pid (the recorded gap).'
    sidecar: 'obs-scope-summary Vector 4 trigger: the server-verify mismatch fail-open path recorded as landed (hook-decision server-verify-failed at WARN, Linux and Windows), chunk 2026-10-26-server-verification-before-any-frame.'
    rationale: 'Report Changes, Schema / config: this build is the first that writes `hook-decision{detail:"server-verify-failed"}` (dialog path `hook-invoked{corr:null}` then `hook-decision{corr:null, decision_emitted:false, ...}`), no pid, start time, path or endpoint asserted; Coverage of new surfaces, `viola hook` row; Retired: macOS stays on the recorded gap. The file lists the mismatch as a required fail-open signal with an as-landed note only for strict-modes; the landed shape of the mismatch line is new detail.'
    basis: .andromeda/obs-plan-detail/obs-scope-summary.md:370

### slo-invariants-telemetry-budgets#2.2 · lost at classify · .andromeda/obs-plan-detail/slo-invariants-telemetry-budgets.md:28
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/slo-invariants-telemetry-budgets.md
    near: 28
    section: "## 10. SLO Invariants & Telemetry Budgets"
    change: "In the Spine hooks / `pre-tool-use` row's Budget assertion cell, after «obs requires only that the max sample is gated», add: «; since chunk 2026-10-26-server-verification-before-any-frame the timed hook path includes the server verification every frame-writing hook makes before its first frame (the connected endpoint's peer pid and the OS start time against the one `snapshot.json` read; macOS the recorded gap, no comparison), so the same `max <` spine-deadline gate bounds it; a failed check is fail-open (`hook-decision{detail:\"server-verify-failed\"}`, exit 0) and adds no budget row»"
    sidecar: "slo-budgets: the spine-hook perf row now states that the pre-frame server verification sits inside the timed hook path, gated by the unchanged `max <` spine-deadline assertion (chunk 2026-10-26-server-verification-before-any-frame)."
    rationale: "Report Outcome: «(tests) `run --perf`: every hook row `max < SPINE_DEADLINE`, zero panics, the check in the timed path — MET»; Symbols bullet for `src/cmd/hook.rs`: a shared `connect` (`read_snapshot` → `connect_by` → `verify_server`) used by `deliver` (event) and `ask` (dialog); Schema / config: `hook-decision{detail:\"server-verify-failed\"}` at WARN. The row describes what the spine gate times; the timed path gained the check this chunk, the assertion itself unchanged."
    basis: viola-0.1.0/chunks/2026-10-26-server-verification-before-any-frame/report.md:111

### span-trace-coverage#1.3 · lost at classify · beside a placed fact F1 (near 18, span-trace-coverage#2.1) · .andromeda/obs-plan-detail/span-trace-coverage.md:10
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/span-trace-coverage.md
    near: 10
    section: '## 4. Span / Trace Coverage'
    change: '- Pre-frame server check (as landed, chunk 2026-10-26-server-verification-before-any-frame; Linux and Windows, macOS on the recorded gap): every frame-writing client verifies the serving process (the connected stream''s peer pid and its start time against the one snapshot read) before its first frame, with no span of its own. The six CLI frame verbs run it inside their dispatch span (`send.client`, `wait.block`, `last.client`, `answer.client`, `pause.client`, `release.client`), after their own exit-13 refusals; a mismatch or an unmakeable check opens no `channel.request` span, writes no `channel-request` line and ends `process-exit{subject:"self", exit_code:21, detail:"server-verify-failed", during:"connect"}` at WARN in `cli-<name>.ndjson`. `viola hook` runs it inside `hook.handle` for both `hook.event` and `hook.dialog`: no `channel.request`, `hook-decision{detail:"server-verify-failed"}` at WARN in `hook-<name>.ndjson` (dialog path: `hook-invoked{corr:null}` then `hook-decision{corr:null, decision_emitted:false, detail:"server-verify-failed"}`), exit 0 with empty streams; a SessionEnd whose check fails takes the direct append. `viola mcp`''s four frame tools run it inside `mcp.tool_call` after the connect and write no line for the mismatch, as on their unreachable path. No pid, start time, path or endpoint reaches any line. The strict-modes half (`strict-modes-failed`) stays owed to "Home and code-bearing file integrity".'
    sidecar: 'span-trace-coverage §4 spans: new item for the pre-frame server check — where it runs (six CLI dispatch spans, hook.handle, mcp.tool_call), its outcome lines (cli process-exit 21 server-verify-failed during connect; hook-decision server-verify-failed; MCP no line), no channel.request on a mismatch.'
    rationale: 'Report Symbols / APIs (src/cmd/{send,wait,last,answer,pause,release}.rs check inside the dispatch span; src/cmd/hook.rs shared connect for deliver and ask; crates/viola-mcp/src/tools.rs server_verified), Schema / config (first build to write server-verify-failed, no pid/path in any line), Coverage of new surfaces and the must-trace check in Decisions & corrections (no channel.request span or channel-request line, since no frame is sent).'
    basis: viola-0.1.0/chunks/2026-10-26-server-verification-before-any-frame/report.md:21

## Held

- log-coverage#2.2
- log-coverage#2.4
- obs-scope-summary#48.1
- span-trace-coverage#11.1
- span-trace-coverage#4.4
- span-trace-coverage#5.3

## Cascade only

- obs-scope-summary#14.3
- obs-scope-summary#25.0
- obs-scope-summary#30.3
- obs-scope-summary#44.1
- pii-scrubbing-compliance#2.1
- span-trace-coverage#2.1
- span-trace-coverage#2.3
- span-trace-coverage#2.5
- span-trace-coverage#4.1
- span-trace-coverage#4.3
- span-trace-coverage#5.1
- span-trace-coverage#5.4
- span-trace-coverage#6.1
- span-trace-coverage#6.3
- span-trace-coverage#7.1
- span-trace-coverage#7.3
- telemetry-strategy#3.3
