proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/metric-coverage.md
    near: 36
    section: '## 5. Metric Coverage'
    change: 'After the line-36 item, add one item: a CLI `wait` refused by the instance strict-modes check (`client::refuse_untrusted`, exit 21, the instance pair or the `instance-unreachable` document) writes no line at all, neither a `process-exit` nor a `channel-request`, so neither `viola.wait.outcome` nor the Counter row''s `detail` grouping counts it; MCP''s `wait` tool refused by its `check_instance` (`instance-unreachable`, `detail: null`) writes none either; the exit code or the `isError` code is its only record.'
    sidecar: 'metric-coverage: the strict-modes refusal of CLI and MCP `wait` writes no line, so no derived metric counts it (chunk 2026-10-26-home-and-code-bearing-file-integrity, the founder''s no-line word inputs#I3 Q2).'
    rationale: 'The report''s Changes add `refuse_untrusted` at `wait` (wait.rs:43-45) and the MCP frame tools'' `check_instance` (tools.rs:337-339), each refusing with no line (Coverage: instrumentation n/a, no line by the founder''s word, inputs#I3 Q2); the file names the server-verify refusal''s `process-exit` as the counted record of a pre-frame `wait` refusal, and the new no-line refusal is a pre-frame refusal no metric can see.'
    basis: viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:15
note: 'metric-coverage.md:25 (`viola.state.recovered`) needs no change for the four new replay readers (report Count 6 lists it no-change: the event and its detail words are unchanged); no row derives a metric from `process-start.sideload_fallback`, so its new word moves nothing here; D-obs-stack and D-obs-pii find nothing (no dependency, no new logged field).'
