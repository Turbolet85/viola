# Fan-out results — 2026-09-28-hook-perf-gate

Seven Explore doc-agents, one parallel batch, the verbatim amendment-flow prompt. Returns parsed as YAML; no HTML
entities in any return (none of the returns carried `&lt;`/`&gt;`/`&amp;`), no stripping changed a `proposals: []` return,
so no raw twin is kept.

## Verdicts
- architecture — 13 proposals (D-arch-resources 11 · D-arch-decisions 2)
- security-plan — 4 proposals (D-security-input, escalate; 3 dependents)
- design-system — `proposals: []` (no UI; tokens n/a on every surface)
- layout-templates — `proposals: []` (no user-facing surface; `viola hook` still has no human surface)
- test-plan — 17 proposals (D-tests-obs-harness 12 · D-tests-coverage 5)
- obs-plan — 4 proposals (D-obs-pii, escalate; 3 dependents) + a notes block naming 6 stale sites outside its
  detectors (§3 :621 · §10 :1316 · §10 :1320–:1321 · §9 G2 :1262–:1275 · §10 :1300–:1308/:1329 · §12 history, §1 verbatim)
- a11y-plan — `proposals: []`

## Parsed lists with dispositions

### architecture
| # | detector | section · basis | change (summary) | disposition |
|---|---|---|---|---|
| A1 | D-arch-resources | §Occupied Resources → Environment variables · :373–374 | second test-seam bullet `FAKE_AGENT_HOOK_PANIC` (seam.rs, exact `1`, after `viola_obs_init`, fixed 4 608 B payload, fail-open lines) | apply — playbook "Boundary widening" escalate → resolved: founder live ratification (A 06:21; A+B 09:52:07, relay the Viola overseer) |
| A2 | dep A1 | §Conventions Environment variables · :172 | "one ratified exception" → two seams | apply (A1 group) |
| A3 | dep A1 | §Established Decisions [Naming] · :107 | one exception → two | apply (A1 group) |
| A4 | dep A1 | §Cross-cutting Config management · :581 | "the one other variable" → the two seams | apply (A1 group) |
| A5 | D-arch-resources | §Occupied Resources `target/perf/` · :407 | build spelling `viola/fake-agent`; exports `target/agent-run/artifacts/perf-<hook>.json`, four rows | apply — routine "Accurate this-chunk addition"; closes report Spec-claims-disproved (arch :407) |
| A6 | D-arch-resources | §Occupied Resources `target/agent-run/` · :397 | `<session>/payload-<hook>.json` + `artifacts/perf-<hook>.json` | apply, the artifacts row only as a gate-read contract file; the per-session payload files are realization at category grain → folded as one clause, not a new row (playbook "Registry over-reach" for the payload half) |
| A7 | D-arch-resources | §Occupied Resources Repository · :402–413 | register `target/g2-probe/` | apply — routine (expected amendment 5) |
| A8 | D-arch-resources | §Infrastructure Patterns dir tree scripts/ · :499–516 | add `g2-zero-panics.sh` | apply — routine |
| A9 | D-arch-resources | §CI/CD Jobs wired today · :556 | 8/15 → 9/18; the `perf` job; test job G2 step | apply — routine (expected amendment 6) |
| A10 | dep A9 | dir tree ci.yml comment · :543–545 | list the 3-OS perf job | apply (A9 group) |
| A11 | dep A9 | §CI/CD Setup steps jq sentence · :555 | G2 is the fail-closed script in test + perf | apply (A9 group) |
| A12 | D-arch-decisions | §Stack Code quality row · :38 | add hyperfine 1.20.0 (the perf timing runner, `cargo install --locked` in the perf job and on the dev host), as measured at `evidence/hyperfine-host.md` | apply — routine (a dev tool the stack lists beside its siblings); version written with its host and basis |
| A13 | dep A12 | §CI/CD Setup steps · :555 | hyperfine installed as its own step in `perf` | apply (A12 group) |

### security-plan
| # | detector | section · basis | change (summary) | disposition |
|---|---|---|---|---|
| S1 | D-security-input (escalate) | §Input Validation · :232 | second test-seam row `FAKE_AGENT_HOOK_PANIC` | apply — escalation resolved by the founder's live ratification of A and B, 2026-09-28 09:52:07 (A first shown and ratified at 06:21), relay the Viola overseer |
| S2 | dep S1 | §Anti-Patterns Universal · :583 | "the one carve-out" → two seams | apply (S1 group) |
| S3 | dep S1 | §Secret Management Storage · :425 | one variable → two | apply (S1 group) |
| S4 | dep S1 | §Security Decisions Log · after :692 | `2026-09-28` entry — the seam + the G2 exact-path exemption, ratification as the founder's | apply (S1 group); the entry's ratification text re-derived: A 06:21, A+B 09:52:07, B marked new, relay the Viola overseer (the proposal named only 06:21) |
| S5 | raised (check 5) | §Dependency Security → CI integration | the `perf` job, its hyperfine install and its scan-gated uploads | apply — routine; expected amendment 4, no detector proposed it (0 perf-upload hits in security-plan) |

### test-plan
| # | detector | section · basis | disposition |
|---|---|---|---|
| T1 | D-tests-obs-harness | §3 run `--perf` · :559 | apply — routine |
| T2 | D-tests-obs-harness | §3 gate perf breach · :656 | apply — routine |
| T3 | dep T2 | §3 gate Output detail codes · :657 | apply |
| T4 | D-tests-obs-harness | §3 ci-tool-install G2/jq · :793 | apply — routine |
| T5 | dep T4 | §9 tool-install paragraph · :1454 | apply |
| T6 | D-tests-obs-harness | §10 Perf run rules Status · :1521 | apply — routine (expected amendment 11) |
| T7 | dep T6 | §9 Pipeline Perf row · :1448 | apply (expected amendment 10) |
| T8 | dep T6 | §2 pyramid Performance row · :435 | apply |
| T9 | D-tests-obs-harness | §10 Perf session · :1517 | apply — routine |
| T10 | dep T9 | §10 table session-end row · :1525 | apply |
| T11 | dep T9 | §10 table pre-tool-use row · :1527 | apply |
| T12 | D-tests-obs-harness | §10 Binary under test · :1516 | apply — routine (report Deviations: feature spelling) |
| T13 | D-tests-obs-harness | §9 Test report format · :1463, :1472 | apply — routine |
| T14 | D-tests-coverage | §6 Security sweep · :1254 | apply — routine; closes report Spec-claims-disproved (test-plan :1254) (expected amendment 9) |
| T15 | dep T14 | §6 matrix summary · :1266 | apply |
| T16 | dep T14 | §1 Test Scope Summary · :331 | apply |
| T17 | dep T14 | §5 Module ↔ DB concurrent append · :924 | apply (expected amendment 14's test-plan half) |
| T18 | D-tests-coverage | §5 CLI controls bullet · :963 | apply — routine (expected amendment 8) |

### obs-plan
| # | detector | section · basis | disposition |
|---|---|---|---|
| O1 | D-obs-pii (escalate) | §8 item 6 Detail-file upload · :1206 | apply — escalation resolved by the operator (overseer): "the same channel (CI artifact upload), the same scan gate, the same synthetic-input basis as the ratified diag-<os>, only a second artifact name. No new crossing" — recorded in the obs sidecar |
| O2 | dep O1 | §8 item 6 Scan failure · :1215 | apply (O1 group) |
| O3 | dep O1 | §9 artifact table hyperfine row · :1244 | apply (O1 group); closes report Spec-claims-disproved (obs `perf/hook-<event>.json`) in part |
| O4 | dep O1 | §9 Step 1 · :1285 | apply (O1 group) |
| O5 | raised (check 5, exp. 14) | §3 Logging stack D-28 · :621 | apply — routine |
| O6 | raised (check 5, exp. 13) | §10 status · :1316 | apply — routine |
| O7 | raised (check 6, disproved) | §10 perf verification · :1320–:1321 | apply — routine |
| O8 | raised (check 5, exp. 12) | §9 G2 · :1262–:1275 | apply — routine (the exact-path exemption + probe; ratified B) |
| O9 | raised (check 3) | §10 zero-panic invariant · :1300–:1308, :1329 | apply — routine: the invariant names the G2 seam exemption |
| — | notes | §12 :1750–:1751 (Decisions Log history), §1 :417–:423 (verbatim copy) | no change — history stays; §1 is the verbatim scope copy (playbook "Verbatim scope copy") |

## Validate summary
- Check 1 playbook: every non-escalated proposal matches "Accurate this-chunk addition" (routine); A6's payload half
  "Registry over-reach". Escalations: the widening class (A1 group, S1 group — resolved by the founder live) and D-obs-pii
  (O1 group — resolved by the operator as the existing class).
- Check 2 cross-contradiction: none (arch A5 and test T12 carry the same feature spelling; O3 and T13 the same upload
  gating).
- Check 3 intent-consistency: the scope record's one companion line serves `viola-harness.rs` (the `--perf` flag) and
  holds; the G2 exemption is within intent (operator P4 fork 2) and now ratified.
- Check 4 absence-needs-evidence: the caught-ALL claims rest on `sites.py`'s per-master listing and the cascade sweep.
- Check 5 expected amendments: all 15 covered — 4 by S5 (raised), 12 by O3/O4/O8, 13 by O6, 14 by O5 + T17; the rest
  by detector proposals.
- Check 6 disproved claims: test-plan :1254 → T14; arch :407 → A5; obs `perf/hook-*` → O3 + O7.
