## 2026-10-31-sanitised-error-surfaces — channel-response's closed detail word
**Section:** §Authentication & Authorization (authentication-authorization.md: Driver-originated `release`) · §Error Handling (error-handling.md) · §Input Validation (input-validation.md: Channel frames)
**Change:**
- Driver-originated `release` gains the log and the client: the answer's `channel-response` line, the server's and the client's, level WARN, carries `detail:"release-from-driver"` (`ProtocolError::cause`), no param name or value on it, a `from` of another type reading `invalid-params` there. The line adds nothing to the wire body; a client keeps that body through `viola_channel::sanitised_fault` only for `data.reason` exactly `release-from-driver`.
- Error Handling gains the writer: `ProtocolError::cause` gives `unsupported-version`, `invalid-params` and `release-from-driver` for the three `-32602` faults and nothing for the other four; `ResponseLine`'s `Drop` writes it as `channel-response`'s `detail`, a closed enum (`schemas/diag-line.v1.json`), on both the server's and the client's line.
- Channel frames gains the refusals on the log: the three `-32602` faults read those three words, on the server's and the client's line, level WARN, no param name or value on it; every other `channel-response` line has no `detail`.
**Why:** the schema closed `channel-response`'s `detail` from any string to the three words, and the line tells the refusals apart with no param on it. The level did not move, the wire form did not change, `v` stays 1 and the seven `ProtocolError` bodies are unchanged.
**Ref:** .andromeda/runs/2026-10-31T09-44-30-wrap/

## 2026-10-31-sanitised-error-surfaces — the clients' screen of a peer's reply
**Section:** §Error Handling (error-handling.md; security-plan.md: External responses, the MCP `isError` line) · §Input Validation (input-validation.md: Channel frames) · §Threat Model Summary (threat-model-summary.md: boundary-ipc, Consequences)
**Change:**
- Error Handling gains `viola_channel::sanitised_fault(&Value) -> Option<Value>`, the fault reader of its two callers, `reply_of` (`src/cmd/client.rs`) and `of_reply` (`crates/viola-mcp/src/result.rs`): `None` without an integer `code`; for each of the five known codes `message` is the fault's fixed text and `data` null, save two `-32602` bodies.
- Those two keep `data`: the unsupported-version body, an integer `data.supported` with a `data.wrapper` of 1 to 64 ASCII letters, digits, `.`, `-` or `+`, and the body whose `data.reason` is exactly `release-from-driver`. Any other `-32602` reads `invalid params`, `data` null.
- A code outside the five reads `{"code": <code>, "message": "wrapper fault", "data": null}`, a fixed output word on `--json` and MCP; the human line reads `error: wrapper fault  <code>` (`src/human/mirror.rs`).
- `viola_core::is_code_word` admits 1 to 64 bytes of `[a-z0-9-]`, a lower-case letter first. In both readers a `refusal` failing it reads `unknown`, a string `detail` failing it reads null, a non-string `detail` is no known reply.
- Channel frames and boundary-ipc gain the same screen; boundary-ipc names the wrapper's input-validation limit as its read of a frame and states that nothing of the peer's but closed values reaches a CLI `--json` document or an MCP result.
- The MCP `isError` line: `wrapper-fault` was with the JSON-RPC fault's `{code,message,data}`; now with them as `sanitised_fault` closes them, never the peer's own. It gains the `refusal` and `detail` readings, `of_reply` and `reply_of` on CLI `--json` alike.
**Why:** `sanitised_fault` replaced both private `fault_detail` copies, so a peer's `message`, `data`, `refusal` or `detail` reaches no document or text. Standing exception, by design: a code word outside the closed list stays visible in the document and in `refused: <word>`, its exit `unknown`'s 14 by `reason_of`.
**Kept:** the boundary-ipc line limiting input validation to JSON-RPC parsing, the `v` check and the `ViolaName` nutype stands, as the wrapper's read. "Shared validators live in `viola-core`" stands beside `sanitised_fault` in `viola-channel`: that is a fault reader, no input validator.
**Ref:** .andromeda/runs/2026-10-31T09-44-30-wrap/

## 2026-10-31-sanitised-error-surfaces — the error bodies' census and what it does not drive
**Section:** §Error Handling (error-handling.md)
**Change:**
- `tests/contract_error_bodies.rs`, carrying `andromeda:walks-tree`, enumerates every error body with its writer and witness (`CLI_DOCUMENTS`, `MCP_RESULTS`, `CHANNEL_FAULTS`, with `NOT_ERROR_BODIES` and `PLUMBING` the named exclusions); `product_error_writers_are_each_named` fails on an unnamed product error writer and `thiserror_messages_interpolate_nothing` on an interpolating `thiserror` message.
- `tests/support/events.rs` holds the error-body check, `error_body_hit` and `assert_error_body_clean`, with the classes `absolute-path` · `caused-by` · `serde-path` · `planted`; it reports place and class, never the bytes it read.
- Absence: no check runs after every E2E test; the three surface witnesses, the enumeration and the census of writers stand in its place, reading that check.
- Absence: no `-32603` is driven in a scanned home, and no failpoint or counted panic drives one there; the unit layer holds it (`answer_a_panicking_dispatch_is_internal_error_and_closes`, `links_set_method_an_append_that_fails_is_internal_with_the_set_unchanged`).
- Exception: the exit-2 paths of `viola answer` and `viola send` write the fixed `UNUSABLE_RESPONSE` / `UNUSABLE_TEXT` line to stderr whatever `--json` says, and no document (`src/cmd/answer.rs`, `src/cmd/send.rs`); `NOT_ERROR_BODIES` names them.
**Why:** the after-every-test check test-plan §6 described was measured absent before this chunk and was not built: the witnesses, the enumeration and the census replace it. The claim that an oversize `answer` gives one `--json` document was disproved by measurement, and the chunk added no way to drive a `-32603` in a scanned home.
**Ref:** .andromeda/runs/2026-10-31T09-44-30-wrap/

## 2026-10-31-sanitised-error-surfaces — the product error enums, by census
**Section:** §Error Handling (error-handling.md; security-plan.md: External responses) · §Bootstrap phases, error-sanitization-wire (security-plan.md)
**Change:**
- error-handling.md gains the census: the `thiserror::Error` derives over product sources are nine, `AgentError` · `ChannelError` · `StateError` · `PinError` · `StampsError` · `StampsReadError` · `Refused` · `McpError` · `ServeError`; the census in `tests/contract_error_bodies.rs` reads their messages.
- error-sanitization-wire: was the `Display` impls on the `<Crate>Error` enums; now on the product error enums (`thiserror`, or hand-written for `PtyError`; §Error Handling lists them), the no-interpolation rule the same.
- External responses, the fixed-message list: was `PtyError` (hand-written), `ChannelError`, `StateError`, `AgentError`, `McpError` and `CoreError` (`thiserror`); now `PtyError` (hand-written) and `AgentError`, `ChannelError`, `StateError`, `PinError`, `StampsError`, `StampsReadError`, `Refused`, `McpError` and `ServeError` (`thiserror`). `CoreError` retired; variant fields holding paths or payloads stay out of `Display`.
**Why:** `CoreError` never existed in the code, and the list was stale in both directions: `PinError`, `StampsReadError` and `Refused` went unnamed. The derives read 9, from 10, with `viola_agent_claude::Refusal` folded. `<Crate>Error` read as one enum per crate, while five of the nine carry no crate name. The fix was a census, not a wider fold.
**Ref:** .andromeda/runs/2026-10-31T09-44-30-wrap/

## 2026-10-31-sanitised-error-surfaces — the open enum divergences and their owner
**Section:** §Error Handling (error-handling.md)
**Change:**
- Gains: open divergences from the error-enum convention are `ServeError`, `StampsError`, `PinError`, `StampsReadError` and `Refused`; owner: a CARRY on the route entry Version done-check, PROVISIONAL, owed the founder's own word.
**Why:** `ServeError` and `StampsError` stay open as the founder's Epoch 5 batch left them, and the census adds the other three. The owner is the overseer's answer in the founder's stead (delegate overseer, 2026-10-31, PROVISIONAL, owed the founder's own word).
**Ref:** .andromeda/runs/2026-10-31T09-44-30-wrap/

## 2026-10-31-sanitised-error-surfaces — viola_agent_claude::Refusal folded into AgentError
**Section:** §Error Handling (error-handling.md) · §Input Validation (input-validation.md: Child executable resolution)
**Change:**
- error-handling.md gains: `resolve_program` returns `Result<PathBuf, AgentError>` in `viola_agent_claude` and in the root wrapper `src/run/mod.rs`, its refusals `AgentError::BatchScriptChild` and `AgentError::ProgramNotFound`, each a fixed message; `src/cmd/revive.rs`'s private `Refusal` is another type.
- Child executable resolution: the `.cmd`/`.bat` refusal was the closed `Refusal::BatchScriptChild`; now the closed `AgentError::BatchScriptChild`, the rest of the sentence as written.
**Why:** `Refusal` was folded rather than ratified, `StampError` the precedent, leaned at the chunk's plan; `ProgramNotFound` was `Refusal::NotFound`, and the two messages and every output are unchanged. Trap for later sweeps: a bare `Refusal` token also reads `RefusalReason` and `revive`'s private enum, so search the backticked name and its variants.
**Ref:** .andromeda/runs/2026-10-31T09-44-30-wrap/

## 2026-10-31-sanitised-error-surfaces — viola mcp's exit on a cut before initialize
**Section:** §Input Validation (input-validation.md: MCP stdin, On the cut)
**Change:**
- A cut before `initialize` completes: was `ServeError::Initialize` (exit 11); now `serve` returns `ServeError::Initialize` and the `viola mcp` process exits 1, both streams empty, with that one line (`src/main.rs` `exit_code`, `Role::Mcp` + `Failed`), 11 being only the unit child's own code (`serve_child_entry`). Exit 0 for an initialized session stands.
**Why:** exit 11 as `viola mcp`'s exit was disproved by measurement: the process reads exit 1, both streams empty, one `parse-rejected{mcp-stdin, oversize, count:1}`. The process exit stays 1 and the plan moves to it. Trap for later sweeps: `exit 11` alone also reads `budget-paused`'s exit 11, so key on `ServeError::Initialize` and `initialize`.
**Ref:** .andromeda/runs/2026-10-31T09-44-30-wrap/
