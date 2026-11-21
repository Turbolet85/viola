@leaf_group group=architecture+security-plan
@leaf_read leaf=.claude/rules/api.md lines=41
last: _This section is owned by `/wrap-session`. setup-project pre
@leaf_edit n=1 leaf=.claude/rules/api.md line=28 class=stale
leaf-text: (`viola_core::UnreachableCause`, no serde path and no `RefusalReason`)
source: ctr-x7v6ouwh «UnreachableCause is a closed set of seven (StrictModes, InstanceDead, UnwrappedName, EndpointLost, ServerVerify, SocketDir as socket-dir-refused, CallDeadline as call-deadline), no serde path»
replace-with: (the six exit-21 causes of `viola_core::UnreachableCause`'s closed seven, never its MCP-only `call-deadline`, no serde path and no `RefusalReason`)
@leaf_edit n=2 leaf=.claude/rules/api.md line=35 class=omission
leaf-text: a refused `peer_admitted` or a failed `server_verified` as `server-verify-failed`, `isError` the only record
source: ctr-wywpxiwa «the cause one of strict-modes-failed, instance-dead, unwrapped-name, endpoint-lost, server-verify-failed, socket-dir-refused, or call-deadline, MCP's alone.»
replace-with: a refused `peer_admitted` or a failed `server_verified` as `server-verify-failed`, and a frame tool call reaching its call deadline as `call-deadline` (MCP's alone, never a CLI exit-21 cause), `isError` the only record
@leaf_edit n=3 leaf=.claude/rules/api.md line=35 class=omission
leaf-text: every call returns below the client's tool-call timeout: MCP `wait` sends `timeout_ms` = `min(given, 50 000)` (`viola_mcp::WAIT_CAP`), 50 000 ms when none is given, below claude 2.1.287's stdio tool-call default (over 50 s, as measured live).
source: dec-eljum6qo «Every MCP frame tool call (send, wait, last, answer) is bounded by a compiled 55 s call deadline, above the 50 s wait cap and below the claude client's tool-call bound; at expiry the call's work is dropped»
replace-with: every call returns below the client's tool-call timeout: each frame tool call (`send`, `wait`, `last`, `answer`; `list` is not under it) is bounded by a compiled 55 s call deadline, at expiry its work dropped and `instance-unreachable` with `detail` `call-deadline` returned, and MCP `wait` sends `timeout_ms` = `min(given, 50 000)` (`viola_mcp::WAIT_CAP`), 50 000 ms when none is given, below that 55 s deadline, itself below the client's tool-call bound; on the MCP surface the deadline bounds the driver's wait, never the send itself.
