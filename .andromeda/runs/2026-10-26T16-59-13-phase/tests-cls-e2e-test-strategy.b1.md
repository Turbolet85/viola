# Classified — tests · e2e-test-strategy.b1

## Answers
e2e-test-strategy#1.1 · yes · the chunk's CLI refusal and 0770 `--home` negatives are cli-surface tests: --json through assert_cmd and predicates, the human refusal layout through trycmd
e2e-test-strategy#1.2 · yes · the hook fail-open matrix's strict-modes case over VIOLA_DIR and the hook statusline refusal cases run through assert_cmd under the Instant bound, on fixture stdin
e2e-test-strategy#1.3 · yes · the booted-run tests of the per-start plugin/ rewrite and the Windows conpty companions run in the outer PTY, exiting via child.wait() with the master drained
e2e-test-strategy#1.4 · yes · the squat, server-identity impostor and no-frame-written checks of the frame verbs run over the wrapper channel, which reads its endpoint only from snapshot.json
e2e-test-strategy#1.5 · yes · the MCP instance-unreachable refusal is an rmcp call against a viola mcp child the test owns, with stdin closed and its exit awaited
e2e-test-strategy#1.6 · no · the library-only runner (nextest and doctests) is the suite every new viola-state unit case keeps, with no action the chunk's unit cases face
e2e-test-strategy#2.1 · yes · Path 1's steps 3 to 6 on the stamped_home fixture, the duplicate, squatted and tampered-pinned-exe run refusals (exit 1) and the restart plugin/ read-back, are what the chunk's re-hash and collision-check edits must keep
e2e-test-strategy#2.2 · yes · Path 1's verification signals bind the chunk's run-start edits: the start event chain before the spawn, the per-start plugin/ rewrite with absolute pinned hook commands, the Unix settings.json override, and the exit-1 refusals
e2e-test-strategy#2.3 · yes · Path 1's one-home ordered flow on the root chain, with its 10 s mutants kill line, is where the chunk's run-start checks and re-hash must fit
e2e-test-strategy#2.4 · no · standing S3 · wrappers in a Drop guard and a fresh tempfile home per test
e2e-test-strategy#3.1 · no · Path 2's surfaces are its confirmed-send coverage, which is not needed; the chunk adds only the send check ahead of its frame, carried by the cli and MCP surface rows
e2e-test-strategy#3.2 · no · Path 2's steps (paste framing, mute and local sends, the MCP send) are send's own confirmation behaviour, which the chunk does not need
e2e-test-strategy#3.3 · no · Path 2's confirmation, CL-1 record and refusal-document signals are send's own behaviour, which the chunk does not need
e2e-test-strategy#3.4 · no · Path 2's cleanup is the harness session teardown, which the chunk's tests do not use
e2e-test-strategy#4.1 · no · Path 3's surfaces are its wake and read-back coverage, which the chunk does not need; the refusal shape it needs sits in the verification signal
e2e-test-strategy#4.2 · yes · step 7's parked wait and rmcp wait on a dead instance are the instance-unreachable refusal pattern the chunk's wait checks follow, steps 1 to 6 being wake behaviour
e2e-test-strategy#4.3 · yes · the instance-unreachable body (CLI exit 21, and the MCP is_error result with the same body and text refused: instance-unreachable) is the refusal shape the chunk's wait and last checks must emit; its wake signals are not needed
e2e-test-strategy#4.4 · no · Path 3's cleanup is the harness teardown with sysinfo pid checks, which the chunk's tests do not use
e2e-test-strategy#5.1 · no · Path 4's surfaces are its dialog and answer coverage, which the chunk does not need; its hook and answer surfaces are carried by the cli, channel and MCP rows
e2e-test-strategy#5.2 · no · Path 4's steps (blocking hook decision, answer by id, the paused-wheel negative) are dialog behaviour, which the chunk does not need; it adds only the check ahead of the hook.dialog and answer frames
e2e-test-strategy#5.3 · no · Path 4's dialog event, decision-body, unknown-id and paused-wheel signals are dialog behaviour, which the chunk does not need; its hook fail-open oracle comes from the chunk's own Needs
e2e-test-strategy#5.4 · no · Path 4's cleanup is the harness teardown, which the chunk's tests do not use
e2e-test-strategy#6.1 · no · Path 5's surfaces are its wheel and human-typing coverage, which the chunk does not need; its pause and release checks are carried by the cli and MCP surface rows
e2e-test-strategy#6.2 · no · Path 5's steps (human keystrokes, pause and release, the harness-injected turn) are wheel behaviour, which the chunk does not need
e2e-test-strategy#6.3 · yes · the rule that no driver-facing hint and no MCP refusal text names release binds the chunk's new refusal output; its wheel and turn-running signals are not needed
