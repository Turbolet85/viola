# Answer card — 2026-11-20-permissionrequest-question-answered (wrap 2026-11-20T13-41-57Z)

The founder's own answers, relayed by the overseer (`inputs#I4`, the phase run's `founder-forks.md` § The founder's
answers; `inputs#I3`):
- Q1 «Удерживать такой вопрос (Recommended)» — W1: the arm-less question raised by PermissionRequest held and answered
  `allow` + `updatedInput`; the rule `decision/permission-request-question-stays-null` ends, over the nine reached
  rows and nothing wider; the strict-kind residual shown as unmeasured.
- Q2 «4-й промпт в конце Run C (Recommended)» — W2.
- Q3 «Только 2.1.328 (Recommended)» — W3.
- The cap «1 старт + 1 запасной (Recommended)» — one start `PQ`, its kept stamp under the rig.

## The operations lying wholly inside those answers (8 lines)

1. `ops-main-architecture.md · decision/permission-request-question-stays-null` · established-decisions · fit: — (a retire) — Q1/W1: the rule ends; heir `decision/permission-request-question-answered`.
   - old rule: «A question first raised by PermissionRequest stays null to the human.»
   - new: retired.
2. `ops-main-architecture.md · decision/permission-request-question-answered` · established-decisions · fit: fits type 1 — Q1/W1, the replacement carrying the strict-kind residual in its except.
   - new name: «A question first raised by PermissionRequest is answered»
   - new rule: «A question first raised by PermissionRequest with no matching armed dialog is held for the driver; the answer returns as PermissionRequest allow with updatedInput, the tool input plus answers and annotations; a human key, an expired hold or an unverified CLI leave it to the human.»
   - new why: «Ratified to end the rule that left such a question null; the body now has its own relied ledger row and verify probe.»
   - new except: «Unmeasured: a user argument to viola run selecting the CLI's strict session kind, which refuses a changed input; the human then sees a denied tool call and no dialog.»
3. `ops-architecture.md · contract/hook-contract/permission-request` · standard-contracts · fit: fits type 3 — Q1/W1.
   - old promise: «PermissionRequest returns `{"hookSpecificOutput":{"hookEventName":"PermissionRequest","decision":{"behavior":"allow|deny","message":"…"}}}`.»
   - new promise: «PermissionRequest returns `{"hookSpecificOutput":{"hookEventName":"PermissionRequest","decision":{"behavior":"allow|deny","message":"…"}}}`; for a question the decision is `{"behavior":"allow","updatedInput":<the tool's input plus answers and annotations>}`.»
4. `ops-architecture.md · contract/answer-refusals/unknown-dialog` · standard-contracts · fit: fits type 3 — Q1/W1.
   - old promise: «A late `answer` to a `question` first raised by PermissionRequest with no matching armed dialog is refused `unknown-dialog`.»
   - new promise: «A late `answer`, to a dialog already answered or after its hold expired, is refused `unknown-dialog`.»
5. `ops-main-test-plan.md · decision/dialog-answer-mapping` · unit-test-strategy · fit: fits type 1 (wave 2) — Q1/W1. (The proposer's set dropped «the tool's own» and was rejected.) The 300 bound turned «are answered by» / «by» into colons; no member or qualifying word dropped.
   - old rule: «Questions are answered by PreToolUse allow with updatedInput answers and annotations; permissions by PermissionRequest allow or deny with a message; a plan is approved only via PreToolUse allow with the tool's own input unchanged, and revised via PermissionRequest deny.»
   - new rule: «Questions: PreToolUse allow, or PermissionRequest allow where first raised there, with updatedInput answers and annotations; permissions: PermissionRequest allow or deny with a message; plans: approved only via PreToolUse allow with the tool's own input unchanged, revised via PermissionRequest deny.»
6. `ops-test-plan.md · contract/hook-decision-body/question` · fit: fits type 3 — Q1/W1.
   - old promise: «A question is answered with PreToolUse allow plus updatedInput.answers, annotations included.»
   - new promise: «A question is answered allow plus updatedInput.answers, annotations included: through PreToolUse, or through PermissionRequest when the question is first raised there.»
7. `ops-test-plan.md · contract/hook-decision/question` · fit: fits type 3 — Q1/W1.
   - old promise: «PreToolUse permissionDecision "allow" with updatedInput.answers, annotations included.»
   - new promise: «PreToolUse permissionDecision "allow" with updatedInput.answers, annotations included; a question first raised by PermissionRequest takes PermissionRequest decision.behavior "allow" with updatedInput, the tool input plus answers and annotations.»
8. `ops-security-plan.md · decision/live-rig-kept-stamp-owner-only` · data-protection · fit: fits type 1 — the cap (`PQ`'s kept stamp).
   - old rule: «The live rig keeps a stamp from its CS or WS scratch home owner-only and no-clobber, …»
   - new rule: «The live rig keeps a stamp from its CS, WS or PQ scratch home owner-only and no-clobber, …» (the rest unchanged).

No `except` was shortened by a cell bound; no member and no qualifying word of a standing row was dropped.
The two reached rows with no operation: security-plan `decision/capture-answers-closed-probe-ids` (names no count — the
sixth id changes no word) and test-plan `decision/ledger-probe-contract-suite` (its words hold, no version list).

## Named beside the card, not asked — rows outside the nine, not folded (founder-forks.md F1–F6)
rejected, each with its most conservative option: architecture `decision/permission-request-question-ledger-row` (add)
· architecture `contract/hook-to-kind-map/permissionrequest` (set) · architecture `decision/verify-never-types-into-dialog`
(set name) · security-plan `contract/wrapper-channel-methods/answer` (set) · design-system
`decision/permission-request-question-held` (add) · layout-templates `decision/permission-request-question-answerable`
(add) · a11y-plan `decision/critical-path-dialog-answer` (set).

## The answer
Given in the session's dialog by the overseer, delegate for the operator, read at 2026-11-20T14:13:47Z — chosen:
«Confirm all 8 (Recommended)»; its note, verbatim: «Overseer's delegate confirm, read at 2026-11-20T14:13:45Z against
answer-card.md and the founder's recorded answers: the 8 operations lie inside Q1 (W1, the nine rows), and line 8
inside the cap he gave with the kept stamp named in its shape. Rows read old beside new; the proposer's set that
dropped a qualifier stays rejected. F1-F6 stand unchanged for the epoch batch. PROVISIONAL until the founder's own
word..»
