
## 2026-09-24-epoch-1-cleanup — mutation leg streams its progress; an unviable swamp is red
**Section:** §2 (Mutation row) · §3 `run` step 4 (Base, Command, Verdict), Exit code semantics, `gate` mutants breach, Bootstrap `quality-gate-config-emit` · §9 (Mutation row, Build failure conditions) · §10 (Mutation gate, Build failure conditions) · §11 (Quality) · §12 (new entry)
**Change:**
- The `run --mutants` invocation gains `--caught --unviable --build-timeout-multiplier=5`. cargo-mutants' stdout streams live to the harness's stderr, so every outcome line reaches the CI step log as it happens.
- The counted verdict requires `unviable <= caught`. A run with more unviable than caught mutants is red with `failures[]` code `unviable-exceeds-caught` and `failed` = survivors + 1. Under `--leg` it is not deferred to the union.
- §10 retires "`unviable` mutants are reported but do not fail the gate": a few do not, and outnumbering the caught ones is red. The measured threshold rows are in the body.
- The `gate` union parenthetical is scoped to the union. The §2/§9/§10 failure lists, `quality-gate-config-emit` and the §11 counting ban name the new condition.
- §3 Base adds the force-push case: a replaced `github.event.before` is reachable from no ref, so the run is `base-missing` (run 36117447745). The remedy is a rewind to the chunk base, then a fast-forward.
- §12 records the decision, including the operator's DECLINE of a reduced partial-verdict upload for cancelled legs (a decision, not a deferral).
**Why:**
- The chunk's report, Changes → Harness/gate surface and Spec claims disproved 1–3.
- CI run 36118112104 windows read `8 caught, 135 unviable` as `ok:true`. A leaked supervisor locked `viola-harness.exe` (relink `os error 5`, reproduced locally).
- CI run 36046091888 windows was silent for 2 h 45 m while the output was captured.
- §10 sits under Founder Direction 1. This tightening was decided by the founder-delegated overseer on 2026-09-25 ("YES to making the leg red when unviable outcomes swamp the verdict … The test-plan §10 wording goes to the wrap reconcile as an amendment"), recorded here as the ratification.
- Green witness: run 36126924953 on d14f234.
**Sweep** (cascade step 2, `sweep.txt` in this wrap's run dir). Pattern `unviable|missed or timed-out|missed == 0|zero missed|only when no leg caught|red only when|missed.{0,20}timeout.{0,40}(fail|red)|copy-target|count \`missed\`` over the seven masters, `playbook.md`, `drift-base.md`, CLAUDE.md, `.claude/rules/*` and `.claude/docs/**`: 25 hits.
- test-plan: 19 hits.
  - 11 amended (:437, :555, :557, :563, :655, :796, :1434, :1468, :1496, :1529, :1609). They were the known-positive control, and the pattern found them.
  - 6 are this pass's new §12 text.
  - :566 (leg-verdict `outcome` shape) and :663 (closed enums): no change, since no new value.
  - :1798: no change. It is chunk 2026-09-24-quality-gates' historical §12 entry.
- architecture.md:515 and obs-plan.md:1253: cross-master restatements of the union as the only red path, amended in this pass (their own sidecars).
- Leaves re-derived: `.claude/docs/commands.md:31/:41`, `.claude/docs/tests-summary.md:42`, `.claude/rules/testing.md:48` (above its Session Additions).
- 0 hits in CLAUDE.md, playbook.md, drift-base.md and the curation homes.
- Fanned 12 proposals (11 `dependent-of`), all applied with their text re-derived from the report. 1 orchestrator-raised (the §3 Base force-push case, from the report's Cross-project CI facts).
