proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/integration-test-strategy.md
    near: 40
    section: '## 5. Integration Test Strategy'
    change: 'Add a paragraph after this one, in the group''s bold-lead shape: **A bare temp dir is no home:** a bare temp dir reads 0755 under the umask and the read rule refuses it, so `tool_inputs_that_pass_reach_the_liveness_check`, vacuous while one refusal body hid which refusal it read, now takes a viola-made `empty_home` and reads `unwrapped-name`.'
    sidecar: 'integration-test-strategy: setup records that a bare temp dir home is refused by the read rule, and the liveness-check case moved to a viola-made home.'
    rationale: 'Report Deviations from intent item 3 and the second Sweep hazard: the case never reached the check it names until `detail` told the causes apart.'
    fact: F16
  - detector: D-retired
    severity: warning
    file: .andromeda/test-plan-detail/integration-test-strategy.md
    near: 56
    section: '## 5. Integration Test Strategy'
    change: 'In the MCP impostor case, replace `{"error":"instance-unreachable","detail":null}` with `{"error":"instance-unreachable","detail":"server-verify-failed"}`; text `refused: instance-unreachable`, no frame and no log line stay.'
    sidecar: 'integration-test-strategy: the MCP impostor case reads `server-verify-failed` in `detail`, no longer null.'
    rationale: 'Report Retired names the exit-21 document''s `detail: null`; Symbols / APIs, MCP: a failed `server_verified` is `ServerVerify`, the `isError` body the CLI document less `v`, the text unmoved.'
    basis: crates/viola-core/src/lib.rs:181-190
  - detector: D-retired
    severity: warning
    file: .andromeda/test-plan-detail/integration-test-strategy.md
    near: 57
    section: '## 5. Integration Test Strategy'
    change: 'In the MCP widened-instance case, replace `{"error":"instance-unreachable","detail":null}` with `{"error":"instance-unreachable","detail":"strict-modes-failed"}`; the rest of the item stays.'
    sidecar: 'integration-test-strategy: the MCP widened-instance case reads `strict-modes-failed` in `detail`, no longer null.'
    rationale: 'Report Retired names the exit-21 document''s `detail: null`; Symbols / APIs, MCP: a refused `check_instance` is `StrictModes`, no frame and no line on it.'
    basis: crates/viola-core/src/lib.rs:181-190
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/integration-test-strategy.md
    near: 58
    section: '## 5. Integration Test Strategy'
    change: 'Add a list item after the MCP screen case: the MCP cause case `handlers_frame_tools_name_the_cause_of_an_unreachable_instance` (`crates/viola-mcp/tests/handlers.rs`) holds MCP''s mapping: a refused `check_instance` is `strict-modes-failed`, no `live_endpoint` is `without_endpoint`''s reading, a failed `server_verified` is `server-verify-failed` (off macOS), `Connect` is `instance-dead`, `Closed` or `Io` is `endpoint-lost`, with no frame and no line.'
    sidecar: 'integration-test-strategy: adds the MCP cause case and the mapping it holds.'
    rationale: 'Report Symbols / APIs, MCP, and Outcome: MCP''s body less `v`, each cause its word, `server-verify-failed` off macOS.'
    basis: crates/viola-mcp/tests/handlers.rs:513-552
    fact: F14
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/integration-test-strategy.md
    near: 58
    section: '## 5. Integration Test Strategy'
    change: 'Add a list item after the MCP cause case: the unserved row in `crates/viola-mcp/tests/handlers.rs` records an endpoint no one bound through `record_live`, because a Windows `Serving` has no `Drop` and its pipe lives until the process exits.'
    sidecar: 'integration-test-strategy: records why the handlers unserved row binds nothing.'
    rationale: 'Report Deviations from intent item 4.'
    basis: crates/viola-mcp/tests/handlers.rs:64-86
    fact: F15
  - detector: D-retired
    severity: warning
    file: .andromeda/test-plan-detail/integration-test-strategy.md
    near: 68
    section: '## 5. Integration Test Strategy'
    change: 'In the strict-modes item, replace `MCP frame tools `instance-unreachable` with `detail: null` and no frame or line` with `MCP frame tools `instance-unreachable` with `detail` `strict-modes-failed` and no frame or line`.'
    sidecar: 'integration-test-strategy: the on-disk strict-modes item gives the MCP frame tools `detail` `strict-modes-failed`.'
    rationale: 'Report Retired names the exit-21 document''s `detail: null`; Symbols / APIs, MCP: a refused `check_instance` is `StrictModes`; Counts, `doc-null`: a strict-modes site takes `strict-modes-failed`.'
    basis: crates/viola-core/src/lib.rs:181-190
  - detector: D-retired
    severity: warning
    file: .andromeda/test-plan-detail/integration-test-strategy.md
    near: 69
    section: '## 5. Integration Test Strategy'
    change: 'In the link verbs'' refusal, replace `{"v":1,"error":"instance-unreachable","detail":null}` with `{"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"}`; the instance pair, the empty stderr and the rest of the item stay.'
    sidecar: 'integration-test-strategy: the link verbs'' widened-instance document carries `strict-modes-failed`.'
    rationale: 'Report Retired names the exit-21 document''s `detail: null`; Symbols / APIs: `refuse_untrusted` writes `StrictModes` through `unreachable_document`; Counts, `doc-null`: a strict-modes site takes `strict-modes-failed`.'
    basis: src/cmd/client.rs:222-226
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/integration-test-strategy.md
    near: 85
    section: '## 5. Integration Test Strategy'
    change: 'Add a list item after the `tests/cli_verify.rs` items: `viola verify` writes its refusal''s code on the `process-exit` it writes under a parsed `VIOLA_NAME`: `batch-script-child`, `program-not-found`, `pinned-hash-mismatch`, `version-unreadable`, `strict-modes-failed` and `unclean-fixture`, beside `interrupted`.'
    sidecar: 'integration-test-strategy: adds the refusal codes `viola verify` logs on its `process-exit`.'
    rationale: 'Report Symbols / APIs, `verify`''s codes, and Schema / config: the `process-exit` `detail` enum admits them.'
    basis: schemas/diag-line.v1.json:78
    fact: F11
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/integration-test-strategy.md
    near: 85
    section: '## 5. Integration Test Strategy'
    change: 'Add a list item after the verify refusal codes: a refused home tree under `verify` stays lineless, a failing row is no refusal and no code, the fixture''s file name enters no code, and `verify` takes no `--json`.'
    sidecar: 'integration-test-strategy: adds what stays out of the `verify` refusal codes.'
    rationale: 'Report Symbols / APIs, `verify`''s codes.'
    fact: F12
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/integration-test-strategy.md
    near: 85
    section: '## 5. Integration Test Strategy'
    change: 'Add a list item after the verify refusal rules: `verify_a_tampered_pinned_copy_refuses_with_its_code` and `verify_window_a_widened_stamps_file_refuses_with_its_code` are new in `tests/cli_verify.rs`, the second under the `verify_window_` prefix because it runs two whole verifies, and `verify_with_no_claude_on_its_path_is_program_not_found` is a row of the table `tests/cross_exit_causes.rs`.'
    sidecar: 'integration-test-strategy: names the new verify refusal witnesses.'
    rationale: 'Report Deviations from intent item 7 and its New text section, `tests/cross_exit_causes.rs`.'
    basis: tests/cross_exit_causes.rs:531-546
    fact: F13
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/integration-test-strategy.md
    near: 99
    section: '## 5. Integration Test Strategy'
    change: 'Add a list item after the machine-contract item: `Frozen` in `tests/cli_instance_state.rs` is a `SIGSTOP` guard on Unix as well as Windows; `run_refuses_a_stale_name` and `revive_json_of_a_stale_holder_is_the_stale_heartbeat_document` stand on it.'
    sidecar: 'integration-test-strategy: records the `Frozen` guard on Unix and the stale-holder cases on it.'
    rationale: 'Report Deviations from intent item 2 and its New text section, `tests/cli_instance_state.rs`.'
    basis: tests/cli_instance_state.rs:149-181
    fact: F17
note: 'The §5 body lines for On-disk and CLI hold as written, because `error` stays `instance-unreachable` and each still covers the `--json` document per exit code, so no body edit is proposed from this group.'
