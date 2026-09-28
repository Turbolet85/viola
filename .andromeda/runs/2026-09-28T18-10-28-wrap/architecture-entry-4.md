
## 2026-09-28-capability-ledger-and-viola-verify — `StampError` named as an interim divergence from one enum per crate
**Section:** Established Decisions [Error Handling]; §Conventions Rust error types; Inherited Defaults (Errors)
**Change:** the one-enum-per-crate rule stands with its one exception (`PtyError`); `viola-agent-claude`'s `StampError` (`Malformed`, the stamps envelope) beside `AgentError` is named as an interim divergence, not an exception, which the "Verify-stamped test homes and harness" route entry folds into `AgentError`.
**Why:** the chunk landed a second enum where the plan read both ways; the overseer ruled at this wrap that the locked rule stands and the fold rides the route (a CARRY on that entry).
**Ref:** .andromeda/runs/2026-09-28T18-10-28-wrap/
