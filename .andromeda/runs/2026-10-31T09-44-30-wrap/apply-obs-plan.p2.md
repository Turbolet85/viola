# Apply — obs-plan · part 2 of 2

## Verdicts
- fanout-obs-plan--cascade-g3.md 4 · applied · .andromeda/obs-plan-detail/error-capture-reporting.md:17+#1
- fanout-obs-plan--cascade-g3.md 5 · applied · .andromeda/obs-plan-detail/error-capture-reporting.md:39#1
- fanout-obs-plan--cascade-g3.md 6 · applied · .andromeda/obs-plan-detail/error-capture-reporting.md:39+#1
- fanout-obs-plan--cascade-new-error-capture-reporting.md 1 · applied · .andromeda/obs-plan-detail/error-capture-reporting.md:41+#1
- fanout-obs-plan--cascade-new-error-capture-reporting.md 2 · applied · .andromeda/obs-plan-detail/error-capture-reporting.md:41+#2
- fanout-obs-plan--cascade-new-error-capture-reporting.md 3 · applied · .andromeda/obs-plan-detail/error-capture-reporting.md:41+#3
- fanout-obs-plan--cascade-new-error-capture-reporting.md 4 · applied · .andromeda/obs-plan-detail/error-capture-reporting.md:41+#4
- fanout-obs-plan--cascade-new-error-capture-reporting.md 5 · applied · .andromeda/obs-plan-detail/error-capture-reporting.md:18+#1
- fanout-obs-plan--cascade-g4.md 1 · applied · .andromeda/obs-plan-detail/pii-scrubbing-compliance.md:40+#1
- fanout-obs-plan--cascade-g4.md 2 · applied · .andromeda/obs-plan-detail/pii-scrubbing-compliance.md:40+#2
- fanout-obs-plan--cascade-g4.md 3 · applied · .andromeda/obs-plan-detail/pii-scrubbing-compliance.md:40+#3
- cascade-compare-obs-plan.md 5 · applied · .andromeda/obs-plan-detail/pii-scrubbing-compliance.md:22#1
- fanout-obs-plan--cascade-g4.md 4 · applied · .andromeda/obs-plan-detail/ci-integration.md:61+#1

## Edits
- after: .andromeda/obs-plan-detail/error-capture-reporting.md:17
  anchor: «  - **Catch-site reach:** on Unix a home under a regular file is a strict-modes refusal with no line — `viola verify` exits 1 with the home pair, the frame verbs exit 21 with the instance pair — so it never reaches the `cli` catch site; on Windows such a path stats `NotFound`, the check skips it as absent, and the catch-site line `process-exit{detail:"internal-error"}` stands.»
  add: «  - **`mcp` at the catch site:** the `mcp` role writes no self `process-exit` line: `internal_error_exit_line` (`src/obs.rs`) writes for `Run` and `Cli` only.»
- line: .andromeda/obs-plan-detail/error-capture-reporting.md:39
  old: «with `process-exit` 0 once `initialize` has completed, or `ServeError::Initialize` (exit 11) when the cut lands before it;»
  new: «with process exit 0 and no self `process-exit` line once `initialize` has completed, or process exit 1 when the cut lands before it (`ServeError::Initialize`; `src/main.rs` `exit_code` gives `Role::Mcp` + `Failed` exit 1, and 11 is the code of the unit child alone, `serve_child_entry`);»
- after: .andromeda/obs-plan-detail/error-capture-reporting.md:39
  anchor: «- A `viola mcp` stdin line past `MAX_FRAME` is no tool reject: it gives no `isError` result and no rmcp line (the reader signals the cut as end of stream, never as an I/O error, and `rmcp` is `OFF` in the sink); the session ends as at a closed stdin, with `process-exit` 0 once `initialize` has completed, or `ServeError::Initialize` (exit 11) when the cut lands before it; its one record is the §6 `parse-rejected{parser:"mcp-stdin", detail:"oversize"}` line.»
  add: «- The same line before `initialize` leaves both streams empty and its one record reads `parse-rejected{parser:"mcp-stdin", detail:"oversize", count:1}`: `mcp_tools_a_line_past_max_frame_before_initialize_exits_1_with_one_line`.»
- after: .andromeda/obs-plan-detail/error-capture-reporting.md:41
  anchor: «<!-- /detail -->»
  add: «»
  add: «<!-- detail: values · errors -->»
  add: «»
  add: «- `viola_channel::sanitised_fault` is the one reader of a peer's fault, for `reply_of` (`src/cmd/client.rs`) and `of_reply` (`crates/viola-mcp/src/result.rs`). A fault with no integer `code` reads nothing. Each of the five known codes reads its fixed `message`: `-32602` with an integer `data.supported` and a `data.wrapper` of 1 to 64 ASCII letters, digits, `.`, `-` or `+` reads the unsupported-version body with those two values, `data.reason` exactly `release-from-driver` reads that body, any other `-32602` reads `invalid params` with `data` null, and every other known code reads `data` null. A code outside the five reads `message` `wrapper fault` with `data` null.»
- after: .andromeda/obs-plan-detail/error-capture-reporting.md:41
  anchor: «<!-- /detail -->»
  add: «- `reply_of` and `of_reply` screen a peer's reply by `viola_core::is_code_word` (1 to 64 bytes of `[a-z0-9-]`, a lower-case letter first): a `refusal` that is no code word reads `unknown`, a string `detail` that is no code word reads null, and a non-string `detail` is no known reply. A code word outside the closed list stays visible in the document and in `refused: <word>` by design, and its exit is `unknown`'s 14 by `reason_of`.»
- after: .andromeda/obs-plan-detail/error-capture-reporting.md:41
  anchor: «<!-- /detail -->»
  add: «- The product's `thiserror::Error` derives read 9: `AgentError`, `ChannelError`, `StateError`, `PinError`, `StampsError`, `StampsReadError`, `Refused`, `McpError`, `ServeError` (as measured at chunk 2026-10-31-sanitised-error-surfaces, a grep of `thiserror::Error` over `src` and `crates`). `AgentError`'s variants include `BatchScriptChild` and `ProgramNotFound`, their two messages fixed.»
- after: .andromeda/obs-plan-detail/error-capture-reporting.md:41
  anchor: «<!-- /detail -->»
  add: «- The exit-2 paths of `viola answer` and `viola send` write the fixed `UNUSABLE_RESPONSE` / `UNUSABLE_TEXT` line to stderr whatever `--json` says, with no document (`src/cmd/answer.rs`, `src/cmd/send.rs`); `NOT_ERROR_BODIES` (`tests/contract_error_bodies.rs`) names them.»
  add: «»
  add: «<!-- /detail -->»
- after: .andromeda/obs-plan-detail/error-capture-reporting.md:18
  anchor: «  - **`run` worker threads:** the PTY pump and handle-wait thread bodies each run inside `catch_unwind`. An `Err` is sent to `main`, which takes the `run` exit path below, because the session cannot continue without either thread. A worker panic that leaves `main` running with no `process-exit` is forbidden. A panic in one channel server connection worker is contained: the `channel.dispatch` `Drop` guard logs `channel-response{result_class:"error", error_code:-32603}`, the connection closes, and `run` continues. The vt100 feed thread (`src/run/gate.rs`, detached) is contained too: its `catch_unwind` wraps each feed and resize, a caught panic poisons the screen model and writes one `parse-rejected`, nothing is sent to `main`, and `run` continues; the ban above covers the PTY pump and the handle-wait thread.»
  add: «  - **`-32603` proof:** no failpoint or counted panic drives a `-32603` in a scanned home; the unit layer holds that fault (`answer_a_panicking_dispatch_is_internal_error_and_closes`, `links_set_method_an_append_that_fails_is_internal_with_the_set_unchanged`).»
- after: .andromeda/obs-plan-detail/pii-scrubbing-compliance.md:40
  anchor: «   - A strict-modes refusal of the six frame verbs (`send`, `wait`, `last`, `answer`, `pause`, `release`, through `client::refuse_untrusted`) carries no pid, path or endpoint and writes no line: it prints only the fixed instance pair, or under `--json` exactly `{"v":1,"error":"instance-unreachable","detail":null}`; `tests/security_negatives_strict_modes.rs` reads every output with `digit_runs` and finds no line, on Linux, macOS and Windows. The `viola mcp` frame tools refused by the same check return `instance-unreachable` with `detail: null` and write no line (`binary(handlers)`).»
  add: «   - **Error-body census:** `tests/contract_error_bodies.rs` (`andromeda:walks-tree`) holds the error bodies of CLI `--json`, MCP results and the seven channel faults in tables: `error_bodies_each_name_a_writer_and_a_witness_that_stand` holds each body to a writer and a witness that stand, `product_error_writers_are_each_named` holds every product error writer to a name, and `thiserror_messages_interpolate_nothing` holds `thiserror` messages free of interpolation.»
- after: .andromeda/obs-plan-detail/pii-scrubbing-compliance.md:40
  anchor: «   - A strict-modes refusal of the six frame verbs (`send`, `wait`, `last`, `answer`, `pause`, `release`, through `client::refuse_untrusted`) carries no pid, path or endpoint and writes no line: it prints only the fixed instance pair, or under `--json` exactly `{"v":1,"error":"instance-unreachable","detail":null}`; `tests/security_negatives_strict_modes.rs` reads every output with `digit_runs` and finds no line, on Linux, macOS and Windows. The `viola mcp` frame tools refused by the same check return `instance-unreachable` with `detail: null` and write no line (`binary(handlers)`).»
  add: «   - **Error-body check:** `tests/support/events.rs` holds `error_body_hit` and `assert_error_body_clean`, classes `absolute-path`, `caused-by`, `serde-path` and `planted`, reporting place and class, never bytes. No check runs after every test: the three live witnesses, the enumeration and the census read it instead. It stands beside the secret scan and leaves the scan as it is.»
- after: .andromeda/obs-plan-detail/pii-scrubbing-compliance.md:40
  anchor: «   - A strict-modes refusal of the six frame verbs (`send`, `wait`, `last`, `answer`, `pause`, `release`, through `client::refuse_untrusted`) carries no pid, path or endpoint and writes no line: it prints only the fixed instance pair, or under `--json` exactly `{"v":1,"error":"instance-unreachable","detail":null}`; `tests/security_negatives_strict_modes.rs` reads every output with `digit_runs` and finds no line, on Linux, macOS and Windows. The `viola mcp` frame tools refused by the same check return `instance-unreachable` with `detail: null` and write no line (`binary(handlers)`).»
  add: «   - **Live witnesses of clean error bodies:** `channel_error_bodies_hold_no_path_or_upstream_text` (`tests/channel_error_bodies.rs`), `cli_error_documents_hold_no_path_or_upstream_text` (`tests/cli_error_bodies.rs`) and `mcp_tools_error_results_hold_no_path_or_upstream_text` (`crates/viola-e2e/tests/mcp_tools.rs`).»
- line: .andromeda/obs-plan-detail/pii-scrubbing-compliance.md:22
  old: «| Closed kebab-case enums |»
  new: «| Closed kebab-case enums; `channel-response`'s `detail` is one of `unsupported-version` · `invalid-params` · `release-from-driver` for the three `-32602` faults, on both the server's and the client's line, absent on every other `channel-response` line, closed by the enum in `schemas/diag-line.v1.json` and naming no param or value; asserted by `answer_invalid_params_names_its_cause_and_no_param`, `reply_from_logs_the_cause_of_each_invalid_params` and `link_verbs_log_the_channel_pair_and_no_names` |»
- after: .andromeda/obs-plan-detail/ci-integration.md:61
  anchor: «<!-- /detail -->»
  add: «»
  add: «<!-- detail: values · ci -->»
  add: «»
  add: «- `v1-47`'s secret-scan reading is after each job's end-to-end run, not after every test: `AGENT_RUN_KEEP_HOMES` is `1` on the job (`.github/workflows/ci.yml`), the scan is `if: always()`, and the `viola-chaos-*` home stays out (step 1 above). OPEN — this reading is PROVISIONAL · owner: the open decision on `v1-47`'s after-every-test reading.»
  add: «»
  add: «<!-- /detail -->»

## Patterns
[[pattern]]
id = "obs2-01"
fixed = '`ServeError::Initialize` (exit 11)'

[[pattern]]
id = "obs2-02"
fixed = 'with `process-exit` 0 once `initialize` has completed'
