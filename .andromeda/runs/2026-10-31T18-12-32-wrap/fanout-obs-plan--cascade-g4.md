proposals:
  - detector: D-retired
    severity: warning
    file: .andromeda/obs-plan-detail/metric-coverage.md
    near: 37
    section: '## 5. Metric Coverage'
    change: 'MCP''s `wait` tool refused by its `check_instance` reads (`instance-unreachable`, `detail: "strict-modes-failed"`) in place of `detail: null`; it still writes no line, and the exit code or the `isError` code stays its only record.'
    sidecar: 'metric-coverage: MCP''s refused `wait` tool now carries `detail` `strict-modes-failed`; still no line, so no metric counts it.'
    rationale: 'Report Retired names the exit-21 document''s `detail: null`; Symbols / APIs MCP: a refused `check_instance` is `StrictModes`, no frame and no line, `isError` the record.'
  - detector: D-retired
    severity: warning
    file: .andromeda/obs-plan-detail/log-coverage.md
    near: 26
    section: '## 6. Log Coverage'
    change: 'The exit-1 list reads `already-live` (a `live` holder of the name or a fresh replayed beat, from `viola run` or from `viola revive`, which prints `run`''s own collision pair, or under `--json` the `{"v":1,"error":"already-live","detail":null}` document), then `stale-heartbeat` (a `stale` holder, the same two callers, under `revive --json` the same-shaped document); a `gone` holder stays taken over with no exit-1 line.'
    sidecar: 'log-coverage: `already-live` no longer covers a stale holder; `stale-heartbeat` is that holder''s printed and logged code.'
    rationale: 'Report Retired names `already-live` as a stale holder''s refusal; Symbols / APIs: `collision_check` prints and logs `stale-heartbeat` for a `stale` holder, and `already-live` for a live holder and a fresh replayed beat; Counts `stale-live` lists `log-coverage.md:26` as changed.'
    basis: .andromeda/obs-plan-detail/log-coverage.md:26
  - detector: D-retired
    severity: warning
    file: .andromeda/obs-plan-detail/log-coverage.md
    near: 26
    section: '## 6. Log Coverage'
    change: 'In place of "revive''s preflight refusals under `--json` are the same-shaped document with their code" read "every refusal of revive''s start under `--json` is the same-shaped document with its code", the logged line unchanged.'
    sidecar: 'log-coverage: under `revive --json` every refusal of the start it shares with `run` prints its document, not the preflight''s alone.'
    rationale: 'Report Retired names the `revive --json` post-preflight gap; Symbols / APIs: `viola revive --json` prints one exit-1 document for every refusal of the start it shares with `run`.'
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/log-coverage.md
    near: 26
    section: '## 6. Log Coverage'
    change: 'Add after it a continuation line: `stale-heartbeat` is exit 1''s cause alone, written by `collision_check` in `src/cmd/run.rs` for a stale holder; `live_endpoint` gives none for a stale wrapper, so a frame verb against a stale instance reads `instance-dead` (chunk 2026-10-31-exit-cause-code-catalogue).'
    sidecar: 'log-coverage: `stale-heartbeat` is exit 1''s cause alone; a frame verb against a stale instance reads `instance-dead`.'
    rationale: 'Report Symbols / APIs, a stale holder''s code: stale heartbeat is exit 1''s cause alone, and `live_endpoint` gives none for a stale wrapper.'
    fact: F3
  - detector: D-cascade
    severity: warning
    file: .andromeda/obs-plan-detail/log-coverage.md
    near: 28
    section: '## 6. Log Coverage'
    change: '`schemas/diag-line.v1.json` closes the `process-exit` `detail` at 19 values (the enum''s members; 13 before chunk 2026-10-31-exit-cause-code-catalogue, 12 before chunk 2026-10-16-unwrapped-sessions-on-the-board, 11 before chunk 2026-10-13-interrupted-verify-cleanup, 9 before chunk 2026-10-10-viola-revive).'
    sidecar: 'log-coverage: the `process-exit` `detail` enum count moves from 13 to 19.'
    rationale: 'Report Counts `enum-count`: enum 13 to 19 members, `log-coverage.md:28` changed; Schema / config: the enum gains six.'
    basis: schemas/diag-line.v1.json:78
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/log-coverage.md
    near: 28
    section: '## 6. Log Coverage'
    change: 'Add after it a continuation line: `viola verify` writes its refusal''s code on the `process-exit` it writes under a parsed `VIOLA_NAME`: `batch-script-child`, `program-not-found`, `pinned-hash-mismatch`, `version-unreadable`, `strict-modes-failed`, `unclean-fixture`, beside the standing `interrupted`; a refused home tree stays lineless, a failing row is no refusal and no code, the fixture''s file name enters no code, and `verify` takes no `--json` (chunk 2026-10-31-exit-cause-code-catalogue).'
    sidecar: 'log-coverage: `verify` writes a code per refusal on its `process-exit`.'
    rationale: 'Report Symbols / APIs, `verify`''s codes: `Ended::Refused` and its code on the `process-exit` under a parsed `VIOLA_NAME`; Counts `verify-codes`: new text wherever the catalog stands.'
    fact: F2
  - detector: D-cascade
    severity: warning
    file: .andromeda/obs-plan-detail/log-coverage.md
    near: 29
    section: '## 6. Log Coverage'
    change: '`process-exit` exit 21: `instance-dead`, `unwrapped-name`, `endpoint-lost`, `strict-modes-failed`, `server-verify-failed`.'
    sidecar: 'log-coverage: the exit-21 `process-exit` list gains `unwrapped-name` and `endpoint-lost`.'
    rationale: 'Report Schema / config: the enum gains both words; Outcome: each logged cause is one `process-exit` with its own `detail`; Symbols / APIs: `Unreachable` is now `Dead`, `Unwrapped`, `Lost`, `ServerUnverified`.'
    basis: schemas/diag-line.v1.json:78
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/log-coverage.md
    near: 30
    section: '## 6. Log Coverage'
    change: 'Add after it a continuation line: chunk 2026-10-31-exit-cause-code-catalogue adds six `process-exit` `detail` words to the catalog: exit 21 `unwrapped-name` and `endpoint-lost`, exit 1 `stale-heartbeat`, `program-not-found`, `version-unreadable` and `unclean-fixture`.'
    sidecar: 'log-coverage: the `process-exit` `detail` catalog gains its six new words.'
    rationale: 'Report Schema / config: the `process-exit` `detail` enum gains these six; `channel-response` and `hook-decision` did not move.'
    basis: schemas/diag-line.v1.json:78
    fact: F1
  - detector: D-cascade
    severity: warning
    file: .andromeda/obs-plan-detail/log-coverage.md
    near: 37
    section: '## 6. Log Coverage'
    change: 'The head reads `strict-modes-failed` (on `process-exit` exit 21, on the `process-exit` exit 1 `viola verify` writes under a parsed `VIOLA_NAME`, and on `parse-rejected{parser:"ledger-stamps"}` from `run`''s version gate, at `warn`, with no path; the rest of the parenthesis as it stands, no exit-21 arm writing the word.'
    sidecar: 'log-coverage: `strict-modes-failed` also stands on `verify`''s own exit-1 `process-exit`; no exit-21 arm writes it, and no refused check gained a line.'
    rationale: 'Report Symbols / APIs, `verify`''s codes: `strict-modes-failed` is one of its `process-exit` codes, a refused home tree staying lineless; Outcome: no refused check gains a line or moves.'
  - detector: D-retired
    severity: warning
    file: .andromeda/obs-plan-detail/log-coverage.md
    near: 39
    section: '## 6. Log Coverage'
    change: '`viola mcp`''s frame tools read (`instance-unreachable`, `detail: "strict-modes-failed"`) in place of `detail: null`, still writing no line.'
    sidecar: 'log-coverage: MCP''s refused frame tools now carry `detail` `strict-modes-failed`.'
    rationale: 'Report Retired names the exit-21 document''s `detail: null`; Symbols / APIs MCP: a refused `check_instance` is `StrictModes`; Expected amendments lists `log-coverage.md:39`.'
    basis: .andromeda/obs-plan-detail/log-coverage.md:39
    dependent-of: D-retired
  - detector: D-cascade
    severity: warning
    file: .andromeda/obs-plan-detail/error-capture-reporting.md
    near: 16
    section: '## 7. Error Capture & Reporting'
    change: 'The `revive --list` clause reads (`process-exit{detail:"internal-error"}`, exactly `error: internal error`, under `--json` the `internal-error` document on stdout); the rest of the line as it stands.'
    sidecar: 'error-capture-reporting: `revive --list` at the `cli` catch site prints the `internal-error` document under `--json`.'
    rationale: 'Report Counts `catch-site`: a site stating the output under `--json` too takes the document, `error-capture-reporting.md:16` named; Symbols / APIs, the `cli` catch site under `--json`.'
    basis: .andromeda/obs-plan-detail/error-capture-reporting.md:16
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/error-capture-reporting.md
    near: 17
    section: '## 7. Error Capture & Reporting'
    change: 'Add after it:   - **`cli` catch site under `--json`:** the `internal-error` document `{"v":1,"error":"internal-error","detail":null}` is printed on stdout in place of `error: internal error`, stderr then empty, decided by `json_asked`, which reads `--json` from argv before any `--`; without `--json` the line stands byte for byte (chunk 2026-10-31-exit-cause-code-catalogue).'
    sidecar: 'error-capture-reporting: the `cli` catch site''s `--json` form, decided by `json_asked` from argv.'
    rationale: 'Report Symbols / APIs, the `cli` catch site under `--json`: `json_asked` and `internal_error_document`; tests `cli_verbs_print_the_internal_error_document_under_json`.'
    basis: src/main.rs:154-161
    fact: F6
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/error-capture-reporting.md
    near: 18
    section: '## 7. Error Capture & Reporting'
    change: 'Add after it:   - **`revive --json` start errors:** an anyhow error from `trusted`, `checked_exe`, `open_wrapper_log`, `preflight` or `start` prints the `internal-error` document and returns the same error, so the catch site still logs its chain and its line (chunk 2026-10-31-exit-cause-code-catalogue).'
    sidecar: 'error-capture-reporting: `revive --json` prints the `internal-error` document on a start error, and the catch site still logs it.'
    rationale: 'Report Symbols / APIs, `viola revive --json`: the anyhow errors of `started` print `refused_document("internal-error")` and return the same error.'
    basis: src/cmd/revive.rs:119-134
    fact: F7
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/error-capture-reporting.md
    near: 18
    section: '## 7. Error Capture & Reporting'
    change: 'Add after it:   - **`pump_child` under `revive --json`:** its error prints no document: the child has held the terminal, and viola writes nothing past it (chunk 2026-10-31-exit-cause-code-catalogue).'
    sidecar: 'error-capture-reporting: a `pump_child` error under `revive --json` prints no document.'
    rationale: 'Report Symbols / APIs, `viola revive --json`; Reverted / negative API facts: a `Started`-time document from `main` was rejected because a `pump_child` error reaches the same catch site after the child has held the terminal.'
    basis: src/cmd/revive.rs:119-134
    fact: F8
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/error-capture-reporting.md
    near: 18
    section: '## 7. Error Capture & Reporting'
    change: 'Add after it:   - **Start refusals under `--json`:** `refused_with` prints the exit-1 document in place of the human pair for `batch-script-child`, `pinned-hash-mismatch` and `squatted-name`, and `internal-error` for a program not found (no human pair today, none added), all before `spawn_child`; `run` passes `json` false and `revive` passes its `--json` (chunk 2026-10-31-exit-cause-code-catalogue).'
    sidecar: 'error-capture-reporting: `refused_with` prints the start refusal''s exit-1 document under `revive --json`, before `spawn_child`.'
    rationale: 'Report Symbols / APIs, `run`''s start under `--json`: `Launch` gains `json`, and `refused_with` prints `refused_document` in place of the pair, all returning before `spawn_child`.'
    basis: src/cmd/run.rs:422-431
    fact: F9
  - detector: D-cascade
    severity: warning
    file: .andromeda/obs-plan.md
    near: 523
    section: '## 7. Error Capture & Reporting'
    change: 'Short-lived `cli` reads: then exactly `error: internal error\n` on stderr in one `write_all`, with no hint, or under `--json` the `{"v":1,"error":"internal-error","detail":null}` document on stdout with stderr empty, and exit 1.'
    sidecar: 'obs-plan: §7''s `cli` per-role line states the catch site''s `--json` document beside the human line.'
    rationale: 'Report Counts `catch-site`: the human line stands byte for byte and a site stating the output under `--json` too takes the document, `obs-plan.md:523` named.'
    basis: .andromeda/obs-plan.md:523
  - detector: D-retired
    severity: warning
    file: .andromeda/obs-plan-detail/error-capture-reporting.md
    near: 27
    section: '## 7. Error Capture & Reporting'
    change: 'The `viola link` / `viola unlink` refusal reads under `--json` `{"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"}` in place of `"detail":null`, still writing no line in any log.'
    sidecar: 'error-capture-reporting: the `link` / `unlink` refused check''s document now names `strict-modes-failed`.'
    rationale: 'Report Retired names the exit-21 document''s `detail: null`; Counts `doc-null`: a strict-modes site reads `strict-modes-failed`.'
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    file: .andromeda/obs-plan-detail/error-capture-reporting.md
    near: 40
    section: '## 7. Error Capture & Reporting'
    change: 'A refusal returns `instance-unreachable` with `detail: "strict-modes-failed"`, the `isError` code of the liveness pre-check, whose own `detail` is that reading''s cause, and writes no frame and no line.'
    sidecar: 'error-capture-reporting: MCP''s refused instance check now carries `detail` `strict-modes-failed`.'
    rationale: 'Report Retired names the exit-21 document''s `detail: null`; Symbols / APIs MCP: `tools.rs` `frame`, a refused `check_instance` is `StrictModes`, no frame and no line.'
    dependent-of: D-retired
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/error-capture-reporting.md
    near: 40
    section: '## 7. Error Capture & Reporting'
    change: 'Add after it: - MCP maps its refusals to causes: a refused `check_instance` or board is `StrictModes`, no `live_endpoint` is `without_endpoint`''s reading, a failed `server_verified` is `ServerVerify`, `Connect` is `InstanceDead`, and `Closed` or `Io` is `EndpointLost` (chunk 2026-10-31-exit-cause-code-catalogue).'
    sidecar: 'error-capture-reporting: MCP''s map from each refusal to its `UnreachableCause`.'
    rationale: 'Report Symbols / APIs MCP: `result.rs` `unreachable(cause)`, `of_reply`, `of_board` and `tools.rs` `frame`.'
    fact: F10
  - detector: D-retired
    severity: warning
    file: .andromeda/obs-plan-detail/pii-scrubbing-compliance.md
    near: 40
    section: '## 8. PII Scrubbing & Compliance › ### PII Scrubbing'
    change: 'Under `--json` the six frame verbs print exactly `{"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"}`, and the `viola mcp` frame tools refused by the same check return `instance-unreachable` with `detail: "strict-modes-failed"`, each writing no line.'
    sidecar: 'pii-scrubbing-compliance: the strict-modes refusal''s document and MCP body now name `strict-modes-failed`; no path, pid or endpoint and no line, as before.'
    rationale: 'Report Retired names the exit-21 document''s `detail: null`; Coverage: the causes are a closed enum with no path, pid or upstream text, so D-obs-pii holds; Symbols / APIs: `refuse_untrusted` writes `StrictModes`.'
    dependent-of: D-retired
note: 'Not this group''s and owed elsewhere: `obs-plan.md:361` and F4, F5 (span-trace-coverage), F11, F12 and the Vector 7 trigger''s "only", which `verify`''s `strict-modes-failed` code reaches (obs-scope-summary); the §6 `detail` catalog heading at `obs-plan.md:451` names no list or count and stays; D-obs-instrumentation, D-obs-stack and D-obs-pii fire nothing here.'
