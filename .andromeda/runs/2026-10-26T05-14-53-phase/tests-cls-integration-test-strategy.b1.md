# Classified — tests · integration-test-strategy.b1

## Answers
integration-test-strategy#1.1 · no · its module ↔ module calls (viola-agent-claude, viola-core, viola-state) hold no tokio, and its suite re-runs only inside the workspace gate
integration-test-strategy#1.2 · yes · the mcp.ndjson check runs two `viola mcp` servers, the Tokio-hosted process the bump reaches, so its one-JSON-object-per-line rule binds the bumped lock
integration-test-strategy#1.3 · no · the `claude` child is a std spawn the bump does not touch, and the entity names no Tokio-hosted caller
integration-test-strategy#1.4 · no · the endpoint, socket-dir, pipe-DACL and squatter checks run on the sync channel server, which holds no tokio
integration-test-strategy#1.5 · no · the PTY seam (viola-pty over portable-pty `=0.8.1`) holds no tokio and the bump does not reach it
integration-test-strategy#1.6 · no · `viola hook` and the live test wrapper are sync, and no hook code changes
integration-test-strategy#2.0 · no · the per-test home, the e2e-home link and the carve-outs are a convention every new test keeps, and the chunk adds no test or home
integration-test-strategy#3.1 · yes · its MCP half (cross_exit_causes.rs, the rmcp client over a `viola mcp` child) is the E2E MCP stdio path the bump reaches, so the `instance-unreachable` result must hold on the bumped lock
integration-test-strategy#3.2 · no · the round-trip, ts, tailing and strict-modes cases touch only viola-state and sync verbs, and the mcp refusal exits at its home check before viola_mcp::serve
integration-test-strategy#4.1 · no · its CLI suites drive sync verbs the bump does not reach, and its one mcp mention is the plugin install registration file check, not a run of viola mcp
