# Curation — 2026-10-13-epoch-4-survivors-ii

CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  none
  Tier 2 (.claude/rules/*):                   corrected in place — verification-harness.md 2026-09-27: the kill filter `test(/(^|::)tests::name(::|$)/)`, a table row named `name::case_<n>_<label>`
  Tier 3 (.claude/docs/session-learnings.md): none
  Filters: 2 dup · 3 task-specific · 0 conflict (→ handoff) · 0 deferred (→ handoff)
  Extended: T2/verification-harness.md: "2026-09-27: A nextest `test(=name)` filter needs the test's full path" + "`(::|$)`, not `$`, so an rstest table's rows are selected"
  CLAUDE.md size: 128/200 · T1 2.6 KB, 0 over 600 B

## The applied correction
- `.claude/rules/verification-harness.md:60` (`## Session Additions`, 2026-09-27). Was "write `test(/(^|::)tests::name$/)`". It
  now reads "write `test(/(^|::)tests::name(::|$)/)` (an rstest table's rows are named `name::case_<n>_<label>`, which a `$`
  right after the name never selects)", tagged `[corrected 2026-10-13: … chunk 2026-10-13-epoch-4-survivors-ii; `(::|$)`
  read each one]`. This is a correction (the measurement made the clause false for table cases), exempt from the
  max-3 cap, on the operator's word `inputs#I2` ("Amend the leaf that holds the kill filter form so a table case
  (::case_n_label) matches, as your deviation found").
  Proof: this chunk's first green run of the new cases (`target/run-archive/1158`) named each table row
  `cmd::verify::typed::tests::<fn>::case_<n>_<label>`. The 14 hand-applied runs used `(::|$)` and read every table's
  rows red under their mutants and green restored (`viola-0.1.0/chunks/2026-10-13-epoch-4-survivors-ii/evidence/survivors.md`).
  Sweep: `grep -rnF 'tests::name$'` and `'tests::<name>$'` over `.claude/` and `CLAUDE.md` found 1 hit, the corrected
  tag itself; no master states the form (P1 site sweep `killfilter`, control unfired). The plan's own step 4 text is
  a chunk record and stays as written.

## Decisions & corrections, each weighed
1. The take-up direction (`inputs#I1`): no mutation run, a hand-applied mutant, no live `claude` — not a learning:
   rejected, Filter 1 dup. It is the founder's standing direction, already in `.claude/rules/testing.md` 2026-10-04
   and test-plan §10.
2. The implement direction (times from `date -u` in the same call, pipeline output to a file, journal before a wait,
   the operator pass as usual) — not a learning: rejected, Filter 1 dup. It is the handoff's standing rules and the
   operator's per-invocation text, and the stamp-ahead recurrence it guards against did not recur this chunk.
3. The wrap direction (`inputs#I2`): amend the kill-filter leaf — **corrected in place** (above).
4. Sweep hazard: an rstest case name is `<fn>::case_<n>_<label>`, and an anchor on `<fn>$` selects no row —
   **folded into the correction** (3). The same fact needs no second entry.
5. Positive: a 2 s red was checked against its log and read the case's own assertion — rejected, Filter 1 dup. The
   2026-09-27 entry already says "count a red only with the case's own panic line".
6. The `Instant - Duration` panic of a `-` mutant did not occur (host clock past 120 s) — rejected, Filter 2
   task-specific. It is one host's uptime, and the plan's own note already covers either reading.
7. Guard or hook refusals: none — nothing to weigh.
