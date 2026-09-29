
## 2026-09-29-fake-agent-drift-contract — The cross-session tag filed as harness origin
**Section:** Security Anti-Patterns → Code Patterns (the compiled-prefix ban); Security Decisions Log (`2026-09-29`, the cross-session-message tag)
**Change:**
- Code Patterns: the ban on building harness prefixes from runtime or upstream text stands; it now names the set — four compiled literals matched on the raw start, no trim (`<agent-message from=`, `<task-notification>`, `<\cross-session-message`, `<cross-session-message`) — and the accepted side effect: the escaped cross-session form is the one escaped tag that classifies `harness`, so a human who types it at a prompt's start is filed `harness`.
- Decisions Log: a `2026-09-29` entry records the decision, the boundary widening and its side effect, the relayed status of the escaped injection form, the condition (the first live test measures it and owns the ledger row) and the witnesses.
**Why:** a boundary widening of the classifier's `harness` class, ratified live by the founder on 2026-09-29 (relay: the Viola overseer) after the widening and its side effect were shown.
**Ref:** .andromeda/runs/2026-09-29T14-38-17-wrap/
