CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  none
  Tier 2 (.claude/rules/*):                   + testing.md: "A colour- or terminal-sensitive case sets or removes NO_COLOR, TERM and COLORTERM on each child it starts …" (confidence 0.8)
                                              + testing.md: "Write a literal of indented output lines with concat! … never a "…\n\ continuation …" (confidence 0.8)
                                              + host-linux.md: "An ad hoc viola list typed in the session's own shell reads the host's real claude agents --json …" (confidence 0.8)
  Tier 3 (.claude/docs/session-learnings.md): none
  Filters: 0 dup · 0 task-specific · 0 conflict · 0 deferred · 1 below threshold (the viola.exe usage word, 0.6)
  No-other-home: "colour-sensitive cases clear the inherited colour variables" · "concat! for indented literals" · "an ad hoc list reads the host's sessions"

Proof:
- testing.md / colour variables: verified by measurement (+0.4) — `chunks/2026-10-17-cli-output-discipline/evidence/red-03-remove-the-guard.txt`, the paste control's first firing 0 passed 8 failed under the host's `COLORTERM=truecolor`, its second (helper clearing the three) 5 passed 3 failed; specific detail (+0.2); no other home (+0.2): test-plan §5 records the one helper, not the rule.
- testing.md / concat!: verified by a real red (+0.4) — `evidence/red-01-help-on-the-current-tree.txt`, the `Options:` lines' leading spaces lost; specific detail (+0.2); no other home (+0.2).
- host-linux.md / ad hoc list: measured (+0.4) — the implement run's ad hoc `target/debug/viola --home … list` listed 11 host sessions (the outcome file, Decisions); specific detail (+0.2); no other home (+0.2). Tier: host/shell mechanics → the host leaf (tiebreaker 6), one sentence (the leaf is always loaded).

Decisions & corrections, each disposition:
- The founder's own word ratifies the env-read widening (`inputs#I2`) — not a learning: amended into the masters at P2 with its sidecars.
- PROVISIONAL Q2 / Q3 for the founder's next batch — not a learning: the handoff carries it.
- Owed at P5, the stale-dim integration witness (`inputs#I4`) — not a learning: P5's CARRY on "Linux and macOS parity".
- Measurement, inherited colour variables — applied (testing.md).
- Measurement, the per-verb usage names `viola.exe` on Windows — rejected (Filter 4: 0.6 — measured +0.4, detail +0.2; no-other-home not available, the fact is amended into layout-templates by P2).
- Hazard, the `\n\` continuation — applied (testing.md).
- Hazard, an ad hoc `viola list` reads the host's sessions — applied (host-linux.md).
- Guard refusals (a `cd` that moves the cwd, ×3 across implement and wrap; a heredoc to a file, ×1) — not a learning: each cost its one re-issue and the guard put the call back on the documented path; the standing rule (handoff, Standing rules) already names both.
- The operator's word for implement (no live start, no mutation run, the operator pass) — not a learning: a per-chunk direction, honoured.
