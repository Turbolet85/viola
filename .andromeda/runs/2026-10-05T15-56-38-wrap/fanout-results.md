# Fan-out results — 2026-10-05-permission-end-to-end

Seven doc-agents, one batch. The prompts carried 15 detectors: arch 2 · security 3 · design 1 · layout 1 · tests 3 ·
obs 3 · a11y 2, which matches the `doc:` names in drift-base.md. Returns were plain YAML, with no HTML entities to
decode and no stripping of substance (only `#` comment notes), so no raw twins were saved.

## Verdicts
- architecture — 2 proposals (below)
- security-plan — `proposals: []` (notes: D-security-input / auth / deps hold; the E1 sweep finds nothing in security-plan)
- design-system — `proposals: []` (notes: `tokens n/a` on both coverage rows)
- layout-templates — `proposals: []` (notes: no new surface; the wait / answer lines are already documented at :427-466)
- test-plan — 4 proposals (below)
- obs-plan — `proposals: []`
- a11y-plan — `proposals: []` (notes: no UI element; no schema change)

## architecture
A1.
- detector: D-arch-decisions
- section: §Established Decisions → [CLI Version Compatibility]
- change: retire "The `permission` kind's end to end and a `question` first raised by PermissionRequest are owed to the
  'Permission end to end' route entry". The permission kind is now end to end, and the question first raised by
  PermissionRequest stays `null` for the measured reason (a static read, no ledger row, the 15th-row cost).
- basis: architecture.md:91
- **disposition: apply.**
  - Check 1: playbook "Accurate this-chunk addition" is routine, and the plan's Expected amendment E1 names this change.
  - Check 5: E1.
  - Check 6: disposes report Spec claims disproved 3.

A2.
- detector: D-arch-decisions
- section: §Standard Contracts (the `hook.dialog` receipt paragraph)
- dependent-of: D-arch-decisions
- change: the list of `null`-at-once cases gains the question first raised by PermissionRequest.
- basis: architecture.md:308
- **disposition: apply** (routine, "Accurate this-chunk addition"; E1's second site). The applied text is re-derived:
  the question is logged once and answered `null` at once, with no "armed PreToolUse" qualifier beyond what the report
  states.

## test-plan
T1.
- detector: D-tests-coverage
- section: §6 Path 4 → Surfaces involved
- change: retire "The `permission` kind is unit / insta only … owed to 'Permission end to end'". The permission kind is
  end to end in `tests/cli_answer.rs` over `fixtures/fake-scripts/path4-permission.json`, in the two named cases.
- basis: test-plan.md:779
- **disposition: apply** (routine; E2; disposes Spec claims disproved 4). Re-derived from the report: the replayed
  fixtures are under `fixtures/claude/2.1.287/`. The MCP / `/api/sessions` / Playwright clause is kept.

T2.
- detector: D-tests-coverage
- section: §6 Path 3 → Surfaces involved
- dependent-of: D-tests-coverage
- change: retire "the `permission` wake stays unit-level … owed to". The permission end-to-end wake witness landed in
  `path4_permission_…`.
- basis: test-plan.md:757
- **disposition: apply** (routine; E3; disposes Spec claims disproved 4).

T3.
- detector: D-tests-coverage
- section: §6 Path 4 → Steps, step 1
- dependent-of: D-tests-coverage
- change: retire "(the step owed to 'Permission end to end')". As landed, the permission dialog runs as its own gated
  script `path4-permission.json`.
- basis: test-plan.md:781
- **disposition: apply** (routine; E2's third site, named in the report's E2 line). Re-derived: the target
  single-script text names the prompt-2 step. The applied text says it landed as a separate gated script and lists its
  five steps; it does not re-describe `boot_over`.

T4.
- detector: D-tests-coverage
- section: §7 Test Data & Fixtures (the fake-script hygiene walk)
- change: `{gated-turn,path3,path4}` → `{gated-turn,path3,path4,path4-permission}`.
- basis: test-plan.md:1084
- **disposition: apply** (routine, "Accurate this-chunk addition"; the report's Schema / config lists the script in the
  hygiene walk).

## Validate summary
- Check 1 (playbook): 6 routine; 0 escalate; no rule collision.
- Check 2 (cross-contradiction): none. A1 and A2 are different sections; T1–T4 are different sites.
- Check 3 (intent): consistent with the plan's acceptance; the scope record is empty (`gate.py scope` clean).
- Check 4 (absence): no absence claim proposed. The orchestrator's E1 sweep is in cascade-dispositions.md.
- Check 5 (expected amendments):
  - E1 → A1 + A2;
  - E2 → T1 + T3;
  - E3 → T2;
  - E4 (`matrix#v1-30 notes`) → ledger note at P7.3.
- Check 6 (disproved claims):
  - 1 and 2 (plan entry-12 atom, F1 command) → disposed as a dated plan correction on the operator's word, recorded in
    the report, the plan being a chunk artifact; no master quotes either (`grep -c 'TIMEOUT \['` 0 per master);
  - 3 → A1;
  - 4 → T1 + T2.
- Escalations: 0.
