
## 2026-10-17-end-to-end-path-binary-and-harness-legs — obs: stall detection reads an absent instance as degraded
**Section:** §3 → Heartbeat ticks (Stall detection)
**Change:** `agent-run status` reads `state: degraded` when a booted instance's item is `stale` or absent, a gone instance being omitted from `list` (was: when any item is `stale`).
**Why:** The test-plan §3 ↔ obs-plan §3 bind: `status` now runs `viola list` and derives its state from it.
**Ref:** .andromeda/runs/2026-10-17T01-57-16-wrap/
