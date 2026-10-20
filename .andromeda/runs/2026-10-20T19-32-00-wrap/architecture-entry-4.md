
## 2026-10-20-epoch-5-cleanup-ii — the test-only child marker and the orphans probe's two cases
**Section:** §Occupied Resources (Environment variables) · §Infrastructure Patterns → `build-system` (the orphans gate's `--probe`)
**Change:**
- `MCP_SERVE_TEST_CHILD`: a test-only marker `viola-mcp`'s unit tests set by `Command::env` on the child they spawn, read only by that crate's `#[cfg(test)]` child entry `serve_child_entry`; read by no `viola` build, outside the `VIOLA_*` namespace. Was: unregistered.
- `scripts/orphans-check.sh --probe` fires on each of its two planted orphans. Was: "fires on a planted orphan".
**Why:** the child entry kills `serve`'s survivor inside its own crate; `VIOLA_*` is product plumbing, so the marker carries no prefix. The probe has held two planted cases since before this chunk; its log reads `2/2 fired`.
**Kept:** `PTY_SIDELOAD_TEST_CASE`, the precedent the marker follows, stays unregistered here: it is not this chunk's.
**Ref:** .andromeda/runs/2026-10-20T19-32-00-wrap/
