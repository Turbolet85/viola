# Curation — 2026-11-16-local-command-delivery

```
CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  + "A builder session sets no scheduled wake, timer or reminder of its own …" (confidence 1.1)
  Tier 1 (CLAUDE.md USER:session-learnings):  corrected "A red stays open until its cause is known …" — its moment named (before the operator pass)
  Tier 2 (.claude/rules/*):                   + security.md: "Write a bounded read as `take(reader, MAX_FRAME).read_to_end(…)` …" (confidence 0.8)
  Tier 2 (.claude/rules/*):                   corrected testing.md 2026-11-16 «A CI red … not the chunk's» — its moment named (read at the operator pass)
  Tier 3 (.claude/docs/session-learnings.md): none
  Filters: 1 dup · 2 task-specific · 0 conflict (→ handoff) · 0 deferred · 1 recurrence-despite-learning (→ handoff)
  No-other-home: "Write a bounded read as `take(reader, MAX_FRAME).read_to_end(…)`"
  CLAUDE.md size: 134/200 · T1 3.9 KB, 0 over 600 B
```

Writes: CLAUDE.md tx 1948 · .claude/rules/testing.md tx 1949 · .claude/rules/security.md tx 1950.

## Applied
- T1 scheduled wake (new). Proof: the wrap's first window was lost when a builder `ScheduleWakeup` left from the
  implement fired into it, the wheel went to «human» and the answer card's dialog reached the CLI's own menu
  (`inputs#I7`, the relay); implement's report *Decisions & corrections* records the 1800 s `ScheduleWakeup` set
  against implement P2's rule. Signals: verified by measurement +0.4 · the operator's directive (correction) +0.4 ·
  «never»/«no» language +0.3 = 1.1.
- T1 correction of «A red stays open … folds into that chunk even outside its diff». Proof: the wrap directive
  `inputs#I6` (5), the overseer's reading, settling the Filter 3 tension the previous handoff named; each rule now
  names its moment. A correction, exempt from the cap.
- T2 testing.md 2026-11-16 entry correction, the same direction: «read at the operator pass» and a pointer to the
  fold rule; the founder's quoted word kept verbatim.
- T2 security.md `take(` form. Proof: implement's block iteration 2, `contract_bounded_readers` red on
  `src/cmd/verify/typed/turns.rs:222` (report *Decisions & corrections*). Signals: verified by a real gate failure
  +0.4 · specific technical detail +0.2 · reached no other durable home +0.2 = 0.8 (no master, rule or route entry
  carries it). Security-sensitive class → security.md (no `paths:`, judged at Tier 1's bar: one sentence, 245 B).

## The report's Decisions & corrections, each dispositioned
- Implement · sweep hazard (row-count sweep keyed on the last row's id missed a verdict vector and a summary) — →
  handoff: `recurrence-despite-learning: testing.md 2026-11-15 «A removal sweeps every test that reads the removed
  field, row or mode …»` (the matched entry records the defect; it recurred for an addition).
- Implement · `contract_bounded_readers` reads `take(` only ahead of the read call — applied (T2 security.md).
- Implement · bash guard refused a `cat` heredoc and two `cd` — not a learning: guard refusals each re-issued once
  (host-linux.md already carries both).
- Implement · permission classifier gave no verdict four times — rejected, duplicate (CLAUDE.md Tier 1 «A tool call
  refused with no verdict …»).
- Implement · a `ScheduleWakeup` set against implement P2's rule — applied (T1, the scheduled-wake entry, with the
  relay's measurement).
- Implement · hygiene refused a `/tmp` path in a relay record, respelled — rejected, task-specific (one file's token).
- After implement · `inputs#I6` (1) widenings as standing rows — not a learning: P2's disposition (rows landed; the
  delegate's card confirm PROVISIONAL by the letter).
- After implement · `inputs#I6` (2) the live re-stamp owed — not a learning: P5 CARRY and P6 handoff.
- After implement · `inputs#I6` (3) named residuals — not a learning: P2 rows (security-plan
  decision/verify-echo-probe-residuals) and the P5 CARRY for `/context`.
- After implement · `inputs#I6` (5) the CI-red rule vs the fold learning — corrected in place, both (T1 and T2).
- After implement · the pass record: no correction — nothing to weigh.
- This window · the answer card's delegate confirm vs the directive's «not PROVISIONAL» — not a project learning:
  a pipeline letter's tension, recorded as an evolve friction.
- This window · a leaf comparer missed `workflow.md:49` — not a project learning: evolve friction
  (`contract.cascade-miss`).
