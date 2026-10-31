proposals:
  - detector: D-obs-instrumentation
    severity: warning
    file: .andromeda/obs-plan-detail/metric-coverage.md
    near: 28
    section: '## 5. Metric Coverage'
    change: The row's Derived metric cell reads `viola.compat.unknown` ← `send-refused{refusal:"unknown"}`; `viola.compat.v_rejected` ← `channel-response{error_code:-32602, detail:"unsupported-version"}`, the word on both the server's and the client's line, the other `-32602` faults reading `detail` `invalid-params` or `release-from-driver`; the over-count clause and its `fault_of` clause go, every other cell unchanged.
    sidecar: Metric Coverage row for `viola.compat.v_rejected` now derives from `channel-response{error_code:-32602, detail:"unsupported-version"}`; the over-count is retired because `channel-response` tells the three `-32602` causes apart (chunk 2026-10-31-sanitised-error-surfaces).
    rationale: The report's Symbols / APIs gives `ProtocolError::cause` three words for the three `-32602` faults, `ResponseLine`'s `Drop` writing `detail` on both sides and `fault_of(&Value)` reading `data` instead of labelling any `-32602`; Schema / config closes `detail` to those words; Expected amendments asks obs-plan §5 for this derivation with the over-count retired, and Counts names `metric-coverage.md:28` under `response-line` and `v-rejected`.
    basis: .andromeda/obs-plan-detail/metric-coverage.md:28
note: The body's own over-count line, .andromeda/obs-plan.md:418, retires with this row (report Expected amendments, pattern `v-rejected`); it is outside this file, so no proposal here carries it.
