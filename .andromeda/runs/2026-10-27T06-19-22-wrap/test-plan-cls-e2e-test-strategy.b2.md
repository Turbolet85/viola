# Classified — test-plan · e2e-test-strategy.b2

## Answers
e2e-test-strategy#6.4 · no · path 5's cleanup names only the outer PTY, child.wait and TempDir drop, no reached file, and the task's Untouched list leaves paths 2 to 7 unchanged
e2e-test-strategy#7.1 · no · path 6's steps name no reached file, and its statusline pass-through shares only the word passthrough with the chaos change
e2e-test-strategy#7.2 · no · its budget.json and statusline signals name no reached file, and replace_private_shared's outcome is unchanged for equal, longer, shorter, different and absent targets
e2e-test-strategy#7.3 · no · its MCP form in the reached mcp_tools.rs still holds, since no tool, handler or refusal changed and lines under MAX_FRAME pass the capped reader unchanged
e2e-test-strategy#7.4 · no · cleanup names only the harness cleanup, no reached file
e2e-test-strategy#8.1 · no · path 7's steps name no reached file, and the rmcp answer call passes the capped stdin reader unchanged below MAX_FRAME with no tool changed
e2e-test-strategy#8.2 · no · the answer refusal and the stamped verify line name no reached file, and no MCP handler or tools/list changed
e2e-test-strategy#8.3 · no · the reached mcp_tools.rs still holds the path-7 MCP leg, since CI ci#40458923095 is green on three OSes for the changed tree and no tool or harness verb changed
e2e-test-strategy#8.4 · no · cleanup names only the harness cleanup and TempDir drop for H3, no reached file
e2e-test-strategy#9.1 · no · the R8 env-strip steps name no reached file, and no env var was added, so the 11 identity-floor names are unchanged
e2e-test-strategy#9.2 · no · the receipt, process-start and random-canary secret scan name no reached file, and the task leaves E1 to E5 untouched
e2e-test-strategy#9.3 · no · cleanup names only TempDir drop, no reached file
e2e-test-strategy#10.1 · no · E3 names only tests/cli_links.rs cases and the link and unlink records, no reached file, and the task names no change to link or unlink
e2e-test-strategy#11.1 · no · E5's replay readers name no reached file, and the task leaves paths 2 to 7 and E1 to E5 untouched
e2e-test-strategy#12.1 · yes · add-to · the MCP negatives gain a stdin line past MAX_FRAME that ends the session with one schema-valid mcp-stdin oversize line (F3), and the cut before initialize (exit 11) that no test drives (F8)
e2e-test-strategy#13.1 · yes · add-to · the new mcp_tools oversize case asserts its canary is in no home-level file, so no byte of the over-long line reaches disk (F3)
e2e-test-strategy#14.1 · no · the exit-cause matrix names exit 21 and exit 1 causes that live in no reached file, and the task adds no exit code, refusal or error variant
e2e-test-strategy#15.1 · yes · stale · the Windows --home case rests on viola_state::fs::create_private_dir in fs.rs, a file the replace_private_shared change reaches whole, and the task's report covers only that compare, not this claim
e2e-test-strategy#16.1 · no · the list row cases name no reached file, and the task names no change to list
e2e-test-strategy#17.1 · yes · add-to · the forced vt100 panic case in the reached tests/chaos_feed_panic.rs now also asserts passthrough reaches the outer terminal (Unix bytes e4 b8 ad, no viola literal on any OS), and its Windows byte oracle is owed (F9)
