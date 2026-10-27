# Classified — test-plan · integration-test-strategy.b1

## Answers
integration-test-strategy#1.1 · no · the module-to-module approach and its hook-parse, core and state-append chain are unchanged, and the new cases are unit, census and child-process tests
integration-test-strategy#1.2 · no · the concurrent-append check and its file set stand, and the session adds no concurrent-append case (its bounded compare is a read)
integration-test-strategy#1.3 · no · the only external boundary is still child processes (the new MCP stdin case is a child spawn), and the fake-agent modes and npm shim are unchanged
integration-test-strategy#1.4 · no · the wrapper-channel endpoint and its socket and pipe checks are unchanged, and the session bounds MCP stdio, which this boundary does not describe
integration-test-strategy#1.5 · no · the PTY spawn, resize and sideload rules are unchanged, and the chaos test's outer-terminal passthrough check is a §6 chaos-suite assertion this approach does not name
integration-test-strategy#1.6 · no · the hook fail-open path and its instance check are unchanged, and the session touches no hook code or hook test
integration-test-strategy#2.1 · no · fs.rs is reached whole, but CI ci#40458923095 runs this witness green on three OSes on the changed tree, and the fs.rs edit is confined to replace_private_shared's compare and its helper
integration-test-strategy#3.0 · no · chaos_feed_panic.rs only gains an outer-terminal passthrough assertion, so its outside_scan boot and its own panic and parse-rejected asserts stand, as CI green runs them
integration-test-strategy#4.1 · yes · add-to · the MCP stdio input this widened-instance case drives is now line-capped at MAX_FRAME, and a line past the cap ends the session
integration-test-strategy#4.2 · no · the on-disk round-trip, replay and strict-modes refusal cases are unchanged, and the session's one new line is a diagnostic parse-rejected, not an events.ndjson kind
integration-test-strategy#5.1 · no · the CLI test list and its literals are unchanged, and diag-line.v1.json only gains the mcp-stdin parser value, so the stop case's schema check holds (CI green)
