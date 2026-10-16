proposals:
  - detector: D-obs-stack
    severity: warning
    section: '§3 → Logging stack'
    change: 'Sink: drop the `ui` row (`<home>/diagnostics/ui-<port>.ndjson`, reserved); the role files are `run-<name>`, `hook-<name>`, `mcp`, `cli-<name>` only, `ObsProcess::ALL` the four `run · hook · mcp · cli`.'
    sidecar: '§3 → Logging stack Sink: the reserved `ui` role file row removed with the code (chunk 2026-10-16-remove-the-browser-leg).'
    rationale: 'Report Symbols / APIs (product obs): `ObsProcess::Ui` removed, `ObsProcess::ALL` is the four; `role_file_name` / `open_role_file` lose `port`, no `ui-<port>` role file. The row said "removed with the code by Remove the browser leg", which this chunk did.'
    basis: 'src/obs.rs; crates/viola-core/src/obs.rs'
  - detector: D-obs-stack
    severity: warning
    section: '§3 → Logging stack'
    change: 'Agent-mode flag: the third-party OFF target list reads `rmcp`, `portable_pty`, `sysinfo`, `interprocess` (`axum`, `tower_http`, `hyper` left with the web leg; no such crate in the tree).'
    sidecar: '§3 → Logging stack: axum / tower_http / hyper leave the third-party OFF target list (chunk 2026-10-16-remove-the-browser-leg).'
    rationale: 'Report Symbols / APIs (src/obs.rs): the OFF-target comment no longer names axum; Schema / config: the deny.toml axum `http2` ban removed; report Expected amendments names "Logging stack''s `viola_ui` / axum / tower_http / hyper rows".'
    basis: 'src/obs.rs; deny.toml'
  - detector: D-obs-stack
    severity: warning
    section: '§3 → Log format JSON schema'
    change: 'Per-event `corr` rules: `process-start`, `process-exit`, `panic`: null; `state-recovered`, `parse-rejected`: null (D-03, D-05) — `http-request`, `liveness-changed`, `sse-opened`, `sse-closed` removed.'
    sidecar: '§3 → Log format JSON schema: the four reserved web events leave the corr rules (chunk 2026-10-16-remove-the-browser-leg).'
    rationale: 'Report Symbols / APIs: `ObsEvent::{HttpRequest, LivenessChanged, SseOpened, SseClosed}` removed, `ObsEvent::ALL` is the fifteen; Schema / config: diag-line.v1.json `$defs.event` is the fifteen. Counts: closed obs events 19 → 15.'
    basis: 'crates/viola-core/src/obs.rs; schemas/diag-line.v1.json'
  - detector: D-obs-stack
    severity: warning
    section: '§3 → Log format JSON schema'
    change: 'Remove the "Harness-side a11y rows" bullet (the a11y Playwright fixture, `e2e-web/test-results/a11y/*.ndjson`, `e2e-web/schemas/a11y-row.v1.json`): no browser leg writes such rows in 0.1.0.'
    sidecar: '§3 → Log format JSON schema: the harness-side a11y Playwright rows leave with the browser leg (chunk 2026-10-16-remove-the-browser-leg).'
    rationale: 'Report Files: `e2e-web/` deleted whole; Harness / gate surface: the browser suite steps removed from ci.yml; Symbols: `--browser` selector and the `browser` module gone. The verbatim copy must stay equal to test-plan §3 Log format (report Expected amendments: "kept equal to obs-plan''s copy").'
    basis: 'crates/viola-e2e/src/harness/run.rs; .github/workflows/ci.yml'
  - detector: D-obs-stack
    severity: warning
    section: '§3 → Log format JSON schema'
    change: 'Constraints: "No line may contain a `Cookie` header or any stripped `CLAUDE*` value. This is enforced by the secret-scan test in §6." (the GUI token and `?t=` forms dropped).'
    sidecar: '§3 → Log format JSON schema Constraints: the GUI token and `?t=` leave the secret-scan list (chunk 2026-10-16-remove-the-browser-leg, inputs#I1).'
    rationale: 'Report Symbols / APIs (secret-scan): `CRITICAL` holds the twelve claude-stripped canaries (`token-query` / `?t=` left); the `gui-token` class and `viola_<port>=` cookie-pair form are gone; the `cookie:` header form stays. I1 fork answer: every GUI secret-scan form leaves.'
    basis: 'crates/viola-e2e/src/harness/secret_scan.rs'
  - detector: D-obs-stack
    severity: warning
    section: '§3 → Log format JSON schema'
    change: 'Process logs: the file list reads `<home>/diagnostics/{run-<name>,hook-<name>,mcp,cli-<name>}.ndjson`; drop the "reserved: no emitter in 0.1.0; removed with the code by Remove the browser leg" parenthetical.'
    sidecar: '§3 → Log format JSON schema: `ui-<port>` and the reserved-vocabulary parenthetical removed (chunk 2026-10-16-remove-the-browser-leg).'
    rationale: 'Report Symbols / APIs: no `ui-<port>` role file; Schema / config: diag-line.v1.json description role-file list is `{run-<name>,hook-<name>,mcp,cli-<name>}.ndjson`.'
    basis: 'schemas/diag-line.v1.json; src/obs.rs'
    dependent-of: D-obs-stack
  - detector: D-obs-stack
    severity: warning
    section: '§3 → Log format JSON schema'
    change: 'Required fields: `process` (`run|hook|mcp|cli`; `cli` is a short-lived verb with a resolved instance, writing `cli-<name>.ndjson`, obs-plan D-06).'
    sidecar: '§3 → Log format JSON schema: `ui` leaves the `process` enum (chunk 2026-10-16-remove-the-browser-leg).'
    rationale: 'Report Symbols: `ObsProcess::ALL` is the four `run · hook · mcp · cli`; Schema / config: both schemas'' `process` enum is the four.'
    basis: 'schemas/diag-line.v1.json; schemas/diag-detail.v1.json'
    dependent-of: D-obs-stack
  - detector: D-obs-stack
    severity: warning
    section: '§3 → Log format JSON schema'
    change: 'Obs extensions, Accepted `event` values: drop the `liveness-changed` (D-02) and `sse-opened`, `sse-closed` (D-04) bullets; the accepted values are `release-from-driver`, `state-recovered`, `parse-rejected`.'
    sidecar: '§3 → Obs extensions: D-02 / D-04 event values removed with the code (chunk 2026-10-16-remove-the-browser-leg).'
    rationale: 'Report Symbols: `ObsEvent::{LivenessChanged, SseOpened, SseClosed}` removed; Schema / config: their field blocks and `$defs.liveness` removed from diag-line.v1.json.'
    basis: 'crates/viola-core/src/obs.rs; schemas/diag-line.v1.json'
    dependent-of: D-obs-stack
  - detector: D-obs-stack
    severity: warning
    section: '§3 → Log format JSON schema'
    change: 'Obs extensions: remove the "Not a product `event` value: `a11y-violation`" block (its Playwright fixture rows and `e2e-web/schemas/a11y-row.v1.json` are gone); `a11y-violation` is simply no value of the closed enum.'
    sidecar: '§3 → Obs extensions: the harness a11y-row carve-out leaves with the browser leg (chunk 2026-10-16-remove-the-browser-leg).'
    rationale: 'Report Files: `e2e-web/` deleted whole; Harness / gate surface: no browser suite in CI.'
    basis: '.github/workflows/ci.yml'
    dependent-of: D-obs-stack
  - detector: D-obs-stack
    severity: warning
    section: '§3 → Log format JSON schema'
    change: 'Null encoding: "it omits `instance` when none resolved" (drop "(always for `ui`)").'
    sidecar: '§3 → Null encoding: the `ui` aside removed (chunk 2026-10-16-remove-the-browser-leg).'
    rationale: 'Report Symbols: `ObsProcess::Ui` removed.'
    basis: 'crates/viola-core/src/obs.rs'
    dependent-of: D-obs-stack
  - detector: D-obs-stack
    severity: warning
    section: '§3 → Log file location'
    change: 'Path: `<home>/diagnostics/{run-<name>,hook-<name>,mcp,cli-<name>}.ndjson`; drop the `ui-<port>` reserved parenthetical.'
    sidecar: '§3 → Log file location: `ui-<port>` removed with the code (chunk 2026-10-16-remove-the-browser-leg).'
    rationale: 'Report Symbols (src/obs.rs): role files are `run-<name>`, `hook-<name>`, `mcp`, `cli-<name>`; no `ui-<port>`.'
    basis: 'src/obs.rs'
    dependent-of: D-obs-stack
  - detector: D-obs-stack
    severity: warning
    section: '§3 → Heartbeat ticks'
    change: 'Tick event format: no process-log line carries a liveness transition (the `liveness-changed` event and its `ui` emitter are removed); short-lived readers (`list`) expose `liveness` in `--json`, read through `agent-run status`.'
    sidecar: '§3 → Heartbeat ticks: `liveness-changed` removed; liveness is read from `list --json` only (chunk 2026-10-16-remove-the-browser-leg).'
    rationale: 'Report Symbols: `ObsEvent::LivenessChanged` and `ObsProcess::Ui` removed; Schema: `$defs.liveness` removed. The `viola list` `liveness` field is a different contract and stays (report Spec claims disproved, Decisions: sweep hazard).'
    basis: 'crates/viola-core/src/obs.rs; schemas/diag-line.v1.json'
    dependent-of: D-obs-stack
  - detector: D-obs-stack
    severity: warning
    section: '§3 → Heartbeat ticks'
    change: 'Stall detection: the agent reads `agent-run status` (`state: degraded` when any item is `stale`); drop the `agent-run logs --process ui | jq ... liveness-changed` alternative. The flip threshold stays `heartbeat_age_ms > 5000` as read by `list`.'
    sidecar: '§3 → Heartbeat ticks Stall detection: the `--process ui` log query removed (chunk 2026-10-16-remove-the-browser-leg).'
    rationale: 'Report Symbols: no `ui` process and no `liveness-changed` event exist.'
    basis: 'crates/viola-core/src/obs.rs'
    dependent-of: D-obs-stack
  - detector: D-obs-stack
    severity: warning
    section: '§3 → Bootstrap phases (derive for route / setup-project)'
    change: 'heartbeat-tick-wire: the 1 s heartbeat file is the tick; no transition emitter exists (the `liveness-changed` and `sse-opened` / `sse-closed` emitters, D-02 / D-04, left with the code at chunk 2026-10-16-remove-the-browser-leg).'
    sidecar: '§3 → Bootstrap phases heartbeat-tick-wire: the reserved emitters recorded as removed (chunk 2026-10-16-remove-the-browser-leg).'
    rationale: 'Report Symbols: the three events are removed from `ObsEvent`; the phase text promised their removal by this entry.'
    basis: 'crates/viola-core/src/obs.rs'
    dependent-of: D-obs-stack
  - detector: D-obs-stack
    severity: warning
    section: '§3 → OTel SDK init'
    change: 'Init order step 3: the role is `run` / `hook` / `mcp` / `cli` (drop `ui`).'
    sidecar: '§3 → OTel SDK init: `ui` leaves the role list (chunk 2026-10-16-remove-the-browser-leg).'
    rationale: 'Report Symbols: `ObsProcess::ALL` is the four; every `viola_obs_init` caller passes `Run` / `Hook` / `Cli`.'
    basis: 'crates/viola-core/src/obs.rs'
    dependent-of: D-obs-stack
  - detector: D-obs-stack
    severity: warning
    section: '§1 Obs Scope Summary → Telemetry surfaces'
    change: 'Intro: the tests log contract keys on `process` (`run|hook|mcp|cli`).'
    sidecar: '§1 Telemetry surfaces: `ui` replaced by `cli` in the process key list (chunk 2026-10-16-remove-the-browser-leg).'
    rationale: 'Report Symbols / Schema: the `process` enum is `run · hook · mcp · cli`.'
    basis: 'schemas/diag-line.v1.json'
    dependent-of: D-obs-stack
  - detector: D-obs-stack
    severity: warning
    section: '§1 Obs Scope Summary → Observability harness specification'
    change: 'Service identity: per line, role and instance are carried by `process` (`run|hook|mcp|cli`) and `instance`.'
    sidecar: '§1 Service identity: the `process` list drops `ui` (chunk 2026-10-16-remove-the-browser-leg).'
    rationale: 'Report Symbols: `ObsProcess::ALL` is the four.'
    basis: 'crates/viola-core/src/obs.rs'
    dependent-of: D-obs-stack
  - detector: D-obs-stack
    severity: warning
    section: '§1 Obs Scope Summary → Observability harness specification'
    change: 'Log format JSON schema copy, Process logs: writing to `<home>/diagnostics/{run-<name>,hook-<name>,mcp,cli-<name>}.ndjson`; drop the reserved `ui-<port>` / four-event parenthetical.'
    sidecar: '§1 Log format copy: `ui-<port>` and the reserved parenthetical removed (chunk 2026-10-16-remove-the-browser-leg).'
    rationale: 'Report Schema / config: diag-line.v1.json role-file list; Symbols: the four events and `Ui` removed.'
    basis: 'schemas/diag-line.v1.json'
    dependent-of: D-obs-stack
  - detector: D-obs-stack
    severity: warning
    section: '§1 Obs Scope Summary → Observability harness specification'
    change: 'Log format JSON schema copy, corr rules: `process-start`, `process-exit`, `panic`: `corr` = null (drop `http-request`).'
    sidecar: '§1 Log format copy: `http-request` leaves the corr rule (chunk 2026-10-16-remove-the-browser-leg).'
    rationale: 'Report Symbols: `ObsEvent::HttpRequest` removed.'
    basis: 'crates/viola-core/src/obs.rs'
    dependent-of: D-obs-stack
  - detector: D-obs-stack
    severity: warning
    section: '§1 Obs Scope Summary → Observability harness specification'
    change: 'Log format JSON schema copy, Required fields: `process` (`run|hook|mcp|cli`).'
    sidecar: '§1 Log format copy: `ui` leaves the `process` enum (chunk 2026-10-16-remove-the-browser-leg).'
    rationale: 'Report Schema / config: both schemas'' `process` enum is the four.'
    basis: 'schemas/diag-line.v1.json'
    dependent-of: D-obs-stack
  - detector: D-obs-stack
    severity: warning
    section: '§1 Obs Scope Summary → Observability harness specification'
    change: 'Log format JSON schema copy, Constraints: "No line may contain a `Cookie` header or any stripped `CLAUDE*` value." (drop the GUI token and `?t=`).'
    sidecar: '§1 Log format copy Constraints: GUI token and `?t=` removed (chunk 2026-10-16-remove-the-browser-leg).'
    rationale: 'Report Symbols (secret-scan): `?t=`, `gui-token` and `viola_<port>=` forms gone; `cookie:` header stays.'
    basis: 'crates/viola-e2e/src/harness/secret_scan.rs'
    dependent-of: D-obs-stack
  - detector: D-obs-stack
    severity: warning
    section: '§1 Obs Scope Summary → Observability harness specification'
    change: 'Log file location: tests require `<home>/diagnostics/{run-<name>,hook-<name>,mcp,cli-<name>}.ndjson`; drop the `ui-<port>` reserved parenthetical.'
    sidecar: '§1 Log file location: `ui-<port>` removed (chunk 2026-10-16-remove-the-browser-leg).'
    rationale: 'Report Symbols (src/obs.rs): no `ui-<port>` role file.'
    basis: 'src/obs.rs'
    dependent-of: D-obs-stack
  - detector: D-obs-stack
    severity: warning
    section: '§1 Obs Scope Summary → Observability harness specification'
    change: 'Heartbeat ticks, `run`: the heartbeat file touched every 1 s is the tick; no process logs liveness transitions (the `ui` reader and `liveness-changed` are removed; `list --json` exposes `liveness`).'
    sidecar: '§1 Heartbeat ticks: the transition-logging sentence retired (chunk 2026-10-16-remove-the-browser-leg).'
    rationale: 'Report Symbols: `ObsEvent::LivenessChanged`, `ObsProcess::Ui` removed.'
    basis: 'crates/viola-core/src/obs.rs'
    dependent-of: D-obs-stack
  - detector: D-obs-stack
    severity: warning
    section: '§1 Obs Scope Summary → Telemetry triggers'
    change: 'creator-explicit-telemetry: structured JSON process logs from every process role (`run` / `hook` / `mcp` / `cli`) under `diagnostics/`.'
    sidecar: '§1 Telemetry triggers: `ui` replaced by `cli` in the role list (chunk 2026-10-16-remove-the-browser-leg).'
    rationale: 'Report Symbols: `ObsProcess::ALL` is the four.'
    basis: 'crates/viola-core/src/obs.rs'
    dependent-of: D-obs-stack
  - detector: D-obs-stack
    severity: warning
    section: '§2 Telemetry Strategy'
    change: 'Trigger map, chaos-instrumentation row: §6 `state-recovered` / `process-exit` `exit_source` + `during`, §10 (drop `liveness-changed`).'
    sidecar: '§2 trigger map: `liveness-changed` leaves the chaos row (chunk 2026-10-16-remove-the-browser-leg).'
    rationale: 'Report Symbols: `ObsEvent::LivenessChanged` removed.'
    basis: 'crates/viola-core/src/obs.rs'
    dependent-of: D-obs-stack
  - detector: D-obs-stack
    severity: warning
    section: '§5 Metric Coverage'
    change: 'Per-surface metrics: remove the "Liveness transitions" row (`viola.state.liveness_transition` ← `liveness-changed{liveness_to}`).'
    sidecar: '§5: the liveness-transition metric row removed with its event (chunk 2026-10-16-remove-the-browser-leg).'
    rationale: 'Report Symbols: `ObsEvent::LivenessChanged` removed; no line carries `liveness_to`.'
    basis: 'schemas/diag-line.v1.json'
    dependent-of: D-obs-stack
  - detector: D-obs-stack
    severity: warning
    section: '§5 Metric Coverage'
    change: 'Cardinality discipline: grouping keys are `event`, `detail`, `refusal`, `method`, `hook_event`, `subject`, `parser`, `process` (drop `route`, `close_cause`, `liveness_to` and the "`path` ... use `route`" clause, all fields of the removed `http-request` / `sse-closed` / `liveness-changed`).'
    sidecar: '§5 cardinality discipline: the removed events'' grouping keys dropped (chunk 2026-10-16-remove-the-browser-leg).'
    rationale: 'Report Schema / config: the field blocks of `http-request`, `liveness-changed`, `sse-opened` / `sse-closed` removed from diag-line.v1.json.'
    basis: 'schemas/diag-line.v1.json'
    dependent-of: D-obs-stack
  - detector: D-obs-stack
    severity: warning
    section: '§6 Log Coverage'
    change: 'Intro: this section only adds fields and the D-01, D-03, D-05 and D-06 enum values; drop the `a11y-violation` harness-row sentence.'
    sidecar: '§6 intro: D-02 / D-04 values and the a11y-row carve-out removed (chunk 2026-10-16-remove-the-browser-leg).'
    rationale: 'Report Symbols: the D-02 / D-04 events removed from `ObsEvent`; Files: `e2e-web/` deleted.'
    basis: 'crates/viola-core/src/obs.rs'
    dependent-of: D-obs-stack
  - detector: D-obs-stack
    severity: warning
    section: '§6 Log Coverage'
    change: 'Required fields: `process`: `run|hook|mcp|cli` (D-06 for `cli`); drop the `ui` reserved parenthetical.'
    sidecar: '§6 Required fields: `ui` removed from `process` (chunk 2026-10-16-remove-the-browser-leg).'
    rationale: 'Report Symbols / Schema: `process` enum is the four.'
    basis: 'schemas/diag-line.v1.json'
    dependent-of: D-obs-stack
  - detector: D-obs-stack
    severity: warning
    section: '§6 Log Coverage'
    change: 'Additive field catalog, `process-start`: drop `port` (ui).'
    sidecar: '§6 catalog: `process-start` loses `port` (chunk 2026-10-16-remove-the-browser-leg).'
    rationale: 'Report Schema / config: diag-line.v1.json `process-start` loses `port`.'
    basis: 'schemas/diag-line.v1.json'
    dependent-of: D-obs-stack
  - detector: D-obs-stack
    severity: warning
    section: '§6 Log Coverage'
    change: 'Additive field catalog: remove the `http-request` row.'
    sidecar: '§6 catalog: `http-request` row removed (chunk 2026-10-16-remove-the-browser-leg).'
    rationale: 'Report Symbols: `ObsEvent::HttpRequest` removed; Schema: its field block removed.'
    basis: 'schemas/diag-line.v1.json'
    dependent-of: D-obs-stack
  - detector: D-obs-stack
    severity: warning
    section: '§6 Log Coverage'
    change: 'Additive field catalog: remove the `liveness-changed` (D-02) row.'
    sidecar: '§6 catalog: `liveness-changed` row removed (chunk 2026-10-16-remove-the-browser-leg).'
    rationale: 'Report Symbols: `ObsEvent::LivenessChanged` removed; Schema: its field block and `$defs.liveness` removed.'
    basis: 'schemas/diag-line.v1.json'
    dependent-of: D-obs-stack
  - detector: D-obs-stack
    severity: warning
    section: '§6 Log Coverage'
    change: 'Additive field catalog: remove the `sse-opened` / `sse-closed` (D-04) row.'
    sidecar: '§6 catalog: `sse-opened` / `sse-closed` row removed (chunk 2026-10-16-remove-the-browser-leg).'
    rationale: 'Report Symbols: `ObsEvent::{SseOpened, SseClosed}` removed; Schema: their field blocks removed.'
    basis: 'schemas/diag-line.v1.json'
    dependent-of: D-obs-stack
  - detector: D-obs-stack
    severity: warning
    section: '§6 Log Coverage'
    change: 'Additive field catalog, `parse-rejected`: the `parser` enum drops `last-event-id` (`claude-agents-json|vt100-feed|config-json|ledger-stamps|hook-stdin|channel-frame|prompt-submitted|paste-text|state-entry`).'
    sidecar: '§6 catalog: `parse-rejected.parser` loses `last-event-id` (chunk 2026-10-16-remove-the-browser-leg).'
    rationale: 'Report Schema / config: diag-line.v1.json `parse-rejected`''s `parser` enum loses `last-event-id`.'
    basis: 'schemas/diag-line.v1.json'
    dependent-of: D-obs-stack
  - detector: D-obs-stack
    severity: warning
    section: '§6 Log Coverage'
    change: 'Log levels mapping, `warn` row: drop "`liveness-changed` to `stale`/`gone`" and "`http-request` status ≥ 500".'
    sidecar: '§6 log levels: the removed events leave the warn row (chunk 2026-10-16-remove-the-browser-leg).'
    rationale: 'Report Symbols: both events removed from `ObsEvent`.'
    basis: 'crates/viola-core/src/obs.rs'
    dependent-of: D-obs-stack
  - detector: D-obs-stack
    severity: warning
    section: '§6 Log Coverage'
    change: 'Per-module log levels: the INFO row names the seven `viola`, `viola_core`, `viola_pty`, `viola_channel`, `viola_state`, `viola_agent_claude`, `viola_mcp` (drop `viola_ui`).'
    sidecar: '§6 per-module levels: `viola_ui` leaves the targets; the row matches `VIOLA_TARGETS` (chunk 2026-10-16-remove-the-browser-leg).'
    rationale: 'Report Symbols (src/obs.rs): `VIOLA_TARGETS` is the seven `viola · viola_core · viola_pty · viola_channel · viola_state · viola_agent_claude · viola_mcp`; Counts: 8 → 7.'
    basis: 'src/obs.rs'
    dependent-of: D-obs-stack
  - detector: D-obs-stack
    severity: warning
    section: '§6 Log Coverage'
    change: 'Per-module log levels, OFF row: `rmcp`, `portable_pty`, `sysinfo`, `interprocess` (drop `axum`, `tower_http`, `hyper`).'
    sidecar: '§6 per-module levels: axum / tower_http / hyper leave the OFF row (chunk 2026-10-16-remove-the-browser-leg).'
    rationale: 'Report Symbols (src/obs.rs): the OFF-target comment no longer names axum; Schema / config: the deny.toml axum ban removed.'
    basis: 'src/obs.rs'
    dependent-of: D-obs-stack
  - detector: D-obs-stack
    severity: warning
    section: '§7 Error Capture & Reporting'
    change: 'Panic hooks: "`instance` is omitted when none resolved" (drop "(always for `ui`)").'
    sidecar: '§7 panic hooks: the `ui` aside removed (chunk 2026-10-16-remove-the-browser-leg).'
    rationale: 'Report Symbols: `ObsProcess::Ui` removed.'
    basis: 'crates/viola-core/src/obs.rs'
    dependent-of: D-obs-stack
  - detector: D-obs-stack
    severity: warning
    section: '§8 PII Scrubbing & Compliance'
    change: 'Data classification: the first row reads "`Cookie` header | Critical | never-log | secret-scan test (case-insensitive `cookie:` header form) over all `diagnostics/`" (the GUI per-launch token and `viola_<port>` cookie leave).'
    sidecar: '§8 classification: GUI token and `viola_<port>` cookie leave; the `cookie:` header stays (chunk 2026-10-16-remove-the-browser-leg, inputs#I1).'
    rationale: 'Report Symbols (secret-scan): `gui-token` class and `viola_<port>=` form gone; the `cookie` class keeps the `cookie:` header form.'
    basis: 'crates/viola-e2e/src/harness/secret_scan.rs'
    dependent-of: D-obs-stack
  - detector: D-obs-stack
    severity: warning
    section: '§8 PII Scrubbing & Compliance'
    change: 'Integration point 5, Verification: the tests secret-scan test (`Cookie` header, `CLAUDE*` values, the content canary, 0600) over every `diagnostics/` file (drop "token" and `?t=`).'
    sidecar: '§8 verification: the scan list matches the narrowed classes (chunk 2026-10-16-remove-the-browser-leg).'
    rationale: 'Report Symbols (secret-scan): CRITICAL is the twelve claude-stripped canaries; `?t=` and the GUI token gone.'
    basis: 'crates/viola-e2e/src/harness/secret_scan.rs'
    dependent-of: D-obs-stack
  - detector: D-obs-stack
    severity: warning
    section: '§8 PII Scrubbing & Compliance'
    change: '`harness-<os>` / `chunk.diff`: "repository source text by construction (it hits the scan''s own Critical-class patterns whenever it touches them)" (drop the `?t=` example).'
    sidecar: '§8: the `chunk.diff` exclusion no longer cites `?t=` (chunk 2026-10-16-remove-the-browser-leg).'
    rationale: 'Report Symbols (secret-scan): `token-query` / `?t=` left `CRITICAL`, so a diff touching `?t=` no longer hits.'
    basis: 'crates/viola-e2e/src/harness/secret_scan.rs'
    dependent-of: D-obs-stack
  - detector: D-obs-stack
    severity: warning
    section: '§8 PII Scrubbing & Compliance'
    change: 'pre-push document: neither the repository root nor the passwd home (`<home>`, `<home>/.cargo/bin`) reaches it (drop `<home>/.local/viola-node/bin`).'
    sidecar: '§8 pre-push: the viola-node PATH element removed (chunk 2026-10-16-remove-the-browser-leg).'
    rationale: 'Report Symbols (pre-push): PATH is `<home>/.cargo/bin:/usr/local/bin:/usr/bin:/bin`, the `<home>/.local/viola-node/bin` element left; Counts: PATH elements 5 → 4.'
    basis: 'crates/viola-e2e/src/harness/pre_push/linux.rs'
    dependent-of: D-obs-stack
  - detector: D-obs-stack
    severity: warning
    section: '§8 PII Scrubbing & Compliance'
    change: 'Default-deny posture: drop "That schema has no `a11y-violation` event: harness a11y rows are validated separately against the tests-owned `e2e-web/schemas/a11y-row.v1.json` (Z7)."'
    sidecar: '§8 default-deny: the a11y-row schema sentence removed with `e2e-web/` (chunk 2026-10-16-remove-the-browser-leg).'
    rationale: 'Report Files: `e2e-web/` deleted whole; no harness a11y rows exist.'
    basis: '.github/workflows/ci.yml'
    dependent-of: D-obs-stack
  - detector: D-obs-stack
    severity: warning
    section: '§9 CI Integration'
    change: 'Telemetry artifact handling, JUnit row: cargo-nextest JUnit XML (`target/nextest/ci/junit.xml`) alone, uploaded as `junit-${{ matrix.os }}` (drop the Playwright `e2e-web/pw-junit.xml` / `junit-playwright.xml` half).'
    sidecar: '§9 artifacts: `junit-<os>` carries the nextest JUnit alone (chunk 2026-10-16-remove-the-browser-leg).'
    rationale: 'Report Counts: `junit-<os>` uploads `target/nextest/ci/junit.xml` alone; Symbols (run): `archive_sources` has no `pw-junit.xml` source.'
    basis: '.github/workflows/ci.yml'
    dependent-of: D-obs-stack
  - detector: D-obs-stack
    severity: warning
    section: '§9 CI Integration'
    change: 'Pipeline integration, E2E tests row: `agent-run logs` / `status`; G2 and G4 (drop "Playwright DOM-attribute reads").'
    sidecar: '§9 pipeline: Playwright reads leave the E2E row (chunk 2026-10-16-remove-the-browser-leg).'
    rationale: 'Report Harness / gate surface: the browser suite steps removed from the ci.yml `test` job.'
    basis: '.github/workflows/ci.yml'
    dependent-of: D-obs-stack
  - detector: D-obs-stack
    severity: warning
    section: '§9 CI Integration'
    change: 'Step order item 1: drop "the browser suite included: `Node (pinned)`, `npm ci`, the Chromium install and `run --browser`, after the harness lifecycle"; keep the `target/e2e-home/` and chaos-home clause.'
    sidecar: '§9 step order 1: the browser suite steps removed (chunk 2026-10-16-remove-the-browser-leg).'
    rationale: 'Report Harness / gate surface: `Node (pinned)`, `npm ci`, Chromium and both Browser suite steps removed from the `test` job; `--browser` gone.'
    basis: '.github/workflows/ci.yml'
    dependent-of: D-obs-stack
  - detector: D-obs-stack
    severity: warning
    section: '§9 CI Integration'
    change: 'Step order item 3: drop the sentence on `target/agent-run/artifacts/junit-playwright.xml`.'
    sidecar: '§9 step order 3: the Playwright JUnit scan note removed (chunk 2026-10-16-remove-the-browser-leg).'
    rationale: 'Report Symbols (run): no `playwright` archive arm, no `pw-junit.xml` source.'
    basis: 'crates/viola-e2e/src/harness/run.rs'
    dependent-of: D-obs-stack
  - detector: D-obs-stack
    severity: warning
    section: '§9 CI Integration'
    change: 'Step order item 4: `junit-<os>` carries the nextest JUnit alone (was "both JUnit files").'
    sidecar: '§9 step order 4: one JUnit file (chunk 2026-10-16-remove-the-browser-leg).'
    rationale: 'Report Harness / gate surface: `Upload JUnit` carries one path.'
    basis: '.github/workflows/ci.yml'
    dependent-of: D-obs-stack
  - detector: D-obs-stack
    severity: warning
    section: '§9 CI Integration'
    change: 'Remove the paragraph "The browser test leg named here ... stands until the route entry Remove the browser leg removes it; that entry''s wrap amends this text."'
    sidecar: '§9: the browser-leg stand-until paragraph retired; the entry removed the leg (chunk 2026-10-16-remove-the-browser-leg).'
    rationale: 'Report Outcome: no workflow, pre-push stage, harness verb, script or config runs Node / npm / the browser suite; the paragraph defers its amendment to this entry''s wrap.'
    basis: '.github/workflows/ci.yml; crates/viola-e2e/src/harness/pre_push/linux.rs'
    dependent-of: D-obs-stack
  - detector: D-obs-stack
    severity: warning
    section: '§10 SLO Invariants & Telemetry Budgets'
    change: 'Zero unlogged panics: every panic in `run`, `hook`, `mcp` and `cli` after init step 4 (drop `ui`).'
    sidecar: '§10 zero-unlogged-panics: `ui` leaves the role list (chunk 2026-10-16-remove-the-browser-leg).'
    rationale: 'Report Symbols: `ObsProcess::ALL` is the four.'
    basis: 'crates/viola-core/src/obs.rs'
    dependent-of: D-obs-stack
  - detector: D-obs-stack
    severity: warning
    section: '§10 SLO Invariants & Telemetry Budgets'
    change: 'Liveness signal: `run` touches `heartbeat` every 1 s; no process logs a transition (`liveness-changed` and the `ui` are removed); the chaos class "stale heartbeat vs live pid" is gated on `list --json` `liveness` through `agent-run status` (`state: degraded`).'
    sidecar: '§10 liveness signal: the `liveness-changed` / `ui` clauses retired (chunk 2026-10-16-remove-the-browser-leg).'
    rationale: 'Report Symbols: `ObsEvent::LivenessChanged` and `ObsProcess::Ui` removed.'
    basis: 'crates/viola-core/src/obs.rs'
    dependent-of: D-obs-stack
  - detector: D-obs-stack
    severity: warning
    section: '§10 SLO Invariants & Telemetry Budgets'
    change: 'Budget table, Heartbeat staleness flip: 4.9 s → `live`, 5.1 s → `stale`/`gone`, read as `list --json` `liveness` (drop `liveness-changed.heartbeat_age_ms > 5000`).'
    sidecar: '§10 budget table: the staleness flip no longer cites the removed event (chunk 2026-10-16-remove-the-browser-leg).'
    rationale: 'Report Symbols / Schema: `liveness-changed` and its fields removed from `ObsEvent` and diag-line.v1.json.'
    basis: 'schemas/diag-line.v1.json'
    dependent-of: D-obs-stack
  - detector: D-obs-stack
    severity: warning
    section: '§10 SLO Invariants & Telemetry Budgets'
    change: 'Cross-input parallel: the tests'' zero-flakiness budget (`retries = 0`) ↔ zero-unlogged-panics (drop "Playwright `retries: 0`").'
    sidecar: '§10 cross-input parallel: the Playwright retries clause removed (chunk 2026-10-16-remove-the-browser-leg).'
    rationale: 'Report Files: `e2e-web/playwright.config.ts` deleted with `e2e-web/`.'
    basis: '.github/workflows/ci.yml'
    dependent-of: D-obs-stack
  - detector: D-obs-stack
    severity: warning
    section: '§11 Obs Anti-Patterns → PII Scrubbing'
    change: 'NEVER log `CLAUDE*` values or a `Cookie` header, at any level, in any file, detail files included (drop "the GUI token" and `?t=`).'
    sidecar: '§11 PII ban: GUI token and `?t=` wording removed with the GUI forms (chunk 2026-10-16-remove-the-browser-leg, inputs#I1).'
    rationale: 'Report Symbols (secret-scan): every GUI form leaves (I1 fork answer); the `cookie:` header and CLAUDE canaries stay.'
    basis: 'crates/viola-e2e/src/harness/secret_scan.rs'
    dependent-of: D-obs-stack
note: 'D-obs-instrumentation and D-obs-pii fire nothing (the chunk adds no operation and no logging; it only removes vocabulary). Left unproposed as past/basis text, for the orchestrator to judge: §1 tier justification "4 process roles (`run` / `hook` / `mcp` / `ui`)" (a tier basis, cf. the a11y tier left for the founder), §1 item "event enum extensions (... liveness signals, the SSE lifecycle)" and §1 Alignment gap (b) (historical Phase 3 notes), and §4 the deferred GUI scenario heading (the 0.2.0 stub). The §3 Log format key is a verbatim copy of test-plan §3 Log format: apply its amendments in lockstep with test-plan''s. The "reserved: no emitter in 0.1.0; removed with the code by Remove the browser leg" phrase is retired at every site above.'
