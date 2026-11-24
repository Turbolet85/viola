# Adaptation record — the operator's release of v2-12 by hand (finding V257)

Written 2026-11-24T18:36:27Z. This is a HAND SEQUENCE on the operator's own word, NOT a skill run: no letter is
followed. The steps are those of the V257 plan's «Hand sequence on Viola tonight», steps 0–4, run by the builder
one step a turn, the overseer reading every read-back.

## Item 1 — the word
- word: «Освободить условие про ребут (Recommended)», the operator's answer to «Как разрешаем?», his dialog of
  2026-11-24T17:01:29Z, relayed verbatim by overseer1 — v2-12's reboot clause (R4b) is RELEASED from «`viola up`»
  (`2026-11-24-viola-up`, `chk-eq4ejvc6`) to «The fleet on the founder's host» (`chk-75xg3mgo`); «`viola up`»
  completes on R4a's green.
- snapshot: `.andromeda/cache/chunks/2026-11-24-viola-up/inputs/I22-relay-founder-answers-1124.md.txt` (line 68).
- the hand sequence itself stands on his second word, 2026-11-24T18:20:42Z, «V257 … Как действуем?» → «Руками
  сегодня и дверь следом (Recommended)»: the writes made by hand tonight by the builder, the door built after.

## Step 0 — backup
`~/.cache/viola-v257-backup-2026-11-24T18-36-18Z/`: `db/log/` (212 files), `master-route.md`,
`viola-0.2.0/working-route.md`, `viola-0.2.0/coverage.md`, `git-status-short.txt`; each read back byte-equal.

## Step 1 — the carries (tx 4873)
`carries.txt`: cry-q7hr3xn4 (CARRY: proves v2-12) and cry-l3a55nfc (BLOCKED-ON) placed on chk-75xg3mgo; cry-fqfwo7jx rev 1 → 2, its stale tail reworded. Read back: 7 pins (6 CARRY:, 1 BLOCKED-ON:), `abstained` 0 rows.

## Step 2 — the release
`release-v2-12.md` = text 3. `matrix.py release --standing`: v2-12 chunk 2026-11-24-viola-up → null, planned and gated kept, entry chk-75xg3mgo; notes +453 chars. Read back: `planned, unclaimed, gated` | null | null.

## Step 3 — the flip (tx 4875)
`matrix.py flip --dry-run`: released 1 to a follow-up v2-12 → chk-75xg3mgo, claimed 0. `route.py flip` with `desc.txt`: one BLOCKED-ON strip (cry-tzxxzdkd, 220ch, carried in step 1 as cry-l3a55nfc), gated → complete, compacted 1. Read back: `gated` 1 row, 2026-11-24-viola-adopt; chk-eq4ejvc6 complete. `rec.py check`: 1 failure, unlanded 2026-11-24-viola-adopt/evidence/r5.md (not this sequence's write).

## Before step 4 — the R5 record landed (tx 4876), on the overseer's word
`rec.py land --chunk 2026-11-24-viola-adopt`: 1 new (evidence/r5.md, the builder's own R5b record of an earlier window). `rec.py check`: clean.
