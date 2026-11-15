# P5 card 1 — 2026-11-15-live-round-on-the-trimmed-product, the plan's fifth revision (rev 10 → rev 12)

Not prompted. The invoking line is the word if every check passes (inputs#I25, the operator), and every check passed.
The ruling is the delegate overseer's, technical (inputs#I26). No row read, no research, no live start.

## The three changes (step 3e)
- (a) A firing that raised before the ledger's `start` row and spawned nothing spent no start. It stays red as fired,
  and its entry is fired once more, named as its second firing. Entry 11's first firing is that case. Step 7, the
  Constraints and entry 11's note follow.
- (b) Each live start makes its own fresh `proj-{stamp}/` and leaves what an earlier start left as found (step 3's cwd
  line and a Constraints line).
- (c) A new rehearsal, `live-hold.py rehearse-reuse`, gate entry 23 (`new`): a second wrapped and unwrapped live-form
  start on a fake home holding a `proj/` and a SIGKILLed instance `round`, green before any live word. Step 3d now
  points its readiness half here. A (tests) criterion covers it.

/implement re-enters at step 3e. It re-fires entries 1–8, 21 and 22, fires entry 23, and stops before entry 11's
second firing, which waits on its own word. No capability's acceptance moves: v2-02 and v2-03 stand as claimed.

## Mechanical checks (rev 12)
| check | result |
|---|---|
| `gate.py run --dry-run` | exit 0, 23 entries |
| `gate.py baseline` | `seen green 1 · not seen 0 · new 15 · leg 7` |
| entry 23 baseline | `red · exit 1`: `live-hold.py` prints its usage, as predicted; recorded |
| `planlint.py check` | `hits: 0`. 12 (d) lists 6 claims, each holding; line 302's «unchanged» (entry 12) is true, since entry 12's lines did not move |
| `inputs.py verify` | 26 entries · drifted 0 · unparsed 0; I25 and I26 snapped and cited |
| `gate.py scope --lists-only` | parsed 3 items |

Validation-1: aligned with the intent.

## Beyond the plan
none
