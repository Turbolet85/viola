# Cascade compare — obs-plan · wrap · 2026-10-26-home-and-code-bearing-file-integrity
E items 48 · held 28 · lost at classify 6 · lost at strong 0 · beside a placed fact 2 · cascade only 16 · listed - · body 0 · unplaced 0

## Lost — the full read carried these and the cascade did not; the wrap validates each, marked «(sampler)»

### error-capture-reporting#1.1 · lost at strong · beside a placed fact F7 (near 31, error-capture-reporting#4.1) · .andromeda/obs-plan-detail/error-capture-reporting.md:8
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/error-capture-reporting.md
    near: 8
    section: '## 7. Error Capture & Reporting'
    change: 'After the line-8 item, add one item: a role file or detail file is opened only through `src/obs.rs` `check_diagnostics`, which runs `strict::check_diagnostics` on the existing `diagnostics/` folder and the file in `open_role_file` and `write_detail` before `ensure_private_dir` narrows anything (a symlink refused, a folder the read rule, a file the read rule on Unix and the write rule on Windows, an absent path passing); a refusal is exactly a failed open, so the role writer ends at `std::io::sink` and the content-bearing detail line (the chain, the panic payload) is dropped.'
    sidecar: 'error-capture-reporting: the role and detail files open only past `check_diagnostics`; a refused path is a failed open, the detail line dropped (chunk 2026-10-26-home-and-code-bearing-file-integrity).'
    rationale: 'The report''s Changes: `src/obs.rs` new private `check_diagnostics(dir, file)` (69-76) runs `strict::check_diagnostics` (strict.rs:127-145) before `ensure_private_dir` in `open_role_file` and `write_detail`, a refusal an `io::Error`, «exactly a failed open (the writer ends at `std::io::sink`, a detail line dropped)»; the file states the detail line is written whenever an instance resolves, with no condition on the diagnostics path.'
    basis: viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:20

### metric-coverage#3.1 · lost at classify · .andromeda/obs-plan-detail/metric-coverage.md:36
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/metric-coverage.md
    near: 36
    section: '## 5. Metric Coverage'
    change: 'After the line-36 item, add one item: a CLI `wait` refused by the instance strict-modes check (`client::refuse_untrusted`, exit 21, the instance pair or the `instance-unreachable` document) writes no line at all, neither a `process-exit` nor a `channel-request`, so neither `viola.wait.outcome` nor the Counter row''s `detail` grouping counts it; MCP''s `wait` tool refused by its `check_instance` (`instance-unreachable`, `detail: null`) writes none either; the exit code or the `isError` code is its only record.'
    sidecar: 'metric-coverage: the strict-modes refusal of CLI and MCP `wait` writes no line, so no derived metric counts it (chunk 2026-10-26-home-and-code-bearing-file-integrity, the founder''s no-line word inputs#I3 Q2).'
    rationale: 'The report''s Changes add `refuse_untrusted` at `wait` (wait.rs:43-45) and the MCP frame tools'' `check_instance` (tools.rs:337-339), each refusing with no line (Coverage: instrumentation n/a, no line by the founder''s word, inputs#I3 Q2); the file names the server-verify refusal''s `process-exit` as the counted record of a pre-frame `wait` refusal, and the new no-line refusal is a pre-frame refusal no metric can see.'
    basis: viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:15

### obs-scope-summary#30.2 · lost at classify · beside a placed fact F13 (near 123, obs-scope-summary#14.3) · .andromeda/obs-plan-detail/obs-scope-summary.md:326
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/obs-scope-summary.md
    near: 326
    section: "## 1. Obs Scope Summary"
    change: "Line 326's must-trace spans read `hook.handle` › `state.snapshot_recover` › `channel.request(hook.dialog)` › …: hook `connect` reads `read_snapshot_or_replay`, so the replay's span sits under `hook.handle` on the hook path (chunk 2026-10-26-home-and-code-bearing-file-integrity)."
    sidecar: "obs-scope-summary dialog path: `state.snapshot_recover` placed under `hook.handle`."
    rationale: "Report Deviation 4 and Count 6: `state.snapshot_recover` sits under `hook.handle` on the hook path; Symbols `src/cmd/hook.rs` `connect` (224-226) reads `read_snapshot_or_replay`."
    basis: viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:16

### pii-scrubbing-compliance#2.1 · lost at classify · .andromeda/obs-plan-detail/pii-scrubbing-compliance.md:38
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/pii-scrubbing-compliance.md
    near: 38
    section: '## 8. PII Scrubbing & Compliance › ### PII Scrubbing'
    change: 'Add one item after line 38: - A strict-modes refusal of the six frame verbs (`send`, `wait`, `last`, `answer`, `pause`, `release`, through `client::refuse_untrusted`) carries no pid, path or endpoint and writes no line: it prints only the fixed instance pair, or under `--json` exactly `{"v":1,"error":"instance-unreachable","detail":null}`, and `tests/security_negatives_strict_modes.rs` reads every output with `digit_runs` and asserts no `cli-<name>.ndjson` line, on all three OSes (chunk 2026-10-26-home-and-code-bearing-file-integrity); the `viola mcp` frame tools refused by the same check return `instance-unreachable` with `detail: null` and no line (`binary(handlers)`).'
    sidecar: 'pii-scrubbing-compliance: verification gains the strict-modes refusal item (six frame verbs and the MCP frame tools: fixed pair or document, no pid or path, no line; digit_runs scan, three OSes), chunk 2026-10-26-home-and-code-bearing-file-integrity.'
    rationale: 'Report Changes › Symbols (`src/cmd/client.rs` refuse_untrusted, `viola-mcp` tools.rs frame 337-339) and Coverage of new surfaces (refuse_untrusted: PII n/a, fixed pair, no path, `digit_runs` read; tests `binary(security_negatives_strict_modes)` three OSes; MCP `detail: null`, no line); the file states the matching server-verification item at line 38 but none for the new strict-modes refusals.'
    basis: viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:85

### pii-scrubbing-compliance#2.1 · lost at classify · .andromeda/obs-plan-detail/pii-scrubbing-compliance.md:38
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/pii-scrubbing-compliance.md
    near: 38
    section: '## 8. PII Scrubbing & Compliance › ### PII Scrubbing'
    change: 'Add one item after the item above: - The content sink is opened only after its strict-modes check: `src/obs.rs` `check_diagnostics` runs `strict::check_diagnostics` on the existing instance `diagnostics/` folder and on the role or `detail-<process>.ndjson` file in `open_role_file` and `write_detail`, before `ensure_private_dir` narrows anything; a symlink, a folder another user may read (Unix group/other bit; Windows a read grant to another SID), or a Unix file with a group/other bit (Windows: the write rule) refuses the open as a failed open: the writer ends at `std::io::sink` and the detail line is dropped, so High-class content never lands in a file another user may read (unit cases obs.rs 610-673).'
    sidecar: 'pii-scrubbing-compliance: the detail-file sink guard (check_diagnostics before every role and detail open; a refused path drops the line), chunk 2026-10-26-home-and-code-bearing-file-integrity.'
    rationale: 'Report Changes › Symbols (`src/obs.rs`: new private `check_diagnostics(dir, file)` 69-76 in `open_role_file` and `write_detail`; a refusal is an io::Error, the writer ends at sink, a detail line dropped; `viola_state::strict::check_diagnostics` 127-145 rules) and Coverage of new surfaces (diagnostics opens, validation symlink + modes); rows 13-15 of the file route High-class content to the detail file but state no guard on that sink.'
    basis: viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:20

### slo-invariants-telemetry-budgets#2.2 · lost at classify · .andromeda/obs-plan-detail/slo-invariants-telemetry-budgets.md:28
  - detector: D-obs-instrumentation
    severity: warning
    file: .andromeda/obs-plan-detail/slo-invariants-telemetry-budgets.md
    near: 28
    section: "## 10. SLO Invariants & Telemetry Budgets"
    change: "The spine-hooks row's budget cell gains, after «… bounded by the same gate»: «; since chunk 2026-10-26-home-and-code-bearing-file-integrity the timed path also holds the hook's instance strict-modes check (`trusted`, before obs init), and its one `snapshot.json` read is `read_snapshot_or_replay` (a replayed or absent snapshot fails open as `channel-unreachable`), inside the same gate (that chunk's `run --perf`: max 2.4–3.3 ms over the five rows)»."
    sidecar: "slo-budgets spine row: the hook's instance check and the replay-aware snapshot read sit in the timed path, gated by the same max (chunk 2026-10-26-home-and-code-bearing-file-integrity)."
    rationale: "Report Outcome: «`run --perf` keeps the five hook rows under `SPINE_DEADLINE` with the check in the timed path — MET (max 2.5 / 2.4 / 2.4 / 2.4 / 3.3 ms)»; Symbols (hook.rs): the check before viola_obs_init and connect on read_snapshot_or_replay."
    basis: viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:137

### slo-invariants-telemetry-budgets#2.4 · lost at classify · .andromeda/obs-plan-detail/slo-invariants-telemetry-budgets.md:30
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/slo-invariants-telemetry-budgets.md
    near: 30
    section: "## 10. SLO Invariants & Telemetry Budgets"
    change: "The obs-overhead row gains, after «… the gate passes vacuously»: «; it passes vacuously too when `VIOLA_DIR` names no existing instance directory or the instance fails the strict-modes check (the hook then opens no file and makes no channel call), so the perf instance is a viola-made one (chunk 2026-10-26-home-and-code-bearing-file-integrity); logger init's one `open` now follows the instance check's and `check_diagnostics`' metadata reads»."
    sidecar: "slo-budgets obs-overhead row: a missing or refused instance directory is a second vacuous pass; the perf instance is viola-made (chunk 2026-10-26-home-and-code-bearing-file-integrity)."
    rationale: "Report Symbols (hook.rs: a refused or absent instance returns exit 0 at once, no config read, no line, no frame; a non-existent VIOLA_DIR writes nothing) and Deviation 5 (cases expecting lines run in a viola-made instance dir, `Env::Made`); the row's vacuous-pass clause names only the D-16 no-op."
    basis: viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:98

### span-trace-coverage#3.1 · lost at classify · .andromeda/obs-plan-detail/span-trace-coverage.md:28
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/span-trace-coverage.md
    near: 28
    section: '## 4. Span / Trace Coverage'
    change: 'Replace «after its own client-side refusals (for `link` / `unlink`, their instance check of the driven instance, then the liveness pre-check)» with «after its own client-side refusals (for every one of the eight, the instance strict-modes check `client::refuse_untrusted` first, then the liveness pre-check)».'
    sidecar: 'span-trace-coverage server check: the instance check precedes the liveness pre-check on all eight frame verbs, chunk 2026-10-26-home-and-code-bearing-file-integrity.'
    rationale: 'Report Symbols client.rs: `refuse_untrusted` called first by send, wait, last, answer, pause, release, and `link_call` moved onto it; Count 2 (check_instance set and call-site files moved).'
    basis: viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:15
    dependent-of: D-new-detail

### span-trace-coverage#3.4 · lost at classify · .andromeda/obs-plan-detail/span-trace-coverage.md:31
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/span-trace-coverage.md
    near: 31
    section: '## 4. Span / Trace Coverage'
    change: 'Add: «Before `live_endpoint`, each frame tool runs `check_instance(home, instance_dir)` inside `mcp.tool_call`; a refusal returns `instance-unreachable` with `detail: null`, no frame and no line.»'
    sidecar: 'span-trace-coverage mcp: the frame tools check the instance before the liveness read, a refusal no line, chunk 2026-10-26-home-and-code-bearing-file-integrity.'
    rationale: 'Report Symbols viola-mcp tools.rs `frame` (337-339): `check_instance` before `live_endpoint`, a refusal `result::unreachable()`, no frame, no line; Retired gap (8).'
    basis: viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:14

## Held

- error-capture-reporting#2.1
- error-capture-reporting#3.1
- log-coverage#1.1
- log-coverage#2.1
- log-coverage#2.7
- obs-anti-patterns#1.1
- obs-scope-summary#18.3
- obs-scope-summary#20.5
- obs-scope-summary#26.1
- obs-scope-summary#27.2
- obs-scope-summary#27.3
- obs-scope-summary#35.1
- obs-scope-summary#36.1
- obs-scope-summary#38.1
- obs-scope-summary#48.1
- slo-invariants-telemetry-budgets#1.1
- span-trace-coverage#13.1
- span-trace-coverage#16.1
- span-trace-coverage#2.1
- span-trace-coverage#2.3
- span-trace-coverage#3.3
- span-trace-coverage#4.1
- span-trace-coverage#4.2
- span-trace-coverage#4.3
- span-trace-coverage#6.3
- telemetry-strategy#2.1
- telemetry-strategy#3.3
- telemetry-strategy#3.6

## Cascade only

- log-coverage#2.4
- obs-scope-summary#14.3
- obs-scope-summary#15.3
- obs-scope-summary#18.5
- obs-scope-summary#20.4
- obs-scope-summary#23.1
- obs-scope-summary#30.3
- obs-scope-summary#43.1
- obs-scope-summary#45.1
- span-trace-coverage#2.5
- span-trace-coverage#5.1
- span-trace-coverage#5.4
- span-trace-coverage#7.1
- span-trace-coverage#7.3
- span-trace-coverage#7.4
- telemetry-strategy#3.16
