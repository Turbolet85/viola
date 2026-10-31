proposals:
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 12
    section: '## 1. Test Scope Summary'
    change: 'Add after this line, as a sub-item of the `viola-channel` entity: `  - **As landed (the fault screen):** `viola_channel::sanitised_fault` reads a peer error for the two clients, `reply_of` and `of_reply`: each of the five known codes takes its fixed `message`, `-32602` keeps only closed `data` values (an integer `supported` with a version-text `wrapper`, or the `reason` `release-from-driver`), every other known code reads `data: null`, a code outside the five reads `wrapper fault` with `data: null`, and an error without an integer `code` reads nothing. The seven `ProtocolError` wire bodies and `v` did not change. `ProtocolError::cause` gives the `detail` of `channel-response` (`unsupported-version` · `invalid-params` · `release-from-driver`). Held over the raw channel by `tests/channel_error_bodies.rs` (§6) and at the unit layer (§4 viola-channel).`'
    sidecar: 'test-scope-summary: the viola-channel entity gains the fault screen as landed, with its witnesses.'
    rationale: 'The report Changes, Symbols / APIs, adds `sanitised_fault` and `ProtocolError::cause`, and Outcome holds the unchanged wire bodies by `channel_error_bodies_hold_no_path_or_upstream_text`; the entity names the error shapes and not this screen.'
    basis: crates/viola-channel/src/lib.rs:113
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 44
    section: '## 1. Test Scope Summary'
    change: 'In the **Signal** line, after the three document shapes, add: `The exception: the exit-2 `UNUSABLE_RESPONSE` path of `viola answer` and `UNUSABLE_TEXT` path of `viola send` write that fixed line to stderr whatever `--json` says, with no document (named in `NOT_ERROR_BODIES`, `tests/contract_error_bodies.rs`).`'
    sidecar: 'test-scope-summary: the cli signal carves out the exit-2 unusable paths of answer and send, which print no document.'
    rationale: 'The report, Spec claims disproved, measured that these exit-2 paths write a fixed stderr line and no `--json` document; the Signal line lists exit 2 with one JSON document on stdout, unqualified.'
    basis: src/cmd/answer.rs:91
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan.md
    near: 94
    section: '## 1. Test Scope Summary'
    change: 'After `its signal is the exit code plus one JSON document on stdout`, add `(the fixed stderr line of the exit-2 unusable paths of `answer` and `send` excepted)`.'
    sidecar: 'test-plan §1: the cli surface broad line carries the same exit-2 exception.'
    rationale: 'The broad line that test-scope-summary#4.1 details states the same document signal the report disproved for these exit-2 paths.'
    basis: src/cmd/send.rs:187
    dependent-of: D-cascade
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 162
    section: '## 1. Test Scope Summary'
    change: 'After `(empty on failure), exit 0.`, add: `The shell-out keeps exactly the frame cap of a longer stdout, and a command the cut ends is a failed command (unit `statusline_shell_out_past_the_frame_cap_keeps_exactly_the_cap`, `statusline_shell_out_a_command_the_cut_ends_is_a_failed_command`).`'
    sidecar: 'test-scope-summary: the budget path statusline equality names its cap and the two cap-driving cases.'
    rationale: 'The report Outcome names these cases as the cap-driving witnesses of the status-line row, in the reached `src/cmd/hook/statusline.rs`; the equality stated here carries no cap.'
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/test-strategy.md
    near: 42
    section: '## 2. Test Strategy'
    change: 'In the description of `events.rs`, after the `boot` clause, add: `and the one error-body check, `error_body_hit` and `assert_error_body_clean` (classes `absolute-path` · `caused-by` · `serde-path` · `planted`, reporting place and class, never bytes)`.'
    sidecar: 'test-strategy: the tests/support/events.rs description gains the error-body check.'
    rationale: 'The report Changes, Symbols / APIs, adds `error_body_hit` and `assert_error_body_clean` to `tests/support/events.rs`; this item lists what that file holds and omits them.'
    basis: tests/support/events.rs:18
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/unit-test-strategy.md
    near: 10
    section: '## 4. Unit Test Strategy'
    change: 'Add after this line: `  - The fault screen `viola_channel::sanitised_fault`: `sanitised_fault_keeps_each_wire_body`, `sanitised_fault_carries_nothing_of_the_peer_but_closed_values`, `sanitised_fault_keeps_a_wrapper_of_sixty_four_version_characters`, `sanitised_fault_without_an_integer_code_is_nothing`.`'
    sidecar: 'unit-test-strategy: viola-channel gains the sanitised_fault cases.'
    rationale: 'New fact F14; the report adds the function and these cases at `crates/viola-channel/src/lib.rs`.'
    basis: crates/viola-channel/src/lib.rs:117
    fact: F14
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/unit-test-strategy.md
    near: 10
    section: '## 4. Unit Test Strategy'
    change: 'Add after this line: `  - A fault code outside the five known reads the fixed message `wrapper fault` with `data: null` on `--json` and MCP; the human line `error: wrapper fault  <code>` is unchanged.`'
    sidecar: 'unit-test-strategy: viola-channel records the fixed reading of a fault code outside the five.'
    rationale: 'New fact F15; the report, Coverage of new surfaces, names this fixed output word and the case holding it.'
    fact: F15
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/unit-test-strategy.md
    near: 10
    section: '## 4. Unit Test Strategy'
    change: 'Add after this line: `  - `ProtocolError::cause` tells the three `-32602` faults apart: `protocol_error_cause_tells_the_three_invalid_params_apart`, `answer_invalid_params_names_its_cause_and_no_param`, `answer_faults_other_than_invalid_params_carry_no_detail`, `reply_from_logs_the_cause_of_each_invalid_params`.`'
    sidecar: 'unit-test-strategy: viola-channel gains the ProtocolError::cause cases.'
    rationale: 'New fact F16; the report adds `cause` and names these cases under Coverage of new surfaces.'
    basis: crates/viola-channel/src/lib.rs:103
    fact: F16
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan.md
    near: 380
    section: '## 4. Unit Test Strategy'
    change: 'After `the JSON-RPC error mapping,` add `the fault screen `sanitised_fault` and `ProtocolError::cause`,`.'
    sidecar: 'test-plan §4: the viola-channel broad line names the fault screen.'
    rationale: 'The report adds both to `viola-channel` with unit cases, and the broad line that unit-test-strategy#1.1 details lists what that crate covers without them.'
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/unit-test-strategy.md
    near: 36
    section: '## 4. Unit Test Strategy'
    change: 'Add after this line: `  - `AgentError` gains `BatchScriptChild` and `ProgramNotFound`, their two messages unchanged, held by `refusal_messages_are_fixed`, `tests/cli_program_resolution.rs` and `tests/cli_verify.rs`.`'
    sidecar: 'unit-test-strategy: viola-agent-claude records the two AgentError variants and what holds them.'
    rationale: 'New fact F20; the report folds `viola_agent_claude::Refusal` into `AgentError` with outputs unchanged, and `resolve_program` returns `Result<PathBuf, AgentError>`.'
    fact: F20
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/unit-test-strategy.md
    near: 52
    section: '## 4. Unit Test Strategy'
    change: 'Add after this line: `  - The end of the child on a line past `MAX_FRAME` before `initialize`: `serve_at_a_line_past_max_frame_before_initialize_is_the_initialize_error` (`crates/viola-mcp/src/lib.rs`) holds `ServeError::Initialize` at the unit layer.`'
    sidecar: 'unit-test-strategy: viola-mcp gains the unit case for a line past the cap before initialize.'
    rationale: 'New fact F18; the report names this case in its Outcome and under Spec claims disproved.'
    basis: crates/viola-mcp/src/lib.rs:168
    fact: F18
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/unit-test-strategy.md
    near: 52
    section: '## 4. Unit Test Strategy'
    change: 'Add after this line: `  - The reply screen `of_reply` (`crates/viola-mcp/src/result.rs`, `of_reply_maps_each_reply_shape`): a `refusal` that is no code word reads `unknown`, a string `detail` that is no code word reads `null`, a non-string `detail` is still no known reply, and a peer fault reads through `viola_channel::sanitised_fault`.`'
    sidecar: 'unit-test-strategy: viola-mcp gains the of_reply screen.'
    rationale: 'The report Changes state this reading of `of_reply` and that `sanitised_fault` replaces its private `fault_detail`; the viola-mcp entity names no such screen.'
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan.md
    near: 389
    section: '## 4. Unit Test Strategy'
    change: 'Add after this line: `  - the reply screen: a `refusal` or a string `detail` that is no code word reads `unknown` or `null`, and a peer fault reads through `viola_channel::sanitised_fault``.'
    sidecar: 'test-plan §4: the viola-mcp broad list names the reply screen.'
    rationale: 'The report adds this screen at `of_reply`; the viola-mcp list that unit-test-strategy#1.4 details does not name it.'
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan.md
    near: 390
    section: '## 4. Unit Test Strategy'
    change: 'Replace the line with: `  - a stdin line past `MAX_FRAME` landing before `initialize` completes ends the `serve` child with `ServeError::Initialize`, as a closed stdin before `initialize` does; 11 is the unit child own code only: `viola mcp` itself exits 1, both streams empty, with one `parse-rejected` `mcp-stdin` / `oversize` line of count 1 (unit `serve_at_a_line_past_max_frame_before_initialize_is_the_initialize_error`, E2E `mcp_tools_a_line_past_max_frame_before_initialize_exits_1_with_one_line`)`. The not-driven statement and the PREREQ owner go.'
    sidecar: 'test-plan §4: viola mcp before initialize exits 1, not 11, and the owed clause is discharged, driven at unit and E2E.'
    rationale: 'The report, Spec claims disproved, measured exit 1 with both streams empty and one line, and 11 as the unit child own code; its Expected amendments discharge this owed clause of the route entry Sanitised error surfaces.'
    basis: crates/viola-e2e/tests/mcp_tools.rs:522
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/unit-test-strategy.md
    near: 59
    section: '## 4. Unit Test Strategy'
    change: 'Add after this line: `  - The reply screen `reply_of` (`src/cmd/client.rs`, `reply_of_reads_the_three_shapes`): a `refusal` that is no code word reads `unknown`, a string `detail` that is no code word reads `None`, a non-string `detail` is still no known reply, and a peer fault reads through `viola_channel::sanitised_fault`.`'
    sidecar: 'unit-test-strategy: root bin gains the reply_of screen ahead of the exit-code mapping.'
    rationale: 'The report Changes state this reading of `reply_of` and that `sanitised_fault` replaces its private `fault_detail`; the exit 14 stated on this line stands.'
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/unit-test-strategy.md
    near: 59
    section: '## 4. Unit Test Strategy'
    change: 'Add after this line: `  - A code word outside the closed refusal list stays visible in the document and in `refused: <word>` by design, exit 14 by `reason_of`, held with the screen by `reply_of_reads_the_three_shapes` and `of_reply_maps_each_reply_shape`.`'
    sidecar: 'unit-test-strategy: root bin records that a code word outside the closed list stays visible at exit 14.'
    rationale: 'New fact F17; the report states this in Changes and again in its Outcome as by design.'
    fact: F17
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/unit-test-strategy.md
    near: 81
    section: '## 4. Unit Test Strategy'
    change: 'Add after this line: `  - Cap-driving cases for the bounded readers: `statusline_shell_out_past_the_frame_cap_keeps_exactly_the_cap`, `statusline_shell_out_a_command_the_cut_ends_is_a_failed_command`, `drain_keeps_cap_bytes_of_an_endless_pipe`, `read_thresholds_keeps_no_key_past_the_frame_cap`, `read_hold_keeps_no_key_past_the_frame_cap`, `env_keep_keeps_no_key_past_the_frame_cap`.`'
    sidecar: 'unit-test-strategy: root bin gains the cap-driving cases of the bounded readers.'
    rationale: 'New fact F19; the report Outcome names these six cases for the five `BOUNDARIES` rows.'
    fact: F19
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan.md
    near: 392
    section: '## 4. Unit Test Strategy'
    change: 'After `the exit-code mapping (an unknown refusal maps to exit 14, never a panic and never 20),` add `the reply screen (a `refusal` or a string `detail` that is no code word reads `unknown` or null, a peer fault through `sanitised_fault`),`.'
    sidecar: 'test-plan §4: the root bin broad line names the reply screen.'
    rationale: 'The report adds this screen at `reply_of`, reading a free-text refusal as `unknown` at the same exit 14; the root bin line that unit-test-strategy#2.1 details does not name it.'
note: F13 (viola-core, `is_code_word`) has no picked entity in this group, so the new-detail pass places it; test-plan.md:196 states only the exit of a line past the cap after earlier lines, and its before-initialize exit 1 has no picked entity here.
