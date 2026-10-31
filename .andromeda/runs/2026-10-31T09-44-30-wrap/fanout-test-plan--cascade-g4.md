proposals:
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/e2e-test-strategy.md
    near: 259
    section: '## 6. E2E Test Strategy › #### Scenario: E5 — snapshot corruption → replay recovery'
    change: '- **Error sanitization and secret scan:** no `events` helper runs after every E2E test: that check was measured absent and is not built. It is replaced by the three surface witnesses (`tests/channel_error_bodies.rs`, `tests/cli_error_bodies.rs`, the MCP cases of `crates/viola-e2e/tests/mcp_tools.rs`), the enumeration and the census of writers (`tests/contract_error_bodies.rs`), reading the check in `tests/support/events.rs`. The rules:'
    sidecar: 'e2e-test-strategy: the after-every-test error-body helper stated as measured absent and replaced, not built, by the surface witnesses, the enumeration and the census of writers'
    rationale: 'Report, Spec claims disproved by measurement: the check is absent at `tests/support/events.rs` before this chunk and is not built; pattern `after-every-test` names `e2e-test-strategy.md:259` changed. Expected amendments: state plainly that it was replaced and not built.'
    basis: .andromeda/test-plan-detail/e2e-test-strategy.md:259
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan.md
    near: 605
    section: '## 6. E2E Test Strategy › #### Scenario: E5 — snapshot corruption → replay recovery'
    change: '- **Error sanitization and secret scan:** no helper runs after every E2E test: that check was measured absent and is not built, replaced by the three surface witnesses (channel, CLI `--json`, MCP), the enumeration and the census of writers (`tests/contract_error_bodies.rs`), reading the check in `tests/support/events.rs`. The rules: no absolute path, serde path or `Caused by` in any error payload, every `-32603` exactly `"internal error"` / `null` (held at the unit layer, no scanned home driving one), no `Cookie` header or stripped-variable value in any log, `diagnostics/` files 0600 on Unix, and the fixed canary in no home-level diagnostics line; a hit is reported by place and class, never the matched bytes.'
    sidecar: 'test-plan §6: the broad line of the error sanitization suite no longer claims an after-every-test check; it names the replacement and the unit-held `-32603`'
    rationale: 'Report, Spec claims disproved by measurement: `test-plan.md:605` is stated by name, pattern `after-every-test` changed. Reverted / negative API facts: no `-32603` is driven in a scanned home, the unit layer holds it.'
    basis: .andromeda/test-plan.md:605
    dependent-of: D-cascade
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/e2e-test-strategy.md
    near: 260
    section: '## 6. E2E Test Strategy › #### Scenario: E5 — snapshot corruption → replay recovery'
    change: '  - Channel witness (chunk 2026-10-31-sanitised-error-surfaces): `tests/channel_error_bodies.rs` `channel_error_bodies_hold_no_path_or_upstream_text`: seven raw-channel fault rows against a wrapper booted through `OuterPty::spawn_without` with a planted value, the bodies clean and the three `detail` words on the lines read.'
    sidecar: 'e2e-test-strategy: the channel error-body witness added to the sanitization suite'
    rationale: 'Report, Changes: `tests/channel_error_bodies.rs` is new, with Coverage of new surfaces naming the case for `channel-response.detail`; deviation 1 names the boot.'
    fact: F3
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/e2e-test-strategy.md
    near: 260
    section: '## 6. E2E Test Strategy › #### Scenario: E5 — snapshot corruption → replay recovery'
    change: '  - CLI witness (chunk 2026-10-31-sanitised-error-surfaces): `tests/cli_error_bodies.rs` `cli_error_documents_hold_no_path_or_upstream_text`: each driven CLI `--json` failure gives one document, a literal exit, empty stderr, no ESC and a clean body, across dead, refused and live rows.'
    sidecar: 'e2e-test-strategy: the CLI error-body witness added to the sanitization suite'
    rationale: 'Report, Outcome: each CLI `--json` failure the witness drives is one document, literal exit, stderr empty, no ESC, clean; the New text lists the `dead`, `refused` and `live` tables.'
    fact: F4
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/e2e-test-strategy.md
    near: 260
    section: '## 6. E2E Test Strategy › #### Scenario: E5 — snapshot corruption → replay recovery'
    change: '  - The check (chunk 2026-10-31-sanitised-error-surfaces): `tests/support/events.rs` `error_body_hit` and `assert_error_body_clean`: classes `absolute-path`, `caused-by`, `serde-path`, `planted`, reporting place and class, never bytes; self-tested by `error_body_check_reads_each_class_as_a_hit` and `error_body_check_reports_no_byte_of_what_it_read` in `tests/cli_error_bodies.rs`.'
    sidecar: 'e2e-test-strategy: the error-body check, its four classes and its self-tests'
    rationale: 'Report, Symbols / APIs names the two functions and the classes; the New text lists the two self-tests in `tests/cli_error_bodies.rs`.'
    basis: tests/support/events.rs:18
    fact: F5
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/e2e-test-strategy.md
    near: 260
    section: '## 6. E2E Test Strategy › #### Scenario: E5 — snapshot corruption → replay recovery'
    change: '  - rmcp''s client reads the wire''s null `data` as `None`, so the live unknown-tool case asserts `None`; on the wire it is the product''s `Some(Value::Null)` (chunk 2026-10-31-sanitised-error-surfaces).'
    sidecar: 'e2e-test-strategy: why the live unknown-tool case asserts `None` where the wire carries null'
    rationale: 'Report, deviation 5 and Decisions: an oracle written against the wire form read red through the client.'
    fact: F7
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/e2e-test-strategy.md
    near: 260
    section: '## 6. E2E Test Strategy › #### Scenario: E5 — snapshot corruption → replay recovery'
    change: '  - `channel_error_bodies` and `cli_error_bodies` are `[[test]]` tables with `required-features = ["fake-agent"]` and terms of the `ci` profile''s kill-line filter.'
    sidecar: 'e2e-test-strategy: the two new witness binaries as `fake-agent` tables on the kill line'
    rationale: 'Report, Harness / gate surface names the `[[test]]` tables; Counts / qualifiers moved adds both binaries to the kill-line filter. No count is stated here: the report reads this file''s kill-line sites no-change.'
    fact: F10
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/e2e-test-strategy.md
    near: 261
    section: '## 6. E2E Test Strategy › #### Scenario: E5 — snapshot corruption → replay recovery'
    change: '  - No `-32603` is driven in a scanned home and no failpoint or counted panic was added; the unit layer holds it (`answer_a_panicking_dispatch_is_internal_error_and_closes`, `links_set_method_an_append_that_fails_is_internal_with_the_set_unchanged`).'
    sidecar: 'e2e-test-strategy: the `-32603` clause stated as held at the unit layer, over no E2E home'
    rationale: 'Report, Reverted / negative API facts, verbatim in substance.'
    fact: F8
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/e2e-test-strategy.md
    near: 264
    section: '## 6. E2E Test Strategy › #### Scenario: E5 — snapshot corruption → replay recovery'
    change: '  - `v1-47`''s "after every end-to-end test" is concretized to "after each job''s end-to-end run", on the delegate overseer''s PROVISIONAL word, owed the founder''s own word: `AGENT_RUN_KEEP_HOMES` `1` on the job, `if: always()`, the `viola-chaos-*` home out by the founder''s ruling of 2026-10-04.'
    sidecar: 'e2e-test-strategy: the after-each-job reading of `v1-47`, provisional on the delegate word'
    rationale: 'Report, Decisions & corrections: the concretization, its measure and its authority; the P5 word is PROVISIONAL.'
    fact: F9
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/e2e-test-strategy.md
    near: 247
    section: '## 6. E2E Test Strategy › #### Scenario: E5 — snapshot corruption → replay recovery'
    change: '  - As landed (chunk 2026-10-31-sanitised-error-surfaces), `crates/viola-e2e/tests/mcp_tools.rs` gains `mcp_tools_a_line_past_max_frame_before_initialize_exits_1_with_one_line` (exit 1, both streams empty, one `parse-rejected{parser:"mcp-stdin", detail:"oversize", count:1}` line; 11 is the unit child''s own code, not this exit), `mcp_tools_a_wrong_shaped_call_answers_no_caller_text` and `mcp_tools_error_results_hold_no_path_or_upstream_text` (an unstamped session, no row reaching a stamped path).'
    sidecar: 'e2e-test-strategy: the three new MCP cases beside the stdin-line case of the security sweep'
    rationale: 'Report, Spec claims disproved by measurement reads the before-`initialize` case; deviation 4 reads the unstamped session; the New text lists all three cases.'
    fact: F6
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/e2e-test-strategy.md
    near: 372
    section: '## 6. E2E Test Strategy › #### Scenario: E5 — snapshot corruption → replay recovery'
    change: '  - `tests/contract_error_bodies.rs` is the error-body enumeration in the contract suite: tables `CLI_DOCUMENTS`, `MCP_RESULTS`, `MCP_FIELD_WORDS`, `CHANNEL_FAULTS`, `CHANNEL_METHODS`, `NOT_ERROR_BODIES` and `PLUMBING` over `PRODUCT_ROOTS`; it carries `andromeda:walks-tree`.'
    sidecar: 'e2e-test-strategy: the error-body enumeration, its tables and its token, added to the contract suite'
    rationale: 'Report, Harness / gate surface names the token; deviation 3 names `PLUMBING`; the New text lists the tables.'
    fact: F1
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/e2e-test-strategy.md
    near: 372
    section: '## 6. E2E Test Strategy › #### Scenario: E5 — snapshot corruption → replay recovery'
    change: '    - That enumeration''s guards: `error_bodies_each_name_a_writer_and_a_witness_that_stand`, `channel_faults_are_the_seven_literal_bodies`, `product_error_writers_are_each_named`, `thiserror_messages_interpolate_nothing`, and the census self-check `census_finds_an_unnamed_writer_and_an_interpolated_message`.'
    sidecar: 'e2e-test-strategy: the error-body enumeration''s four guards and its census self-check'
    rationale: 'Report, the New text lists the five tests in `tests/contract_error_bodies.rs`; Outcome reads the enumeration green.'
    fact: F2
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan.md
    near: 623
    section: '## 6. E2E Test Strategy › #### Scenario: E5 — snapshot corruption → replay recovery'
    change: '  - Discharged at chunk 2026-10-31-sanitised-error-surfaces, the PREREQ of the route entry "Sanitised error surfaces": the enumeration''s rows for the status-line command''s stdout, the child pipe drain shared by `claude --version` and `claude agents --json`, and the `config.json` readers of the thresholds, the dialog hold and `claude_env_keep` each name a cap-driving case (`statusline_shell_out_past_the_frame_cap_keeps_exactly_the_cap`, `statusline_shell_out_a_command_the_cut_ends_is_a_failed_command`, `drain_keeps_cap_bytes_of_an_endless_pipe`, `read_thresholds_keeps_no_key_past_the_frame_cap`, `read_hold_keeps_no_key_past_the_frame_cap`, `env_keep_keeps_no_key_past_the_frame_cap`).'
    sidecar: 'test-plan §6: the contract suite''s Owed clause on the five `BOUNDARIES` rows discharged, the cap-driving cases named'
    rationale: 'Report, Expected amendments: the owed clause discharged, patterns `title` and `bound-rows` both name `test-plan.md:623` changed. Harness / gate surface: five rows'' `witness` renamed. Outcome names the cases and reads them met.'
    basis: .andromeda/test-plan.md:623
note: 'At e2e-test-strategy lines 262-263 the report names no holder for the no-`Cookie` and 0600 clauses once the helper is gone beyond the secret scan''s clean reading, so those lines stand as rules with their check unstated, an owner question for the orchestrator; #9.1 says `Command::env` where deviation 1 reads `tests/tui_env_strip.rs` as planting through `env_set`, standing and not moved by this chunk, so not proposed.'
