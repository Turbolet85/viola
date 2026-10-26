# Classified — tests · e2e-test-strategy.b1

## Answers
e2e-test-strategy#1.1 · no · the cli drivers and outer-PTY SGR checks are tokio-free and outside the bump's reach
e2e-test-strategy#1.2 · no · the hook surface is sync with no Tokio step and no hook code change, so the bump does not reach its env_clear rule
e2e-test-strategy#1.3 · no · the outer PTY is portable-pty over the sync viola-pty seam, which the bump does not reach
e2e-test-strategy#1.4 · no · the wrapper channel's sync client is tokio-free and its endpoint read is unchanged; the bump reaches only the Tokio client behind viola-channel's feature
e2e-test-strategy#1.5 · yes · the rmcp client over a viola mcp child is the Tokio-bearing MCP stdio path the bump reaches, and names viola-e2e's client-only rmcp feature set
e2e-test-strategy#1.6 · no · standing S2 · each tool runs at the one version ci.yml pins, a convention the task lists
e2e-test-strategy#2.1 · no · the run start sequence is sync CLI, PTY and channel steps with no rmcp or Tokio step, re-run only inside the workspace suite
e2e-test-strategy#2.2 · no · its first-records, snapshot, plugin and settings.json signals are sync run and list behaviour with no MCP step
e2e-test-strategy#2.3 · no · the as-landed root-chain placement and nextest kill-line timings are not reached by the bump, and the flow has no MCP step
e2e-test-strategy#2.4 · no · the path-1 cleanup (wrapper kill, TempDir removal, squatter drop) is sync and reached by no Tokio path
e2e-test-strategy#3.1 · yes · the MCP send step in viola-e2e's mcp_tools.rs (rmcp client over a viola mcp child) is the Tokio-bearing path the bump reaches
e2e-test-strategy#3.2 · yes · step 5, the rmcp client's call_tool("send") over a viola mcp child, is the Tokio-bearing MCP stdio path the bump reaches
e2e-test-strategy#3.3 · yes · its MCP bullet (is_error:false; structured_content keys equal the CLI ok keys) is the rmcp-client path the bump reaches
e2e-test-strategy#3.4 · no · the path-2 cleanup is the harness's agent-run cleanup, with no MCP or Tokio step
e2e-test-strategy#4.1 · yes · its MCP wait and last steps in mcp_tools.rs and the MCP wait's 50 000 ms clamp (a Tokio timer, left as it is) are the Tokio-bearing paths the bump reaches
e2e-test-strategy#4.2 · yes · steps 6 and 7 (rmcp call_tool("last"); rmcp wait on a dead instance) are the Tokio-bearing MCP stdio path the bump reaches
e2e-test-strategy#4.3 · yes · its live MCP wait wake and MCP is_error:true refusal are the rmcp-client path the bump reaches
e2e-test-strategy#4.4 · no · the path-3 cleanup (harness cleanup and sysinfo checks of killed wrappers) has no MCP or Tokio step
e2e-test-strategy#5.1 · yes · the MCP answer step in mcp_tools.rs (rmcp client over a viola mcp child) is the Tokio-bearing path the bump reaches
e2e-test-strategy#5.2 · yes · step 5, rmcp call_tool("answer") on a fresh dialog, is the Tokio-bearing MCP stdio path the bump reaches
e2e-test-strategy#5.3 · yes · its MCP is_error:false signal is the rmcp-client path the bump reaches
e2e-test-strategy#5.4 · no · the path-4 cleanup is the harness cleanup, with no MCP or Tokio step
e2e-test-strategy#6.1 · yes · the MCP send step and its refusal check in mcp_tools.rs are the Tokio-bearing path the bump reaches
e2e-test-strategy#6.2 · yes · step 3, rmcp send after the CLI send, is the Tokio-bearing MCP stdio path the bump reaches
e2e-test-strategy#6.3 · yes · its MCP refusal (is_error:true with refused: human-typing; no release in the MCP text or structured_content) is the rmcp-client path the bump reaches
