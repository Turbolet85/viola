# P5 card 1 — 2026-11-15-live-round-on-the-trimmed-product, the plan's sixth revision (rev 12 → rev 14)

Not prompted. The invoking line is the word if every check passes (inputs#I28, the operator), and every check passed.
The brief is the overseer's, written after reading entry 11's kept frames (inputs#I29). No row read, no research, no
live start.

## The changes (step 3f)
- **(a) CARRY 3 reads the screen's status row.** `present | absent`, its place below the input box's lower rule, and
  its sha256; never its text.
  - `hits` stays a named second column, never the verdict.
  - Unwrapped side: entry 11's second firing's kept frame, read by the new `live-read.py status-row` (entry 25, no
    start, the frame file's sha256 recorded).
  - Wrapped side: entry 12's `start` frame.
  - The two hashes are recorded and never required equal.
  - The security criterion is rewritten to this.
- **(b) `selected` reads unnumbered rows.** Entry 11's five summary lines are re-derived from its kept frame (no start)
  and rewritten in place, and entry 22 stays clean.
- **(c) `rehearse-trust-form`** (entry 24, no start): the measured form drawn by a stub, with four arms:
  - granted: `passed/1/1`, `selected` on the trust row;
  - stuck selector: no Enter;
  - another literal: no key;
  - wrapped: the wheel reads `human` after the keys and `driver` after the release.

  Green before entry 12's word. A (tests) criterion covers it.
- **Entry 11's naming:** «second firing, the first raised before any start (FileExistsError, make_cwd)», in its note
  and step 3e (a). Step 7 adds the re-entry line; entry 12's note names entry 24 and the status-row verdict.

/implement re-enters at step 3f. It re-fires entries 1–8 and 21–23, fires 24 and 25, and stops before entry 12.
Entry 12's word comes separately. No capability's acceptance moves.

## Mechanical checks (rev 14)
| check | result |
|---|---|
| `gate.py run --dry-run` | exit 0, 25 entries |
| `gate.py baseline` | exit 0 |
| entries 24 and 25 baseline | each `red · exit 1` (usage, not yet built), as predicted; recorded |
| `planlint.py check` | `hits: 0`. 12 (d)'s six claims each hold |
| `inputs.py verify` | 29 entries · drifted 0 · unparsed 0; I28 and I29 cited |
| `gate.py scope --lists-only` | parsed 3 items |

Validation-1: aligned.

## Beyond the plan
none
