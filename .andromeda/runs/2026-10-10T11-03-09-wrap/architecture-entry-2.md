
## 2026-10-10-self-healing-state — the root waits read 23 sites in 17 files; tracing-subscriber's dev-dependents named
**Section:** §Occupied Resources → Filesystem · §Stack and Technologies (Logging) · §Infrastructure Patterns → Crate dependency direction
**Change:**
- Filesystem, the root watch: `Instant::now() + WITHIN` reads 23 sites in 17 files under `tests/`, as measured at this chunk's report (was 22 sites in 16 files). The pattern stays the count's rule; `tests/chaos_torn_append.rs` adds one site and one file.
- Stack, Logging row: the root bin is tracing-subscriber's only product dependent, and `viola-channel` and `viola-state` take it as a dev-dependency for their unit tests' line capture (was "root bin only").
- Crate dependency direction, the `viola-state` row: its dev-dependency beside rstest is tracing-subscriber at the workspace pin, for the `#[cfg(test)]` line capture; no product edge.
**Why:** the chunk added a root chaos test with one wait, and gave `viola-state` the test capture `viola-channel` already had. No product dependency moved and no crate entered the graph.
**Kept:** the same count in test-plan §3 → 5-command implementation moved in the same pass.
**Ref:** .andromeda/runs/2026-10-10T11-03-09-wrap/
