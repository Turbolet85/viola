# P5 card 1 — 2026-11-15-live-round-on-the-trimmed-product, the plan's seventh revision (rev 14 → rev 15)

Not prompted. The invoking line is the word if every check passes (inputs#I34, the operator), and every check passed.
It rests on the founder's own words after entry 12 (inputs#I33). No row read, no research.

## The four lines
1. **The Goal:** the chunk completes on v2-02 alone. The wrapped start (v2-03) is read, NOT verified this round, and
   un-claimed at the wrap with a dated note and a CARRY.
2. **The v2-03 criterion:** it leaves. This chunk does not prove v2-03, on the founder's own word «Не засчитывать сейчас
   (Recommended)», and no ref is written. Two of its clauses cannot read on a start that types the trust keys. The line
   still cites `matrix#v2-03`, so the claimed id stays cited until the wrap un-claims it.
3. **The entry-12-red criterion:**
   - entry 12 stands red as fired on v2-03's two clauses alone;
   - v2-02 rests on entry 12's hold reading and the founder's `hold: stands`, and its ref is written with his words as
     the clearance;
   - CARRY 3's wrapped side was read.
4. **The new CARRY:** below.

Also: a «Revised (7)» metadata line, and one premise-corrected scope bullet (inputs#I33). /implement re-enters at step
9 (`hold: stands`: `DIALOG_HOLD_MAX`'s doc comment), then entries 13–20. No capability's acceptance moves here.

## Mechanical checks (rev 15)
| check | result |
|---|---|
| `gate.py run --dry-run` | exit 0, 25 entries |
| `gate.py baseline` | exit 0 |
| `planlint.py check` | `hits: 0`; 12 (d)'s six claims each hold |
| `inputs.py verify` | 34 entries · drifted 0 · unparsed 0; I33 and I34 cited |
| `gate.py scope --lists-only` | parsed 3 items |
| `matrix.py audit` | v2-02 and v2-03 cited in `## Acceptance Criteria` |

Validation-1: aligned with the intent's hold reading. The wrapped-start half is carried on the founder's word.

## Beyond the plan
| # | proposal | kind | where |
|---|---|---|---|
| 1 | A CARRY on a later live entry (the founder's own word, inputs#I33): «a wrapped start in a cwd trusted beforehand, so no trust key is typed — lines 1–2 and the `x`'s own wheel line read». matrix#v2-03 is un-claimed at this chunk's wrap with a dated note naming the word, and the CARRY owns its reading | a new CARRY, v2-03 un-claimed | the wrap's route-resolve and its ledger write |
