proposals:
  - detector: D-obs-instrumentation
    severity: warning
    file: .andromeda/obs-plan-detail/span-trace-coverage.md
    near: 91
    section: '## 4. Span / Trace Coverage › #### Scenario: Confirmed `send` (CL-1) from driver to readback'
    change: '  - `channel-response{corr:<id>, conn, result_class, refusal, duration_ms}` on both sides, plus `detail` on a `-32602` reply alone (`invalid-params`, or `unsupported-version` for a too-new request);'
    sidecar: 'span-trace-coverage, Scenario Confirmed send: `channel-response` no longer lists `detail` as a general field; it is carried on a `-32602` reply alone (chunk 2026-10-31-sanitised-error-surfaces).'
    rationale: 'Report Changes, Schema / config: `channel-response` `detail` closes to `unsupported-version` · `invalid-params` · `release-from-driver` and is absent on every other `channel-response` line; Symbols: `ResponseLine` Drop writes `detail = fault.and_then(ProtocolError::cause)`. A refusal line with `detail` no longer holds.'
    basis: schemas/diag-line.v1.json:42
  - detector: D-obs-instrumentation
    severity: warning
    file: .andromeda/obs-plan-detail/span-trace-coverage.md
    near: 132
    section: '## 4. Span / Trace Coverage › #### Scenario: Dialog → `answer` (question / permission / plan), including the unverified-CLI branch'
    change: '  - driver-side refusals: `channel-response{result_class:"refusal", refusal:"not-delivered"}` (exit 13), or `refusal:"unverified-cli"` (exit 12, answer withheld); no `detail` on either line.'
    sidecar: 'span-trace-coverage, Scenario Dialog: `detail:"unknown-dialog"` removed from the driver-side refusal `channel-response` (chunk 2026-10-31-sanitised-error-surfaces).'
    rationale: 'Same retired claim as the Confirmed send item: `unknown-dialog` is outside the closed enum the report states for `channel-response` `detail`, and the report reads that field absent on every line but the three `-32602` faults (Changes, Schema / config).'
    basis: schemas/diag-line.v1.json:42
    dependent-of: D-obs-instrumentation
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/span-trace-coverage.md
    near: 21
    section: '## 4. Span / Trace Coverage'
    change: '| ipc-internal (viola-channel) | none (interprocess 2.4.4 emits no `log`/`tracing`) | `channel.request` (client) / `channel.dispatch` (server) spans. `channel-request` / `channel-response` with `method`, `corr`, `conn`, `result_class`, `error_code`, `sender`; a `-32602` `channel-response` alone adds `detail` (`unsupported-version` · `invalid-params` · `release-from-driver`, `ProtocolError::cause`), on both sides at the unchanged WARN. Never `params` or result bodies |'
    sidecar: 'span-trace-coverage, ipc-internal (viola-channel) row: the `-32602` `channel-response` `detail` and its three closed words added (chunk 2026-10-31-sanitised-error-surfaces).'
    rationale: 'Report Changes, Symbols: `ProtocolError::cause` and `ResponseLine` Drop on both the server and the client line, level unchanged; Coverage of new surfaces: `channel-response.detail` closed enum, no param value or name.'
    basis: schemas/diag-line.v1.json:42
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/span-trace-coverage.md
    near: 149
    section: '## 4. Span / Trace Coverage › #### Scenario: Human takes the wheel, automation is refused, `release` returns it'
    change: '  - `channel-request` / `channel-response{method:"release"}`, the `channel-response` of a `-32602` reply adding `detail`: `release-from-driver` for the string-`from` refusal, `invalid-params` for a plain `-32602`; this word on `channel-response` is apart from the `release-from-driver` event below;'
    sidecar: 'span-trace-coverage, Scenario Human takes the wheel: the two `detail` words on a `release` `-32602` `channel-response` added (chunk 2026-10-31-sanitised-error-surfaces).'
    rationale: 'Report Changes, Symbols: `ProtocolError::cause` names the three `-32602` faults and `fault_of` reads `data` for `-32602` (`supported` · `reason` · neither), so the `release` causes are now told apart on the line.'
    basis: schemas/diag-line.v1.json:42
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/span-trace-coverage.md
    near: 32
    section: '## 4. Span / Trace Coverage'
    change: '- **The clients, a peer reply:** CLI `reply_of` and MCP `of_reply` screen a reply through `viola_core::is_code_word` and `viola_channel::sanitised_fault`, adding no span and no line, as landed (chunk 2026-10-31-sanitised-error-surfaces).'
    sidecar: 'span-trace-coverage: the reply screen of `reply_of` and `of_reply` recorded as adding no span and no line (chunk 2026-10-31-sanitised-error-surfaces).'
    rationale: 'Report Changes, Coverage of new surfaces: the `reply_of` / `of_reply` screen reads instrumentation n/a (no line added), PII redacted; the file records no-span operations as landed elsewhere in §4.'
note: 'The report counts `response-line` 45 no-change, yet lines 91 and 132 of .andromeda/obs-plan-detail/span-trace-coverage.md state `detail` on `channel-response`, so that sweep verdict needs review; outside this file, .andromeda/obs-plan.md:527 still says the `mcp` main-thread catch site writes `process-exit`, which the report disproves, and its `mcp-exit-line` no-change list does not name that line.'
