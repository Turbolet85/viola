# Classified — tests · e2e-test-strategy.b2

## Answers
e2e-test-strategy#6.4 · no · Path 5 cleanup of a CLI PTY scenario; the task's Not needed holds the critical-path procedures to a workspace re-run, and no MCP call or Tokio path is named
e2e-test-strategy#7.1 · no · Path 6 steps are CLI and statusline-hook calls (send, release, list --json); no MCP call or Tokio path is named
e2e-test-strategy#7.2 · no · Path 6 verification signal reads budget.json and CLI documents; the -32602 release refusal is a channel method, not one of the MCP tools
e2e-test-strategy#7.3 · yes · the MCP form of the budget refusal (`mcp_tools_path6_budget_paused_send`: `isError:true`, `{"refusal":"budget-paused","detail":"five-hour"}`) runs on the E2E MCP stdio path the bump reaches
e2e-test-strategy#7.4 · no · harness cleanup of a CLI scenario; no MCP call or Tokio path is named
e2e-test-strategy#8.1 · yes · step 3's rmcp `answer` call drives the E2E MCP stdio path (Tokio-bearing) that the chunk re-runs
e2e-test-strategy#8.2 · yes · the MCP `answer` result (`is_error:true`, text `refused: unverified-cli`) is checked on the E2E MCP stdio path the bump reaches
e2e-test-strategy#8.3 · no · cleanup of a CLI scenario (harness cleanup, TempDir drop); no MCP call or Tokio path is named
e2e-test-strategy#9.1 · no · R8 env strip of `viola run` with the fake agent, a CLI-only step; no MCP call or Tokio path is named
e2e-test-strategy#9.2 · no · R8 env strip signals on the claude child's receipt and its `process-start` record; no MCP call or Tokio path is named
e2e-test-strategy#9.3 · no · cleanup of the R8 scenario (TempDir drop); no MCP call or Tokio path is named
e2e-test-strategy#10.1 · no · E3 link scenario on the CLI and the wrapper's `-32601` for the unbuilt `link` / `unlink` methods; no MCP tool (send, wait, last, answer, list) or Tokio path is named
e2e-test-strategy#11.1 · no · E5 replay recovery through `revive`, `list --json` and the budget and link legs, all CLI; no MCP tool or Tokio path is named
e2e-test-strategy#12.1 · yes · names the MCP schema-violation handler test `handlers_schema_violations_write_no_frame` in `crates/viola-mcp/tests/handlers.rs` (the TokioClient path) and the E2E `mcp_tools_list_is_exactly_the_five_tools_and_list_matches_the_cli`
e2e-test-strategy#13.1 · yes · the MCP `isError` and `-32603` bodies carry codes and fixed messages only, checked after each E2E test on the E2E MCP stdio path the bump reaches; the canary and junit-scan conventions are for new tests the chunk does not add
e2e-test-strategy#14.1 · no · the exit-cause matrix is CLI-only (exit 21 and exit 1 causes); the task's Not needed holds it to no case added or changed, and no MCP or Tokio path is named
e2e-test-strategy#15.1 · yes · the single-writers rule names `mcp` tool calls, which must write neither `stamps.json` nor `snapshot.json` on the Tokio-bearing MCP path; the other negatives sit on the sync channel server
e2e-test-strategy#16.1 · no · the list row-integrity rules cover the CLI rendering and `list --json`; no MCP check or Tokio path is named, and the Not needed names the suite
e2e-test-strategy#17.1 · no · chaos cases on the CLI verbs and the statusline hook; no MCP check or Tokio path is named, and the Not needed names the suite
e2e-test-strategy#18.1 · no · proptest over sync parsers and `validate_paste_text` in `viola-core`, which the bump does not reach; the Not needed names the suite
e2e-test-strategy#19.1 · no · the contract suite's fake-agent order, schema and `sha2` hash checks; no fixture is recorded, `sha2` is not bumped, and no MCP or Tokio path is named
