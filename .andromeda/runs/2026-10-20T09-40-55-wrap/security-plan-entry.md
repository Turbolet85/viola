
## 2026-10-20-epoch-5-cleanup — the e2e-home keepers refuse a backing below the headroom floor
**Section:** §Security Anti-Patterns (the test-side removal guard; the two keepers)
**Change:** The two keepers also refuse a linked base whose filesystem reads less than the 9 GiB headroom floor free (`statvfs`). Their refusal stays a fixed message that names no path; the headroom one holds the two counts (free and floor, in MiB). Was: absolute target, a real owner-only directory, no target that is itself a link; now that, and the floor.
**Why:** a refusal added on the backing the keepers already reach. It reads filesystem metadata only, creates and removes nothing, and widens no boundary. The floor is a compiled constant; no env var, flag or config key sets it.
**Ref:** .andromeda/runs/2026-10-20T09-40-55-wrap/
