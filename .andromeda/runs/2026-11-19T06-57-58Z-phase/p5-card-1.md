# P5 card 1 — revision 3 of 2026-11-18-the-live-home-s-re-stamp-and-the-product-re-pin (plan rev 13)

The changed lines only (`inputs#I15` item 5). The plan approved on revision 2's card
(`.andromeda/runs/2026-11-19T06-43-19Z-phase/p5-card-1.md`, rev 12) stands in every other line.

**Door.** `/andromeda-implement` ran on rev 12 and stopped at the round's entry 14
(`.andromeda/runs/2026-11-19T06-49-43Z-implement`, its outcome file). Entries 1–8 were green. `L1` read verify 24/24
green, with one red atom: `lacks 'disabled-mcp: unread'`. Opened on the operator's word (`inputs#I14`) and the
overseer's relay (`inputs#I15`). The one P4 fork was answered by the overseer (`inputs#I16`).

## What changed, rev 12 → rev 13
- **`L1` is not run again, and the spare `L1-2` is not spent.** The live home holds `L1`'s 24-row 2.1.328 stamp, every
  row counting. The red atom read a measurement that the entry's expect shaped too narrowly.
- **Entry 14's expect is mended; the rig's code is not changed.**
  - The atom `lacks 'disabled-mcp: unread'` leaves. `unread` stays the rig's word for «no entry to read it from».
  - In its place: `contains needle-claude-json:`, `contains needle-settings:` and, for each of the two words,
    `lacks …: unread`, `…: absent/unread`, `…: present/unread` and `…: absent/present`.
  - **It can still fail** (`inputs#I16` item 1). Each of these turns it red: either needle word reading `unread` on
    either side (the read not taken); either needle moving `absent` → `present` in either file; or either word
    missing.
  - **Checked against `14.log`** by a script (1881 B, `a4eeb096`): 34 of 34 content atoms held, plus `exit 0` from the
    record. Four synthetic negatives each read RED: `unread/absent`, `absent/unread`, `absent/present` and
    `present/unread`.
- **New step 19a:** entry 14 is read against the mended expect from its own log, and nothing is re-fired.
  - Implement reads it through `gate.py show --run-dir .andromeda/runs/2026-11-19T06-49-43Z-implement --marker … --n
    14`, atom by atom, into `evidence/live-starts.md` under «Entry 14 against the mended expect».
  - The stopped run's trail, its red line, `evidence/round-065404Z.txt` and the log all stand as they are.
  - If the temp dir has been cleared, the reading comes from the summary copied whole into `live-starts.md` and from
    the ledger file's `L1` line.
  - An atom failing there stops implement, as a red `L1` would.
- **Entries 1–8 run again, by the letter.** Implement P2 fires the plan's `## Test Commands` through `gate.py run` on
  every run, and no clause carries an earlier run's green into a new one. The `bt-line` control is not fired again.
- **The round re-enters at `L2`:** `gate.py run … --live-legs --entry 17` fires 15 (census), 16 (debug build) and 17
  (`L2`, `--cap 2`). The dry-run at this revision selects 15–17 alone.
- **Entry 18 is re-keyed `leg = 'operator'`** (`inputs#I16`, the fork's recommended option). `--entry 17` cannot fire
  a round step after the last leg, and the whole round would re-fire `L1`. So implement fires entry 18 once, with
  `--operator 18`, right after the round reads `COMPLETE`. Its `run` and `expect` are unchanged.
- **C6 is reworded** (`inputs#I15` item 2). On claude 2.1.328, `/mcp disable viola-probe-absent` of a server that does
  not exist wrote nothing naming it into `~/.claude.json` or `~/.claude/settings*.json`: the needles read
  `absent/absent` in both files. `projects-new: 0`, and so `disabled-mcp: unread`, is recorded as read. Why no new
  entry appeared is named UNMEASURED.
- **Acceptance added (round):** entry 14's mended atoms held from its log; the `--entry 17` round reads `COMPLETE ·
  legs fired 1/1`; entry 18 reads `census: 0`.
- **Constraints added:**
  - no re-fire of `L1`, and no `L1-2`, on this atom;
  - no rig change to the `unread` word;
  - no dropped atom without a failing reading in its place;
  - no start to chase the `projects` question.

## Top acceptance criteria
All other criteria stand as on revision 2's card. C3 and C4 were read GREEN by `L1` on `cli: 2.1.328`, and are
re-checked by step 19a. C1 read P0 `present`. C6 is read, never verification. The re-pin and the tests are unchanged.

## Touchpoints · test commands
- **Touchpoints:** 9 files to modify, 0 new, the same as rev 12. No source edit comes from this revision.
- **Test commands:** 27 `[[gate]]` entries. The only `[[gate]]` edits are entry 14's `expect` and note, and entry
  18's `leg` and note. `gate.py baseline`: seen green 5 · new 3 · leg 19. Entries 3, 5 and 6 keep their baselines,
  because their `run` is unchanged.

Research: lists synced to the revision's delta (+0 −0), rev 5; not re-derived since the take-up's rev 2.

## Checks
- `gate.py scope --sync`: **PASS**, delta +0 −0 (plan rev 12 → 13, research rev 5).
- `gate.py scope --lists-only`: parsed, 9 items.
- `gate.py run --dry-run`: 27 entries parse. The `--live-legs --entry 17` dry-run selects 15–17, and the pre-flight
  raises nothing.
- `planlint.py check`: 0 hits. The 12 (d) listing (12 claims) was read, and each claim holds. The new one, at line
  298 («`run` and `expect` are unchanged» on entry 18), is true of the fence.
- `inputs.py verify`: 16 entries, 0 drifted, 0 uncited.
- Validation-1: aligned with scope rev 7, which adds a `## Revision 3` section of three verified bullets.
- Matrix: no capability is claimed, and the `coverage` view holds no row for this chunk.

## Documents
The plan's render is `.andromeda/cache/docs/2026-11-18-the-live-home-s-re-stamp-and-the-product-re-pin/plan.md`;
open it in an editor. No plan stands in the chunk's folder, and writing in that file changes no record until a `put`.
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
- (standing — `.andromeda/runs/2026-11-19T06-43-19Z-phase/p5-card-1.md`) **The intermittent R1 red, a carry proposal
  for the wrap's route-resolve:**
  - at the next red of `rig_rehearsal_reads_the_box_empty_after_x_and_del`, read the printed rig summary and the
    `after-del` frame (step 18a's print);
  - name the cause, and only then mend the wait;
  - no retry rule, no loosened entry.

  The readings so far: red 2026-11-18T22:56Z, red 2026-11-18T23:12Z, none in 30 on 2026-11-19, and green inside
  entries 6 and 7 in the stopped run of 2026-11-19T06:49Z.
- (standing — `.andromeda/runs/2026-11-19T06-43-19Z-phase/p5-card-1.md`) **`$TMPDIR/vrig-hand-7IXWRN`** joins the
  operator's desk list. It is not removed in this chunk.
- (new, `inputs#I16` item 2) **Why no new `projects` entry appeared for verify's probe folder, a carry proposal for
  the wrap's route-resolve.**
  - **Next reader:** the next live start that reads a real Claude config by key names.
  - **Reading at `L1`:** `L1`, 2026-11-19T06:54:18Z, claude 2.1.328, read `projects-new: 0`, `projects-new-keys:
    none` and `disabled-mcp: unread`.
  - **The two candidates, both unmeasured:** the entry was filed under the repository root's existing key, or none
    was written.
  - **What it does:** read whether Run B's `/mcp disable` lands under an existing key, by key names and never by a
    value. No start is spent on this question alone.
  - **Placement:** a route entry the wrap places, or a carry on the next entry taken up.

Apply? (yes / review / cancel)
