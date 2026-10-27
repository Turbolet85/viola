# Classified — tests · e2e-test-strategy.b1

## Answers
e2e-test-strategy#1.1 · yes · the cli driver (assert_cmd, trycmd for human layouts) that the chunk's new send, answer and path-name refusal tests run through
e2e-test-strategy#1.2 · yes · the hook driver (assert_cmd with the spine Instant deadline) that the oversize and malformed hook stdin fail-open cases run through
e2e-test-strategy#1.3 · yes · the outer-PTY driver that the chunk's passthrough re-verify (child bytes still reach the terminal under a vt100 panic) runs through
e2e-test-strategy#1.4 · yes · the wrapper-channel driver that the chunk's MAX_FRAME ±1 and -32600 frame cases and its wrapper-side paste refusal run through
e2e-test-strategy#1.5 · yes · the MCP stdio driver (rmcp client over a spawned viola mcp child) that the chunk's mcp stdin cap and MCP send paste re-verify run through
e2e-test-strategy#1.6 · yes · the library-only driver (nextest, doc tests) that the chunk's validate_paste_text unit cases and parser proptests run under
e2e-test-strategy#2.1 · yes · the one ordered start flow of path1_run_start_one_home_in_order, the test whose settled CI wall the chunk's WATCH tallies
e2e-test-strategy#2.2 · yes · the start signals inside that WATCH flow, including the duplicate-run exit 1 that reads snapshot.json, a file the chunk caps
e2e-test-strategy#2.3 · yes · names path1_run_start_one_home_in_order on the ci profile's 20 s kill line, the WATCH the chunk tallies
e2e-test-strategy#2.4 · no · standing S4 · each test makes its own fresh home under target/e2e-home, with started wrappers held by a Drop guard
e2e-test-strategy#3.1 · yes · the CLI and MCP send surfaces, the two clients the chunk re-verifies the paste rule at
e2e-test-strategy#3.2 · yes · the send inputs the paste rule covers, the stdin text, the wrapped long text and the MCP send text
e2e-test-strategy#3.3 · yes · the paste-rule witnesses, typed_text trailing CR and LF removal, inner CR typed as one LF and the empty-text refusal
e2e-test-strategy#3.4 · no · the harness session teardown, which the chunk does not reach (the harness record is Not needed)
e2e-test-strategy#4.1 · yes · the Stop's background_tasks carried only as its length, a hook payload bound the chunk sets, and the MCP wait timeout clamp
e2e-test-strategy#4.2 · no · the wait and last readback steps, behaviour the chunk does not change; its bounds sit in 4.1
e2e-test-strategy#4.3 · no · the wait wake, timed_out and dead-instance signals, behaviour the chunk does not change
e2e-test-strategy#4.4 · no · the harness teardown and the wait-vanish kill check, which the chunk does not reach
e2e-test-strategy#5.1 · yes · the answer surfaces the chunk bounds (cli answer, channel hook.dialog and answer, MCP answer) and the option_details payload bound
e2e-test-strategy#5.2 · yes · the answer inputs the chunk bounds, oversize answer JSON on stdin, free-text answers under the paste rule and the question payload
e2e-test-strategy#5.3 · yes · the input-side witnesses, the unknown-dialog refusal on a u64 dialog_id, a dialog never refused by its payload, and the MCP answer result
e2e-test-strategy#5.4 · no · the harness session teardown, which the chunk does not reach (the harness record is Not needed)
e2e-test-strategy#6.1 · no · Path 5's wheel surfaces (human keystroke, pause, release), which the chunk adds no wheel behaviour to
e2e-test-strategy#6.2 · no · Path 5's wheel steps (human keystroke, pause, release, the injected harness turn), which the chunk adds no wheel behaviour to
e2e-test-strategy#6.3 · no · Path 5's wheel and turn-running verification (wheel records, manual-pause, release-from-driver), which the chunk adds no wheel behaviour to
