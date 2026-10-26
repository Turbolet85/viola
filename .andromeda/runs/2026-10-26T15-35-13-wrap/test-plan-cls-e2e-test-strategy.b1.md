# Classified — test-plan · e2e-test-strategy.b1

## Answers
e2e-test-strategy#1.1 · yes · add-to · the verbs link and unlink with --json and their three plain result lines are literal-pinned in tests/cli_links.rs, a pinning file this row does not name
e2e-test-strategy#1.2 · no · the hook and statusline arms and their assert_cmd driver are unchanged, the session adding only the link and unlink verbs
e2e-test-strategy#1.3 · no · the run passthrough (outer PTY, child wait, drained master reader) is not reached, and run.rs gains only the link and unlink method route
e2e-test-strategy#1.4 · no · the wrapper channel driver (viola-channel sync client, raw stream, jsonschema) and the snapshot-only endpoint read still hold, the new methods using the same client and schema assertions
e2e-test-strategy#1.5 · no · the MCP stdio surface is untouched, tools/list stays the five tools and no MCP handler changed
e2e-test-strategy#1.6 · no · the library-only driver (cargo-nextest and cargo test --doc) is unchanged, and the library crates are untouched per the report's §4 line
e2e-test-strategy#2.1 · no · the Path 1 start steps run no link verb and no changed path, per the report's Paths 1–7 line
e2e-test-strategy#2.2 · no · the Path 1 signals stand per the report's Paths 1–7 line, and the statusline override unit in src/cmd/run.rs keeps its claim since the change adds only the link and unlink route test there
e2e-test-strategy#2.3 · no · path1_run_start stays on the ci 20 s kill line, the change only adding binary(channel_links) to that filter, and the mutants 10 s bound is unchanged
e2e-test-strategy#2.4 · no · the Path 1 cleanup (wrapper kill and wait, TempDir removal, listener drop) is not reached, per the report's Paths 1–7 line
e2e-test-strategy#3.1 · no · Path 2's surfaces (cli send, MCP send, fake-agent receipt) are not reached by the link verbs, per the report's Paths 1–7 line
e2e-test-strategy#3.2 · no · the Path 2 send steps and their env are unchanged, and send with its implicit link is not touched
e2e-test-strategy#3.3 · no · Path 2's records, receipt and human-mode read-back lines are not reached, per the report's Paths 1–7 line
e2e-test-strategy#3.4 · no · the Path 2 cleanup (agent-run cleanup) is unchanged
e2e-test-strategy#4.1 · no · wait, last and their named tests are not reached by the link verbs, per the report's Paths 1–7 line
e2e-test-strategy#4.2 · no · the Path 3 steps are unchanged, the wait and last verbs and their channel frames not touched
e2e-test-strategy#4.3 · no · the Path 3 signals (exit codes, turn-ended wake, instance-unreachable body) are not reached, per the report's Paths 1–7 line
e2e-test-strategy#4.4 · no · the Path 3 cleanup (harness cleanup, pid and start-time check) is unchanged
e2e-test-strategy#5.1 · no · the answer and dialog surfaces are not reached by the link verbs, per the report's Paths 1–7 line
e2e-test-strategy#5.2 · no · the Path 4 steps are unchanged, the hook and answer paths not touched
e2e-test-strategy#5.3 · no · the Path 4 signals (dialog events, hook stdout snapshots, unknown-dialog exit 13) are not reached, per the report's Paths 1–7 line
e2e-test-strategy#5.4 · no · the Path 4 cleanup (harness cleanup) is unchanged
e2e-test-strategy#6.1 · no · the wheel surfaces (outer-PTY keys, pause and release, the stdin classifier) are not reached by the link verbs, per the report's Paths 1–7 line
e2e-test-strategy#6.2 · no · the Path 5 steps are unchanged, release and pause not touched
e2e-test-strategy#6.3 · no · the Path 5 signals, the hints that never name release included, are not reached, per the report's Paths 1–7 line
