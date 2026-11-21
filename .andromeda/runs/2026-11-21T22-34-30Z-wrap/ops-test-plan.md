@row_set n=1 row=decision/time-faked-through-seams cell=rule seen=1 fact=F4 detector=D-tests-framework
value: Time is faked only through injected seams, with no mock_instant and no paused Tokio clock: the viola_core Clock, the MCP adapter's injected deadline provider, which tests fire by a signal and never by waiting out the 55 s MCP call deadline, and a pure decision takes its instant as an argument.
basis: fct-vjs7vhbz
because: viola-mcp now runs a Tokio timer for its production call deadline, so the row's claim that the MCP adapter runs no tokio timer is false; the deadline is an injected seam the tests fire by a signal.
@row_set n=2 row=contract/mcp-tools/result-body cell=promise seen=1 detector=D-tests-count-literals
value: Each MCP tool's result and refusal equal the CLI's --json document less v, save instance-unreachable with detail call-deadline, which the MCP frame tools alone write and which has no CLI document.
basis: fct-vjs7vhbz
because: The MCP instance-unreachable cause set gained call-deadline while the CLI exit-21 set stays six, so not every MCP refusal equals a CLI document.
@row_set n=3 row=contract/mcp-tools/instance-unreachable cell=promise seen=1 detector=D-tests-count-literals
value: is_error:true with the CLI document less v, text refused: instance-unreachable; detail call-deadline, written by the MCP frame tools alone, has no CLI document.
basis: fct-vjs7vhbz
because: call-deadline is an MCP-only cause with no CLI exit-21 counterpart, so the row's equality with the CLI document no longer holds for every detail.
@row_set n=4 row=contract/mcp-tool-result/instance-unreachable cell=promise seen=1 detector=D-tests-count-literals
value: isError true with {"error":"instance-unreachable","detail":"<cause>"}, text refused: <code>; the body equals the CLI exit-21 document less v for every cause but call-deadline, which the CLI never writes.
basis: fct-vjs7vhbz
because: The MCP detail set is seven with call-deadline while the CLI exit-21 set stays six, so the body is the CLI document less v only for the shared causes.
@row_set n=5 row=decision/cli-mcp-identical-payloads cell=except seen=1 fact=F3 detector=D-tests-count-literals
value: The MCP instance-unreachable cause set has seven members while the CLI exit-21 cause set stays six: call-deadline is the one MCP-only cause and has no CLI counterpart.
basis: fct-vjs7vhbz
because: call-deadline is written by the MCP frame tools alone, so the identical-payloads decision now carries a named exception.
@row_add n=1 row=contract/instance-unreachable-cause/call-deadline area=test-scope-summary fact=F1 detector=D-tests-count-literals
boundary: instance-unreachable causes
item: call-deadline
promise: A closed cause word of the MCP instance-unreachable detail alone, written only by the MCP frame tools send, wait, last and answer when a call reaches the 55 s MCP call deadline; the frame is dropped and an in-flight send's outcome is unknown, as with endpoint-lost.
basis: fct-xhke6zun
because: The closed unreachable-cause set moved from six to seven members with call-deadline, and the per-member rows owe a call-deadline row beside them.
@row_add n=2 row=contract/mcp-unreachable-detail/call-deadline area=integration-test-strategy fact=F2 detector=D-tests-coverage
boundary: MCP frame tools instance-unreachable detail
item: call-deadline
promise: Each of send, wait, last and answer ends with detail call-deadline at the MCP call deadline, the frame dropped; a reply arriving before the deadline is returned as the reply.
basis: fct-rkhivonu
because: The MCP frame tools now race every frame call against the call deadline, a new detail the integration suite holds per tool beside the other detail rows.
@row_add n=3 row=decision/live-rig-mb-long-call-verdict area=e2e-test-strategy fact=F5 detector=none
name: Live rig MB arm long-call verdict
rule: The live rig's MB arm judges a long MCP call green only on timed-out true beside call-error false; the arm keeps no stdout byte and its MCP config is owner-only inside the scratch.
why: A client result with no is_error key reads call-error false, so call-error alone could turn a cut call green.
state: provisional
basis: fct-wl4q6mbh
because: The rig gained the MB arm beside AV, and its long-call predicate and output discipline are a test verdict rule later live rounds must keep.
@row_add n=4 row=decision/claude-mcp-default-bound-is-a-reading area=6-e2e-test-strategy fact=F6 detector=none
name: Claude MCP default tool-call bound is a reading
rule: The claude CLI's default MCP tool-call bound is a reading of each version's bundle strings, never a measurement; live MB starts measure only that a 50 s call returns whole and that a per-server timeout cuts the call while the server outlives it.
basis: fct-xcouqzdk
because: The chunk read the default bound from the 2.1.328 and 2.1.330 bundles and measured only the whole 50 s call and the per-server cut, so the line between reading and measurement must stand for later rounds.
