proposals:
  - detector: D-obs-instrumentation
    severity: warning
    section: '## 5. Metric Coverage → Per-surface / per-path metrics'
    change: '`viola.compat.v_rejected` counts `channel-response{error_code:-32602, detail:"unsupported-version"}` only: the line now tells the three `-32602` causes apart by its closed `detail` (`unsupported-version` · `invalid-params` · `release-from-driver`), so the other two words do not count and the over-count is gone.'
    sidecar: '§5 `viola.compat.v_rejected` reads `channel-response{error_code:-32602, detail:"unsupported-version"}`; the over-count clause retired now that the line names its cause.'
    rationale: 'Report Changes, Symbols / APIs: `ResponseLine` Drop writes `detail = fault.and_then(ProtocolError::cause)` on both sides, `ProtocolError::cause` gives the three words for the three `-32602` faults; Schema / config closes the field to that enum; Expected amendments names this edit (`v-rejected`, `obs-plan.md:418`). The body still says the metric over-counts until the line tells the causes apart, which this chunk delivered.'
    basis: 'schemas/diag-line.v1.json:42'
  - detector: D-retired
    severity: warning
    section: '## 7. Error Capture & Reporting → Platform pick'
    change: 'The thiserror 2.0.20 list reads `AgentError`, `ChannelError`, `StateError`, `PinError`, `StampsError`, `StampsReadError`, `Refused`, `McpError` and `ServeError` (the nine derives over product sources); `CoreError` leaves the list, the rest of the sentence unchanged.'
    sidecar: '§7 Platform pick: thiserror enum list corrected to the census of nine; `CoreError`, which never existed in the code, removed.'
    rationale: 'Report Retired names `CoreError` as listed by obs-plan and never existing in the code; Spec claims disproved measures 0 hits at `obs-plan.md:496`; Counts / qualifiers moved gives the nine derives after `Refusal` folded and marks `obs-plan.md:496` changed under `enum-rule`; Expected amendments lists the §7 enum list.'
    basis: '.andromeda/obs-plan.md:496'
  - detector: D-obs-instrumentation
    severity: warning
    section: '## 7. Error Capture & Reporting → Panic hooks → Per-role behaviour, `mcp`'
    change: 'A main-thread panic reaches the main-thread catch site and the process exits 1 with no `process-exit` line, since `obs::internal_error_exit_line` writes for `Run` and `Cli` only; its record is the panic hook line; `mcp` writes nothing to stdout on this path.'
    sidecar: '§7 `mcp` per-role behaviour: the main-thread catch site writes no `process-exit`; the claimed `process-exit{detail:"internal-error"}` retired.'
    rationale: 'Report Spec claims disproved, `mcp` writing a `process-exit` line: `src/obs.rs:232` `internal_error_exit_line` writes for `Run` and `Cli` only, and no self `process-exit` is written on the `mcp` role; Expected amendments lists §7 `mcp` catch site writes no `process-exit`. The body bullet still states the line is written.'
    basis: 'src/obs.rs:232'
note: The `mcp` catch-site bullet in §7 body fell outside the report's `mcp-exit-line` pattern (it names `obs-plan.md:523` as no-change, not this bullet) yet carries the disproved claim, so the proposal rests on the Spec claims and Expected amendments bullets; the §6 three `detail` words and the §5 / §7 detail edits at `metric-coverage.md:28`, `log-coverage.md:13`, `error-capture-reporting.md:39` are the cascade's and are not proposed here; D-obs-stack and D-obs-pii read clean (no dependency moved, `channel-response.detail` a closed enum with no param value or name).
