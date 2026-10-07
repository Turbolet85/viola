# Fan-out results — 2026-10-07-test-homes-off-the-contended-volume

Seven doc-agents, one parallel batch; the prompts' detector counts sum to 15, the drift-base's 15 `doc:` names
(architecture 2 · security-plan 3 · design-system 1 · layout-templates 1 · test-plan 3 · obs-plan 3 · a11y-plan 2).
Entity probe on every return: `entities=0` (no `&lt;` `&gt;` `&amp;` in any of the seven; `<uid>` arrived as typed).
The `change` lines below are condensed by the orchestrator to their claim; the applied text is re-derived from the
report's facts, never pasted from them. Dispositions are §Validate's.

## Verdict lines
- architecture — 2 proposals. Stripped: none (the return was the YAML alone).
- security-plan — 1 proposal. Stripped: a leading comment block giving the three detector verdicts (none violated;
  the proposal is the report's expected amendment 3, filed under the nearest detector).
- design-system — `proposals: []`. Stripped: a trailing comment (no UI element, every Coverage row `tokens n/a`).
- layout-templates — `proposals: []`. Stripped: a trailing comment (no user-facing surface added).
- test-plan — 7 proposals. Stripped: a leading comment block (D-tests-coverage and D-tests-framework hold; the sites
  swept and left alone as true by path or CI-scoped).
- obs-plan — 3 proposals. Stripped: a leading comment block (the three detectors hold literally; the proposals are
  the report's expected amendments 5 and 6, filed under the nearest detector).
- a11y-plan — `proposals: []`. Stripped: a trailing comment (no interactive element, no schema moved; 0 hits swept).

The three empty returns carried commentary and no proposal the stripping removed: no raw twin is warranted beyond
this line (the substance of each stripped comment is named above).

## architecture
- **A1** · D-arch-resources · warning · §Occupied Resources → Repository (the `target/e2e-home/…` entry) · change:
  the base is prepared by the two keepers under one contract; on the Linux dev host the path is a link to the tmpfs
  backing; creation and removal go through the link, outside the working directory (founder-ratified); CI
  unlinked; the `cargo clean` gap; `target/e2e-home.disk/` registered.
  - **Disposition: apply** — expected amendment 1 (check 5); playbook "Accurate this-chunk addition". Its widening
    clause is the playbook's never-routine class: **escalated and resolved** on the founder's own live answer of
    2026-10-07T07:25Z, given after the widening was shown to him in those words (inputs#I6; the playbook's "what
    ratifies it" test holds), relayed by the overseer and named again as the ratification of these amendments in
    the overseer's disposition (1) at this wrap. One clause **not applied**: `target/e2e-home.disk/` is a leftover
    the operator's desk owns (disposition 2), not a standing resource; the handoff carries it.
- **A2** · D-arch-resources · warning · §Occupied Resources → Filesystem (a new test-only bullet) · change:
  register `/tmp/viola-e2e-home-<uid>/`, the dev host's 0700 tmpfs backing, its writers, the kept-home lifetime,
  never named by `viola`, no CI runner has it.
  - **Disposition: apply** — expected amendment 2 (check 5); playbook "Accurate this-chunk addition" (a resource of
    a kind the registry enumerates: a test-only site outside the repository).

## security-plan
- **S1** · D-security-input · escalate · §Security Anti-Patterns → Code Patterns · change: a bullet after the
  `run --mutants` scratch bullet: the test side's creation and removal reach the link's target on the dev host,
  only dirs it created, the keepers' three checks, `viola` never touching the backing, the ratification.
  - **Disposition: apply, re-derived** — expected amendment 3 (check 5). Playbook "Boundary widening" → escalate:
    **resolved** on the same ratification as A1. The proposal's opening ban ("NEVER let the test side create or
    remove anything outside the working directory except through … the link") is **not applied as written**: the
    test side already writes outside the working directory at sites architecture registers (the chaos home, the
    watch reports, verify's probe dirs), so the ban would be false. The applied bullet bans what the report
    supports: a removal under `target/e2e-home` taking anything but a dir the test side itself created.

## test-plan
One group: T1 primary, T2 to T7 `dependent-of: D-tests-obs-harness`. Validated and applied atomically.
- **T1** · D-tests-obs-harness · warning · §3 → 5-command implementation, `boot` step 2 · change: the base is
  prepared by `Workspace::ensure_e2e_home`; the link arm's contract; a refusal reads `build-failed`; the dev-host
  link, path unchanged.
  - **Disposition: apply** — expected amendment 4 (its `boot` site); "Accurate this-chunk addition".
- **T2** · §3 → 5-command implementation, `run --local-live` · change: the keeper after the build, before the
  verify; a refused base is `verify-exit-none`; no code added.
  - **Disposition: apply** — "Accurate this-chunk addition".
- **T3** · §3 → 5-command implementation, `cleanup` step 6 · change: on the dev host the `viola-session-*` removal
  deletes through the link, outside the working directory; only harness-created dirs; ratified.
  - **Disposition: apply** — widening clause, **escalated and resolved** as A1.
- **T4** · §3 → 5-command implementation, `logs` Retention window · change: on the dev host a kept home is on
  tmpfs: gone at a reboot, aged out of `/tmp` after ten untouched days; CI unchanged.
  - **Disposition: apply** — the tests side of the tests↔obs bind for O3 (cascade step 2).
- **T5** · §3 → Test data bootstrap, Mechanism · change: a bullet for `prepare_home_base` in `TestHome::new`, the
  same contract, a panic with a fixed message.
  - **Disposition: apply** — expected amendment 4; "Accurate this-chunk addition".
- **T6** · §3 → Test data bootstrap, Cleanup · change: drop and owner sweep delete through the link (ratified); a
  kept home's lifetime; the reboot and `cargo clean` behaviour; CI unchanged.
  - **Disposition: apply** — expected amendment 4 (Cleanup); widening clause **escalated and resolved** as A1.
- **T7** · §5 Integration Test Strategy → Setup / teardown lifecycle · change: the base is prepared by the keeper
  and the statement holds by path; the dev-host link; CI unchanged; not a third carve-out.
  - **Disposition: apply** — expected amendment 4 (§5); "Accurate this-chunk addition".

## obs-plan
- **O1** · D-obs-stack · warning · §9 CI Integration → Gate commands, G2's non-empty check · change: the start
  point reads `target/e2e-home/`; the comment says why.
  - **Disposition: apply** — expected amendment 6 and the report's first disproved claim (check 6); playbook
    "Accurate this-chunk addition" (illustrative command brought to the shipped script).
- **O2** · `dependent-of: D-obs-stack` · §9 Gate commands, G2's count line · change: the same start point.
  - **Disposition: apply** with O1 (the duplicate occurrence, one code block).
- **O3** · D-obs-stack · warning · §3 → Log file location, Rotation · change: on the dev host a kept home's
  diagnostics live on tmpfs behind the link, end at a reboot and age out after ten untouched days; CI unchanged.
  - **Disposition: apply** — expected amendment 5; "Accurate this-chunk addition".

## Validate, the six checks
1. Playbook: every proposal matched "Accurate this-chunk addition" (routine); A1, S1, T3 and T6 also carry the
   "Boundary widening" class (escalate), resolved as stated. No two rules collided with opposite verdicts on one
   clause: the widening rule governs the widening clause, the addition rule the rest. No re-derivation tell: every
   rationale cites the report or its own doc.
2. Cross-contradiction: none. No two proposals edit one section in opposing directions.
3. Intent-consistency: the report matches the working-route entry and the plan's acceptance criteria; six
   deviations, each justified in the report; scope record none (`gate.py scope` clean).
4. Absence needs evidence: the 0-hit claims cite their greps (security-plan `e2e-home` 0; obs registries `find
   target/e2e-home` 0). The cascade sweep re-reads them under control (`cascade-dispositions.md`).
5. Expected amendments: six entries, six matched (1 → A1 · 2 → A2 · 3 → S1 · 4 → T1, T5, T6, T7 · 5 → O3 · 6 → O1,
   O2). None raised by the orchestrator.
6. Disproved claims: the G2 start point → O1 and O2. The plan's step 5 check by process name and step 3's
   call-site swap are plan-text facts in no master → routed to curation (P3).

## Escalations
One class, four proposals (A1, S1, T3, T6): the boundary widening. No halt was taken at this wrap: the founder's
answer was given live after this specific widening was shown to him (inputs#I6, 2026-10-07T07:25Z, relayed by the
overseer), and the overseer's disposition with the wrap invocation names that answer as the ratification of the
widening amendments. Recorded in each owning sidecar as the founder's, the relay named. No playbook rule is
proposed: the class is never routine.
