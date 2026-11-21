# Answer card — 2026-11-21-mcp-calls-bounded wrap (Q1)

The operator is away (`inputs#I7`): the card goes to the overseer, whose confirm of rows lying inside the operator's
own answers is a delegate's — PROVISIONAL, superseded by his own word in the epoch batch (rul-bznresq5).

## The operator's own answers (read 2026-11-21T21:34:21Z, relayed verbatim by the overseer, `inputs#I2`)
1. Q1, the bound's form: «Вшитый дедлайн 55 с (Recommended)» — a compiled MCP-side deadline of 55 s on every frame
   tool, its expiry a new closed result word; no ledger row, no stamp moved (founder-forks §4 (A), the Tokio timer in
   viola-mcp shown in §7).
2. Q2, the measurement and the cap: «3 базовых + 1 запасной (Recommended)» — the rig's `MB` arm in the shape of §6 and
   §7 as shown to him, the cap, the cut start.
- P4-F1 option (1) and P4-F2 option (1): the overseer's delegate confirm inside answers 1 and 2 (`inputs#I3`),
  PROVISIONAL.

## The operations read as lying wholly inside those answers (4)
- ops-architecture.md · decision/mcp-call-deadline · area established-decisions · fit: fits type 1 — the compiled
  55 s deadline over send, wait, last, answer, its expiry instance-unreachable / call-deadline (answer 1)
- ops-main-architecture.md · contract/unreachable-cause/cause-set · area conventions · fit: fits type 3 —
  CallDeadline one named member, written by the MCP client alone, «written by both clients» kept for the six (P4-F1 in
  answer 1); with it (check 6's group): the member and detail rows security-plan
  contract/instance-unreachable-cause/call-deadline, test-plan contract/instance-unreachable-cause/call-deadline and
  contract/mcp-unreachable-detail/call-deadline, a11y-plan contract/instance-unreachable-detail/call-deadline
- ops-test-plan.md · decision/time-faked-through-seams · area mocking-stubbing-discipline · fit: fits type 1 — viola-mcp
  runs a Tokio timer in production, its tests fire the deadline through the injected seam, no paused clock (§7 (A) in
  answer 1)
- ops-main-security-plan.md · decision/live-rig-mb-arm-keeps-no-output · area data-protection · fit: fits type 1 — the
  `MB` arm's row of its own, the ratified shape, `state: provisional`, the long-call predicate (P4-F2 in answer 2)

Not on the card (fit `not_master`, none escalated): none.

## The answer
_(appended as given)_

«Confirm all four (Recommended)» — notes: «The overseer's delegate confirm, 2026-11-21T23:00:53Z: the four operations
read against answer-card.md and the operator's recorded answers of 2026-11-21; each lies inside them. PROVISIONAL, for
the epoch batch.»
