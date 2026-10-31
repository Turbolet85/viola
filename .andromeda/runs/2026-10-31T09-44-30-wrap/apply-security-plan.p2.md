# Apply — security-plan · part 2 of 2

## Verdicts
- fanout-security-plan--cascade-g2.md 3 · applied · .andromeda/security-plan-detail/error-handling.md:14+#1
- fanout-security-plan--cascade-g2.md 4 · applied · .andromeda/security-plan-detail/error-handling.md:14+#2
- fanout-security-plan--cascade-g2.md 5 · applied · .andromeda/security-plan-detail/error-handling.md:14+#3
- fanout-security-plan--cascade-g2.md 6 · applied · .andromeda/security-plan-detail/error-handling.md:14+#4
- fanout-security-plan--cascade-g2.md 7 · applied · .andromeda/security-plan-detail/error-handling.md:14+#5
- fanout-security-plan--cascade-g2.md 8 · applied · .andromeda/security-plan-detail/error-handling.md:14+#6
- fanout-security-plan--cascade-g2.md 9 · applied · .andromeda/security-plan-detail/error-handling.md:14+#7
- fanout-security-plan--cascade-g2.md 10 · applied · .andromeda/security-plan-detail/error-handling.md:15+#1
- fanout-security-plan--cascade-g2.md 11 · applied · .andromeda/security-plan-detail/error-handling.md:17+#4
- fanout-security-plan--cascade-g2.md 12 · applied · .andromeda/security-plan-detail/error-handling.md:17+#5
- fanout-security-plan--cascade-g2.md 13 · applied · .andromeda/security-plan-detail/error-handling.md:17+#6
- fanout-security-plan--cascade-g2.md 14 · applied · .andromeda/security-plan-detail/error-handling.md:17+#2
- fanout-security-plan--cascade-g2.md 15 · applied · .andromeda/security-plan-detail/error-handling.md:17+#3
- cascade-compare-security-plan.md 3 · applied · .andromeda/security-plan-detail/error-handling.md:17+#1

## Edits
- after: .andromeda/security-plan-detail/error-handling.md:14
  anchor: «- The wrapper's `link` / `unlink` methods answer a failed `record` / `remove` as `-32603` with the fixed message `internal error` and `data: null`. At `viola link` / `viola unlink`, a wrapper error reply (a `-32602` included) is a wrapper fault, exit 20, and a `Refused` reply or an `ok` of a shape other than the verb's own (`link`: a string `since` and a boolean `already_linked`; `unlink`: an object) is an internal error, exit 1.»
  add: «- `viola_channel::sanitised_fault(&Value) -> Option<Value>` is the fault reader of its two callers, `reply_of` (`src/cmd/client.rs`) and `of_reply` (`crates/viola-mcp/src/result.rs`): `None` without an integer `code`; for each of the five known codes `message` is the fault's fixed text and `data` null, save the two `-32602` bodies that keep `data`.»
- after: .andromeda/security-plan-detail/error-handling.md:14
  anchor: «- The wrapper's `link` / `unlink` methods answer a failed `record` / `remove` as `-32603` with the fixed message `internal error` and `data: null`. At `viola link` / `viola unlink`, a wrapper error reply (a `-32602` included) is a wrapper fault, exit 20, and a `Refused` reply or an `ok` of a shape other than the verb's own (`link`: a string `since` and a boolean `already_linked`; `unlink`: an object) is an internal error, exit 1.»
  add: «- A `-32602` read by `sanitised_fault` keeps `data` in two bodies only: the unsupported-version body, an integer `data.supported` with a `data.wrapper` of 1 to 64 ASCII letters, digits, `.`, `-` or `+`, and the body whose `data.reason` is exactly `release-from-driver`. Any other `-32602` reads `invalid params` with `data` null.»
- after: .andromeda/security-plan-detail/error-handling.md:14
  anchor: «- The wrapper's `link` / `unlink` methods answer a failed `record` / `remove` as `-32603` with the fixed message `internal error` and `data: null`. At `viola link` / `viola unlink`, a wrapper error reply (a `-32602` included) is a wrapper fault, exit 20, and a `Refused` reply or an `ok` of a shape other than the verb's own (`link`: a string `since` and a boolean `already_linked`; `unlink`: an object) is an internal error, exit 1.»
  add: «- A fault code outside the five reads `{"code": <code>, "message": "wrapper fault", "data": null}`, `wrapper fault` a fixed output word on `--json` and MCP; the human line reads `error: wrapper fault  <code>` (`src/human/mirror.rs`).»
- after: .andromeda/security-plan-detail/error-handling.md:14
  anchor: «- The wrapper's `link` / `unlink` methods answer a failed `record` / `remove` as `-32603` with the fixed message `internal error` and `data: null`. At `viola link` / `viola unlink`, a wrapper error reply (a `-32602` included) is a wrapper fault, exit 20, and a `Refused` reply or an `ok` of a shape other than the verb's own (`link`: a string `since` and a boolean `already_linked`; `unlink`: an object) is an internal error, exit 1.»
  add: «- `viola_core::is_code_word` admits 1 to 64 bytes of `[a-z0-9-]`, a lower-case letter first. In `reply_of` and `of_reply`, a `refusal` failing it reads `unknown`, a string `detail` failing it reads null, and a non-string `detail` is no known reply.»
- after: .andromeda/security-plan-detail/error-handling.md:14
  anchor: «- The wrapper's `link` / `unlink` methods answer a failed `record` / `remove` as `-32603` with the fixed message `internal error` and `data: null`. At `viola link` / `viola unlink`, a wrapper error reply (a `-32602` included) is a wrapper fault, exit 20, and a `Refused` reply or an `ok` of a shape other than the verb's own (`link`: a string `since` and a boolean `already_linked`; `unlink`: an object) is an internal error, exit 1.»
  add: «- Exception by design: a code word outside the closed list stays visible in the document and in `refused: <word>`, and its exit is `unknown`'s 14 by `reason_of`.»
- after: .andromeda/security-plan-detail/error-handling.md:14
  anchor: «- The wrapper's `link` / `unlink` methods answer a failed `record` / `remove` as `-32603` with the fixed message `internal error` and `data: null`. At `viola link` / `viola unlink`, a wrapper error reply (a `-32602` included) is a wrapper fault, exit 20, and a `Refused` reply or an `ok` of a shape other than the verb's own (`link`: a string `since` and a boolean `already_linked`; `unlink`: an object) is an internal error, exit 1.»
  add: «- `ProtocolError::cause` gives `unsupported-version`, `invalid-params` and `release-from-driver` for the three `-32602` faults and nothing for the other four; `ResponseLine`'s `Drop` writes it as `channel-response`'s `detail`, a closed enum (`schemas/diag-line.v1.json`), on both the server's and the client's line.»
- after: .andromeda/security-plan-detail/error-handling.md:14
  anchor: «- The wrapper's `link` / `unlink` methods answer a failed `record` / `remove` as `-32603` with the fixed message `internal error` and `data: null`. At `viola link` / `viola unlink`, a wrapper error reply (a `-32602` included) is a wrapper fault, exit 20, and a `Refused` reply or an `ok` of a shape other than the verb's own (`link`: a string `since` and a boolean `already_linked`; `unlink`: an object) is an internal error, exit 1.»
  add: «- Absence: no `-32603` is driven in a scanned home, and no failpoint or counted panic drives one there; the unit layer holds it (`answer_a_panicking_dispatch_is_internal_error_and_closes`, `links_set_method_an_append_that_fails_is_internal_with_the_set_unchanged`).»
- after: .andromeda/security-plan-detail/error-handling.md:15
  anchor: «- CLI `send` · `wait` · `last` · `answer` · `pause` · `release`, as `link` / `unlink`, run the instance's strict-modes check (`client::refuse_untrusted`: `check_instance` on the home and `instances/<name>`) before any frame: a refusal exits 21, prints the instance pair (`human::instance_state_pair`) through `human::refuse`, or under `--json` exactly `{"v":1,"error":"instance-unreachable","detail":null}`, opens nothing under the home and writes no line.»
  add: «- Exception: the exit-2 paths of `viola answer` and `viola send` write the fixed `UNUSABLE_RESPONSE` / `UNUSABLE_TEXT` line to stderr whatever `--json` says, and no document (`src/cmd/answer.rs`, `src/cmd/send.rs`); `NOT_ERROR_BODIES` names them.»
- after: .andromeda/security-plan-detail/error-handling.md:17
  anchor: «- `viola_state::stamps::StampsError` (`StrictModes` · `State`, the error `update_stamps` returns) and `viola_state::pin::PinError::StrictModes` carry fixed `Display` messages with no path or payload: `the capability stamps failed the strict-modes check`, a write failure its own fixed message, and `the pinned copy failed the strict-modes check`; `run` maps `PinError::StrictModes` to the `sideload_fallback` outcome `strict-modes-failed`.»
  add: «- The `thiserror::Error` derives over product sources are nine: `AgentError` · `ChannelError` · `StateError` · `PinError` · `StampsError` · `StampsReadError` · `Refused` · `McpError` · `ServeError`; the census in `tests/contract_error_bodies.rs` reads their messages.»
- after: .andromeda/security-plan-detail/error-handling.md:17
  anchor: «- `viola_state::stamps::StampsError` (`StrictModes` · `State`, the error `update_stamps` returns) and `viola_state::pin::PinError::StrictModes` carry fixed `Display` messages with no path or payload: `the capability stamps failed the strict-modes check`, a write failure its own fixed message, and `the pinned copy failed the strict-modes check`; `run` maps `PinError::StrictModes` to the `sideload_fallback` outcome `strict-modes-failed`.»
  add: «- Open divergences from the error-enum convention: `ServeError`, `StampsError`, `PinError`, `StampsReadError` and `Refused`; owner: a CARRY on the route entry Version done-check, PROVISIONAL, owed the founder's own word.»
- after: .andromeda/security-plan-detail/error-handling.md:17
  anchor: «- `viola_state::stamps::StampsError` (`StrictModes` · `State`, the error `update_stamps` returns) and `viola_state::pin::PinError::StrictModes` carry fixed `Display` messages with no path or payload: `the capability stamps failed the strict-modes check`, a write failure its own fixed message, and `the pinned copy failed the strict-modes check`; `run` maps `PinError::StrictModes` to the `sideload_fallback` outcome `strict-modes-failed`.»
  add: «- `resolve_program` returns `Result<PathBuf, AgentError>` in `viola_agent_claude` and in the root wrapper `src/run/mod.rs`, its refusals `AgentError::BatchScriptChild` and `AgentError::ProgramNotFound`, each a fixed message; `src/cmd/revive.rs`'s private `Refusal` is another type.»
- after: .andromeda/security-plan-detail/error-handling.md:17
  anchor: «- `viola_state::stamps::StampsError` (`StrictModes` · `State`, the error `update_stamps` returns) and `viola_state::pin::PinError::StrictModes` carry fixed `Display` messages with no path or payload: `the capability stamps failed the strict-modes check`, a write failure its own fixed message, and `the pinned copy failed the strict-modes check`; `run` maps `PinError::StrictModes` to the `sideload_fallback` outcome `strict-modes-failed`.»
  add: «- `tests/contract_error_bodies.rs`, carrying `andromeda:walks-tree`, enumerates every error body with its writer and witness (`CLI_DOCUMENTS`, `MCP_RESULTS`, `CHANNEL_FAULTS`, with `NOT_ERROR_BODIES` and `PLUMBING` the named exclusions); `product_error_writers_are_each_named` fails on an unnamed product error writer and `thiserror_messages_interpolate_nothing` on an interpolating `thiserror` message.»
- after: .andromeda/security-plan-detail/error-handling.md:17
  anchor: «- `viola_state::stamps::StampsError` (`StrictModes` · `State`, the error `update_stamps` returns) and `viola_state::pin::PinError::StrictModes` carry fixed `Display` messages with no path or payload: `the capability stamps failed the strict-modes check`, a write failure its own fixed message, and `the pinned copy failed the strict-modes check`; `run` maps `PinError::StrictModes` to the `sideload_fallback` outcome `strict-modes-failed`.»
  add: «- `tests/support/events.rs` holds the error-body check, `error_body_hit` and `assert_error_body_clean`, with the classes `absolute-path` · `caused-by` · `serde-path` · `planted`; it reports place and class, never the bytes it read.»
- after: .andromeda/security-plan-detail/error-handling.md:17
  anchor: «- `viola_state::stamps::StampsError` (`StrictModes` · `State`, the error `update_stamps` returns) and `viola_state::pin::PinError::StrictModes` carry fixed `Display` messages with no path or payload: `the capability stamps failed the strict-modes check`, a write failure its own fixed message, and `the pinned copy failed the strict-modes check`; `run` maps `PinError::StrictModes` to the `sideload_fallback` outcome `strict-modes-failed`.»
  add: «- Absence: no check runs after every E2E test; the three surface witnesses, the enumeration and the census of writers stand in its place, reading the check in `tests/support/events.rs`.»

## Patterns
(none)
