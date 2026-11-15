# Implement outcome — 2026-11-15-live-round-on-the-trimmed-product (run 2026-11-15T09-00-58)

**Outcome: stuck — the old box stop rule stands in a plan line, so the phase door comes before entry 10. Entry 10
was NOT fired.** No live start in this run: starts stay 6 of 9.

Setup's approval line, quoted: `2026-11-15T08-48-39-phase/matrix-2026-11-15-live-round-on-the-trimmed-product.json · open · 5048 B`
(plan rev 7). The word for this run is snapped as inputs#I15.

## Done (plan rev 7's re-entry)
- **`stamp-check` built** (`live-hold.py stamp-check <base>`, step 3b): the kept stamp read, no start and no write.
  Its readings before first graded use (`evidence/rehearsal.md`):
  - kept home → `stamp-check: pass — stamped 2.1.325 · 21 of 21 relied pass · stamps sha256 86f763d8f43f5281 · start rows 5 · latest 2.1.325`
  - one relied row `fail` → `stamp-check: red — relied row local-command-compact reads fail`
  - absent base → `stamp-check: unread — kept home absent`
- **Entries 1–8 and 21**, fired twice in this run: `green 9 · red 0` both times. Entry 21 is green on the kept
  stamp.

## Why entry 10 did not fire
Before entry 10 I raised a technical fork: the rig read the `x` echo and the DEL erase from an ANSI-stripped stream,
which would misread a cell-diff redraw as «stands» and, by the rule, end the step and lose the hold. The overseer
answered (inputs#I16):
- **Add a minimal VT screen model** — done, controlled and rehearsed (below).
- **Correct the rule:** a box reading «stands» no longer ends the step. The rig records the reading, types no further
  key, releases, and sends; the send is the second witness. Done, and rehearsed with the true «stands» under the fake.
- **If the stop rule stands in a PLAN line, say which and stop for the phase door before firing.**

It does. Plan rev 7, `## Implementation notes`, lines 474–476 (its render):
«`rehearse-wrapped` reads the keys after the hold, and `live-wrapped` checks the DEL erase from the drained bytes
before `release` and the send, ending the step on a standing `x` (inputs#I4)».

As corrected, the step no longer ends on a standing `x`, and the erase is read from the screen model, not the
drained bytes. So that line moves. Step 3's (d) («read back as the `x` in the drained bytes within 2 s …») reads
the echo from the drained bytes too; with the model it is read from the screen those bytes build. I'd move it with
the note.

## The screen model and the corrected arm (inputs#I16)
- `Screen` (in `evidence/live-hold.py`) at 210×45: printable cells, CR/LF/BS/TAB, CSI cursor moves, erases, inserts
  and deletes; SGR, modes and OSC ignored. The box row is the last row opening `❯` under a box rule.
- **Controls** (`screen_controls()` → `True`):
  - whole-row rewrite → echo `x`, then `erased`
  - one-cell diff → echo `x`, then `erased`
  - placeholder after the erase → `erased`
  - no redraw → `stands`
- **Rehearsed** (gate entry 5, firing 2): `keys=echo=True,human=True,box=stands,release=0,driver=True,keys_after_del=0`
  · `stands_arm_send=13/x-stood` · `screen_model=True`.
- **Live green, as the rig now reads it:** the box either way, `keys_after_del=0`, release and driver, and
  `box_witness=empty` (a confirmed send). A `not-delivered` send reads `x-stood`: the hold is lost and the step is red.

## Files changed (work dir only)
- `evidence/live-hold.py` — `stamp-check`; `derive(…, state_path=None, facts)`; the `Screen` model,
  `screen_controls()`, `box_reading`, `send_witness`; `input_box` without the stop; the wrapped flow's (d), the send
  witness, and the rehearsal arms.
- `evidence/rehearsal.md` — step 3b's readings, and the screen model and corrected arm.

## Deviations, with justifications
1. **The box read moves from the stripped stream to a screen model**, and «stands» no longer ends the step
   (inputs#I16, the overseer's technical answer). This corrects the earlier deviation of inputs#I4. It waits on the
   plan line above for the phase door.
2. **`stamp-check`'s must-fail control copy** was made under `target/e2e-home/viola-live-control-*` (the subcommand
   accepts only that base form) and removed by the script that made it.

## Matrix
No ref. v2-02 and v2-03 stay gated and unread; entry 10 is unfired.

## Process hygiene
| process | started by | final state |
|---|---|---|
| entries 4–7's rehearsal wrappers, fake agents, verify and tap (both firings) | this run | terminated — `census_held=0` in each closing line |
| `stamp-check`'s three readings and entry 21 | this run | terminated — no process of their own beyond the read |
| no live `claude` | — | none started |

## Decisions & corrections
- The overseer's technical answer (inputs#I16): a screen model; «stands» no longer ends the step; the send is the second
  witness; stop for the phase door if the rule stands in a plan line. It does, in Implementation notes lines 474–476.
- A recurrence of the standing learning «a timestamp … is read from `date -u`»: an evolve record's `ts` was typed ahead
  of the clock by about three minutes. `evolve.py` refused it (`REFUSED ts:`), nothing landed, and it was re-stamped
  from `date -u` (2026-11-15T09:08:05Z).
