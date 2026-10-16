
## 2026-10-16-remove-the-browser-leg — the reserved web vocabulary leaves: the `ui` process and four events
**Section:** §1 Obs Scope Summary (tier basis, enum extensions, Telemetry surfaces, the Observability harness specification copy and Alignment gap (b), Telemetry triggers); §2 trigger map; §3 → Log format JSON schema (copy, Obs extensions, Null encoding), → Logging stack, → Log file location, → Heartbeat ticks, → OTel SDK init, → Bootstrap phases (heartbeat-tick-wire); the §3 index labels
**Change:**
- Was a reserved web half with no emitter, marked «removed with the code by "Remove the browser leg"»: the `ui` process and role file `ui-<port>`, the events `http-request`, `liveness-changed` (D-02), `sse-opened` / `sse-closed` (D-04), and the harness `a11y-violation` rows against `e2e-web/schemas/a11y-row.v1.json`. Now the role files are `run-<name>`, `hook-<name>`, `mcp`, `cli-<name>`; `process` is `run|hook|mcp|cli` (`ObsProcess::ALL`, four); the accepted `event` extensions are D-01, D-03, D-05 and `cli` (D-06).
- No process logs a liveness transition: `run` touches `heartbeat` every 1 s, and short-lived readers expose `liveness` in `list --json`, read through `agent-run status` (`state: degraded`); the flip is a heartbeat older than 5 s.
- The third-party OFF targets are `rmcp`, `portable_pty`, `sysinfo`, `interprocess` (`axum`, `tower_http`, `hyper` left).
- §1 is kept current: «4 process roles (`run` / `hook` / `mcp` / `cli`)», the count unchanged; the enum-extension item, gap (b) and the per-parser counts drop the SSE lifecycle and `Last-Event-ID`.
**Why:** This chunk removed the reserved vocabulary with the browser leg (founder ruling 2026-10-16, item 3); no product path emitted it. The Log format copy moves with test-plan §3's.
**Kept:** The §4 GUI-view scenario heading over its 0.2.0 stub; the copy's two older differences from test-plan's Log format (the `D-07` spelling, the `diag-line` `corr` rule's place), which predate this chunk.
**Ref:** .andromeda/runs/2026-10-16T17-15-05-wrap/
