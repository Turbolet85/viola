
## 2026-10-04-running-turn-refusal — the fake agent quiesces its hooks before it exits
**Section:** §7 Test Data & Fixtures → Fake agent
**Change:** was "It exits on `\x03`."; now it exits on `\x03` or at stdin EOF and, before exiting, waits for a hook still running and starts no other — as measured at this chunk.
**Why:** the agent is its PTY's session leader and its hooks sit in that terminal's foreground group, so its exit hung up an in-flight `viola hook` mid-exit: a truncated coverage profile, the `.profraw` WATCH's cause. Folded as a recorded widening on the overseer's founder-delegated word (provisional per the delegate rule); the WATCH closed on four consecutive green pre-push runs after the fix.
**Ref:** .andromeda/runs/2026-10-04T22-27-20-wrap/
