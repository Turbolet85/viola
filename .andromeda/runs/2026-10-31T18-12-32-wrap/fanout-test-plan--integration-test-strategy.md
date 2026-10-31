proposals:
  - detector: D-retired
    severity: warning
    file: .andromeda/test-plan-detail/integration-test-strategy.md
    near: 56
    section: '## 5. Integration Test Strategy'
    change: 'In the MCP impostor case replace `{"error":"instance-unreachable","detail":null}` with `{"error":"instance-unreachable","detail":"server-verify-failed"}`; the text `refused: instance-unreachable`, no frame and no log line stand as written.'
    sidecar: 'integration: the MCP impostor case reads `detail` `server-verify-failed`, the retired `detail: null` gone (chunk 2026-10-31-exit-cause-code-catalogue).'
    rationale: 'Report Retired names the exit-21 document `detail: null`; Symbols / APIs, MCP: a failed `server_verified` is `ServerVerify`, the `isError` result the CLI document less `v`, the text unchanged.'
  - detector: D-retired
    severity: warning
    file: .andromeda/test-plan-detail/integration-test-strategy.md
    near: 57
    section: '## 5. Integration Test Strategy'
    change: 'In the MCP widened-instance case replace `{"error":"instance-unreachable","detail":null}` with `{"error":"instance-unreachable","detail":"strict-modes-failed"}`; the rest stands as written.'
    sidecar: 'integration: the MCP widened-instance case reads `detail` `strict-modes-failed` (chunk 2026-10-31-exit-cause-code-catalogue).'
    rationale: 'Report Retired names the exit-21 document `detail: null`; Symbols / APIs, MCP: a refused `check_instance` is `StrictModes`, no frame and no line.'
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    file: .andromeda/test-plan-detail/integration-test-strategy.md
    near: 68
    section: '## 5. Integration Test Strategy'
    change: 'In the strict-modes refusals item replace "MCP frame tools `instance-unreachable` with `detail: null` and no frame or line" with "MCP frame tools `instance-unreachable` with `detail` `strict-modes-failed` and no frame or line"; the rest stands as written.'
    sidecar: 'integration: the on-disk strict-modes item gives the MCP frame tools `detail` `strict-modes-failed` (chunk 2026-10-31-exit-cause-code-catalogue).'
    rationale: 'Report Retired names the exit-21 document `detail: null`; Symbols / APIs, MCP: a refused `check_instance` is `StrictModes`.'
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    file: .andromeda/test-plan-detail/integration-test-strategy.md
    near: 69
    section: '## 5. Integration Test Strategy'
    change: 'In the `viola link` / `viola unlink` item replace `{"v":1,"error":"instance-unreachable","detail":null}` with `{"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"}`; the rest stands as written.'
    sidecar: 'integration: the link verbs strict-modes document reads `detail` `strict-modes-failed` (chunk 2026-10-31-exit-cause-code-catalogue).'
    rationale: 'Report Retired names the exit-21 document `detail: null`; Symbols / APIs: `refuse_untrusted`, the check the link verbs call, writes `StrictModes` through `unreachable_document`.'
    dependent-of: D-retired
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/integration-test-strategy.md
    near: 58
    section: '## 5. Integration Test Strategy'
    change: 'New item under Wrapper channel: - the MCP cause case `handlers_frame_tools_name_the_cause_of_an_unreachable_instance` (`crates/viola-mcp/tests/handlers.rs`) reads each cause its own word in the `isError` body `{"error":"instance-unreachable","detail":"<cause>"}`, the CLI document less `v`, the text still `refused: instance-unreachable`, `server-verify-failed` off macOS; no frame and no line on any of them, `isError` the record; its unserved row records, through `record_live`, an endpoint no one ever bound, because a Windows `Serving` has no `Drop`.'
    sidecar: 'integration: added the MCP cause case of the handlers suite (chunk 2026-10-31-exit-cause-code-catalogue).'
    rationale: 'Report Symbols / APIs, MCP; Outcome, the MCP criterion names the case; deviation 4 the unserved row. The MCP handler list of this file holds no witness of a cause.'
    basis: crates/viola-mcp/tests/handlers.rs:513
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/integration-test-strategy.md
    near: 79
    section: '## 5. Integration Test Strategy'
    change: 'New item under CLI: - exit 21 prints one document per cause since chunk 2026-10-31-exit-cause-code-catalogue, `{"v":1,"error":"instance-unreachable","detail":"<cause>"}`, the five closed causes `strict-modes-failed` · `instance-dead` · `unwrapped-name` · `endpoint-lost` · `server-verify-failed`; `error` stays `instance-unreachable` and no exit moved. A strict-modes site in `tests/cli_*.rs` and `tests/security_negatives_{server_identity,strict_modes}.rs` reads `strict-modes-failed`, a not-running site its own cause; the table is `tests/cross_exit_causes.rs` (§6).'
    sidecar: 'integration: the CLI list states the exit-21 document per cause (chunk 2026-10-31-exit-cause-code-catalogue).'
    rationale: 'Report Symbols / APIs, the exit-21 document moved; Counts, `doc-null`: each changed site its cause. This file says "the `--json` document per exit code", which no longer tells the exit-21 causes apart.'
    basis: crates/viola-core/src/lib.rs:181
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/integration-test-strategy.md
    near: 85
    section: '## 5. Integration Test Strategy'
    change: 'New item under CLI: - `tests/cli_verify.rs` pins the code a refused `viola verify` writes on its `process-exit` under a parsed `VIOLA_NAME` (`batch-script-child`, `program-not-found`, `pinned-hash-mismatch`, `version-unreadable`, `strict-modes-failed`, `unclean-fixture`, beside the standing `interrupted`): `verify_a_tampered_pinned_copy_refuses_with_its_code` and `verify_window_a_widened_stamps_file_refuses_with_its_code`, the second under the `verify_window_` prefix because it runs two whole verifies, both read through `assert_refused_line`. A refused home tree stays lineless, a failing row stays no refusal and no code, the fixture `<file>` enters no code, and `verify` takes no `--json`.'
    sidecar: 'integration: added the verify refusal-code cases of `tests/cli_verify.rs` (chunk 2026-10-31-exit-cause-code-catalogue).'
    rationale: 'Report Symbols / APIs, `verify` codes; Coverage of new surfaces, `verify` process-exit codes tested in `tests/cli_verify.rs`; deviation 7 names the two cases.'
    basis: tests/cli_verify.rs:348
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/integration-test-strategy.md
    near: 98
    section: '## 5. Integration Test Strategy'
    change: 'New item under CLI: - `tests/cli_wait_last.rs` `cli_verbs_print_the_internal_error_document_under_json` pins the `cli` catch site under `--json`: where `prints_internal_error` is true, a `--json` among the arguments before a `--` prints `{"v":1,"error":"internal-error","detail":null}` on stdout in place of `error: internal error`, stderr then empty; without `--json` the line is byte for byte as it stood, and `run`, the start of `revive`, `mcp` and `hook` stay silent there.'
    sidecar: 'integration: added the catch-site `--json` document witness (chunk 2026-10-31-exit-cause-code-catalogue).'
    rationale: 'Report Symbols / APIs, the `cli` catch site under `--json`; Coverage of new surfaces names `cli_wait_last`; Counts, `catch-site`.'
    basis: tests/cli_wait_last.rs:463
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/integration-test-strategy.md
    near: 99
    section: '## 5. Integration Test Strategy'
    change: 'New item under CLI: - a stale holder (`tests/cli_instance_state.rs`): `revive_json_of_a_stale_holder_is_the_stale_heartbeat_document` reads the `stale-heartbeat` document, and `run_refuses_a_stale_name` stands on `Frozen`, since chunk 2026-10-31-exit-cause-code-catalogue a `SIGSTOP` guard on Unix too. `stale-heartbeat` is the code printed and logged for a `stale` holder and exit 1 alone, `already-live` a live holder and a fresh replayed beat, and a frame verb against a stale instance still reads `instance-dead`.'
    sidecar: 'integration: added the stale-holder witnesses of `tests/cli_instance_state.rs` (chunk 2026-10-31-exit-cause-code-catalogue).'
    rationale: 'Report Symbols / APIs, a stale holder code; Retired, `already-live` as a stale holder refusal; deviation 2 names `Frozen` on Unix and `run_refuses_a_stale_name`.'
    basis: tests/cli_instance_state.rs:190
note: 'D-tests-coverage, D-tests-framework, D-tests-obs-harness and D-tests-count-literals fired no drift in this file; test-plan.md:444 stays true as worded, and `tool_inputs_that_pass_reach_the_liveness_check` (deviation 3) is not proposed here because the report names no file for it.'
