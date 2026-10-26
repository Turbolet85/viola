# Classified — tests · test-scope-summary.b1

## Answers
test-scope-summary#1.1 · yes · the Tokio client feature, which the chunk re-reads for containment (crates/viola-channel/Cargo.toml, channel-tokio-free.sh) and which viola-mcp's handler tests drive through TokioClient
test-scope-summary#2.1 · no · a sync crate no edit or named Tokio path reaches; its parsers, mapping, ledger and version gate are unchanged and no Claude-specific shape changes (Not needed)
test-scope-summary#3.1 · no · the sync CLI verb contract (exit codes, --json) is not reached by the bump, since tokio sits only in viola-mcp and the hidden mcp verb, which this verb list omits; its suites re-run only in the workspace gate
test-scope-summary#4.1 · no · the cli surface drives the sync verbs through assert_cmd and reaches no Tokio path; its suites re-run only in the workspace gate
test-scope-summary#5.1 · no · the run wrapper's PTY passthrough, wheel and events are sync (Tokio only in viola-mcp) and no step or signal of this surface changes
test-scope-summary#5.2 · yes · the result, refusal and protocol-fault shapes that the TokioClient in viola-mcp's handler tests (named in the Needs) receives from the wrapper channel
test-scope-summary#5.3 · yes · the viola mcp stdio server is the Tokio-bearing surface the bump moves, and its tools/list, isError and -32603 rules are asserted by the named E2E rmcp-client path, the handler tests and mcp_recorded_exchange
test-scope-summary#6.1 · yes · the run verb's suite order and exit-0 rule are the gate the chunk re-runs (agent-run run --unit/--integration/--e2e/--coverage, ending in viola-harness gate --require), while boot, status, cleanup and logs are not reached
test-scope-summary#7.1 · no · the chunk adds, records and scrubs no fixture and seeds no state, and per-test home and fixture hygiene are conventions each new test owes, with no new test unless a forced source change needs one
test-scope-summary#8.1 · no · run start (critical Path 1) is a sync wrapper path the bump does not reach, and the critical paths are Not needed (no step or signal changes)
test-scope-summary#8.2 · no · the confirmed-send critical path (Path 2) is Not needed (no step or signal changes) and its CLI send is sync; its MCP-send line is carried by test-scope-summary#5.3
test-scope-summary#9.1 · no · the human-takes-the-wheel critical path (Path 5) is Not needed and is sync wrapper behaviour; its MCP refusal line is carried by test-scope-summary#5.3
test-scope-summary#9.2 · no · the budget-governor critical path (Path 6) is Not needed and runs through the sync statusline hook and the wrapper's send gate, which the bump does not reach
test-scope-summary#10.1 · no · the chaos suite is Not needed (no case added or changed); its MCP instance-unreachable line is carried by test-scope-summary#5.3
test-scope-summary#11.1 · no · the fake-agent contract suite and its recorded fixture sets are Not needed (no case added or changed); the MCP recorded exchange is carried by test-scope-summary#5.3
