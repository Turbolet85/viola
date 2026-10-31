proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/log-coverage.md
    near: 13
    section: "## 6. Log Coverage"
    change: "In the `channel-response` row, the bare `detail` field now reads: `detail` (`unsupported-version|invalid-params|release-from-driver`, the cause of a `-32602` fault by `ProtocolError::cause`, written by `ResponseLine`'s `Drop` on both the server's and the client's line, level unchanged at WARN, the client reading the fault's `data` for it: `supported`, `reason`, neither; absent on every other `channel-response` line; no name or param on the line; `schemas/diag-line.v1.json` closes it, chunk 2026-10-31-sanitised-error-surfaces)"
    sidecar: "§6 log-fields: `channel-response`'s `detail` closes from an open string to the three `-32602` causes `unsupported-version`, `invalid-params`, `release-from-driver`, on both sides, absent on every other line, level unchanged."
    rationale: "Report Changes, Schema / config closes `detail` to the three-word enum, absent on every other `channel-response` line; Symbols has `ResponseLine`'s `Drop` write `fault.and_then(ProtocolError::cause)` on both lines at WARN; Counts' `response-line` and Expected amendments' obs-plan §6 name this line as changed; the row names `detail` with no values."
    basis: schemas/diag-line.v1.json:42
note: "Outside this file, for the §7 owner: the body's `mcp` per-role bullet (A main-thread panic reaches the main-thread catch site) still says the process writes `process-exit{detail:internal-error}`, while the report's disproved `mcp-exit-line` and Expected amendments say `mcp`'s catch site writes none, and the pattern's listed sites name only the `error-capture-reporting.md` change and the body's short-lived cli bullet."
