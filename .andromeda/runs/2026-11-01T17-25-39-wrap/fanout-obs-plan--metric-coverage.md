proposals:
  - detector: D-retired
    severity: warning
    file: .andromeda/obs-plan-detail/metric-coverage.md
    near: 36
    section: '## 5. Metric Coverage'
    change: 'In the item on `viola.wait.outcome`, replace «(Linux and Windows)» with «(Linux, Windows and macOS, where on macOS it means an euid mismatch alone)»; the rest of the item stands.'
    sidecar: 'Metric Coverage: the CLI server-verification refusal counted by the `detail` grouping now reaches macOS, as an euid mismatch alone.'
    rationale: 'The dated client euid gap is retired (report.md:26), and `server-verify-failed` now reads on Linux, Windows and macOS, every row of the pattern changing (report.md:31); the parenthesis still states the old reach.'
    basis: .andromeda/obs-plan-detail/metric-coverage.md:36
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/metric-coverage.md
    near: 36
    section: '## 5. Metric Coverage'
    change: 'New item after this line: - A CLI `wait` refused by the socket-directory check is one `process-exit{exit_code:21, detail:"socket-dir-refused", during:"connect"}` at WARN and no `channel-request` line, counted by the `detail` grouping of the Counter row and not by `viola.wait.outcome`; the MCP `wait` tool refused by the same check in `frame` returns `result::unreachable(UnreachableCause::SocketDir)`, the word `socket-dir-refused`, and writes no line, `isError` its only record.'
    sidecar: 'Metric Coverage: a `wait` refused by the socket-directory check recorded, the CLI line counted by the `detail` grouping, the MCP tool lineless.'
    rationale: 'The chunk adds the sixth exit-21 cause `socket-dir-refused` at the CLI frame verbs and at the MCP `frame` (report.md:10, report.md:15, report.md:17, report.md:71, report.md:73); this file already says which `wait` refusals each metric counts, and this one has no item.'
note: 'report.md:65 lists no §5 among the obs-plan sections carried; both proposals rest on report.md:26, report.md:31, report.md:71 and report.md:73. D-obs-stack and D-obs-pii read no drift here: no dependency moved (report.md:23), and the new lines carry no path, pid or uid (report.md:70-73).'
