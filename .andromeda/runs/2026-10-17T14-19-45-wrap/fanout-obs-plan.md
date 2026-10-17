proposals:
  - detector: D-obs-stack
    severity: warning
    section: '§1. Obs Scope Summary → surface ipc-internal (`viola mcp`) → Exporter'
    change: 'The `viola mcp` sink is always the file `<home>/diagnostics/mcp.ndjson` (the home resolved like every verb, through `resolved_home`, after the strict-modes check of the home and its `diagnostics/`), with stderr JSON only when that file cannot be opened (D-09); retire "when `VIOLA_DIR` is set, otherwise stderr" and the arch quote that carries it.'
    sidecar: 'obs-plan §1 mcp Exporter: the sink is always mcp.ndjson, stderr JSON only on a failed open (D-09); the VIOLA_DIR-unset stderr branch retired (chunk 2026-10-17-mcp-server-for-drivers).'
    rationale: 'Report, Spec claims disproved by measurement: `obs-plan.md:184` ("(tests excerpt) when `VIOLA_DIR` is …") is stale, `viola mcp` resolves its home through `resolved_home` and always writes `mcp.ndjson`; Symbols / APIs: `src/cmd/mcp.rs` `mcp(home)` checks the home and `diagnostics/` through `check_path`, then inits obs as `ObsProcess::Mcp`; Expected amendments name "§1: the stale stderr when `VIOLA_DIR` is unset" (pattern `mcp-ndjson`). §3 → Logging stack already reads the D-09 form.'
    basis: '.andromeda/obs-plan.md:184'
  - detector: D-obs-stack
    severity: warning
    section: '§1. Obs Scope Summary → surface ipc-internal (`viola mcp`) → Notes'
    change: 'Replace "There is a misalignment between tests and arch; see section 3, Log file location" with: the `mcp` sink follows tests (`mcp.ndjson`), reconciled by D-09 and landed (stderr JSON only when the file cannot be opened).'
    sidecar: 'obs-plan §1 mcp Notes: the tests/arch sink misalignment note retired as reconciled (D-09, landed at chunk 2026-10-17-mcp-server-for-drivers).'
    rationale: 'The note points at the same stale stderr-without-VIOLA_DIR claim the primary retires; report Spec claims disproved (`obs-plan.md:184`, `viola mcp` always writes `mcp.ndjson`).'
    dependent-of: D-obs-stack
  - detector: D-obs-stack
    severity: warning
    section: '§1. Obs Scope Summary → Observability harness specification → Logging stack (sink rules)'
    change: 'The sink rule "stderr only for `mcp` without `VIOLA_DIR`" reads "stderr JSON only for `mcp`, and only when `mcp.ndjson` cannot be opened (D-09)".'
    sidecar: 'obs-plan §1 Logging stack sink rules: the mcp stderr rule restated as the D-09 failed-open fallback (chunk 2026-10-17-mcp-server-for-drivers).'
    rationale: 'Same retired claim (mcp logs to stderr when VIOLA_DIR is unset), restated as a sink rule; report Spec claims disproved (`obs-plan.md:184`).'
    dependent-of: D-obs-stack
  - detector: D-obs-stack
    severity: warning
    section: '§1. Obs Scope Summary → Observability harness specification → Log file location'
    change: 'Replace "`mcp` without `VIOLA_DIR` logs to stderr (arch). Tests assume a file. Flag this for the same reconciliation." with: `mcp` always writes `<home>/diagnostics/mcp.ndjson`, as tests assume (D-09; stderr JSON only when the file cannot be opened).'
    sidecar: 'obs-plan §1 Log file location: the mcp stderr flag closed — mcp always writes mcp.ndjson (D-09, landed at chunk 2026-10-17-mcp-server-for-drivers).'
    rationale: 'Same retired claim, restated as an open reconciliation flag; report Spec claims disproved (`obs-plan.md:184`) and Schema / config (`mcp.ndjson` and the `process` value `mcp` stood before; new writers into it).'
    dependent-of: D-obs-stack
  - detector: D-obs-stack
    severity: warning
    section: '§1. Obs Scope Summary → Obs tier justification (the items needing Decisions Log entries or Phase 5 reconciliation)'
    change: 'The item "`mcp` logging to stderr when `VIOLA_DIR` is unset" reads reconciled: D-09, `mcp` always writes `mcp.ndjson`, stderr JSON only when the file cannot be opened.'
    sidecar: 'obs-plan §1 tier justification: the mcp-stderr reconciliation item marked reconciled by D-09 (chunk 2026-10-17-mcp-server-for-drivers).'
    rationale: 'The list carries the retired claim as an open item; report Spec claims disproved (`obs-plan.md:184`).'
    dependent-of: D-obs-stack
  - detector: D-obs-stack
    severity: warning
    section: '§1. Obs Scope Summary → closing note (Section 1 is a verbatim copy of obs-scope.md …)'
    change: 'Drop "the `mcp` sink (D-09: always `mcp.ndjson`, stderr only when the file cannot be opened)" from the list of differences where §3, §6 and §12 win, since §1 now reads the D-09 form.'
    sidecar: 'obs-plan §1 closing note: the mcp sink leaves the list of §1 differences, brought current at chunk 2026-10-17-mcp-server-for-drivers.'
    rationale: 'The note asserts §1 still holds the stale mcp-sink claim; once the primary and its dependents land, that difference no longer exists (report Spec claims disproved, `obs-plan.md:184`).'
    dependent-of: D-obs-stack
  - detector: D-obs-instrumentation
    severity: warning
    section: '§6. Log Coverage → Boundary-call wrappers → Child / shell spawns'
    change: 'The writers of the `agents-probe` pair add `viola mcp`''s `list` tool: its one `claude agents --json` read, made in the `mcp` process, writes its `process-start` / `process-exit{subject:"agents-probe"}` pair into `mcp.ndjson` (beside `viola list` into `cli-<name>.ndjson`, `viola revive` into `run-<name>.ndjson` and verify''s Run B); chunk 2026-10-17-mcp-server-for-drivers.'
    sidecar: 'obs-plan §6 Child / shell spawns: the mcp list tool named as a writer of the agents-probe pair, into mcp.ndjson (chunk 2026-10-17-mcp-server-for-drivers).'
    rationale: 'Report Schema / config: "New writers into `mcp.ndjson`: the `list` tool''s `agents-probe` pair"; Spec claims disproved: "the `list` tool makes the one `claude agents --json` read in the `mcp` process" (`inputs#I2` Q1, ratified); Expected amendments: "obs-plan §6 Boundary-call wrappers: `mcp` writes the `agents-probe` pair". The section names three writers and no mcp one.'
    basis: 'viola-0.1.0/chunks/2026-10-17-mcp-server-for-drivers/report.md'
  - detector: D-obs-instrumentation
    severity: warning
    section: '§6. Log Coverage → `detail` code catalog → Filesystem refusals (the `state-entry` call site)'
    change: 'The `viola_state::board::read` call site writes its `parse-rejected{parser:"state-entry"}` lines into `cli-<name>.ndjson` under a `VIOLA_NAME` for `viola list`, and into `mcp.ndjson` for `viola mcp`''s `list` tool (through `list::board_document`); retire "into `cli-<name>.ndjson` under a `VIOLA_NAME` only".'
    sidecar: 'obs-plan §6 state-entry call site: board::read also logs into mcp.ndjson for the mcp list tool (chunk 2026-10-17-mcp-server-for-drivers).'
    rationale: 'Same retired claim as the agents-probe primary (the board reads log only into the cli role file); report Schema / config: new writers into `mcp.ndjson` include "the board''s `state-recovered` / `parse-rejected` lines"; Symbols / APIs: `src/cmd/list.rs` `board_document(home)` (scan → read → document) is the `mcp` provider.'
    dependent-of: D-obs-instrumentation
  - detector: D-obs-instrumentation
    severity: warning
    section: '§4. Span / Trace Coverage → E5 (snapshot corruption)'
    change: 'Beside `viola list`''s `viola_state::board::read` writing `state-recovered` into `cli-<name>.ndjson` under a `VIOLA_NAME`, name `viola mcp`''s `list` tool (the same board read through `list::board_document`), whose `state-recovered` lines go into `mcp.ndjson`.'
    sidecar: 'obs-plan §4 E5: the mcp list tool''s board read named as a state-recovered writer into mcp.ndjson (chunk 2026-10-17-mcp-server-for-drivers).'
    rationale: 'Same retired claim (board-read lines go only to the cli role file); report Schema / config lists "the board''s `state-recovered` / `parse-rejected` lines" among the new `mcp.ndjson` writers.'
    dependent-of: D-obs-instrumentation
  - detector: D-obs-instrumentation
    severity: warning
    section: '§3 → OTel SDK init'
    change: 'Init step 6: beside `viola list` (its per-key `config-json` threshold rejections into `cli-<name>.ndjson` under a `VIOLA_NAME`), name `viola mcp`''s `list` tool, whose board read writes the same per-key `parse-rejected{parser:"config-json"}` lines into `mcp.ndjson`; step 5''s parenthetical naming `run` and `viola list` as the second readers of `config.json` adds the `mcp` `list` tool.'
    sidecar: 'obs-plan §3 OTel SDK init steps 5–6: the mcp list tool named as a threshold reader, its config-json rejections into mcp.ndjson (chunk 2026-10-17-mcp-server-for-drivers).'
    rationale: 'Same retired claim (the board''s logged reads have the cli role only); report Schema / config: new writers into `mcp.ndjson` include "the `config-json` rejections (fields already in the catalogue)". Steps 5 and 6 name only `run` and `viola list` for the threshold read.'
    dependent-of: D-obs-instrumentation
  - detector: D-obs-instrumentation
    severity: warning
    section: '§10. SLO Invariants & Telemetry Budgets → Zero unlogged panics → Bounded exemptions (item 7)'
    change: 'Item 7 adds `viola mcp`''s refused start-time home check (the home or its `diagnostics/` failing `viola_state::strict::check_path`): exit 1 with nothing on stdout or stderr, no role file opened and no line, so a panic there writes none.'
    sidecar: 'obs-plan §10 exemption 7: viola mcp''s refused home check (exit 1, no role file, no line) added (chunk 2026-10-17-mcp-server-for-drivers).'
    rationale: 'Report Symbols / APIs: `mcp(home)` checks the home and its `diagnostics/` through `check_path` "before anything under the home opens — a refusal exits 1 with nothing on stdout or stderr and no role file"; Coverage of new surfaces: "instrumentation none by design (a refusal opens no log)". Item 7 enumerates the refused checks that open no role file and names revive, hook statusline and list only.'
    basis: 'viola-0.1.0/chunks/2026-10-17-mcp-server-for-drivers/report.md'
  - detector: D-obs-instrumentation
    severity: warning
    section: '§10. SLO Invariants & Telemetry Budgets → Zero unlogged panics → Bounded exemptions (item 1, the pre-init window)'
    change: 'The pre-init window adds `viola mcp`''s check of its home and `diagnostics/`, which runs before its `mcp` role file opens.'
    sidecar: 'obs-plan §10 exemption 1: viola mcp''s home check named in the pre-init window (chunk 2026-10-17-mcp-server-for-drivers).'
    rationale: 'Same claim as the exemption-7 primary (the checks run before a role file opens are those of revive, hook statusline and list); report Symbols / APIs (`src/cmd/mcp.rs` `mcp(home)`).'
    dependent-of: D-obs-instrumentation
  - detector: D-obs-instrumentation
    severity: warning
    section: '§7. Error Capture & Reporting → panic handling (Before step 4 of init …)'
    change: 'The pre-step-4 window adds `viola mcp`''s check of its home and `diagnostics/`, run before any role file is opened, beside the instance check of `viola revive`''s start arm and `viola hook statusline`.'
    sidecar: 'obs-plan §7 pre-init window: viola mcp''s home check named (chunk 2026-10-17-mcp-server-for-drivers).'
    rationale: 'Same claim as the exemption-7 primary, restated in §7; report Symbols / APIs (`mcp(home)`: the check precedes obs init as `ObsProcess::Mcp`).'
    dependent-of: D-obs-instrumentation
  - detector: D-obs-instrumentation
    severity: warning
    section: '§1. Obs Scope Summary → telemetry triggers → logging-sensitive, security plan Vector 5 (MCP) → Required telemetry signal'
    change: 'The codes-only set of a rejected tool call adds `invalid-params`, whose `detail` is a closed field word (`target · text · after · timeout_ms · dialog_id · response`): `refusal` / `detail` / `instance-unreachable` / `wrapper-fault` / `invalid-params`, with no upstream text, absolute paths or type names.'
    sidecar: 'obs-plan §1 Vector 5 trigger: invalid-params (closed field-word detail) added to the MCP codes-only rejection set (chunk 2026-10-17-mcp-server-for-drivers).'
    rationale: 'Report Schema / config: "A new MCP error code `invalid-params` beside `instance-unreachable` / `wrapper-fault`"; Symbols / APIs: a failed re-validation is `isError: true`, `{"error":"invalid-params","detail":"<field>"}` with the closed field words, text `refused: invalid-params`, no frame. The trigger lists the closed rejection codes and omits it.'
    basis: 'viola-0.1.0/chunks/2026-10-17-mcp-server-for-drivers/report.md'
  - detector: D-obs-instrumentation
    severity: warning
    section: '§1. Obs Scope Summary → surface ipc-internal (`viola mcp`) → Notes'
    change: 'The `isError` codes-only list reads `refusal` / `detail`, `instance-unreachable`, `wrapper-fault`, `invalid-params` (its `detail` a closed field word).'
    sidecar: 'obs-plan §1 mcp Notes: invalid-params added to the isError codes (chunk 2026-10-17-mcp-server-for-drivers).'
    rationale: 'Same closed code list as the Vector 5 trigger primary, restated in the mcp surface notes; report Schema / config (the new code `invalid-params`).'
    dependent-of: D-obs-instrumentation
note: 'D-obs-instrumentation carries three primaries (agents-probe writers in §6; the mcp home-check exemption in §10 item 7; the invalid-params code in the §1 Vector 5 trigger) — each dependent names its primary in its rationale. Not proposed (no obs-plan site claims otherwise): report Decisions & corrections 14 — the live `viola mcp` logs no `process-exit` because Claude Code ends it by a signal, not a stdin close; D-obs-pii found no drift (send text and responses asserted absent from `mcp.ndjson`); D-obs-stack found no off-spec dependency (rmcp target stays OFF, tracing unchanged).'
