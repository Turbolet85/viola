# Fan-out results — 2026-10-04-windows-boundary-mutation-workflow

Seven Explore doc-agents, one parallel batch, prompts from `amendment-flow.md` substituted verbatim. The returns came
through the subagent hand-back channel: no HTML entities in any return (`&lt;` / `&gt;` / `&amp;` absent: probe
`entities=0`). Comment lines after the YAML were stripped as commentary; their substance is noted per doc.

## Verdicts
- **architecture** — 7 proposals (A1–A7).
- **security-plan** — `proposals: []`. Stripped commentary: the three detectors (input · auth · deps) hold; it flags
  the stale CI-integration facts at :341 / :344 / :356 as outside its detectors → check 5 raises them (S1–S3).
- **design-system** — `proposals: []` (no UI; `tokens n/a`).
- **layout-templates** — `proposals: []` (no surface; 0 hits for workflow / mutant / ci.yml).
- **test-plan** — 8 proposals (T1–T8).
- **obs-plan** — `proposals: []`. Stripped commentary: §9 Platform :993 and the Mutation row :1013 are outside its
  detectors → check 5 raises them (O1, O2).
- **a11y-plan** — `proposals: []`. Stripped commentary: a11y-plan.md:888 "one push/PR workflow `ci.yml`" still true.
No `.raw-fanout-*` twin: the empty returns were unchanged by stripping except comment lines, and none failed the
parse or probe.

## Proposals and dispositions
### architecture
- **A1** D-arch-decisions · warning · §Infrastructure Patterns → CI/CD approach (`ci-cd-approach.md:2`) — "two
  workflows" → three; register `windows-mutants.yml` (dispatch-only, no inputs, `permissions: {}`, `windows-2025`,
  `contents: read`, 120 min, `fail-fast: false`, six-package matrix, ci.yml's pins, the pwsh `run --mutants --package
  … --file …` step, no cache / upload / secret / concurrency / needs, report-only). → **apply** (playbook "Accurate
  this-chunk addition", routine; check 5 entry 1).
- **A2** dependent-of A1 · same key — "Neither workflow has a `concurrency:` block" → "No workflow …". → **apply**
  (routine).
- **A3** dependent-of A1 · same key — the parenthetical "CI runs no mutation job" narrowed to push/PR; the dispatch
  workflow uploads nothing. → **apply** (routine).
- **A4** dependent-of A1 · `ci-cd-approach.md:5` — "CI runs no mutation job" → "ci.yml runs no mutation job"; the
  dispatch workflow named as the harness verb's Windows CI form. → **apply** (routine).
- **A5** dependent-of A1 · §Infrastructure Patterns → Project directory structure (`project-directory-structure.md:94`)
  — add `windows-mutants.yml` to the `.github/workflows/` tree. Its absence claim (no `tests/` per-binary line) is
  verified: `grep -n 'tests/'` → `:26` `├── tests/  # root integration tests (sync)` is one line with no enumeration.
  → **apply** (routine). The plan's entry "add `tests/contract_windows_mutation_scope.rs`" is **superseded**:
  registry over-reach (playbook "Registry over-reach", category grain).
- **A6** dependent-of A1 · §Occupied Resources (`architecture.md:379`, the `AGENT_RUN_CHUNK_BASE` row) — reworded so
  it stays true (the `--package` arm reads no base). → **apply** (routine).
- **A7** D-arch-resources · §Occupied Resources (`architecture.md:433`, the `viola-mutants-scratch` row) — the
  windows-2025 runner's checkout parent registered as an occupant. → **apply** (routine).

### test-plan
- **T1** D-tests-framework · **escalate** · §3 → Bootstrap phases (`bootstrap-phases-derive-for-route-setup-project.md:11`)
  — `terminate = "immediate"` → `"wait"`, the immediate reason retired, the measured reason in. → **escalate**:
  playbook "Accurate this-chunk addition" note excludes "A REVERSAL of a locked decision … escalate that once to
  ratify it"; routed to the overseer by pre-direction (1). Re-derivation note: the change line's "only nextest TIMEOUT
  / SIGKILL kills still leave dirs" overstates `leak.md` (21 fixture repos also remain). The applied text is
  re-derived.
- **T2** dependent-of T1 · escalate · §3 → Test data bootstrap (`test-data-bootstrap.md:15`) — the killed-test example
  no longer names `immediate`. → **escalate** with T1 (a dependent-of group applies atomically).
- **T3** D-tests-framework · warning · §9 Mutation row (`test-plan.md:1146`) — "none in CI" → no push/PR job or gate;
  the dispatch-only `windows-mutants.yml`. → **apply** (routine; check 5 entry 4).
- **T4** dependent-of · §9 Platform line (`:1133`) — workflows listed 2 → 3. → **apply** (routine).
- **T5** dependent-of · §9 tool-pin paragraph (`:1153`) — workflow list + the new workflow's `tool:` pins
  single-sourced by `contract_windows_mutation_scope`. → **apply** (routine).
- **T6** dependent-of · §9 concurrency note (`:1154`) — 2 low → 3 low. → **apply** (routine; check 5's beyond-list
  site).
- **T7** dependent-of · §9 Test report format, `mutants.out` bullet (`:1170`) — "no CI job runs mutation" → no push/PR
  mutation job; `windows-mutants.yml` uploads nothing. → **apply** (routine).
- **T8** dependent-of · §10 Mutation gate (`:1209`) — the dispatch workflow named as where cfg(windows) coordinates are
  measured, report-only, red until the audit classifies unix twins. → **apply** (routine).

### Raised by the orchestrator (Validate check 5 — expected amendments no detector proposed)
- **S1** security-plan §Dependency Security, CI integration header (`security-plan.md:341`) — the workflow list gains
  `windows-mutants.yml`. Report substantiates (Counts: workflows 2 → 3). → **apply** (routine).
- **S2** security-plan same section (`:344`) — "(2 low findings)" → 3 low; "CI runs no mutation job" narrowed.
  → **apply** (routine).
- **S3** security-plan (`:356`) — "runs no mutation job" narrowed (read at apply). → **apply** (routine).
- **O1** obs-plan §9 Platform (`obs-plan.md:993`) — the dispatch workflow named beside ci.yml / nightly.yml.
  → **apply** (routine).
- **O2** obs-plan §9 Mutation row (`:1013`) — "no CI job since 2026-09-28" → no push/PR job; the dispatch workflow is
  the boundary run's Windows form. → **apply** (routine).

## Escalation resolved
- **T1 + T2** — the overseer, as operator (founder-delegated), answered **Ratify**: "the 2026-09-24 choice was a
  chunk-level decision, not a founder ruling. The measured basis carries the reversal: 170 vs 0 leftovers two-sided,
  and 0 Timeout grades over 711 Linux + 508 Windows mutants. Record it as the operator ratification of 2026-10-04."
  → applied (body re-derived; sidecar entry names the ratification). No new playbook rule: the existing "Accurate
  this-chunk addition" note already routes a reversal to one escalation.

## Validate summary
1. Playbook: 20 routine ("Accurate this-chunk addition"); T1 + T2 escalate (reversal of a locked decision). Boundary
   widening checked: the workflow adds no permission, secret, input class, upload or crossing of a hardened boundary
   (report Symbols / Coverage) → not the widening class.
2. Cross-contradiction: none (A1–A4 are one key's coherent edit; T3–T8 distinct rows).
3. Intent-consistency: the report's deviations are justified (transitive propagation in-intent; the leak unmet →
   overseer ACCEPT); scope record empty, `gate.py scope` clean.
4. Absence needs evidence: A5's "no per-binary tests/ line" verified (above); T7's site read at apply.
5. Expected amendments: every plan entry matched (A1–A7, T1–T8) or raised (S1–S3, O1, O2); the
   `tests/contract_windows_mutation_scope.rs` tree line superseded (A5).
6. Disproved claims: #1 (leak acceptance, plan-level) → DISPOSED by the overseer, remainder routed to route-resolve
   :72; #2 (the `immediate` reason) → T1/T2 escalation.
