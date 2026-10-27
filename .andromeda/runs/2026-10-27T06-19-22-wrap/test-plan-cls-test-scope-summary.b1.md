# Classified — test-plan · test-scope-summary.b1

## Answers
test-scope-summary#1.1 · no · no channel file is reached and no dependency moved, so its framing, MAX_FRAME coverage and endpoint hash still hold on the changed files
test-scope-summary#2.1 · no · no agent-claude file is reached, and deny.toml's unbounded_depth ban keeps the default serde_json depth, so its parsing and probe claims still hold
test-scope-summary#3.1 · no · the session adds no exit code, refusal or error variant (mcp exits 0 or 11, the existing closed-stdin path), and the hidden mcp verb is not in this list
test-scope-summary#4.1 · no · the listed verbs, their --json documents and exit codes are unchanged, and the one verb the session touches, the hidden mcp, is not among them
test-scope-summary#5.1 · yes · add-to · tests/chaos_feed_panic.rs now witnesses passthrough continuing to the outer terminal after a forced vt100 panic (child bytes e4 b8 ad on Unix, no viola literal on any OS), which this passthrough surface's signals do not name
test-scope-summary#5.2 · no · no wrapper-channel file is reached, the stdin cap is MCP-only, and the channel's methods, refusals and -32xxx codes are unchanged
test-scope-summary#5.3 · yes · add-to · the stdio server's stdin is now capped at MAX_FRAME, so an oversize line passes no byte and ends the session with one parse-rejected mcp-stdin line (mcp_tools drives it), which this surface's stdin text does not carry
test-scope-summary#6.1 · no · the §3 harness contract is untouched (no verb, flag, usage detail, session-record field or status shape changed), the new binary runs under the existing run --e2e, and the logs claim still holds on the diag-line schema whose parser enum only grew by mcp-stdin
test-scope-summary#7.1 · no · no fixture, fake script, Mcp. exchange or fake-agent mode is added or recorded, the new mcp_tools case starts from its own fresh home as this strategy states, and no generator is added or removed
test-scope-summary#8.1 · no · Path 1's text is untouched (its WATCH tally lives on the route) and the run start steps are unchanged, and the replace_private_shared compare change in fs.rs keeps its outcome for equal, longer, shorter, different and absent targets, so the plugin/ rewrite still holds
test-scope-summary#8.2 · no · Path 2 is untouched, and a send line within the cap still passes whole through the capped stdin reader, so its MCP send claims still hold
test-scope-summary#9.1 · no · Path 5 is untouched, its MCP send refusal still passes a fitting line whole through the capped stdin reader, and the chaos change is an output-side witness that leaves typing as it was
test-scope-summary#9.2 · no · Path 6 is untouched (its statusline pass-through rows share only the word passthrough with the chaos change), and the fs.rs compare change keeps its outcome, so any budget.json write through it still holds
test-scope-summary#10.1 · no · its required chaos rows (torn ndjson line, corrupt snapshot, child exit without EOF, endpoint vanish, stale heartbeat) are not the feed-panic test the session changed, and the fs.rs compare change keeps its outcome, so atomic writes still hold
test-scope-summary#11.1 · no · its required contract suite is the fake-agent-against-fixtures contract, unchanged with no fixture, fake script or fake-agent mode touched, and the new contract_bounded_readers binary is a reader census, not a sandbox contract
