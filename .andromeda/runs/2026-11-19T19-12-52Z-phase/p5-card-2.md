# P5 card 2 — 2026-11-19-context-as-a-local-command

**Plan:** `doc:2026-11-19-context-as-a-local-command/plan` rev 5, rendered to
`.andromeda/cache/docs/2026-11-19-context-as-a-local-command/plan.md`. Card 1 (rev 4) stands, with the review's one
change folded in.

**The review's change** (the operator's word at the card-1 review):
- `--cap` is the running total of base starts in the ledger file. `CS` (entry 16) and `CS-2` (entry 22) take
  `--cap 1`; `CC` (entry 20) and `CC-2` (entry 23) keep `--cap 2`; every start takes `--spare 1`.
- Step 22 says so. It also says the host is read before EACH start: the host-read entry is re-fired by `--entry`
  right before `CC` and before a spare, QUIET fires, and CONTENDED with a neighbour named stops and is re-read.
- Nothing else changed.

**Checks re-run whole after the edit:**
- the dry-run parses (31 entries);
- planlint: 0 hits;
- the scope lists parse (25 items);
- `inputs.py verify`: 2 unchanged;
- the baseline listing: exit 0, the baselines on entries 3, 5 and 6 standing;
- validation-1: aligned.

**Unchanged from card 1:** the goal, the acceptance, the 25 touchpoints, the two widenings (F3, F5;
`rul-bznresq5`), no capability claimed.

## Beyond the plan
- (standing — .andromeda/runs/2026-11-19T19-12-52Z-phase/p5-card-1.md) The live home's 25-row re-stamp of `~/.viola`:
  owner the product re-pin operator step (no route entry), outside this chunk.
- (standing — .andromeda/runs/2026-11-19T19-12-52Z-phase/p5-card-1.md) C4 (cry-4olcx4il): acted on only at its next
  red during this chunk. With no red, the wrap re-pins it whole on the next route entry.

Apply? (yes / review / cancel)
