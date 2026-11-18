# P5 card 2 — 2026-11-18-trust-record-row-read-live, Revision 3, folded on the founder's answer (the changed lines only)

Plan: `doc:2026-11-18-trust-record-row-read-live/plan` rev 8, rendered to
`.andromeda/cache/docs/2026-11-18-trust-record-row-read-live/plan.md` to open in an editor. No plan stands in the
chunk's folder; writing in that file changes no record until a `put`. `python -X utf8
~/.claude/skills/andromeda-tools/scripts/rec.py diff --root . 2026-11-18-trust-record-row-read-live` diffs the chunk's
records.

**Fold:** card 1 (`.andromeda/runs/2026-11-18T07-42-29Z-phase/p5-card-1.md`) went to review. The founder answered its
one choice himself, «Строки целиком, только для рига», read 08:06:08Z (inputs#I13). The operator's review is
inputs#I12. Everything else on card 1 stands as accepted (inputs#I13 item 4).

## What changed since card 1
- **Metadata** names the P5 fold.
- **Revision 3 section:**
  - The lean paragraph is replaced by the founder's answer. The rows stay whole through the payload scrub, and every
    row and every join is checked, so any leftover path, name or email refuses the set with nothing written.
  - These rows are written ONLY under a new HIDDEN verify option, `--record-first-screens`. It is absent from
    `--help` and from the user docs, it does nothing without `--record`, and only the rig's `V` arm passes it.
  - A plain `--record` behaves exactly as before this chunk.
  - The terminal-queries difference is cut to one line: ruled out from the code.
- **Step 22:**
  - The option is `#[arg(long, hide = true)]`. The writer runs only when `--record` and the option are both set and
    the verify failed.
  - A plain failed `--record` writes no screen and creates no record dir.
  - The passing path is untouched, so its file list and bytes are unchanged, with the option or without it.
  - The option's doc comment names both answers.
- **Step 23, the cases:**
  - `verify_record_on_a_failed_run_writes_no_screen_without_the_rig_option`: plain `--record` on a failed verify
    leaves no record dir, the same listing as before this chunk.
  - `verify_record_first_screens_on_a_failed_run_writes_its_first_screen_only`: with the option, exactly
    `FirstScreen.f.json` is written.
  - The TERM case runs with both flags and leaves no record dir.
  - `verify_help_names_no_first_screens_option` (unit): `--help` does not name the option.
  - The plain passing `--record` keeps its exact file list.
- **Step 24:** the `V` argv gains `--record <scratch>/record --record-first-screens`. No other caller passes the
  option.
- **Step 26** spells out the two lines implement reads (inputs#I13 item 4):
  1. **`failing:`**:
     - If it names none of the 21 floor ids (listed in the step), the floor held. Implement then reads
       `trust-record:` and takes step 11's two ends.
     - If it names any floor id, implement reads line 2.
  2. **`kept:`**:
     - `kept: <letters>` → GREEN as a diagnostic reading. Implement names each kept screen.
     - `kept: none` → RED: the instrument failed, recorded as read.

  Either way, implement stops before the operator pass.
- **Control:** `rec-wordfilter` is replaced by **`rec-hiddengate`** (`--timeout 900`, entry 5). It removes the option's
  gate. The no-option case must then turn red, because a plain failed `--record` would write `FirstScreen.f.json`.
- **Acceptance:** the Revision 3 tests line names the option, the no-option case and the `--help` case. The V-arm
  line names the option, and the control line names `rec-hiddengate`.
- **Constraints:**
  - A first-screen file needs both flags.
  - New: never change what a plain `--record` writes, never show the option in `--help` or the docs, and no caller
    passes it but the `V` arm.
- **Expected amendment (security-plan, the three rows)** is reworded on BOTH answers (07:41:36Z and 08:06:08Z): a
  rig-only, hidden, failed-verify first-screen record of whole scrubbed rows, kept local and never committed. It is
  not a change to what `--record` writes for a user.
  - Pattern: «Boundary widening» (`rul-r74hshif`, `rul-p2zl2kgl`).
  - Answered by the founder himself, relayed by viola-overseer.
  - It also names that the hidden flag stands beside the invariant «no CLI flag widens redaction», as the founder's
    own narrowing of his ratified widening.
- **The live entry's note** names the option and the two lines. Its atoms are unchanged.
- **Scope rev 7:** the Revision 3 bullet names the narrowing.

## Checks (re-run in full after the fold)
- Dry-run: `entries 19 · not-run 19`.
- Baseline: 2 seen green and 5 new (key), carried over unchanged.
- planlint: `hits: 0`. The 12 (d) claims were read: the «unchanged» and «not touched» claims on `--record` and `record`
  all say the plain or passing path is unchanged, which steps 22 and 23 hold.
- `inputs.py verify`: 13 entries, 0 drifted. I6 and I9 are uncited, as they stood.
- Research lists: `scope lists: parsed 19 items`.
- List sync: `scope sync: PASS · delta +3 −0`.
- Validation-1: aligned with scope rev 7 and the working entry.
- Research: lists synced to the revision's delta (+3 −0), rev 5; not re-derived since the take-up's rev 2.
- No capability is claimed.

## Beyond the plan
- (standing — `.andromeda/runs/2026-11-18T06-13-37Z-phase/p5-card-1.md`) none, as carried from
  `.andromeda/runs/2026-11-18T00-06-23Z-phase/p5-card-2.md`.
- (standing — `.andromeda/runs/2026-11-18T06-13-37Z-phase/p5-card-1.md`) F2, the product finding (verify in a
  first-run config at 80×24 reads its interactive rows `fail`). Its options are the founder's, and no owner route
  entry is proposed; it is recorded in `founder-forks.md` only. S0C shows that the dialog is verify's doing, so F2
  stands as a question for whatever S0R-3 names.
- **Ended:** (standing — `.andromeda/runs/2026-11-18T06-13-37Z-phase/p5-card-1.md`) «S0R-3 is left for the founder's
  word … Revision 2 keeps it unspent». His word came (inputs#I11 item 1), and this revision spends S0R-3 as the last
  start of the cap.
- A new cap, asked of the founder after S0R-3, for any mend and re-read (inputs#I11 item 7). It is no route entry; it
  is named at the stop.

Apply? (yes / review / cancel)
