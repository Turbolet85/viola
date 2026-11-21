@row_set n=1 row=contract/mcp-iserror/instance-unreachable cell=promise seen=1 detector=none
value: An unreachable instance, or a frame tool call reaching its call deadline, reads error instance-unreachable with detail one of its closed causes, call-deadline among them on MCP alone.
basis: fct-vjs7vhbz; fct-rsmdwa5q; fct-xhke6zun
because: The MCP instance-unreachable result now also ends a frame tool call at its deadline with detail call-deadline, so the promise must name that cause beside the instance's own.
@row_add n=1 row=decision/mcp-call-deadline area=error-handling fact=F1 detector=none
name: MCP frame tool call deadline
rule: Every MCP frame tool call (send, wait, last, answer) ends at a compiled 55 s call deadline, above the 50 s wait cap and below the client's tool-call bound read per version, returning instance-unreachable call-deadline with an in-flight frame's outcome unknown; list is not under it.
why: So every MCP tool call completes below the client's tool-call timeout.
guard: handlers_frame_tools_end_at_the_call_deadline
basis: fct-xhke6zun; fct-yzlav4k4; fct-t2uvkzop
because: The chunk bounds every MCP frame tool call by a compiled deadline whose expiry is a new error result, a decision the error-handling rows do not yet hold.
@row_add n=2 row=contract/instance-unreachable-cause/call-deadline area=error-handling fact=F2 detector=none
boundary: instance-unreachable detail causes
item: call-deadline
promise: A closed cause of instance-unreachable, written by the MCP frame tools alone when a call reaches its call deadline; the CLI exit-21 cause set stays six and never carries it.
basis: fct-vjs7vhbz; fct-xhke6zun; fct-h7kd3liu
because: The closed unreachable-cause set grows from six to seven with call-deadline, and each member of the set stands as its own row beside the six.
@row_add n=3 row=decision/live-rig-mb-arm-keeps-no-output area=data-protection fact=F3 detector=none
name: Live rig's MB arm keeps no client output
rule: The live rig's MB arm keeps no stdout byte of the claude client, read through MAX_FRAME into closed words, keeps stderr only as a byte count and a SHA-256, writes its MCP config 0600 inside the scratch, removed with it, and puts a server timeout on the cut label alone.
why: A long call counts whole only with timed-out true beside call-error false, so a result with no is_error key cannot turn a cut call green.
state: provisional
basis: fct-wvnepvu3; fct-rkhivonu; fct-wl4q6mbh
because: The chunk adds the rig's MB arm, whose ratified shape is owed a row of its own beside the MCP recording arm's row.
@row_none n=1 fact=F4 type=9
what: The wrapper paste's missing time bound is an open finding pinned as a CARRY, its route entry a proposal; the route holds it.
