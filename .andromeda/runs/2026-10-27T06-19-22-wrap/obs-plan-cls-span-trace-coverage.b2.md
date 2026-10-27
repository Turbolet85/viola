# Classified — obs-plan · span-trace-coverage.b2

## Answers
span-trace-coverage#7.4 · no · the wait.block close paths hold on the changed crates/viola-mcp/src/lib.rs, which the session changes only at its stdin reader and session end, and a cut ends the session as a closed stdin already does
span-trace-coverage#8.1 · no · the MCP answer's instance check order and the implicit link record hold on the changed crates/viola-mcp/src/lib.rs, which the session changes only at its stdin reader and session end
span-trace-coverage#8.2 · no · the dialog spans and their required attributes are unchanged, the session adding no span or attribute
span-trace-coverage#8.3 · no · a refused MCP answer still writes no line on the changed crates/viola-mcp/src/lib.rs, and the session's new mcp.ndjson line is written by serve at session end, not by answer
span-trace-coverage#8.4 · no · the dialog-expired cleanup and its expiry line sit in the wrapper's dialog path, which no reached file holds
span-trace-coverage#9.1 · no · pause and release are CLI wrapper paths with no MCP tool and no reached file, so the wheel transition and its server-check order hold
span-trace-coverage#9.2 · no · run.wheel_transition and its attributes are unchanged, the session adding no span or attribute
span-trace-coverage#9.3 · no · the send-refused, release-from-driver, budget-gate and release channel lines sit in the wrapper's wheel path, which no reached file holds
span-trace-coverage#9.4 · no · run.wheel_transition stays a point-in-time span and the wheel product event is unchanged
span-trace-coverage#10.1 · no · the budget governor's spans hold, since the fs.rs edit (a bounded compare in replace_private_shared) adds no event, field or detail code
span-trace-coverage#10.2 · no · state.budget_write and the other budget attributes are unchanged, since the fs.rs edit adds no event, field or detail code
span-trace-coverage#10.3 · no · the statusline lines hold and the schema still admits hook-stdin beside the session's new mcp-stdin parser value
span-trace-coverage#10.4 · no · the statusline shell-out cleanup and its deadline sit in src/cmd/hook.rs, which no reached file holds
span-trace-coverage#11.1 · no · the R8 strip and its env-name attributes sit in the child spawn path, which no reached file holds
span-trace-coverage#12.1 · no · the implicit link record and its line sit in the wrapper's send and answer handling, which no reached file holds
span-trace-coverage#13.1 · no · link and unlink are wrapper channel methods, which no reached file holds
span-trace-coverage#14.1 · no · the state-recovered lines and their callers hold on the changed crates/viola-mcp/src/lib.rs, since the session's new mcp.ndjson line is parse-rejected and E5 replay is unchanged
span-trace-coverage#15.1 · no · the torn-append heal and its state-recovered line are unchanged, and the fs.rs edit adds no event or field
span-trace-coverage#16.1 · no · the ledger read, its strict-modes refusal and its ledger-stamps parse-rejected lines hold, and the schema still admits ledger-stamps beside mcp-stdin
span-trace-coverage#17.1 · no · verify and plugin install keep their spans, eight spawn pairs and home-tree checks, and the fs.rs edit adds no event, field or detail code to any replace
