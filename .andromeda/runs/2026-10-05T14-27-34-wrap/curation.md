# Curation — wrap 2026-10-05T14-27-34 (chunk 2026-10-05-dialog-rows-and-re-probe)

Scope: this window's conversation plus the report's *Decisions & corrections*. The window that ran implement and
P1 is gone, so a correction only it held is not curated here.

```
CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  none
  Tier 2 (.claude/rules/*):                   extended testing.md (1 write)
  Tier 3 (.claude/docs/session-learnings.md): none
  Filters: 0 dup · 2 task-specific/one-off · 1 conflict (→ handoff) · 0 deferred; 2 recurrence-despite-learning (→ handoff)
  No-other-home: "read a CI timing red two-sided across the runs' JUnit"
  Extended: T2/testing.md: "2026-09-28: A timing red is never fixed by raising a timeout or a test bound…" + the two-sided JUnit read
```

## Applied
- T2 `testing.md`, Extended 2026-10-05: before naming a timing red's cause, compare the failing runs' JUnit timings
  for tests that never touch the changed code with the changed ones.
  Proof: `viola-0.1.0/chunks/2026-10-05-dialog-rows-and-re-probe/evidence/ci-rounds.md` and
  `evidence/verify-window-class.md`. Both reds fell in runs with a runner-wide 2.5-3x slow tail, read over tests
  outside the diff. Round 2's timing-only measurement falsified the hook-count hypothesis. Score: measured +0.4,
  technical detail +0.2, no other home +0.2 (no master, route annotation or ledger note carries the technique) = 0.8.

## Rejected
- "The settle before a kill is a 300 ms quiet grace, not a guarantee" scored 0.6 (measured +0.4, detail +0.2),
  which is an exact-0.6 reject. It is also amended into architecture [CLI Version Compatibility] this wrap, so no
  conditional signal applies.
- The host `grep` is ugrep and refused a bounded-context regex for exceeding its complexity limit (this window).
  One-off: measured +0.4, one-off −0.3, detail +0.2 = 0.3.
- `ls -t | head -1` over a task dir names the watcher's own output: a one-off (report).
- The `plan-` substring sweep hazard: carried in the test itself, one instance, 0.2.

## Recurrences (→ handoff Deferred learnings)
- `recurrence-despite-learning: host-win32.md 2026-09-25` — `pkill -f` matched the calling shell's own command line
  and killed it (report, Sweep hazards).
- `recurrence-despite-learning: host-win32.md 2026-09-28/29` — a heredoc to a file was refused by the Bash guard
  (implement window, per the resume point).

## Conflict (→ handoff, user review)
- `testing.md` 2026-09-28 says "A timing red is never fixed by raising a timeout or a test bound … then remove the
  slow work". This chunk's `verify_window_` class is a per-test kill sized from a measured designed floor plus a 3x
  tail (the overseer's founder-delegated decision, `evidence/verify-window-class.md`). The overseer says it
  reverses his earlier "not option 3" only because the floor is designed, not a regression. Not auto-resolved:
  whether the entry gains a "designed floor" carve-out is the user's call.
