@row_set n=1 row=decision/mcp-unreachable-result-for-every-cause cell=rule seen=1 detector=D-arch-decisions
value: Every unreachable cause an MCP frame tool meets surfaces as the instance-unreachable result with the cause in detail and no line of its own; every cause but endpoint-lost and call-deadline is found before any frame is sent.
basis: fct-xhke6zun
because: The MCP frame tools now also end a call at the call deadline, which can fall after the frame is sent, so the row's claim that only endpoint-lost comes after a frame no longer holds.
@row_set n=2 row=decision/send-time-bound cell=rule seen=1 fact=F2 detector=D-arch-decisions
value: send blocks at most the readiness gate's maximum wait plus the paste plus the window, the paste write itself unbounded; on the MCP surface the call deadline bounds the driver's wait, never the wrapper's send, whose outcome is then unknown to the driver.
basis: fct-t2uvkzop
because: The MCP call deadline bounds only the driver's wait while the paste write stays unbounded, so the row's promise that the total stays below the client's tool-call timeout no longer holds as written.
@row_set n=3 row=decision/mcp-wait-cap cell=why seen=1 fact=F3 detector=D-arch-decisions
value: The cap sits below the MCP call deadline, itself below the claude client's tool-call bound as read from the CLI bundle strings per version, never measured, so a wait returns before the call is cut.
basis: fct-usvnottw
because: The cap's reason was re-based from the client's stdio default onto the 55 s call deadline and the per-version bundle reading of the client's bound.
@row_set n=4 row=contract/unreachable-cause/cause-set cell=promise seen=1 fact=F4 detector=none
value: UnreachableCause is a closed set of seven (StrictModes, InstanceDead, UnwrappedName, EndpointLost, ServerVerify, SocketDir as socket-dir-refused, CallDeadline as call-deadline), no serde path, no RefusalReason; call-deadline is written by the MCP frame tools alone, the CLI exit-21 set staying six.
basis: fct-vjs7vhbz
because: The closed unreachable-cause set gained call-deadline, written only by the MCP frame tools, while the CLI exit-21 cause set stays six.
@row_set n=5 row=contract/mcp-tool-results/instance-unreachable cell=promise seen=1 detector=none
value: An unreachable instance is an isError true result whose structuredContent is {"error":"instance-unreachable","detail":cause}, as CLI --json, the cause one of strict-modes-failed, instance-dead, unwrapped-name, endpoint-lost, server-verify-failed, socket-dir-refused, call-deadline.
basis: fct-vjs7vhbz
because: The MCP instance-unreachable result's detail set gained call-deadline, so the row's closed list of causes is incomplete.
@row_add n=1 row=decision/mcp-call-deadline area=established-decisions fact=F1 detector=D-arch-decisions
name: MCP call deadline of 55 s
rule: Every MCP frame tool call (send, wait, last, answer) is bounded by a compiled 55 s call deadline, above the 50 s wait cap and below the claude client's tool-call bound; at expiry the call's work is dropped and the tool returns instance-unreachable with detail call-deadline; list is not under it.
why: So every MCP tool call completes below the client's tool-call timeout.
guard: mcp_call_deadline_sits_between_the_wrapper_bounds_and_the_client_read
basis: fct-xhke6zun
because: The chunk lands a compiled MCP call deadline over the four frame tools with its own closed result word, a decision no standing row states.
@row_add n=2 row=decision/mcp-deadline-injected-seam area=cross-cutting-patterns fact=F5 detector=D-arch-decisions
name: MCP deadline on a Tokio timer, tested through a seam
rule: viola-mcp runs a Tokio timer for its production call deadline, and tests fire the deadline through an injected seam, never through a paused Tokio clock.
basis: fct-vjs7vhbz
because: viola-mcp now takes Tokio's time feature for the production deadline while its tests keep an injected clock, a convention no standing row states.
