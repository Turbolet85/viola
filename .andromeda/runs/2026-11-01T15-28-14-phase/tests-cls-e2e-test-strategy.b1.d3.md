# Classified — tests · e2e-test-strategy.b1

## Answers
e2e-test-strategy#1.1 · yes · the cli --json driver: the new socket-dir and euid refusals are CLI --json failures held by cli_error_bodies.rs, and the viola list board refusal is pinned by literal in tests/cli_list.rs
e2e-test-strategy#1.2 · yes · the hook driver with an Instant deadline runs the hook's new fail-open matrix and the hook_fails_open_silently_within_the_spine_bound time-bound case
e2e-test-strategy#1.3 · no · the outer-PTY passthrough is untouched; the new run refusals read exit 1 before the log over a non-PTY surface
e2e-test-strategy#1.4 · yes · the wrapper channel: the wrapper-side peer euid check, the squatter and the channel fault-row bodies read by the raw-stream and sync clients
e2e-test-strategy#1.5 · yes · the MCP stdio driver: the client euid check before the first frame, the no-frame-on-mismatch case and the MCP instance-unreachable bodies
e2e-test-strategy#1.6 · no · the generic nextest and doctest runner every new library test owes; the injected-euid unit cases run under it with no rule moved
e2e-test-strategy#2.1 · yes · Path 1 steps: the step-1 PTY wrapper bind through the new socket-dir check and the step-4 squat, whose squatted-endpoint exit 1 the chunk leaves true
e2e-test-strategy#2.2 · yes · the start home-tree check that the board's one home rule unifies, and the run refusal exit 1 before its log for a widened or other-user-readable home
e2e-test-strategy#2.3 · yes · the single root flow's steps 1–6 and its start-refusal helpers shared through tests/support/start.rs, which the new run refusals and the squat must keep
e2e-test-strategy#2.4 · no · standing S12 · a started wrapper sits in a Drop guard and is never run through assert_cmd
e2e-test-strategy#3.1 · no · Path 2 surfaces; the chunk changes no send typing, confirmation or paste behaviour, and send's pre-frame euid check is a re-check every send owes
e2e-test-strategy#3.2 · no · Path 2 send steps; the chunk moves no step, and send's connect is only the pre-frame euid check the steps keep
e2e-test-strategy#3.3 · no · Path 2 signals on records, receipt and exit codes 0, 10 and 13; the chunk changes none and names no new send refusal
e2e-test-strategy#3.4 · no · agent-run cleanup of the p2 session; no Unix cleanup rule or force-kill is stated here
e2e-test-strategy#4.1 · yes · Path 3 surfaces: chaos_wait_parked_wrapper_killed_exits_21 (the vanished-wrapper exit 21, left true) and the MCP wait and last frame tools under the new client check
e2e-test-strategy#4.2 · yes · Path 3 steps: the wait client and step 7's killed wrapper under a parked wait, whose exit 21 reading the chunk keeps
e2e-test-strategy#4.3 · yes · step 7's CLI exit 21 instance-unreachable with endpoint-lost and instance-dead, and the MCP isError body with the same content, text refused: instance-unreachable
e2e-test-strategy#4.4 · yes · harness cleanup verifying killed wrappers gone by sysinfo pid and start time, the verdict the chunk measures against endpoint_gone after a force-kill
e2e-test-strategy#5.1 · no · Path 4 dialog procedure and surface inventory; the chunk moves no dialog path and its hook refusal rule is stated at 5.3
e2e-test-strategy#5.2 · no · Path 4 dialog steps; the chunk adds no step and changes no dialog answer
e2e-test-strategy#5.3 · yes · the hook's no-body, exit 0 and stderr 0 contract, which a hook refused by the directory or euid check must hold as fail-open
e2e-test-strategy#5.4 · no · names harness cleanup for Path 4 only; no Unix cleanup rule is ruled over here
e2e-test-strategy#6.1 · yes · the rule that no MCP result, body or tool description names release, which the new MCP instance-unreachable bodies must hold
e2e-test-strategy#6.2 · no · Path 5 wheel steps; pause, release and the stdin classifier are unchanged by the chunk
e2e-test-strategy#6.3 · yes · driver-facing hints and MCP refusal text never name release, a rule the new refusal hints shown to a driver must hold
