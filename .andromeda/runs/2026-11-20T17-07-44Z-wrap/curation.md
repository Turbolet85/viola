CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  none
  Tier 2 (.claude/rules/*):                   + testing.md: "A remove-the-guard control over a doubled guard … cannot read red by removing one of them …"
                                              + testing.md: "A new booted case belongs to a gate entry's filter in the plan's own phase …"
                                              + verification-harness.md: "A plan's non-leg entry that reads a file a round entry writes … is red by construction in any block run before the round …"
  Tier 3 (.claude/docs/session-learnings.md): none
  Filters: 0 dup · 3 task-specific · 0 conflict (→ handoff) · 0 deferred (→ handoff)
  CLAUDE.md size: 136/200 · T1 4.8 KB, 0 over 600 B

## Applied
- testing.md (tx 3520) — the doubled-guard control.
  Proof: report Decisions & corrections «A remove-the-guard control over a doubled guard …»; Deviations 1; `evidence/control-1.txt` (the `attempted-arm` control could read red only once the under-lock re-read was narrowed, then `holds`).
- testing.md (tx 3520) — a new booted case in a phase entry's filter.
  Proof: report Decisions & corrections «A new booted case that only the workspace layer selects …»; Outcome «`bash scripts/agent-run.sh run --integration` — red at the first firing (cases 15–16), green after the fix».
- verification-harness.md (tx 3521) — a non-leg entry reading a round's file.
  Proof: report Decisions & corrections «A plan's non-leg entry that reads a file a round entry writes …»; outcome file «entry 23 (`live-stamp: unmoved`) … a bare block run before the round reads it red by construction».

## Every Decisions & corrections entry, dispositioned
implement's:
- The host read CONTENDED before the round; the overseer's word to go on over a later window — not a learning: a ruling of this run, task-specific (Filter 2); the host rule already stands (verification-harness 2026-11-19 CONTENDED entry).
- `hostwatch.py read --to now` refused — rejected, Filter 2: a tool of the overseer's tree outside this repository, no home here.
- A non-leg entry reading a round entry's file — applied (verification-harness.md).
- The Bash guard refused a heredoc append, a `cd`, a pipe into `grep` — not a learning: each act the guard put back on the documented path (host-linux.md and the guard already state them).
- A remove-the-guard control over a doubled guard — applied (testing.md).
- A new booted case only the workspace layer selects — applied (testing.md).
After implement's:
- The pass: none.
- The wrap directive (inputs#I5) — rejected, Filter 2: this wrap's own instructions (the away form, the answer card's bounds, the leaves' content), not a project learning.
This window:
- Proposers of several masters offering one new fact, and sets dropping a qualifying word — not a learning: pipeline telemetry (the friction family's, recorded at the reconcile checkpoint).
