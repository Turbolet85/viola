proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/metric-coverage.md
    near: 24
    section: "## 5. Metric Coverage"
    change: "In the `wait` outcome row's Derived metric cell, after `channel-response{method:\"wait\"}.outcome`, add: «— a `wait` refused before any frame has no `channel-response` and is not in this count: the CLI's server-verification mismatch (Linux and Windows, as built at chunk 2026-10-26-server-verification-before-any-frame) is one `process-exit{subject:\"self\", exit_code:21, detail:\"server-verify-failed\", during:\"connect\"}` at WARN in `cli-<name>.ndjson`, counted under the Counter row's `detail` grouping; MCP's `wait` tool writes no line for it»."
    sidecar: "metric-coverage §5: the `wait` outcome counter's source scoped — a pre-frame server-verification refusal writes no `channel-response`; it counts as `process-exit` `server-verify-failed` (CLI) or not at all (MCP) (chunk 2026-10-26-server-verification-before-any-frame)."
    rationale: "Report Changes › Schema / config: this build is the first to WRITE `server-verify-failed`, as `process-exit{subject:\"self\", exit_code:21, detail:\"server-verify-failed\", during:\"connect\"}` in `cli-<name>.ndjson`, and MCP writes no line for the mismatch; Decisions & corrections (inputs#I6 must-trace check): no `channel.request` span or `channel-response` line since no frame is sent; Symbols bullet: `wait` goes through `client::connect` inside `wait.block`. So `viola.wait.outcome`, derived from `channel-response{method:\"wait\"}`, silently omits the new refusal cause — the row needs the qualifier, in the style of the `viola.compat.v_rejected` over-count note at :28."
    basis: .andromeda/obs-plan-detail/metric-coverage.md:24
note: D-obs-instrumentation, D-obs-stack and D-obs-pii hold for this file (no new dependency; the new check's lines carry no pid, path or endpoint per the report's Schema / config; the generic Counter row at :10 already derives `detail` counts); no Retired item is described in this file; hook rows' perf (max 2.2–2.4 ms with the check in the timed path) belongs to SLO Invariants, not here.
