# Fan-out results — wrap 2026-10-26-home-and-code-bearing-file-integrity

check's `proposals:` line (stage1): architecture 11 · security-plan 6 · design-system 4 · layout-templates 4 · test-plan 17 · obs-plan 12 · a11y-plan 0

architecture: 11 proposals · fanout-architecture.md · 53 cascade proposals · fanout-architecture--cascade-g1.md, fanout-architecture--cascade-g2.md · 21 facts · 20 placed · 0 no home · 1 the doc agent's · (sampler) 5 lost
security-plan: 6 proposals · fanout-security-plan.md · 61 cascade proposals · fanout-security-plan--cascade-g1.md, fanout-security-plan--cascade-g2.md, fanout-security-plan--cascade-new-error-handling.md · 21 facts · 21 placed · 0 no home · (sampler) 9 lost
design-system: 4 proposals · fanout-design-system.md · 14 cascade proposals · fanout-design-system--cascade.md · 5 facts · 5 placed · 0 no home · (sampler) 0 lost
layout-templates: 4 proposals · fanout-layout-templates.md (whole master, no cascade)
test-plan: 17 proposals · fanout-test-plan.md · 56 cascade proposals · fanout-test-plan--cascade-g1.md, fanout-test-plan--cascade-g2.md · 18 facts · 18 placed · 0 no home · (sampler) 7 lost
obs-plan: 12 proposals · fanout-obs-plan.md · 70 cascade proposals · fanout-obs-plan--cascade-g1.md … -g4.md, fanout-obs-plan--cascade-new-error-capture-reporting.md · 14 facts · 14 placed · 0 no home · (sampler) 9 lost
a11y-plan: 0 proposals · fanout-a11y-plan.md · 7 cascade proposals · fanout-a11y-plan--cascade.md · 4 facts · 4 placed · 0 no home · (sampler) 5 lost

Notes (the doc agents' `note:` lines, read whole in each file): architecture — D-arch-decisions no violation, stale detail rows the cascade's; security-plan — the body's broad lines move with the retirements, D-security-input/-auth/-deps no drift; design-system — detail sites the cascade's, tokens n/a; layout-templates — no new surface, no retired gap stated; test-plan — detail sites the cascade's, no harness change; obs-plan — D-obs-stack / -pii no drift; a11y-plan — D-a11y-surface / -obs-schema no drift, §8 the cascade's. No fix was made by script: no PREAMBLE, COMMENTARY or escape fix, so no `.raw-` twin of a result file.

Sampler (`detail.py compare`, the operator's forced draw `inputs#I5`): architecture `sampler: E carried 42 · the cascade lost 3 · beside a placed fact 2`; security-plan `E carried 83 · the cascade lost 5`; design-system `E carried 10 · the cascade lost 0`; test-plan `E carried 47 · the cascade lost 4 · beside a placed fact 1`; obs-plan `E carried 48 · the cascade lost 6 · beside a placed fact 2`; a11y-plan `E carried 6 · the cascade lost 2 · beside a placed fact 2`.

Validate's checks 3–6, read against the inputs: check 3 — every proposal rests on the report's Changes and the plan's acceptance; the scope record's five companion lines each serve the step it names (report Deviations); no divergence escalates. Check 4 — the absence claims («no product reader under `src/`», «`check_instance(` 3 sites») rest on the report's remaining-caller facts and their commands. Check 5 — every plan Expected-amendments entry is proposed (security-plan, architecture, obs-plan, test-plan, design-system and layout-templates, a11y-plan §8: fanout-a11y-plan--cascade.md 6 and cascade-compare-a11y-plan.md S1–S3); the citation-dispositions row `claim false` (authentication-authorization.md:27 `src/cmd/run.rs:383`) is settled by fanout-security-plan--cascade-g1.md 10's direction. Check 6 — each `Spec claims disproved` entry is matched: test-data-fixtures.md:10 (fanout-test-plan--cascade-g2.md 20), integration-test-strategy.md:33 (fanout-test-plan--cascade-g1.md 21, 22), unit-test-strategy.md:25 (fanout-test-plan--cascade-g1.md 15), occupied-resources.md:113 (fanout-architecture--cascade-g2.md 18), the hard-link propagation (fanout-test-plan--cascade-g2.md 21, fanout-architecture--cascade-g2.md 22, cascade-compare-test-plan.md S1). Rejected for a source the report does not carry: 0.

## Dispositions

fanout-architecture.md
1. apply — check 1: Accurate this-chunk addition (its fact in the report's Changes)
2. apply — check 1: Accurate this-chunk addition (its fact in the report's Changes)
3. apply — check 1: Accurate this-chunk addition (its fact in the report's Changes)
4. apply — check 1: Accurate this-chunk addition (its fact in the report's Changes)
5. apply — check 1: Accurate this-chunk addition (its fact in the report's Changes)
6. escalate → Q1 — check 1: Boundary widening (a reader moved onto `read_snapshot_or_replay`, so hook, CLI and MCP processes read `events.ndjson` on a new basis), the founder answered himself to P4 fork Q1 (`inputs#I3`): answer-card item · resolved: apply (Q1) — the card confirmed, `inputs#I6`, PROVISIONAL (delegate the overseer); the ratification stays the founder's `inputs#I3` Q1
7. escalate → Q1 — check 1: Boundary widening (a reader moved onto `read_snapshot_or_replay`, so hook, CLI and MCP processes read `events.ndjson` on a new basis), the founder answered himself to P4 fork Q1 (`inputs#I3`): answer-card item · resolved: apply (Q1) — the card confirmed, `inputs#I6`, PROVISIONAL (delegate the overseer); the ratification stays the founder's `inputs#I3` Q1
8. apply — check 1: Accurate this-chunk addition (its fact in the report's Changes)
9. apply — check 1: Accurate this-chunk addition (its fact in the report's Changes)
10. apply — check 1: Accurate this-chunk addition (its fact in the report's Changes)
11. apply — check 1: Accurate this-chunk addition (its fact in the report's Changes)

fanout-architecture--cascade-g1.md
1. apply — check 1: Accurate this-chunk addition (its fact in the report's Changes)
2. escalate → Q1 — check 1: Boundary widening (a reader moved onto `read_snapshot_or_replay`, so hook, CLI and MCP processes read `events.ndjson` on a new basis), the founder answered himself to P4 fork Q1 (`inputs#I3`): answer-card item · resolved: apply (Q1) — the card confirmed, `inputs#I6`, PROVISIONAL (delegate the overseer); the ratification stays the founder's `inputs#I3` Q1
3. escalate → Q1 — check 1: Boundary widening (a reader moved onto `read_snapshot_or_replay`, so hook, CLI and MCP processes read `events.ndjson` on a new basis), the founder answered himself to P4 fork Q1 (`inputs#I3`): answer-card item; also a check-7 contradiction (companion fanout-architecture--cascade-g1.md 2 rewrites established-decisions.md:79), Q2 · resolved: apply (Q1) — the card confirmed, `inputs#I6`, PROVISIONAL (delegate the overseer); the ratification stays the founder's `inputs#I3` Q1; its check-7 contradiction resolved with its companion (Q2)
4. apply — check 1: Accurate this-chunk addition (its fact in the report's Changes)
5. escalate → Q1 — check 1: Boundary widening (a reader moved onto `read_snapshot_or_replay`, so hook, CLI and MCP processes read `events.ndjson` on a new basis), the founder answered himself to P4 fork Q1 (`inputs#I3`): answer-card item · resolved: apply (Q1) — the card confirmed, `inputs#I6`, PROVISIONAL (delegate the overseer); the ratification stays the founder's `inputs#I3` Q1
6. apply — check 1: Accurate this-chunk addition; check 7: architecture-newgroup-established-decisions.md · distinct
7. apply — check 1: Accurate this-chunk addition (its fact in the report's Changes)
8. escalate → Q1 — check 1: Boundary widening (a reader moved onto `read_snapshot_or_replay`, so hook, CLI and MCP processes read `events.ndjson` on a new basis), the founder answered himself to P4 fork Q1 (`inputs#I3`): answer-card item · resolved: apply (Q1) — the card confirmed, `inputs#I6`, PROVISIONAL (delegate the overseer); the ratification stays the founder's `inputs#I3` Q1
9. escalate → Q1 — check 1: Boundary widening (a reader moved onto `read_snapshot_or_replay`, so hook, CLI and MCP processes read `events.ndjson` on a new basis), the founder answered himself to P4 fork Q1 (`inputs#I3`): answer-card item · resolved: apply (Q1) — the card confirmed, `inputs#I6`, PROVISIONAL (delegate the overseer); the ratification stays the founder's `inputs#I3` Q1
10. apply — check 1: Accurate this-chunk addition (its fact in the report's Changes)
11. escalate → Q1 — check 1: Boundary widening (a reader moved onto `read_snapshot_or_replay`, so hook, CLI and MCP processes read `events.ndjson` on a new basis), the founder answered himself to P4 fork Q1 (`inputs#I3`): answer-card item · resolved: apply (Q1) — the card confirmed, `inputs#I6`, PROVISIONAL (delegate the overseer); the ratification stays the founder's `inputs#I3` Q1
12. apply — check 1: Accurate this-chunk addition (its fact in the report's Changes)
13. apply — check 1: Accurate this-chunk addition (its fact in the report's Changes)
14. apply — check 1: Accurate this-chunk addition (its fact in the report's Changes)
15. apply — check 1: Accurate this-chunk addition; check 7: architecture-newgroup-established-decisions.md · distinct
16. apply — check 1: Accurate this-chunk addition; check 7: architecture-newgroup-established-decisions.md · distinct
17. apply — check 1: Accurate this-chunk addition; check 7: architecture-newgroup-conventions.md · distinct
18. apply — check 1: Accurate this-chunk addition; check 7: architecture-newgroup-conventions.md · distinct
19. apply — check 1: Accurate this-chunk addition; check 7: architecture-newgroup-conventions.md · distinct
20. apply-merged → fanout-architecture.md 2 — check 2: the same body line, one edit (the doc agent's and the cascade's proposals of one fact)
21. apply — check 1: Accurate this-chunk addition (its fact in the report's Changes)
22. apply — check 1: Accurate this-chunk addition; check 7: architecture-newgroup-conventions.md · distinct
23. escalate → Q1 — check 1: Boundary widening (a reader moved onto `read_snapshot_or_replay`, so hook, CLI and MCP processes read `events.ndjson` on a new basis), the founder answered himself to P4 fork Q1 (`inputs#I3`): answer-card item · resolved: apply (Q1) — the card confirmed, `inputs#I6`, PROVISIONAL (delegate the overseer); the ratification stays the founder's `inputs#I3` Q1
24. escalate → Q1 — check 1: Boundary widening (a reader moved onto `read_snapshot_or_replay`, so hook, CLI and MCP processes read `events.ndjson` on a new basis), the founder answered himself to P4 fork Q1 (`inputs#I3`): answer-card item · resolved: apply (Q1) — the card confirmed, `inputs#I6`, PROVISIONAL (delegate the overseer); the ratification stays the founder's `inputs#I3` Q1
25. apply — check 1: Accurate this-chunk addition; check 7: architecture-newgroup-standard-contracts.md · distinct
26. escalate → Q1 — check 1: Boundary widening (a reader moved onto `read_snapshot_or_replay`, so hook, CLI and MCP processes read `events.ndjson` on a new basis), the founder answered himself to P4 fork Q1 (`inputs#I3`): answer-card item · resolved: apply (Q1) — the card confirmed, `inputs#I6`, PROVISIONAL (delegate the overseer); the ratification stays the founder's `inputs#I3` Q1
27. apply — check 1: Accurate this-chunk addition (its fact in the report's Changes)
28. apply — check 1: Accurate this-chunk addition (its fact in the report's Changes)
29. escalate → Q1 — check 1: Boundary widening (a reader moved onto `read_snapshot_or_replay`, so hook, CLI and MCP processes read `events.ndjson` on a new basis), the founder answered himself to P4 fork Q1 (`inputs#I3`): answer-card item · resolved: apply (Q1) — the card confirmed, `inputs#I6`, PROVISIONAL (delegate the overseer); the ratification stays the founder's `inputs#I3` Q1
30. escalate → Q1 — check 1: Boundary widening (a reader moved onto `read_snapshot_or_replay`, so hook, CLI and MCP processes read `events.ndjson` on a new basis), the founder answered himself to P4 fork Q1 (`inputs#I3`): answer-card item · resolved: apply (Q1) — the card confirmed, `inputs#I6`, PROVISIONAL (delegate the overseer); the ratification stays the founder's `inputs#I3` Q1

fanout-architecture--cascade-g2.md
1. escalate → Q1 — check 1: Boundary widening (a reader moved onto `read_snapshot_or_replay`, so hook, CLI and MCP processes read `events.ndjson` on a new basis), the founder answered himself to P4 fork Q1 (`inputs#I3`): answer-card item · resolved: apply (Q1) — the card confirmed, `inputs#I6`, PROVISIONAL (delegate the overseer); the ratification stays the founder's `inputs#I3` Q1
2. apply — check 1: Accurate this-chunk addition (its fact in the report's Changes)
3. apply — check 1: Accurate this-chunk addition; check 7: architecture-newgroup-occupied-resources.md · distinct
4. apply — check 1: Accurate this-chunk addition (its fact in the report's Changes)
5. apply — check 1: Accurate this-chunk addition (its fact in the report's Changes)
6. apply — check 1: Accurate this-chunk addition (its fact in the report's Changes)
7. escalate → Q1 — check 1: Boundary widening (a reader moved onto `read_snapshot_or_replay`, so hook, CLI and MCP processes read `events.ndjson` on a new basis), the founder answered himself to P4 fork Q1 (`inputs#I3`): answer-card item · resolved: apply (Q1) — the card confirmed, `inputs#I6`, PROVISIONAL (delegate the overseer); the ratification stays the founder's `inputs#I3` Q1
8. apply — check 1: Accurate this-chunk addition; check 7: architecture-newgroup-occupied-resources.md · distinct
9. apply-merged → fanout-architecture.md 5 — check 2: the same body line, one edit (the doc agent's and the cascade's proposals of one fact)
10. apply — check 1: Accurate this-chunk addition (its fact in the report's Changes)
11. apply — check 1: Accurate this-chunk addition (its fact in the report's Changes)
12. escalate → Q3 — check 1 no match: names an owner for `viola list`'s home-check inconsistency (report Count 14; directive `inputs#I5` item 6c), a route placement P5 lands · resolved: apply (Q3) — `inputs#I6` (delegate the overseer) · as: the owner is a CARRY on «Unix endpoint and home hardening» (P5 pins it); name it so, with no other owner
13. escalate → Q2 — check 7: contradiction against a standing line this same pass rewrites (its own replacement of occupied-resources.md:112's set) · resolved: apply (Q2) — with its companion, `inputs#I6` (delegate the overseer)
14. apply — check 1: Accurate this-chunk addition (its fact in the report's Changes)
15. apply — check 1: Accurate this-chunk addition (its fact in the report's Changes)
16. apply — check 1: Accurate this-chunk addition; check 7: architecture-newgroup-occupied-resources.md · distinct
17. apply — check 1: Accurate this-chunk addition; check 7: architecture-newgroup-occupied-resources.md · distinct
18. apply — check 1: Accurate this-chunk addition (its fact in the report's Changes)
19. reject — check 7: architecture-newgroup-occupied-resources.md · duplicate · lines 185–185: the standing mutation-scratch group already holds the inherited Users write ACE refused `Writable`; the home-made-before-viola reading lands in security-plan (fanout-security-plan--cascade-g2.md 18) and test-plan (fanout-test-plan--cascade-g2.md 20)
20. apply — check 1: Accurate this-chunk addition; check 7: architecture-newgroup-occupied-resources.md · distinct
21. apply — check 1: Accurate this-chunk addition (its fact in the report's Changes)
22. apply — check 1: Accurate this-chunk addition; check 7: architecture-newgroup-occupied-resources.md · distinct
23. apply — check 1: Accurate this-chunk addition; check 7: architecture-newgroup-occupied-resources.md · distinct

(sampler) cascade-compare-architecture.md
S1. apply — check 1: Accurate this-chunk addition; check 7: architecture-newgroup-cross-cutting-patterns.md · distinct
S2. reject — check 7: architecture-newgroup-established-decisions.md · twin · fanout-architecture--cascade-g1.md 6
S3. apply — check 1: Accurate this-chunk addition; check 7: architecture-newgroup-established-decisions.md · distinct
S4. apply — check 1: Accurate this-chunk addition; check 7: architecture-newgroup-established-decisions.md · distinct
S5. apply — check 1: Accurate this-chunk addition; check 7: architecture-newgroup-occupied-resources.md · distinct

fanout-security-plan.md
1. apply — check 1: `D-retired`, its item the report's Retired bullet names (the eight dated gaps, `read_snapshot` as a product reader)
2. apply — check 1: `D-retired`, its item the report's Retired bullet names (the eight dated gaps, `read_snapshot` as a product reader)
3. apply — check 1: `D-retired`, its item the report's Retired bullet names (the eight dated gaps, `read_snapshot` as a product reader)
4. escalate → Q1 — check 1: Boundary widening (a reader moved onto `read_snapshot_or_replay`, so hook, CLI and MCP processes read `events.ndjson` on a new basis), the founder answered himself to P4 fork Q1 (`inputs#I3`): answer-card item · resolved: apply (Q1) — the card confirmed, `inputs#I6`, PROVISIONAL (delegate the overseer); the ratification stays the founder's `inputs#I3` Q1
5. apply — check 1: `D-retired`, its item the report's Retired bullet names (the eight dated gaps, `read_snapshot` as a product reader)
6. apply — check 1: `D-retired`, its item the report's Retired bullet names (the eight dated gaps, `read_snapshot` as a product reader)

fanout-security-plan--cascade-g1.md
1. escalate → Q1 — check 1: Boundary widening (a reader moved onto `read_snapshot_or_replay`, so hook, CLI and MCP processes read `events.ndjson` on a new basis), the founder answered himself to P4 fork Q1 (`inputs#I3`): answer-card item · resolved: apply (Q1) — the card confirmed, `inputs#I6`, PROVISIONAL (delegate the overseer); the ratification stays the founder's `inputs#I3` Q1
2. escalate → Q1 — check 1: Boundary widening (a reader moved onto `read_snapshot_or_replay`, so hook, CLI and MCP processes read `events.ndjson` on a new basis), the founder answered himself to P4 fork Q1 (`inputs#I3`): answer-card item · resolved: apply (Q1) — the card confirmed, `inputs#I6`, PROVISIONAL (delegate the overseer); the ratification stays the founder's `inputs#I3` Q1
3. apply — check 1: Accurate this-chunk addition (its fact in the report's Changes)
4. apply — check 1: Accurate this-chunk addition (its fact in the report's Changes)
5. escalate → Q1 — check 1: Boundary widening (a reader moved onto `read_snapshot_or_replay`, so hook, CLI and MCP processes read `events.ndjson` on a new basis), the founder answered himself to P4 fork Q1 (`inputs#I3`): answer-card item · resolved: apply (Q1) — the card confirmed, `inputs#I6`, PROVISIONAL (delegate the overseer); the ratification stays the founder's `inputs#I3` Q1
6. apply — check 1: Accurate this-chunk addition (its fact in the report's Changes)
7. apply — check 1: `D-retired`, its item the report's Retired bullet names (the eight dated gaps, `read_snapshot` as a product reader)
8. apply — check 1: `D-retired`, its item the report's Retired bullet names (the eight dated gaps, `read_snapshot` as a product reader)
9. apply — check 1: `D-retired`, its item the report's Retired bullet names (the eight dated gaps, `read_snapshot` as a product reader)
10. escalate → Q1 — check 1: Boundary widening (a reader moved onto `read_snapshot_or_replay`, so hook, CLI and MCP processes read `events.ndjson` on a new basis), the founder answered himself to P4 fork Q1 (`inputs#I3`): answer-card item · resolved: apply (Q1) — the card confirmed, `inputs#I6`, PROVISIONAL (delegate the overseer); the ratification stays the founder's `inputs#I3` Q1 · as: also re-point the paragraph's citation `src/cmd/run.rs:383` to `src/cmd/run.rs:418` (`collision_check`), settling citation-dispositions.md's `claim false` row (check 5)
11. escalate → Q1 — check 1: Boundary widening (a reader moved onto `read_snapshot_or_replay`, so hook, CLI and MCP processes read `events.ndjson` on a new basis), the founder answered himself to P4 fork Q1 (`inputs#I3`): answer-card item · resolved: apply (Q1) — the card confirmed, `inputs#I6`, PROVISIONAL (delegate the overseer); the ratification stays the founder's `inputs#I3` Q1
12. escalate → Q2 — check 7: contradiction against a standing line this same pass rewrites (companions fanout-security-plan--cascade-g1.md 20 and 21 retire the gaps of authentication-authorization.md:36-37) · resolved: apply (Q2) — with its companion, `inputs#I6` (delegate the overseer)
13. escalate → Q2 — check 7: contradiction against a standing line this same pass rewrites (its own replacement of authentication-authorization.md:34's set) · resolved: apply (Q2) — with its companion, `inputs#I6` (delegate the overseer)
14. apply — check 1: Accurate this-chunk addition (its fact in the report's Changes)
15. escalate → Q3 — check 1 no match: names an owner for `viola list`'s home-check inconsistency (report Count 14; directive `inputs#I5` item 6c), a route placement P5 lands · resolved: apply (Q3) — `inputs#I6` (delegate the overseer) · as: the owner is a CARRY on «Unix endpoint and home hardening» (P5 pins it); name it so, with no other owner
16. apply — check 1: Accurate this-chunk addition (its fact in the report's Changes)
17. escalate → Q2 — check 7: contradiction against a standing line this same pass rewrites (its own replacement of authentication-authorization.md:35's mcp clause) · resolved: apply (Q2) — with its companion, `inputs#I6` (delegate the overseer)
18. escalate → Q2 — check 7: contradiction against a standing line this same pass rewrites (companion fanout-security-plan--cascade-g1.md 20 retires authentication-authorization.md:36's hook gap) · resolved: apply (Q2) — with its companion, `inputs#I6` (delegate the overseer)
19. apply — check 1: Accurate this-chunk addition; check 7: security-plan-newgroup-authentication-authorization.md · distinct
20. apply — check 1: `D-retired`, its item the report's Retired bullet names (the eight dated gaps, `read_snapshot` as a product reader)
21. apply — check 1: `D-retired`, its item the report's Retired bullet names (the eight dated gaps, `read_snapshot` as a product reader)
22. apply — check 1: a recorded direction settles it: CARRY 17 closed by ruling, the founder's own word on the P5 card's Beyond-the-plan row 2 (`inputs#I4`)
23. apply — check 7: security-plan-newgroup-authentication-authorization.md · duplicate · lines 38–40, read: an edit of the standing group · as: add to lines 38 and 40 only the symbol names `unix_read_verdict` / `windows_read_verdict`, the pinned `READ_RIGHTS` = `0x9000_0001` and the Unix rule's reach to `diagnostics/`; restate no rule the lines hold
24. escalate → Q2 — check 7: contradiction against a standing line this same pass rewrites (companion fanout-security-plan--cascade-g1.md 20 retires authentication-authorization.md:36's stamps gap) · resolved: apply (Q2) — with its companion, `inputs#I6` (delegate the overseer)
25. apply — check 1: Accurate this-chunk addition; check 7: security-plan-newgroup-authentication-authorization.md · distinct
26. escalate → Q2 — check 7: contradiction against a standing line this same pass rewrites (it states the one per-OS difference beside «The outcomes match Unix» at authentication-authorization.md:41) · resolved: apply (Q2) — with its companion, `inputs#I6` (delegate the overseer)
27. apply — check 1: Accurate this-chunk addition; check 7: security-plan-newgroup-authentication-authorization.md · distinct
28. apply — check 1: Accurate this-chunk addition; check 7: security-plan-newgroup-authentication-authorization.md · distinct
29. escalate → Q1 — check 1: Boundary widening (a reader moved onto `read_snapshot_or_replay`, so hook, CLI and MCP processes read `events.ndjson` on a new basis), the founder answered himself to P4 fork Q1 (`inputs#I3`): answer-card item · resolved: apply (Q1) — the card confirmed, `inputs#I6`, PROVISIONAL (delegate the overseer); the ratification stays the founder's `inputs#I3` Q1
30. apply — check 1: Accurate this-chunk addition (its fact in the report's Changes)
31. apply — check 1: Accurate this-chunk addition (its fact in the report's Changes)
32. escalate → Q2 — check 7: contradiction against a standing line this same pass rewrites (companions fanout-security-plan--cascade-g1.md 20 and 21 retire the frame verbs' gaps at authentication-authorization.md:36-37) · resolved: apply (Q2) — with its companion, `inputs#I6` (delegate the overseer)
33. apply — check 1: Accurate this-chunk addition (its fact in the report's Changes)

fanout-security-plan--cascade-g2.md
1. apply — check 1: Accurate this-chunk addition (its fact in the report's Changes)
2. apply — check 1: `D-retired`, its item the report's Retired bullet names (the eight dated gaps, `read_snapshot` as a product reader)
3. apply — check 1: `D-retired`, its item the report's Retired bullet names (the eight dated gaps, `read_snapshot` as a product reader)
4. apply — check 1: `D-retired`, its item the report's Retired bullet names (the eight dated gaps, `read_snapshot` as a product reader)
5. apply — check 1: Accurate this-chunk addition (its fact in the report's Changes)
6. apply — check 1: Accurate this-chunk addition (its fact in the report's Changes)
7. apply-merged → fanout-security-plan.md 3 — check 2: the same body line, one edit (the doc agent's and the cascade's proposals of one fact)
8. escalate → Q1 — check 1: Boundary widening (a reader moved onto `read_snapshot_or_replay`, so hook, CLI and MCP processes read `events.ndjson` on a new basis), the founder answered himself to P4 fork Q1 (`inputs#I3`): answer-card item · resolved: apply (Q1) — the card confirmed, `inputs#I6`, PROVISIONAL (delegate the overseer); the ratification stays the founder's `inputs#I3` Q1
9. escalate → Q1 — check 1: Boundary widening (a reader moved onto `read_snapshot_or_replay`, so hook, CLI and MCP processes read `events.ndjson` on a new basis), the founder answered himself to P4 fork Q1 (`inputs#I3`): answer-card item; also a check-7 contradiction (companion fanout-security-plan--cascade-g2.md 8 rewrites input-validation.md:53's «two product callers»), Q2 · resolved: apply (Q1) — the card confirmed, `inputs#I6`, PROVISIONAL (delegate the overseer); the ratification stays the founder's `inputs#I3` Q1; its check-7 contradiction resolved with its companion (Q2)
10. apply — check 1: `D-retired`, its item the report's Retired bullet names (the eight dated gaps, `read_snapshot` as a product reader)
11. escalate → Q2 — check 7: contradiction against a standing line this same pass rewrites (its own replacement of input-validation.md:30's verify clause) · resolved: apply (Q2) — with its companion, `inputs#I6` (delegate the overseer)
12. apply-merged → fanout-security-plan.md 4 — check 2: the same body line, one edit (the doc agent's and the cascade's proposals of one fact)
13. escalate → Q1 — check 1: Boundary widening (a reader moved onto `read_snapshot_or_replay`, so hook, CLI and MCP processes read `events.ndjson` on a new basis), the founder answered himself to P4 fork Q1 (`inputs#I3`): answer-card item · resolved: apply (Q1) — the card confirmed, `inputs#I6`, PROVISIONAL (delegate the overseer); the ratification stays the founder's `inputs#I3` Q1
14. escalate → Q2 — check 7: contradiction against a standing line this same pass rewrites (companion fanout-security-plan--cascade-g2.md 15 adds `strict-modes-failed` to input-validation.md:80's closed list) · resolved: apply (Q2) — with its companion, `inputs#I6` (delegate the overseer)
15. apply — check 1: Accurate this-chunk addition (its fact in the report's Changes)
16. apply-merged → fanout-security-plan.md 6 — check 2: the same body line, one edit (the doc agent's and the cascade's proposals of one fact)
17. apply — check 1: Accurate this-chunk addition; check 7: security-plan-newgroup-data-protection.md · distinct
18. apply — check 1: Accurate this-chunk addition; check 7: security-plan-newgroup-data-protection.md · distinct
19. apply — check 1: Accurate this-chunk addition; check 7: security-plan-newgroup-data-protection.md · distinct
20. apply — check 1: Accurate this-chunk addition (its fact in the report's Changes)
21. apply — check 1: Accurate this-chunk addition; check 7: security-plan-newgroup-data-protection.md · distinct
22. apply — check 1: `D-retired`, its item the report's Retired bullet names (the eight dated gaps, `read_snapshot` as a product reader)
23. apply — check 1: Accurate this-chunk addition (its fact in the report's Changes)
24. apply — check 1: `D-retired`, its item the report's Retired bullet names (the eight dated gaps, `read_snapshot` as a product reader)
25. apply — check 1: `D-retired`, its item the report's Retired bullet names (the eight dated gaps, `read_snapshot` as a product reader)
26. apply — check 1: Accurate this-chunk addition (its fact in the report's Changes)
27. apply-merged → fanout-security-plan.md 5 — check 2: the same body line, one edit (the doc agent's and the cascade's proposals of one fact)

fanout-security-plan--cascade-new-error-handling.md
1. apply — check 1: Accurate this-chunk addition; check 7: security-plan-newgroup-error-handling.md · distinct

(sampler) cascade-compare-security-plan.md
S1. reject — check 7: security-plan-newgroup-authentication-authorization.md · twin · fanout-security-plan--cascade-g1.md 27
S2. escalate → Q1 — check 1: Boundary widening (a reader moved onto `read_snapshot_or_replay`, so hook, CLI and MCP processes read `events.ndjson` on a new basis), the founder answered himself to P4 fork Q1 (`inputs#I3`): answer-card item · resolved: apply (Q1) — the card confirmed, `inputs#I6`, PROVISIONAL (delegate the overseer); the ratification stays the founder's `inputs#I3` Q1
S3. escalate → Q1 — check 1: Boundary widening (a reader moved onto `read_snapshot_or_replay`, so hook, CLI and MCP processes read `events.ndjson` on a new basis), the founder answered himself to P4 fork Q1 (`inputs#I3`): answer-card item · resolved: apply (Q1) — the card confirmed, `inputs#I6`, PROVISIONAL (delegate the overseer); the ratification stays the founder's `inputs#I3` Q1
S4. apply — check 1: Accurate this-chunk addition; check 7: security-plan-newgroup-error-handling.md · distinct
S5. apply — check 1: Accurate this-chunk addition; check 7: security-plan-newgroup-error-handling.md · distinct
S6. apply — check 1: Accurate this-chunk addition; check 7: security-plan-newgroup-error-handling.md · distinct
S7. reject — check 7: security-plan-newgroup-error-handling.md · twin · fanout-security-plan--cascade-new-error-handling.md 1 (the `Refused` messages ride fanout-security-plan--cascade-g1.md 25)
S8. apply — check 1: Accurate this-chunk addition; check 7: security-plan-newgroup-error-handling.md · distinct
S9. apply — check 1: Accurate this-chunk addition; check 7: security-plan-newgroup-security-anti-patterns.md · distinct

fanout-design-system.md
1. apply — check 1: Accurate this-chunk addition (its fact in the report's Changes)
2. apply — check 1: Accurate this-chunk addition (its fact in the report's Changes)
3. apply — check 1: Accurate this-chunk addition (its fact in the report's Changes)
4. apply — check 1: Accurate this-chunk addition (its fact in the report's Changes)

fanout-design-system--cascade.md
1. escalate → Q2 — check 7: contradiction against a standing line this same pass rewrites (companion fanout-design-system--cascade.md 2 replaces surface.md:57's cause) · resolved: apply (Q2) — with its companion, `inputs#I6` (delegate the overseer)
2. apply — check 1: Accurate this-chunk addition (its fact in the report's Changes)
3. apply — check 1: Accurate this-chunk addition (its fact in the report's Changes)
4. apply-merged → fanout-design-system.md 2 — check 2: the same body line, one edit (the doc agent's and the cascade's proposals of one fact)
5. apply — check 1: Accurate this-chunk addition (its fact in the report's Changes)
6. escalate → Q3 — check 1 no match: names an owner for `viola list`'s home-check inconsistency (report Count 14; directive `inputs#I5` item 6c), a route placement P5 lands · resolved: apply (Q3) — `inputs#I6` (delegate the overseer) · as: the owner is a CARRY on «Unix endpoint and home hardening» (P5 pins it); name it so, with no other owner
7. apply — check 1: Accurate this-chunk addition; check 7: design-system-newgroup-surface.md · distinct
8. apply — check 1: Accurate this-chunk addition (its fact in the report's Changes)
9. apply — check 1: Accurate this-chunk addition (its fact in the report's Changes)
10. apply — check 1: Accurate this-chunk addition; check 7: design-system-newgroup-surface.md · distinct
11. apply — check 1: Accurate this-chunk addition; check 7: design-system-newgroup-surface.md · distinct
12. apply-merged → fanout-design-system.md 4 — check 2: the same body line, one edit (the doc agent's and the cascade's proposals of one fact)
13. apply — check 1: Accurate this-chunk addition (its fact in the report's Changes)
14. apply-merged → fanout-design-system.md 1 — check 2: the same body line, one edit (the doc agent's and the cascade's proposals of one fact)

(sampler) cascade-compare-design-system.md

fanout-layout-templates.md
1. apply — check 1: Accurate this-chunk addition (its fact in the report's Changes)
2. apply — check 1: Accurate this-chunk addition (its fact in the report's Changes)
3. apply — check 1: Accurate this-chunk addition (its fact in the report's Changes)
4. apply — check 1: Accurate this-chunk addition (its fact in the report's Changes)

fanout-test-plan.md
1. apply — check 1: Accurate this-chunk addition (its fact in the report's Changes)
2. apply — check 1: Accurate this-chunk addition (its fact in the report's Changes)
3. escalate → Q1 — check 1: Boundary widening (a reader moved onto `read_snapshot_or_replay`, so hook, CLI and MCP processes read `events.ndjson` on a new basis), the founder answered himself to P4 fork Q1 (`inputs#I3`): answer-card item · resolved: apply (Q1) — the card confirmed, `inputs#I6`, PROVISIONAL (delegate the overseer); the ratification stays the founder's `inputs#I3` Q1
4. escalate → Q1 — check 1: Boundary widening (a reader moved onto `read_snapshot_or_replay`, so hook, CLI and MCP processes read `events.ndjson` on a new basis), the founder answered himself to P4 fork Q1 (`inputs#I3`): answer-card item · resolved: apply (Q1) — the card confirmed, `inputs#I6`, PROVISIONAL (delegate the overseer); the ratification stays the founder's `inputs#I3` Q1
5. apply — check 1: Accurate this-chunk addition (its fact in the report's Changes)
6. apply — check 1: Accurate this-chunk addition (its fact in the report's Changes)
7. apply — check 1: Accurate this-chunk addition (its fact in the report's Changes)
8. apply — check 1: Accurate this-chunk addition (its fact in the report's Changes)
9. apply — check 1: Accurate this-chunk addition (its fact in the report's Changes)
10. apply — check 1: Accurate this-chunk addition (its fact in the report's Changes)
11. apply — check 1: Accurate this-chunk addition (its fact in the report's Changes)
12. apply — check 1: Accurate this-chunk addition (its fact in the report's Changes)
13. apply — check 1: Accurate this-chunk addition (its fact in the report's Changes)
14. apply — check 1: Accurate this-chunk addition (its fact in the report's Changes)
15. apply — check 1: Accurate this-chunk addition (its fact in the report's Changes)
16. escalate → Q1 — check 1: Boundary widening (a reader moved onto `read_snapshot_or_replay`, so hook, CLI and MCP processes read `events.ndjson` on a new basis), the founder answered himself to P4 fork Q1 (`inputs#I3`): answer-card item · resolved: apply (Q1) — the card confirmed, `inputs#I6`, PROVISIONAL (delegate the overseer); the ratification stays the founder's `inputs#I3` Q1
17. apply — check 1: Accurate this-chunk addition (its fact in the report's Changes)

fanout-test-plan--cascade-g1.md
1. apply — check 1: `D-retired`, its item the report's Retired bullet names (the eight dated gaps, `read_snapshot` as a product reader)
2. apply — check 1: Accurate this-chunk addition (its fact in the report's Changes)
3. apply — check 1: Accurate this-chunk addition (its fact in the report's Changes)
4. apply — check 1: Accurate this-chunk addition (its fact in the report's Changes)
5. escalate → Q1 — check 1: Boundary widening (a reader moved onto `read_snapshot_or_replay`, so hook, CLI and MCP processes read `events.ndjson` on a new basis), the founder answered himself to P4 fork Q1 (`inputs#I3`): answer-card item · resolved: apply (Q1) — the card confirmed, `inputs#I6`, PROVISIONAL (delegate the overseer); the ratification stays the founder's `inputs#I3` Q1
6. escalate → Q1 — check 1: Boundary widening (a reader moved onto `read_snapshot_or_replay`, so hook, CLI and MCP processes read `events.ndjson` on a new basis), the founder answered himself to P4 fork Q1 (`inputs#I3`): answer-card item · resolved: apply (Q1) — the card confirmed, `inputs#I6`, PROVISIONAL (delegate the overseer); the ratification stays the founder's `inputs#I3` Q1
7. apply — check 1: Accurate this-chunk addition; check 7: test-plan-newgroup-test-strategy.md · distinct
8. escalate → Q1 — check 1: Boundary widening (a reader moved onto `read_snapshot_or_replay`, so hook, CLI and MCP processes read `events.ndjson` on a new basis), the founder answered himself to P4 fork Q1 (`inputs#I3`): answer-card item · resolved: apply (Q1) — the card confirmed, `inputs#I6`, PROVISIONAL (delegate the overseer); the ratification stays the founder's `inputs#I3` Q1
9. escalate → Q1 — check 1: Boundary widening (a reader moved onto `read_snapshot_or_replay`, so hook, CLI and MCP processes read `events.ndjson` on a new basis), the founder answered himself to P4 fork Q1 (`inputs#I3`): answer-card item · resolved: apply (Q1) — the card confirmed, `inputs#I6`, PROVISIONAL (delegate the overseer); the ratification stays the founder's `inputs#I3` Q1
10. escalate → Q1 — check 1: Boundary widening (a reader moved onto `read_snapshot_or_replay`, so hook, CLI and MCP processes read `events.ndjson` on a new basis), the founder answered himself to P4 fork Q1 (`inputs#I3`): answer-card item · resolved: apply (Q1) — the card confirmed, `inputs#I6`, PROVISIONAL (delegate the overseer); the ratification stays the founder's `inputs#I3` Q1
11. apply — check 1: Accurate this-chunk addition; check 7: test-plan-newgroup-unit-test-strategy.md · distinct
12. apply — check 1: Accurate this-chunk addition (its fact in the report's Changes)
13. apply — check 1: Accurate this-chunk addition (its fact in the report's Changes)
14. apply — check 1: Accurate this-chunk addition; check 7: test-plan-newgroup-unit-test-strategy.md · distinct
15. apply — check 1: Accurate this-chunk addition; check 7: test-plan-newgroup-unit-test-strategy.md · distinct
16. apply — check 1: Accurate this-chunk addition; check 7: test-plan-newgroup-unit-test-strategy.md · distinct
17. apply — check 1: Accurate this-chunk addition; check 7: test-plan-newgroup-unit-test-strategy.md · distinct
18. apply — check 1: Accurate this-chunk addition (its fact in the report's Changes)
19. apply — check 1: Accurate this-chunk addition; check 7: test-plan-newgroup-integration-test-strategy.md · distinct
20. apply — check 1: Accurate this-chunk addition (its fact in the report's Changes)
21. apply — check 1: Accurate this-chunk addition (its fact in the report's Changes)
22. apply — check 1: Accurate this-chunk addition (its fact in the report's Changes)
23. apply — check 1: Accurate this-chunk addition; check 7: test-plan-newgroup-integration-test-strategy.md · distinct
24. apply — check 1: Accurate this-chunk addition (its fact in the report's Changes)
25. apply — check 1: Accurate this-chunk addition (its fact in the report's Changes)
26. escalate → Q2 — check 7: contradiction against a standing line this same pass rewrites (companion fanout-test-plan--cascade-g1.md 24 rewrites integration-test-strategy.md:58's mapping) · resolved: apply (Q2) — with its companion, `inputs#I6` (delegate the overseer)
27. apply — check 1: Accurate this-chunk addition; check 7: test-plan-newgroup-integration-test-strategy.md · distinct
28. apply — check 1: Accurate this-chunk addition; check 7: test-plan-newgroup-integration-test-strategy.md · distinct
29. apply — check 1: Accurate this-chunk addition; check 7: test-plan-newgroup-integration-test-strategy.md · distinct
30. apply — check 1: Accurate this-chunk addition (its fact in the report's Changes)
31. apply — check 1: Accurate this-chunk addition (its fact in the report's Changes)
32. apply-merged → fanout-test-plan.md 8 — check 2: the same body line, one edit (the doc agent's and the cascade's proposals of one fact)
33. apply-merged → fanout-test-plan.md 15 — check 2: the same body line, one edit (the doc agent's and the cascade's proposals of one fact)

fanout-test-plan--cascade-g2.md
1. apply — check 1: Accurate this-chunk addition (its fact in the report's Changes)
2. escalate → Q1 — check 1: Boundary widening (a reader moved onto `read_snapshot_or_replay`, so hook, CLI and MCP processes read `events.ndjson` on a new basis), the founder answered himself to P4 fork Q1 (`inputs#I3`): answer-card item · resolved: apply (Q1) — the card confirmed, `inputs#I6`, PROVISIONAL (delegate the overseer); the ratification stays the founder's `inputs#I3` Q1
3. escalate → Q1 — check 1: Boundary widening (a reader moved onto `read_snapshot_or_replay`, so hook, CLI and MCP processes read `events.ndjson` on a new basis), the founder answered himself to P4 fork Q1 (`inputs#I3`): answer-card item · resolved: apply (Q1) — the card confirmed, `inputs#I6`, PROVISIONAL (delegate the overseer); the ratification stays the founder's `inputs#I3` Q1
4. escalate → Q1 — check 1: Boundary widening (a reader moved onto `read_snapshot_or_replay`, so hook, CLI and MCP processes read `events.ndjson` on a new basis), the founder answered himself to P4 fork Q1 (`inputs#I3`): answer-card item · resolved: apply (Q1) — the card confirmed, `inputs#I6`, PROVISIONAL (delegate the overseer); the ratification stays the founder's `inputs#I3` Q1
5. apply-merged → fanout-test-plan.md 1 — check 2: the same body line, one edit (the doc agent's and the cascade's proposals of one fact)
6. apply-merged → fanout-test-plan.md 3 — check 2: the same body line, one edit (the doc agent's and the cascade's proposals of one fact)
7. apply-merged → fanout-test-plan.md 2 — check 2: the same body line, one edit (the doc agent's and the cascade's proposals of one fact)
8. apply — check 1: Accurate this-chunk addition (its fact in the report's Changes)
9. apply-merged → fanout-test-plan.md 10 — check 2: the same body line, one edit (the doc agent's and the cascade's proposals of one fact)
10. apply — check 1: Accurate this-chunk addition (its fact in the report's Changes)
11. apply — check 1: Accurate this-chunk addition (its fact in the report's Changes)
12. apply — check 1: Accurate this-chunk addition (its fact in the report's Changes)
13. apply-merged → fanout-test-plan.md 7 — check 2: the same body line, one edit (the doc agent's and the cascade's proposals of one fact)
14. apply — check 1: Accurate this-chunk addition (its fact in the report's Changes)
15. apply — check 1: Accurate this-chunk addition (its fact in the report's Changes)
16. escalate → Q2 — check 7: contradiction against a standing line this same pass rewrites (companion fanout-test-plan--cascade-g2.md 14 replaces e2e-test-strategy.md:303's owed clause) · resolved: apply (Q2) — with its companion, `inputs#I6` (delegate the overseer)
17. apply — check 1: Accurate this-chunk addition; check 7: test-plan-newgroup-e2e-test-strategy.md · distinct
18. apply-merged → fanout-test-plan.md 5 — check 2: the same body line, one edit (the doc agent's and the cascade's proposals of one fact)
19. escalate → Q1 — check 1: Boundary widening (a reader moved onto `read_snapshot_or_replay`, so hook, CLI and MCP processes read `events.ndjson` on a new basis), the founder answered himself to P4 fork Q1 (`inputs#I3`): answer-card item; also a check-7 contradiction (companion fanout-test-plan--cascade-g2.md 4 rewrites e2e-test-strategy.md:237), Q2 · resolved: apply (Q1) — the card confirmed, `inputs#I6`, PROVISIONAL (delegate the overseer); the ratification stays the founder's `inputs#I3` Q1; its check-7 contradiction resolved with its companion (Q2)
20. apply — check 1: Accurate this-chunk addition (its fact in the report's Changes)
21. apply — check 1: Accurate this-chunk addition; check 7: test-plan-newgroup-test-data-fixtures.md · distinct
22. apply — check 1: Accurate this-chunk addition; check 7: test-plan-newgroup-test-data-fixtures.md · distinct
23. apply — check 1: Accurate this-chunk addition (its fact in the report's Changes)

(sampler) cascade-compare-test-plan.md
S1. apply — check 1: Accurate this-chunk addition; check 7: test-plan-newgroup-mocking-stubbing-discipline.md · distinct
S2. reject — check 7: test-plan-newgroup-test-data-fixtures.md · twin · fanout-test-plan--cascade-g2.md 21
S3. reject — check 7: test-plan-newgroup-test-data-fixtures.md · twin · fanout-test-plan--cascade-g2.md 22
S4. apply — check 1: Accurate this-chunk addition; check 7: test-plan-newgroup-test-data-fixtures.md · distinct
S5. apply — check 1: Accurate this-chunk addition; check 7: test-plan-newgroup-test-scope-summary.md · distinct
S6. apply — check 1: Accurate this-chunk addition; check 7: test-plan-newgroup-test-strategy.md · distinct
S7. apply — check 1: Accurate this-chunk addition; check 7: test-plan-newgroup-test-strategy.md · distinct

fanout-obs-plan.md
1. apply — check 1: Accurate this-chunk addition (its fact in the report's Changes)
2. apply — check 1: Accurate this-chunk addition (its fact in the report's Changes)
3. apply — check 1: Accurate this-chunk addition (its fact in the report's Changes)
4. apply — check 1: Accurate this-chunk addition (its fact in the report's Changes)
5. apply — check 1: Accurate this-chunk addition (its fact in the report's Changes)
6. apply — check 1: Accurate this-chunk addition (its fact in the report's Changes)
7. apply — check 1: Accurate this-chunk addition (its fact in the report's Changes)
8. apply — check 1: Accurate this-chunk addition (its fact in the report's Changes)
9. apply — check 1: `D-retired`, its item the report's Retired bullet names (the eight dated gaps, `read_snapshot` as a product reader)
10. apply — check 1: `D-retired`, its item the report's Retired bullet names (the eight dated gaps, `read_snapshot` as a product reader)
11. escalate → Q1 — check 1: Boundary widening (a reader moved onto `read_snapshot_or_replay`, so hook, CLI and MCP processes read `events.ndjson` on a new basis), the founder answered himself to P4 fork Q1 (`inputs#I3`): answer-card item · resolved: apply (Q1) — the card confirmed, `inputs#I6`, PROVISIONAL (delegate the overseer); the ratification stays the founder's `inputs#I3` Q1
12. apply — check 1: Accurate this-chunk addition (its fact in the report's Changes)

fanout-obs-plan--cascade-g1.md
1. apply — check 1: Accurate this-chunk addition; check 7: obs-plan-newgroup-obs-scope-summary.md · distinct
2. escalate → Q1 — check 1: Boundary widening (a reader moved onto `read_snapshot_or_replay`, so hook, CLI and MCP processes read `events.ndjson` on a new basis), the founder answered himself to P4 fork Q1 (`inputs#I3`): answer-card item · resolved: apply (Q1) — the card confirmed, `inputs#I6`, PROVISIONAL (delegate the overseer); the ratification stays the founder's `inputs#I3` Q1
3. apply — check 1: Accurate this-chunk addition; check 7: obs-plan-newgroup-obs-scope-summary.md · distinct
4. apply — check 1: Accurate this-chunk addition; check 7: obs-plan-newgroup-obs-scope-summary.md · distinct
5. apply — check 1: Accurate this-chunk addition (its fact in the report's Changes)
6. escalate → Q2 — check 7: contradiction against a standing line this same pass rewrites (companion fanout-obs-plan--cascade-g2.md 5 replaces obs-scope-summary.md:370's «one arm») · resolved: apply (Q2) — with its companion, `inputs#I6` (delegate the overseer)
7. apply — check 1: Accurate this-chunk addition (its fact in the report's Changes)
8. apply — check 1: Accurate this-chunk addition; check 7: obs-plan-newgroup-obs-scope-summary.md · distinct
9. apply — check 1: Accurate this-chunk addition (its fact in the report's Changes)
10. apply — check 1: Accurate this-chunk addition (its fact in the report's Changes)
11. apply — check 1: Accurate this-chunk addition (its fact in the report's Changes)

fanout-obs-plan--cascade-g2.md
1. apply — check 1: Accurate this-chunk addition (its fact in the report's Changes)
2. escalate → Q1 — check 1: Boundary widening (a reader moved onto `read_snapshot_or_replay`, so hook, CLI and MCP processes read `events.ndjson` on a new basis), the founder answered himself to P4 fork Q1 (`inputs#I3`): answer-card item · resolved: apply (Q1) — the card confirmed, `inputs#I6`, PROVISIONAL (delegate the overseer); the ratification stays the founder's `inputs#I3` Q1
3. apply — check 1: Accurate this-chunk addition (its fact in the report's Changes)
4. apply — check 1: Accurate this-chunk addition (its fact in the report's Changes)
5. apply — check 1: Accurate this-chunk addition (its fact in the report's Changes)
6. reject — check 7: obs-plan-newgroup-obs-scope-summary.md · contradiction; main: the same F5 fact as fanout-obs-plan--cascade-g1.md 6 in the same file, line 370 carried by its companion fanout-obs-plan--cascade-g2.md 5
7. reject — check 7: obs-plan-newgroup-obs-scope-summary.md · twin · fanout-obs-plan--cascade-g1.md 8
8. apply — check 1: Accurate this-chunk addition (its fact in the report's Changes)
9. apply — check 1: Accurate this-chunk addition (its fact in the report's Changes)
10. apply — check 1: Accurate this-chunk addition (its fact in the report's Changes)
11. reject — check 7: obs-plan-newgroup-obs-scope-summary.md · twin · fanout-obs-plan--cascade-g1.md 1
12. reject — check 7: obs-plan-newgroup-obs-scope-summary.md · twin · fanout-obs-plan--cascade-g1.md 2
13. apply — check 1: Accurate this-chunk addition (its fact in the report's Changes)
14. apply — check 1: Accurate this-chunk addition (its fact in the report's Changes)
15. reject — check 7: obs-plan-newgroup-obs-scope-summary.md · twin · fanout-obs-plan--cascade-g1.md 3
16. reject — check 7: obs-plan-newgroup-obs-scope-summary.md · twin · fanout-obs-plan--cascade-g1.md 4

fanout-obs-plan--cascade-g3.md
1. apply — check 1: Accurate this-chunk addition (its fact in the report's Changes)
2. apply — check 1: Accurate this-chunk addition (its fact in the report's Changes)
3. apply — check 1: Accurate this-chunk addition (its fact in the report's Changes)
4. apply — check 1: Accurate this-chunk addition (its fact in the report's Changes)
5. apply — check 1: Accurate this-chunk addition (its fact in the report's Changes)
6. apply — check 1: Accurate this-chunk addition (its fact in the report's Changes)
7. apply — check 1: Accurate this-chunk addition (its fact in the report's Changes)
8. escalate → Q1 — check 1: Boundary widening (a reader moved onto `read_snapshot_or_replay`, so hook, CLI and MCP processes read `events.ndjson` on a new basis), the founder answered himself to P4 fork Q1 (`inputs#I3`): answer-card item · resolved: apply (Q1) — the card confirmed, `inputs#I6`, PROVISIONAL (delegate the overseer); the ratification stays the founder's `inputs#I3` Q1
9. escalate → Q1 — check 1: Boundary widening (a reader moved onto `read_snapshot_or_replay`, so hook, CLI and MCP processes read `events.ndjson` on a new basis), the founder answered himself to P4 fork Q1 (`inputs#I3`): answer-card item; also a check-7 contradiction (companions fanout-obs-plan--cascade-g3.md 8 and 25 rewrite span-trace-coverage.md:30 and :192), Q2 · resolved: apply (Q1) — the card confirmed, `inputs#I6`, PROVISIONAL (delegate the overseer); the ratification stays the founder's `inputs#I3` Q1; its check-7 contradiction resolved with its companion (Q2)
10. escalate → Q1 — check 1: Boundary widening (a reader moved onto `read_snapshot_or_replay`, so hook, CLI and MCP processes read `events.ndjson` on a new basis), the founder answered himself to P4 fork Q1 (`inputs#I3`): answer-card item · resolved: apply (Q1) — the card confirmed, `inputs#I6`, PROVISIONAL (delegate the overseer); the ratification stays the founder's `inputs#I3` Q1
11. apply — check 1: Accurate this-chunk addition (its fact in the report's Changes)
12. apply — check 1: Accurate this-chunk addition; check 7: obs-plan-newgroup-span-trace-coverage.md · distinct
13. escalate → Q1 — check 1: Boundary widening (a reader moved onto `read_snapshot_or_replay`, so hook, CLI and MCP processes read `events.ndjson` on a new basis), the founder answered himself to P4 fork Q1 (`inputs#I3`): answer-card item; also a check-7 contradiction (companions fanout-obs-plan--cascade-g3.md 24 and 25 rewrite span-trace-coverage.md:191-192), Q2 · resolved: apply (Q1) — the card confirmed, `inputs#I6`, PROVISIONAL (delegate the overseer); the ratification stays the founder's `inputs#I3` Q1; its check-7 contradiction resolved with its companion (Q2)
14. apply — check 1: Accurate this-chunk addition (its fact in the report's Changes)
15. apply — check 1: Accurate this-chunk addition (its fact in the report's Changes)
16. escalate → Q1 — check 1: Boundary widening (a reader moved onto `read_snapshot_or_replay`, so hook, CLI and MCP processes read `events.ndjson` on a new basis), the founder answered himself to P4 fork Q1 (`inputs#I3`): answer-card item · resolved: apply (Q1) — the card confirmed, `inputs#I6`, PROVISIONAL (delegate the overseer); the ratification stays the founder's `inputs#I3` Q1
17. apply — check 1: Accurate this-chunk addition (its fact in the report's Changes)
18. apply — check 1: `D-retired`, its item the report's Retired bullet names (the eight dated gaps, `read_snapshot` as a product reader)
19. apply-merged → fanout-obs-plan.md 9 — check 2: the same body line, one edit (the doc agent's and the cascade's proposals of one fact)
20. apply — check 1: Accurate this-chunk addition (its fact in the report's Changes)
21. apply — check 1: Accurate this-chunk addition (its fact in the report's Changes)
22. apply — check 1: Accurate this-chunk addition (its fact in the report's Changes)
23. apply — check 1: Accurate this-chunk addition (its fact in the report's Changes)
24. escalate → Q1 — check 1: Boundary widening (a reader moved onto `read_snapshot_or_replay`, so hook, CLI and MCP processes read `events.ndjson` on a new basis), the founder answered himself to P4 fork Q1 (`inputs#I3`): answer-card item · resolved: apply (Q1) — the card confirmed, `inputs#I6`, PROVISIONAL (delegate the overseer); the ratification stays the founder's `inputs#I3` Q1
25. escalate → Q1 — check 1: Boundary widening (a reader moved onto `read_snapshot_or_replay`, so hook, CLI and MCP processes read `events.ndjson` on a new basis), the founder answered himself to P4 fork Q1 (`inputs#I3`): answer-card item · resolved: apply (Q1) — the card confirmed, `inputs#I6`, PROVISIONAL (delegate the overseer); the ratification stays the founder's `inputs#I3` Q1
26. apply — check 1: Accurate this-chunk addition (its fact in the report's Changes)

fanout-obs-plan--cascade-g4.md
1. apply — check 1: Accurate this-chunk addition (its fact in the report's Changes)
2. escalate → Q2 — check 7: contradiction against a standing line this same pass rewrites (companion fanout-obs-plan--cascade-g4.md 1 rewrites log-coverage.md:10's word list) · resolved: apply (Q2) — with its companion, `inputs#I6` (delegate the overseer)
3. apply — check 1: Accurate this-chunk addition (its fact in the report's Changes)
4. apply — check 1: Accurate this-chunk addition (its fact in the report's Changes)
5. escalate → Q1 — check 1: Boundary widening (a reader moved onto `read_snapshot_or_replay`, so hook, CLI and MCP processes read `events.ndjson` on a new basis), the founder answered himself to P4 fork Q1 (`inputs#I3`): answer-card item · resolved: apply (Q1) — the card confirmed, `inputs#I6`, PROVISIONAL (delegate the overseer); the ratification stays the founder's `inputs#I3` Q1
6. apply — check 1: Accurate this-chunk addition (its fact in the report's Changes)
7. apply — check 1: Accurate this-chunk addition (its fact in the report's Changes)
8. apply — check 1: Accurate this-chunk addition; check 7: obs-plan-newgroup-log-coverage.md · distinct
9. apply — check 1: Accurate this-chunk addition (its fact in the report's Changes)
10. apply — check 1: Accurate this-chunk addition; check 7: obs-plan-newgroup-error-capture-reporting.md · distinct
11. apply — check 1: Accurate this-chunk addition (its fact in the report's Changes)
12. apply — check 1: Accurate this-chunk addition (its fact in the report's Changes)
13. apply — check 1: Accurate this-chunk addition (its fact in the report's Changes)
14. apply — check 1: Accurate this-chunk addition (its fact in the report's Changes)
15. apply — check 1: Accurate this-chunk addition (its fact in the report's Changes)
16. apply — check 1: Accurate this-chunk addition (its fact in the report's Changes)

fanout-obs-plan--cascade-new-error-capture-reporting.md
1. apply — check 1: Accurate this-chunk addition; check 7: obs-plan-newgroup-error-capture-reporting.md · distinct

(sampler) cascade-compare-obs-plan.md
S1. apply — check 1: Accurate this-chunk addition; check 7: obs-plan-newgroup-error-capture-reporting.md · distinct
S2. apply — check 1: Accurate this-chunk addition; check 7: obs-plan-newgroup-metric-coverage.md · distinct
S3. escalate → Q1 — check 1: Boundary widening (a reader moved onto `read_snapshot_or_replay`, so hook, CLI and MCP processes read `events.ndjson` on a new basis), the founder answered himself to P4 fork Q1 (`inputs#I3`): answer-card item · resolved: apply (Q1) — the card confirmed, `inputs#I6`, PROVISIONAL (delegate the overseer); the ratification stays the founder's `inputs#I3` Q1
S4. apply — check 1: Accurate this-chunk addition; check 7: obs-plan-newgroup-pii-scrubbing-compliance.md · distinct
S5. apply — check 1: Accurate this-chunk addition; check 7: obs-plan-newgroup-pii-scrubbing-compliance.md · distinct
S6. apply — check 1: Accurate this-chunk addition (its fact in the report's Changes)
S7. apply — check 1: Accurate this-chunk addition; check 7: obs-plan-newgroup-slo-invariants-telemetry-budgets.md · distinct
S8. escalate → Q2 — check 7: contradiction against a standing line this same pass rewrites (companion fanout-obs-plan--cascade-g3.md 18 retires span-trace-coverage.md:102's clause) · resolved: apply (Q2) — with its companion, `inputs#I6` (delegate the overseer)
S9. apply — check 1: Accurate this-chunk addition; check 7: obs-plan-newgroup-span-trace-coverage.md · distinct

fanout-a11y-plan.md

fanout-a11y-plan--cascade.md
1. reject — check 2: main: its facts land as cascade-compare-a11y-plan.md S1 and S2, in the file's entity form, which check 7 read distinct
2. apply — check 1: Accurate this-chunk addition; check 7: a11y-plan-newgroup-cognitive-accessibility.md · distinct
3. apply — check 1: Accurate this-chunk addition; check 7: a11y-plan-newgroup-cognitive-accessibility.md · distinct
4. apply — check 1: Accurate this-chunk addition; check 7: a11y-plan-newgroup-cognitive-accessibility.md · distinct
5. apply — check 1: Accurate this-chunk addition; check 7: a11y-plan-newgroup-cognitive-accessibility.md · distinct
6. apply — check 1: Accurate this-chunk addition (its fact in the report's Changes)
7. apply — check 1: Accurate this-chunk addition (its fact in the report's Changes)

(sampler) cascade-compare-a11y-plan.md
S1. apply — check 1: Accurate this-chunk addition; check 7: a11y-plan-newgroup-a11y-scope-summary.md · distinct
S2. apply — check 1: Accurate this-chunk addition; check 7: a11y-plan-newgroup-a11y-scope-summary.md · distinct
S3. apply — check 1: Accurate this-chunk addition; check 7: a11y-plan-newgroup-a11y-scope-summary.md · distinct
S4. reject — check 7: a11y-plan-newgroup-cognitive-accessibility.md · twin · fanout-a11y-plan--cascade.md 3
S5. reject — check 2: main: the same verification fact as fanout-a11y-plan--cascade.md 6 in the same file (check 7 read it against standing groups only)
