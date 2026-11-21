@row_set n=1 row=contract/mcp-iserror/instance-unreachable cell=promise seen=1 detector=none
value: The instance-unreachable detail is one closed cause: strict-modes-failed, instance-dead, unwrapped-name, endpoint-lost, server-verify-failed, socket-dir-refused or call-deadline, the last written by the frame tools alone; the CLI's document less v, its text `refused: instance-unreachable`.
basis: fct-vjs7vhbz
because: The unreachable-cause set and the MCP instance-unreachable detail set grew from six to seven with call-deadline, so the row's closed list must name it.
@row_set n=2 row=decision/mcp-unreachable-carries-one-cause-word cell=rule seen=1 detector=none
value: Each MCP unreachable outcome (a refused check, a dead or unwrapped instance, a lost endpoint, an unverified server, a refused socket directory or a frame call past the MCP call deadline) carries exactly one UnreachableCause word as its detail.
basis: fct-xhke6zun
because: A frame tool call that reaches the MCP call deadline now returns instance-unreachable with the cause word call-deadline, an outcome the row's list did not name.
@row_set n=3 row=decision/instance-unreachable-not-logged cell=rule seen=1 detector=D-obs-instrumentation
value: An MCP instance-unreachable reject, from any instance, liveness, socket-directory, euid or server-verification check or from the MCP call deadline, is logged by no line; the isError code is its only record.
basis: fct-rkhivonu
because: The frame tools under the MCP call deadline return instance-unreachable and, by this rule, write no line, so the row's list of sources must include the deadline.
@row_set n=4 row=decision/claude-internals-not-a-contract cell=rule seen=4 fact=F3 detector=none
value: The wrapped claude CLI is a third-party binary run unmodified and instrumented at its boundary only (spawn and exit, paste writes, the vt100 readiness-gate outcome); its internal state, transcripts and bundle strings, the MCP client's tool-call bound among them, are never read as a contract.
basis: fct-xhke6zun
because: The claude MCP client's tool-call bound is now a per-version reading of its bundle strings, never a contract, which the row already says less of for the CLI's internals.
@row_add n=1 row=decision/mcp-call-deadline area=error-capture-reporting fact=F1 detector=none
name: MCP call deadline on the frame tools
rule: Every viola mcp frame tool call (send, wait, last, answer) ends at a compiled 55 s MCP call deadline and returns instance-unreachable with cause call-deadline; a send's outcome is then unknown to the driver, as with endpoint-lost, and list is not under the deadline.
why: The deadline sits below the claude MCP client's tool-call bound as read per version from its bundle (120 s) and above the 50 s wait cap.
basis: fct-xhke6zun
because: The chunk adds the compiled MCP call deadline and its closed result, a decision the error-capture section holds no row for.
@row_add n=2 row=contract/unreachable-cause/call-deadline area=obs-scope-summary fact=F2 detector=none
boundary: unreachable cause words
item: call-deadline
promise: The seventh member of the closed unreachable-cause set, written only by the MCP frame tools into the instance-unreachable isError body; never a CLI exit 21 cause, whose set stays at six.
basis: fct-vjs7vhbz
because: The unreachable-cause set gained call-deadline as its seventh member, written by the MCP frame tools alone, and no row names it beside the six exit 21 cause rows.
@row_none n=1 fact=F4 type=4
what: How the MCP call deadline is built: a Tokio timer in production behind an injected seam that tests fire by a signal; the code holds it, and the test seam is the test plan's.
