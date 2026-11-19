# P5 card 1 — Revision 1 of 2026-11-19-the-2-1-328-recorded-fixture-set (changed lines only)

Plan rev 3 → 5 (`doc:2026-11-19-the-2-1-328-recorded-fixture-set/plan`). The render to open in an editor:
`.andromeda/cache/docs/2026-11-19-the-2-1-328-recorded-fixture-set/plan.md`. No plan stands in a chunk's folder, and
writing in that file changes no record until a `put`. Diff of the chunk's records:
`python -X utf8 ~/.claude/skills/andromeda-tools/scripts/rec.py diff --root . 2026-11-19-the-2-1-328-recorded-fixture-set`.

**Why the revision:** `R4` was RED at 14:07:46Z on `record: exists`, its own closed word. The stale 2.1.325 kept dir
under `target/live-rig/R4/record/` stopped `keep_record`, so the 2.1.328 record was lost. Word: `inputs#I5`, relay
`inputs#I6`.

## Changed
- **New §R:** done with evidence: steps 1–6 (entries 1–5 green at 12:00, `cx-relook` «holds»), and `R4` recorded
  red from its log. `R4` is never re-fired. Entries 1–5 are **not** re-fired: the tree has not moved since; implement
  checks that first with `git status` and `stat`.
- **Re-entry:** step 8, the rename. Then the host read, entries 6–9, **`R4-2`** (10), 11–12, `R4M` (13), 14, `L2`
  (15), and the operator census (16).
- **New step 8, the rename:** `target/live-rig/{R4,R4M}/record` → `…/record-2.1.325`. Nothing is deleted. Each dir's
  file count before and after (27 / 1 now) goes to evidence and must be equal, or implement stops before any start.
- **New entries 9 and 12** (leg `round`): `test ! -e … && test ! -L …` on `target/live-rig/R4-2/record` and
  `target/live-rig/R4M/record`, right before each start and inside its resume run. A standing dir reads red and stops
  the start. `L2` keeps nothing under a label dir (only VR keeps a record and only MT a tap fixture), so it gets no
  absence entry.
- **Entry 10 is now `live R4-2 … --cap 1 --spare 1`** (was `R4`). A `-2` label counts against `--spare` alone
  (`ledger::admit`), so `R4M --cap 2` and `L2 --cap 3` stand, and the cap is unchanged. There is no `R4-3`; a fourth
  base start is the founder's.
- **The one kept dir `R4-2` writes:** its own label's, `target/live-rig/R4-2/record/2.1.328/` (mod.rs:530). These
  entries read it: entry 10's `artifact`, step 13's copy and `evidence/landing.md`, and `diff -r` (entry 17,
  re-pointed; baseline re-read red, exit 2, both dirs absent). `cmp` (entry 18) reads
  `target/live-rig/R4M/record/2.1.328/…`. The hygiene suite and `gate.py hygiene` read only `fixtures/`.
- **`cx-relook`** now names entry 4 (`run --unit`), the entry it was truly measured on.
- **Entry numbers shifted by 2 past entry 8:** the operator census is entry 16, the post-landing entries are 17–23,
  and the operator pass is 24–27.
- **Measured at P3:** `R4M`'s tap word keys on the version file, so a stale `2.1.325/` alone would not have stopped
  it. The outcome's line saying it would is corrected. The rename and absence entry stay for `R4M` by the relay.
- Research: lists synced to the revision's delta (+0 −0), rev 2; not re-derived since the take-up's rev 1.
- Caps: none claimed before, none now.

## Beyond the plan
1. (standing — `.andromeda/runs/2026-11-19T09-28-18Z-phase/p5-card-2.md`) **(conditional) The primed two-send `CX`
   start**, on an `L2` reading `context-transcript-after: absent`: a carry proposal for the wrap's route adaptation,
   owner named there, its start a further cap of the founder's (`inputs#I2` item 2, `inputs#I3` item 1).
2. (standing — `.andromeda/runs/2026-11-19T09-28-18Z-phase/p5-card-2.md`) **(conditional) Listing `/context` as a
   local command**, only if `L2`'s reading supports it: a carry proposal for the wrap, owner named there (`inputs#I2`
   item 6).
3. (standing — `.andromeda/runs/2026-11-19T09-28-18Z-phase/p5-card-2.md`) **The pre-send gate reads a quoted modal
   literal as a standing modal.** A transcript row that quotes a modal literal refuses the driver's send
   (`not-delivered input-not-ready`, measured 2026-11-19T10:22Z). Not mended here; a carry proposal for the wrap,
   with `inputs#I3` (its item FIRST and item 2) as its evidence.
4. **Key the rig's kept dir by the CLI version** (`target/live-rig/<label>/record/<version>/` checked per version,
   not per label), so that a later version's set never meets an earlier one's. A carry proposal for the wrap, owner
   named there. The rig's code is not changed here (`inputs#I6` item 5). Evidence: `R4`'s `record: exists` today.

Apply? (yes / review / cancel)
