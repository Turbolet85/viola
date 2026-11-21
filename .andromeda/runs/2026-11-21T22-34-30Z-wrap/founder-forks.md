# Founder forks — the wrap of 2026-11-21-mcp-calls-bounded («MCP calls bounded»)

Written by the wrap run `.andromeda/runs/2026-11-21T22-34-30Z-wrap/` under the overseer's directive
(`chunks/2026-11-21-mcp-calls-bounded/inputs/I7-wrap-mcp-directive.md.txt`, items 3, 5 and 7). The operator is away:
nothing here was asked of him, and nothing here landed on his behalf. Each item is his to answer in the epoch batch.

## 1. A route entry for the paste's missing time bound — PROPOSED, NOT MINTED
The finding (this chunk's report, *Insufficient fixes*): the MCP call deadline bounds the driver's wait on `send`,
never the wrapper's paste. `crates/viola-pty/src/pump.rs:77` is one blocking `write_all` with no time bound; at the
deadline the wrapper's `send` goes on and the driver reads `call-deadline`, the outcome unknown (as with
`endpoint-lost`). A new route entry is the operator's own (`inputs#I2` item 9; the P5 card's *Beyond the plan* row 1,
`.andromeda/runs/2026-11-21T19-17-43Z-phase/p5-card-1.md`); the overseer's delegate confirm (`inputs#I3`) did not
answer it.

- **The entry's words (proposed):** «Paste write bounded — the wrapper's paste into the child's PTY ends within a
  compiled bound; a paste that cannot finish ends the send with a closed word, the wheel unmoved».
- **Its place (proposed):** Epoch 4, directly after «Key after a resize» (the P5 card's placement).
- **What it measures:** whether a paste into a child that stops reading its PTY blocks the wrapper's `send` past
  the readiness gate's and the confirm window's bounds (a fake agent that stops reading stdin, its PTY buffer full);
  the bound's value read against the send's own compiled total, so the MCP call deadline stays above it; what the
  driver reads at the bound (a closed word, never upstream text).
- **Options, in order:** (1) **Recommended** — mint it as proposed, the CARRY below folding into it; (2) mint it in
  Epoch 7 («Polish & ship») instead; (3) leave it a CARRY on its pin and decide at «Version done-check».
- **Meanwhile (the most conservative option):** no entry minted; the finding stands as a CARRY on the entry the
  overseer's route dialog named (this wrap's route record), so it cannot be passed by.

## 2. The PROVISIONAL rows of this wrap (the epoch batch)
The answer card's four operations (`answer-card.md`) landed PROVISIONAL on the overseer's delegate confirm
(2026-11-21T23:00:53Z), inside the operator's answers «Вшитый дедлайн 55 с (Recommended)» and «3 базовых + 1
запасной (Recommended)»: architecture decision/mcp-call-deadline, architecture contract/unreachable-cause/cause-set
(P4-F1), test-plan decision/time-faked-through-seams, security-plan decision/live-rig-mb-arm-keeps-no-output (P4-F2,
its row `state: provisional`). His own word ratifies or reverses each.

## 3. No cell bound shortened a list
No `except` cell or named list was shortened: the two a11y rows each gained «MCP call deadline» as one added member
within their bound; no row of this wrap is a fork for a cell that could not hold a member.
