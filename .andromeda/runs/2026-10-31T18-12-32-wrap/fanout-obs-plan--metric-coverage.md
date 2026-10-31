proposals:
  - detector: D-retired
    severity: warning
    file: .andromeda/obs-plan-detail/metric-coverage.md
    near: 37
    section: '## 5. Metric Coverage'
    change: 'The closing clause of the line reads: MCP''s `wait` tool refused by its `check_instance` (`instance-unreachable`, `detail: "strict-modes-failed"`) writes none either. `detail: null` goes; the rest of the line stands.'
    sidecar: 'Metric Coverage: MCP `wait` refused by `check_instance` now reads `detail: "strict-modes-failed"`, the retired `detail: null` removed (chunk 2026-10-31-exit-cause-code-catalogue).'
    rationale: 'The report''s Retired bullet names the exit-21 document''s `detail: null`; its Symbols / APIs MCP bullet says a refused `check_instance` is `StrictModes`, no line written and `isError` the record, so the no-line claim stands and only the value moves.'
    basis: .andromeda/obs-plan-detail/metric-coverage.md:37
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/metric-coverage.md
    near: 37
    section: '## 5. Metric Coverage'
    change: 'New bullet after the line: - A CLI `wait` past that check reads each other exit-21 cause as its own `detail`, each one `process-exit` the Counter row''s `detail` grouping counts apart: no endpoint reads `instance-dead` or `unwrapped-name` at `during:"connect"`, and a wrapper lost with the call open reads `endpoint-lost` at `during:"call"`. MCP''s `wait` tool writes no line for any of them, its `isError` body naming the cause.'
    sidecar: 'Metric Coverage: the Counter row''s `detail` grouping now tells a CLI `wait`''s no-endpoint and lost causes apart, MCP''s still lineless (chunk 2026-10-31-exit-cause-code-catalogue).'
    rationale: 'The report''s Symbols / APIs bullets (the six no-endpoint arms, the process-exit `during`, MCP) and its Schema / config bullet admit these words to `process-exit`, and its Coverage of new surfaces marks the exit-21 causes instrumented by `process-exit` and the MCP causes by no line; the file names only the server-verification cause as counted by the grouping.'
    basis: .andromeda/obs-plan-detail/metric-coverage.md:36
