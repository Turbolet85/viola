# Fan-out results — 2026-09-24-fake-agent-and-test-data-fixtures (wrap 2026-09-24T08-45-42)

| doc | verdict | proposals | raw twin |
|---|---|---|---|
| architecture | drift | 6 (warning) | `.raw-fanout-architecture.md` |
| security-plan | no drift | 0. Note: no `cargo deny check` is shown in the report; `deny.toml` is owned by "Supply-chain and workflow gates" (markerless, Epoch 1), and the C-crate probe = 0 stands in | — |
| design-system | no drift | 0 | — |
| layout-templates | no drift | 0 | — |
| test-plan | drift | 14 (11 warning, 3 escalate) | `.raw-fanout-test-plan.md` |
| obs-plan | no drift | 0. Note: obs-plan.md:967 describes `stamped_home` as the future stamps writer; that is target state, and the interim seam is a sequencing deferral | — |
| a11y-plan | no drift | 0 | — |

## Dispositions

**architecture**
- A1 · Occupied Resources › Environment variables: register `AGENT_RUN_KEEP_FAILED`; `AGENT_RUN_KEEP_HOMES` is also read by the root test chain. **routine: apply** (Accurate this-chunk addition; disproved claim 2).
- A2 · Occupied Resources › Repository: split e2e-home into `viola-session-*` (harness) and `viola-test-*` (root tests). **routine: apply** (disproved claim 1).
- A3 · Occupied Resources › Repository: add `fixtures/fake-scripts/`, `schemas/fake-script.v1.json`, `crates/viola-core/proptest-regressions/`. **routine: apply**. The proposed `tests/support/` line is **rejected** (Registry over-reach: a test module inside the already-listed `tests/`; the Repository registry enumerates artifact paths).
- A4 · Infrastructure Patterns directory tree: add `fixtures/fake-scripts/`, `schemas/`, `tests/support/` annotation, viola-core `proptest-regressions/`. **routine: apply** (dependent of A3; the tree annotates modules, as `src/cmd/` does).
- A5 · Occupied Resources › Filesystem: test-only `<home>/fake/<name>.control` / `.receipt.ndjson`. **routine: apply** (plan expected amendment).
- A6 · Stack and Technologies: a Testing row for rstest/proptest/jsonschema. **rejected**. The invariant holds: architecture.md:106 [Deferred] hands "the test framework, fake-agent harness …" to tests, and test-plan §3 test-runner-install pins the same three versions. Precedent: tempfile =3.27.0 (chunk 1) is likewise not in §Stack.

**test-plan**
- T1 §3 `run` step 4 Verdict · T2 §10 Mutation gate · T3 §3 `run` Output format · T4 §3 Closed enums · T5 §12 entry: **routine: apply** (Accurate this-chunk addition; disproved claim 3; expected amendments).
- T6 §3 `run` step 2 (chain copy deferred; interim stamped_home): **routine: apply** (expected amendment).
- T7 §7 Fake agent: **routine: apply** (expected amendment); the transport's `<\` is restored to `<`.
- T8 §7 seed table agents row · T9 §3 boot `--agents-mode`/`--statusline-echo` · T10 §6 E1: **routine, no body change** (Sequencing deferral: the spec states the target, and ownership is a CARRY pin at P5). T9's "usage error" claim is a harness fact the report does not carry, so it is rejected.
- T11 §3 boot step 5 `--fixtures`: **escalated → resolved** (operator: amend the doc to the parent `fixtures/claude`; the fake agent joins `<cli-version>`). The `--plugin-dir` half resolves by architecture.md:345: `viola run` passes `--plugin-dir` to the child.
- T12 §2 fixture naming · T13 §7 seed table recorded-payload row: **escalated → resolved** (operator: amend to PascalCase `<Event>.<variant>.json`; "code wins where the doc invented a shape; no mapping tables").
- T14 §7 Fixture hygiene: **routine: apply** (expected amendment). The `/home/` sweep-hazard sentence is left to curation.

**Checks**
- Cross-contradiction: none.
- Intent-consistency: aligned (scope.md P4 decisions carry the deferrals).
- Absence-evidence: A6's rejection cites architecture.md:106 plus the test-plan pin.
- Expected-amendments floor: all 7 plan entries matched (test-plan §3 step 4 T1 · §12 T5 · §7 fake agent T7 · §7 hygiene T14 · §3 step 2 T6 · arch A1-A5 · matrix#v1-06 at P7.3).
- Disproved claims: 1 → A2, 2 → A1, 3 → T1/T2.
