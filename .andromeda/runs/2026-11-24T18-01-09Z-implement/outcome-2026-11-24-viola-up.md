# Outcome — 2026-11-24-viola-up, gated implement (directive §B step 4)

**Run:** 2026-11-24T18-01-09Z-implement · **Outcome:** refused at Setup 2 — nothing to implement (the gated arm reads
the block STANDING). Not green, not stuck, not surfaced: no code, no gate fired, no ref written, no report authored.

## Inputs
- inputs#I21 — the overseer directive `gated-focus-up-directive.md` (§B step 4).
- inputs#I22 — `relay-founder-answers-1124.md`, the operator's own answers relayed verbatim by overseer1 (U6 arm 3;
  the 2026-11-24T17:01:50Z release of v2-12's reboot clause).

## What the letters make the gated implement do
- Setup 2, 0 pending → the gated arm: each `gated` row's premise is re-checked against the artifact its frozen
  `BLOCKED-ON` names. viola-up's block is ONE premise of two halves: «R4, viola up once on a real compositor over a
  planted fleet (R4a) and once after a real reboot over a real parent and child (R4b) …».
  - R4a: green — `chunks/2026-11-24-viola-up/evidence/r4.md`: entry 24 fired once 2026-11-24T17:05:59Z,
    `r4: green phase=windows order=parents-first placed=2/2 lines=matched`.
  - R4b: never fired; the operator released it (U6 arm 3, then 17:01:50Z) — the block reads STANDING.
  - viola-adopt's R5: STANDING (not this run's).
- No row CLEARED → the letter's words: «nothing to implement — gated 2026-11-24-viola-up, 2026-11-24-viola-adopt
  wait on R4 (R4b), R5; run /andromeda-phase». GATED MODE does not open: no `--void … block cleared`, no
  `--cleared-file`, no `matrix.py implement`. The tool would refuse it anyway — a gated cap's ref needs the
  clearance recorded, and a half is no clearance. v2-12 stays `planned, gated`, ref null — as the directive orders.
- The letters hold no partial-clearance arm: R4a green does not count as the block cleared.

## The door the letters give — and a finding for the overseer before §B step 5
- The release: route-resolve's «Unproved claim» row, at wrap P5, `matrix.py release … --standing`. The entry must
  carry `CARRY: proves v2-12 — …` and `BLOCKED-ON: {the cap's gated text}`, and the cap must read `planned` with no
  ref. That row's AUTO trigger EXCLUDES this case: «every gated cap of this chunk below verified reading STANDING
  with no `ref` is P7.3's `flip --gated`, never this row». So it runs only on the operator's word, which is the
  17:01:50Z answer (I22). This is the directive's own reading.
- **Wrap Setup 2 does not open the GATED WRAP for this record:** «0 pending and a `gated` row … whose gated caps all
  read `implemented` with a ref → its GATED WRAP … A `gated` record whose caps are not yet implemented is not
  wrapped here». v2-12 is viola-up's only gated cap and must stay ref-null for the release. So a plain
  `/andromeda-wrap-session` takes the 0-pending path (step 6). That path re-verifies premises (STANDING) and runs P5
  route-resolve only for an operator ROUTE-ADAPTATION request. Its P7 flips no record (`gated → complete` is P7's,
  in a chunk wrap).
- **Our recommendation:** the overseer names which door the wrap takes for §B step 5 before it fires. The two
  candidates are the 0-pending path carrying the release as an operator-requested adaptation, or the GATED WRAP
  entered on his word against Setup 2's implemented-with-ref condition. Not improvised here.

## Files changed
none (source, plan, matrix, route, masters untouched). Written: this file; the two snaps under
`.andromeda/cache/chunks/2026-11-24-viola-up/inputs/`.

## Deviations
none — §B step 4 done as ordered, and the run stopped there.

## Process hygiene
| process | started by | final state |
|---|---|---|
| — | none started | — |

No window, no key, no `claude`, no live script, no gate entry fired.

## Decisions & corrections
- The operator's release of v2-12's reboot clause (2026-11-24T17:01:50Z, relayed verbatim by overseer1, inputs#I22)
  is read here only as context for step 5. It lands at the wrap, not here.
