## 2026-10-31-sanitised-error-surfaces — the `ci` kill line's verify-driven binaries
**Section:** §3 Bootstrap phases (registry row `bootstrap-phases-derive-for-route-setup-project.md`: test-runner-install, `[profile.ci]`)
**Change:**
- The verify-driven binaries on the `ci` profile's `{ period = "10s", terminate-after = 2 }` line (a 20 s kill) read twenty-four (was twenty-two).
- The `binary(…)` names now list `channel_error_bodies` and `cli_error_bodies`, on the line because each one's witness starts from `stamped_home`.
**Why:** the `ci` filter gained the chunk's two new binaries; a count of `binary(` over `.config/nextest.toml` reads 24.
**Ref:** .andromeda/runs/2026-10-31T09-44-30-wrap/

## 2026-10-31-sanitised-error-surfaces — the error bodies' surface witnesses and the planted value
**Section:** §6 E2E Test Strategy (e2e-test-strategy.md: the driver table's cli, wrapper-channel and MCP stdio rows; the suite-error-sanitization `[[test]]` item) · §7 Test Data & Fixtures (test-data-fixtures.md: a new row)
**Change:**
- cli row: `tests/cli_error_bodies.rs` `cli_error_documents_hold_no_path_or_upstream_text` holds each CLI `--json` failure it drives, across dead, refused and live rows, to one document, a literal exit, empty stderr, no ESC and a clean body.
- wrapper-channel row: `tests/channel_error_bodies.rs` `channel_error_bodies_hold_no_path_or_upstream_text` writes seven fault rows to the channel of a wrapper booted with a planted value, its bodies clean and the three `detail` words read on the lines.
- MCP stdio row: `mcp_tools_error_results_hold_no_path_or_upstream_text` (an unstamped session, no row reaching a stamped path) and `mcp_tools_a_wrong_shaped_call_answers_no_caller_text` stand in `tests/mcp_tools.rs`. rmcp's client reads the wire's `"data":null` as `None`, so the live unknown-tool case asserts `None`; on the wire it is the product's `Some(Value::Null)`.
- `channel_error_bodies` and `cli_error_bodies` are `[[test]]` tables with `required-features = ["fake-agent"]`, each on the `ci` profile's 20 s kill line.
- §7's new row, strategy per-test (rstest `stamped_home`): `booted_with_the_value` starts from `stamped_home` and boots through `OuterPty::spawn_without`, not `Wrapper` or `Command::env` (`Wrapper` takes no planted value); the value reaches that one child through the PTY spawn's `env_set`, as `tests/tui_env_strip.rs` does; `assert_error_body_clean` takes the planted values and reads a hit as class `planted`, reporting place and class, never bytes.
**Why:** the chunk added these witnesses; `Wrapper` offered no way to plant the value, so the boot went through the PTY spawn. An oracle asserted against the wire form read red through rmcp's client, hence `None`. The row's "no planted value" wording, in place of "no environment", is the overseer's answer as the founder's delegate at the P2 halt, PROVISIONAL.
**Kept:** §7's standing line on `Wrapper::boot_as_user`, which sets `HOME` (on Windows `USERPROFILE`) on its one child: that wording keeps it true.
**Ref:** .andromeda/runs/2026-10-31T09-44-30-wrap/

## 2026-10-31-sanitised-error-surfaces — `viola mcp`'s exit at a line past the cap before `initialize`
**Section:** §6 E2E Test Strategy (e2e-test-strategy.md: the MCP stdio row) · §1 Test Scope Summary (test-scope-summary.md: the MCP surface, **As landed (the line before `initialize`)**) · §4 Unit Test Strategy (unit-test-strategy.md: viola-mcp; test-plan.md: viola-mcp)
**Change:**
- test-plan.md §4 was: not driven by any test, ending `serve` with `ServeError::Initialize` (exit 11), owner the route entry "Sanitised error surfaces", as a PREREQ. Now: the case is driven, the owner clause is gone, and it ends `serve` with `ServeError::Initialize`, as a closed stdin before `initialize` does.
- Exit was 11; now 11 is the unit child's own code and `viola mcp` itself exits 1, both streams empty, with one `parse-rejected{parser:"mcp-stdin", detail:"oversize", count:1}` line.
- Its holders: unit `serve_at_a_line_past_max_frame_before_initialize_is_the_initialize_error` (`crates/viola-mcp/src/lib.rs`), E2E `mcp_tools_a_line_past_max_frame_before_initialize_exits_1_with_one_line` (`crates/viola-e2e/tests/mcp_tools.rs`).
**Why:** measured: the E2E case reads exit 1 with both streams empty and that one line; 11 is the unit child's own code, and the process exit for `Role::Mcp` with `Failed` is 1. The process exit stays 1 and the masters move, as the plan leaned. The owed clause is discharged.
**Ref:** .andromeda/runs/2026-10-31T09-44-30-wrap/

## 2026-10-31-sanitised-error-surfaces — the after-every-test error-body check, replaced and not built
**Section:** §6 E2E Test Strategy (e2e-test-strategy.md: suite-error-sanitization; test-plan.md: Error sanitization and secret scan) · §2 Test Strategy (test-strategy.md: the `tests/support` item, the trigger table's error sanitization row)
**Change:**
- Was: after every E2E test, the `events` support helper checks. Now: no helper runs after each E2E test; none is built, replaced by the three surface witnesses, the enumeration and the census of writers (`tests/contract_error_bodies.rs`), reading the check in `tests/support/events.rs`.
- The check: `error_body_hit` and `assert_error_body_clean`, classes `absolute-path` · `caused-by` · `serde-path` · `planted`, reporting place and class, never bytes, self-tested by `error_body_check_reads_each_class_as_a_hit` and `error_body_check_reports_no_byte_of_what_it_read` (`tests/cli_error_bodies.rs`); test-strategy's `events.rs` description names it.
- Every `-32603` stays exactly `"internal error"` / `null`, held at the unit layer (`answer_a_panicking_dispatch_is_internal_error_and_closes`, `links_set_method_an_append_that_fails_is_internal_with_the_set_unchanged`): no scanned home drives one, no failpoint or counted panic stands for one.
- `v1-47`'s "after every end-to-end test" reads "after each job's end-to-end run", PROVISIONAL: `AGENT_RUN_KEEP_HOMES` `"1"` on the job and `if: always()`, the `viola-chaos-*` home out.
- test-plan.md names the holders: the no-`Cookie`, stripped-value and canary clauses are `viola-harness secret-scan`'s, after each job's end-to-end run; the 0600 clause `write_detail_refuses_a_widened_diagnostics_folder_and_leaves_it`'s (`secret_scan.rs` reads `cookie:` whatever its case).
- The trigger row names `cli_error_bodies.rs`, `channel_error_bodies.rs` and `mcp_tools.rs` with their witness cases, and `contract_error_bodies.rs`.
**Why:** the check was measured absent before this chunk and is not built. `v1-47`'s reading is the delegate overseer's word, PROVISIONAL, owed the founder's own word; escalated at the P2 halt as a contradiction, it was applied on the overseer's answer as the founder's delegate, PROVISIONAL the same. The chaos home stands out by the founder's ruling of 2026-10-04, as the report records. The report names no holder for the log, 0600 and canary clauses; the disposition records them named by main's direction.
**Ref:** .andromeda/runs/2026-10-31T09-44-30-wrap/

## 2026-10-31-sanitised-error-surfaces — the error-body enumeration in the contract suite
**Section:** §6 E2E Test Strategy (e2e-test-strategy.md: Contract suite)
**Change:**
- `tests/contract_error_bodies.rs` (a `contract_` binary, `run --e2e`) is the error-body enumeration: the tables `CLI_DOCUMENTS`, `MCP_RESULTS`, `MCP_FIELD_WORDS`, `CHANNEL_FAULTS`, `CHANNEL_METHODS`, `NOT_ERROR_BODIES` and `PLUMBING` over `PRODUCT_ROOTS`, its file carrying the `andromeda:walks-tree` token.
- Its guards: `error_bodies_each_name_a_writer_and_a_witness_that_stand`, `channel_faults_are_the_seven_literal_bodies`, `product_error_writers_are_each_named` and `thiserror_messages_interpolate_nothing`, with the census self-check `census_finds_an_unnamed_writer_and_an_interpolated_message`.
**Why:** the chunk's new contract binary, green at the final HEAD's run. `PLUMBING` names the census lines that build or route a document a row names, so a row stays one body.
**Ref:** .andromeda/runs/2026-10-31T09-44-30-wrap/

## 2026-10-31-sanitised-error-surfaces — the reply screen and the fault screen
**Section:** §5 Integration Test Strategy (integration-test-strategy.md: the MCP screen case) · §1 Test Scope Summary (test-scope-summary.md: the `viola-channel` entity's **As landed (the fault screen)**, the MCP surface's **As landed (a peer's reply)**) · §4 Unit Test Strategy (unit-test-strategy.md: a new viola-core group, viola-channel, viola-mcp, root bin; test-plan.md: viola-channel, viola-mcp, root bin)
**Change:**
- viola-core group: `viola_core::is_code_word` accepts 1 to 64 bytes of `[a-z0-9-]`, a lower-case letter first (`code_word_literal_table`).
- `viola_channel::sanitised_fault` reads a peer's error for `reply_of` and `of_reply`: each of the five known codes takes its fixed `message`; `-32602` keeps only an integer `data.supported` with a version-text `data.wrapper`, or a `data.reason` exactly `release-from-driver`, any other reading `invalid params` with `data: null`; every other known code reads `data: null`; a code outside the five reads `wrapper fault` with `data: null`; an error without an integer `code` reads nothing. The human line reads `error: wrapper fault  <code>`.
- Its units: `sanitised_fault_keeps_each_wire_body`, `sanitised_fault_carries_nothing_of_the_peer_but_closed_values`, `sanitised_fault_keeps_a_wrapper_of_sixty_four_version_characters`, `sanitised_fault_without_an_integer_code_is_nothing`.
- `of_reply` and `reply_of`: a `refusal` that is no code word reads `unknown`, a string `detail` that is no code word reads `null` / `None`, a non-string `detail` is no known reply (`of_reply_maps_each_reply_shape`, `reply_of_reads_the_three_shapes`, `handlers_a_refusal_of_free_text_reads_as_unknown_with_no_detail`, `mcp_tools_error_results_hold_no_path_or_upstream_text`).
- A code word outside the closed refusal list stays visible in the document and in `refused: <word>` by design, its exit `unknown`'s 14 by `reason_of`.
- test-plan.md §4's three broad lines name these screens.
**Why:** `sanitised_fault` replaces the two private `fault_detail` copies, so a peer's `message`, `data`, `refusal` or `detail` reaches no document or text. `wrapper fault` is a new fixed output word on `--json` and MCP; the human line is unchanged.
**Ref:** .andromeda/runs/2026-10-31T09-44-30-wrap/

## 2026-10-31-sanitised-error-surfaces — the exit-2 paths that write no document
**Section:** §5 Integration Test Strategy (integration-test-strategy.md: oversize `answer`) · §1 Test Scope Summary (test-scope-summary.md: the cli **Signal**; test-plan.md: the cli surface)
**Change:**
- Oversize `viola answer` stdin was: exits non-zero with one `--json` error document before connecting. Now: exits 2 before connecting, with the fixed `UNUSABLE_RESPONSE` line (`src/cmd/answer.rs`) on stderr whatever `--json` says and no document (named in `NOT_ERROR_BODIES`, `tests/contract_error_bodies.rs`).
- The cli **Signal** gains the exception: the exit-2 paths of `viola answer` (`UNUSABLE_RESPONSE`) and `viola send` (`UNUSABLE_TEXT`) write that fixed line to stderr, with no document; test-plan.md's cli surface carries the same exception.
**Why:** measured: these exit-2 paths write the fixed line whatever `--json` says, disproving the one-document claim.
**Ref:** .andromeda/runs/2026-10-31T09-44-30-wrap/

## 2026-10-31-sanitised-error-surfaces — `channel-response`'s closed `detail` and the wire bodies
**Section:** §5 Integration Test Strategy (integration-test-strategy.md: `tests/cli_links.rs`) · §1 Test Scope Summary (test-scope-summary.md: the wrapper-channel surface's **As landed (the error bodies)**, the fault screen's `ProtocolError::cause` sentence) · §4 Unit Test Strategy (unit-test-strategy.md: viola-channel)
**Change:**
- A `channel-response` line's `detail` is one of `unsupported-version` · `invalid-params` · `release-from-driver` for the three `-32602` faults and absent on every other such line, on both the server's and the client's line at WARN, with no name or param on it, held by `link_verbs_log_the_channel_pair_and_no_names` and `schemas/diag-line.v1.json`.
- `ProtocolError::cause` gives those words and is `None` for the other four faults: `protocol_error_cause_tells_the_three_invalid_params_apart`, `answer_invalid_params_names_its_cause_and_no_param`, `answer_faults_other_than_invalid_params_carry_no_detail`, `reply_from_logs_the_cause_of_each_invalid_params`.
- The seven `ProtocolError` wire bodies stand literal with `v` 1, a too-new request answered `-32602` with the wrapper's version (`protocol_error_body_is_the_literal_wire_object`, `channel_error_bodies_hold_no_path_or_upstream_text`). No `-32603` is driven in a scanned home; the unit layer holds it.
**Why:** the schema closes `detail` from any string to the three words; the wire form does not change and the seven bodies are unchanged.
**Ref:** .andromeda/runs/2026-10-31T09-44-30-wrap/

## 2026-10-31-sanitised-error-surfaces — the bounded readers' cap-driving witnesses
**Section:** §1 Test Scope Summary (test-scope-summary.md: the budget path's statusline signal) · §4 Unit Test Strategy (unit-test-strategy.md: root bin) · §6 E2E Test Strategy (test-plan.md: Contract suite)
**Change:**
- The Contract suite's clause was: Owed, the five rows name a witness that reads the bounded read without driving past its cap, their caps held by the row's needle and the census alone, owner the route entry "Sanitised error surfaces", as a PREREQ. Now: As landed, the five `BOUNDARIES` rows for the status-line command's stdout, the child pipe drain shared by `claude --version` and `claude agents --json`, and the `config.json` readers of the thresholds, the dialog hold and `claude_env_keep` each name a cap-driving case.
- The cases: `statusline_shell_out_past_the_frame_cap_keeps_exactly_the_cap` and `statusline_shell_out_a_command_the_cut_ends_is_a_failed_command` (`src/cmd/hook/statusline.rs`), `drain_keeps_cap_bytes_of_an_endless_pipe` (`src/run/version_gate.rs`), `read_thresholds_keeps_no_key_past_the_frame_cap` (`src/run/budget.rs`), `read_hold_keeps_no_key_past_the_frame_cap` (`src/run/dialog.rs`), `env_keep_keeps_no_key_past_the_frame_cap` (`src/run/env.rs`).
- The statusline signal gains: the shell-out keeps exactly the frame cap of a longer stdout, and a command the cut ends is a failed command.
**Why:** the chunk renamed the five rows' `witness` to cap-driving cases, discharging the owed clause; the cases and the census read green.
**Ref:** .andromeda/runs/2026-10-31T09-44-30-wrap/

## 2026-10-31-sanitised-error-surfaces — `resolve_program`'s error after `Refusal`
**Section:** §4 Unit Test Strategy (unit-test-strategy.md: viola-agent-claude)
**Change:**
- `resolve_program` returns `Result<PathBuf, AgentError>`, `AgentError` holding `BatchScriptChild` and `ProgramNotFound`, their messages and outputs held by `refusal_messages_are_fixed`, `tests/cli_program_resolution.rs` and `tests/cli_verify.rs`.
**Why:** `viola_agent_claude::Refusal` is removed and folded into `AgentError` (`ProgramNotFound` was `Refusal::NotFound`), its two messages and the outputs unchanged.
**Ref:** .andromeda/runs/2026-10-31T09-44-30-wrap/
