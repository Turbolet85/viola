# Apply — architecture · part 1 of 2

## Verdicts
- fanout-architecture--cascade.md 13 · applied · .andromeda/architecture.md:98#1
- fanout-architecture.md 6 · folded into fanout-architecture--cascade.md 13
- fanout-architecture--cascade.md 15 · applied · .andromeda/architecture.md:227#1
- fanout-architecture.md 5 · folded into fanout-architecture--cascade.md 15
- fanout-architecture.md 1 · applied · .andromeda/architecture.md:65#1
- fanout-architecture--cascade.md 6 · folded into fanout-architecture.md 1
- fanout-architecture.md 2 · applied · .andromeda/architecture.md:106#1, .andromeda/architecture.md:106#2
- fanout-architecture--cascade.md 7 · folded into fanout-architecture.md 2
- fanout-architecture.md 3 · applied · .andromeda/architecture.md:449#1
- fanout-architecture--cascade.md 8 · folded into fanout-architecture.md 3
- fanout-architecture--cascade-new-cross-cutting-patterns.md 1 · applied · .andromeda/architecture-detail/cross-cutting-patterns.md:29+#1, .andromeda/architecture.md:462#1
- fanout-architecture--cascade-new-cross-cutting-patterns.md 2 · applied · .andromeda/architecture-detail/cross-cutting-patterns.md:29+#2
- fanout-architecture--cascade-new-cross-cutting-patterns.md 3 · applied · .andromeda/architecture-detail/cross-cutting-patterns.md:29+#3
- fanout-architecture--cascade-new-cross-cutting-patterns.md 4 · applied · .andromeda/architecture-detail/cross-cutting-patterns.md:29+#4
- fanout-architecture--cascade-new-standard-contracts.md 1 · applied · .andromeda/architecture-detail/standard-contracts.md:83+#1
- fanout-architecture--cascade.md 16 · applied · .andromeda/architecture-detail/standard-contracts.md:111+#1
- fanout-architecture--cascade.md 17 · applied · .andromeda/architecture-detail/standard-contracts.md:111+#2
- fanout-architecture--cascade.md 1 · applied · .andromeda/architecture-detail/established-decisions.md:130#1
- fanout-architecture--cascade.md 2 · applied · .andromeda/architecture-detail/established-decisions.md:130+#1
- fanout-architecture--cascade.md 3 · applied · .andromeda/architecture-detail/established-decisions.md:267#1, .andromeda/architecture-detail/established-decisions.md:267#2
- fanout-architecture--cascade.md 4 · applied · .andromeda/architecture-detail/established-decisions.md:267+#1
- fanout-architecture--cascade.md 5 · applied · .andromeda/architecture-detail/established-decisions.md:267+#2

## Edits
- line: .andromeda/architecture.md:98
  old: «each `detail` from a closed set per reason, extended only in `viola-core`, an unknown one opaque text to a reader»
  new: «each `detail` from a closed set per reason (`unknown`'s a code word or null), extended only in `viola-core`, an unrecognised code word opaque text to a reader»
- line: .andromeda/architecture.md:227
  old: «"data":<object|null>}}` (exit 20)»
  new: «"data":<object|null>}}` (exit 20): `message` a fixed text and `data` a closed object or null, read through `viola_channel::sanitised_fault`»
- line: .andromeda/architecture.md:65
  old: «and three open divergences, `viola-agent-claude`'s `Refusal`, `viola-mcp`'s `ServeError` and `viola-state`'s `StampsError`, the rule standing»
  new: «and two open divergences, `viola-mcp`'s `ServeError` and `viola-state`'s `StampsError`, with `PinError`, `StampsReadError` and `Refused` the three the census adds, the rule standing»
- line: .andromeda/architecture.md:106
  old: «`AgentError`, `McpError`, `CoreError`)»
  new: «`AgentError`, `McpError`)»
- line: .andromeda/architecture.md:106
  old: «; `viola-agent-claude`'s pre-existing `Refusal` beside `AgentError`, `viola-mcp`'s `ServeError` beside `McpError` and `viola-state`'s `StampsError` beside `StateError` are the three open divergences named there)»
  new: «; `viola-mcp`'s `ServeError` beside `McpError` and `viola-state`'s `StampsError` beside `StateError` are the two open divergences named there, with `PinError`, `StampsReadError` and `Refused` the three the census adds, nine `thiserror` derives over the product sources in all, by `command grep -rn "thiserror::Error" src crates --include=*.rs`)»
- line: .andromeda/architecture.md:449
  old: «`viola-agent-claude`'s pre-existing `Refusal` beside `AgentError` and `viola-mcp`'s `ServeError` beside `McpError` are open divergences, [Error Handling])»
  new: «`viola-mcp`'s `ServeError` beside `McpError` and `viola-state`'s `StampsError` beside `StateError` are open divergences, with `PinError`, `StampsReadError` and `Refused` the three the census adds, [Error Handling])»
- line: .andromeda/architecture.md:462
  old: «crosscut-bounded-readers · infra-crate-dependency-direction»
  new: «crosscut-bounded-readers · crosscut-sanitised-error-surfaces · infra-crate-dependency-direction»
- after: .andromeda/architecture-detail/cross-cutting-patterns.md:29
  anchor: «<!-- /detail -->»
  add: «»
  add: «<!-- detail: procedure · crosscut-sanitised-error-surfaces -->»
  add: «»
  add: «- **Sanitised error surfaces.** CLI `--json` documents, MCP error results and channel error data carry codes and fixed messages only, no path and no upstream text; a peer's `message`, `data`, `refusal` or `detail` reaches no document or text but as a closed value, and a code word outside the closed list stays visible by design.»
- after: .andromeda/architecture-detail/cross-cutting-patterns.md:29
  anchor: «<!-- /detail -->»
  add: «  - The E2E contract binary `tests/contract_error_bodies.rs` (run under `run --e2e`, carrying `andromeda:walks-tree`) holds it: its tables `CLI_DOCUMENTS`, `MCP_RESULTS` and `CHANNEL_FAULTS` enumerate the error bodies, and `error_bodies_each_name_a_writer_and_a_witness_that_stand` checks that each body's writer and witness stand; `NOT_ERROR_BODIES` names what is no error body, and `PLUMBING` the census lines that build or route a document a row names, so a row stays one body. A census, `product_error_writers_are_each_named`, covers the product's error writers, and `thiserror_messages_interpolate_nothing` holds that a `thiserror` message interpolates nothing.»
- after: .andromeda/architecture-detail/cross-cutting-patterns.md:29
  anchor: «<!-- /detail -->»
  add: «  - Three surface witnesses read the bodies: `cli_error_documents_hold_no_path_or_upstream_text` in `tests/cli_error_bodies.rs`, `channel_error_bodies_hold_no_path_or_upstream_text` in `tests/channel_error_bodies.rs` and `mcp_tools_error_results_hold_no_path_or_upstream_text` in `crates/viola-e2e/tests/mcp_tools.rs`. The two root binaries' `[[test]]` tables require the `fake-agent` feature, and both sit in the `ci` profile's kill-line filter.»
- after: .andromeda/architecture-detail/cross-cutting-patterns.md:29
  anchor: «<!-- /detail -->»
  add: «  - No `-32603` is driven in a scanned home, and no failpoint or counted panic exists for it: the unit layer holds it, through `answer_a_panicking_dispatch_is_internal_error_and_closes` and `links_set_method_an_append_that_fails_is_internal_with_the_set_unchanged`.»
  add: «»
  add: «<!-- /detail -->»
- after: .andromeda/architecture-detail/standard-contracts.md:83
  anchor: «<!-- /detail -->»
  add: «»
  add: «<!-- detail: values · contract-channel-methods -->»
  add: «»
  add: «- `viola_channel::ProtocolError::cause(self) -> Option<&'static str>` reads `unsupported-version`, `invalid-params` or `release-from-driver` for the three `-32602` faults and `None` for the other four.»
  add: «»
  add: «<!-- /detail -->»
- after: .andromeda/architecture-detail/standard-contracts.md:111
  anchor: «- `viola send`, `viola wait`, `viola last`, `viola answer`, `viola pause` and `viola release` print the same `{"v":1,"error":"instance-unreachable","detail":null}` on stdout under `--json` (exit 21) for their refused strict-modes check of the home and the target instance, made by `client::refuse_untrusted` before anything under the home is opened, with stderr empty and no line in any log.»
  add: «- `viola_channel::sanitised_fault(&Value) -> Option<Value>` is the one fault reader, of `reply_of` and `of_reply`: `None` without an integer `code`; for each of the five codes `message` is the fault's fixed text; `-32602` with an integer `data.supported` and a version-text `data.wrapper` (1 to 64 ASCII letters, digits, `.`, `-` or `+`) reads the unsupported-version body with those two values, `data.reason` exactly `release-from-driver` reads that body, and any other `-32602` reads `invalid params` with `data` null; every other known code reads `data` null.»
- after: .andromeda/architecture-detail/standard-contracts.md:111
  anchor: «- `viola send`, `viola wait`, `viola last`, `viola answer`, `viola pause` and `viola release` print the same `{"v":1,"error":"instance-unreachable","detail":null}` on stdout under `--json` (exit 21) for their refused strict-modes check of the home and the target instance, made by `client::refuse_untrusted` before anything under the home is opened, with stderr empty and no line in any log.»
  add: «- A fault code outside the five reads `{"code": <code>, "message": "wrapper fault", "data": null}`: the literal `wrapper fault` is a fixed output word on `--json` and MCP, and the human line reads `error: wrapper fault  <code>`.»
- line: .andromeda/architecture-detail/established-decisions.md:130
  old: «or `ServeError::Initialize` (exit 11) when the cut lands»
  new: «or `ServeError::Initialize` (exit 1) when the cut lands»
- after: .andromeda/architecture-detail/established-decisions.md:130
  anchor: «- After a session a cut ended, `viola_mcp::serve` writes exactly one `parse-rejected{parser:"mcp-stdin", detail:"oversize", count:1}` at WARN through `obs_event!`, holding no byte of the line, then returns what the session gave: `Ok(())` once the session initialized (exit 0, as at a closed stdin), or `ServeError::Initialize` (exit 11) when the cut lands before `initialize` completes.»
  add: «- `viola mcp` on a line past `MAX_FRAME` before `initialize` exits 1 with both streams empty and one `parse-rejected{mcp-stdin, oversize, count:1}` line (`mcp_tools_a_line_past_max_frame_before_initialize_exits_1_with_one_line`): `src/main.rs`'s `exit_code` gives `Role::Mcp` with `Failed` exit 1, and 11 is only the unit child's own code (`serve_child_entry`).»
- line: .andromeda/architecture-detail/established-decisions.md:267
  old: «-and-harness. One open divergence, not an exception: the crate also holds the thiserror enum `Refusal` (`BatchScriptChild`, `NotFound`, the `resolve_program` refusal), which predates that fold; the rule stands, and its fold or ratification is open). A second open divergence, not an exception: `viola-mcp` holds»
  new: «-and-harness). A first open divergence, not an exception: `viola-mcp` holds»
- line: .andromeda/architecture-detail/established-decisions.md:267
  old: «A third open divergence, not an exception: `viola-state` holds `StampsError` (`StrictModes`, `State`; fixed messages, returned by `update_stamps`) beside `StateError`; the rule stands, and its fold or ratification is open; anyhow 1.0.104»
  new: «A second: `viola-state` holds `StampsError` (`StrictModes`, `State`; fixed messages, returned by `update_stamps`) beside `StateError`, the same. Three more the census below adds, the same: `PinError`, `StampsReadError` and `Refused`. The owner of all five: a CARRY on the working-route entry "Version done-check" (PROVISIONAL); anyhow 1.0.104»
- after: .andromeda/architecture-detail/established-decisions.md:267
  anchor: «- **[Error Handling] thiserror 2.0.20, one enum per workspace crate (one ratified exception: `viola-pty`'s `PtyError` has a hand-written `Display`/`Error` with fixed messages, because the seam's normal dependencies are exactly portable-pty, tracing, windows-sys and libc; portable-pty's anyhow error is kept only as a boxed source. `viola-agent-claude`'s `AgentError` sits at the crate root and holds the stamps envelope's `StampsMalformed` beside the hook payload's `Malformed` — the second enum `StampError` was folded into it by chunk 2026-09-29-verify-stamped-test-homes-and-harness. One open divergence, not an exception: the crate also holds the thiserror enum `Refusal` (`BatchScriptChild`, `NotFound`, the `resolve_program` refusal), which predates that fold; the rule stands, and its fold or ratification is open). A second open divergence, not an exception: `viola-mcp` holds `ServeError` (`Runtime`, `Initialize`, `Session`; fixed messages, returned by `serve`) beside `McpError` (`Internal`, Display `internal error`) (chunk 2026-10-17-mcp-server-for-drivers); the rule stands, and its fold or ratification is open. A third open divergence, not an exception: `viola-state` holds `StampsError` (`StrictModes`, `State`; fixed messages, returned by `update_stamps`) beside `StateError`; the rule stands, and its fold or ratification is open; anyhow 1.0.104 only in the `viola` bin: its `main`, dispatch (`cmd::dispatch` returns the error with the resolved home and instance) and the catch-site reporter `viola::obs::report_internal_error`.** `RefusalReason` lives in `viola-core`. Rationale: callers branch on refusal variants across three surfaces (channel, CLI exit codes, MCP `isError`). anyhow's context chains are only useful where a person reads them: the owner's post-mortem, which gets the chain as `chain` in `instances/<name>/diagnostics/detail-<role>.ndjson`, never a role line, stdout or stderr.»
  add: «- `AgentError` holds `resolve_program`'s two refusals, `BatchScriptChild` and `ProgramNotFound`, each with a fixed message: `resolve_program` returns `Result<PathBuf, AgentError>`, in the crate and in the root wrapper `src/run/mod.rs`.»
- after: .andromeda/architecture-detail/established-decisions.md:267
  anchor: «- **[Error Handling] thiserror 2.0.20, one enum per workspace crate (one ratified exception: `viola-pty`'s `PtyError` has a hand-written `Display`/`Error` with fixed messages, because the seam's normal dependencies are exactly portable-pty, tracing, windows-sys and libc; portable-pty's anyhow error is kept only as a boxed source. `viola-agent-claude`'s `AgentError` sits at the crate root and holds the stamps envelope's `StampsMalformed` beside the hook payload's `Malformed` — the second enum `StampError` was folded into it by chunk 2026-09-29-verify-stamped-test-homes-and-harness. One open divergence, not an exception: the crate also holds the thiserror enum `Refusal` (`BatchScriptChild`, `NotFound`, the `resolve_program` refusal), which predates that fold; the rule stands, and its fold or ratification is open). A second open divergence, not an exception: `viola-mcp` holds `ServeError` (`Runtime`, `Initialize`, `Session`; fixed messages, returned by `serve`) beside `McpError` (`Internal`, Display `internal error`) (chunk 2026-10-17-mcp-server-for-drivers); the rule stands, and its fold or ratification is open. A third open divergence, not an exception: `viola-state` holds `StampsError` (`StrictModes`, `State`; fixed messages, returned by `update_stamps`) beside `StateError`; the rule stands, and its fold or ratification is open; anyhow 1.0.104 only in the `viola` bin: its `main`, dispatch (`cmd::dispatch` returns the error with the resolved home and instance) and the catch-site reporter `viola::obs::report_internal_error`.** `RefusalReason` lives in `viola-core`. Rationale: callers branch on refusal variants across three surfaces (channel, CLI exit codes, MCP `isError`). anyhow's context chains are only useful where a person reads them: the owner's post-mortem, which gets the chain as `chain` in `instances/<name>/diagnostics/detail-<role>.ndjson`, never a role line, stdout or stderr.»
  add: «- The product sources hold 9 `thiserror::Error` derives, the census of [Error Handling]: `AgentError`, `ChannelError`, `StateError`, `PinError`, `StampsError`, `StampsReadError`, `Refused`, `McpError`, `ServeError` (by `command grep -rn "thiserror::Error" src crates --include=*.rs`).»

## Patterns
[[pattern]]
id = "arch1-01"
fixed = 'an unknown one opaque text to a reader'

[[pattern]]
id = "arch1-02"
fixed = 'and three open divergences'

[[pattern]]
id = "arch1-03"
fixed = '`McpError`, `CoreError`)'

[[pattern]]
id = "arch1-04"
fixed = 'the three open divergences named there'

[[pattern]]
id = "arch1-05"
fixed = 'pre-existing `Refusal` beside `AgentError` and'

[[pattern]]
id = "arch1-06"
fixed = '`ServeError::Initialize` (exit 11)'

[[pattern]]
id = "arch1-07"
fixed = 'the crate also holds the thiserror enum `Refusal`'

[[pattern]]
id = "arch1-08"
fixed = 'A third open divergence, not an exception'
