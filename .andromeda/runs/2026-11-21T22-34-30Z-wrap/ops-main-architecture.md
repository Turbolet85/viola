@row_set n=1 row=decision/mcp-wait-cap cell=why seen=1 fact=F3 detector=D-arch-decisions
value: Below the 55 s MCP call deadline and the client's bound, a per-version reading of the 2.1.328 and 2.1.330 strings whose one 60 s figure floors HTTP only; live, a 50 s wait returned whole on both.
basis: fct-usvnottw
because: The cap's why rested on a 2.1.287 reading that 2.1.328 and 2.1.330 no longer show; the directive (inputs#I7 item 4) asks it to say what these versions read and what was measured.
@row_set n=2 row=contract/unreachable-cause/cause-set cell=promise seen=1 fact=F4 detector=none
value: UnreachableCause is a closed set of seven (StrictModes, InstanceDead, UnwrappedName, EndpointLost, ServerVerify, SocketDir as socket-dir-refused, CallDeadline as call-deadline), no serde path, no RefusalReason; the six exit 21's, written by both clients, call-deadline by the MCP frame tools alone.
basis: fct-vjs7vhbz
because: CallDeadline joins the closed set as one named member written by the MCP client alone (P4-F1), and the row keeps every member and qualifier it stated, «written by both clients» kept for the six.
@row_set n=3 row=contract/mcp-tool-results/instance-unreachable cell=promise seen=1 detector=none
value: An unreachable instance is an isError true result whose structuredContent is {"error":"instance-unreachable","detail":cause}, as CLI --json, the cause one of strict-modes-failed, instance-dead, unwrapped-name, endpoint-lost, server-verify-failed, socket-dir-refused, or call-deadline, MCP's alone.
basis: fct-vjs7vhbz
because: The MCP instance-unreachable detail set gained call-deadline, which the CLI never writes, so the list names it as MCP's alone.
