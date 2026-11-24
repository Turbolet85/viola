# Outcome — 2026-11-24-viola-up (implement run 2026-11-24T06-34-55Z)

**Word:** green. Next: the operator pass in this window, WITHOUT any R4 entry (`inputs#I13`–`inputs#I15`).

## Files changed (this run, step 22)
- new `tests/support/terminal.rs`: `end_trees`. It SIGKILLs first only the leaves under a running parent, waits on
  every other member under one `WITHIN` deadline, SIGKILLs at that deadline as the last resort, then calls
  `gone_within`.
- `tests/support/mod.rs`: `pub mod terminal;`.
- `tests/cli_open.rs`: the `Terminal` guard now calls `end_trees`.
- `tests/cli_up.rs`: the `Terminals` guard now calls `end_trees`.

## Deviations
- The parent links are read from one `sysinfo` table beside `recorded_tree`, because `recorded_tree` returns no
  parent. A root, or a member whose parent cannot be read, is always waited on.
- The pre-push's corrupt profile was already kept under `target/profraw-census/pre-push-3061597/`. This run moved
  nothing (`evidence/pre-push-profile-kept.md`).
- The `old-kill` word reads `INDETERMINATE: case … passed`, not the forecast `… no result line`. It is indeterminate
  by construction either way (`evidence/old-kill.md`).
- Scope: `clean — changed 26 · listed 23 · recorded 3 (companion 2 · mechanical 0 · in-intent 1 · widening 0)`. This
  run wrote no new line.

## Gates
- Entries 1, 2 and 27–37 read green 13 · red 0. Entries 28–36 each read `refused 0` at 192 × 16, and the control,
  entry 37, also reads `refused 0`.
- `old-kill` on entry 30: in place `refused 0`, removed `refused 7` of 2227 (a rate, recorded).
- Entries 13, 14 and 15 (unit, integration, e2e) read green 3 · red 0 (15.9 s, 73.44 s, 11.91 s), after the
  overseer's quiet-window word.
- Matrix: v2-12 is GATED on R4 and gets no ref. Entry 17 stays deferred.
- No `gate.py guard` entry in the plan.

## Host
- 06:36:43Z: QUIET.
- 06:44:59Z: CONTENDED (escher alive 5 s of a 15 s stall), so this run stopped.
- 06:45:48Z (`--from 06:45:38Z`): QUIET.
- `df -m /tmp` read 31065 MB free each time.

## Process hygiene
| process | started by | final state |
|---|---|---|
| the census, layer and control test trees | this run (through gate.py) | terminated (gate tool census) |
| the control clone `~/.cache/andromeda-control/7301985d` | this run | removed by the tool |
| 12 host processes (`gate.py census`: overseer feet and their claudes, escher-builder's foot, wrapper and claude, `~/.viola` `viola mcp`, the product's and the prototype's `viola wait`) | NOT DERIVED, the operator's fleet | left running — the operator's |

## Decisions & corrections
- The CONTENDED host stopped the whole layers (directive `inputs#I13`). The overseer's word `inputs#I15` resumed
  them.
- bash-guard refused a `cd` outside the project twice. Each was re-issued once with absolute paths.
