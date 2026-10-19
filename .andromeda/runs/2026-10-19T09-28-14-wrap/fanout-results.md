# Fan-out results — 2026-10-19-dialog-hold-for-the-driver

Batch: one parallel batch of seven `general-purpose` doc-agents. Snapshots: before `tree ad86465cf5b71e8ae711048e4f9a29ca0c96fc44
index 5eb3692473acb5b4fe2eec4503382025be94bb40 head d146c50c6fe2f58164ceece52ecc091773450f30`, after `tree
0827130368489d5d9cb608e41a3e8dc7bf7fe286`, index and head unchanged. `git diff-tree` printed exactly the seven `A
.andromeda/runs/2026-10-19T09-28-14-wrap/fanout-{doc}.md` lines. No file needed a fix (no twin): each split at its
`- detector:` items, every detector one of its doc's or `D-retired`, no entity. Receipts matched the files.

Rejected for a source the report does not carry: 0 (every `basis` coordinate is a report coordinate or a row of its
`## New text, by line`).

Validate's checks: 1 playbook · 2 cross-contradiction (none: the obs-plan §4 Dialog proposals 1–3 touch one scenario
in one direction) · 3 intent (Deviations 1–4 justified; Deviation 4 on the operator's word `inputs#I4`; scope record
none) · 4 absence (the arch proposal 13 site was found by the detector's own read of its master, at a line the
report's `wake5` pattern missed) · 5 expected amendments (three raised by main below; obs-plan §6 and §1 read, no
site) · 6 disproved claims (`obs-plan.md:615` matched by obs-plan 1).

Escalated = the playbook rule «Boundary widening the founder answered himself to a P4 fork» (the `config.json` key
`dialog_hold_s`, a validated surface admitting a new input; the founder's own answer `inputs#I2` Q1) → the answer card
`answer-card.md`. Routine = «Accurate this-chunk addition».

architecture: 14 proposals · fanout-architecture.md
  note: D-arch-decisions found no drift; the §Stack Domain newtypes row and [Validation] name only `ViolaName` and
  `Percent` as nutype uses (raised as items 15–16).
  1. escalate (check 1, widening, on the card) — [Hook Contract] the hold (`dialog_hold_s`, the three homes, `hold_for`), the hook bound 3605 s, timeout 3615, `null` + one `dialog-expired` + `dialog-answered{deadline_hit:true}` at the hold's end, the 671 s live reading
  2. apply (check 1, routine) — [Human Takeover / Wheel] "dialog deadline" → "dialog hold" ×2 (dependent of 1)
  3. apply (check 1, routine) — §Occupied Resources registered hook events: `timeout` 75 → 3615
  4. apply (check 1, routine) — [Deployment / Distribution] `"timeout": 75` → 3615 (dependent of 3)
  5. escalate (check 1, widening, on the card) — §Occupied Resources `config.json` + `dialog_hold_s`
  6. escalate (check 1, widening, on the card) — §Occupied Resources `schemas/config.v1.json` + `dialog_hold_s`; `CONFIG_KEYS` five → six
  7. escalate (check 1, widening, on the card) — §Cross-cutting Patterns Config management Settings + `dialog_hold_s`
  8. apply (check 1, routine) — §Conventions normalised kinds + `dialog-expired`
  9. apply (check 1, routine) — §Standard Contracts Event `data` + `dialog-expired` `{dialog_id, dialog_kind}`
  10. apply (check 1, routine) — §Standard Contracts `wait`: the wake set six
  11. apply (check 1, routine) — §Standard Contracts `wait`: "Only hook lines signal" gains the wrapper's `dialog-expired`
  12. apply (check 1, routine) — §Standard Contracts wrapper-appended records gain `dialog-expired`, the one that wakes `wait`
  13. apply (check 1, routine) — [Message Broker / IPC] the driver-relevant events + `dialog-expired`
  14. apply (check 1, routine) — §Standard Contracts snapshot `pending_dialog` written before the dialog's line, cleared on a failed append
  15. apply (raised by main, check 5 floor, routine) — §Stack Domain newtypes row: + `DialogHold` (60–3600 s)
  16. apply (raised by main, routine) — [Validation] "nutype newtypes `ViolaName` and `Percent`" + `DialogHold`
security-plan: 2 proposals · fanout-security-plan.md
  note: D-security-auth and D-security-deps no drift; the Threat Model's `config.json` line was never an exhaustive key list.
  1. escalate (check 1, widening, on the card) — §Input Validation Configuration values row + `dialog_hold_s`
  2. apply (check 1, routine) — §Input Validation Hook stdin row: the reply bound `DIALOG_HOLD_MAX + 5 s` (3605 s)
design-system: 0 proposals · fanout-design-system.md
  note: D-design-tokens and D-retired clean; §Component Patterns 3 does not say `dialog-expired` takes the dialog form;
  §Color Palette's Domain status colors table has no `dialog-expired` row (a 0.2.0 frontend input).
  1. apply (raised by main, check 5 — the plan's expected amendment, P5-approved, names the change; routine) — §Surface: cli §Component Patterns 3: `dialog-expired  builder  dialog 7  cursor 49400` takes the dialog form, the planning form with the founder's wording owed; "every other non-dialog kind" excepts it
  — the Domain status colors row: no change (a 0.2.0 frontend colour is a design decision this chunk did not make; no detector covers it; carried to the founder's Epoch 5 batch with the wording, `inputs#I3` row 3)
layout-templates: 2 proposals · fanout-layout-templates.md
  note: no D-retired site; the wireframe's cursor figure is illustrative.
  1. apply (check 1, routine) — §Surface: cli Output structure `viola wait`: the `dialog-expired` line (planning form, the founder's wording owed)
  2. apply (check 1, routine) — the same wireframe's turn-ended-form annotation excepts `dialog-expired` (dependent of 1)
test-plan: 7 proposals · fanout-test-plan.md
  note: D-tests-obs-harness no drift; no D-retired site; `:571`, `:596` no-change rows.
  1. apply (check 1, routine) — §4 root bin channel `wait`: the wake set six
  2. apply (check 1, routine) — §1 Path 3: the wake set six (dependent of 1)
  3. apply (check 1, routine) — §5 On-disk: 16 kinds
  4. escalate (check 1, widening, on the card) — §5 CLI controls table: the `dialog_hold_s` row, `SETTINGS` ten
  5. apply (check 1, routine) — §4 root bin: the dialog-hold unit cases
  6. apply (check 1, routine) — §4 viola-core: `DialogHold` bounds, `ALL` 16 / `WAIT_WAKE` 6
  7. apply (check 1, routine; re-derived without the proposal's policy clause) — §4 Test grouping: `viola-core` (no rstest dev-dependency) writes table-driven cases as literal tables, beside `viola-e2e`
obs-plan: 8 proposals · fanout-obs-plan.md
  note: D-obs-stack and D-obs-pii no drift; no D-retired site; `:758` and the §6 rows no-change; apply the §4 Dialog proposals together.
  1. apply (check 1, routine; check 6 — disposes the report's disproved claim at `obs-plan.md:615`) — §4 Dialog scenario Cleanup: the expiry is the wrapper's
  2. apply (check 1, routine) — §4 Dialog scenario Required log fields: no `from` / `from_trust` at expiry
  3. apply (check 1, routine) — §4 Dialog scenario span attributes: `deadline_ms` the configured hold
  4. apply (check 1, routine) — §4 `wait` scenario log fields: the `outcome` wake set six
  5. apply (check 1, routine) — §4 `wait` scenario span attributes: `woken_kind` + `dialog-expired` (dependent of 4)
  6. escalate (check 1, widening, on the card) — §3 → OTel SDK init step 6: `run`'s start reads `dialog_hold_s`, one `parse-rejected` on a bad value
  7. escalate (check 1, widening, on the card) — §3 → OTel SDK init step 6: `CONFIG_KEYS` six names (dependent of 6)
  8. escalate (check 1, widening, on the card) — §3 → OTel SDK init step 5: the start reads `config.json` a third time (dependent of 6)
  — expected amendment «§6 catalogs the product kind, and §1's copy moves with §4»: no site (§6, lines 717–797, holds no product-kind catalog, its `dialog-answered` row already names `deadline_hit`, `from`, `from_trust`; §1 restates no wake-kind list and no dialog deadline)
a11y-plan: 1 proposal · fanout-a11y-plan.md
  note: D-a11y-surface and D-a11y-obs-schema no drift.
  1. escalate (check 1, widening, on the card) — §8 Timeout extensions: the configurable hold, the timeout 3615, the hook bound 3605, `dialog-expired` at the hold's end, any human key still hands back

Totals: 34 proposals + 3 raised = 37 · escalated 11 (all on the answer card) · routine 26 · rejected 0.
