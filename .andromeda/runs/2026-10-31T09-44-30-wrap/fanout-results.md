# Fan-out results — 2026-10-31-sanitised-error-surfaces

Forced sample (`cascade-sample.txt`). The `stage1` task writers of architecture, security-plan and obs-plan re-spawned once on `UNTOUCHED_REACHED` (`stage1-r1`, 3 of 3); one classifier, `test-plan-cls-e2e-test-strategy.b2`, re-spawned once on `DIED` / `ABSENT` on the operator's word (`classify-r1`, 1 of 1, `inputs#I4`).

architecture: 6 proposals · fanout-architecture.md · 27 cascade proposals · fanout-architecture--cascade.md, fanout-architecture--cascade-new-cross-cutting-patterns.md, fanout-architecture--cascade-new-standard-contracts.md · 15 facts · 15 placed · 0 no home · (sampler) 1 lost
note: D-arch-decisions no drift; the standing detail sites are the cascade's; the CARRY on «Version done-check» is P5's route edit.

security-plan: 3 proposals · fanout-security-plan.md · 20 cascade proposals · fanout-security-plan--cascade-g1.md, fanout-security-plan--cascade-g2.md · 13 facts · 13 placed · 0 no home · (sampler) 5 lost
note: D-security-input, D-security-auth, D-security-deps hold; the two body edits at `security-plan.md:411` and `:413` may meet a body pass's edit of the same lines.

design-system: 0 proposals · fanout-design-system.md · 12 cascade proposals · fanout-design-system--cascade.md, fanout-design-system--cascade-new-surface.md · 8 facts · 7 placed · 1 no home · (sampler) 2 lost
note: every drift site stands in a detail group, so the cascade's; F8 no home, its census belonging to test-plan §6.

layout-templates: 2 proposals · fanout-layout-templates.md
note: D-layout-surface and D-retired fired nothing.

test-plan: 4 proposals · fanout-test-plan.md · 39 cascade proposals · fanout-test-plan--cascade-g1.md, fanout-test-plan--cascade-g2.md, fanout-test-plan--cascade-g3.md, fanout-test-plan--cascade-g4.md, fanout-test-plan--cascade-g5.md, fanout-test-plan--cascade-new-unit-test-strategy.md · 20 facts · 20 placed · 0 no home · (sampler) 5 lost
note: the report does not say which step reads `diagnostics/` 0600 and the no-`Cookie` clause now that the helper is recorded as not built (`fanout-test-plan.md`, `fanout-test-plan--cascade-g4.md`).

obs-plan: 3 proposals · fanout-obs-plan.md · 27 cascade proposals · fanout-obs-plan--cascade-g1.md, fanout-obs-plan--cascade-g2.md, fanout-obs-plan--cascade-g3.md, fanout-obs-plan--cascade-g4.md, fanout-obs-plan--cascade-new-error-capture-reporting.md · 15 facts · 14 placed · 1 no home · (sampler) 6 lost
note: `obs-scope-summary#20.5` and `slo-invariants-telemetry-budgets#2.2` label as `max` what the report labels `mean`, unresolved by the report; F9 no home.

a11y-plan: 0 proposals · fanout-a11y-plan.md · 15 cascade proposals · fanout-a11y-plan--cascade.md · 7 facts · 7 placed · 0 no home · (sampler) 2 lost
note: `a11y-plan.md:270` and `:346` tie a code to the word `unable`, while the report has a word in `refused: <word>` that stays visible; no proposal.

## Dispositions

Directive `inputs#I3` item 3 governs check 7: every `contradiction` verdict escalates as written, a companion rewriting its line or not.

fanout-architecture.md
1. apply — check 5: plan's entry, report Retired and Counts, two named plus three the census adds.
2. apply — check 6: `CoreError` disproved, plan's entry · as: end with "nine `thiserror` derives over the product sources in all, by `command grep -rn "thiserror::Error" src crates --include=*.rs`".
3. apply — check 5: plan's entry §Inherited Defaults, Errors.
4. apply — check 1: accurate this-chunk addition, report `refusal-enum` @c1247.
5. apply-merged → fanout-architecture--cascade.md 15 — check 2: same line, the broad form kept, the table standing in `standard-contracts.md`.
6. apply-merged → fanout-architecture--cascade.md 13 — check 2: same line, the broad form kept, the definition standing in `conventions.md`.

fanout-architecture--cascade.md
1. apply — check 6: exit 11 disproved at `established-decisions.md:130`.
2. escalate → Q9 — check 7: architecture-newgroup-established-decisions.md · contradiction · lines 130-130. · resolved: apply (Q9)
3. apply — check 5: plan's entry, the owner per `inputs#I2`, PROVISIONAL.
4. escalate → Q10 — check 7: architecture-newgroup-established-decisions.md · contradiction · lines 267-267. · resolved: apply (Q10)
5. escalate → Q10 — check 7: architecture-newgroup-established-decisions.md · contradiction · lines 267-267. · resolved: apply (Q10)
6. apply-merged → fanout-architecture.md 1 — check 2: same line, "two named plus three" the report's own reading.
7. apply-merged → fanout-architecture.md 2 — check 2: same line.
8. apply-merged → fanout-architecture.md 3 — check 2: same line.
9. apply — check 5: plan's entry, `unknown`'s detail a code word or null.
10. apply — check 5: plan's entry, a reader's opaque rule kept, `code-word`.
11. apply — check 7: architecture-newgroup-conventions.md · distinct.
12. escalate → Q8 — check 7: architecture-newgroup-conventions.md · contradiction · lines 15-15. · resolved: apply (Q8)
13. apply — check 5: plan's entry, `code-word`.
14. apply — check 1: `fault-reader` names `conventions.md:64`.
15. apply — check 5: plan's entry, `wrapper-fault`'s fields closed.
16. apply — check 7: architecture-newgroup-standard-contracts.md · distinct.
17. apply — check 7: architecture-newgroup-standard-contracts.md · distinct.
18. apply — check 6: exit 11 disproved, plan's entry §Occupied Resources (`mcp`).
19. apply — check 6: `mcp` writing a `process-exit` disproved · as: the `mcp.ndjson` list reads "and no self `process-exit`: none on stdin EOF or at that cut (`command grep -n 'ObsEvent::ProcessExit' -r src crates --include=*.rs`), and none when Claude Code ends it by a signal, as measured live on claude 2.1.287 at that chunk;", the live measurement kept.
20. apply — check 7: architecture-newgroup-occupied-resources.md · distinct.
21. apply — check 7: architecture-newgroup-occupied-resources.md · distinct.
22. apply — check 7: architecture-newgroup-occupied-resources.md · distinct.

fanout-architecture--cascade-new-cross-cutting-patterns.md
1. apply — check 7: architecture-newgroup-cross-cutting-patterns.md · distinct.
2. apply — check 7: architecture-newgroup-cross-cutting-patterns.md · distinct.
3. apply — check 7: architecture-newgroup-cross-cutting-patterns.md · distinct.
4. apply — check 7: architecture-newgroup-cross-cutting-patterns.md · distinct.

fanout-architecture--cascade-new-standard-contracts.md
1. apply — check 7: architecture-newgroup-standard-contracts.md · distinct.

(sampler) cascade-compare-architecture.md
S1. reject — check 7: architecture-newgroup-cross-cutting-patterns.md · twin · fanout-architecture--cascade-new-cross-cutting-patterns.md 1.

fanout-security-plan.md
1. apply-merged → fanout-security-plan--cascade-g2.md 1 — check 2: same line, a body holds no "struck" history.
2. apply — check 6: `CoreError` disproved, the same claim worded without its token.
3. apply-merged → fanout-security-plan--cascade-g2.md 2 — check 2: same line, the table standing in `error-handling.md`.

fanout-security-plan--cascade-g1.md
1. apply — check 1: verbatim copy kept current, report Coverage of new surfaces.
2. apply — check 1: accurate this-chunk addition, report Symbols / APIs.
3. apply — check 1: accurate this-chunk addition, report Coverage of new surfaces.
4. apply — check 6: exit 11 disproved, plan's entry §Input Validation.
5. apply — check 1: `refusal-enum` names `input-validation.md:74`.

fanout-security-plan--cascade-g2.md
1. apply — check 5: plan's entry, the enum list by its named nine.
2. apply — check 5: plan's entry, the clients' screen and `sanitised_fault`.
3. apply — check 7: security-plan-newgroup-error-handling.md · distinct.
4. apply — check 7: security-plan-newgroup-error-handling.md · distinct.
5. apply — check 7: security-plan-newgroup-error-handling.md · distinct.
6. apply — check 7: security-plan-newgroup-error-handling.md · distinct.
7. apply — check 7: security-plan-newgroup-error-handling.md · distinct.
8. apply — check 7: security-plan-newgroup-error-handling.md · distinct.
9. apply — check 7: security-plan-newgroup-error-handling.md · distinct.
10. apply — check 7: security-plan-newgroup-error-handling.md · distinct.
11. apply — check 7: security-plan-newgroup-error-handling.md · distinct.
12. apply — check 7: security-plan-newgroup-error-handling.md · distinct.
13. apply — check 7: security-plan-newgroup-error-handling.md · distinct.
14. apply — check 7: security-plan-newgroup-error-handling.md · distinct.
15. apply — check 7: security-plan-newgroup-error-handling.md · distinct.

(sampler) cascade-compare-security-plan.md
S1. reject — check 7: security-plan-newgroup-error-handling.md · twin · fanout-security-plan--cascade-g2.md 3.
S2. reject — check 7: security-plan-newgroup-error-handling.md · twin · fanout-security-plan--cascade-g2.md 6.
S3. apply — check 7: security-plan-newgroup-error-handling.md · distinct.
S4. reject — check 7: security-plan-newgroup-error-handling.md · twin · fanout-security-plan--cascade-g2.md 8.
S5. reject — check 7: security-plan-newgroup-error-handling.md · twin · fanout-security-plan--cascade-g2.md 12.

fanout-design-system--cascade.md
1. apply — check 5: plan's entry, row 14.
2. escalate → Q1 — check 2: its kept clause «their detail goes only to `diagnostics/`» is the line Q1 rules on. · resolved: apply (Q1) · as: the line's last clause reads «because they are faults, not refusals: the human line holds the code only, further detail goes to `diagnostics/`, and the `--json` document's `detail` is row 20's closed object.»
3. apply — check 5: plan's entry §Semantic Colors.
4. apply — check 7: design-system-newgroup-surface.md · distinct.
5. escalate → Q6 — check 7: design-system-newgroup-surface.md · contradiction · lines 147-147. · resolved: apply (Q6)
6. apply — check 7: design-system-newgroup-surface.md · distinct.
7. escalate → Q5 — check 7: design-system-newgroup-surface.md · contradiction · lines 142-142. · resolved: apply (Q5) · as: line 142's row reads `| 2 | clap usage text (never produced by `viola hook`), or the fixed `UNUSABLE_RESPONSE` line (an oversize `viola answer`) or `UNUSABLE_TEXT` line (an unusable `viola send`) on stderr, whatever `--json` says; `tests/contract_error_bodies.rs` names them in `NOT_ERROR_BODIES` | no document |`, the clap text kept
8. escalate → Q1 — check 7: design-system-newgroup-surface.md · contradiction · lines 81-81. · resolved: apply (Q1)
9. escalate → Q1 — check 7: design-system-newgroup-surface.md · contradiction · lines 81-81. · resolved: apply (Q1)
10. escalate → Q5 — check 6: dependent of 7. · resolved: apply (Q5)
11. escalate → Q5 — check 6: dependent of 7. · resolved: apply (Q5)

fanout-design-system--cascade-new-surface.md
1. apply — check 7: design-system-newgroup-surface.md · distinct.

(sampler) cascade-compare-design-system.md
S1. escalate → Q6 — check 7: design-system-newgroup-surface.md · contradiction · lines 147-147. · resolved: reject (Q6)
S2. escalate → Q1 — check 7: design-system-newgroup-surface.md · contradiction · lines 81-81. · resolved: reject (Q1)

fanout-layout-templates.md
1. apply — check 5: plan's entry §Component — Primary content block 2.
2. reject — re-derivation tell: its rationale rests on the detector's own grep of two masters; the fact stands on design-system's exit-20 row.

fanout-test-plan.md
1. apply-merged → fanout-test-plan--cascade-g1.md 14 — check 2: same line.
2. apply — check 5: plan's entry §6 Contract suite, the owed clause discharged, as landed.
3. apply-merged → fanout-test-plan--cascade-g4.md 2 — check 2: same line, its holders named by main's direction.
4. apply — check 5: plan's entry, the kill line by its `binary(…)` rule.

fanout-test-plan--cascade-g1.md
1. apply — check 1: a sub-item of a standing entity, report Symbols / APIs.
2. apply — check 6: oversize `answer` disproved.
3. apply — check 6: oversize `answer` disproved, the broad line.
4. apply — check 1: accurate this-chunk addition, report Outcome.
5. apply — check 1: accurate this-chunk addition, report Symbols / APIs.
6. apply — check 7: test-plan-newgroup-unit-test-strategy.md · distinct.
7. apply — check 7: test-plan-newgroup-unit-test-strategy.md · distinct.
8. apply — check 7: test-plan-newgroup-unit-test-strategy.md · distinct.
9. apply — check 1: the broad line of viola-channel.
10. apply — check 7: test-plan-newgroup-unit-test-strategy.md · distinct.
11. apply — check 7: test-plan-newgroup-unit-test-strategy.md · distinct.
12. apply — check 1: a sub-item of a standing entity, report Symbols / APIs.
13. apply — check 1: the broad list of viola-mcp.
14. apply — check 5: plan's entry §4 viola-mcp, exit 11 disproved.
15. apply — check 1: a sub-item of a standing entity, report Symbols / APIs.
16. apply — check 7: test-plan-newgroup-unit-test-strategy.md · distinct.
17. apply — check 7: test-plan-newgroup-unit-test-strategy.md · distinct.
18. apply — check 1: the broad line of the root bin.

fanout-test-plan--cascade-g2.md
1. apply — check 5: plan's entry §5, oversize `answer` disproved · as: on this `viola answer` path the fixed line is `UNUSABLE_RESPONSE` alone (`src/cmd/answer.rs:91`); `UNUSABLE_TEXT` is `viola send`'s.
2. apply — check 7: test-plan-newgroup-integration-test-strategy.md · distinct.
3. apply — check 7: test-plan-newgroup-integration-test-strategy.md · distinct.

fanout-test-plan--cascade-g3.md
1. apply — check 7: test-plan-newgroup-e2e-test-strategy.md · distinct.
2. apply — check 7: test-plan-newgroup-e2e-test-strategy.md · distinct.
3. apply — check 7: test-plan-newgroup-e2e-test-strategy.md · distinct.
4. apply — check 7: test-plan-newgroup-e2e-test-strategy.md · distinct.

fanout-test-plan--cascade-g4.md
1. apply — check 5: plan's entry §6, the helper replaced and not built, stated plainly.
2. apply — check 5: plan's entry §6, the helper replaced and not built · as: after the rules, name their holders: the no-`Cookie`, stripped-value and canary clauses are `viola-harness secret-scan`'s, after each job's end-to-end run (`v1-47`'s reading, the delegate overseer's word, PROVISIONAL), the 0600 clause `write_detail_refuses_a_widened_diagnostics_folder_and_leaves_it`'s (`v1-47`'s acceptance; `crates/viola-e2e/src/harness/secret_scan.rs:129` reads `cookie:` whatever its case).
3. reject — check 7: test-plan-newgroup-e2e-test-strategy.md · twin · fanout-test-plan--cascade-g3.md 2.
4. reject — check 7: test-plan-newgroup-e2e-test-strategy.md · twin · fanout-test-plan--cascade-g3.md 1.
5. apply — check 7: test-plan-newgroup-e2e-test-strategy.md · distinct.
6. reject — check 7: test-plan-newgroup-e2e-test-strategy.md · twin · fanout-test-plan--cascade-g3.md 4.
7. apply — check 7: test-plan-newgroup-e2e-test-strategy.md · distinct.
8. apply — check 7: test-plan-newgroup-e2e-test-strategy.md · distinct.
9. escalate → Q2 — check 7: test-plan-newgroup-e2e-test-strategy.md · contradiction · lines 259-259. · resolved: apply (Q2) · as: PROVISIONAL, owed the founder's own word, as its text reads
10. reject — check 7: test-plan-newgroup-e2e-test-strategy.md · twin · fanout-test-plan--cascade-g3.md 3.
11. apply — check 7: test-plan-newgroup-e2e-test-strategy.md · distinct.
12. apply — check 7: test-plan-newgroup-e2e-test-strategy.md · distinct.
13. apply-merged → fanout-test-plan.md 2 — check 2: same line, as landed.

fanout-test-plan--cascade-new-unit-test-strategy.md
1. apply — check 7: test-plan-newgroup-unit-test-strategy.md · distinct.

(sampler) cascade-compare-test-plan.md
S1. escalate → Q4 — check 7: test-plan-newgroup-test-data-fixtures.md · contradiction · lines 14-14. · resolved: apply (Q4) · as: the row reads «`Wrapper` takes no planted value» in place of «`Wrapper` takes no environment», so `test-data-fixtures.md:14` stays true
S2. apply — check 7: test-plan-newgroup-test-scope-summary.md · distinct.
S3. apply — check 7: test-plan-newgroup-test-scope-summary.md · distinct.
S4. apply — check 7: test-plan-newgroup-test-scope-summary.md · distinct.
S5. apply — check 7: test-plan-newgroup-test-strategy.md · distinct.

fanout-obs-plan.md
1. apply-merged → fanout-obs-plan--cascade-g3.md 2 — check 2: same line, current truth with no history.
2. apply-merged → fanout-obs-plan--cascade-g3.md 8 — check 2: same line.
3. apply-merged → fanout-obs-plan--cascade-g3.md 7 — check 2: same line.

fanout-obs-plan--cascade-g1.md
1. apply — check 1: verbatim copy kept current, report Schema / config.
2. apply — check 1: verbatim copy kept current, report Symbols / APIs.
3. apply — check 1: verbatim copy kept current, the scrub rule's field list.
4. apply — check 6: `mcp` writing a `process-exit` disproved.
5. apply — check 6: `mcp` writing a `process-exit` disproved.

fanout-obs-plan--cascade-g2.md
1. apply — check 7: obs-plan-newgroup-span-trace-coverage.md · distinct.
2. apply — check 1: report Schema / config, `detail` absent on every other line.
3. apply — check 1: report Schema / config, `detail` closed to three words.
4. apply — check 1: report Symbols / APIs.
5. apply — check 1: report Symbols / APIs.

fanout-obs-plan--cascade-g3.md
1. escalate → Q12 — check 7: obs-plan-newgroup-metric-coverage.md · contradiction · lines 28-28. · resolved: apply (Q12)
2. apply — check 5: plan's entry §5.
3. apply — check 7: obs-plan-newgroup-log-coverage.md · distinct.
4. escalate → Q11 — check 7: obs-plan-newgroup-error-capture-reporting.md · contradiction · lines 39-39. · resolved: apply (Q11)
5. apply — check 5: plan's entry §7, exit 11 and `process-exit` disproved.
6. escalate → Q11 — check 7: obs-plan-newgroup-error-capture-reporting.md · contradiction · lines 39-39. · resolved: apply (Q11)
7. apply — check 5: plan's entry §7, `mcp`'s catch site.
8. apply — check 5: plan's entry §7, the enum list.

fanout-obs-plan--cascade-g4.md
1. apply — check 7: obs-plan-newgroup-pii-scrubbing-compliance.md · distinct.
2. apply — check 7: obs-plan-newgroup-pii-scrubbing-compliance.md · distinct.
3. apply — check 7: obs-plan-newgroup-pii-scrubbing-compliance.md · distinct.
4. apply — check 7: obs-plan-newgroup-ci-integration.md · distinct.

fanout-obs-plan--cascade-new-error-capture-reporting.md
1. apply — check 7: obs-plan-newgroup-error-capture-reporting.md · distinct.
2. apply — check 7: obs-plan-newgroup-error-capture-reporting.md · distinct.
3. apply — check 7: obs-plan-newgroup-error-capture-reporting.md · distinct.
4. apply — check 7: obs-plan-newgroup-error-capture-reporting.md · distinct.
5. apply — check 7: obs-plan-newgroup-error-capture-reporting.md · distinct.

(sampler) cascade-compare-obs-plan.md
S1. escalate → Q13 — check 7: obs-plan-newgroup-obs-scope-summary.md · contradiction · lines 499-499. · resolved: apply (Q13)
S2. apply — check 7: obs-plan-newgroup-obs-scope-summary.md · distinct.
S3. apply — check 7: obs-plan-newgroup-obs-scope-summary.md · distinct.
S4. escalate → Q3 — check 7: obs-plan-newgroup-pii-scrubbing-compliance.md · contradiction · lines 22-22. · resolved: reject (Q3)
S5. apply — check 7: obs-plan-newgroup-pii-scrubbing-compliance.md · distinct.
S6. apply — check 7: obs-plan-newgroup-span-trace-coverage.md · distinct.

fanout-a11y-plan--cascade.md
1. apply — check 5: plan's entry §8 Error recovery, `code-word`.
2. apply — check 6: oversize `answer` disproved, `--json`'s exception.
3. apply — check 5: plan's entry §8 Error recovery, `a11y-plan.md:257`.
4. apply — check 6: oversize `answer` disproved, the broad line.
5. apply — check 5: plan's entry, `a11y-plan.md:345`.
6. apply — check 7: a11y-plan-newgroup-cognitive-accessibility.md · distinct.
7. apply — check 7: a11y-plan-newgroup-cognitive-accessibility.md · distinct.
8. apply — check 7: a11y-plan-newgroup-cognitive-accessibility.md · distinct.
9. escalate → Q7 — check 7: a11y-plan-newgroup-cognitive-accessibility.md · contradiction · lines 19-19. · resolved: apply (Q7)
10. apply — check 7: a11y-plan-newgroup-cognitive-accessibility.md · distinct.
11. apply — check 7: a11y-plan-newgroup-cognitive-accessibility.md · distinct.
12. apply — check 7: a11y-plan-newgroup-cognitive-accessibility.md · distinct.
13. apply — check 1: report Coverage of new surfaces, a sub-item of a standing entity.
14. apply — check 6: report Outcome and the exit-2 paths, a sub-item of a standing entity.
15. apply — check 1: report Outcome, the rule gains a witness.

(sampler) cascade-compare-a11y-plan.md
S1. reject — check 7: a11y-plan-newgroup-cognitive-accessibility.md · twin · fanout-a11y-plan--cascade.md 6.
S2. reject — check 7: a11y-plan-newgroup-cognitive-accessibility.md · twin · fanout-a11y-plan--cascade.md 10.

## Validate, the rest

- Rejected before the checks as re-derivation: 1 (`fanout-layout-templates.md` 2). Every other basis outside `.andromeda/` was checked against the report's bullets and its New text, by line: 81 bases read, 0 outside.
- Check 5, the plan's list: every entry has a proposal; the ledger's and the route's are P7's and P5's.
- Check 6, disproved claims: every site has a proposal; the leaves (`viola-mcp.md:23`, `conventions.md:15`, `stack.md:23`, `services/viola-core.md:14`, `:25`, `.claude/rules/observability.md:36`) go to the leaf batch.
- `obs-scope-summary#20.5` and `slo-invariants-telemetry-budgets#2.2` label `max` the five readings the report and implement's outcome label `mean`: no value moved, left unedited and named in the Phase 2 summary.
- `security-plan`'s «shared validators live in `viola-core`» beside `sanitised_fault` in `viola-channel`: a fault reader, no input validator; no proposal.
- `a11y-plan.md:270`, `:346`: the code tied to `unable` is the human form; `refused: <word>` is MCP's text; no proposal.

Q1 to Q13 answered by the overseer as the founder's delegate, `inputs#I5`: PROVISIONAL.
