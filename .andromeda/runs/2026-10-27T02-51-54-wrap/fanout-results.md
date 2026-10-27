# Fan-out results — 2026-10-27-pinned-copy-race-and-reproducible-builds

Wrap run dir `.andromeda/runs/2026-10-27T02-51-54-wrap/`; report `viola-0.1.0/chunks/2026-10-27-pinned-copy-race-and-reproducible-builds/report.md`. Sample: `sample: no · wrap · draw 1 of 10` (`cascade-sample.txt`). Batches: stage1 (13 agents; the test-plan task writer re-spawned once for `[UNTOUCHED_REACHED]`, stage1-r1 passed) · classify (66, `yes 34 of 864`) · strong (4) · newdetail (1, architecture occupied-resources F4) · newgroup (11) — every check `PASS · index and head unchanged`.

## Verdict lines
- architecture: 3 proposals · fanout-architecture.md · 11 cascade proposals · fanout-architecture--cascade.md · 1 new-detail proposal · fanout-architecture--cascade-new-occupied-resources.md · 7 facts · 7 placed (by pass 1) · 0 no home
- security-plan: 0 proposals · fanout-security-plan.md · 10 cascade proposals · fanout-security-plan--cascade.md · 6 facts · 6 placed · 0 no home
- design-system: 0 proposals · fanout-design-system.md · 0 cascade proposals (pick yes 0) · 0 facts
- layout-templates: 0 proposals · fanout-layout-templates.md (whole, no cascade)
- test-plan: 3 proposals · fanout-test-plan.md · 13 cascade proposals · fanout-test-plan--cascade.md · 7 facts · 7 placed · 0 no home
- obs-plan: 0 proposals · fanout-obs-plan.md · 2 cascade proposals · fanout-obs-plan--cascade.md · 1 fact · 1 placed · 0 no home
- a11y-plan: 0 proposals · fanout-a11y-plan.md · 0 cascade proposals (pick yes 0) · 0 facts

Notes carried from the files (no fix was made; no twin): the security, design, layouts, obs and a11y doc agents found no body drift (their `note:` lines); the architecture strong reader's note names `architecture.md:319` (no picked entity — the doc agent's proposal 1 carries it) and `architecture.md:445` (Inherited Defaults, «tempfile 3.27.0 `persist` through one shared helper» — main read it: it speaks of the Database line's snapshots and logs, which still go through `replace_private`; no change); the test-plan strong reader's note names the key file `5-command-implementation.md:26` (the doc agent's proposal 1 carries it) and the `hook` receipt at `test-data-fixtures.md:59`, not `:42` (the report's `:42` is the non-absolute arm's line; `:59` is the `hook` kind's — proposal 12 targets `:59`).

## Dispositions

Check 7 note (main's ruling, stated once): five new-group verdicts read `contradiction`, and each names as its standing text the very stale line a companion proposal of this pass amends to current truth (ED:53 by architecture cascade 1; DP:17 by security cascade 2; OR:137 by architecture cascade 9; UTS:26 by test-plan cascade 2). The contradiction is the stale claim itself (report: Expected amendments; Spec claims disproved by measurement), so it resolves in this pass, and no standing truth contradicts the new group. Each such line says so.

fanout-architecture.md
1. apply — check 1: the plan's P5-approved Expected amendment (architecture §Occupied Resources → Filesystem) names the change; report Symbols / APIs; site `architecture.md:319`.
2. apply — check 1: the plan's Expected amendment ([Snapshot writer]) names the change; the body broad line `architecture.md:37` names the no-clobber placement beside `replace_private`.
3. apply — check 1: the plan's Expected amendment ([Deployment / Distribution]) names measurement B's reading; check 6 disposes the report's measurement-B disproved claim (`architecture.md:69`).

fanout-architecture--cascade.md
1. apply — check 1: the plan's Expected amendment ([Snapshot writer]): `established-decisions.md:53` lists the pinned copy among `replace_private_shared`'s files; the head and the list to current truth.
2. apply-merged → fanout-architecture.md 2 — the same body line `architecture.md:37`.
3. apply — check 1: Accurate this-chunk addition (report Symbols / APIs, Dependencies: `into_temp_path` + `persist_noclobber`, tempfile 3.27.0 unchanged); `stack-and-technologies.md:18`.
4. apply — check 7: architecture-newgroup-established-decisions.md · contradiction (ED:53) — resolved in-pass by fanout-architecture--cascade.md 1.
5. apply — check 7: architecture-newgroup-established-decisions.md · distinct.
6. apply — check 7: architecture-newgroup-established-decisions.md · contradiction (ED:53) — resolved in-pass by fanout-architecture--cascade.md 1; the rationale carries `as measured at` the chunk's evidence.
7. apply — check 7: architecture-newgroup-established-decisions.md · distinct; check 6: measurement B disposed here (the decision's detail, ED:249).
8. apply-merged → fanout-architecture.md 3 — the same body line `architecture.md:69`.
9. apply — check 6: the report's disproved Users-SID claim, site `occupied-resources.md:137` marked changed.
10. apply — check 7: architecture-newgroup-occupied-resources.md · contradiction (OR:137) — resolved in-pass by fanout-architecture--cascade.md 9; it sits in `res-repository` beside the LA-alias oracle reading of the same kind (`occupied-resources.md:252`).
11. reject — check 1: Registry over-reach (playbook) — a receipt field inside an already-registered test-only binary; the anchor `occupied-resources.md:23` is the `viola-mcp-tap` fixture line; the field's home is test-plan §7 (fanout-test-plan--cascade.md 12).

fanout-architecture--cascade-new-occupied-resources.md
1. apply — check 7: architecture-newgroup-occupied-resources.md · distinct; the detail record beside the body line fanout-architecture.md 1 amends.

fanout-security-plan--cascade.md
1. apply — check 6: the report's disproved Users-SID claim, site `data-protection.md:12` marked changed.
2. apply — check 5: the plan's Expected amendment (security-plan §Data Protection) closes the owed line `data-protection.md:17`.
3. apply — check 7: security-plan-newgroup-data-protection.md · distinct.
4. apply — check 7: security-plan-newgroup-data-protection.md · distinct.
5. apply — check 7: security-plan-newgroup-data-protection.md · contradiction (DP:17) — resolved in-pass by fanout-security-plan--cascade.md 2.
6. apply — check 7: security-plan-newgroup-data-protection.md · contradiction (DP:17) — resolved in-pass by fanout-security-plan--cascade.md 2 · as: one sentence — clean release builds of one tree can give two files with equal loaded sections, so one tree can key two `bin/<key>/` and `plugin/<key>/` folders; the reading stands at architecture §Established Decisions [Deployment / Distribution], no cause claimed, acting on it the founder's.
7. apply — check 1: Accurate this-chunk addition (report Symbols / APIs: the pinned `viola`'s write moved from `replace_private_shared` to `place_private_new`, mode before any byte); `authentication-authorization.md:33`.
8. apply — check 1: Accurate this-chunk addition (report Dependencies); `authentication-authorization.md:42`.
9. apply — check 7: security-plan-newgroup-authentication-authorization.md · distinct · as: add only «(read by the inherited write right whichever SID holds it, with a viola-made-home control)» after the case; the runner-ACL narrative is test-plan's (fanout-test-plan--cascade.md 8).
10. apply — check 7: security-plan-newgroup-threat-model-summary.md · distinct.

fanout-test-plan.md
1. apply — check 1: a `D-retired` proposal whose item the report's Retired bullet names (playbook); the key file `5-command-implementation.md:26`.
2. apply — check 6: the report's disproved Users-SID claim, site `test-plan.md:634` marked changed.
3. apply — check 1: Accurate this-chunk addition (report Coverage of new surfaces: unit tests `fs::tests::place_private_new_*`, `pin::tests::place_absent_*`); the viola-state broad line `test-plan.md:382`.

fanout-test-plan--cascade.md
1. apply — check 6: the disproved Users-SID claim at a site the report's `plain-home` pattern did not reach (`test-scope-summary.md:117`, «its inherited Users write ACE», no SID token).
2. apply — check 6: the disproved Users-SID claim, site `unit-test-strategy.md:26` marked changed.
3. apply — check 7: test-plan-newgroup-unit-test-strategy.md · contradiction (UTS:26) — resolved in-pass by fanout-test-plan--cascade.md 2.
4. apply — check 7: test-plan-newgroup-unit-test-strategy.md · distinct.
5. apply-merged → fanout-test-plan.md 3 — the same body line `test-plan.md:382`.
6. apply — check 5: the plan's Expected amendment (test-plan §5, the forced-window placement witness); check 7: test-plan-newgroup-integration-test-strategy.md · distinct.
7. apply — check 6: the disproved Users-SID claim, site `integration-test-strategy.md:34` marked changed.
8. apply — check 7: test-plan-newgroup-integration-test-strategy.md · distinct.
9. apply — check 5: the plan's Expected amendment (test-plan §6 Path 7); check 7: test-plan-newgroup-e2e-test-strategy.md · distinct.
10. apply — check 6: the disproved Users-SID claim, site `test-data-fixtures.md:10` marked changed.
11. apply-merged → fanout-test-plan.md 2 — the same body line `test-plan.md:634`.
12. apply — check 5: the plan's Expected amendment (test-plan §7 fake agent, `spawn_os_error`); check 7: test-plan-newgroup-test-data-fixtures.md · distinct.
13. apply — check 7: test-plan-newgroup-mocking-stubbing-discipline.md · distinct.

fanout-obs-plan--cascade.md
1. apply — check 7: obs-plan-newgroup-span-trace-coverage.md · distinct.
2. apply — check 1: Verbatim upstream copy kept current (playbook, the founder's 2026-10-04 ruling) + Accurate this-chunk addition; `obs-scope-summary.md:398`.

## The checks over the whole set
- Check 0 (re-derivation tell): 0 proposals rejected for a source the report does not carry.
- Check 2 (cross-contradiction): none — the overlapping pairs edit one line in one direction and are merged (architecture 2/cascade 2, architecture 3/cascade 8, test-plan 2/cascade 11, test-plan 3/cascade 5).
- Check 3 (intent-consistency): the report holds to the working-route entry and the plan's acceptance criteria; scope record none (`gate.py scope` clean). Step 10 under S2 is the plan's own branch.
- Check 4 (absence needs evidence): no proposal claims an absence; the report's sweep counts carry their patterns and rows.
- Check 5 (expected amendments): every entry has a proposal — [Snapshot writer] (architecture 2, cascade 1, 4), Occupied Resources → Filesystem (architecture 1, new-detail 1), Data Protection owed line (security 2), test-plan §7 (test-plan cascade 12), §6 Path 7 (test-plan 1, cascade 9), §5 (test-plan cascade 6), [Deployment / Distribution] (architecture 3, cascade 7); §6 Path 1 is not carried (step 10 removed nothing, the report says so). `citation-dispositions.md`: 0 rows, no `claim false`.
- Check 6 (disproved claims): the Users-SID claim — 6 report sites plus `test-scope-summary.md:117`, each amended (architecture cascade 9 · security cascade 1 · test-plan 2 · test-plan cascade 1, 2, 7, 10); measurement B against the route CARRY's «repeatable for a given source text» — the masters by architecture 3 and cascade 7, the CARRY itself P5's (a founder item on «Version done-check», `inputs#I6` item 4) and the handoff's FOR DISCUSSION list; the CARRY's «the kernel error is not read» — read, confirming the hypothesis, no master site; P5 (the frozen line leaves the route at P7's compaction).
- Check 7: 11 new-group readers, every verdict file read whole; 0 duplicates, 0 twins, 5 contradictions each resolved in-pass (above).

Escalations: 0. Applied: 32 (4 of them apply-merged). Rejected: 1 (architecture cascade 11).
