# P5 card 1 — 2026-11-15-live-round-on-the-trimmed-product, the plan's ninth revision (rev 18 → rev 19)

Not prompted. The invoking line is the word if every check passes (inputs#I44, the operator). Every check passed.
The authority is the founder's second one-time override, «Ещё один обход на одну строку (Recommended)» (12:29Z,
inputs#I44); its binding is inputs#I45. The plan's render is at
`.andromeda/cache/docs/2026-11-15-live-round-on-the-trimmed-product/plan.md`.

## The one line
Step 3's environment line now reads: «no `CLAUDE*`, no `VIOLA*` — save in entries 26 and 27 alone».
- Their wrapped start adds ONE dummy probe, `CLAUDE_CODE_REHEARSAL_PROBE`, with a fixed dummy value.
- The probe's NAME stands in the trail; its value never does.
- Every other entry keeps the old line.

## What it forces
- **Research's Conventions line** (Rig environment) follows it.
- **Step 3g (b)(2)** adds a clause: the probe's name is among the strip fields, and its value is nowhere in the run log
  (`probe_name_logged`, `probe_value_logged: false`).
- **«Revised (9)» metadata line:** added.
- **One Test Commands sentence:** entry 26's first green (run `2026-11-15T12-19-22-implement`) stood on a deviation,
  because its probe was set against the old line. Entry 26 is re-fired on the revised line, then entry 8, before entry
  27's word.

Nothing else moves. No entry, criterion or count changes, and v2-03's acceptance wording stands.

## Mechanical checks (rev 19)
| check | result |
|---|---|
| `gate.py run --dry-run` | exit 0, 30 entries |
| `gate.py baseline` | exit 0 |
| `planlint.py check` | `hits: 0` |
| `inputs.py verify` | 45 entries · drifted 0 · unparsed 0; I44 and I45 cited |
| `gate.py scope --lists-only` | parsed 3 items |

Validation-1: aligned. The change makes entry 27's strip-field reading able to fail; it adds no surface.

## Beyond the plan
| # | proposal | kind | where |
|---|---|---|---|
| 1 | A CARRY on a later live entry (the founder's own word, inputs#I33): «a wrapped start in a cwd trusted beforehand, so no trust key is typed — lines 1–2 and the `x`'s own wheel line read». matrix#v2-03 is un-claimed at this chunk's wrap with a dated note naming the word, and the CARRY owns its reading (standing — .andromeda/runs/2026-11-15T11-17-37-phase/p5-card-1.md) | a new CARRY, v2-03 un-claimed | the wrap's route-resolve and its ledger write |
| 2 | Row 1 now holds ONLY if entry 27 reads red. On a green entry 27, row 1 ends: no CARRY, no un-claim, and v2-03's ref is written (inputs#I41 item 6) (standing — .andromeda/runs/2026-11-15T12-10-18-phase/p5-card-1.md) | a condition on a standing row | the wrap's route-resolve and its ledger write |
| 3 | After entry 30, the halted wrap resumes in its own run dir `.andromeda/runs/2026-11-15T11-38-30-wrap/`, its report amended for entry 27, and the plain gate flips v2-02 and v2-03 (inputs#I40 (d)) (standing — .andromeda/runs/2026-11-15T12-10-18-phase/p5-card-1.md) | a resume of the halted wrap | `/andromeda-wrap-session` |
| 4 | The report names entry 26's first green (run `2026-11-15T12-19-22-implement`) as resting on a deviation: its probe was set against step 3's old environment line (inputs#I45) | a report line | the wrap's report |
