# Curation — wrap 2026-11-24T03-08-15Z-wrap · chunk 2026-11-24-viola-focus

CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  none
  Tier 2 (.claude/rules/*):                   + verification-harness.md: "A live-reading script prints verdict words from its plan's closed set alone: a state the set names no word for takes the set's nearest word (a hand wrapper never live reads `unread`), never a new word, and its rehearsal is fired again after the change."
    Proof: implement's Deviations and Decisions & corrections — `scripts/r3-focus.sh` first printed `hand=not-live`, a word outside the plan's set; it was replaced by `unread` before any graded use and entries 7 and 8 (the syntax check and the rehearsal) were fired again green (`outcome-2026-11-24-viola-focus.md`, run 2026-11-24T02-28-24Z-implement); the implement directive's «no new closed word beyond the plan's» (`inputs#I5`). Score: "never" language +0.3, verified by measurement (a word replaced before graded use) +0.4 = 0.7.
  Tier 3 (.claude/docs/session-learnings.md): none
  Filters: 1 dup · 0 task-specific · 0 conflict (→ handoff) · 0 deferred (→ handoff) · 2 below threshold
  CLAUDE.md size: 136/200 · T1 4.8 KB, 0 over 600 B

## Every candidate, its disposition
- implement D&C «the directive's host rule stopped the three whole layers … the overseer's quiet window resumed them» — not a learning: a directive's run-time order for that run, held by the overseer's record (`inputs#I5`, `inputs#I6`).
- implement D&C «`hostwatch.py read --to now` is refused; `--to $(date -u +%FT%TZ)` reads it» — rejected, Filter 4: specific technical detail +0.2, proven by a real refusal +0.4 = 0.6, exactly the threshold (rejects); the reader's usage line in verification-harness.md §Host pressure names `--from <UTC> --to <UTC>` already.
- implement D&C «Bash guard refusals, each re-issued once: a session `cd`; a `cat` heredoc appending to a source file» — duplicate, Filter 1: host-linux.md (§Paths & the working directory; Session Additions 2026-09-28, heredoc to a file) states both; the guard put each back on the documented path at its one re-issue, no recurrence.
- implement D&C «A verdict word a live script prints joins only from the plan's own set» — applied, Tier 2 verification-harness.md (above).
- After implement D&C: none during the pass; the wrap directive's rulings (`inputs#I7`) — not a learning: this chunk's fork landings, carried by the sidecar entries and the handoff.
- This window: the plan's expected amendment put focus on security-plan `decision/state-tree-strict-modes-before-use` (the home-tree check); the code runs `check_instance` through `client::refuse_untrusted`, caught at the leaf decision and corrected (tx 4473–4474) — rejected, Filter 4: verified by measurement +0.4, specific technical detail +0.2 = 0.6, exactly the threshold; its fact is amended into the masters this wrap (security-plan `decision/cli-frame-verbs-strict-modes-first`), so neither conditional signal applies.
- This window: a stray `cat > /dev/null` in a Bash call waited on stdin to its 120 s bound — not a learning: a one-off slip of mine, no rule it would follow.
