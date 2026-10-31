proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/metric-coverage.md
    near: 28
    section: '## 5. Metric Coverage'
    change: '| `unknown` fallback / higher-`v` rejections | `viola.compat.unknown` ← `send-refused{refusal:"unknown"}`; `viola.compat.v_rejected` ← `channel-response{error_code:-32602, detail:"unsupported-version"}`, version rejections alone: invalid params on `send` and a bad `link` / `unlink` name or pair read `detail` `invalid-params`, and `release` carrying `from` reads `release-from-driver` | counter | 1 | n/a |'
    sidecar: 'metric-coverage: `viola.compat.v_rejected` derives from `channel-response` with `-32602` and `detail` `unsupported-version`; the over-count caveat is retired.'
    rationale: 'Report, Expected amendments, obs-plan §5, and Schema / config: the line now tells the three `-32602` causes apart, which is the condition the caveat named as its end.'
    basis: .andromeda/obs-plan-detail/metric-coverage.md:28
    fact: F3
  - detector: D-cascade
    severity: warning
    file: .andromeda/obs-plan.md
    near: 418
    section: '## 5. Metric Coverage'
    change: '- `viola.compat.v_rejected` counts version rejections alone: it reads `channel-response{error_code:-32602, detail:"unsupported-version"}`, and a `-32602` for invalid params on `send`, `release` carrying `from` or a bad `link` / `unlink` name or pair carries another `detail` word.'
    sidecar: 'obs-plan §5: the broad line on `viola.compat.v_rejected` no longer says it over-counts.'
    rationale: 'Report, Expected amendments, obs-plan §5: pattern `v-rejected`, 2 standing and 2 changed, `obs-plan.md:418` and `metric-coverage.md:28`.'
    basis: .andromeda/obs-plan.md:418
    dependent-of: D-new-detail
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/log-coverage.md
    near: 13
    section: '## 6. Log Coverage'
    change: 'In the row, `detail` reads: `detail` (`unsupported-version|invalid-params|release-from-driver`, closed by `schemas/diag-line.v1.json`: the three `-32602` faults, on the server line and the client line alike, and absent on every other `channel-response` line); every other field of the row stands.'
    sidecar: 'log-coverage: `channel-response` `detail` is the closed set of three words on the `-32602` faults, absent otherwise.'
    rationale: 'Report, Schema / config and Symbols: `schemas/diag-line.v1.json:42` closes the field to the enum, and `ResponseLine`, by its `Drop`, writes `fault.and_then(ProtocolError::cause)` on both lines at an unchanged level; pattern `response-line` names `log-coverage.md:13` as changed.'
    basis: .andromeda/obs-plan-detail/log-coverage.md:13
    fact: F1
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/error-capture-reporting.md
    near: 17
    section: '## 7. Error Capture & Reporting'
    change: '  - **`mcp` at the catch site:** the `mcp` role writes no self `process-exit` line: `internal_error_exit_line` in `src/obs.rs` covers `Run` and `Cli` only, so the `mcp` catch site writes none.'
    sidecar: 'error-capture-reporting: the `mcp` role writes no self `process-exit` line at its catch site.'
    rationale: 'Report, Spec claims disproved, `mcp` writing a `process-exit` line: `src/obs.rs:232` `internal_error_exit_line` writes for `Run` and `Cli` only, and a grep of `ObsEvent::ProcessExit` shows no self line on the `mcp` role.'
    fact: F7
  - detector: D-cascade
    severity: warning
    file: .andromeda/obs-plan-detail/error-capture-reporting.md
    near: 39
    section: '## 7. Error Capture & Reporting'
    change: 'The clause reads: the session ends as at a closed stdin, with process exit 0 and no self `process-exit` line once `initialize` has completed, or process exit 1 when the cut lands before it (`ServeError::Initialize`, whose 11 is the code of the unit child alone, `serve_child_entry`; `src/main.rs` `exit_code` gives `Role::Mcp` + `Failed` exit 1); the rest of the line stands.'
    sidecar: 'error-capture-reporting: the stdin clause corrects exit 11 to process exit 1 and states no self `process-exit` line on the `mcp` role.'
    rationale: 'Report, Spec claims disproved: `mcp_tools_a_line_past_max_frame_before_initialize_exits_1_with_one_line` reads exit 1; patterns `exit-eleven` and `mcp-exit-line` both name `error-capture-reporting.md:39` as changed.'
    basis: .andromeda/obs-plan-detail/error-capture-reporting.md:39
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/error-capture-reporting.md
    near: 39
    section: '## 7. Error Capture & Reporting'
    change: '- A `viola mcp` stdin line past `MAX_FRAME` before `initialize` gives process exit 1, both streams empty and exactly one `parse-rejected{parser:"mcp-stdin", detail:"oversize", count:1}`.'
    sidecar: 'error-capture-reporting: the measured outcome of an oversize `viola mcp` stdin line before `initialize`.'
    rationale: 'Report, Spec claims disproved: `mcp_tools_a_line_past_max_frame_before_initialize_exits_1_with_one_line` reads exit 1, both streams empty, one `parse-rejected{mcp-stdin, oversize, count:1}`.'
    basis: .andromeda/obs-plan-detail/error-capture-reporting.md:39
    fact: F6
  - detector: D-cascade
    severity: warning
    file: .andromeda/obs-plan.md
    near: 527
    section: '## 7. Error Capture & Reporting'
    change: '    - A main-thread panic reaches the main-thread catch site. The process writes no `process-exit` line (`obs::internal_error_exit_line` covers `Run` and `Cli` only) and exits 1. `mcp` writes nothing to stdout on this path.'
    sidecar: 'obs-plan §7: the `mcp` main-thread branch writes no `process-exit` and exits 1.'
    rationale: 'Report, Expected amendments, obs-plan §7: `mcp` catch site writes no `process-exit`; Spec claims disproved measures `internal_error_exit_line` writing for `Run` and `Cli` only; the `mcp-exit-line` sweep did not list this line.'
  - detector: D-retired
    severity: warning
    file: .andromeda/obs-plan.md
    near: 496
    section: '## 7. Error Capture & Reporting'
    change: 'The thiserror list reads: thiserror 2.0.20 on `AgentError`, `ChannelError`, `StateError`, `PinError`, `StampsError`, `StampsReadError`, `Refused`, `McpError` and `ServeError`; `CoreError` leaves it and the rest of the line stands.'
    sidecar: 'obs-plan §7: `CoreError`, which never existed in the code, leaves the thiserror list, and the list names the nine derives.'
    rationale: 'Report, Retired and Spec claims disproved: `CoreError` has 0 hits in the code; Counts read 9 derives; pattern `enum-rule` names `obs-plan.md:496` as changed.'
    basis: .andromeda/obs-plan.md:496
note: obs-plan.md:496 and :527 are §7 broad lines no picked entity details directly and are proposed from the report and task Touches; F4, F5, F8, F9, F10 and F11 fit no picked entity and stay for the new-detail pass; obs-plan.md:479 is not proposed, since its field list never named `refusal`, `outcome` or `duration_ms` either; the leaf .claude/rules/observability.md:36 is another pass.
