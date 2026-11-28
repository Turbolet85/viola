# Curation — wrap 2026-11-28T12-37-34Z-wrap (chunk 2026-11-28-fleet-console-view)

```
CLAUDE.md ecosystem: no new learnings this session.
  Filters: 0 dup · 0 task-specific · 0 conflict (→ handoff) · 0 deferred (→ handoff) · 1 confidence · 1 recurrence (→ handoff)
  CLAUDE.md size: 136/200 · T1 4.8 KB, 0 over 600 B
```

## Every candidate and its disposition
Implement's *Decisions & corrections*:
1. The `pgrep -f` self-match recurred (the transcript monitor's end test matched its own shell) — rejected at Filter 1
   (dup of host-linux.md §Processes, a DEFECT record) → handoff `recurrence-despite-learning: host-linux.md §Processes
   «never pgrep -f on a pattern the invoking shell's own command line contains»`.
2. The Bash guard refused a `cd` into the skill's references dir; re-issued once — not a learning: the guard put the
   call on its documented path (`guard-refused`, telemetry).
3. Sweep hazard: a new background read added to a verb reaches that verb's old test cases — rejected at Filter 4:
   +0.4 measured (it changed the chunk's design: every `tui_console` start moved to a test-scoped PATH) +0.2
   technical detail = 0.6, and neither conditional signal applies — this wrap amended it into a master (test-plan
   `decision/tests-never-read-host-sessions`, tx 5380) and the cascade re-derived `.claude/rules/testing.md:20`.
   Proof: report Deviations, «Every `tui_console` start … now runs with `PATH` set to an empty test-scoped bin dir».
4. Decision: an unwrapped NAME is not cut by the fleet text — not a learning: a master row holds it (design-system
   `contract/fleet-console-view/line-cut`, tx 5376).

`## After implement`'s *Decisions & corrections after implement*:
5. The pass: none — nothing to weigh.
6. The wrap directive (inputs#I7): the operator away, no dialog, the record `gated` — not a learning: a one-wrap
   direction.

This conversation:
7. The answer card's first stamp was estimated (13 min ahead); the stamp-ahead hook refused it and one re-issue from
   `date -u` fixed it — not a learning: the hook held CLAUDE.md's `date -u` rule at the cost of its one re-issue
   (`guard-refused`), no recurrence.
8. Two Bash guard refusals (a `cd` into the run dir; a heredoc to a scratch file), and one of a piped `rec.py` read —
   each re-issued once in the guard's form: not learnings (`guard-refused`).
