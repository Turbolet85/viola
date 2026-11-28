# Curation — 2026-11-27-epoch-5-cleanup-2

CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  none
  Tier 2 (.claude/rules/*):                   none
  Tier 3 (.claude/docs/session-learnings.md): + "A whitespace-free comparison of moved Rust still differs by the commas rustfmt drops" (confidence 0.8)
                                              + "A move into a sibling module tree needs every module on the path opened" (confidence 0.8)
  Filters: 0 dup · 0 task-specific · 0 conflict (→ handoff) · 0 deferred (→ handoff)
  No-other-home: "A whitespace-free comparison of moved Rust still differs by the commas rustfmt drops"
  No-other-home: "A move into a sibling module tree needs every module on the path opened"
  CLAUDE.md size: 136/200 · T1 4.8 KB, 0 over 600 B

Landed: `rec.py harness land` `.claude/docs/session-learnings.md` rev 19, tx 5233.

## Applied
- Tier 3 «A whitespace-free comparison of moved Rust still differs by the commas rustfmt drops» — +0.4 measured,
  +0.2 technical detail, +0.2 no other home (not on the route, no master row, no rule) = 0.8. Tier 3: a sweep hazard
  of a moved-body check; `testing-src.md` is a pointer index and the check is no test authoring, so no Tier-2 home.
  Proof: implement's report, Deviations 5 and *Decisions & corrections* (the sweep hazard); `evidence/moved-bodies.md`
  rule 2, 3 named items of 159.
- Tier 3 «A move into a sibling module tree needs every module on the path opened» — +0.4 measured (it falsified plan
  step 7's claim, the report's Spec claims disproved), +0.2 technical detail, +0.2 no other home = 0.8. Tier 3: Rust
  visibility context across `src/`, no rule file of its class.
  Proof: report Deviations 3 and Spec claims disproved (plan step 7); `src/cmd/mod.rs:15` `pub(crate) mod run;`.

## Decisions & corrections, each dispositioned
Implement's:
- The invocation directive, kept in full — not a learning (a run's directive, honoured).
- Guard refusals, each re-issued once — not a learning (`guard-refused`, the evolve record's).
- Sweep hazard (trailing commas) — applied, Tier 3 (above).
- A move into a sibling module tree crosses a private module boundary — applied, Tier 3 (above).
`## After implement`'s:
- The overseer's relayed word for the pass (`inputs#I5`) — not a learning (a ruling for this chunk).
- The departures, each closed — not a learning (this chunk's record).
- The log line's `target` move, read against the masters — not a learning (no row pins it; named in the close).
This wrap's conversation:
- The directive's leaf follow written by hand (no comparer can propose a tree fact no master delta carries) — not a
  learning: pipeline telemetry, the reconcile evolve record's `problem`.
