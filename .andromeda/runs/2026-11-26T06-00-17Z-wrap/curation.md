# Curation — gated wrap 2026-11-26T06-00-17Z (chunk 2026-11-25-placing-without-moving-the-view)

CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  none
  Tier 2 (.claude/rules/*):                   none new
  Tier 3 (.claude/docs/session-learnings.md): none
  Filters: 2 dup (1 extended in place · 1 recurrence → handoff) · 5 task-specific / not a learning · 0 conflict · 0 deferred
  Extended: T2/ci.md: "2026-10-13: Between the pre-CI commit and the push, write nothing into the tree" + "the gate tool fired with --run-dir is such a writer: fire the push entry with no --run-dir"
  CLAUDE.md size: 136/200 · T1 4.8 KB, 0 over 600 B

## Applied
- Extended T2 `.claude/rules/ci.md` (2026-10-13 entry), rec.py harness land tx 5102.
  Proof: `evidence/operator-pass-gated.md` — `19 probe       red · exit 0 ✗ (exit 1) · exit 1 · 0.01s · 0 B → 19.log`, fired `--operator 19 --run-dir .andromeda/runs/2026-11-26T05-27-23Z-implement`; the trail append into that run dir dirtied the tree the commit `8942ffbd` had just taken; the amend to `5ddedeff` and the re-fire with no run dir read green.

## Every Decisions & corrections entry, dispositioned
- report-gated: Bash guard refused `matrix.py implement --help | head` — not a learning (a guard put the call back on its documented path).
- report-gated: Bash guard refused a `cd` into `~/.claude/skills` — not a learning (guard; this wrap met the same arm twice, each re-issued once).
- report-gated: the `date -u` timestamp rule recurred (5th) — rejected, Filter 1 dup of CLAUDE.md's Tier 1 timestamp entry; a recurrence-despite-learning → handoff (Deferred learnings).
- report-gated: the operator's word for R7 came with a ten-minute window; R7 fired once inside it — not a learning (task-specific timing).
- After implement: entry 19's red (push entry fired with `--run-dir`) — extended (above).
- After implement: the `date -u` recurrence — same disposition as above.
- After implement: the guard refusals — not a learning (as above).
- After implement: directive item 2 read against the store, no row qualifies — not a learning (task-specific reading of one directive).
- This wrap: `cascade.py leaves` read the epoch `open` while this chunk's own `gated` record stood, so the whole mode was run by hand — not curated: a pipeline tool's behaviour, recorded as evolve telemetry (reconcile record), never a session learning.
