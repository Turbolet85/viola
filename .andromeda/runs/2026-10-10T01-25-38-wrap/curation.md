# Curation — 2026-10-09-epoch-3-cleanup-ii

Sources: this window's conversation and the report's Decisions & corrections. The first wrap window's
conversation (Setup and P1) did not survive the clear; a correction only it held is not curated.

```
CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  + "A count written into a master carries the rule that produced it" (confidence 0.8)
  Tier 2 (.claude/rules/*):                   + verification-harness.md: "cargo-mutants keeps the per-mutant logs of one earlier run only" (confidence 0.8)
                                              + verification-harness.md: "Sweeping cargo-mutants' outcome lines" (confidence 0.8)
  Tier 3 (.claude/docs/session-learnings.md): none
  Filters: 1 dup · 0 task-specific · 0 conflict · 2 below the threshold · 1 deferred (→ handoff) · 2 recurrences (→ handoff)
  Load-bearing: "cargo-mutants keeps the per-mutant logs of one earlier run only" → Windows mutation grade
  Load-bearing: "Sweeping cargo-mutants' outcome lines" → Windows mutation grade
```

## Applied
- Tier 1 — a count in a master carries its rule. Signals: the operator's direction (+0.4), verified by measurement
  (+0.4).
  Proof: the operator's word (`inputs#I5`) that architecture's count of root waits is amended only after its rule is
  read; the rule, read from two earlier chunks' reports, was a named list of 8 + 1 that no wrap re-took, while the
  pattern read 30 at this chunk's base and 22 after it (the report, Counts / qualifiers moved; `fanout-results.md`
  O1 and T4).
- Tier 2, `verification-harness.md` — per-mutant logs. Signals: verified by measurement (+0.4), a specific technical
  detail with context (+0.2), load-bearing for the next entry (+0.2).
  Proof: the report's Deviations: five of the fourteen kill rows name their killing test by construction, because
  the next witness run replaced that run's per-mutant logs before they were read (`evidence/survivors.md`).
- Tier 2, `verification-harness.md` — the two sweep hazards. Signals: verified by measurement (+0.4), a specific
  technical detail with context (+0.2), load-bearing for the next entry (+0.2).
  Proof: the report's Decisions & corrections, "Sweep hazards": two rows share a mutation text and function inside
  `both_parallel_answered` and inside `has_email`, and `clear_start` keeps a `+ → *` line on another expression
  (`evidence/survivors.md`, `evidence/survivors-build.py`).

## Not applied
- Deferred by the cap (0.8): a mutation run of `viola-e2e` leaves session homes on the shared test-home base, because
  the tool's copy of the tree carries the `target/e2e-home` link (26 after the score run; the report). → handoff.
- Below the threshold (0.4): a formatter width above its range panics at run time; build a large padding with
  `repeat`. One event, could be task-specific.
- Below the threshold (0.4): a reading is searched under each spelling the masters use (`78 m`, not `78 min`). One
  event, could be task-specific.
- Duplicate: a red with no failing assertion, or a timeout in a mutation record, is read against the backing and the
  host record and never re-run for green. Held already by the Tier-1 entry "A red stays open until its cause is
  known" and the handoff's standing rule.
- Recurrence, not a duplicate (→ handoff, `recurrence-despite-learning`): a time written into a record ahead of the
  clock: once in the implement run (the report) and three stamps in this wrap window (one caught by the stamp hook
  after the write, two by a re-read against the manifest and the cites trail).
- Recurrence, not a duplicate (→ handoff): a heredoc with a file target, refused by the guard, in the implement run.
