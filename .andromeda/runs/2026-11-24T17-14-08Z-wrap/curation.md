# Curation — wrap 2026-11-24T17-14-08Z-wrap · chunk 2026-11-24-viola-focus

CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  none
  Tier 2 (.claude/rules/*):                   + host-linux.md: "On this host's Hyprland 0.56.2 the seam's window move takes the ACTIVE workspace with the moved window …"
  Proof: R4a, `.andromeda/cache/chunks/2026-11-24-viola-up/evidence/r4.md` — active workspace 1 before, 5 after («R4a's own move; no dispatch of this session's»); the move's argv `hl.dsp.window.move(…)` in `crates/viola-hyprland/src/lib.rs` (`Request::MoveToWorkspace`); the fixture `fixtures/hyprland/0.56.2/MoveToWorkspace.recorded-1.json` keeps argv and `ok` only.
  Tier 3 (.claude/docs/session-learnings.md): none
  Filters: 2 dup · 3 task-specific · 0 conflict · 0 deferred
  CLAUDE.md size: 136/200 · T1 4.8 KB, 0 over 600 B

## Decisions & corrections, each weighed
Implement's (report-gated):
- How R3 ran against the block's words — not a learning: task-specific (this block's clearance), recorded in the ledger notes.
- No behaviour change between 69b5af50 and 85898007 — not a learning: task-specific.
- Entry 8 skipped on the invocation's word — not a learning: task-specific (one invocation's wording).
- Recurrence of the CLAUDE.md learning «A timestamp written into any record … is read from `date -u`» — rejected, Filter 1 dup; a DEFECT recurrence → `recurrence-despite-learning`, carried to the handoff's deferred list (a fourth recurrence).
- Guard refusals (`cd`, a pipe into `head`) — rejected, Filter 1 dup of host-linux.md «Paths & the working directory» and the andromeda-tools guard; this wrap met the same two refusals, each re-issued once.
After implement's:
- The directive's brief — not a learning: task-specific.
- R4a's active workspace — applied (Tier 2, host-linux.md above).
- The rehearsal entry may fire where the gate fires it — not a learning: one invocation's word.
