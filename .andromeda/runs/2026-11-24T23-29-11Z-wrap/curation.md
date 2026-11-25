# Curation — 2026-11-24-adopt-s-resume-folder (wrap 2026-11-24T23-29-11Z-wrap)

CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  none
  Tier 2 (.claude/rules/*):                   + testing.md: "A case whose start must end before its session starts cannot go through `Wrapper::adopt` or any readiness-waiting test wrapper … run the verb piped outside a terminal … read stdout and stderr apart."
    Proof: implement's deviation (report, Deviations from intent, step 6; Decisions & corrections, the sweep hazard): `Wrapper::adopt` (`tests/support/home.rs`) waits for readiness a not-resumed start never reaches, so the three step-6 cases run a piped `viola adopt` (`tests/cli_adopt.rs` 356-434 in the report's New text, by line); gate entry 5 green over them. Confidence 0.8: verified by measurement +0.4 (the finding changed the chunk's test design) · specific technical detail with context +0.2 · reached no other durable home +0.2 (P2 rejected the test-plan row `decision/sessionless-start-runs-piped` as curation's, fanout-results.md; no route annotation holds it; no hook refuses it).
  Tier 3 (.claude/docs/session-learnings.md): none
  Filters: 0 dup · 0 task-specific · 0 conflict (→ handoff) · 0 deferred (→ handoff)
  No-other-home: "a start that must end before its session starts runs piped, never through a readiness-waiting wrapper"
  CLAUDE.md size: 136/200 · T1 4.8 KB, 0 over 600 B

## Every Decisions & corrections entry, dispositioned
Implement's:
- The overseer directive `inputs#I3` (no real `claude`, the R5 script only under `--rehearse`, a CONTENDED read stops the whole layers) — not a learning: a one-run directive of the overseer's.
- The stop fired (CONTENDED at 22:38:59Z and 22:40:20Z) — not a learning: an event; the host rule it follows stands in the overseer's tooling.
- The overseer's word `inputs#I4` (quiet since 23:06:31Z; read `--from`, never `--last 15`) — not a learning: a one-run direction.
- Sweep hazard: `Wrapper::adopt` waits for readiness — applied (Tier 2, testing.md, above).
- Guard refusals: two `cd` into the skill's references dir — rejected: the Bash guard already refuses the act (a hook home, Filter 4's no-other-home bar).
After implement's (the close):
- The pass record: no correction or ruling beyond `inputs#I4` — nothing to weigh.
- The wrap directive `inputs#I5` (the operator away, whose word each amendment rests on, no screen literal or env value printed, no timer, no live arm) — not a learning: a one-wrap directive; its «never print a compiled screen literal» clause is already a Tier 1 entry (Filter 1 duplicate).
- The carry `cry-vz3fiqnh` — not a learning: the route's (P5).
- The entry's title misnames what was built — not a learning: the route's (P5).
This window:
- The Bash guard refused a pipeline tool's output piped into a filter (one re-issue) — rejected: a hook home.
- Main's own design-system add ran past its 300-char bound and `rows-land` refused it — not a learning: the tool's bound caught it (telemetry, the reconcile evolve record).
