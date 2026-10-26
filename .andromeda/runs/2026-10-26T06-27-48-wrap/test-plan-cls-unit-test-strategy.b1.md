# Classified — test-plan · unit-test-strategy.b1

## Answers
unit-test-strategy#1.1 · no · the session moves only the tokio pin and its lock entry, and no viola-channel unit case here names tokio, a manifest line or a moved dependency
unit-test-strategy#1.2 · no · viola-state is a sync crate with no tokio dependency, so the tokio-only lock delta reaches none of its unit cases
unit-test-strategy#1.3 · no · viola-agent-claude is a sync crate with no tokio dependency, and its unit cases name no version or manifest line the session moved
unit-test-strategy#2.1 · no · the root bin's unit cases name no tokio version or manifest line, and the viola-mcp start_paused tests it cites are the MCP adapter's own, while the pin move changes no manifest feature
