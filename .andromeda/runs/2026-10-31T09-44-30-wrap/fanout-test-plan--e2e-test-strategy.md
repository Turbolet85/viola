proposals:
  - detector: D-tests-coverage
    severity: warning
    file: .andromeda/test-plan-detail/e2e-test-strategy.md
    near: 259
    section: '## 6. E2E Test Strategy › #### Scenario: E5 — snapshot corruption → replay recovery'
    change: '- **Error sanitization and secret scan:** the after-every-test helper this suite first named was replaced and not built (measured absent at `tests/support/events.rs` before chunk 2026-10-31-sanitised-error-surfaces; inputs#I2 note 2, inputs#I3 item 4). Three surface witnesses, the enumeration and the census of writers hold the error bodies, the check standing in `tests/support/events.rs`, and the CI secret scan reads the kept homes after the end-to-end run of each job, not after each test (`v1-47` as concretized at that chunk, accepted by the delegate overseer word, PROVISIONAL, owed the word of the founder; the `viola-chaos-*` home out by the ruling of the founder of 2026-10-04). What they hold:'
    sidecar: 'e2e-test-strategy, error sanitization: the after-every-test helper stated as replaced and not built; the witnesses, the enumeration and the census hold the bodies, the secret scan runs per job (chunk 2026-10-31-sanitised-error-surfaces).'
    rationale: 'Report, Spec claims disproved, pattern `after-every-test`, and Expected amendments, which owe §6 a plain statement that the helper was replaced and not built; Decisions, the `v1-47` concretization.'
    basis: .andromeda/test-plan-detail/e2e-test-strategy.md:259
  - detector: D-tests-coverage
    severity: warning
    file: .andromeda/test-plan-detail/e2e-test-strategy.md
    near: 261
    section: '## 6. E2E Test Strategy › #### Scenario: E5 — snapshot corruption → replay recovery'
    change: '  - every -32603 is exactly `"internal error"` / `null`, held at the unit layer (`answer_a_panicking_dispatch_is_internal_error_and_closes`, `links_set_method_an_append_that_fails_is_internal_with_the_set_unchanged`): no -32603 is driven in a scanned home, and no failpoint or counted panic was added'
    sidecar: 'e2e-test-strategy, error sanitization: the -32603 body stated as held at the unit layer, no -32603 being driven in a scanned home.'
    rationale: 'Report, Reverted / negative API facts, states the unit layer holds it; with no after-every-test helper this line otherwise reads as an end-to-end check that does not exist.'
    dependent-of: D-tests-coverage
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/e2e-test-strategy.md
    near: 247
    section: '## 6. E2E Test Strategy › #### Scenario: E5 — snapshot corruption → replay recovery'
    change: '  - The MCP stdin line past `MAX_FRAME` before `initialize`: `mcp_tools_a_line_past_max_frame_before_initialize_exits_1_with_one_line` (`crates/viola-e2e/tests/mcp_tools.rs`, chunk 2026-10-31-sanitised-error-surfaces) reads exit 1, both streams empty and exactly one `parse-rejected{parser:"mcp-stdin", detail:"oversize", count:1}` line. Exit 11 is the code of the unit child alone (`serve_at_a_line_past_max_frame_before_initialize_is_the_initialize_error`, `serve_child_entry`), never the process exit: `src/main.rs` `exit_code` gives `Role::Mcp` + `Failed` exit 1.'
    sidecar: 'e2e-test-strategy, security sweep: the MCP stdin line past MAX_FRAME before initialize added, exit 1 measured (chunk 2026-10-31-sanitised-error-surfaces).'
    rationale: 'Report, Spec claims disproved, «`ServeError::Initialize` (exit 11)», and Outcome, the stdin line before `initialize`; the case is new and this suite names only the after-initialize one.'
    basis: crates/viola-e2e/tests/mcp_tools.rs:522
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/e2e-test-strategy.md
    near: 264
    section: '## 6. E2E Test Strategy › #### Scenario: E5 — snapshot corruption → replay recovery'
    change: '  - **The check** (`tests/support/events.rs`, chunk 2026-10-31-sanitised-error-surfaces): `error_body_hit` and `assert_error_body_clean`, classes `absolute-path` · `caused-by` · `serde-path` · `planted`, reporting place and class, never bytes; its self-checks are `error_body_check_reads_each_class_as_a_hit` and `error_body_check_reports_no_byte_of_what_it_read` (`tests/cli_error_bodies.rs`).'
    sidecar: 'e2e-test-strategy, error sanitization: the check in tests/support/events.rs recorded with its four classes.'
    rationale: 'Report, Symbols / APIs, Tests check; New text, by line, the two self-checks.'
    basis: tests/support/events.rs:18
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/e2e-test-strategy.md
    near: 264
    section: '## 6. E2E Test Strategy › #### Scenario: E5 — snapshot corruption → replay recovery'
    change: '  - **Channel witness:** `channel_error_bodies_hold_no_path_or_upstream_text` (`tests/channel_error_bodies.rs`, `binary(channel_error_bodies)`, `[[test]]` `required-features = ["fake-agent"]`, on the `ci` profile kill line): the seven wire bodies unchanged, `v` 1, a too-new request answered `-32602` with the version of the wrapper, and the `detail` words of `channel-response` read (`schemas/diag-line.v1.json` closes `detail` to `unsupported-version` · `invalid-params` · `release-from-driver`, absent on every other `channel-response` line). It boots through `OuterPty::spawn_without`, not `Wrapper` or `Command::env`, because `Wrapper` takes no environment: the planted value reaches the child through the `env_set` of the PTY spawn.'
    sidecar: 'e2e-test-strategy, error sanitization: the channel witness recorded, a new binary on the ci kill line.'
    rationale: 'Report, Outcome, Coverage of new surfaces, Schema / config, Harness / gate surface, Counts (`channel_error_bodies` added to the kill line), Deviation 1; fact `new-binaries`, no master names it yet.'
    basis: tests/channel_error_bodies.rs:100
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/e2e-test-strategy.md
    near: 264
    section: '## 6. E2E Test Strategy › #### Scenario: E5 — snapshot corruption → replay recovery'
    change: '  - **CLI witness:** `cli_error_documents_hold_no_path_or_upstream_text` (`tests/cli_error_bodies.rs`, `binary(cli_error_bodies)`, `[[test]]` `required-features = ["fake-agent"]`, on the `ci` profile kill line): each `--json` failure it drives is one document, its literal exit, stderr empty, no ESC, the document clean.'
    sidecar: 'e2e-test-strategy, error sanitization: the CLI witness recorded, a new binary on the ci kill line.'
    rationale: 'Report, Outcome (each CLI `--json` failure the witness drives), Harness / gate surface, Counts (`cli_error_bodies` added to the kill line); fact `new-binaries`, no master names it yet.'
    basis: tests/cli_error_bodies.rs:115
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/e2e-test-strategy.md
    near: 264
    section: '## 6. E2E Test Strategy › #### Scenario: E5 — snapshot corruption → replay recovery'
    change: '  - **MCP witnesses:** `mcp_tools_error_results_hold_no_path_or_upstream_text` and `mcp_tools_a_wrong_shaped_call_answers_no_caller_text` (`crates/viola-e2e/tests/mcp_tools.rs`, `binary(mcp_tools)`). The session of the first is unstamped, no row reaching a stamped path. The rmcp client reads the wire `"data":null` as `None`, so the live unknown-tool case reads `None`; the wire null is the product `Some(Value::Null)` (`crates/viola-mcp/src/tools.rs`).'
    sidecar: 'e2e-test-strategy, error sanitization: the MCP witnesses recorded, with the rmcp client reading a wire null as None.'
    rationale: 'Report, New text, by line (`crates/viola-e2e/tests/mcp_tools.rs`), Deviations 4 and 5, Decisions (the oracle written against the wire form).'
    basis: crates/viola-e2e/tests/mcp_tools.rs:674
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/e2e-test-strategy.md
    near: 372
    section: '## 6. E2E Test Strategy › #### Scenario: E5 — snapshot corruption → replay recovery'
    change: '    - Since chunk 2026-10-31-sanitised-error-surfaces the five `BOUNDARIES` rows for the status-line command stdout, the child pipe drain shared by `claude --version` and `claude agents --json`, and the `config.json` readers of the thresholds, the dialog hold and `claude_env_keep` each name a cap-driving case: `statusline_shell_out_past_the_frame_cap_keeps_exactly_the_cap`, `statusline_shell_out_a_command_the_cut_ends_is_a_failed_command`, `drain_keeps_cap_bytes_of_an_endless_pipe`, `read_thresholds_keeps_no_key_past_the_frame_cap`, `read_hold_keeps_no_key_past_the_frame_cap`, `env_keep_keeps_no_key_past_the_frame_cap`.'
    sidecar: 'e2e-test-strategy, contract suite: the five BOUNDARIES rows name cap-driving witnesses, the owed clause discharged.'
    rationale: 'Report, Harness / gate surface, Outcome (the five rows each name a cap-driving case), Expected amendments (test-plan §6 owed clause); pattern `bound-rows` leaves line 371 itself unchanged, so this is added detail.'
    basis: tests/contract_bounded_readers.rs:91
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/e2e-test-strategy.md
    near: 372
    section: '## 6. E2E Test Strategy › #### Scenario: E5 — snapshot corruption → replay recovery'
    change: '  - `tests/contract_error_bodies.rs` (a `contract_` binary in the E2E layer, `run --e2e`, carrying the `andromeda:walks-tree` token; chunk 2026-10-31-sanitised-error-surfaces) holds the error-body enumeration in literal tables: `CLI_DOCUMENTS` and `MCP_RESULTS` name each body with its writer and witness, `CHANNEL_FAULTS` each fault with its literal wire body and witness, `MCP_FIELD_WORDS` and `CHANNEL_METHODS` their words and methods, `NOT_ERROR_BODIES` the lines that are no error body, each with its reason (the fixed `UNUSABLE_RESPONSE` / `UNUSABLE_TEXT` lines among them), and `PLUMBING` the census lines that build or route a document a row names, so a row stays one body. `error_bodies_each_name_a_writer_and_a_witness_that_stand` holds the rows, and `channel_faults_are_the_seven_literal_bodies` the seven wire bodies.'
    sidecar: 'e2e-test-strategy, contract suite: the error-body enumeration in tests/contract_error_bodies.rs recorded.'
    rationale: 'Report, Changes (Files, Harness / gate surface), New text, by line, Deviation 3, Spec claims disproved (`NOT_ERROR_BODIES`); fact `new-binaries`, no master names it yet.'
    basis: tests/contract_error_bodies.rs:652
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/e2e-test-strategy.md
    near: 372
    section: '## 6. E2E Test Strategy › #### Scenario: E5 — snapshot corruption → replay recovery'
    change: '    - The census of writers (`product_error_writers_are_each_named`, `thiserror_messages_interpolate_nothing`, its self-check `census_finds_an_unnamed_writer_and_an_interpolated_message`) walks `PRODUCT_ROOTS` and holds every error writer to a named row and every `thiserror` message to no interpolation. It keeps its own scanner beside that of `tests/contract_bounded_readers.rs`: that one blanks string literals, this one reads them.'
    sidecar: 'e2e-test-strategy, contract suite: the census of error writers recorded, with its own scanner.'
    rationale: 'Report, New text, by line (`tests/contract_error_bodies.rs`), Deviation 2, Spec claims disproved (the census of writers replaces the after-every-test check).'
    basis: tests/contract_error_bodies.rs:720
note: 'Line 263 (diagnostics 0600 on Unix) stands unedited though the report names no holder for it once the after-every-test helper is gone; the report says the three witnesses read the check in tests/support/events.rs while its new-text list shows mcp_tools.rs with its own assert_no_path_or_upstream_text, so no proposal says which witness reads which; Deviation 1 says tests/tui_env_strip.rs plants through the PTY spawn env_set while line 213 says Command::env, an older line this chunk did not move; D-tests-framework, D-tests-obs-harness and D-tests-count-literals fired nothing, and no D-retired hit stands here.'
