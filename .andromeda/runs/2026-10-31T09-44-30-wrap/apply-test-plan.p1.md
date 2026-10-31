# Apply — test-plan · part 1 of 3

## Verdicts
- fanout-test-plan--cascade-g1.md 3 · applied · .andromeda/test-plan.md:94#1
- fanout-test-plan--cascade-g1.md 9 · applied · .andromeda/test-plan.md:380#1
- fanout-test-plan--cascade-g1.md 13 · applied · .andromeda/test-plan.md:389+#1
- fanout-test-plan--cascade-g1.md 14 · applied · .andromeda/test-plan.md:390#1
- fanout-test-plan.md 1 · folded into fanout-test-plan--cascade-g1.md 14
- fanout-test-plan--cascade-g1.md 18 · applied · .andromeda/test-plan.md:392#1
- fanout-test-plan--cascade-g4.md 2 · applied · .andromeda/test-plan.md:605#1
- fanout-test-plan.md 3 · folded into fanout-test-plan--cascade-g4.md 2
- fanout-test-plan.md 2 · applied · .andromeda/test-plan.md:623#1
- fanout-test-plan--cascade-g4.md 13 · folded into fanout-test-plan.md 2
- fanout-test-plan--cascade-g1.md 1 · applied · .andromeda/test-plan-detail/test-scope-summary.md:12+#1
- fanout-test-plan--cascade-g1.md 2 · applied · .andromeda/test-plan-detail/test-scope-summary.md:44#1
- fanout-test-plan--cascade-g1.md 4 · applied · .andromeda/test-plan-detail/test-scope-summary.md:162#1
- cascade-compare-test-plan.md 2 · applied · .andromeda/test-plan-detail/test-scope-summary.md:63+#1
- cascade-compare-test-plan.md 3 · applied · .andromeda/test-plan-detail/test-scope-summary.md:70+#1
- cascade-compare-test-plan.md 4 · applied · .andromeda/test-plan-detail/test-scope-summary.md:71+#1
- fanout-test-plan--cascade-g1.md 5 · applied · .andromeda/test-plan-detail/test-strategy.md:42#1
- cascade-compare-test-plan.md 5 · applied · .andromeda/test-plan-detail/test-strategy.md:73#1

## Edits
- line: .andromeda/test-plan.md:94
  old: «the exit code plus one JSON document on stdout,»
  new: «the exit code plus one JSON document on stdout (the exit-2 unusable paths of `answer` and `send` excepted: a fixed stderr line and no document),»
- line: .andromeda/test-plan.md:380
  old: «the JSON-RPC error mapping,»
  new: «the JSON-RPC error mapping, the fault screen `sanitised_fault` and `ProtocolError::cause`,»
- after: .andromeda/test-plan.md:389
  anchor: «  - the stdin line cap: a line of exactly `MAX_FRAME` bytes, its newline counted, passes whole, and one byte more passes no byte and reads as end of stream across any split of the reads; `serve` ends the session at a line past the cap»
  add: «  - the reply screen: a `refusal` that is no code word reads `unknown`, a string `detail` that is no code word reads `null`, and a peer fault reads through `viola_channel::sanitised_fault`»
- line: .andromeda/test-plan.md:390
  old: «not driven by any test: a stdin line past `MAX_FRAME` landing before `initialize` completes ends `serve` with `ServeError::Initialize` (exit 11), as a closed stdin before `initialize` does; owner: the route entry "Sanitised error surfaces", as a PREREQ»
  new: «a stdin line past `MAX_FRAME` landing before `initialize` completes ends `serve` with `ServeError::Initialize`, as a closed stdin before `initialize` does; 11 is the unit child's own code, and `viola mcp` itself exits 1, both streams empty, with one `parse-rejected` `mcp-stdin` / `oversize` line of count 1 (unit `serve_at_a_line_past_max_frame_before_initialize_is_the_initialize_error`, E2E `mcp_tools_a_line_past_max_frame_before_initialize_exits_1_with_one_line`)»
- line: .andromeda/test-plan.md:392
  old: «(an unknown refusal maps to exit 14, never a panic and never 20),»
  new: «(an unknown refusal maps to exit 14, never a panic and never 20), the reply screen (a `refusal` that is no code word reads `unknown`, a string `detail` that is no code word reads null, a peer fault reads through `sanitised_fault`),»
- line: .andromeda/test-plan.md:605
  old: «after every E2E test, no absolute path, serde path or `Caused by` in any error payload, every `-32603` exactly `"internal error"` / `null`, no `Cookie` header or stripped-variable value in any log, `diagnostics/` files 0600 on Unix, and the fixed canary in no home-level diagnostics line; a hit is reported by place and class, never the matched bytes.»
  new: «no helper runs after each E2E test: none is built, and the three surface witnesses (channel, CLI `--json`, MCP), the enumeration and the census of writers (`tests/contract_error_bodies.rs`) replace it, reading the check in `tests/support/events.rs`. The rules: no absolute path, serde path or `Caused by` in any error payload, every `-32603` exactly `"internal error"` / `null` (held at the unit layer, no scanned home driving one), no `Cookie` header or stripped-variable value in any log, `diagnostics/` files 0600 on Unix, and the fixed canary in no home-level diagnostics line; a hit is reported by place and class, never the matched bytes. Their holders: the no-`Cookie`, stripped-value and canary clauses are `viola-harness secret-scan`'s, after each job's end-to-end run (`v1-47`'s reading, the delegate overseer's word, PROVISIONAL), the 0600 clause `write_detail_refuses_a_widened_diagnostics_folder_and_leaves_it`'s (`v1-47`'s acceptance; `crates/viola-e2e/src/harness/secret_scan.rs:129` reads `cookie:` whatever its case).»
- line: .andromeda/test-plan.md:623
  old: «Owed: the enumeration's rows for the status-line command's stdout, the child pipe drain shared by `claude --version` and `claude agents --json`, and the `config.json` readers of the thresholds, the dialog hold and `claude_env_keep` name a witness that reads the bounded read without driving past its cap, so their caps are held by the row's needle and the census alone; owner: the route entry "Sanitised error surfaces", as a PREREQ (a cap-driving case for each row).»
  new: «As landed: the enumeration's five rows for the status-line command's stdout, the child pipe drain shared by `claude --version` and `claude agents --json`, and the `config.json` readers of the thresholds, the dialog hold and `claude_env_keep` each name a cap-driving case (`statusline_shell_out_past_the_frame_cap_keeps_exactly_the_cap`, `statusline_shell_out_a_command_the_cut_ends_is_a_failed_command`, `drain_keeps_cap_bytes_of_an_endless_pipe`, `read_thresholds_keeps_no_key_past_the_frame_cap`, `read_hold_keeps_no_key_past_the_frame_cap`, `env_keep_keeps_no_key_past_the_frame_cap`).»
- after: .andromeda/test-plan-detail/test-scope-summary.md:12
  anchor: «  - **As landed (server verification):** the pid/start-time server verification is asserted on Linux and Windows: the eight CLI frame verbs (`send`, `wait`, `last`, `answer`, `pause`, `release`, `link`, `unlink`) by `tests/security_negatives_server_identity.rs` (an impostor server gets no frame and the verb exits 21, beside a control that reaches the recorded wrapper; §6 Security control negatives), the hook by `tests/hook_fail_open.rs`'s impostor rows (exit 0, no body, no frame), the MCP frame tools by `crates/viola-mcp/tests/handlers.rs` (`instance-unreachable`, no frame), and the decision table by the `viola-state` unit `server_verdict_rows`. These negatives compile out on macOS, where the client reads no peer pid; the macOS cases are owed (§6 Exit-cause matrix).»
  add: «  - **As landed (the fault screen):** `viola_channel::sanitised_fault` reads a peer's error for the two clients, `reply_of` and `of_reply`: each of the five known codes takes its fixed `message`; `-32602` keeps only an integer `data.supported` with a version-text `data.wrapper`, or a `data.reason` exactly `release-from-driver`, any other `-32602` reading `invalid params` with `data: null`; every other known code reads `data: null`; a code outside the five reads `wrapper fault` with `data: null`; an error without an integer `code` reads nothing (units `sanitised_fault_keeps_each_wire_body`, `sanitised_fault_carries_nothing_of_the_peer_but_closed_values`, `sanitised_fault_without_an_integer_code_is_nothing`). `ProtocolError::cause` gives `channel-response`'s `detail` for the three `-32602` faults and nothing for the other four (`protocol_error_cause_tells_the_three_invalid_params_apart`; its words at the wrapper-channel surface's **As landed (the error bodies)**, below).»
- line: .andromeda/test-plan-detail/test-scope-summary.md:44
  old: «`{"v":1,"error":…}`).»
  new: «`{"v":1,"error":…}`). The exception: the exit-2 paths of `viola answer` (`UNUSABLE_RESPONSE`) and `viola send` (`UNUSABLE_TEXT`) write that fixed line to stderr whatever `--json` says, with no document (named in `NOT_ERROR_BODIES`, `tests/contract_error_bodies.rs`).»
- after: .andromeda/test-plan-detail/test-scope-summary.md:63
  anchor: «  - **As landed (`link` / `unlink`):** both are served by the driven instance's wrapper; a missing, non-string or invalid name, a `driven` that is not the serving wrapper's own name, or a `driver` equal to `driven` is `-32602` with `data: null` and no line written, and a failed `record` / `remove` is `-32603`; `link` answers `ok` `{since, already_linked}` and `unlink` an empty `ok` object.»
  add: «  - **As landed (the error bodies):** the seven `ProtocolError` wire bodies stand literal with `v` 1, a too-new request answered `-32602` with the wrapper's version (`protocol_error_body_is_the_literal_wire_object`, `channel_error_bodies_hold_no_path_or_upstream_text`). `channel-response`'s `detail`, on the server's and the client's line at WARN, is closed to `unsupported-version` · `invalid-params` · `release-from-driver` for the three `-32602` faults and absent on every other line, with no name or param on it (`answer_invalid_params_names_its_cause_and_no_param`, `answer_faults_other_than_invalid_params_carry_no_detail`, `reply_from_logs_the_cause_of_each_invalid_params`). No `-32603` is driven in a scanned home; the unit layer holds it (`answer_a_panicking_dispatch_is_internal_error_and_closes`, `links_set_method_an_append_that_fails_is_internal_with_the_set_unchanged`).»
- after: .andromeda/test-plan-detail/test-scope-summary.md:70
  anchor: «  - **As landed (the instance check):** the frame tools `send`, `wait`, `last` and `answer` run `check_instance` before the liveness pre-check; a refused instance returns `isError:true` `{"error":"instance-unreachable","detail":null}` with no frame and no line (`crates/viola-mcp/tests/handlers.rs` `handlers_frame_tools_refuse_a_widened_instance_before_any_frame`).»
  add: «  - **As landed (a peer's reply):** `of_reply` (`crates/viola-mcp/src/result.rs`) applies the screen the CLI's `reply_of` does: a `refusal` that is no code word reads `unknown`, a string `detail` that is no code word reads `null`, and a non-string `detail` is no known reply. `wrapper-fault`'s `detail` is `viola_channel::sanitised_fault`'s (the `viola-channel` entity's **As landed (the fault screen)**, above) (`of_reply_maps_each_reply_shape`, `handlers_a_refusal_of_free_text_reads_as_unknown_with_no_detail`, `mcp_tools_error_results_hold_no_path_or_upstream_text`).»
- after: .andromeda/test-plan-detail/test-scope-summary.md:71
  anchor: «  - **As landed (the stdin line cap):** `viola mcp` reads its stdin through a line reader capped at `MAX_FRAME` (`CappedLines`); a line past the cap passes no byte, is never truncated or skipped, and ends the session as a stdin close does, with one `parse-rejected{parser:"mcp-stdin", detail:"oversize", count:1}` WARN line holding no byte of it; on an initialized session the earlier lines are answered, and `viola mcp` exits 0 with stderr empty and stdout only the earlier frames (`crates/viola-e2e/tests/mcp_tools.rs` `mcp_tools_a_line_past_max_frame_ends_the_session_with_one_line`).»
  add: «  - **As landed (the line before `initialize`):** a stdin line past `MAX_FRAME` before `initialize` completes ends `viola mcp` with exit 1, both streams empty and one `parse-rejected{parser:"mcp-stdin", detail:"oversize", count:1}` line (`crates/viola-e2e/tests/mcp_tools.rs` `mcp_tools_a_line_past_max_frame_before_initialize_exits_1_with_one_line`); `serve` returns its initialize error (`serve_at_a_line_past_max_frame_before_initialize_is_the_initialize_error`), whose 11 is the unit child's own code and not the process's exit.»
- line: .andromeda/test-plan-detail/test-scope-summary.md:162
  old: «(empty on failure), exit 0.»
  new: «(empty on failure), exit 0. The shell-out keeps exactly the frame cap of a longer stdout, and a command the cut ends is a failed command (unit `statusline_shell_out_past_the_frame_cap_keeps_exactly_the_cap`, `statusline_shell_out_a_command_the_cut_ends_is_a_failed_command`).»
- line: .andromeda/test-plan-detail/test-strategy.md:42
  old: «and the `boot` that waits for the session-start record;»
  new: «and the `boot` that waits for the session-start record, and the one error-body check, `error_body_hit` and `assert_error_body_clean` (classes `absolute-path` · `caused-by` · `serde-path` · `planted`, reporting place and class, never bytes);»
- line: .andromeda/test-plan-detail/test-strategy.md:73
  old: «§6 Error sanitization and secret scan;»
  new: «§6 Error sanitization and secret scan (the three surface witnesses, root `cli_error_bodies.rs` `cli_error_documents_hold_no_path_or_upstream_text`, root `channel_error_bodies.rs` `channel_error_bodies_hold_no_path_or_upstream_text` and `crates/viola-e2e/tests/mcp_tools.rs` `mcp_tools_error_results_hold_no_path_or_upstream_text`; root `contract_error_bodies.rs`, the enumeration and the census of writers);»

## Patterns
[[pattern]]
id = "tp1-01"
fixed = '`ServeError::Initialize` (exit 11)'

[[pattern]]
id = "tp1-02"
fixed = 'after every E2E test'

[[pattern]]
id = "tp1-03"
fixed = 'name a witness that reads the bounded read without driving past its cap'
