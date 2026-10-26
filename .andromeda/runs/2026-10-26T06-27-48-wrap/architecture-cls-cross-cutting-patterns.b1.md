# Classified — architecture · cross-cutting-patterns.b1

## Answers

cross-cutting-patterns#1.1 · yes · stale · the lock's tokio entry moved to 1.53.3, and the env-var rule covers any dependency that reads an environment variable, a point the report does not show for tokio 1.53.3
cross-cutting-patterns#2.1 · no · only tokio's pin and lock entry moved, no ledger row, probe or stamp moved, and tokio is built only in viola-mcp, viola-channel's optional feature and viola-e2e, not viola-agent-claude
