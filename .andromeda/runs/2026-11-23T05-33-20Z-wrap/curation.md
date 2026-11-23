# Curation — wrap 2026-11-23T05-33-20Z-wrap (chunk 2026-11-22-lineage-registry)

CLAUDE.md ecosystem: no new learnings this session.
  Tier 1: none · Tier 2: none · Tier 3: none
  Filters: 1 dup (recurrence-despite-learning → handoff) · 2 task-specific / below threshold · 0 conflict · 0 deferred

## Dispositions — implement's *Decisions & corrections*
- Host rule fired (hostwatch CONTENDED → stop; the overseer's quiet-window word `inputs#I5`) — not a learning: the
  host rule and the overseer's directive already carry it; this chunk followed it.
- Recurrence of testing.md 2026-10-04 (six start-record counters outside the plan's readers, four red only at the whole
  layers) — rejected, Filter 1: the matched entry (`.claude/rules/testing.md`, 2026-10-04, extended 2026-11-14) already
  states the facet («find them by the counted literals (`len() >=`, `events.len()`, «start records») across the whole
  test tree, never by the removed kind's name»). A DEFECT record that did not prevent its recurrence →
  `recurrence-despite-learning`, logged in the handoff's Deferred learnings. What would have caught it at P3 (the
  phase's plan, where the reader list was written): the plan's reader search run by the counted literals the entry
  names, over the whole test tree, instead of by `session-start`; the remedy is a check in the phase's plan reference,
  for the pipeline's owners.
  Proof: report Spec claims disproved (`tests/hook_events.rs` ×3, `tests/path1_run_start.rs` ×1,
  `tests/cli_instance_state.rs` ×2); implement's gate reds on `run --integration` and `run --e2e` before their re-fires.
- Sweep hazard: a struct-literal sweep keyed on `Replayed {` also matched `Recovered::Replayed {` — rejected, Filter 4:
  measured (+0.4) and specific (+0.2), caught at the next compile with no gate red, one-off (-0.3) → 0.3.
- The operator smoke caught the harness inheriting the builder's name as its instances' parent — rejected, Filter 4:
  measured (+0.4), specific (+0.2) = 0.6, exactly at the threshold; no conditional signal applies, since this wrap's P2
  landed the fact in test-plan `decision/supervisor-spawn-strips-bridge-env` and the leaf
  `.claude/rules/verification-harness.md` (its master home).
- Guard refusals (a `cat` heredoc, a `cd`, a pipe into grep) — not a learning: each cost its one re-issue (telemetry,
  `guard-refused`).

## Dispositions — `## After implement`'s *Decisions & corrections after implement*
- The operator's word moving v2-09's overseer clause to v2-28 (`inputs#I6`) — not a learning: a ruling carried by the
  matrix (P7.3 `refine --chunk`) and the route's owed v2-28 line (P5).
- F3, F6, F7, F8 and the P5 `yes` held as a delegate's word — not a learning: landed as PROVISIONAL rows and in
  `founder-forks.md` row W1.
- The `inputs#I5` quiet-window word — not a learning (as above).
- Curation candidate (the recurrence) — as above, `recurrence-despite-learning`.

## Dispositions — this wrap's conversation
- A leaf comparer proposed a figure the code states («16 kinds»), which D-leaf-figures bars; held — a recurrence of the
  handoff's deferred «a leaf names a time bound by its constant, never its figure»: `recurrence-despite-learning`,
  logged beside it.
- `rec.py rows-land` asks `state: provisional` of a delegate-word contract add that has no `state` cell — tool
  telemetry (an evolve record), not a session learning.
