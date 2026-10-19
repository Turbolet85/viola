# Curation — 2026-10-19-dialog-hold-for-the-driver

CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  none
  Tier 2 (.claude/rules/*):                   + host-linux.md (extended) · + testing.md (extended)
  Tier 3 (.claude/docs/session-learnings.md): none
  Filters: 0 dup · 0 task-specific · 0 conflict · 0 deferred · 1 below threshold
  Load-bearing: "a census keyed on target/debug/viola reads the overseer pair's bridge" → Overseer switch to viola
  No-other-home: "a write order is witnessed by a capture layer at the obs line after the append"
  Extended: T2/host-linux.md: "On the Linux dev host a process named `viola` is not this repository's by its name…" + the `gate.py census --args` form
  Extended: T2/testing.md: "To test a timing window, force it open with a test-only hold…" + a write order witnessed without a hold
  CLAUDE.md size: 128/200 · T1 3.0 KB, 0 over 600 B

## Applied

- **T2 host-linux.md, extended 2026-10-19** (score 0.8: verified by measurement +0.4, specific technical detail +0.2,
  load-bearing for "Overseer switch to viola" +0.2). The extension takes host-linux.md, the always-loaded host leaf,
  by tiebreaker 6, judged at Tier 1's ~600 B bar; the whole entry now stands at about 520 B.
  Proof: the implement run's first census (`gate.py census --args target/debug/viola …`) listed 11 processes of
  `~/dev/projects/additional/viola-lab/prototype/target/debug/viola`; re-keyed on `projects/viola/target/debug/viola`,
  it read `none running` before and after the live round (`.andromeda/runs/2026-10-19T08-22-37-implement/census-*`
  trail; `evidence/live-round.md` §Setup). Next markerless entry: "Overseer switch to viola" runs live with that
  bridge on the host.
- **T2 testing.md, extended 2026-10-19** (score 0.8: verified by measurement +0.4, technical detail +0.2, no other
  durable home +0.2, as test-plan §4 names the case but not the technique).
  Proof: `src/run/dialog.rs`. The `wait`-woken order case stayed green 12 of 12 runs on the reversed order. Its
  replacement `hook_dialog_held_is_on_the_snapshot_when_its_line_is_logged` (a capture layer reading
  `pending_dialog` at the `dialog-raised` obs line) read red 3 of 3 on the reversed order and green on the fix
  (`evidence/operator-pass.md` §5).

## Every *Decisions & corrections* entry, disposed

1. The founder's cap restated at /implement: not a learning. It was a chunk-scoped directive, kept and recorded in
   `evidence/live-round.md`.
2. The operator's wrap directive (`inputs#I4`): not a learning. It was a chunk-scoped direction, applied (the
   Deviation, the PREREQ, the card rows).
3. Sweep hazard, the census on `target/debug/viola`: **extended**, T2 host-linux.md (above). It is also a
   recurrence of that entry's class (`pgrep -x` → the census tool), and is logged in the handoff as
   `recurrence-despite-learning`.
4. Witnessing a write order: **extended**, T2 testing.md (above).
5. events.md 2026-10-16 recurred as a runner-only red: not a new entry, since the rule stands, correct. The
   `register` order predated the rule and no sweep of it reached that site. It is logged in the handoff as
   `recurrence-despite-learning: events.md 2026-10-16`.
6. The preservation guard read red on a recorded companion edit: not a learning. It is a carried operator item in
   the handoff, re-carried with this chunk's recurrence.
7. A plan named an rstest in `viola-core`, which has no rstest dev-dependency: not a curation entry. Its home is
   test-plan §4 Test grouping, amended at this wrap's P2.
8. The Bash guard refused a `cd` into a skill dir: not a learning. The guard worked; one re-issue.
9. The answer body was built in memory and piped: rejected at Filter 4 (0.2, a one-off technique with no
   corroboration).
10. The `ci.md` order held through one red: not a learning; the rule worked as written.
