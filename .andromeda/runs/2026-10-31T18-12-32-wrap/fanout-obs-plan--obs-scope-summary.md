proposals:
  - detector: D-retired
    severity: warning
    file: .andromeda/obs-plan-detail/obs-scope-summary.md
    near: 159
    section: '## 1. Obs Scope Summary'
    change: 'In the sentence on `send`, `wait`, `last`, `answer`, `pause` and `release`, replace the document `{"v":1,"error":"instance-unreachable","detail":null}` with `{"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"}`; the rest of the sentence stands.'
    sidecar: 'Obs Scope Summary, cli surface exporter: a refused instance check''s exit-21 document now carries `detail` `strict-modes-failed` (chunk 2026-10-31-exit-cause-code-catalogue).'
    rationale: 'Report **Retired** names the exit-21 document''s `detail: null`; **Symbols / APIs** says each cause prints its own `detail` and `refuse_untrusted` writes `StrictModes` through `unreachable_document`.'
    basis: src/cmd/client.rs:222
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/obs-scope-summary.md
    near: 215
    section: '## 1. Obs Scope Summary'
    change: 'After `instance-unreachable` in the `isError` code list, add: (its `detail` the cause, one of `strict-modes-failed` · `instance-dead` · `unwrapped-name` · `endpoint-lost` · `server-verify-failed`, the text still `refused: instance-unreachable`, no line on any, the `isError` result the record).'
    sidecar: 'Obs Scope Summary, mcp surface notes: `instance-unreachable` results now name one of five closed causes in `detail`, with no log line (chunk 2026-10-31-exit-cause-code-catalogue).'
    rationale: 'Report **Symbols / APIs**, MCP: `unreachable(cause: UnreachableCause)` and the `of_reply`, `of_board` and `frame` mappings; **Coverage of new surfaces**: MCP `detail` causes write no line by `inputs#I2` Q4.'
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/obs-scope-summary.md
    near: 302
    section: '## 1. Obs Scope Summary'
    change: 'In the exit-1 `detail` code list, add `stale-heartbeat` after already-live: already-live (a live holder or a fresh replayed beat), stale-heartbeat (a stale holder), squatted-name, and the rest as they stand.'
    sidecar: 'Obs Scope Summary, `viola run` start path: a stale holder now logs its own `stale-heartbeat` exit-1 code (chunk 2026-10-31-exit-cause-code-catalogue).'
    rationale: 'Report **Symbols / APIs**, a stale holder''s code: `collision_check` prints and logs `stale-heartbeat` for a `stale` holder and `already-live` for a live holder and a fresh replayed beat; **Retired** names `already-live` as a stale holder''s refusal.'
    basis: schemas/diag-line.v1.json:78
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/obs-scope-summary.md
    near: 320
    section: '## 1. Obs Scope Summary'
    change: 'Replace `exit_code` (21 when the wrapper is killed mid-wait) with `exit_code` (21 when the wrapper is killed mid-wait, `detail` `endpoint-lost`, `during:"call"`).'
    sidecar: 'Obs Scope Summary, `wait` / `last` path: a wrapper killed mid-wait now logs `endpoint-lost` with `during:"call"` (chunk 2026-10-31-exit-cause-code-catalogue).'
    rationale: 'Report **Symbols / APIs**, a process-exit `during`: a wrapper lost with the call open is `endpoint-lost` · `during:"call"`; **Schema / config** adds `endpoint-lost` to the `process-exit` `detail` enum.'
    basis: schemas/diag-line.v1.json:78
  - detector: D-retired
    severity: warning
    file: .andromeda/obs-plan-detail/obs-scope-summary.md
    near: 381
    section: '## 1. Obs Scope Summary'
    change: 'Replace `instance-unreachable` with `detail: null` with `instance-unreachable` with `detail` `strict-modes-failed`; the rest of the sentence stands.'
    sidecar: 'Obs Scope Summary, MCP trigger: a frame tool''s refused instance check now returns `detail` `strict-modes-failed` (chunk 2026-10-31-exit-cause-code-catalogue).'
    rationale: 'Report **Retired** names the exit-21 document''s `detail: null`; **Symbols / APIs**, MCP: in `tools.rs` `frame` a refused `check_instance` is `StrictModes`, its result the CLI body less `v`.'
    dependent-of: D-retired
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/obs-scope-summary.md
    near: 399
    section: '## 1. Obs Scope Summary'
    change: 'In the clause on a refused `update_stamps` in `viola verify`, replace (with an instance; exit 1, no `detail`) with (with an instance; exit 1, `detail` `strict-modes-failed`).'
    sidecar: 'Obs Scope Summary, filesystem trigger: verify''s refused `update_stamps` line now carries `detail` `strict-modes-failed` (chunk 2026-10-31-exit-cause-code-catalogue).'
    rationale: 'Report **Symbols / APIs**, `verify`''s codes: `Ended::Refused` writes its code on verify''s `process-exit`, `strict-modes-failed` among them, while a refused home tree stays lineless; **Deviations** item 7 adds `verify_window_a_widened_stamps_file_refuses_with_its_code`.'
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/obs-scope-summary.md
    near: 445
    section: '## 1. Obs Scope Summary'
    change: 'Replace the row with: endpoint vanished during `wait` (exit 21 / `instance-unreachable`, `detail` `endpoint-lost`, `during:"call"`);'
    sidecar: 'Obs Scope Summary, chaos trigger: the vanished-endpoint fault now has its own `endpoint-lost` code (chunk 2026-10-31-exit-cause-code-catalogue).'
    rationale: 'Report **Symbols / APIs**: `answer_of` reads `Closed | Io` as `Lost`, and a wrapper lost with the call open is `endpoint-lost` · `during:"call"`; `tests/chaos_wait_vanish.rs` is in **Files**.'
    basis: schemas/diag-line.v1.json:78
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/obs-scope-summary.md
    near: 446
    section: '## 1. Obs Scope Summary'
    change: 'Replace the row with: stale heartbeat vs live pid (a start over the stale holder refuses exit 1 with `process-exit` `detail` `stale-heartbeat`; a frame verb against it still reads `instance-dead`).'
    sidecar: 'Obs Scope Summary, chaos trigger: stale heartbeat now has an exit-1 code at the collision check, and a frame verb still reads `instance-dead` (chunk 2026-10-31-exit-cause-code-catalogue).'
    rationale: 'Report **Symbols / APIs**, a stale holder''s code: stale heartbeat is exit 1''s cause alone, and `live_endpoint` gives no endpoint for a stale wrapper, so a frame verb reads `instance-dead`.'
    basis: schemas/diag-line.v1.json:78
  - detector: D-retired
    severity: warning
    file: .andromeda/obs-plan-detail/obs-scope-summary.md
    near: 490
    section: '## 1. Obs Scope Summary'
    change: 'In the `viola link` / `viola unlink` sentence, replace the document `{"v":1,"error":"instance-unreachable","detail":null}` with `{"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"}`; revive''s `{"v":1,"error":"strict-modes-failed","detail":null}` exit-1 document stands.'
    sidecar: 'Obs Scope Summary, Founder Direction 4 trigger: the `link` / `unlink` strict-modes exit-21 document now carries `detail` `strict-modes-failed` (chunk 2026-10-31-exit-cause-code-catalogue).'
    rationale: 'Report **Retired** names the exit-21 document''s `detail: null`; **Symbols / APIs** says `refused_document` stays exit 1''s, so only the exit-21 document in this passage moves.'
    basis: src/cmd/client.rs:222
    dependent-of: D-retired
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/obs-scope-summary.md
    near: 490
    section: '## 1. Obs Scope Summary'
    change: 'Replace the two cause lists with: exit 21 (dead instance, unwrapped name, endpoint lost during a call, strict-mode fail, server-verify fail) and exit 1 (already live, stale heartbeat, squatted name, SHA-256 mismatch, `.cmd`/`.bat` child, `viola revive`''s no logged session, session live elsewhere and missing recorded directory, and `viola verify`''s `program-not-found`, `version-unreadable` and `unclean-fixture`).'
    sidecar: 'Obs Scope Summary, Founder Direction 4 trigger: the exit-21 and exit-1 cause lists gain the codes this chunk landed (chunk 2026-10-31-exit-cause-code-catalogue).'
    rationale: 'Report **Schema / config**: the `process-exit` `detail` enum gains `stale-heartbeat`, `unwrapped-name`, `endpoint-lost`, `program-not-found`, `version-unreadable` and `unclean-fixture`; **Symbols / APIs** lists `session-live` among revive''s codes.'
    basis: schemas/diag-line.v1.json:78
note: 'D-obs-instrumentation, D-obs-stack and D-obs-pii found no violation in this file: the report adds no hot-path operation or telemetry dependency, and the new codes are closed with no path or pid.'
