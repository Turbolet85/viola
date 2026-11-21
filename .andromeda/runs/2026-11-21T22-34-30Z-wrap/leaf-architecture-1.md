@leaf_group group=architecture-1
@leaf_read leaf=.claude/docs/conventions.md lines=61
last: - For warnings and gotchas, see `.claude/docs/gotchas.md`
@leaf_read leaf=.claude/docs/services/viola-agent-claude.md lines=47
last: - `doc:architecture` [CLI Version Compatibility] · `.claude/
@leaf_read leaf=.claude/docs/services/viola-channel.md lines=45
last: - `doc:architecture` §Standard Contracts (Wrapper channel fr
@leaf_read leaf=.claude/docs/services/viola-core.md lines=43
last: - `.claude/docs/stack.md` · `.claude/docs/conventions.md` · 
@leaf_read leaf=.claude/docs/services/viola-mcp.md lines=39
last: - `doc:architecture` [MCP] · `.claude/rules/api.md`
@leaf_edit n=1 leaf=.claude/docs/services/viola-core.md line=6 class=stale
leaf-text: `UnreachableCause` (`StrictModes` · `InstanceDead` · `UnwrappedName` · `EndpointLost` · `ServerVerify` · `SocketDir`, each word through `as_str`, `SocketDir`'s `socket-dir-refused`: exit 21's closed cause set, with no serde path, the two clients writing every word, and no `RefusalReason`)
source: ctr-x7v6ouwh «UnreachableCause is a closed set of seven (StrictModes, InstanceDead, UnwrappedName, EndpointLost, ServerVerify, SocketDir as socket-dir-refused, CallDeadline as call-deadline), no serde path, no Ref»
replace-with: `UnreachableCause` (`StrictModes` · `InstanceDead` · `UnwrappedName` · `EndpointLost` · `ServerVerify` · `SocketDir` · `CallDeadline`, each word through `as_str`, `SocketDir`'s `socket-dir-refused`, `CallDeadline`'s `call-deadline`: a closed set of seven, with no serde path and no `RefusalReason`; the first six are exit 21's, written by both clients, `call-deadline` by the MCP frame tools alone)
@leaf_edit n=2 leaf=.claude/docs/services/viola-mcp.md line=22 class=omission
leaf-text: `strict-modes-failed` · `instance-dead` · `unwrapped-name` · `endpoint-lost` · `server-verify-failed` · `socket-dir-refused`, `of_reply` reading
source: ctr-wywpxiwa «An unreachable instance is an isError true result whose structuredContent is {"error":"instance-unreachable","detail":cause}, as CLI --json, the cause one of strict-modes-failed, instance-dead, unwrap»
replace-with: `strict-modes-failed` · `instance-dead` · `unwrapped-name` · `endpoint-lost` · `server-verify-failed` · `socket-dir-refused` · `call-deadline` (MCP's alone, the 55 s call deadline's expiry), `of_reply` reading
@leaf_edit n=3 leaf=.claude/docs/services/viola-mcp.md line=24 class=stale
leaf-text: below claude 2.1.287's stdio tool-call default, which exceeds 50 s (as measured live; the exact default unmeasured).
source: dec-xgdfrbld «MCP `wait` sends `timeout_ms` as the smaller of the given value and 50 000 ms, and 50 000 ms when none is given. · why: The cap sits below the 55 s MCP call deadline, itself below the client's tool-call bou»
replace-with: below the 55 s MCP call deadline, itself below the client's tool-call bound, so a wait returns before either cuts the call. Every frame tool call (`send`, `wait`, `last`, `answer`; `list` is not under it) is bounded by that compiled 55 s call deadline: at expiry the call's work is dropped and the tool returns `instance-unreachable` with `detail` `call-deadline`.
@leaf_edit n=4 leaf=.claude/docs/conventions.md line=33 class=stale
leaf-text: `<cause>` one of `viola_core::UnreachableCause`'s closed words
source: ctr-x7v6ouwh «UnreachableCause is a closed set of seven (StrictModes, InstanceDead, UnwrappedName, EndpointLost, ServerVerify, SocketDir as socket-dir-refused, CallDeadline as call-deadline), no serde path, no Ref»
replace-with: `<cause>` one of the six exit-21 words of `viola_core::UnreachableCause`'s closed seven (the seventh, `call-deadline`, written by the MCP frame tools alone)
