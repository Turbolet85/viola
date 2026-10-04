
## 2026-10-04-readiness-gate-and-timing-constants — vt100 in viola-agent-claude, the new modules and the fourth fuzz target
**Section:** §Infrastructure Patterns → crate-dependency-direction, project-directory-structure · §Occupied Resources (`fuzz/`)
**Change:**
- crate-dependency-direction: `viola-agent-claude` depends, as landed, on vt100 `=0.16.2` (the pure `screen` module); its vt100 line reads "`run`'s feed thread feeds it a copy of the pump output" (was "`run`'s pump feeds it bytes"); the closing sentence no longer lists vt100 as arriving later.
- project-directory-structure: `src/run/` names `gate.rs` (the pump-output tee + vt100 feed thread); `viola-core/` lists `SPINE_DEADLINE`, `Clock` / `SystemClock`; `viola-agent-claude/` names the vt100 screen model (`screen`) in place of "screen signatures"; `fuzz_targets/` adds `vt100_feed`.
- `fuzz/` row: targets `viola_name`, `channel_frame`, `hook_stdin` and `vt100_feed`; `vt100_feed` has 6 synthetic seeds.
**Why:** vt100 0.16.2 entered the graph through `viola-agent-claude` only, and the chunk added `src/run/gate.rs`, the `screen` module, the two viola-core items and the `vt100_feed` target.
**Ref:** .andromeda/runs/2026-10-04T05-25-03-wrap/
