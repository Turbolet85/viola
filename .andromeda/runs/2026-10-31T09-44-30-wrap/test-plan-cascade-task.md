# Task — test-plan · 2026-10-31-sanitised-error-surfaces

## The session
CLI `--json`, MCP errors and channel error data now carry codes and fixed messages only, with no paths or upstream text: `viola_channel::sanitised_fault` and `viola_core::is_code_word` screen a peer's reply in `reply_of` and `of_reply`, and the channel-response line's `detail` closes to three words.
It added three surface witnesses and an enumeration with a census of writers (`tests/contract_error_bodies.rs`, `tests/channel_error_bodies.rs`, `tests/cli_error_bodies.rs`, the MCP cases in `crates/viola-e2e/tests/mcp_tools.rs`), reading one check in `tests/support/events.rs`.
It discharged the two owed clauses test-plan names on this route entry: the stdin line past `MAX_FRAME` before `initialize` is now driven, and five `BOUNDARIES` rows now name cap-driving cases.
It retired `viola_agent_claude::Refusal` (folded into `AgentError`) and both private `fault_detail` copies.
It measured five standing claims false: `viola mcp` exit 11, `CoreError` among the crate enums, the oversize `answer` error document, a `process-exit` line on the `mcp` role, and test-plan §6's after-every-test error-body check, which was never built and is replaced, not built now.
CI `ci#41188681876` over `fa7be1f37746` read green, 15/15 checks; `test` jobs passed 3347 (ubuntu), 3301 (macos), 3300 (windows-2025).

## Reached files
- tests/contract_error_bodies.rs
- tests/channel_error_bodies.rs
- tests/cli_error_bodies.rs
- crates/viola-core/src/lib.rs
- crates/viola-channel/src/lib.rs
- crates/viola-channel/src/client.rs
- crates/viola-channel/src/server.rs
- schemas/diag-line.v1.json
- src/cmd/client.rs
- crates/viola-mcp/src/result.rs
- crates/viola-mcp/src/lib.rs
- crates/viola-mcp/tests/handlers.rs
- crates/viola-agent-claude/src/lib.rs
- src/run/mod.rs
- src/cmd/run.rs
- src/cmd/verify.rs
- tests/support/events.rs
- Cargo.toml
- .config/nextest.toml
- crates/viola-e2e/tests/mcp_tools.rs
- src/cmd/hook/statusline.rs
- src/run/version_gate.rs
- src/run/budget.rs
- src/run/dialog.rs
- src/run/env.rs
- tests/contract_bounded_readers.rs
- viola-0.1.0/verification-matrix.json

## Touches
- `.andromeda/test-plan.md:390`, the value: a `viola mcp` stdin line past `MAX_FRAME` before `initialize` moves from exit 11 to process exit 1, both streams empty, one `parse-rejected` `mcp-stdin` / `oversize` line with count 1; 11 is only the unit child's own code for `ServeError::Initialize`. `.andromeda/test-plan-detail/unit-test-strategy.md:52` states no exit and reads no change. `budget-paused`'s exit 11 is another code and stays.
- `.andromeda/test-plan.md:390`, the owed clause: its not-driven statement and its PREREQ owner on the route entry Sanitised error surfaces are discharged, the case now driven at unit and E2E.
- `.andromeda/test-plan.md:623`, the Owed clause of the contract suite: discharged, five `BOUNDARIES` rows (`tests/contract_bounded_readers.rs:91`, `:109`, `:118`, `:127`, `:217`) now name cap-driving witnesses. `.andromeda/test-plan.md:622` and `.andromeda/test-plan-detail/e2e-test-strategy.md:371` name the binary and read no change.
- `.andromeda/test-plan.md:605` and `.andromeda/test-plan-detail/e2e-test-strategy.md:259`, the error sanitization suite: the after-every-E2E-test helper check was measured absent and is not built; the text must say plainly it was replaced by the three surface witnesses, the enumeration and the census of writers, reading the check in `tests/support/events.rs`.
- The same suite's `-32603` clause: no scanned home drives a `-32603`, so that clause is held at the unit layer rather than over E2E homes.
- `.andromeda/test-plan-detail/integration-test-strategy.md:59`, the oversize `viola answer` sentence: disproved, the exit-2 paths write the fixed `UNUSABLE_RESPONSE` / `UNUSABLE_TEXT` line to stderr whatever `--json` says, no document. `.andromeda/test-plan.md:202` is a label and reads no change.
- The `ci` nextest profile's kill-line filter: 22 to 24 `binary(…)` terms in `.config/nextest.toml`. Flag a stated count only; `.andromeda/test-plan-detail/e2e-test-strategy.md:38`, `:306`, `:322`, `.andromeda/test-plan-detail/quality-gates-coverage-targets.md:19` and `.andromeda/test-plan-detail/test-strategy.md:29` read no change in the report's sweep.
- Test inventories and counts: the root binaries gain one `contract_`, one `channel_` and one `cli_` file, so any stated binary count, suite list or pass total moves; latest totals 3347, 3301 and 3300 by OS.
- `.andromeda/test-plan.md:269` and `:196`, the error-sanitization and MCP security vectors: indirectly, through the new witnesses; each surface's no-path, no-upstream-text claim now has a named witness, and channel `error.data` is closed values or null.
- The `RefusalReason` `unknown` fallback and exit 14 lines (`.andromeda/test-plan.md:261`, `:392`): sharpened, a refusal that is no code word now reads `unknown`, a string `detail` that is no code word reads null, a non-string `detail` is still no known reply; the exit stays 14.
- Cross-surface parity (CLI and MCP return the same `{refusal, detail}`): indirectly, both surfaces now share one screen, `sanitised_fault` and `is_code_word`, in place of two private `fault_detail` copies.
- Any citation of `viola_agent_claude::Refusal`, `Refusal::NotFound` or `fault_detail`: retired. `resolve_program` now returns `Result<PathBuf, AgentError>` in the crate and in `src/run/mod.rs`; `.cmd` refusal and not-found outputs are unchanged.
- Any citation of `crates/viola-channel/src/client.rs` `fault_of(i64)`: now `fault_of(&Value)`, reading `data` for `-32602`.
- Schema conformance (`.andromeda/test-plan.md:607`, obs gate G4): indirectly, `schemas/diag-line.v1.json:42` closes channel-response `detail` to `unsupported-version`, `invalid-params`, `release-from-driver`, absent on every other such line; so the conformance check now rejects any other word. `v` stays 1.
- The `-32602` / higher-`v` trigger lines: read against the moved surroundings and unmoved; the seven `ProtocolError` wire bodies and the wire form did not change, only the log line's `detail` and the clients' reading of a fault.
- §1 entity lines for `viola-core`, `viola-channel` and `viola-mcp`, and §4's lists for them: indirectly, each surface now has a code-word or fault screen under test that the lists do not name.
- `CoreError`, if any test-plan entity names it: measured absent from the code, 0 hits. The report's sweep names no test-plan site, so flag only an entity that states it; likewise a stated `thiserror` derive count, now 9 from 10.
- A statement that `viola mcp` writes a `process-exit` line of its own: measured false, the `mcp` role writes none; the report's sweep names no test-plan site, so flag only an entity that states it.
- `.andromeda/test-plan.md:857` zero-flakiness budget: indirectly, `verify_stopped_during_the_resume_run_removes_its_dir_and_exits_1` went red in both cases at the 7 s `WITHIN` in one pre-push under a `CONTENDED` host record and green in every other run; contention is inferred, not proved; no bound moved; its owner or note is P5's.
- Any test-plan note on Path 1's CI timeout watch: three green runs on `test (ubuntu-latest)` (15.010 s, 14.205 s, 9.067 s), its retire condition reached; P5 retires it, its runner-variance reading stays provisional.
- CI lifecycle lines on kept homes and the secret scan (`.andromeda/test-plan.md:697`): indirectly, `v1-47` reads them for its after-each-job reading: `AGENT_RUN_KEEP_HOMES` `1` on the job, `if: always()`, the `viola-chaos-*` home out.
- The per-child env rule (`.andromeda/test-plan.md:695`, set via `Command::env`): indirectly, `tests/channel_error_bodies.rs` plants its value through `OuterPty::spawn_without`'s `env_set`, as `tests/tui_env_strip.rs` does; no `set_var`.
- The one-set-of-helpers rule for `tests/support/` (`.andromeda/test-plan.md:319`): indirectly, `tests/contract_error_bodies.rs` holds its own scanner beside `tests/contract_bounded_readers.rs`'s, since one blanks string literals and the other reads them.
- The table conventions of §4: indirectly, `code_word_literal_table` follows the `viola-core` literal-table rule and the `mcp_tools` rows the `viola-e2e` labelled-table rule; re-read and unmoved.

## New facts
- F1 · e2e-test-strategy · `tests/contract_error_bodies.rs` is the error-body enumeration in the contract suite: tables `CLI_DOCUMENTS`, `MCP_RESULTS`, `MCP_FIELD_WORDS`, `CHANNEL_FAULTS`, `CHANNEL_METHODS`, `NOT_ERROR_BODIES` and `PLUMBING` over `PRODUCT_ROOTS`; it carries `andromeda:walks-tree`.
- F2 · e2e-test-strategy · That enumeration's guards: `error_bodies_each_name_a_writer_and_a_witness_that_stand`, `channel_faults_are_the_seven_literal_bodies`, `product_error_writers_are_each_named`, `thiserror_messages_interpolate_nothing`, and the census self-check `census_finds_an_unnamed_writer_and_an_interpolated_message`.
- F3 · e2e-test-strategy · `tests/channel_error_bodies.rs` `channel_error_bodies_hold_no_path_or_upstream_text`: seven raw-channel fault rows against a wrapper booted with a planted value, bodies clean and the three `detail` words on the lines read.
- F4 · e2e-test-strategy · `tests/cli_error_bodies.rs` `cli_error_documents_hold_no_path_or_upstream_text`: each driven CLI `--json` failure gives one document, a literal exit, empty stderr, no ESC and a clean body, across dead, refused and live rows.
- F5 · e2e-test-strategy · `tests/support/events.rs` `error_body_hit` and `assert_error_body_clean`: classes `absolute-path`, `caused-by`, `serde-path`, `planted`, reporting place and class, never bytes; self-tested by `error_body_check_reads_each_class_as_a_hit` and `error_body_check_reports_no_byte_of_what_it_read` in `tests/cli_error_bodies.rs`.
- F6 · e2e-test-strategy · `crates/viola-e2e/tests/mcp_tools.rs` gains `mcp_tools_error_results_hold_no_path_or_upstream_text` (an unstamped session, no row reaching a stamped path), `mcp_tools_a_wrong_shaped_call_answers_no_caller_text` and `mcp_tools_a_line_past_max_frame_before_initialize_exits_1_with_one_line`.
- F7 · e2e-test-strategy · rmcp's client reads the wire's null `data` as `None`, so the live unknown-tool case asserts `None`; on the wire it is the product's `Some(Value::Null)`.
- F8 · e2e-test-strategy · No `-32603` is driven in a scanned home and no failpoint or counted panic was added; the unit layer holds it (`answer_a_panicking_dispatch_is_internal_error_and_closes`, `links_set_method_an_append_that_fails_is_internal_with_the_set_unchanged`).
- F9 · e2e-test-strategy · `v1-47`'s after every end-to-end test is concretized to after each job's end-to-end run, on the delegate overseer's provisional word, owed the founder's own word.
- F10 · e2e-test-strategy · `channel_error_bodies` and `cli_error_bodies` are `[[test]]` tables with `required-features = ["fake-agent"]` and terms of the `ci` profile's kill-line filter.
- F11 · integration-test-strategy · `crates/viola-mcp/tests/handlers.rs` `handlers_a_refusal_of_free_text_reads_as_unknown_with_no_detail` holds the MCP screen against a test wrapper.
- F12 · integration-test-strategy · channel-response `detail` is written on both the server's and the client's line, level WARN unchanged, with no name or param on the line, held by `link_verbs_log_the_channel_pair_and_no_names` and the schema.
- F13 · unit-test-strategy · `viola_core::is_code_word` accepts 1 to 64 bytes of `[a-z0-9-]` with a lower-case letter first, held by `code_word_literal_table`.
- F14 · unit-test-strategy · `viola_channel::sanitised_fault` cases: `sanitised_fault_keeps_each_wire_body`, `sanitised_fault_carries_nothing_of_the_peer_but_closed_values`, `sanitised_fault_keeps_a_wrapper_of_sixty_four_version_characters`, `sanitised_fault_without_an_integer_code_is_nothing`.
- F15 · unit-test-strategy · A fault code outside the five known reads the fixed message `wrapper fault` with `data: null` on `--json` and MCP; the human line `error: wrapper fault  <code>` is unchanged.
- F16 · unit-test-strategy · `ProtocolError::cause` tells the three `-32602` faults apart (`protocol_error_cause_tells_the_three_invalid_params_apart`, `answer_invalid_params_names_its_cause_and_no_param`, `answer_faults_other_than_invalid_params_carry_no_detail`, `reply_from_logs_the_cause_of_each_invalid_params`).
- F17 · unit-test-strategy · A code word outside the closed refusal list stays visible in the document and in `refused: <word>` by design, exit 14 by `reason_of`; held with the screen by `reply_of_reads_the_three_shapes` and `of_reply_maps_each_reply_shape`.
- F18 · unit-test-strategy · `crates/viola-mcp/src/lib.rs` `serve_at_a_line_past_max_frame_before_initialize_is_the_initialize_error` holds the child's `ServeError::Initialize` end at the unit layer.
- F19 · unit-test-strategy · Cap-driving cases for the bounded readers: `statusline_shell_out_past_the_frame_cap_keeps_exactly_the_cap`, `statusline_shell_out_a_command_the_cut_ends_is_a_failed_command`, `drain_keeps_cap_bytes_of_an_endless_pipe`, `read_thresholds_keeps_no_key_past_the_frame_cap`, `read_hold_keeps_no_key_past_the_frame_cap`, `env_keep_keeps_no_key_past_the_frame_cap`.
- F20 · unit-test-strategy · `AgentError` gains `BatchScriptChild` and `ProgramNotFound` with their two messages unchanged, held by `refusal_messages_are_fixed`, `tests/cli_program_resolution.rs` and `tests/cli_verify.rs`.

## Untouched
- The fixture hygiene walk and its classes (`tests/contract_fixture_hygiene.rs`, `tests/support/hygiene.rs`): the new check shares the word `absolute-path` but is a separate checker over error bodies, not fixtures.
- The fake-agent contract and receipts: no mode, script, seam or failpoint was added; the `fake-agent` feature on the two new `[[test]]` tables is only a use of it.
- Test-home setup and teardown, `seed_conpty`, `stamped_home` and `tests/support/home.rs`: outside this chunk's list, unchanged.
- Coverage thresholds, `COVERAGE_IGNORE` and the mutation gate: no gate, threshold, exclusion or suite moved; the session only reported a green coverage run.
- `.github/workflows/ci.yml`'s jobs, matrix, uploads and permissions: no CI step moved; the session's `v1-47` reading only reads them.
- The harness contract in §3 (commands, status shape, exit codes, `gate::SUITES`): no harness verb or status shape moved.
- `viola revive` and its refusal cases: `src/cmd/revive.rs`'s private `Refusal` is another type and stays; only `viola_agent_claude::Refusal` is retired.
- The `list` row integrity suite and `src/human/mirror.rs`'s escaping: no human-mode layout or `hint:` line moved.
- E5 snapshot replay and `viola-state`'s readers: no crate-state path was changed.
- Mocking discipline's process, HTTP and fake-server entries: no double, fake server or failpoint was added.
- File and function naming conventions: the three new binaries use the existing `contract_`, `channel_` and `cli_` prefixes; the rule is unchanged.
