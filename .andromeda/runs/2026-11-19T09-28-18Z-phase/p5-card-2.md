# P5 review card 2 — «The 2.1.328 recorded fixture set» (chunk 2026-11-19-the-2-1-328-recorded-fixture-set)

**Plan:** `doc:2026-11-19-the-2-1-328-recorded-fixture-set/plan` rev 3 (tx 3027: 5 parts revised, nothing else).
Its render, `.andromeda/cache/docs/2026-11-19-the-2-1-328-recorded-fixture-set/plan.md`, opens in an editor; writing
there changes no record until a `put`, and no plan stands in the chunk's folder.
`python -X utf8 ~/.claude/skills/andromeda-tools/scripts/rec.py diff --root . 2026-11-19-the-2-1-328-recorded-fixture-set`
diffs the chunk's records. Everything else stands as on `.andromeda/runs/2026-11-19T09-28-18Z-phase/p5-card-1.md`.

**The review folded:** `inputs#I3` (the overseer's P5 review relay, the word `review` on card 1).

## The one change, on `L2` alone
- Entry 13's expect: `lacks context-transcript-after: absent` → `lacks context-transcript-after: unread`. It keeps
  `contains context-transcript-after:`. Its note says the same.
- Step 10 and the (C3) acceptance criterion now read: an entry cannot be both RED and passed over — a red live entry
  stops implement, always. So `context-transcript-after` is a READING of entry 13, not a failing atom of its expect;
  whichever word it reads, `read` or `absent`, is recorded as read in the evidence and the report. `absent` means the
  tag half stays unread on 2.1.328, and the two-send start is the wrap's carry proposal. Every other atom of `L2`
  stays a stopping red.
- §Implementation notes (C3, Order) follow it.

## Checks, re-run on rev 3
| check | reading |
|---|---|
| planlint | hits 0; 12 (d) lists the same 3 «unchanged» claims, each read true as on card 1 |
| dry-run | 25 entries, exit 0 |
| baseline | seen green 5 · not seen 0 · new (key) 7 · leg 13 — unchanged, entry 13 a live leg (exempt) |
| inputs | 3 unchanged (I3 new, cited 5 times) |
| research lists | `scope lists: parsed 4 items (new 1 · modify 3 · derived 0)` |
| Validation-1 | aligned |

No capability is claimed; no concretization preview. No live start was made.

## Beyond the plan
1. **(conditional) The primed two-send `CX` start**, on an `L2` reading `context-transcript-after: absent`: a carry
   proposal for the wrap's route adaptation, owner named there, its start a further cap of the founder's
   (`inputs#I2` item 2, `inputs#I3` item 1).
2. **(conditional) Listing `/context` as a local command**, only if `L2`'s reading supports it: a carry proposal for
   the wrap, owner named there (`inputs#I2` item 6).
3. **The pre-send gate reads a quoted modal literal as a standing modal.** A transcript row that quotes a modal
   literal refuses the driver's send (`not-delivered input-not-ready`, measured 2026-11-19T10:22Z). Not mended here;
   a carry proposal for the wrap, with `inputs#I3` (its item FIRST and item 2) as its evidence.

Apply? (yes / review / cancel)
