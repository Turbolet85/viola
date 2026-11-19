# P5 card 1 — revision 2 of 2026-11-18-the-live-home-s-re-stamp-and-the-product-re-pin (plan rev 12)

The changed lines only (`inputs#I10` item 6). The plan approved on revision 1's card
(`.andromeda/runs/2026-11-19T06-14-59Z-phase/p5-card-1.md`, rev 11) stands in every other line.

**Door.** `/andromeda-implement` ran on rev 11 and stopped before the live round
(`.andromeda/runs/2026-11-19T06-21-00Z-implement`, its outcome file). Entries 1–9 were green on the first firing and
the `bt-line` control held. The R1 count read 30 of 30 green on the unmended tree, so the cause stayed unread and the
plan's own stop fired. No live start was made. Opened on the operator's word (`inputs#I9`) and the overseer's relay
(`inputs#I10`).

## What changed, rev 11 → rev 12
- **The fold leaves the gates.**
  - **Step 18a stays as built.** A failing `box-text` assert of `rig_rehearsal_reads_the_box_empty_after_x_and_del`
    prints the rig summary and the kept `after-del` frame.
  - **Steps 18b–18d leave the plan:** no cause-hunt, no settle mend in `r1_keys`.
  - **The 30-run entry (rev 11's entry 7) leaves the block.** It now holds **27 entries**: the round is 9–18, the
    operator pass 19–22 and the re-pin 23–27 (only after entry 22 reads `verdict: green`).
  - **No retry rule is added and no entry is loosened.** A red of the case in entry 6 or 7 is still a red, read from
    18a's print.
- **The count's one reading is recorded.** 30 of 30 green on the unmended tree, 2026-11-19 (`evidence/r1-red.md`). It
  closes nothing, and the report says so.
- **The R1 readings so far.** The relay calls these «three reds», but two of the three readings were red:
  - red at entry 6's P5 baseline, 2026-11-18T22:56Z;
  - red at the 25th firing of the 30-run baseline, 2026-11-18T23:12Z;
  - none in the stopped run's 30 firings, 2026-11-19.
- **Re-entry at step 19.** Steps 1–17 and 18a are done, with evidence: the outcome file, `evidence/r1-red.md` and
  `evidence/control-1.txt`.
  - Step 19 fires entries 1–8 again on the tree as the stopped run left it. The `bt-line` control already held and is
    not fired again.
  - Then comes the round, unchanged: `L1`, `L2`, the spare `L1-2`, the cap and every stop.
  - One line now says why the round does not wait on the fold: `arm_r1` and `r1_keys` are reached only from `Arm::R1`,
    and neither `VH` nor `CX` rests on them.
- **Acceptance (R1).** «The cause is named, then the mend, then 30 of 30» is replaced by: the 18a instrument stands,
  `binary(rig_rehearsal)` is green, and the count's reading is named as closing nothing.
- **Constraints added:**
  - no retry rule or loosened entry for the R1 case;
  - the round is not held behind the fold;
  - no cause-hunt in this chunk.
- **What the next run's report names:**
  - the count's reading;
  - `$TMPDIR/vrig-hand-7IXWRN`, which stands and goes to the operator's desk list. Nobody removes it in this chunk.
- **Hygiene, done in this phase on the operator's word** (`inputs#I10` item 4). In
  `.andromeda/runs/2026-11-18T21-59-43Z-phase/founder-forks.md:72`, the old prefix (the temp root) now reads
  `$TMPDIR/viola-verify-*`. Nothing else on the line changed.

## Top acceptance criteria
- All other criteria stand as on revision 1's card. C3 and C4 are `L1` GREEN on `cli: 2.1.328`. C1 is P0 `present`.
  C6 is read, never verification. Re-pin and tests are unchanged.

## Touchpoints · test commands
- **Touchpoints:** 9 files to modify, 0 new, the same as rev 11. `rig_rehearsal.rs` now also names step 18a.
- **Test commands:** 27 `[[gate]]` entries. Entries 3, 5 and 6 are `new` and keep their take-up baselines, because
  their `run` is unchanged (`gate.py baseline`: seen green 5 · new 3 · leg 19).

Research: lists synced to the revision's delta (+0 −0), rev 4; not re-derived since the take-up's rev 2.

## Checks
- `gate.py scope --sync`: **PASS**, delta +0 −0 (plan rev 11 → 12, research rev 4).
- `gate.py scope --lists-only`: parsed, 9 items.
- `gate.py run --dry-run`: 27 entries parse.
- `planlint.py check`: 0 hits. The 12 (d) listing (11 claims, the same as at rev 11) was read: each claim still holds.
- `inputs.py verify`: 10 entries, 0 drifted, 0 uncited.
- Validation-1: aligned with scope rev 6. That revision adds a `## Revision 2` section of three verified bullets.
- Matrix: no capability is claimed, and the `coverage` view holds no row for this chunk.
- `gate.py hygiene`: the `founder-forks.md:72` row is cleared. One row stands that is NOT this phase's:
  `.andromeda/runs/2026-11-19T06-21-00Z-implement/outcome-…md:66`. It is the stopped run's outcome file, quoting the
  same temp-root `viola-verify-*` spelling while it describes the row. The operator's word covered `founder-forks.md:72`
  alone, so this phase left it as found. It is the operator pass's hygiene entry (20) to clear, or your word now.

## Documents
The plan's render is `.andromeda/cache/docs/2026-11-18-the-live-home-s-re-stamp-and-the-product-re-pin/plan.md`; open it
in an editor. No plan stands in the chunk's folder, and writing in that file changes no record until a `put`.
`python -X utf8 ~/.claude/skills/andromeda-tools/scripts/rec.py diff --root . 2026-11-18-the-live-home-s-re-stamp-and-the-product-re-pin`
shows a diff of the chunk's records.

## Beyond the plan
- (standing — `.andromeda/runs/2026-11-18T21-59-43Z-phase/p5-card-2.md`) **The switch** is an operator step with no
  entry, owned by the overseer and the founder after the pass:
  - replace `~/.local/viola-product/bin/viola` with the placed `viola.<sha8>`;
  - restart this builder by `/exit` through the product, then the operator's launcher.
- (standing — `.andromeda/runs/2026-11-18T21-59-43Z-phase/p5-card-2.md`) **C5's two unmeasured items** are left
  unmeasured. The wrap's route-resolve places or drops them with the reason.
- (standing — `.andromeda/runs/2026-11-19T06-14-59Z-phase/p5-card-1.md`) **A 2.1.328 recorded fixture set, and
  `STAMPED` naming it**, is proposed as a carry for the wrap's route-resolve. Its proposed title is «The 2.1.328
  recorded fixture set», placed after this entry.
- (new, `inputs#I10` item 2) **The intermittent R1 red, a carry proposal for the wrap's route-resolve**, worded as
  what the next reader does:
  - at the next red of `rig_rehearsal_reads_the_box_empty_after_x_and_del`, read the printed rig summary and the
    `after-del` frame (step 18a's print);
  - name the cause, and only then mend the wait;
  - no retry rule, no loosened entry.

  The readings so far are those named above (red 2026-11-18T22:56Z, red 2026-11-18T23:12Z, none in 30 on 2026-11-19).
  It goes to a route entry the wrap places, or as a carry on the next entry taken up.
- (new, `inputs#I10` item 5) **`$TMPDIR/vrig-hand-7IXWRN`**, the stopped run's by-hand rehearsal dir, joins the
  operator's desk list. It is not removed in this chunk.

Apply? (yes / review / cancel)
