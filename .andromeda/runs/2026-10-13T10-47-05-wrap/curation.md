# Curation — 2026-10-13-epoch-4-cleanup

CLAUDE.md ecosystem: no new learnings this session.
  Filters: 1 dup (recurrence → handoff) · 0 task-specific · 0 conflict · 0 deferred · 4 below the confidence threshold · 4 not a learning
  CLAUDE.md size: see the P7 report (health check 1)

## Decisions & corrections, each weighed
1. `cargo modules orphans` reads a `#[cfg(test)] mod x;` file orphaned — rejected, Filter 4 (0.4: gate failure +0.4, specific detail +0.2, a plausible alternative −0.2: `test_support.rs`, declared `#[cfg(test)]` the same way, was not flagged and why was not measured, so the stated mechanism is not established).
2. A pure move proven by the test-name diff, the size probe and a line-multiset read — rejected, Filter 4 (one-off method, 0.2 − 0.3).
3. Slicing scripts re-read before running caught two defects — rejected, Filter 4 (one-off, no repeat).
4. Hygiene refused a Windows drive path quoted from a CI log in a phase run-dir note — Filter 1: `docs/session-learnings.md` "gate.py hygiene reads `/home/<x>/` in prose as a POSIX user home", whose 2026-10-06 extension already says the read takes a quoted Windows drive path → a DEFECT record recurring → `recurrence-despite-learning` (handoff).
5. A Windows-only assertion kept from passing vacuously by a refused control on CI's ordinary leg — not curated: this wrap amended it into test-plan §3 `run` step 4 (a master owns it); 0.6 without the no-other-home signal → rejected, Filter 4.
6. The operator's directions at implement — not a learning (directions for that run).
7. The operator's direction at this wrap (`inputs#I3`) — not a learning (a route direction; P5 acts on it).
8. Guard refusal: a `cd` into a skill references dir refused by the Bash guard, re-issued as a subshell — `.claude/rules/host-linux.md` §Paths already says `cd` only inside a subshell; the guard held the rule at the cost of its one re-issue (`guard-refused`), so it is not a recurrence and not a learning.
9. Implement's guard refusals: none — nothing to weigh.
