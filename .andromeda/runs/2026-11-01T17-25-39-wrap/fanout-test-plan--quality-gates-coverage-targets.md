proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/quality-gates-coverage-targets.md
    near: 19
    section: "## 10. Quality Gates & Coverage Targets"
    change: New item of its own after this one, inside the gate-mutation block. - **A survivor by construction (chunk 2026-11-01-unix-endpoint-and-home-hardening):** a mutant replacing a client seam's `peer_admitted` read with an always-admit (`src/cmd/client.rs` `connect`, `src/cmd/hook.rs` `connect`, `crates/viola-mcp/src/tools.rs` `frame`) survives every committed test, because one account can make no foreign peer and no seam may be added (security-plan §Security Anti-Patterns, Universal). Each seam's wiring was read once, through a forced refusing decision in that chunk's remove-the-guard runs, which no committed test repeats; the decisions over injected euids and the wrapper's drop over an injected decision are committed tests. The mutation audit reads this survivor at "Version done-check".
    sidecar: test-plan §10 gate-mutation records the euid-wiring survivor at the three client seams, read by the audit at "Version done-check" (chunk 2026-11-01-unix-endpoint-and-home-hardening).
    rationale: The report's Insufficient fixes names a mutant that survives every committed test by construction and says the mutation audit reads it at "Version done-check". This file holds the mutation gate and its named survivors, and states no such case.
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:54
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/quality-gates-coverage-targets.md
    near: 26
    section: "## 10. Quality Gates & Coverage Targets"
    change: New continuation line after this one, indented as it is. Since chunk 2026-11-01-unix-endpoint-and-home-hardening each timed hook's `connect` also runs the socket-directory check (`check_endpoint_dir`, before it connects) and the peer euid decision (`peer_admitted`, after it connects), both before its first frame and each a real check on Unix only; with those two in the timed path the five hook rows read a mean of 2.0, 2.0, 2.0, 2.0 and 2.3 ms and a highest `max` of 2.8 ms, under `SPINE_DEADLINE`, as measured at chunk 2026-11-01-unix-endpoint-and-home-hardening (report, Outcome, `run --perf`). The `max` reading above stays the record of its own chunk, and no gate value moves.
    sidecar: test-plan §10 gate-perf names the socket-directory check and the peer euid decision in the timed hook path, with that chunk's reading; no value moved (chunk 2026-11-01-unix-endpoint-and-home-hardening).
    rationale: The report puts both new checks in the timed path and gives the new reading, and states the existing band stays a true record with no value moved, so this adds a line and edits none. This item names only the two earlier checks.
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:38
note: The gate's `missed == 0` (line 15) has no carve-out for a survivor that is reachable but not killable, as the euid-wiring mutant is; the report rules its owner (`matrix#v1-44` at P7.3 and test-plan §1's untestable zone) and no gate rule, so none is proposed and the audit's reading at "Version done-check" stays open.
