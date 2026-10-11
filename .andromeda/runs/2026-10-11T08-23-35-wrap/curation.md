# Curation — wrap of 2026-10-11-the-board-viola-list

Scope: this wrap's two windows. The first window (implement, the operator pass, Setup and Phase 1) is gone; its
candidates are read from `resume-point.md` (nine, with their evidence). This window's own: the review card's owners
on no file (pipeline telemetry, friction record, not curated) and three shell recurrences.

CLAUDE.md ecosystem curated:
  Tier 2 (.claude/rules/*):  + testing.md: "A tree-equality assertion over a viola home compares each file's SHA-256, never its bytes: the home holds the pinned `viola` copy, and a red prints the whole binary into the result." (confidence 0.8)
  Proof: a remove-the-guard red in implement printed the pinned `viola` copy whole (a 64 MB tool result); `tests/cli_list.rs` `tree()` was changed to hash (report, Reverted / negative API facts; `evidence/red-green.md`). Signals: measurement +0.4, technical detail +0.2, no other durable home +0.2.
  Filters: 1 dup · 1 task-specific · 0 conflict · 0 deferred by cap · 5 below threshold
  No-other-home: "A tree-equality assertion over a viola home compares hashes, never bytes"
  CLAUDE.md size: 127/200 · T1 2.3 KB, 0 over 600 B

## Each candidate

1. Windows x64 unstamped root-test home fails the strict-modes check; a root case running a home-checking reader
   starts from `stamped_home` — 0.6 (measurement +0.4, detail +0.2), rejected at the threshold: the fact already
   stands in test-plan and `.claude/docs/tests-summary.md` (since chunk 2026-10-10-budget-governor), so no-other-home
   does not fire. It recurred in this chunk anyway (CI red, four cases) → handoff, Deferred learnings, as
   `recurrence-despite-learning`.
2. `std::fs::create_dir_all` over a dangling symlink returns `AlreadyExists` — 0.6 (measurement, detail); its home
   is this wrap's P5 pin for implement's firing-1 red, which carries the mechanism → no-other-home does not fire;
   rejected.
3. Lint the Windows target before the push — duplicate of `ci.md` 2026-10-10 (a confirmation, no new facet).
4. Tree equality by hash — applied (above).
5. The stamp-ahead hook reads a unit test's synthetic instants dated today — 0.6; the hook itself refuses the act
   the fact is about → no-other-home does not fire; rejected.
6. A time written ahead of the clock (four times) → handoff, `recurrence-despite-learning`.
7. The gate trail's `ts` per firing dates a journal line — 0.2; rejected.
8. `gh run view <run> --log-failed` gives the failed steps' log in one call — 0.2 (technical detail only; the
   `--allow-escape-sequences` half is `ci.md` 2026-10-04's); rejected.
9. The masters' row-escape wording as a sweep hazard — task-specific and partly stale after this wrap's
   amendments (design-system now says "row mode"); rejected.

Recurrences this window (→ handoff): the embedded-grep bounded repetition (the global CLAUDE.md Shell rule); an
inline python heredoc carrying document text (`host-linux.md` Transports).
