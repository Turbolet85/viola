# Curation — 2026-11-18-the-2-1-325-recorded-fixture-set

CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  none
  Tier 2 (.claude/rules/*):                   + verification-harness.md: "Before a live round the token window is the rig's own check (its live arm refuses with under 20 minutes left), never a hand read of the real credentials file …" (confidence 0.7)
  Proof: the report's Deviations and `## After implement` → Decisions & corrections: the builder read the real credentials file's `expiresAt` by hand before the round, a SECOND read beside the rig's own, outside the letter of the founder's cap «READ once» (inputs#I2 answer 2); the overseer's wrap directive names it a curation candidate (inputs#I4 item 2). Signals: correction relayed by the overseer +0.4 · «never» language +0.3. Landed tx 2885.
  Tier 3 (.claude/docs/session-learnings.md): none
  Filters: 1 dup (recurrence) · 0 task-specific · 0 conflict · 1 below threshold · 0 deferred (cap)
  CLAUDE.md size: 134/200 · T1 3.9 KB, 0 over 600 B

Dispositions of every *Decisions & corrections* entry:
- implement — the Edit anchored on text the rustfmt PostToolUse hook had reflowed missed: rejected — Filter 1, duplicate of a DEFECT record (testing.md 2026-09-25 «… the rustfmt PostToolUse hook reflows files after every write, so an Edit anchored on pre-format text misses …») → handoff Deferred learnings as `recurrence-despite-learning`.
- implement — a helper script written into the implement run dir with absolute host paths, moved to the scratchpad before hygiene: rejected — Filter 4 (one-off, self-caught before hygiene: 0.2 − 0.3).
- implement — the token window read by hand: applied (Tier 2, above).
- implement — C7 disposed as not readable by `R4`, re-pin on «A live StopFailure payload»: not a learning — a route disposition, carried at P5.
- implement — `user-config: changed-by-others` on R4: rejected — Filter 1, duplicate of host-linux.md 2026-11-17 (the host's own sessions rewrite `~/.claude.json`; witness isolation by own marks).
- after implement — the second credentials read: the same candidate as the applied entry (one entry).
- after implement — the directive's judgment of the 5 s wait, the 0644 modes, the scope line: not a learning — the reconcile's judgment, no rule asked.
- after implement — the pass record «Corrections or rulings given during the pass: none»: no candidate.
