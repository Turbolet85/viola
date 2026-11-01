proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/obs-anti-patterns.md
    near: 8
    section: '## 11. Obs Anti-Patterns (NEVER do these) › ### Logs'
    change: 'In the human::refuse census, the callers read `run` 8 · `verify` 6 · `list` 2 · `revive` 1 · `plugin install` 1 · `client` 1, in place of `run` 7; the rest of the line stands.'
    sidecar: 'Obs Anti-Patterns, Logs: the human::refuse census moves to `run` 8 (19 call sites under src/), the new site being refuse_socket_dir, chunk 2026-11-01-unix-endpoint-and-home-hardening.'
    rationale: 'The report moves human::refuse( under src/ 18 to 19 and `run` 7 to 8 (report.md:34), the new call being refuse_socket_dir in src/cmd/run.rs (report.md:14); the other five callers hold their figures.'
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:34
note: 'Outside this file, obs-plan.md:311 carries the same census as `run` (7 sites), which report.md:34 names; the D-obs-instrumentation, D-obs-stack and D-obs-pii invariants are not violated in this category text.'
