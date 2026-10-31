# Apply — obs-plan · part 3 of 4

## Verdicts
- fanout-obs-plan--cascade-g4.md 2 · applied · .andromeda/obs-plan-detail/log-coverage.md:26#1, .andromeda/obs-plan-detail/log-coverage.md:26#3
- fanout-obs-plan--cascade-g4.md 3 · applied · .andromeda/obs-plan-detail/log-coverage.md:26#2
- fanout-obs-plan--cascade-g4.md 4 · applied · .andromeda/obs-plan-detail/log-coverage.md:26+#1
- fanout-obs-plan--cascade-g4.md 5 · applied · .andromeda/obs-plan-detail/log-coverage.md:28#1
- fanout-obs-plan--cascade-g4.md 6 · applied · .andromeda/obs-plan-detail/log-coverage.md:26#4, .andromeda/obs-plan-detail/log-coverage.md:28+#1
- fanout-obs-plan--cascade-g4.md 7 · applied · .andromeda/obs-plan-detail/log-coverage.md:29#1
- fanout-obs-plan--cascade-g4.md 8 · folded into fanout-obs-plan--cascade-g4.md 5
- fanout-obs-plan--cascade-g4.md 9 · applied · .andromeda/obs-plan-detail/log-coverage.md:37#1
- fanout-obs-plan--cascade-g4.md 10 · applied · .andromeda/obs-plan-detail/log-coverage.md:39#1
- fanout-obs-plan--cascade-g4.md 11 · applied · .andromeda/obs-plan-detail/error-capture-reporting.md:16#1
- fanout-obs-plan--cascade-g4.md 12 · applied · .andromeda/obs-plan-detail/error-capture-reporting.md:17+#1
- fanout-obs-plan--cascade-g4.md 13 · applied · .andromeda/obs-plan-detail/error-capture-reporting.md:18+#1
- fanout-obs-plan--cascade-g4.md 14 · applied · .andromeda/obs-plan-detail/error-capture-reporting.md:18+#2
- fanout-obs-plan--cascade-g4.md 15 · applied · .andromeda/obs-plan-detail/error-capture-reporting.md:18+#3
- fanout-obs-plan--cascade-g4.md 17 · applied · .andromeda/obs-plan-detail/error-capture-reporting.md:27#1
- fanout-obs-plan--cascade-g4.md 18 · applied · .andromeda/obs-plan-detail/error-capture-reporting.md:40#1
- fanout-obs-plan--cascade-g4.md 19 · applied · .andromeda/obs-plan-detail/error-capture-reporting.md:40+#1
- cascade-compare-obs-plan.md 1 · applied · .andromeda/obs-plan-detail/error-capture-reporting.md:51+#1
- fanout-obs-plan--cascade-g4.md 20 · applied · .andromeda/obs-plan-detail/pii-scrubbing-compliance.md:40#1, .andromeda/obs-plan-detail/pii-scrubbing-compliance.md:40#2

## Edits
- line: .andromeda/obs-plan-detail/log-coverage.md:26
  old: «(a `live` or a `stale` holder of the name, from»
  new: «(a `live` holder of the name or a fresh replayed beat, from»
- line: .andromeda/obs-plan-detail/log-coverage.md:26
  old: «revive's preflight refusals under `--json` are the same-shaped document with their code, the logged line unchanged (chunk 2026-10-17-cli-machine-contract);»
  new: «every refusal of revive's start under `--json` is the same-shaped document with its code, the logged line unchanged;»
- line: .andromeda/obs-plan-detail/log-coverage.md:26
  old: «is taken over with no exit-1 line), `squatted-name`,»
  new: «is taken over with no exit-1 line), `stale-heartbeat` (a `stale` holder, from the same two, under `revive --json` the same-shaped document), `squatted-name`,»
- line: .andromeda/obs-plan-detail/log-coverage.md:26
  old: «`strict-modes-failed` is no longer one of them (chunk 2026-10-10-budget-governor):»
  new: «On `run`'s and `revive`'s start `strict-modes-failed` is not one of them (chunk 2026-10-10-budget-governor; `viola verify`'s stamps refusal carries the word, below):»
- after: .andromeda/obs-plan-detail/log-coverage.md:26
  anchor: «- `process-exit` exit 1: `already-live` (a `live` or a `stale` holder of the name, from `viola run` or from `viola revive`, which prints `run`'s own collision pair, or under `--json` the `{"v":1,"error":"already-live","detail":null}` document — revive's preflight refusals under `--json` are the same-shaped document with their code, the logged line unchanged (chunk 2026-10-17-cli-machine-contract); a `gone` holder is taken over with no exit-1 line), `squatted-name`, `pinned-hash-mismatch`, `batch-script-child`, `internal-error`, and from `viola revive`'s start-arm preflight `no-session` (no logged session, or an `--id` the log does not hold), `session-live` (one `claude agents --json` read lists the logged session id live in another `claude`; skipped under `--fork`; a failed, oversize or malformed read passes, fail open; chunk 2026-10-16-unwrapped-sessions-on-the-board) and `cwd-missing` (the snapshot records no directory, or one that is not an existing directory). The preflight reads the holder, then the session, then `session-live`, then the directory, and takes no later reading once one refuses; each refusal is one `process-exit{subject:"self", exit_code:1, detail}` in `run-<name>.ndjson` with no child started. `strict-modes-failed` is no longer one of them (chunk 2026-10-10-budget-governor): the start arm's instance check (`check_instance`) runs before the wrapper log opens and outside the preflight, so its refusal is the stderr `unable:` / `hint:` pair (or under `--json` the `strict-modes-failed` document, chunk 2026-10-17-cli-machine-contract) and exit 1 with no log opened, no `process-start`, no `process-exit` and no child.»
  add: « `stale-heartbeat` is exit 1's cause alone, written by `collision_check` (`src/cmd/run.rs`): `live_endpoint` gives none for a stale wrapper, so a frame verb against a stale instance reads `instance-dead`.»
- line: .andromeda/obs-plan-detail/log-coverage.md:28
  old: «at 13 values (the enum's members; 12 before chunk 2026-10-16-unwrapped-sessions-on-the-board, 11 before chunk 2026-10-13-interrupted-verify-cleanup, 9 before chunk 2026-10-10-viola-revive).»
  new: «at 19 values (the enum's members: the words of the exit-1, exit-21 and exit-20 lists here, as measured at chunk 2026-10-31-exit-cause-code-catalogue).»
- after: .andromeda/obs-plan-detail/log-coverage.md:28
  anchor: « And `interrupted`: a `viola verify` stopped by Ctrl-C or TERM (Unix `SIGINT` / `SIGTERM`, Windows Ctrl-C / Ctrl-Break) before its rows are checked ends with one `process-exit{subject:"self", exit_code:1, detail:"interrupted"}` in `cli-<name>.ndjson` under a `VIOLA_NAME` (no file with no instance, D-06), after every spawn pair it opened is closed (chunk 2026-10-13-interrupted-verify-cleanup). `schemas/diag-line.v1.json` closes the `process-exit` `detail` at 13 values (the enum's members; 12 before chunk 2026-10-16-unwrapped-sessions-on-the-board, 11 before chunk 2026-10-13-interrupted-verify-cleanup, 9 before chunk 2026-10-10-viola-revive).»
  add: « `viola verify` writes its refusal's code on the `process-exit` it writes under a parsed `VIOLA_NAME`: `batch-script-child`, `program-not-found`, `pinned-hash-mismatch`, `version-unreadable`, `strict-modes-failed`, `unclean-fixture`, beside `interrupted`. A refused home tree writes no line, a failing row is no refusal and carries no code, the fixture's `<file>` enters no code, and `verify` takes no `--json`.»
- line: .andromeda/obs-plan-detail/log-coverage.md:29
  old: «`process-exit` exit 21: `instance-dead`, `strict-modes-failed`,»
  new: «`process-exit` exit 21: `instance-dead`, `unwrapped-name`, `endpoint-lost`, `strict-modes-failed`,»
- line: .andromeda/obs-plan-detail/log-coverage.md:37
  old: «`strict-modes-failed` (on `process-exit` exit 21, and on `parse-rejected{parser:"ledger-stamps"}`»
  new: «`strict-modes-failed` (on `process-exit` exit 21, on the `process-exit` exit 1 `viola verify` writes under a parsed `VIOLA_NAME`, and on `parse-rejected{parser:"ledger-stamps"}`»
- line: .andromeda/obs-plan-detail/log-coverage.md:39
  old: «`viola mcp`'s frame tools (`instance-unreachable`, `detail: null`);»
  new: «`viola mcp`'s frame tools (`instance-unreachable`, `detail: "strict-modes-failed"`);»
- line: .andromeda/obs-plan-detail/error-capture-reporting.md:16
  old: «(`process-exit{detail:"internal-error"}`, exactly `error: internal error`);»
  new: «(`process-exit{detail:"internal-error"}`, exactly `error: internal error`, under `--json` the `internal-error` document on stdout);»
- after: .andromeda/obs-plan-detail/error-capture-reporting.md:17
  anchor: «  - **Catch-site reach:** on Unix a home under a regular file is a strict-modes refusal with no line — `viola verify` exits 1 with the home pair, the frame verbs exit 21 with the instance pair — so it never reaches the `cli` catch site; on Windows such a path stats `NotFound`, the check skips it as absent, and the catch-site line `process-exit{detail:"internal-error"}` stands.»
  add: «  - **`cli` catch site under `--json`:** `internal_error_document` (`src/cmd/mod.rs`) prints `{"v":1,"error":"internal-error","detail":null}` on stdout in place of `error: internal error`, stderr then empty, decided by `json_asked` (`src/main.rs`), which reads `--json` from argv before any `--`. Without `--json` the line is `error: internal error`; `run`, `revive`'s start, `mcp` and `hook` print neither there.»
- after: .andromeda/obs-plan-detail/error-capture-reporting.md:18
  anchor: «  - **`mcp` at the catch site:** the `mcp` role writes no self `process-exit` line: `internal_error_exit_line` (`src/obs.rs`) writes for `Run` and `Cli` only.»
  add: «  - **`revive --json` start errors:** an anyhow error from `trusted`, `checked_exe`, `open_wrapper_log`, `preflight` or `start` prints the `internal-error` document and returns the same error, so the catch site logs its chain and its line.»
- after: .andromeda/obs-plan-detail/error-capture-reporting.md:18
  anchor: «  - **`mcp` at the catch site:** the `mcp` role writes no self `process-exit` line: `internal_error_exit_line` (`src/obs.rs`) writes for `Run` and `Cli` only.»
  add: «  - **`pump_child` under `revive --json`:** its error prints no document: the child has held the terminal, and viola writes nothing past it.»
- after: .andromeda/obs-plan-detail/error-capture-reporting.md:18
  anchor: «  - **`mcp` at the catch site:** the `mcp` role writes no self `process-exit` line: `internal_error_exit_line` (`src/obs.rs`) writes for `Run` and `Cli` only.»
  add: «  - **Start refusals under `--json`:** `refused_with` (`src/cmd/run.rs`) prints the exit-1 document in place of the human pair for `batch-script-child`, `pinned-hash-mismatch` and `squatted-name`, and the `internal-error` document for a program not found, which has no human pair, all before `spawn_child`; `run` passes `Launch`'s `json` false and `revive` passes its `--json`.»
- line: .andromeda/obs-plan-detail/error-capture-reporting.md:27
  old: «or under `--json` `{"v":1,"error":"instance-unreachable","detail":null}`)»
  new: «or under `--json` `{"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"}`)»
- line: .andromeda/obs-plan-detail/error-capture-reporting.md:40
  old: «a refusal returns `instance-unreachable` with `detail: null`,»
  new: «a refusal returns `instance-unreachable` with `detail: "strict-modes-failed"`,»
- after: .andromeda/obs-plan-detail/error-capture-reporting.md:40
  anchor: «- MCP rejects on a refused instance check: `viola mcp`'s frame tools (`send`, `wait`, `last`, `answer`) run `check_instance` on the home and the instance directory in `frame` (`crates/viola-mcp/src/tools.rs`) before `live_endpoint`; a refusal returns `instance-unreachable` with `detail: null`, the `isError` code of the liveness pre-check, and writes no frame and no line.»
  add: «- MCP maps each refusal to its `UnreachableCause`: in `frame` a refused `check_instance` is `StrictModes`, no `live_endpoint` is `without_endpoint`'s reading and a failed `server_verified` is `ServerVerify`; in `of_board` (`crates/viola-mcp/src/result.rs`) the board's refusal is `StrictModes`; in `of_reply` `Connect` is `InstanceDead`, and `Closed` or `Io` is `EndpointLost`.»
- after: .andromeda/obs-plan-detail/error-capture-reporting.md:51
  anchor: «- The exit-2 paths of `viola answer` and `viola send` write the fixed `UNUSABLE_RESPONSE` / `UNUSABLE_TEXT` line to stderr whatever `--json` says, with no document (`src/cmd/answer.rs`, `src/cmd/send.rs`); `NOT_ERROR_BODIES` (`tests/contract_error_bodies.rs`) names them.»
  add: «- Exit 21 prints one document per cause, `{"v":1,"error":"instance-unreachable","detail":"<cause>"}`; `detail` is one of the five closed words of `viola_core::UnreachableCause`: `strict-modes-failed`, `instance-dead`, `unwrapped-name`, `endpoint-lost`, `server-verify-failed`. No serde path reaches them: the two clients write every word. `unreachable_document` (`src/cmd/client.rs`) is the one writer of the document, `refuse_untrusted` and `src/cmd/plugin.rs` through it; `refused_document` in `src/cmd/list.rs` builds `strict-modes-failed` as a `Value` for `board_document`, the board's only refusal.»
- line: .andromeda/obs-plan-detail/pii-scrubbing-compliance.md:40
  old: «or under `--json` exactly `{"v":1,"error":"instance-unreachable","detail":null}`;»
  new: «or under `--json` exactly `{"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"}`;»
- line: .andromeda/obs-plan-detail/pii-scrubbing-compliance.md:40
  old: «return `instance-unreachable` with `detail: null` and write no line»
  new: «return `instance-unreachable` with `detail: "strict-modes-failed"` and write no line»

## Patterns
[[pattern]]
id = "obs3-01"
fixed = 'a `live` or a `stale` holder'

[[pattern]]
id = "obs3-02"
fixed = 'preflight refusals under `--json` are the same-shaped document'

[[pattern]]
id = "obs3-03"
fixed = 'is no longer one of them'

[[pattern]]
id = "obs3-04"
fixed = '`detail` at 13 values'

[[pattern]]
id = "obs3-05"
fixed = 'exit 21: `instance-dead`, `strict-modes-failed`'

[[pattern]]
id = "obs3-06"
fixed = 'on `process-exit` exit 21, and on `parse-rejected'

[[pattern]]
id = "obs3-07"
fixed = 'frame tools (`instance-unreachable`, `detail: null`)'

[[pattern]]
id = "obs3-08"
fixed = 'exactly `error: internal error`);'

[[pattern]]
id = "obs3-09"
fixed = 'or under `--json` `{"v":1,"error":"instance-unreachable","detail":null}`'

[[pattern]]
id = "obs3-10"
fixed = 'returns `instance-unreachable` with `detail: null`'

[[pattern]]
id = "obs3-11"
fixed = 'under `--json` exactly `{"v":1,"error":"instance-unreachable","detail":null}`'

[[pattern]]
id = "obs3-12"
fixed = 'return `instance-unreachable` with `detail: null`'
