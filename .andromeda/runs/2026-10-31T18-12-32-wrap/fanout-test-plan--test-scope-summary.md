proposals:
  - detector: D-retired
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 12
    section: '## 1. Test Scope Summary'
    change: 'In **As landed (server verification)**: the verb exits 21, under `--json` with `detail` `server-verify-failed`; the MCP frame tools read `instance-unreachable` with `detail` `server-verify-failed`, no frame; and the last sentence ends: the macOS cases are owed to the route entry "Unix endpoint and home hardening" (the cause named outside the built exit-cause table, §6 Exit-cause matrix).'
    sidecar: test-scope-summary §1 viola-channel server verification: the cause word added and the macOS cases owned by the route entry, the exit-cause table being built (chunk 2026-10-31-exit-cause-code-catalogue).
    rationale: The report Retired bullet names the not-built lines of the exit-cause matrix and its cross-table row lists test-scope-summary.md:12 as changed; its Outcome leaves the macOS server check with "Unix endpoint and home hardening".
    basis: .andromeda/test-plan-detail/test-scope-summary.md:12
  - detector: D-retired
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 34
    section: '## 1. Test Scope Summary'
    change: 'The `revive` clause reads: an exit-1 refusal of its strict-modes check, its preflight or the start it shares with `run` prints `{"v":1,"error":"<code>","detail":null}` on stdout, `<code>` one of `strict-modes-failed`, `already-live`, `no-session`, `session-live`, `cwd-missing`, `stale-heartbeat`, `batch-script-child`, `pinned-hash-mismatch`, `squatted-name`, `internal-error` (the start refusals by chunk 2026-10-31-exit-cause-code-catalogue; `internal-error` for a program not found and for an anyhow error of the start; a `pump_child` error prints no document); the `--list --json` words stand; the clause on a post-preflight refusal printing no document and its owed-to entry are removed; its exits read 1 for a refusal (the strict-modes check ahead of the log, a preflight reading or a refusal of the start).'
    sidecar: test-scope-summary §1 CLI entity, the revive --json post-preflight gap closed, its five codes now ten, the owed-to clause spent (chunk 2026-10-31-exit-cause-code-catalogue).
    rationale: The report Retired bullet names the revive --json post-preflight gap, and Symbols / APIs lists the ten codes and states the no-document sentence is no longer true (revive-gap, start-codes-rev, title).
    basis: src/cmd/revive.rs:119-134
  - detector: D-tests-count-literals
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 43
    section: '## 1. Test Scope Summary'
    change: 'Under `--json` the exit-1 `{"v":1,"error":"<code>","detail":null}` document (its ten codes) or the `--list` document `{"v":1,"ok":{"sessions":[…]}}`; the literal "its five codes" goes.'
    sidecar: test-scope-summary §1 cli surface, the revive --json document count five to ten (chunk 2026-10-31-exit-cause-code-catalogue).
    rationale: Report Counts row start-codes-rev moved with revive-gap; Symbols / APIs lists ten codes for revive --json.
    basis: src/cmd/revive.rs:119-134
  - detector: D-retired
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 37
    section: '## 1. Test Scope Summary'
    change: 'A refusal exits 21 with the instance pair, or under `--json` exactly `{"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"}`, and writes no line (§5 On-disk).'
    sidecar: test-scope-summary §1 CLI instance check, the exit-21 document names its cause strict-modes-failed in place of detail null (chunk 2026-10-31-exit-cause-code-catalogue).
    rationale: The report Retired bullet names the exit-21 document detail null; the doc-null row sets a strict-modes site to strict-modes-failed.
    basis: src/cmd/client.rs:222-226
  - detector: D-retired
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 69
    section: '## 1. Test Scope Summary'
    change: 'The Signal alternative reads `isError:true` + `{"error":"instance-unreachable","detail":"<cause>"}`, `<cause>` one of `strict-modes-failed` · `instance-dead` · `unwrapped-name` · `endpoint-lost` · `server-verify-failed`; the `wrapper-fault` and `invalid-params` alternatives and each text `refused: <code>` stand.'
    sidecar: test-scope-summary §1 MCP surface signal, the instance-unreachable body carries its cause (chunk 2026-10-31-exit-cause-code-catalogue).
    rationale: The report states MCP isError result is the exit-21 document less `v` and the text is unchanged; the Retired bullet names detail null.
    basis: crates/viola-core/src/lib.rs:181-190
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 72
    section: '## 1. Test Scope Summary'
    change: 'A refused instance returns `isError:true` `{"error":"instance-unreachable","detail":"strict-modes-failed"}` with no frame and no line.'
    sidecar: test-scope-summary §1 MCP instance check, the refused instance body names strict-modes-failed (chunk 2026-10-31-exit-cause-code-catalogue).
    rationale: The report MCP bullet says a refused check_instance in frame is StrictModes; the Retired bullet names detail null.
    basis: crates/viola-core/src/lib.rs:181-190
    dependent-of: D-retired
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 37
    section: '## 1. Test Scope Summary'
    change: 'New item under the CLI entity: `  - **As landed (the causes of exit 21):** every cause prints `{"v":1,"error":"instance-unreachable","detail":"<cause>"}`, `v` still 1 and `error` unmoved, `<cause>` closed to `strict-modes-failed` · `instance-dead` · `unwrapped-name` · `endpoint-lost` · `server-verify-failed` (`viola_core::UnreachableCause`, the one writer `src/cmd/client.rs` `unreachable_document`), no exit moved. With no live endpoint `viola_state::liveness::without_endpoint` reads the instance path: a directory is `instance-dead`, anything else `unwrapped-name`, the strict-modes check first; a wrapper lost with the call open is `endpoint-lost`; `list` and `plugin install` refuse only `strict-modes-failed`. Witnesses: `tests/cross_exit_causes.rs` (`strict_modes_on_a_frame_verb_is_exit_21_and_opens_no_log`, `a_stopped_instance_is_instance_dead`, `a_name_never_started_is_unwrapped_name`, `a_wrapper_killed_under_a_parked_wait_is_endpoint_lost`, its control `no_two_codes_share_a_hint`); server verification stands in `tests/security_negatives_server_identity.rs`.'
    sidecar: test-scope-summary §1 CLI entity gains the five causes of exit 21 and their witnesses (chunk 2026-10-31-exit-cause-code-catalogue).
    rationale: The report Symbols / APIs moves the exit-21 document, and its Coverage of new surfaces names the table as the witness; the CLI contract entity holds this surface.
    basis: crates/viola-state/src/liveness.rs:107-115
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 37
    section: '## 1. Test Scope Summary'
    change: 'New item under the CLI entity: `  - **As landed (the catch site under `--json`):** where `prints_internal_error` holds, a verb with `--json` among the arguments before a `--` prints `{"v":1,"error":"internal-error","detail":null}` on stdout in place of `error: internal error`, stderr then empty; without `--json` the line stands byte for byte, and `run`, the start of `revive`, `mcp` and `hook` stay silent there (`src/main.rs` `json_asked`, `src/cmd/mod.rs` `internal_error_document`; `tests/cli_wait_last.rs` `cli_verbs_print_the_internal_error_document_under_json`, `tests/cross_exit_causes.rs` `a_frame_verb_that_cannot_open_its_log_is_internal_error`).'
    sidecar: test-scope-summary §1 CLI entity gains the catch-site document under --json (chunk 2026-10-31-exit-cause-code-catalogue).
    rationale: The report catch-site row and Symbols / APIs add this document to the --json contract the entity states.
    basis: src/main.rs:154-161
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 37
    section: '## 1. Test Scope Summary'
    change: 'New item under the CLI entity: `  - **As landed (the codes of `verify`):** a refused `verify` under a parsed `VIOLA_NAME` writes its code on `process-exit`: `batch-script-child`, `program-not-found`, `pinned-hash-mismatch`, `version-unreadable`, `strict-modes-failed`, `unclean-fixture`, beside the standing `interrupted`; a refused home tree stays lineless, a failing row is no refusal and no code, the fixture `<file>` enters no code, and `verify` takes no `--json` (`src/cmd/verify.rs` `Ended::Refused`; `tests/cli_verify.rs` `verify_a_tampered_pinned_copy_refuses_with_its_code`, `verify_window_a_widened_stamps_file_refuses_with_its_code`; `tests/cross_exit_causes.rs` `verify_with_no_claude_on_its_path_is_program_not_found`).'
    sidecar: test-scope-summary §1 CLI entity gains the process-exit codes of verify (chunk 2026-10-31-exit-cause-code-catalogue).
    rationale: The report Symbols / APIs bullet on the codes of verify and its Coverage row name an integration witness for this new surface.
    basis: schemas/diag-line.v1.json:78
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 72
    section: '## 1. Test Scope Summary'
    change: 'New item under the MCP surface: `  - **As landed (the causes of an unreachable instance):** the body is the exit-21 document less `v`, the text still `refused: instance-unreachable`; `of_reply` reads `Connect` as `instance-dead` and `Closed | Io` as `endpoint-lost`, `of_board` the board refusal as `strict-modes-failed`, and `frame` a refused `check_instance` as `strict-modes-failed`, no `live_endpoint` as the `without_endpoint` reading and a failed `server_verified` as `server-verify-failed` (off macOS), those three with no frame and no line, `isError` the record; JSON-RPC errors unmoved, `-32603` keeping its fixed body (`handlers_frame_tools_name_the_cause_of_an_unreachable_instance`, `mcp_tools_a_wrapper_killed_under_a_parked_wait_reads_endpoint_lost`).'
    sidecar: test-scope-summary §1 MCP surface gains the cause mapping and its witnesses (chunk 2026-10-31-exit-cause-code-catalogue).
    rationale: The report MCP bullet and Outcome criterion give the mapping and the two named witnesses.
    basis: crates/viola-mcp/tests/handlers.rs:513-552
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 134
    section: '## 1. Test Scope Summary'
    change: 'New Verification signal row: `    - A stale holder refuses the start exit 1 with `stale-heartbeat`, a live holder or a fresh replayed beat with `already-live`; a frame verb against a stale instance still reads `instance-dead`, stale heartbeat being the cause of exit 1 alone (`tests/cli_instance_state.rs` `run_refuses_a_stale_name`, `revive_json_of_a_stale_holder_is_the_stale_heartbeat_document`, `tests/cross_exit_causes.rs` `a_live_holder_is_already_live`).'
    sidecar: test-scope-summary §1 run start path gains the stale holder code (chunk 2026-10-31-exit-cause-code-catalogue).
    rationale: The report bullet A stale holder code, and Retired names `already-live` as a stale holder refusal; the path signal states only that a duplicate run exits 1.
    basis: tests/cli_instance_state.rs:190-222
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 182
    section: '## 1. Test Scope Summary'
    change: 'The row reads: endpoint vanishes during `wait`: `instance-unreachable`, `detail` `endpoint-lost` (`tests/cross_exit_causes.rs` `a_wrapper_killed_under_a_parked_wait_is_endpoint_lost`).'
    sidecar: test-scope-summary §1 chaos trigger, the vanished endpoint names its cause (chunk 2026-10-31-exit-cause-code-catalogue).
    rationale: The report states a wrapper lost with the call open is `endpoint-lost`; the row stays true and gains the cause.
    basis: tests/cross_exit_causes.rs:328-361
note: 'Lines 155 and 183 left unchanged as the report rows detail-null and stale-live dispose; D-tests-coverage, D-tests-framework and D-tests-obs-harness found no drift in this file.'
