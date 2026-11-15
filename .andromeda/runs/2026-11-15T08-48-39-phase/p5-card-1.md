# P5 review card 1 — 2026-11-15-live-round-on-the-trimmed-product (REVISION 2)

**Mode:** second revision of a pending chunk's plan (Setup 3 offer b), on the operator's word by the delegate overseer
(`relay-1.md`, inputs#I10), against the overseer's brief (inputs#I11, read whole). Run dir
`.andromeda/runs/2026-11-15T08-48-39-phase/`.
**Cause:** /implement (`2026-11-15T08-33-57-implement`) fired entry 9 once. **Entry 9 stands RED as fired:**
- its STAMP read `stamped 2.1.325  21 pass  0 fail`;
- its RECORDING was refused whole (`SessionStart.default.json absolute-path`), because the rig's test-home dir is
  named `home`.

It is never re-fired and never re-labelled. The state save is fixed and `round-state.json` is derived by a rig read
(inputs#I9).
**Not run, by the word:** the row read and research (`research` stands as written), and any live start.

Open the plan in an editor: `.andromeda/cache/docs/2026-11-15-live-round-on-the-trimmed-product/plan.md` (rev 6). No
plan stands in the chunk's folder; writing in that file changes no record until a `put`.
`python -X utf8 {tools_dir}/rec.py diff --root . 2026-11-15-live-round-on-the-trimmed-product` diffs the chunk's records.

## What changed (plan rev 4 → 6; scope rev 4 → 5)
- **New gate entry 21 — no start, no write.** Run:
  `python3 .andromeda/cache/chunks/2026-11-15-live-round-on-the-trimmed-product/evidence/live-hold.py stamp-check target/e2e-home/viola-live-20261115T083805Z`
  (`role = 'probe'`, `expect = ['exit 0', 'contains stamp-check: pass — stamped 2.1.325']`). It reads the kept home's
  `ledger/stamps.json` and passes only on `stamped 2.1.325` with every `LedgerRow::relied()` row `pass`, naming the
  file's sha256 and the run's `start` rows. It is listed last, so no entry number moves.
  - **Its baseline:** `red — exit 1: stamp-check not yet built, live-hold.py prints its usage; the red the plan
    predicts …`.
  - **Its red control (P5)** fired the rig's own stamp reading (`derive`) on three inputs, writing only to the
    scratchpad, `evidence/round-state.json` unchanged before and after:
    - `a · the kept home: pass · stamp pass · 21 rows · 86f763d8f43f5281`
    - `b · one relied row fail: refused — relied row shim-resolution reads fail` (state written False)
    - `c · an absent base: refused — FileNotFoundError` (state written False)
  - `stamp-check`'s own must-pass and must-fail readings are /implement's one-shot run before entry 21's first graded
    use: pass · red · unread.
  - **If the tmpfs home is gone (a reboot):** entry 21 reads `stamp-check: unread — kept home absent`, exit 1, and
    v2-02 and v2-03 are then unread.
- **The kept stamp, copied now** to `evidence/stamps-2.1.325.json`, byte for byte (`cmp` exit 0), sha256
  `86f763d8f43f5281ef31b7b7fef82f1e663ac78b7db42fe48b1f2f0c65c256ff`, so the reading the wrap cites is a landed file.
- **Step 3b (new)** names what is done (the state-save fix, `derive-state`, `rehearse-derive`) and what to build
  (`stamp-check`). Step 7's re-entry: /implement re-enters at `stamp-check`, takes its readings, re-fires entries 1–8,
  fires entry 21, and stops before entry 10.
- **Constraints:** entry 9 is never re-read, re-fired or re-labelled green. `round-state.json` and the stamp copy are
  never written by hand.

## Your four lines (inputs#I11 item 4)
1. **The CARRY 2 criterion** — before: «entry 9's recording read by `live-read.py box` and recorded as read
   (`box-only: yes|no` per phase) in `evidence/live-round.md` …». After: «entry 9's recording was refused whole
   (`SessionStart.default.json absolute-path`, the rig's `home` directory), so no `Screen.*.json` exists and
   `box-only` is UNREAD, stated so in `evidence/live-round.md` with `box_only_guards: 3` named as verify's own
   measurement, not a box reading; the recording is spent for this round under the no-re-fire rule, and CARRY 2 is
   carried onto «The 2.1.325 recorded fixture set», whose own recording it will be».
2. **Step 8:** «the `Screen.*.json` of entry 9's `--record`» leaves. Only entry 12's `Mcp.tools-list-1.json` can stay
   as evidence. No `Screen.*.json` exists, and CARRY 2 is carried.
3. **The `latest` criterion** — before: «every live `start` row and closing line of entries 9–12 records `latest` as
   2.1.325». After: «every live `start` row of entries 9–12 records `latest` as 2.1.325, and so does the closing line
   of entries 10–12; entry 9 printed no closing line (its rig raised in `save_state` after its verdict), so for it the
   `reading` row of `live-sessions.ndjson` (`latest: 2.1.325`) records it beside its `start` row».
4. **Step 3:** `live-verify`'s state is a PRECONDITION file, never a verdict. It holds the base and home as spelled
   (never resolved through the link), with the stamp (`pass|fail`), the recording (`written|refused`) and the
   entry's own verdict apart. For any FUTURE recording the test-home directory is named without the word `home`;
   nothing is re-recorded in this chunk. `derive-state` is named in step 3b.

## «Entry 9 green» as a condition — before and after (held for the founder's word: it moves what «verified» rests on)
Every sentence that made entry 9's green a condition of v2-02 or v2-03, plan and ledger alike:

1. **Step 7** (plan)
   - before: «(10 and 11 need 9's green stamp; 12 needs none)»
   - after: «(10 and 11 need a passing stamp for the round's CLI on entry 9's kept home — read by entry 21, and
     carried to them by `round-state.json`, a precondition file — not entry 9's verdict, which stands red as fired;
     12 needs none)»
2. **Entry 9's `note`** (plan, gate fence)
   - before: «… a red here leaves v2-02 and v2-03 unread; the recording stays evidence; …»
   - after: «… fired once 2026-11-15T08:38:05Z, RED as fired (stamp 0 fail, recording refused), never re-fired; a stamp
     that does not read 0 fail leaves v2-02 and v2-03 unread, and entry 21 reads the stamp; …»
3. **The v2-03 criterion** (plan, `## Acceptance Criteria`)
   - before: «entry 10 `live-wrapped: green`: on the home entry 9 stamped green for the round's CLI, 2.1.325
     (`stamped 2.1.325  {n} pass  0 fail` — every row of the relied set …), one live wrapped start …»
   - after: «entry 10 `live-wrapped: green`: on the home whose 2.1.325 stamp entry 21 reads `pass`
     (`stamp-check: pass — stamped 2.1.325`; `stamped 2.1.325  {n} pass  0 fail` — every row of the relied set …),
     entry 9 itself red as fired (its stamp green, its recording refused on the rig's `home` directory), one live
     wrapped start …»
4. **The v2-02 criterion** (plan; it named no stamp before — the basis is now explicit)
   - before: «matrix#v2-02 — entry 10's hold reading: one `question`, …»
   - after: «matrix#v2-02 — on the CLI whose 2.1.325 stamp entry 21 reads `pass` (entry 9 itself red as fired: its stamp
     green, its recording refused), entry 10's hold reading: one `question`, …»
5. **v2-03's claimed acceptance** (ledger)
   - before: «… on a test home that plan entry 9's `viola verify` stamped `0 fail` for 2.1.325, the round's CLI
     (`stamped 2.1.325  {n} pass  0 fail`: every row of the relied set, `LedgerRow::relied()`, passing, `{n}` as
     verify prints it): …»
   - after: «… on the test home whose 2.1.325 stamp plan entry 21 reads `pass` (`stamp-check: pass — stamped 2.1.325`:
     every row of `LedgerRow::relied()` `pass`, the stamps file's sha256 and the run's `start` rows named) — plan
     entry 9 itself red as fired, its stamp `stamped 2.1.325  21 pass  0 fail` green and its recording refused on the
     rig's `home` directory: …» (every other clause unchanged)
6. **v2-02's claimed acceptance** (ledger)
   - before: «A live dialog hold read near the 3600 s ceiling stands on record (plan entry 10, on a verified CLI with
     `config.json` `dialog_hold_s` 3600): …»
   - after: «A live dialog hold read near the 3600 s ceiling stands on record (plan entry 10, on the CLI whose 2.1.325
     stamp plan entry 21 reads `pass` — plan entry 9 itself red as fired, its stamp green and its recording refused —
     with `config.json` `dialog_hold_s` 3600): …» (every other clause unchanged)

**Kept as written** (inputs#I11 item 3): Test Commands prose «A red entry 9 (a 2.1.325 verify that does not stamp
`0 fail`) leaves v2-02 and v2-03 unread …», and the criterion «A red entry 9 — a 2.1.325 verify that does not stamp
`0 fail` — leaves …». Both condition on the stamp, not on entry 9's verdict.

## Mechanical checks (re-run whole after the last `put`, rev 6)
- `gate.py run --dry-run`: the fence parses, 21 entries.
- `gate.py baseline`: seen green 1 · not seen 0 · new (key) 13 · leg 7. Entries 1–20 are carried unchanged with their
  baselines; entry 21 was baseline-run (above).
- `planlint.py check`: `hits: 0`. Its six «unchanged» claims were each read and all hold: the cmp'd rigs ·
  `DIALOG_HOLD_MAX`'s value · the stamp copy (`cmp` 0, twice) · mise's link · the committed 2.1.302 exchange.
- `inputs.py verify`: 11 entries — drifted 0 · unparsed 0 (I2, I7, I8 uncited: information).
- `gate.py scope --lists-only`: `scope lists: parsed 4 items`.
- Absence assertions over the plan's own names:
  - no «green stamp» / «stamped green» / «a red here» wording left (grep, 0 hits);
  - entry 21's control (above);
  - the refusal literal `refused: no green live-verify for this cli` kept in the rig.
- Validation-1: aligned with scope rev 5 (its CARRY 2 and `--record` bullets premise-corrected, and the stamp-entry
  bullet closed against the kept stamp).

## Concretization preview
- **v2-03** and **v2-02** (claimed, gated; both stay claimed): the stamp basis moves from entry 9 to entry 21, as
  shown in items 5 and 6 above. Every other clause is unchanged.
  - Never weaker: the same relied-set predicate, now on a gate entry whose red control is recorded, and entry 9's red
    stated beside it.
  - Re-claimed after the word, each with a dated note. The gated block is unchanged; fork 4 (`hold:`) still waits on
    entry 10's reading. A stand-in, stub or recorded artifact never verifies a gated cap.

## Beyond the plan
| # | proposal | kind | where |
|---|---|---|---|
| 1 | «The 2.1.325 recorded fixture set» (approved on `.andromeda/runs/2026-11-15T08-23-45-phase/p5-card-1.md`) now also owns CARRY 2's recording: a clean `viola verify --record` on 2.1.325 whose test-home directory is not named `home`, its box-only screen read there; with CARRY 1's re-recorded `Mcp.tools-list-1.json` | the existing CARRY row, widened | the wrap's route-resolve |

Apply? (yes / review / cancel)
