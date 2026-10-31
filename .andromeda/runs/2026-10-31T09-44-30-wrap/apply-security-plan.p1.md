# Apply — security-plan · part 1 of 2

## Verdicts
- fanout-security-plan--cascade-g2.md 1 · applied · .andromeda/security-plan.md:411#1
- fanout-security-plan.md 1 · folded into fanout-security-plan--cascade-g2.md 1
- fanout-security-plan--cascade-g2.md 2 · applied · .andromeda/security-plan.md:413#1
- fanout-security-plan.md 3 · folded into fanout-security-plan--cascade-g2.md 2
- fanout-security-plan.md 2 · applied · .andromeda/security-plan.md:340#1
- fanout-security-plan--cascade-g1.md 1 · applied · .andromeda/security-plan-detail/threat-model-summary.md:58+#1
- fanout-security-plan--cascade-g1.md 2 · applied · .andromeda/security-plan-detail/authentication-authorization.md:30+#1
- fanout-security-plan--cascade-g1.md 3 · applied · .andromeda/security-plan-detail/input-validation.md:19+#1
- fanout-security-plan--cascade-g1.md 4 · applied · .andromeda/security-plan-detail/input-validation.md:31#1
- fanout-security-plan--cascade-g1.md 5 · applied · .andromeda/security-plan-detail/input-validation.md:74#1

## Edits
- line: .andromeda/security-plan.md:411
  old: «`PtyError` (hand-written), `ChannelError`, `StateError`, `AgentError`, `McpError` and `CoreError` (`thiserror`) use fixed messages.»
  new: «`PtyError` (hand-written) and on `AgentError`, `ChannelError`, `StateError`, `PinError`, `StampsError`, `StampsReadError`, `Refused`, `McpError` and `ServeError` (`thiserror`) use fixed messages.»
- line: .andromeda/security-plan.md:413
  old: «`wrapper-fault` with the JSON-RPC fault's `{code,message,data}`, `invalid-params` with a closed field word), and every `isError` text»
  new: «`wrapper-fault` with `{code,message,data}` as `viola_channel::sanitised_fault` closes them, never the peer's own, `invalid-params` with a closed field word), a `refusal` that is no code word (`viola_core::is_code_word`) reading `unknown` and a string `detail` that is none reading null, `of_reply` here and `reply_of` on CLI `--json` alike, and every `isError` text»
- line: .andromeda/security-plan.md:340
  old: «The `Display` impls on the `<Crate>Error` enums (`thiserror`, or hand-written for `PtyError`) must not interpolate»
  new: «The `Display` impls on the product error enums (`thiserror`, or hand-written for `PtyError`; §Error Handling lists them) must not interpolate»
- after: .andromeda/security-plan-detail/threat-model-summary.md:58
  anchor: «    - Input validation is limited to JSON-RPC parsing, the `v` check (`-32602`) and the `ViolaName` nutype.»
  add: «    - That limit is the wrapper's read of a frame. A client's read of the answer is screened on its own: `reply_of` and `of_reply` take a `refusal` or a string `detail` only as a code word (`viola_core::is_code_word`) and a fault only through `viola_channel::sanitised_fault`, so nothing of the peer's but closed values reaches a CLI `--json` document or an MCP result; a code word outside the closed list stays visible by design.»
- after: .andromeda/security-plan-detail/authentication-authorization.md:30
  anchor: «  - **Implementation:** The wrapper rejects a `release` whose `params` carry a string `from` with `-32602` `"invalid params"` and `data: {"reason":"release-from-driver"}` (`viola_channel::ProtocolError::ReleaseFromDriver`), and logs one `release-from-driver` line (`from_trust:"self-reported"`); a `from` of another type is `-32602` with `data: null`. CLI `viola release` sends `from` from `VIOLA_NAME`, so run from a driver session it exits 20. Per Standard Contracts, `release` params are `{budget?, from?}`; a `release` on a wheel the driver already holds changes nothing. The refusal comes before `budget` is acted on, so a `release` with `budget: true` and a string `from` lifts no budget pause and leaves the snapshot's two budget fields as they were (chunk 2026-10-10-budget-governor). This is an affordance-level guard against drivers handing the wheel back to automation. It is not enforcement: a same-user process can leave out `from`. `pause`, `link` and `unlink` from drivers are allowed.»
  add: « **On the log and at a client:** the answer's `channel-response` line, the server's and the client's, level WARN, carries `detail:"release-from-driver"` (`ProtocolError::cause`), no param name or value on it, a `from` of another type reading `invalid-params` there. The line adds nothing to the wire body; a client keeps that body through `viola_channel::sanitised_fault` only for `data.reason` exactly `release-from-driver`.»
- after: .andromeda/security-plan-detail/input-validation.md:19
  anchor: « **`hook.event` re-validation (wrapper):** `kind` is one of the five hook kinds, `data` is an object, and for `prompt-submitted` `text` is a string and `origin` is `harness` · `human`; an invalid event is not appended.»
  add: « **A reply, read by a client:** `reply_of` (`src/cmd/client.rs`) and `of_reply` (`crates/viola-mcp/src/result.rs`) take a `refusal` or a string `detail` only as a code word (`viola_core::is_code_word`), and a fault only through `viola_channel::sanitised_fault` (§ Error Handling). **The refusals on the log:** the three `-32602` faults read `channel-response`'s `detail` `unsupported-version`, `invalid-params` and `release-from-driver`, on the server's and the client's line, level WARN, no param name or value on it; every other `channel-response` line has no `detail`.»
- line: .andromeda/security-plan-detail/input-validation.md:31
  old: «and returns what the session gave: exit 0 for an initialized session, `ServeError::Initialize` (exit 11) when the cut lands before `initialize` completes.»
  new: «and returns what the session gave: exit 0 for an initialized session; when the cut lands before `initialize` completes, `serve` returns `ServeError::Initialize` and the `viola mcp` process exits 1, both streams empty, with that one line (`src/main.rs` `exit_code`, `Role::Mcp` + `Failed`), 11 being only the unit child's own code (`serve_child_entry`).»
- line: .andromeda/security-plan-detail/input-validation.md:74
  old: «refused with the closed `Refusal::BatchScriptChild`»
  new: «refused with the closed `AgentError::BatchScriptChild`»

## Patterns
[[pattern]]
id = "sec1-01"
fixed = '`CoreError` (`thiserror`) use fixed messages'

[[pattern]]
id = "sec1-02"
fixed = '`wrapper-fault` with the JSON-RPC fault'

[[pattern]]
id = "sec1-03"
fixed = 'impls on the `<Crate>Error` enums'

[[pattern]]
id = "sec1-04"
fixed = '`ServeError::Initialize` (exit 11)'

[[pattern]]
id = "sec1-05"
fixed = 'refused with the closed `Refusal::BatchScriptChild`'
