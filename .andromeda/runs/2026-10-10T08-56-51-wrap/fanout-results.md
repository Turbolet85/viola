# Fan-out results — 2026-10-10-windows-mutation-grade

Seven doc-agents, one batch, 15 detectors (architecture 2 · security-plan 3 · design-system 1 · layout-templates 1 ·
test-plan 3 · obs-plan 3 · a11y-plan 2; the sum equals the drift-base's `doc:` names). Report read:
`viola-0.1.0/chunks/2026-10-10-windows-mutation-grade/report.md`. The entity probe found no escaped `<`, `>` or `&`
entity in any return.

## Verdicts

- architecture — 5 proposals (A1 to A5). No text stripped.
- security-plan — 2 proposals (S1, S2). Stripped: a preamble of comment lines giving the three detectors' verdicts
  (all three invariants hold) and saying both proposals are stale doc facts filed under the nearest detector.
- design-system — `proposals: []`. Stripped: five comment lines (no UI rendered; every Coverage row reads
  `tokens n/a`; the doc holds no claim about the workflow or the harness). Raw twin `.raw-fanout-design-system.md`.
- layout-templates — `proposals: []`. Stripped: one comment line (no user-facing surface or region added). Raw twin
  `.raw-fanout-layout-templates.md`.
- test-plan — 15 proposals (T1 to T15). Stripped: a closing comment block (D-tests-framework clean; new symbols
  each carry a test; four key files untouched; the obs row is the obs detector's).
- obs-plan — 3 proposals (B1 to B3). Stripped: a preamble (D-obs-stack and D-obs-pii clean; D-obs-instrumentation's
  invariant holds, the three proposals filed under it as the nearest detector).
- a11y-plan — `proposals: []`. Stripped: three comment lines (both invariants hold; the §10 Standard+ sentence at
  line 933 "is not falsified by the Changes" and "covers" still reads true over the measurable set). Raw twin
  `.raw-fanout-a11y-plan.md`.

## Proposals and dispositions

| # | detector | section | change, in short | disposition |
|---|---|---|---|---|
| A1 | D-arch-resources | §Infrastructure Patterns → CI/CD approach | one job definition `mutants (<label>)` over a nine-item `matrix.include`, the `viola` item split per file; the contract test keeps labels distinct | APPLY — playbook "Accurate this-chunk addition"; the plan's third expected entry. The count is written with its rule. |
| A2 | D-arch-resources, dependent of A1 | §Infrastructure Patterns → Project directory structure | the `windows-mutants.yml` tree comment: per labelled matrix item | APPLY — same rule; the third expected entry names it. |
| A3 | D-arch-decisions | §Infrastructure Patterns → CI/CD approach | "its jobs read red while scoped files carry `#[cfg(unix)]` twins" retired; the harness leaves host-excluded missed mutants out and the harness document judges a job | APPLY — the claim is this chunk's to retire (report, Counts and Harness / gate surface); the same claim as T9 in the same direction. |
| A4 | D-arch-decisions | §Stack and Technologies (Code quality row) | syn `=2.0.119` and proc-macro2 `=1.0.107`, direct dependencies of test-only `viola-e2e` | APPLY — "Accurate this-chunk addition". Home: §Stack, beside jsonschema. The plan named the key Crate dependency direction; that key's own line says viola-e2e's dependencies are not listed there (read), so it takes no edit. |
| A5 | D-arch-resources | §Infrastructure Patterns → Project directory structure | the `viola-e2e/` tree comment names `harness::run::mutants::host` | APPLY, at the comment's own grain (it already names `harness::pre_push`) — the third expected entry names the new module, a recorded direction (check 1). "Registry over-reach" does not govern: the row is a tree comment, not a resource registry. |
| S1 | D-security-deps (as nearest), `warning` | §Dependency Security → CI integration (Permissions sub-bullet) | "six-package matrix" becomes a nine-item matrix with its rule, job name `mutants (<label>)` | APPLY — "Accurate this-chunk addition"; the fourth expected entry. |
| S2 | D-security-deps (as nearest), `warning` | §Threat Model Summary → Infrastructure → CI/CD (the Jobs sentence) | drop "mutation (ubuntu and windows legs plus a union verdict)", name the per-OS `perf` job | APPLY — check 1's recorded-direction arm: the plan's fifth expected entry, approved at P5, names the change, and the route CARRY that carries it sits on this chunk's own line, so this chunk is its owner ("Not this chunk's drift" sends a real pre-existing drift to its owned channel, which is here). "Verbatim upstream copy kept current" applies: judged like any body amendment, the label stays. No count is written. |
| T1 | D-tests-obs-harness | §3 → 5-command implementation (`run` step 4, Verdict) | the host exclusion stated; the requirement reads over what is not left out | APPLY — the first expected entry. obs-plan §3 says nothing of the `mutants` object (the detector's read), so the §3 bind holds one-sided. |
| T2 | dependent of T1 | §3 → 5-command implementation (Exit code semantics) | "no missed mutant" reads after the exclusion | APPLY |
| T3 | dependent of T1 | §3 → 5-command implementation (Output format, "Built from") | `survived` = missed − left out + timeout | APPLY |
| T4 | dependent of T1 | §3 → 5-command implementation (Output format, the `mutants` object) | the optional `host_excluded` array of `{name, cfg}` | APPLY |
| T5 | dependent of T1 | §3 → 5-command implementation (`gate`, the `mutants` suite line) | the same `survived` formula | APPLY |
| T6 | dependent of T1 | §2 Test Strategy (Mutation row) | "any `missed` outcome" qualified | APPLY |
| T7 | dependent of T1 | §10 Mutation gate (`missed == 0`, twice) | read after the harness's exclusion | APPLY |
| T8 | dependent of T1 | §10 Mutation gate ("A mutant this host cannot compile or reach") | split by mechanism: a `cfg`-attribute mutant is left out and named by the harness; a const-gated one stays a by-coordinate record | APPLY, re-derived from the report (Harness / gate surface; Counts, the Windows-grades line). The proposal's citation of `evidence/host-excluded.md` is not relied on. The overseer's ruling of 2026-10-04 in the sentence is not reworded. |
| T9 | dependent of T1 | §10 Mutation gate ("Its jobs read red until…") | the first run's reading kept as dated history; run 38036448183 named with what it read | APPLY — the seventh expected entry; check 6 (the report's third disproved claim). The dispatch is named as this chunk's exception with its date; ruling C2's wording is untouched (the operator's wrap note 3, `inputs#I6`). |
| T10 | dependent of T1 | §10 Build failure conditions | the missed-mutant condition qualified | APPLY |
| T11 | dependent of T1 | §11 Test Anti-Patterns → Quality | judge by the harness document, never cargo-mutants' exit code or summary line alone | APPLY |
| T12 | D-tests-obs-harness | §9 Pipeline structure (Mutation row) | nine `mutants (<label>)` jobs, one per matrix item, the count with its rule | APPLY — the second expected entry. The dispatch wording of the row is untouched. |
| T13 | dependent of T12 | §10 Mutation gate ("per package on `windows-2025`") | per matrix item | APPLY, folded into T9's sentence group. |
| T14 | D-tests-coverage | §10 Mutation gate (the 2 missed `prepare` mutants) | the measurement owed to "Windows mutation grade" was made: both caught in run 38036448183 | REJECTED before the checks — it rests on `evidence/survivors.md` and `evidence/host-excluded.md`, which the report did not carry when the detector read it. The fact is real and is the seventh expected entry's "Windows grades": raised by the orchestrator as O1. |
| T15 | D-tests-coverage | §10 Mutation gate (the measurable set) | name the four `src/cmd/run.rs:385:5` mutants as measured on no host | REJECTED before the checks — it rests on `evidence/host-excluded.md`, and `src/cmd/run.rs` is not a file of this chunk. Its fact goes to the route step (P5), where the plan sends it. |
| B1 | D-obs-instrumentation (as nearest) | §9 Pipeline integration (Mutation row) | the obs-code reading: `mutants (viola-panic-frames)`, 23 tested, 14 caught, 9 left out, run 38036448183 | APPLY — the eighth expected entry. The invocation form in the row did not change and is not edited. |
| B2 | D-obs-instrumentation (as nearest) | §9 Pipeline integration (Mutation row) | "a missed obs-code mutant is red" reads over what the harness does not leave out | APPLY |
| B3 | dependent of B2 | §10 Build / deploy failure conditions | "a surviving mutant" is the harness document's `survived` | APPLY |
| O1 | check 5 (the seventh expected entry) | test-plan §10 Mutation gate (the 2 missed `prepare` mutants) | owed to "Windows mutation grade" becomes measured: both caught on `windows-2025` in run 38036448183; on Linux they still read missed and stay "not measured here" | APPLY — routine: the report substantiates it (Counts, the Windows-grades line, added at Validate with its basis). |
| O2 | the operator's wrap note 3 (`inputs#I6`) | architecture §Infrastructure Patterns → CI/CD approach · security-plan §Dependency Security → CI integration · test-plan §10 Mutation gate (and the run named in obs-plan §9's row by B1) | beside each statement of when the workflow is dispatched: one dispatch was made outside the audit, on 2026-10-10, for this chunk alone, on the founder's own word; the ruling stands as written | APPLY — the operator's recorded direction names the change. No sentence that states ruling C2 is reworded; one sentence is added after it. |
| O3 | orchestrator, found at the cascade | test-plan §3 → Bootstrap phases (derive for route / setup-project), the `viola-e2e` dependency line | syn and proc-macro2 added to the list of that crate's dependencies | APPLY — "Accurate this-chunk addition": the report's Dependencies bullet carries both; the line lists that crate's own dependencies and A4's fact belongs beside it. |

## The six checks

1. Playbook: A1, A2, A4, S1, T1 to T7, T10 to T13, B1 to B3 and O1 match "Accurate this-chunk addition". A3, T8 and
   T9 retire or restate a claim the chunk itself made false: the same rule's second arm (the body reconciled to the
   shipped mechanism, the invariant standing). A5 and S2 are settled by the plan's recorded direction. No proposal
   widens a boundary. Nothing escalated.
2. Cross-contradiction: none. A3 and T9 edit the same claim in two masters in one direction; A1, S1, T12 and B1
   state one job shape.
3. Intent-consistency: the report does not diverge from the route line or the acceptance criteria. The scope record
   holds no line (`scope: clean`, 0 recorded).
4. Absence needs evidence: every "no other site" rests on the sweep in `cascade-dispositions.md`, run after the
   last amendment was authored.
5. Expected amendments, nine entries: first → T1 to T5; second → T12; third → A1, A2, A5 (and A4 in §Stack);
   fourth → S1; fifth → S2; sixth (a11y-plan §10, "if the wrap's detector finds the sentence needs it") → NOT
   APPLIED: the detector read the sentence at line 933 as still true, and so does the orchestrator (the
   crate-test invariants it names are covered by the audit over the mutants a host compiles); seventh → T9, T13
   and O1; eighth → B1; ninth (the `testing.md` line) → P3.
6. Disproved claims: the plan's wall-time note → no master carries it; its learning goes to P3. The plan's
   forecast of 29 → recorded in `evidence/host-excluded.md`, no master carries it. `test-plan.md:1212` → T9.

Rejected for a source the report does not carry: 2 (T14, T15).
