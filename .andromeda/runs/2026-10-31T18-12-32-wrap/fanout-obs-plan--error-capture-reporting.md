proposals:
  - detector: D-retired
    severity: warning
    file: .andromeda/obs-plan-detail/error-capture-reporting.md
    near: 27
    section: '## 7. Error Capture & Reporting'
    change: 'The `viola link` / `viola unlink` instance-check refusal under `--json` reads `{"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"}` in place of the `"detail":null` form; the rest of the line stands.'
    sidecar: 'error-capture-reporting: the link and unlink instance-check refusal under --json names its cause, strict-modes-failed (chunk 2026-10-31-exit-cause-code-catalogue).'
    rationale: 'The report **Retired** bullet retires the exit-21 document with `detail: null`; its `doc-null` count row gives a strict-modes site the word `strict-modes-failed`.'
    basis: src/cmd/client.rs:222
  - detector: D-retired
    severity: warning
    file: .andromeda/obs-plan-detail/error-capture-reporting.md
    near: 40
    section: '## 7. Error Capture & Reporting'
    change: 'A refused instance check in `frame` returns `instance-unreachable` with `detail: "strict-modes-failed"`, the `isError` code the liveness pre-check also returns under its own `detail`, and writes no frame and no line.'
    sidecar: 'error-capture-reporting: an MCP frame tool refused at its instance check reads detail strict-modes-failed, and no longer shares a body with the liveness pre-check (chunk 2026-10-31-exit-cause-code-catalogue).'
    rationale: 'The report **Retired** bullet retires `detail: null`; its **MCP** symbol bullet says a refused `check_instance` is `StrictModes`, and no `live_endpoint` is the `without_endpoint` reading, so the two bodies now differ.'
    dependent-of: D-retired
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/error-capture-reporting.md
    near: 16
    section: '## 7. Error Capture & Reporting'
    change: 'The `viola revive <name> --list` catch-site clause reads (`process-exit{detail:"internal-error"}`, exactly `error: internal error`, or under `--json` the `internal-error` document on stdout); the rest of the line stands.'
    sidecar: 'error-capture-reporting: the revive --list catch-site output gains its --json form, the internal-error document (chunk 2026-10-31-exit-cause-code-catalogue).'
    rationale: 'The report `catch-site` count row names this line among the sites stating the output under `--json`, each of which takes the document.'
    basis: src/main.rs:154
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/error-capture-reporting.md
    near: 17
    section: '## 7. Error Capture & Reporting'
    change: 'New sub-bullet after this line: `- **The `cli` catch site under `--json`:** where `prints_internal_error` is true and `json_asked` (`src/main.rs`: `--json` among the arguments before a `--`, read from argv as `role_of` is) finds the flag, `internal_error_document` (`src/cmd/mod.rs`) prints `{"v":1,"error":"internal-error","detail":null}` on stdout in place of `error: internal error`, stderr then empty. Without `--json` the line is byte for byte as it stood; `run`, the start of `revive`, `mcp` and `hook` stay silent there.`'
    sidecar: 'error-capture-reporting: the cli catch site under --json prints the internal-error document on stdout, stderr empty (chunk 2026-10-31-exit-cause-code-catalogue).'
    rationale: 'The report symbol bullet "The `cli` catch site under `--json`" adds `json_asked` and `internal_error_document`; the file holds the catch-site rules and has no record of this form.'
    basis: src/cmd/mod.rs:99
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/error-capture-reporting.md
    near: 18
    section: '## 7. Error Capture & Reporting'
    change: 'New sub-bullet after this line: `- **The start of `viola revive --json`:** every refusal of the start it shares with `run` prints one exit-1 document, a refused check on the start arm the `strict-modes-failed` document in place of the stderr pair, still with no process log opened. In `started` (`src/cmd/revive.rs`), an anyhow error from `trusted`, `checked_exe`, `open_wrapper_log`, `preflight` or `start` prints `refused_document("internal-error")` and returns the same error, so the catch site still logs its chain and its line. A `pump_child` error prints no document: the child has held the terminal, and viola writes nothing past it.`'
    sidecar: 'error-capture-reporting: the start of revive --json prints one exit-1 document for each refusal and for an anyhow error before the child, and none for a pump_child error (chunk 2026-10-31-exit-cause-code-catalogue).'
    rationale: 'The report symbol bullet "`viola revive --json`" states these codes and the `pump_child` exception, and its **Retired** bullet closes the post-preflight gap; the record stands apart from the line-16 clause because that line would pass the 2 000-char entity limit with it.'
    basis: src/cmd/revive.rs:119
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/error-capture-reporting.md
    near: 40
    section: '## 7. Error Capture & Reporting'
    change: 'New bullet after this line: `- The `instance-unreachable` `isError` result of `viola mcp` names its cause in `detail` (`unreachable(cause)`, `crates/viola-mcp/src/result.rs`): the CLI document less `v`, its text still `refused: instance-unreachable`. In `frame` a refused `check_instance` is `strict-modes-failed`, no `live_endpoint` is the `without_endpoint` reading, and a failed `server_verified` is `server-verify-failed`; in `of_reply` `Connect` is `instance-dead` and `Closed | Io` is `endpoint-lost`; in `of_board` the refusal is `strict-modes-failed`. No frame and no line on any of them, `isError` the record; JSON-RPC errors are unmoved and `-32603` keeps its fixed body.`'
    sidecar: 'error-capture-reporting: each MCP instance-unreachable result names its cause in detail, with no frame and no line (chunk 2026-10-31-exit-cause-code-catalogue).'
    rationale: 'The report **MCP** symbol bullet maps each path to its cause, and the coverage row for MCP `detail` causes records no line by `inputs#I2` Q4.'
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/error-capture-reporting.md
    near: 51
    section: '## 7. Error Capture & Reporting'
    change: 'New bullet after this line: `- Exit 21 prints one document per cause, `{"v":1,"error":"instance-unreachable","detail":"<cause>"}`, with `v` still 1 and `error` unmoved; `detail` is one of the five closed words of `viola_core::UnreachableCause`: `strict-modes-failed`, `instance-dead`, `unwrapped-name`, `endpoint-lost`, `server-verify-failed`. No serde path reaches them: every word is written by the two clients. `unreachable_document` (`src/cmd/client.rs`) is the one writer of the document, with `refuse_untrusted` and `src/cmd/plugin.rs` through it; `refused_document` in `src/cmd/list.rs` builds `strict-modes-failed` as a `Value` for `board_document`, the only refusal of the board.`'
    sidecar: 'error-capture-reporting: the exit-21 document names one of five closed causes, and its one writer is recorded (chunk 2026-10-31-exit-cause-code-catalogue).'
    rationale: 'The report symbol bullets "The exit-21 document moved", `UnreachableCause` and `src/cmd/client.rs` state each fact; the group this follows holds the error-surface rules.'
    basis: crates/viola-core/src/lib.rs:181
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/error-capture-reporting.md
    near: 51
    section: '## 7. Error Capture & Reporting'
    change: 'New bullet after this line: `- `viola_state::liveness::without_endpoint` gives a caller with no endpoint its cause: `symlink_metadata` reads a directory `instance-dead`, and anything else (absent, a regular file, a symlink) `unwrapped-name`; it opens nothing and writes no line. The strict-modes check stays first at every caller, `Unreachable::without_endpoint` in `src/cmd/client.rs` and `frame` in `crates/viola-mcp/src/tools.rs`.`'
    sidecar: 'error-capture-reporting: without_endpoint reads instance-dead or unwrapped-name from the instance directory, opening nothing and writing no line (chunk 2026-10-31-exit-cause-code-catalogue).'
    rationale: 'The report symbol bullet `viola_state::liveness::without_endpoint` states the reading, its callers and that it writes no line.'
    basis: crates/viola-state/src/liveness.rs:107
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/error-capture-reporting.md
    near: 51
    section: '## 7. Error Capture & Reporting'
    change: 'New bullet after this line: `- `refused_with(json, detail, human)` (`src/cmd/run.rs`) prints `refused_document` in place of the stderr pair when `Launch.json` is set (`run` passes false, `revive` passes `args.json`), for `batch-script-child`, `pinned-hash-mismatch`, `squatted-name`, and for a program not found `internal-error` (no pair today, none added). All return before `spawn_child`, and the human pairs are byte for byte as they stood.`'
    sidecar: 'error-capture-reporting: refused_with prints the start refusal documents under revive --json, before spawn_child (chunk 2026-10-31-exit-cause-code-catalogue).'
    rationale: 'The report symbol bullet "`run`''s start under `--json`" adds `refused_with` and the `json` field of `Launch`, and names the codes.'
    basis: src/cmd/run.rs:422
note: line 16 holds 1906 chars and 1965 with its catch-site clause, so the revive start-arm form is its own sub-bullet; D-obs-instrumentation, D-obs-stack and D-obs-pii found no violation in this file; verify's new codes are left to the §6 catalog.
