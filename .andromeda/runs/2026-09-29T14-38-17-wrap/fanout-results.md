# Fan-out results — 2026-09-29-fake-agent-drift-contract

Seven Explore doc-agents, one parallel batch, the amendment-flow prompt verbatim; each return stripped of its
YAML-comment commentary and parsed. The architecture return arrived with the harness's `harness-envelope-tag`
neutralisation (`<` → `<\` on the quoted prefix literals `<agent-message from=` / `<task-notification>` — the
agent quoting the spec's own tags, no instruction); its `change` lines are therefore never pasted — the applied text
is re-derived from the report (Apply step 1). No `proposals: []` return was altered by stripping beyond the comment
lines, so no raw twin is saved.

## Verdicts
- architecture — 3 proposals (D-arch-decisions ×3). D-arch-resources: no drift.
- security-plan — `proposals: []` (D-security-input: no new external-input surface, :224 / :573 still hold;
  D-security-auth, D-security-deps: none). Note returned: the Decisions Log entry (expected amendment 2) is outside its
  detectors → raised by the orchestrator (R1, R2).
- design-system — `proposals: []` (no UI; every `tokens` flag n/a).
- layout-templates — `proposals: []` (no surface; layout :263 `prompt · harness` tape line already covers the class).
- test-plan — 3 proposals (D-tests-framework ×3). D-tests-coverage, D-tests-obs-harness: no drift.
- obs-plan — `proposals: []` (no hot path, dependency or logging; obs-plan states none of the retired claims).
- a11y-plan — `proposals: []` (no interactive element; no schema change).

## Proposals (parsed) and dispositions

### architecture
- **A1** D-arch-decisions · warning · §Established Decisions → [Human Takeover / Wheel] (:70) — the harness-injected
  prompt set becomes the compiled four-entry `HARNESS_PREFIXES` (`<agent-message from=`, `<task-notification>`,
  `<\cross-session-message`, `<cross-session-message`), raw `starts_with`, no trim; the side effect (a human typing the
  cross-session tag at a prompt's start is filed `harness`) accepted by the founder.
  → **escalate** (playbook "Boundary widening" + "what ratifies it": a validated classifier admits a new input class to
  `harness`) → **resolved** on the founder's live ratification, 2026-09-29 15:21:44, given AFTER the widening and its
  side effect were shown (plan.md §Provenance P4 fork 1; relay: the Viola overseer; re-stated by the operator's wrap
  directive) — the on-disk answer the rule admits. **apply**, recorded as the founder's.
- **A2** dependent-of D-arch-decisions · [CLI Version Compatibility] → Tag escaping row (:89) — escaped means typed,
  with the one exception: the cross-session tag's escaped form is itself the CLI-injected one; that injection form is
  relayed, not measured here. → same escalation, resolved as A1. **apply** (epistemic status in the body: relayed,
  measured by the first live test).
- **A3** dependent-of D-arch-decisions · [CLI Version Compatibility] → Harness prompt prefixes row (:80) — name the
  four-entry set and the two forms' relayed status. → same escalation, resolved as A1. **apply**.

### test-plan
- **T1** D-tests-framework · warning · §2 test pyramid, Contract row (:438) — insta pins decision bodies only; the fake
  agent's hook sequence/payloads are compared byte for byte against the recorded fixtures (receipt `stdin_hex`, spine
  order a test literal). → playbook "Accurate this-chunk addition" (the report's Changes carry `drift`, `stdin_hex`,
  Dependencies "insta is NOT added"; disproved claim 1) → **routine, apply**.
- **T2** dependent-of · §6 Contract suite (:1336) — "pinned with insta" → "compared byte for byte against the recorded
  fixture files, the spine order a test literal". → routine, **apply** (expected amendment 4).
- **T3** dependent-of · §7 Seed strategies table (:1359) — split the row: decision bodies stay insta; fake-agent
  transcripts pin against the recorded fixture files. → routine, **apply**.

## Raised by the orchestrator (Validate check 5 — expected amendments; check 6 — disproved claims)
- **R1** security-plan §Security Decisions Log — a `2026-09-29` entry: the cross-session tag filed `harness`
  (escaped and plain, at a prompt's start), a boundary widening ratified by the founder live 15:21:44 (relay: the Viola
  overseer), the side effect named, the escaped-injection form relayed until the first live test (expected amendment
  2). → escalate (Boundary widening) → **resolved** as A1. **apply**.
- **R2** security-plan §Security Anti-Patterns → Code Patterns (:573) — the ban stands (compiled list); it gains the
  set's current content and the accepted side effect (expected amendment 2). → escalate → resolved as A1. **apply**.
- **R3** test-plan §6 Path 5 (:270) — the harness-injected turn list becomes the four prefixes (expected amendment 3).
  → routine (Accurate this-chunk addition; the widening itself resolved at A1). **apply**.
- **R4** test-plan §4 viola-agent-claude (:872) — M2 prefixes → four, the escaped cross-session form the one escaped
  prefix that classifies (expected amendment 3). → routine. **apply**.
- **R5** test-plan §6 Contract suite (:1337) — the S8 `annotations` bullet is owned by "Dialog answers by dialog_id",
  where the question answer path lands (expected amendment 4; disproved claim 3). → routine. **apply**.
- **R6** test-plan §7 Fake agent (:1365) — matchers deferred to "Dialog answers by dialog_id" (was "lands with the
  fake-agent drift contract"); the UserPromptSubmit payload keeps the fixture's trailing newline (expected amendment 5;
  disproved claim 2). → routine. **apply**.
- **R7** test-plan §7 receipt kinds, `size` (:1371) — written at start and on every change by a watcher on the control
  poll, no key needed; interactive mode only (expected amendment 5; disproved claim 4). → routine. **apply**.
- **R8** test-plan §7 receipt kinds, `hook` (:1378) — `stdin_hex` joins the fields written when it ran (expected
  amendment 5). → routine. **apply**.
- **R9** test-plan §12 Test Decisions Log — a `2026-09-29` entry: the fake-agent contract pins against the recorded
  fixture files, not insta (P4 fork 4, the overseer agreeing); matcher evaluation and S8 `annotations` move to "Dialog
  answers by dialog_id". → routine (the chunk's recorded decision). **apply**.

## Checks
1. Playbook — above per proposal. No rule collision.
2. Cross-contradiction — none: A1-A3 and R1-R2 state one widening; T1-T3 and R5/R9 one insta retirement.
3. Intent-consistency — the route entry (:63) names matcher evaluation and "annotations forwarded"; the chunk moved
   both to "Dialog answers by dialog_id" on P4 operator forks with the overseer agreeing → a JUSTIFIED divergence
   (intent incomplete: its premises were false at HEAD, report disproved claims 2-3) → amended at P5 by re-pinning both
   on :72. The scope record's one line (`tests/tui_pty_seam.rs` · in-intent · serves step 5) holds: step 5 is the plan's.
4. Absence needs evidence — the architecture agent's "only :70, :80, :89" is re-derived by the cascade sweep (its
   patterns over all seven masters, long lines by offset) before any sidecar entry.
5. Expected amendments — 1 → A1-A3 · 2 → R1, R2 · 3 → R3, R4 · 4 → T2, R5 · 5 → R6, R7, R8 (plus T1, T3 for the other
   insta sites). None under-run.
6. Disproved claims — 1 → T1-T3 · 2 → R6 · 3 → R5 + P5 re-pin · 4 → R7. All DISPOSED.

**Escalations:** 5 (A1, A2, A3, R1, R2 — one widening), resolved on the founder's recorded live ratification; 0 open.
