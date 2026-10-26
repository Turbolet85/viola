# New groups — architecture · standard-contracts

## Verdicts
- fanout-architecture--cascade-new-standard-contracts.md 1 · distinct · no standing group names `LiveEndpoint`, `ServerVerdict`, `server_verdict` or `verify_server`; the only session-liveness group (lines 85–89) holds the `claude agents --json` read alone
- fanout-architecture--cascade.md 14 · distinct · no standing line states a pre-frame server check on any channel method; the `send` item (line 47) covers the budget gate and links only, and line 25's `read_snapshot` callers note is no server-identity claim
- cascade-compare-architecture.md 2 · twin · fanout-architecture--cascade.md 14 · it states the same fact, that every frame-writing client (the six CLI verbs, `hook.event` / `hook.dialog`, the MCP frame tools) compares the peer pid and OS start time with the snapshot's `pid` / `started_at` from one read before its first frame and writes none on a mismatch, macOS a recorded gap; it lands under contract-instance-snapshot rather than the `send` item, and its API names also repeat proposal 1's
