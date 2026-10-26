proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/slo-invariants-telemetry-budgets.md
    near: 28
    section: "## 10. SLO Invariants & Telemetry Budgets"
    change: "In the Spine hooks / `pre-tool-use` row's Budget assertion cell, after «obs requires only that the max sample is gated», add: «; since chunk 2026-10-26-server-verification-before-any-frame the timed hook path includes the server verification every frame-writing hook makes before its first frame (the connected endpoint's peer pid and the OS start time against the one `snapshot.json` read; macOS the recorded gap, no comparison), so the same `max <` spine-deadline gate bounds it; a failed check is fail-open (`hook-decision{detail:\"server-verify-failed\"}`, exit 0) and adds no budget row»"
    sidecar: "slo-budgets: the spine-hook perf row now states that the pre-frame server verification sits inside the timed hook path, gated by the unchanged `max <` spine-deadline assertion (chunk 2026-10-26-server-verification-before-any-frame)."
    rationale: "Report Outcome: «(tests) `run --perf`: every hook row `max < SPINE_DEADLINE`, zero panics, the check in the timed path — MET»; Symbols bullet for `src/cmd/hook.rs`: a shared `connect` (`read_snapshot` → `connect_by` → `verify_server`) used by `deliver` (event) and `ask` (dialog); Schema / config: `hook-decision{detail:\"server-verify-failed\"}` at WARN. The row describes what the spine gate times; the timed path gained the check this chunk, the assertion itself unchanged."
    basis: viola-0.1.0/chunks/2026-10-26-server-verification-before-any-frame/report.md:111
note: "No D-obs-instrumentation / D-obs-stack / D-obs-pii / D-retired drift in this file: the new pre-frame refusals write their lines after init step 4 (no new panic exemption), no dependency moved, no pid/path/endpoint is logged, and the Retired dated gaps are not stated here; the measured 2.2–2.4 ms hook maxima are report-only per this row and are not proposed into the master."
