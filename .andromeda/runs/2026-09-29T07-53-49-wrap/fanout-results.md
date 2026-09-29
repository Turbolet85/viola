# Fan-out results — 2026-09-29-verify-stamped-test-homes-and-harness

Seven Explore doc-agents, one parallel batch, the amendment-flow prompt sent verbatim. Every return was YAML followed
by `#` commentary lines. Stripping removed only that commentary: a per-detector "no drift" rationale, no proposal
content. No return failed the parse. No entity-escaped `<`/`>`/`&` appeared in a value (the decoded text equals the
received text), so no raw twin is warranted.

## Verdicts
- design-system: `proposals: []` (D-design-tokens: every new surface `tokens n/a`)
- layout-templates: `proposals: []` (D-layout-surface: no UI surface; verify's stdout is unchanged)
- a11y-plan: `proposals: []` (D-a11y-surface: none interactive; D-a11y-obs-schema: a11y rows use a11y-row.v1.json, unaffected)
- security-plan: `proposals: []` (deps none; auth untouched; no new product input surface; the `interim` hits @207,234,719 are the ratified Epoch-6 gaps, a different fact)
- architecture: 9 proposals
- obs-plan: 8 proposals
- test-plan: 10 proposals

## architecture (9)
| # | detector | section | change (as proposed, abridged) | disposition |
|---|---|---|---|---|
| A1 | D-arch-resources | Occupied Resources → Environment variables (test-harness only) | register `CI`, harness presence-read refusing `run --local-live` (`live-in-ci`), never read by `viola` | APPLY — playbook "Accurate this-chunk addition" (report Schema/config names the read) |
| A2 | D-arch-resources | Occupied Resources → Filesystem `diagnostics/` (`cli-<name>.ndjson`) | verify's self pair holds a version-probe and a verify-probe pair; subject enum gains `verify-probe` | APPLY — accurate addition; expected amendment 6 |
| A3 | D-arch-resources | Occupied Resources → Filesystem (`viola-root-watch` Watch report) | 8 → 9 root waits, `wait_endpoint_gone` | APPLY — accurate addition (Counts moved) |
| A4 | D-arch-resources | Occupied Resources → Repository (`target/e2e-home/…`) | add `viola-live-<pid>/home`; test and harness homes are verify-stamped | APPLY — expected amendment 8 |
| A5 | D-arch-decisions | Established Decisions → [Error Handling] | `StampError` folded into `AgentError::StampsMalformed`; the pre-existing `Refusal` is named as the one open divergence | APPLY — expected amendment 7; disproved claim (3); the operator's word "carry the Refusal-enum line" |
| A6 | D-arch-decisions (dependent-of A5) | Conventions → Rust error types | retire the StampError interim note; name `Refusal` | APPLY (group A5) |
| A7 | D-arch-decisions (dependent-of A5) | Standard Contracts → Ledger stamps envelope | malformed envelope is `AgentError::StampsMalformed`; outcome unchanged | APPLY (group A5) |
| A8 | D-arch-decisions (dependent-of A5) | Inherited Defaults → Errors | retire the StampError clause; name `Refusal` | APPLY (group A5) |
| A9 | D-arch-decisions | Established Decisions → [PTY] H2 note | "the real-CLI verify entry" → "First live test and self-drive"; `--local-live` does not claim it | APPLY — expected amendment 9 |

## obs-plan (8)
| # | detector | section | change (abridged) | disposition |
|---|---|---|---|---|
| O1 | D-obs-instrumentation | §6 additive field catalog, `process-start` row | subject enum gains `verify-probe` | APPLY — expected amendment 5 |
| O2 | dependent-of O1 | §6 Boundary-call wrappers, child/shell spawns | spawn subject set gains `verify-probe`; `run_bounded` unlogged | APPLY (group O1) |
| O3 | dependent-of O1 | §4 `verify` process-log bullet | verify's log carries the two spawn pairs between its self start and exit | APPLY (group O1) |
| O4 | dependent-of O1 | §4 `verify` CI bullet | boot step 4 and `stamped_home` landed; real-claude verify only via local `run --local-live`, refused under `CI` | APPLY (group O1) |
| O5 | dependent-of O1 | §4 Scenario 1, product-order bullet | harness boot readiness now checks events 1-3 (`start_records`, `<name>:events`) | APPLY (group O1); binds with T1 (tests↔obs §3/§4) |
| O6 | dependent-of O1 | §1 trigger "Vector 8 (child spawn)" (:400) | name verify's print-mode probe | REJECT — playbook "Verbatim scope copy": §1 is lines 21-492 and stays verbatim; the fact lands in §4/§6/§12 (O1-O4, O8). Group integrity holds: rejecting a §1 dependent retires no claim the primary retires |
| O7 | dependent-of O1 | §1 Entity "`claude agents --json` and `--version` probe" (:120) | name verify's print-mode probe | REJECT — same rule, same reason |
| O8 | dependent-of O1 | §12 Obs Decisions Log | 2026-09-29 entry: `verify-probe` joins the closed subject enum | APPLY — obs rule: a new closed value needs a DL entry; expected amendment 5 |

## test-plan (10)
| # | detector | section | change (abridged) | disposition |
|---|---|---|---|---|
| T1 | D-tests-obs-harness | §3 boot Readiness signal (events bullet, :526) | boot checks lines 1-3 via `start_records`; drop "lines 1–2 today" | APPLY — expected amendment 2 |
| T2 | dependent-of T1 | §3 boot Readiness signal (staged codes, :523) | add `<name>:events` after snapshot/endpoint/heartbeat | APPLY (group T1) |
| T3 | D-tests-obs-harness | §3 `run` step 2 (:542), interim seam | `stamped_home` stamps through verify at 2.1.283; `StampedHome::unstamped`; `Wrapper::boot --cli-version` | APPLY — expected amendment 1 |
| T4 | D-tests-obs-harness | §3 `run` step 2 (:542), `WITHIN` sentence | 9 root waits; `wait_endpoint_gone`; stop waits for the endpoint to be unconnectable | APPLY — disproved claim (1); Counts moved |
| T5 | D-tests-obs-harness | §3 `run` `--local-live` bullet (:561) | CI refusal, build then verify, `viola-live-<pid>` home, suite and four codes, `run_with`'s `ci` | APPLY — accurate addition (the codes carried in Deviations with justification) |
| T6 | dependent-of T5 | §3 `run` Output format (:563) | suite enum gains `local-live` | APPLY (group T5) |
| T7 | dependent-of T5 | §3 Closed enums (:665-672) | suite `local-live`, reason `live-in-ci`, four codes, `<name>:events` | APPLY (group T5); expected amendment 3 |
| T8 | dependent-of T5 | §12 Test Decisions Log | 2026-09-29 entry for the chunk | APPLY (group T5); expected amendment 3 |
| T9 | D-tests-obs-harness | §7 Fake agent (:1379) | `DEFAULT_CLI_VERSION` 2.1.283; default answer; the harness constant | APPLY — expected amendment 4 |
| T10 | dependent-of T5 | §10 Perf session (:1517) | the perf boot is stamped (`stamp: true`); retire "nothing is stamped yet" | APPLY — accurate addition (the scope record's perf.rs line) |

## Orchestrator raises (Validate check 5 / check 6)
- R1 (check 5, expected amendment 9's duplicate): test-plan §5 H2 row (:926) says "owned by the real-CLI verify
  entry", the same claim A9 retires, and no detector proposed it. Raised ROUTINE: → "First live test and self-drive".
- Disproved-claims disposition (check 6):
  - (1) stop premise → T4 (and the root seam fact T3).
  - (2) the collision hypothesis → no master states it (it lived in scope.md only); disposed by the report and
    `evidence/`. No master edit.
  - (3) the `Refusal` enum → A5-A8 name it as the open divergence. The operator's "carry the Refusal-enum line" also
    takes a route CARRY at P5 (ownership is a route annotation, playbook "Sequencing deferral" note).
  - (4) gate 6's directory `artifact` → plan retarget (done on the operator's word) plus P3 curation (an `artifact`
    key names a file). No master states it.

## Validate — the six checks
1. Playbook: 25 applied under "Accurate this-chunk addition"; 2 rejected under "Verbatim scope copy". No collision,
   no boundary widening: `CI` is a test-harness read already specified at test-plan.md:561; `verify-probe` is a closed
   log value, codes only.
2. Cross-contradiction: none. T1/T2 and O5 state the same readiness fact from both sides of the tests↔obs bind. A2,
   O1-O4 and the schema agree.
3. Intent-consistency: the scope record's 6 companion lines each serve a listed file (`boot.rs` / `run.rs`); the
   `--local-live` build step and its extra codes are a justified divergence (the plan left the binary unstated),
   recorded in T5/T7. Consistent.
4. Absence needs evidence: no proposal claims an absence. The caught-ALL claims land in the cascade sweep (step 2).
5. Expected amendments: all 9 are covered (1 T3/T4 · 2 T1/T2 · 3 T5-T8 · 4 T9 · 5 O1/O2/O8 · 6 A2 · 7 A5-A8 ·
   8 A4 · 9 A9 plus R1).
6. Disproved claims: all four disposed (above).

Escalations: 0.
