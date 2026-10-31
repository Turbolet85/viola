proposals:
  - detector: D-retired
    severity: warning
    file: .andromeda/security-plan-detail/error-handling.md
    near: 12
    section: '## Error Handling'
    change: '- A `viola mcp` frame tool (`send` · `wait` · `last` · `answer`) whose server check fails (the connected peer is not the recorded wrapper, or the check cannot be made) sends no frame and returns the result of an unreachable instance: `isError` `instance-unreachable` with `detail: "server-verify-failed"`, text `refused: instance-unreachable`; `error` stays `instance-unreachable`, the cause its `detail` word, and it writes no log line.'
    sidecar: 'error-handling: an MCP frame tool failing its server check now returns `detail` `server-verify-failed`, in place of `detail: null` and "adds no error word" (chunk 2026-10-31-exit-cause-code-catalogue).'
    rationale: 'The report **Retired** names the exit-21 document `detail: null`; **Symbols / APIs** MCP has a failed `server_verified` read `ServerVerify`, and **Counts** `no-own-code` lists this line as changed.'
    basis: .andromeda/security-plan-detail/error-handling.md:12
  - detector: D-retired
    severity: warning
    file: .andromeda/security-plan-detail/error-handling.md
    near: 13
    section: '## Error Handling'
    change: '- A `viola mcp` frame tool runs the instance''s strict-modes check (`check_instance`) before its liveness pre-check; a refused check sends no frame and returns the unreachable result with `detail: "strict-modes-failed"`, and a snapshot that is replayed or absent (no live endpoint) sends no frame and returns it with the reading of `without_endpoint`, `detail: "instance-dead"` or `detail: "unwrapped-name"`: `isError` `instance-unreachable`, text `refused: instance-unreachable`, and no log line.'
    sidecar: 'error-handling: an MCP frame tool refused by strict modes or finding no live endpoint now returns its cause in `detail`, in place of `detail: null` (chunk 2026-10-31-exit-cause-code-catalogue).'
    rationale: 'The report **Retired** names `detail: null`; **Symbols / APIs** MCP has a refused `check_instance` read `StrictModes` and no `live_endpoint` read the `without_endpoint` reading; **Expected amendments** lists this line.'
    basis: crates/viola-state/src/liveness.rs:107-115
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    file: .andromeda/security-plan-detail/error-handling.md
    near: 22
    section: '## Error Handling'
    change: '- CLI `send` · `wait` · `last` · `answer` · `pause` · `release`, as `link` / `unlink`, run the instance''s strict-modes check (`client::refuse_untrusted`: `check_instance` on the home and `instances/<name>`) before any frame: a refusal exits 21, prints the instance pair (`human::instance_state_pair`) through `human::refuse`, or under `--json` exactly `{"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"}`, opens nothing under the home and writes no line.'
    sidecar: 'error-handling: the CLI strict-modes refusal under `--json` now prints `detail` `strict-modes-failed`, in place of `detail: null` (chunk 2026-10-31-exit-cause-code-catalogue).'
    rationale: 'The report **Retired** names `detail: null`; **Symbols / APIs** has `refuse_untrusted` write `StrictModes` through `unreachable_document`; **Expected amendments** lists this line.'
    basis: src/cmd/client.rs:222-226
    dependent-of: D-retired
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/error-handling.md
    near: 13
    section: '## Error Handling'
    change: '- `viola_core::UnreachableCause` (`StrictModes` · `InstanceDead` · `UnwrappedName` · `EndpointLost` · `ServerVerify`, its words `strict-modes-failed` · `instance-dead` · `unwrapped-name` · `endpoint-lost` · `server-verify-failed`) is the closed set of the exit-21 `detail`, with no serde path: every word is written by the two clients, and none holds a path, pid or upstream text. `error` stays `instance-unreachable`, `v` stays 1, MCP reads the same body less `v`, and no exit or `RefusalReason` moved (the founder''s own answer, chunk 2026-10-31-exit-cause-code-catalogue `inputs#I2` Q1). Each word passes `is_code_word` (`every_code_is_a_code_word`) and is a row of `tests/contract_error_bodies.rs`.'
    sidecar: 'error-handling: added the closed exit-21 cause set `UnreachableCause` and its five words (chunk 2026-10-31-exit-cause-code-catalogue).'
    rationale: 'The report **Symbols / APIs**, the exit-21 document and `viola_core::UnreachableCause`; **Coverage of new surfaces** records the closed enum and no path, pid or upstream text.'
    basis: crates/viola-core/src/lib.rs:181-190
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/error-handling.md
    near: 13
    section: '## Error Handling'
    change: '- `crates/viola-mcp/src/result.rs` `unreachable(cause)` writes the MCP result: in `of_reply` `Connect` is `instance-dead` and `Closed | Io` is `endpoint-lost`, and in `of_board` the board refusal is `strict-modes-failed`. No frame and no line on any of them, `isError` the record (the founder''s own answer, chunk 2026-10-31-exit-cause-code-catalogue `inputs#I2` Q4). JSON-RPC errors are unmoved and `-32603` keeps its fixed body.'
    sidecar: 'error-handling: added the MCP `unreachable(cause)` mapping of replies and the board refusal to their causes (chunk 2026-10-31-exit-cause-code-catalogue).'
    rationale: 'The report **Symbols / APIs**, MCP, names the mapping and the unmoved JSON-RPC errors; **Coverage of new surfaces** names the no-line ruling.'
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/error-handling.md
    near: 13
    section: '## Error Handling'
    change: '- `viola_state::liveness::without_endpoint(instance_dir)` reads a missing endpoint by `symlink_metadata`: a directory is `InstanceDead`, anything else (absent, a regular file, a symlink) is `UnwrappedName`; it opens nothing and writes no line, and the strict-modes check stays first at every caller (`Unreachable::without_endpoint` in `src/cmd/client.rs`, `frame` in `crates/viola-mcp/src/tools.rs`). The reading by the instance directory is the delegate''s word, PROVISIONAL (the overseer, chunk 2026-10-31-exit-cause-code-catalogue `inputs#I2` Q2).'
    sidecar: 'error-handling: added `without_endpoint`, the directory reading of a missing endpoint, PROVISIONAL on the delegate''s word (chunk 2026-10-31-exit-cause-code-catalogue).'
    rationale: 'The report **Symbols / APIs**, `viola_state::liveness::without_endpoint`; **Expected amendments** carries the unwrapped reading as `inputs#I2` Q2, `delegate the overseer`, PROVISIONAL.'
    basis: crates/viola-state/src/liveness.rs:107-115
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/error-handling.md
    near: 22
    section: '## Error Handling'
    change: '- `client::unreachable_document(cause)` (`src/cmd/client.rs`) is the one writer of the exit-21 document; `refuse_untrusted` and `src/cmd/plugin.rs` write `strict-modes-failed` through it, and `refused_document` stays that of exit 1. The `refused_document` of `src/cmd/list.rs`, a `Value` for `board_document`, builds `strict-modes-failed`, the only refusal of the board. `Unreachable` is `Dead` · `Unwrapped` · `Lost` · `ServerUnverified`; in `answer_of` `Connect` is `Dead` and `Closed | Io` is `Lost`.'
    sidecar: 'error-handling: added `unreachable_document`, the one CLI writer of the exit-21 document, and the `Unreachable` variants behind each cause (chunk 2026-10-31-exit-cause-code-catalogue).'
    rationale: 'The report **Symbols / APIs**, `src/cmd/client.rs`.'
    basis: src/cmd/client.rs:222-226
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/error-handling.md
    near: 24
    section: '## Error Handling'
    change: '- Under `--json` the start that `run` and `revive` share prints one exit-1 document for a refusal: `refused_with(json, detail, human)` (`src/cmd/run.rs`) prints `refused_document` in place of the human pair for `batch-script-child`, `pinned-hash-mismatch` and `squatted-name`, and for a program not found `internal-error` (no pair today, none added), all before `spawn_child`. `Launch.json` is false from `run` and `args.json` from `revive`; the human pairs are byte for byte as they stood.'
    sidecar: 'error-handling: added `refused_with`, the start refusal documents under `--json` (chunk 2026-10-31-exit-cause-code-catalogue).'
    rationale: 'The report **Symbols / APIs**, the start of `run` under `--json`.'
    basis: src/cmd/run.rs:422-431
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/error-handling.md
    near: 24
    section: '## Error Handling'
    change: '- `viola revive --json` (`src/cmd/revive.rs`, `started`) prints one exit-1 document for every refusal of its start, its codes `strict-modes-failed` · `already-live` · `no-session` · `session-live` · `cwd-missing` · `stale-heartbeat` · `batch-script-child` · `pinned-hash-mismatch` · `squatted-name` · `internal-error`. An anyhow error from `trusted`, `checked_exe`, `open_wrapper_log`, `preflight` or `start` prints `refused_document("internal-error")` and returns the same error, so the catch site still logs its chain and its line. Exception: a `pump_child` error prints no document, the child having held the terminal.'
    sidecar: 'error-handling: added the `revive --json` start codes and its `internal-error` document, closing the post-preflight gap (chunk 2026-10-31-exit-cause-code-catalogue).'
    rationale: 'The report **Symbols / APIs**, `viola revive --json`; **Retired** names the post-preflight gap.'
    basis: src/cmd/revive.rs:119-134
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/error-handling.md
    near: 24
    section: '## Error Handling'
    change: '- At the `cli` catch site, where `prints_internal_error` is true, `json_asked` (`src/main.rs`: `--json` among the arguments before a `--`, read from argv as `role_of` is) prints `internal_error_document()` (`src/cmd/mod.rs`), `{"v":1,"error":"internal-error","detail":null}`, on stdout in place of `error: internal error`, stderr then empty. Without `--json` the line is byte for byte as it stood; `run`, the start of `revive`, `mcp` and `hook` stay silent there.'
    sidecar: 'error-handling: added the `internal-error` document of the `cli` catch site under `--json` (chunk 2026-10-31-exit-cause-code-catalogue).'
    rationale: 'The report **Symbols / APIs**, the `cli` catch site under `--json`.'
    basis: src/main.rs:154-161
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/error-handling.md
    near: 24
    section: '## Error Handling'
    change: '- `Ended::Refused` (`src/cmd/verify.rs`) carries a closed code onto the `process-exit` that `viola verify` writes under a parsed `VIOLA_NAME`: `batch-script-child`, `program-not-found`, `pinned-hash-mismatch`, `version-unreadable`, `strict-modes-failed`, `unclean-fixture`, beside the standing `interrupted`. A refused home tree stays lineless, a failing row is no refusal and no code, the fixture `<file>` enters no code, and `verify` takes no `--json`.'
    sidecar: 'error-handling: added the closed `Ended::Refused` codes of `viola verify` (chunk 2026-10-31-exit-cause-code-catalogue).'
    rationale: 'The report **Symbols / APIs**, the codes of `verify`; **Coverage of new surfaces** records that the fixture `<file>` enters no code.'
note: 'D-security-input, D-security-auth and D-security-deps found no violation; .andromeda/security-plan.md:413 still states `instance-unreachable` with `detail: null`, outside this file, and error-handling.md:24 still gives `StampsError::StrictModes` "no line" while the report adds a `strict-modes-failed` verify code without naming its writer, so that clause is left unproposed for the orchestrator to settle.'
