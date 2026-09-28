# Fan-out results — 2026-09-28-mutation-testing-to-the-epoch-boundary

Report: `viola-0.1.0/chunks/2026-09-28-mutation-testing-to-the-epoch-boundary/report.md`. Seven Explore doc-agents,
one parallel batch, the amendment-flow prompt verbatim. No return needed stripping beyond trailing `#` notes (kept as
the agents' checked-no-proposal notes, summarised per doc); no HTML entity in any parsed value; no raw twin warranted.

## Verdicts
- **architecture** — 12 proposals (D-arch-decisions 8 · D-arch-resources 4).
- **security-plan** — 10 proposals (D-security-auth 5 · D-security-deps 5). Notes: D-security-input no drift
  (removals only); `TMPDIR` hits :57 :66 :155 :203 :503 :604 are the Unix socket dir; :331 :702 still true; :656 :692
  :718 :738 :748 are dated history.
- **design-system** — `proposals: []` (0 new UI; 0 sweep hits).
- **layout-templates** — `proposals: []` (0 new surface; 0 sweep hits).
- **test-plan** — 37 proposals (D-tests-framework 34 · D-tests-coverage 3). D-tests-obs-harness: no drift (obs §3 names
  no leg/pre-push/mutation surface).
- **obs-plan** — `proposals: []`; note: §8 item 6 (:1208 :1210 :1212), §9 (:1243 :1256), §10 (:1289) are plan-listed
  expected amendments outside its detectors → raised by the orchestrator (check 5).
- **a11y-plan** — `proposals: []`; note: :1153 (§10, not §9 — the report's label corrected) is outside its detectors →
  raised by the orchestrator (check 5); :1145 "the union of the per-state axe verdicts" — no change.

## Dispositions (check that decided)
### architecture (all: playbook "Accurate this-chunk addition" → APPLY; re-derived from the report, not pasted)
A1 Stack/Code quality :38 syn+proc-macro2 clause out · A2 Crate dependency direction :465 viola-e2e syn line out ·
A3 Stack/CI-CD :36 download-artifact pin out + pre-push "mutation leg" out · A4 CI/CD Setup steps :571 download-artifact
sentence out · A5 CI/CD jobs :572 9/18 → 7/15 at ci#36483042659, mutants + mutants-verdict descriptions out ·
A6 CI/CD pre-push :588 reduced stages, TMPDIR out, cache fields · A7 directory tree :559 ci.yml comment ·
A8 CI/CD concurrency :569 mutation clause · A9 Occupied Resources Repository :408 chunk.diff owner + leg verdict out ·
A10 Occupied Resources :427 host scratch "pre-push windows leg and CI's windows-2025 leg" out · A11 env
`AGENT_RUN_CHUNK_BASE` :377 re-scoped to `run --mutants` · A12 Filesystem :398 `Linux::cmd_env` → `Linux::cmd`.

### security-plan
S1 §Secret Management Development :432 TMPDIR carve-out retired → APPLY (narrowing, not the widening class) ·
S2 Decisions Log new 2026-09-28 narrowing entry (supersedes the 2026-09-27 TMPDIR carve-out and the 2026-09-24
unscanned `mutants-verdict` upload; past entries untouched) → APPLY · S3 §Bootstrap `secret-scanning-ci-gate` :395
two unscanned uploads → one → APPLY · S4 same :394 chunk.diff owner → APPLY · S5 §Secret Management scanning :447
chunk.diff owner → APPLY · S6 §Dependency Security CI integration :346 download-artifact bullet out (5 → 4) → APPLY
(detector severity escalate; the ban invariant holds — a removal; playbook "Accurate this-chunk addition") ·
S7 Threat Model Summary :131 → **REJECT** (playbook "Verbatim upstream copy (other masters)"; precedent: security
sidecar :98 :140 :166 — every wrap since the rule keeps the section verbatim; the fact lands in S6) ·
S8 Threat Model Summary :161 → **REJECT** (same rule; the fact lands in S6/S9/S10 and arch A5) · S9 §Dependency
Security concurrency note :338 → APPLY · S10 §Dependency Security :348 mutation-base line re-scoped → APPLY.
Collision note: "Accurate this-chunk addition" also reads on S7/S8 — not escalated: the verbatim rule is the later,
section-specific ruling and has governed this section at three consecutive wraps; the plan's "weigh" is settled by
that precedent.

### test-plan (all APPLY under "Accurate this-chunk addition", re-derived)
T1 §2 Mutation row :437 · T2 §1 tier justification :28 (date corrected on apply: the founder ruling is 2026-09-28
17:59 via the overseer, not "2026-09-27") · T3 §1 `run` entity :182 · T4 §1 mutation trigger :414 · T5 §3 Exit codes
:509 · T6 §3 `run` body :533 · T7 §3 step 4 Base :552 · T8 §3 step 4 Command :554 · T9 §3 Exit code semantics :562 ·
T10 §3 Output format :565 · T11 §3 Test selection :573 · T12 §3 `secret-scan` :628 · T13 §3 `gate` signature :644 ·
T14 `gate` Inputs :649 · T15 `gate` breaches :655 · T16 `gate` Output :657 · T17 `gate` in CI :659 · T18 `pre-push`
Stages :662 · T19 `pre-push` Sync :663 · T20 `pre-push` document :664 · T21 Closed enums verdict :668 · T22 Closed
enums usage :669 · T23 Closed enums pre-push :671 · T24 Bootstrap `quality-gate-config-emit` :803 · T25 §9 Coverage
report row :1448 · T26 §9 Mutation row :1449 · T27 §9 Quality gates :1454 · T28 §9 Matrix builds :1460 · T29 §9 Test
report format :1465 · T30 §9 mutants upload :1472 · T31 §9 Build failure :1483 · T32 §10 Mutation gate :1511 ·
T33 §10 Build failure :1545 · T34 §11 CI :1611 · T35 §12 Decisions Log 2026-09-28 entry (date and ratifier
corrected: founder 2026-09-28 17:59, overseer relay; macOS arm by the operator's P5 ruling) · T36 §10 macOS exclusion
(folded into T32's apply) · T37 §2 Unit row :431 macOS carve-out · T38 §5 oversize-frame ENOTCONN :939.
(The agent's list numbers 37; T36 and T32 share §10 and apply as one edit.)

### Orchestrator-raised (check 5 — the plan's Expected amendments floor; routine, the report substantiates each)
R1 obs-plan §8 item 6 (:1208 :1210 :1212) mutation-leg / scratch / mutants-verdict clauses → current truth ·
R2 obs-plan §9 (:1243 :1256) Mutation row → the epoch-boundary audit · R3 obs-plan §10 (:1289) surviving-mutant
clause → judged at `run --mutants` / the audit · R4 a11y-plan §10 :1153 "under the tests' mutation gate" → current
truth. obs §1 untouched (playbook "Verbatim scope copy").

### Checks
1 playbook — as above · 2 cross-contradiction — none (A5/S9/T17 state one fact three ways, consistent) · 3 intent —
the scope record's `widening` `tests/channel_endpoint.rs` carries the operator's word ("Fold every red into this
chunk") → the justified branch: the acceptance's "no root test" clause was incomplete for a folded red; recorded in
the report's Outcome, T38 carries the fact · 4 absence — no proposal claims absence without its hits · 5 expected
amendments — every plan entry matched (arch A1–A12; security S1–S10; test-plan T1–T38; obs R1–R3; a11y R4 found by
the report's sweep); route/cascade entries → P5 / cascade · 6 disproved claims — both live in chunk evidence and the
scope's CARRY 2 (0 master hits for `73 s|jobserver|XProtect`): DISPOSED to `evidence/macos-mutants-phases.md` + the
CARRY's closure at P5.

**Escalations: 0.**
