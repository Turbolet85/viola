# Implement outcome — 2026-11-15-live-round-on-the-trimmed-product (run 2026-11-15T10-00-02)

**Outcome: stopped on the operator's line before entry 12. Every gate fired reads green (`green 13 · red 0`), and no
start was made: starts stay 8 of 9.** The chunk is not complete. Entry 12's word, entry 13 onward and the operator
pass remain. No capability ref.

Setup's approval line: `2026-11-15T09-56-08-phase/matrix-2026-11-15-live-round-on-the-trimmed-product.json` (plan rev
14). The operator's word for this run is snapped as inputs#I30.

## Step 3f — built
- **`live-read.py`:**
  - `option_block` and `selected` read numbered or unnumbered option rows;
  - `status_row` and `status-row <frame file>` give the status row in closed words;
  - `resummarise <frame file>` re-derives one frame file's summary lines;
  - `statusline(…, status=…)` makes a live verdict on the status row, `hits` a column.
- **`live-hold.py`:** the live `start` frame's status row feeds the verdict, and `rehearse-trust-form` drives a drawn
  stub of the measured form.
- **Entry 11's summary lines re-derived** (no start): `resummarise: 5 lines · 2 changed`. `trust-seen` reads `selected`
  0 and `trust-selected` reads 1.
- **The one-shot readings** (`evidence/rehearsal.md` § Step 3f):
  - `status-row` reads `present` exit 0 over a row below the lower rule;
  - `absent` exit 1 with nothing below, and `absent` exit 1 with only a footer row below;
  - `unread — frame absent` exit 1 over an absent file;
  - `selected` reads 0 and 1 on unnumbered rows, 0 on numbered ones, and `None` on an input box's prompt row alone.

## The gates — `green 13 · red 0`
`1 · 2 · 3 · 4 · 5 · 6 · 7 · 8 · 21 · 22 · 23 · 24 · 25` all green. Entry 22 reads
`screens-check: clean — 65 lines · 65 frames local · 0 deleted`.

### Entry 24, each arm as the rig printed it
`rehearse-trust-form: green — granted_trust=passed/1/1 · granted_keys=down+enter · granted_selected=0->1 · granted_after=input-box · granted_screens=frames=4,clean=True,summary=4,closed=True · stuck_trust=modal-unpassed/2/0 · stuck_keys=down+down · stuck_screens=frames=2,clean=True,summary=2,closed=True · other_trust=no-dialog · other_keys=0 · other_found=unknown-modal · other_screens=frames=1,clean=True,summary=1,closed=True · wrapped_trust=passed/1/1 · wrapped_wheel_human=True · wrapped_release=0 · wrapped_wheel_driver=True · wrapped_screens=frames=4,clean=True,summary=4,closed=True · census_held=0`
- **granted** `passed/1/1`: the keys were exactly `down+enter`, `selected` went 0 → 1, and an input box came after.
- **stuck selector** `modal-unpassed/2/0`: the keys were `down+down` and no Enter was typed.
- **other literal** `no-dialog`: 0 keys, the frame read `unknown-modal`.
- **wrapped** `passed/1/1` through `viola run`: the wheel read `human` after the keys and `driver` after `viola release`
  (exit 0). No send was made in this arm; the release alone moves the wheel back.

### Entry 25, as the rig printed it
`status-row: present — below-rule 1 · sha256 70f400cec092bb6793d1688fce4438842f09ed8e31dafbe83cbc7b7491123b58 · frame start · file sha256 cf964762441a9711f5acf7f621c047530145a539b9efc0ff7614bb0b79f815d9`
CARRY 3's unwrapped side reads `present`, one row below the input box's lower rule; the wrapped side waits on entry 12.

## Files changed (work dir only)
- `evidence/live-read.py`, `evidence/live-hold.py` — step 3f.
- `evidence/screens-summary.ndjson` — entry 11's 2 lines re-derived, 20 rehearsal lines added.
- `evidence/rehearsal.md` — § Step 3f.
- `evidence/live-round.md` — § Entry 11's second firing, read again.
- `target/live-round-screens/` — the rehearsals' local frames.

## Deviations
- None from the plan's steps. Scope read: `scope: clean — changed 0 · listed 0 · recorded 0`.
- **The wrapped arm:** it reads the wheel after the keys and after the release. It makes no send: the stub has no
  hooks, so a send would read nothing about the trust step.
- **`stuck_trust`:** it reads `modal-unpassed` (the rig's own word for a dialog not passed within its arrows). The
  closed word on the frames for the other-literal arm is `unknown-modal`, as the brief named it.

## Matrix
No ref. v2-02 and v2-03 stay gated, and entry 12 is unfired.

## Process hygiene
| process | started by | final state |
|---|---|---|
| entries 4–7, 23 and 24's rehearsal sessions (fake agent, the drawn stub, the wrappers) | this run (gate entries) | terminated — `census_held=0` in each closing line |
| `python3` hostwatch recorder and an input-method indicator (`gate.py census --comm python3`) | not this run (NOT DERIVED) | left running — not this run's |

## Decisions & corrections
- **The operator's word:** step 3f and entries 1–8 and 21–25 with no start, then stop. Entry 12 has no word
  (inputs#I30).
- **The frame's row shapes** were read as booleans and codepoints only, to place the status row and the option
  column; no row text was printed or kept.
- **A recurrence of the standing learning** «a timestamp … is read from `date -u`»: the fix-loop and smoke evolve
  records were stamped `10:05:00Z`, typed in the same call that printed `date -u` as `10:05:15Z`. It landed, since it is
  past and after the bound, and it was not re-stamped.
