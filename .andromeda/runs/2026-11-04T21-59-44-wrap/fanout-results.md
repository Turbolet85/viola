# Fan-out results — 2026-11-04-linux-and-macos-parity

Sampler: off (`cascade-sample.txt` reads `sample: off`, not forced — `inputs#I5` item 1). Batches: stage1 13/13 ·
classify 138/140, re-spawned `architecture-cls-stack-and-technologies.b1.d2` and `test-plan-cls-test-scope-summary.b1.d2`
([EVERY_ENTITY]), classify-r1 2/2 (no dead draw) · strong 8/8 · newdetail 3/3. `[EVERY_FACT]`: PASS on all six extracted docs.

- architecture: 3 proposals · fanout-architecture.md · 3 cascade proposals · fanout-architecture--cascade.md · new-detail fanout-architecture--cascade-new-cross-cutting-patterns.md (1) · fanout-architecture--cascade-new-established-decisions.md (1) · 4 facts · 4 placed · 0 no home
- security-plan: 0 proposals · fanout-security-plan.md · 1 cascade proposal · fanout-security-plan--cascade.md · 0 facts · 0 placed · 0 no home
- design-system: 0 proposals · fanout-design-system.md · 1 cascade proposal · fanout-design-system--cascade.md · 1 fact · 1 placed · 0 no home
- layout-templates: 0 proposals · fanout-layout-templates.md (whole)
- test-plan: 1 proposal · fanout-test-plan.md · 16 cascade proposals · fanout-test-plan--cascade-g1.md (4) · fanout-test-plan--cascade-g2.md (2) · fanout-test-plan--cascade-g3.md (10) · 9 facts · 9 placed · 0 no home
- obs-plan: 0 proposals · fanout-obs-plan.md · 2 cascade proposals · fanout-obs-plan--cascade.md · new-detail fanout-obs-plan--cascade-new-ci-integration.md (0) · 3 facts · 2 placed · 1 no home
- a11y-plan: 0 proposals · fanout-a11y-plan.md · 5 cascade proposals · fanout-a11y-plan--cascade.md · 2 facts · 2 placed · 0 no home

Remarks carried (the files are the list):
- fanout-architecture.md: D-arch-decisions no drift (no dependency; `scripts/junit-os-sets.py` python 3 stdlib, operator-only); D-retired 0 hits; the kill line's count has no architecture site.
- fanout-test-plan.md: the cascade owns the detail drift (fct-lzdm6e7w, fct-63urz52n, fct-5qxwlgko, fct-4m5t4osf); the key file 5-command-implementation fct-hgewdmj2 states `Instant::now() + WITHIN` «28 sites in 20 files», predating this chunk (the gate reads 32).
- fanout-a11y-plan.md: no detector fires; body fct-lchghj7u's Owed line stale — left to the cascade.
- fanout-design-system.md: test-only; the witness's sequence agrees with fct-dufqiv2l / fct-erfdav33.
- fanout-architecture--cascade.md: F2, F4 left to the new-detail passes.
- fanout-obs-plan--cascade.md: F2 (the kill line, 27) has no obs home — test-plan §9's.
- fanout-security-plan--cascade.md: no external input, auth path or dependency added.

## Dispositions

fanout-architecture.md
1. apply — check 1: the plan's expected amendment (architecture §Infrastructure Patterns, `scripts/junit-os-sets.py` beside the repository's scripts) names the change itself; accurate this-chunk addition.
2. apply — check 1: accurate this-chunk addition; the `tests/support/` row already names its modules (sgr.rs), so the new module joins it.
3. apply — check 1: accurate this-chunk addition (the census backs the Cross-platform discipline line); its detail is fanout-architecture--cascade-new-cross-cutting-patterns.md 1 · as: one clause on the body line pointing at the census binary, the rule's detail left to the category group.

fanout-architecture--cascade.md
1. apply — check 1: a count with its rule (`Instant::now() + WITHIN` under `tests/`); the report's gate reads 32; every site of the count amended in one apply with fanout-test-plan.md 2 · as: «reads 32 sites in 24 files under `tests/` (`grep -rn 'Instant::now() + WITHIN' tests/ | wc -l` and `grep -rln … | wc -l`), as measured at chunk 2026-11-04-linux-and-macos-parity's wrap on the Linux dev host», then the Frozen clause as proposed.
2. apply — check 7: architecture-newgroup-occupied-resources.md · distinct.
3. apply — check 7: architecture-newgroup-occupied-resources.md · distinct.

fanout-architecture--cascade-new-cross-cutting-patterns.md
1. apply — check 7: architecture-newgroup-cross-cutting-patterns.md · distinct.

fanout-architecture--cascade-new-established-decisions.md
1. escalate → Q1 — check 1: no rule, and strange: a new `dec-ci-cd` detail group under Established Decisions records a test reader's gap (macOS `ps` has no `sigignore` keyword) and one test case's gate — a test-tier fact the test-plan §6 amendment (fanout-test-plan--cascade-g3.md 4) already carries; check 7: architecture-newgroup-established-decisions.md · distinct · resolved: reject (Q1) — the overseer's technical answer, founder-delegated (`inputs#I6`): the fact lands once, in test-plan §6; Established Decisions holds product decisions.

fanout-security-plan--cascade.md
1. apply — check 1: accurate this-chunk addition (a stack reference: the census's `NO_PEER_PID` reason, 43 of the 48 ubuntu-not-macOS cases); the carve-out itself unchanged.

fanout-design-system--cascade.md
1. apply — check 7: design-system-newgroup-surface.md · distinct.

fanout-test-plan.md
1. apply — check 1: a count with its rule (the `binary(…)` names on the filter line); every site amended in one apply with fanout-test-plan--cascade-g3.md 8. The report's «No master states the count» is false: this key file states it in words («twenty-six»), which the digit-only `kill27` pattern did not match — carried to curation.
2. apply — check 5 (raised by main; the companion of fanout-architecture--cascade.md 1): §3 → key `5-command-implementation`, fct-hgewdmj2 · change: replace «`Instant::now() + WITHIN` reads 28 sites in 20 files under `tests/`, as measured at chunk 2026-10-10-budget-governor's wrap …» with the same measured reading · as: «reads 32 sites in 24 files under `tests/` (`grep -rn 'Instant::now() + WITHIN' tests/ | wc -l` and `grep -rln … | wc -l`), as measured at chunk 2026-11-04-linux-and-macos-parity's wrap on the Linux dev host», the rest of the sentence kept.

fanout-test-plan--cascade-g1.md
1. apply — check 1: accurate this-chunk addition; check 5: the expected amendment's witness.
2. apply — check 1: accurate this-chunk addition; check 5: test-plan §2, `tests/support/frozen.rs`.
3. apply — check 7: test-plan-newgroup-test-strategy.md · distinct.
4. apply — check 5: test-plan §2, the census (the expected amendment names it).

fanout-test-plan--cascade-g2.md
1. apply — check 5: test-plan §6 / §1, the force-kill case on both Unix runners (the expected amendment names it).
2. apply — check 1: D-retired, the item the report's Retired bullet names (the local `Frozen` twins).

fanout-test-plan--cascade-g3.md
1. apply — check 5: test-plan §6, the `v1-40` witness landed, the CARRY sentence discharged.
2. apply — check 7: test-plan-newgroup-e2e-test-strategy.md · contradiction · fct-4m5t4osf · rewritten by fanout-test-plan--cascade-g3.md 1.
3. apply — check 7: test-plan-newgroup-e2e-test-strategy.md · distinct; check 5: `run_child_ends_when_its_wrapper_is_terminated` on every OS.
4. apply — check 7: test-plan-newgroup-e2e-test-strategy.md · distinct; check 5 and check 6: Linux only (`inputs#I5` item 5), the plan's «on both Unix OSes through `ps`» disproved by measurement · as: the gate's authority written «the overseer's technical answer (founder-delegated) to the implement dialog» (`inputs#I6`: not a founder ruling), never «the operator's ruling» or «the founder's».
5. apply — check 7: test-plan-newgroup-e2e-test-strategy.md · distinct; check 5: the census.
6. apply — check 7: test-plan-newgroup-e2e-test-strategy.md · distinct; check 6: the plan's predicted 50 disposed (`inputs#I5` item 5: 52 functions and 7 files).
7. apply — check 1: D-retired, the item the report's Retired bullet names.
8. apply — check 7: test-plan-newgroup-ci-integration.md · distinct; check 5: test-plan §9, the kill line at 27; with fanout-test-plan.md 1.
9. apply — check 7: test-plan-newgroup-ci-integration.md · distinct.
10. apply — check 7: test-plan-newgroup-ci-integration.md · distinct; check 6: research's 44 + 4 disposed (`inputs#I5` item 5: 43 + 5).

fanout-obs-plan--cascade.md
1. apply — check 7: obs-plan-newgroup-ci-integration.md · distinct.
2. apply — check 7: obs-plan-newgroup-ci-integration.md · distinct.

fanout-a11y-plan--cascade.md
1. apply — check 5: a11y-plan §4, P6's outer-PTY witness landed on the Unix legs.
2. apply — check 7: a11y-plan-newgroup-aria-patterns-roles.md · contradiction · fct-uok334yy · rewritten by fanout-a11y-plan--cascade.md 1.
3. apply — check 5: a11y-plan §6's companion of the same witness.
4. apply — check 7: a11y-plan-newgroup-visual-design-verification.md · contradiction · fct-es7le4as · rewritten by fanout-a11y-plan--cascade.md 3.
5. apply — check 5: a11y-plan §4 body, the Owed line discharged.

Check 5 (expected amendments): every entry matched — test-plan §6 (g3 1, 3, 4), §6 / §1 (g2 1), §2 / §9 (g1 2, 3, 4; g3 5, 8; fanout-test-plan.md 1), a11y-plan §4 (a11y 1, 2, 5), architecture §Infrastructure Patterns (architecture 1); matrix#v1-40 notes — ledger-note, P7.3.
Check 6 (spec claims disproved): the plan's step 5 macOS reader → g3 4; research's 44 + 4 → g3 10; the plan's predicted 50 → g3 6. All plan- or research-side; no master states them.
Citation dispositions: 0 rows (`citation-dispositions.md`).

## Apply rulings
- apply-architecture.md row 6 (fanout-architecture.md 1) `v=refused` — the row's place is a new line inside the code-block fact fct-irhgihut, which an in-line change cannot carry · ruled: reject, folded into fanout-architecture--cascade.md 3 (E6: `scripts/junit-os-sets.py` registered under occupied-resources), which meets the plan's expected amendment («beside the repository's scripts») in substance; the tree listing of `scripts/` is not a census.
- apply-test-plan.md row 1 (fanout-test-plan.md 2) `v=refused` — the key file fct-hgewdmj2 was outside the writer's read addresses; «24 files» is main's own measurement (`grep -rln 'Instant::now() + WITHIN' tests/ | wc -l` = 24, `grep -rn … | wc -l` = 32, this wrap) given as the `as:` direction · ruled: main's own edit, `edits-main.md`, landed tx 138.
- Landed: apply-architecture tx 132 (7) · apply-security-plan tx 133 (1) · apply-design-system tx 134 (1) · apply-test-plan tx 135 (17) · apply-obs-plan tx 136 (2) · apply-a11y-plan tx 137 (6) · edits-main tx 138 (1).
