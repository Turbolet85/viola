proposals:
  - detector: D-retired
    severity: warning
    file: .andromeda/obs-plan-detail/log-coverage.md
    near: 26
    section: "## 6. Log Coverage"
    change: In line 26 the first code's parenthetical opens `already-live` (a `live` holder of the name or a fresh replayed beat, from `viola run` or from `viola revive`, the rest of that parenthetical as it stands), and a new code follows it, `stale-heartbeat` (a `stale` holder of the name, printed and logged by `collision_check`, under `revive --json` the same-shaped document with its code; chunk 2026-10-31-exit-cause-code-catalogue); the rest of the line stands.
    sidecar: log-coverage exit-1 codes, a stale holder now reads `stale-heartbeat` and `already-live` keeps a live holder and a fresh replayed beat.
    rationale: The report's Retired bullet names `already-live` as a stale holder's refusal, and Symbols / APIs (A stale holder's code) and Counts (`stale-live`) name log-coverage.md:26 as changed.
    basis: .andromeda/obs-plan-detail/log-coverage.md:26
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/log-coverage.md
    near: 27
    section: "## 6. Log Coverage"
    change: New line after line 27, indented as line 27 is, ` `viola revive --json` prints one exit-1 document for every refusal of the start it shares with `run`, `batch-script-child`, `pinned-hash-mismatch` and `squatted-name` by their codes, a program not found and any anyhow error from `trusted`, `checked_exe`, `open_wrapper_log`, `preflight` or `start` as `internal-error`, each logged line unchanged and the catch site still logging its chain; a `pump_child` error prints no document, the child having held the terminal (chunk 2026-10-31-exit-cause-code-catalogue).`
    sidecar: log-coverage exit-1 codes, `revive --json` start refusals now print their documents with the logged lines unchanged.
    rationale: The report's Symbols / APIs (`run`'s start under `--json`, `viola revive --json`) closes the post-preflight gap its Retired bullet names, and the file's exit-1 group states the document form for the preflight refusals only.
    basis: src/cmd/revive.rs:119
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/log-coverage.md
    near: 28
    section: "## 6. Log Coverage"
    change: In line 28 `schemas/diag-line.v1.json` closes the `process-exit` `detail` at 19 values (the enum's members; 13 before chunk 2026-10-31-exit-cause-code-catalogue, 12 before chunk 2026-10-16-unwrapped-sessions-on-the-board, 11 before chunk 2026-10-13-interrupted-verify-cleanup, 9 before chunk 2026-10-10-viola-revive).
    sidecar: log-coverage `process-exit` `detail` enum count 13 to 19, the six new members from this chunk.
    rationale: The report's Schema / config adds six members at schemas/diag-line.v1.json:78, and Counts (`enum-count`) names log-coverage.md:28 as changed.
    basis: schemas/diag-line.v1.json:78
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/log-coverage.md
    near: 28
    section: "## 6. Log Coverage"
    change: New line after line 28, indented as line 28 is, ` `viola verify` under a parsed `VIOLA_NAME` also ends each refusal with one `process-exit{subject:"self", exit_code:1, detail}` in `cli-<name>.ndjson`, its `detail` one of `batch-script-child`, `program-not-found`, `pinned-hash-mismatch`, `version-unreadable`, `strict-modes-failed`, `unclean-fixture`, beside `interrupted` (`strict-modes-failed` here is verify's own, the start arm's refused instance check staying lineless); a refused home tree stays lineless, a failing row is no refusal and carries no code, the fixture's `<file>` enters no code, and `verify` takes no `--json` (chunk 2026-10-31-exit-cause-code-catalogue).`
    sidecar: log-coverage exit-1 codes, `viola verify`'s refusals each named on its `process-exit`.
    rationale: The report's Symbols / APIs (`verify`'s codes) and Counts (`verify-codes`, new text wherever the catalog stands) bring these codes, and the file names only `interrupted` for `verify`.
    basis: schemas/diag-line.v1.json:78
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/log-coverage.md
    near: 29
    section: "## 6. Log Coverage"
    change: Line 29 reads - `process-exit` exit 21 with `instance-dead`, `unwrapped-name`, `endpoint-lost`, `strict-modes-failed`, `server-verify-failed`.
    sidecar: log-coverage exit-21 codes gain `unwrapped-name` and `endpoint-lost`.
    rationale: The report's Symbols / APIs names the five closed causes, and its Schema / config admits `unwrapped-name` and `endpoint-lost` to the `process-exit` `detail` enum.
    basis: crates/viola-core/src/lib.rs:181
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/log-coverage.md
    near: 30
    section: "## 6. Log Coverage"
    change: New line after line 30, indented as line 30 is, ` `instance-dead` (a directory at the name with no endpoint, a stale wrapper among them since `live_endpoint` gives it none, or a refused connect) and `unwrapped-name` (anything else at the name, absent, a regular file or a symlink, by `without_endpoint`'s reading after the strict-modes check) carry `during:"connect"`; `endpoint-lost` (the wrapper lost with the call open) carries `during:"call"`, and `instance-dead` no longer does; `viola mcp`'s frame tools write no line for any cause, the `isError` body its record (chunk 2026-10-31-exit-cause-code-catalogue).`
    sidecar: log-coverage exit-21 causes, each cause's reading and `during` value, with `during` `call` moved from `instance-dead` to `endpoint-lost`.
    rationale: The report's Symbols / APIs (`without_endpoint`, A process-exit `during`, A stale holder's code, MCP) and Retired (`instance-dead` with `during` `call`) define these causes, and the file states none of them.
    basis: crates/viola-state/src/liveness.rs:107
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/log-coverage.md
    near: 37
    section: "## 6. Log Coverage"
    change: In line 37's opening parenthetical `strict-modes-failed` also stands on `process-exit` exit 1 from `viola verify` under a parsed `VIOLA_NAME` (chunk 2026-10-31-exit-cause-code-catalogue); the sentence that no arm writes the word on `process-exit` exit 21 stands.
    sidecar: log-coverage filesystem refusals, `strict-modes-failed` also on `viola verify`'s `process-exit` exit 1.
    rationale: The report's Symbols / APIs (`verify`'s codes) lists `strict-modes-failed` on the `process-exit` `verify` writes, and this list names only exit 21 and `parse-rejected` for the word.
    dependent-of: D-new-detail
  - detector: D-retired
    severity: warning
    file: .andromeda/obs-plan-detail/log-coverage.md
    near: 39
    section: "## 6. Log Coverage"
    change: In line 39 `viola mcp`'s frame tools read (`instance-unreachable`, `detail:"strict-modes-failed"`) in place of the null detail, and the CLI's form through `client::refuse_untrusted` under `--json` reads the `instance-unreachable` document with `detail:"strict-modes-failed"`; the rest of the line stands.
    sidecar: log-coverage refused strict-modes checks, the exit-21 document and MCP body now carry `strict-modes-failed` in `detail`.
    rationale: The report's Retired bullet names the exit-21 document's null detail, and its Expected amendments list log-coverage.md:39 under obs-plan with `doc-null` and `detail-null`.
    basis: .andromeda/obs-plan-detail/log-coverage.md:39
note: D-obs-instrumentation, D-obs-stack and D-obs-pii found no violation in this file, the report adding no hot-path operation, no telemetry dependency and no line with user data.
