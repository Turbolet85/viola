# Cascade compare — obs-plan · wrap · 2026-10-31-exit-cause-code-catalogue
E items 47 · held 23 · lost at classify 1 · lost at strong 0 · beside a placed fact 2 · cascade only 5 · listed - · body 0 · unplaced 0

## Lost — the full read carried these and the cascade did not; the wrap validates each, marked «(sampler)»

### error-capture-reporting#6.4 · lost at classify · .andromeda/obs-plan-detail/error-capture-reporting.md:51
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/error-capture-reporting.md
    near: 51
    section: '## 7. Error Capture & Reporting'
    change: 'New bullet after this line: `- Exit 21 prints one document per cause, `{"v":1,"error":"instance-unreachable","detail":"<cause>"}`, with `v` still 1 and `error` unmoved; `detail` is one of the five closed words of `viola_core::UnreachableCause`: `strict-modes-failed`, `instance-dead`, `unwrapped-name`, `endpoint-lost`, `server-verify-failed`. No serde path reaches them: every word is written by the two clients. `unreachable_document` (`src/cmd/client.rs`) is the one writer of the document, with `refuse_untrusted` and `src/cmd/plugin.rs` through it; `refused_document` in `src/cmd/list.rs` builds `strict-modes-failed` as a `Value` for `board_document`, the only refusal of the board.`'
    sidecar: 'error-capture-reporting: the exit-21 document names one of five closed causes, and its one writer is recorded (chunk 2026-10-31-exit-cause-code-catalogue).'
    rationale: 'The report symbol bullets "The exit-21 document moved", `UnreachableCause` and `src/cmd/client.rs` state each fact; the group this follows holds the error-surface rules.'
    basis: crates/viola-core/src/lib.rs:181

### error-capture-reporting#6.4 · lost at classify · .andromeda/obs-plan-detail/error-capture-reporting.md:51
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/error-capture-reporting.md
    near: 51
    section: '## 7. Error Capture & Reporting'
    change: 'New bullet after this line: `- `viola_state::liveness::without_endpoint` gives a caller with no endpoint its cause: `symlink_metadata` reads a directory `instance-dead`, and anything else (absent, a regular file, a symlink) `unwrapped-name`; it opens nothing and writes no line. The strict-modes check stays first at every caller, `Unreachable::without_endpoint` in `src/cmd/client.rs` and `frame` in `crates/viola-mcp/src/tools.rs`.`'
    sidecar: 'error-capture-reporting: without_endpoint reads instance-dead or unwrapped-name from the instance directory, opening nothing and writing no line (chunk 2026-10-31-exit-cause-code-catalogue).'
    rationale: 'The report symbol bullet `viola_state::liveness::without_endpoint` states the reading, its callers and that it writes no line.'
    basis: crates/viola-state/src/liveness.rs:107

### error-capture-reporting#6.4 · lost at classify · .andromeda/obs-plan-detail/error-capture-reporting.md:51
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/error-capture-reporting.md
    near: 51
    section: '## 7. Error Capture & Reporting'
    change: 'New bullet after this line: `- `refused_with(json, detail, human)` (`src/cmd/run.rs`) prints `refused_document` in place of the stderr pair when `Launch.json` is set (`run` passes false, `revive` passes `args.json`), for `batch-script-child`, `pinned-hash-mismatch`, `squatted-name`, and for a program not found `internal-error` (no pair today, none added). All return before `spawn_child`, and the human pairs are byte for byte as they stood.`'
    sidecar: 'error-capture-reporting: refused_with prints the start refusal documents under revive --json, before spawn_child (chunk 2026-10-31-exit-cause-code-catalogue).'
    rationale: 'The report symbol bullet "`run`''s start under `--json`" adds `refused_with` and the `json` field of `Launch`, and names the codes.'
    basis: src/cmd/run.rs:422

### obs-scope-summary#29.3 · lost at classify · beside a placed fact F11 (near 36, obs-scope-summary#4.3) · .andromeda/obs-plan-detail/obs-scope-summary.md:320
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/obs-scope-summary.md
    near: 320
    section: '## 1. Obs Scope Summary'
    change: 'Replace `exit_code` (21 when the wrapper is killed mid-wait) with `exit_code` (21 when the wrapper is killed mid-wait, `detail` `endpoint-lost`, `during:"call"`).'
    sidecar: 'Obs Scope Summary, `wait` / `last` path: a wrapper killed mid-wait now logs `endpoint-lost` with `during:"call"` (chunk 2026-10-31-exit-cause-code-catalogue).'
    rationale: 'Report **Symbols / APIs**, a process-exit `during`: a wrapper lost with the call open is `endpoint-lost` · `during:"call"`; **Schema / config** adds `endpoint-lost` to the `process-exit` `detail` enum.'
    basis: schemas/diag-line.v1.json:78

### span-trace-coverage#3.7 · lost at strong · beside a placed fact F5 (near 78, span-trace-coverage#6.1) · .andromeda/obs-plan-detail/span-trace-coverage.md:34
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/span-trace-coverage.md
    near: 34
    section: '## 4. Span / Trace Coverage'
    change: 'Add after this line: `- **cli (short-lived verbs), the no-endpoint reading:** the six no-endpoint arms and MCP''s `frame` read `viola_state::liveness::without_endpoint` after the strict-modes check: `symlink_metadata` reads a directory `instance-dead`, anything else (absent, a regular file, a symlink) `unwrapped-name`; it opens nothing and writes no line. A frame verb against a stale instance still reads `instance-dead`, since `live_endpoint` gives none for a stale wrapper; stale heartbeat is exit 1''s cause alone.`'
    sidecar: 'Span coverage: new item, how a frame verb with no live endpoint tells instance-dead from unwrapped-name, and that a stale instance stays instance-dead.'
    rationale: 'Report Symbols / APIs `viola_state::liveness::without_endpoint`, the six no-endpoint arms and **A stale holder''s code**.'
    basis: crates/viola-state/src/liveness.rs:107

## Held

- error-capture-reporting#2.1
- error-capture-reporting#3.1
- error-capture-reporting#5.1
- log-coverage#2.1
- log-coverage#2.2
- log-coverage#2.8
- metric-coverage#3.2
- obs-scope-summary#18.3
- obs-scope-summary#22.5
- obs-scope-summary#27.3
- obs-scope-summary#36.1
- obs-scope-summary#38.1
- obs-scope-summary#43.1
- obs-scope-summary#48.1
- pii-scrubbing-compliance#2.1
- span-trace-coverage#17.1
- span-trace-coverage#3.3
- span-trace-coverage#3.6
- span-trace-coverage#4.3
- span-trace-coverage#6.4
- span-trace-coverage#7.3
- span-trace-coverage#8.3
- telemetry-strategy#3.16

## Cascade only

- obs-scope-summary#18.5
- obs-scope-summary#30.3
- obs-scope-summary#4.3
- span-trace-coverage#6.1
- span-trace-coverage#7.4
