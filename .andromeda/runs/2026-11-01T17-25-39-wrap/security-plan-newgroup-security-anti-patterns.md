# New groups — security-plan · security-anti-patterns

## Verdicts
- cascade-compare-security-plan.md 4 · contradiction · lines 9–9 · «only every client's euid check and macOS's pid and start-time check stay owed» · the file holds every client's euid check as still owed, while the proposal has `peer_admitted` wired at each client seam (`cmd::client::connect`, `cmd::hook::connect`, `viola-mcp`'s `frame`); the injected-proof and no-seam fact itself stands nowhere, line 39's general seam ban not naming it
