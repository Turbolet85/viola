# Apply — test-plan · part 2 of 3

## Verdicts
- fanout-test-plan--cascade-g1.md 6 · applied · .andromeda/test-plan-detail/unit-test-strategy.md:10+#1
- fanout-test-plan--cascade-g1.md 7 · applied · .andromeda/test-plan-detail/unit-test-strategy.md:10+#2
- fanout-test-plan--cascade-g1.md 8 · applied · .andromeda/test-plan-detail/unit-test-strategy.md:10+#3
- fanout-test-plan--cascade-g1.md 10 · applied · .andromeda/test-plan-detail/unit-test-strategy.md:36+#1
- fanout-test-plan--cascade-g1.md 11 · applied · .andromeda/test-plan-detail/unit-test-strategy.md:52+#1
- fanout-test-plan--cascade-g1.md 12 · applied · .andromeda/test-plan-detail/unit-test-strategy.md:52+#2
- fanout-test-plan--cascade-g1.md 15 · applied · .andromeda/test-plan-detail/unit-test-strategy.md:59+#1
- fanout-test-plan--cascade-g1.md 16 · applied · .andromeda/test-plan-detail/unit-test-strategy.md:59+#2
- fanout-test-plan--cascade-g1.md 17 · applied · .andromeda/test-plan-detail/unit-test-strategy.md:81+#1
- fanout-test-plan--cascade-new-unit-test-strategy.md 1 · applied · .andromeda/test-plan-detail/unit-test-strategy.md:7+#1
- fanout-test-plan--cascade-g2.md 1 · applied · .andromeda/test-plan-detail/integration-test-strategy.md:59#1
- fanout-test-plan--cascade-g2.md 2 · applied · .andromeda/test-plan-detail/integration-test-strategy.md:57+#1
- fanout-test-plan--cascade-g2.md 3 · applied · .andromeda/test-plan-detail/integration-test-strategy.md:89+#1

## Edits
- after: .andromeda/test-plan-detail/unit-test-strategy.md:10
  anchor: «  - JSON-RPC error mapping: -32700 / -32600 / -32601 / -32602 with `data:{supported,wrapper}` / -32603 with the exact `"internal error"` and `data:null`.»
  add: «  - The fault screen `viola_channel::sanitised_fault` (`crates/viola-channel/src/lib.rs`): `sanitised_fault_keeps_each_wire_body`, `sanitised_fault_carries_nothing_of_the_peer_but_closed_values`, `sanitised_fault_keeps_a_wrapper_of_sixty_four_version_characters` and `sanitised_fault_without_an_integer_code_is_nothing`.»
- after: .andromeda/test-plan-detail/unit-test-strategy.md:10
  anchor: «  - JSON-RPC error mapping: -32700 / -32600 / -32601 / -32602 with `data:{supported,wrapper}` / -32603 with the exact `"internal error"` and `data:null`.»
  add: «  - A fault code outside the five reads `{"code": <code>, "message": "wrapper fault", "data": null}` on `--json` and MCP, held by `sanitised_fault_carries_nothing_of_the_peer_but_closed_values`; the human line reads `error: wrapper fault  <code>` (`src/human/mirror.rs`).»
- after: .andromeda/test-plan-detail/unit-test-strategy.md:10
  anchor: «  - JSON-RPC error mapping: -32700 / -32600 / -32601 / -32602 with `data:{supported,wrapper}` / -32603 with the exact `"internal error"` and `data:null`.»
  add: «  - `ProtocolError::cause` tells the three `-32602` faults apart and is `None` for the other four: `protocol_error_cause_tells_the_three_invalid_params_apart` (`crates/viola-channel/src/lib.rs`), `answer_invalid_params_names_its_cause_and_no_param` and `answer_faults_other_than_invalid_params_carry_no_detail` (`crates/viola-channel/src/server.rs`) and `reply_from_logs_the_cause_of_each_invalid_params`.»
- after: .andromeda/test-plan-detail/unit-test-strategy.md:36
  anchor: «  - npm-shim → `claude.exe` resolution over fixture shim files in a tempdir.»
  add: «  - `resolve_program` returns `Result<PathBuf, AgentError>`, `AgentError` holding `BatchScriptChild` and `ProgramNotFound`, their messages and outputs held by `refusal_messages_are_fixed`, `tests/cli_program_resolution.rs` and `tests/cli_verify.rs`.»
- after: .andromeda/test-plan-detail/unit-test-strategy.md:52
  anchor: «  - The `serve` child cases (`crates/viola-mcp/src/lib.rs`), each child killed at its 5 s bound: `serve_ends_the_session_at_a_line_past_max_frame` and `serve_on_a_closed_stdin_is_the_initialize_error` (`ServeError::Initialize`).»
  add: «  - A line past `MAX_FRAME` before `initialize` ends the `serve` child in `ServeError::Initialize`: `serve_at_a_line_past_max_frame_before_initialize_is_the_initialize_error` (`crates/viola-mcp/src/lib.rs`).»
- after: .andromeda/test-plan-detail/unit-test-strategy.md:52
  anchor: «  - The `serve` child cases (`crates/viola-mcp/src/lib.rs`), each child killed at its 5 s bound: `serve_ends_the_session_at_a_line_past_max_frame` and `serve_on_a_closed_stdin_is_the_initialize_error` (`ServeError::Initialize`).»
  add: «  - The reply screen `of_reply` (`crates/viola-mcp/src/result.rs`), held by `of_reply_maps_each_reply_shape`: a `refusal` that is no code word reads `unknown`, a string `detail` that is no code word reads `null`, a non-string `detail` is no known reply, and a peer fault reads through `viola_channel::sanitised_fault`.»
- after: .andromeda/test-plan-detail/unit-test-strategy.md:59
  anchor: «  - The exit-code mapping from `RefusalReason` / fault to 0/1/2/10–14/20/21. It includes the multi-version-compat case: a wrapper `{"result":{"refusal":"<literal unknown string>","detail":…}}` payload deserialises to the `#[serde(other)] unknown` variant and maps to exit 14 with a `--json` refusal document. It never panics and never maps to 20.»
  add: «  - The reply screen `reply_of` (`src/cmd/client.rs`), held by `reply_of_reads_the_three_shapes`: a `refusal` that is no code word reads `unknown`, a string `detail` that is no code word reads `None`, a non-string `detail` is no known reply, and a peer fault reads through `viola_channel::sanitised_fault`.»
- after: .andromeda/test-plan-detail/unit-test-strategy.md:59
  anchor: «  - The exit-code mapping from `RefusalReason` / fault to 0/1/2/10–14/20/21. It includes the multi-version-compat case: a wrapper `{"result":{"refusal":"<literal unknown string>","detail":…}}` payload deserialises to the `#[serde(other)] unknown` variant and maps to exit 14 with a `--json` refusal document. It never panics and never maps to 20.»
  add: «  - A code word outside the closed refusal list stays visible in the document and in `refused: <word>` by design, its exit `unknown`'s 14 by `reason_of`.»
- after: .andromeda/test-plan-detail/unit-test-strategy.md:81
  anchor: «  - The diagnostics opens (`src/obs.rs`): a `diagnostics/` folder or a role or detail file that fails `strict::check_diagnostics` is a failed open, checked before `ensure_private_dir` narrows anything, so the writer ends at `std::io::sink` and a detail line is dropped (`open_role_file_refuses_a_widened_diagnostics_path_and_leaves_it`, `open_role_file_refuses_a_symlinked_role_file`, `write_detail_refuses_a_widened_diagnostics_folder_and_leaves_it`).»
  add: «  - The bounded readers' cap-driving cases, the witnesses of five `BOUNDARIES` rows in `tests/contract_bounded_readers.rs`: `statusline_shell_out_past_the_frame_cap_keeps_exactly_the_cap` and `statusline_shell_out_a_command_the_cut_ends_is_a_failed_command` (`src/cmd/hook/statusline.rs`), `drain_keeps_cap_bytes_of_an_endless_pipe` (`src/run/version_gate.rs`), `read_thresholds_keeps_no_key_past_the_frame_cap` (`src/run/budget.rs`), `read_hold_keeps_no_key_past_the_frame_cap` (`src/run/dialog.rs`) and `env_keep_keeps_no_key_past_the_frame_cap` (`src/run/env.rs`).»
- after: .andromeda/test-plan-detail/unit-test-strategy.md:7
  anchor: «»
  next: «- **viola-channel:**»
  add: «- **viola-core:**»
  add: «  - `viola_core::is_code_word` accepts 1 to 64 bytes of `[a-z0-9-]` with a lower-case letter first, held by `code_word_literal_table` (`crates/viola-core/src/lib.rs`).»
- line: .andromeda/test-plan-detail/integration-test-strategy.md:59
  old: «exits non-zero with one `--json` error document before connecting.»
  new: «exits 2 before connecting, with the fixed `UNUSABLE_RESPONSE` line (`src/cmd/answer.rs`) on stderr whatever `--json` says and no document (named in `NOT_ERROR_BODIES`, `tests/contract_error_bodies.rs`).»
- after: .andromeda/test-plan-detail/integration-test-strategy.md:57
  anchor: «  - the MCP widened-instance case `handlers_frame_tools_refuse_a_widened_instance_before_any_frame` (`crates/viola-mcp/tests/handlers.rs`) drives the MCP tools over stdio after an `initialize` handshake: with the instance directory widened and its recorded wrapper serving, MCP `send`, `wait`, `last` and `answer` each get `isError` with `{"error":"instance-unreachable","detail":null}` before any snapshot read or connection; no frame reaches the wrapper, no line is written and the widening is left as found.»
  add: «  - the MCP screen case `handlers_a_refusal_of_free_text_reads_as_unknown_with_no_detail` (`crates/viola-mcp/tests/handlers.rs`) holds `of_reply`'s screen against a test wrapper: a `refusal` of free text reads `unknown`, with no detail.»
- after: .andromeda/test-plan-detail/integration-test-strategy.md:89
  anchor: «  - `tests/cli_links.rs` pins the link verbs' outputs and refusals: `link_verbs_print_the_design_lines` (the three result lines byte for byte, and `{"v":1,"ok":…}` under `--json`), `link_verbs_against_a_stopped_instance_exit_21` (exit 21, the not-running hint last) and `link_verbs_log_the_channel_pair_and_no_names` (the wrapper's `channel-request` / `channel-response` pair carrying `method` `link` / `unlink`, and no driver or driven name in the wrapper's process log).»
  add: «    - A `channel-response` line's `detail` is one of `unsupported-version` · `invalid-params` · `release-from-driver` and absent on every other such line, on both the server's and the client's line at level WARN, with no name or param on the line, held by `link_verbs_log_the_channel_pair_and_no_names` and `schemas/diag-line.v1.json`.»

## Patterns
[[pattern]]
id = "tp2-01"
fixed = 'exits non-zero with one `--json` error document'
