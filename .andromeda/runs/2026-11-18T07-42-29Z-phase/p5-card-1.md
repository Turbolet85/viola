# P5 card 1 — 2026-11-18-trust-record-row-read-live, Revision 3 (the changed lines only)

Plan: `doc:2026-11-18-trust-record-row-read-live/plan` rev 7, rendered to
`.andromeda/cache/docs/2026-11-18-trust-record-row-read-live/plan.md` to open in an editor. No plan stands in the
chunk's folder; writing in that file changes no record until a `put`. `python -X utf8
~/.claude/skills/andromeda-tools/scripts/rec.py diff --root . 2026-11-18-trust-record-row-read-live` diffs the chunk's
records.

**Cause:** implement stopped before the operator pass after S0C read GREEN with no dialog (`screen: box` at 1 062 ms,
the rig's own plain `claude` at 80×24). So the screen Runs B–F meet is verify's own doing. **Word:** inputs#I10 (the
operator) and inputs#I11 (the founder's answer «Verify с записью экрана», read 07:41:36Z, and the overseer's relay).

## What changed
- **Metadata** gains the Revision 3 line.
- **New section «Revision 3»:**
  - What is done: steps 17–20 (S0C GREEN, `vc-nokey` holds). S0R, S0R-2 and S0C are not re-fired.
  - The word and its bound, from the relay's items 1–2.
  - What the scrub does to a folder path the dialog prints, read from the code:
    - The home becomes `~` and the user word becomes `<user>`.
    - Under the rig, the scratch and temp dirs are under `/tmp/`, so a path there is kept as written. It holds the
      uid's digits, never the name.
    - A home path that the dialog wraps across a seam refuses the whole set.
  - **The lean, decisive, for your review:** the file keeps the rows WHOLE through `ledger::scrub` + `screen_fault`.
    It does not apply the `signature_rows` cut. The cut keeps only rows that hold a compiled literal. B–F's screen holds
    one such row (S0R-2: `lit=rule@1`), so the cut would name nothing. This is the widening's substance. If you reject
    it, the plan goes back to P4.
  - Item 6, the three differences, each with what the kept screen would show if it is the cause:
    - **The order:** the same dialog on B, C, D and F whatever their folder, naming none of its own or another start's
      (`temp`).
    - **The trust path:** a folder-trust-family dialog naming `own-run-dir` (or `parent` on B–E).
    - **Terminal queries:** closed from the code. Neither verify's pty nor the rig's answers any query, and S0C reached
      the box unanswered, so this is no difference.
  - Implement re-enters at **step 21**.
- **Steps 21–26 (new):**
  - 21 — `TypedRun.first_rows`: the rows of each run's first settle, held in memory only. No `measured` key, no row
    and no `run` field reads them.
  - 22 — `src/cmd/verify.rs`, under these conditions only:
    - On a failed verify with `--record`, before `Exit(1)`, it writes one `<dir>/<version>/FirstScreen.<x>.json`
      (`{"v":1,"run","screen","cols":80,"rows"}`) per run whose word is not `box` (and not A's `trust`).
    - The rows are scrubbed, every row and seam is checked, and one fault refuses the whole set.
    - The files go through the passing path's `write_recording`. The passing branch is not touched.
    - A stop never reaches the writer.
    - `.gitignore` gains `fixtures/claude/*/FirstScreen.*.json`.
  - 23 — the cases:
    - Unit: `first_screen_files_take_each_failed_run_and_no_box` and
      `first_screen_scrub_keeps_a_temp_path_and_rewrites_the_home`.
    - `cli_verify`: `verify_record_on_a_failed_run_writes_its_first_screen_only` (exactly `FirstScreen.f.json`) and
      `verify_stopped_with_record_writes_no_first_screen` (TERM, no record dir).
    - The passing record's exact file list stands, and the kept-config case asserts no `FirstScreen.` file.
    - `verify_started` takes its before-args.
  - 24 — the `V` arm:
    - It passes `--record <scratch>/record`.
    - Before the teardown it copies each `FirstScreen.<x>.json` (read through `MAX_FRAME`) into
      `target/live-rig/<run>/` through `write_private`.
    - It prints `kept: <letters>|none|unread`.
    - The V rehearsal adds `kept: none`. A new rehearsal, `rig_rehearsal_v_keeps_a_failed_runs_first_screen_locally`,
      expects `kept: f`.
  - 25 — the block green, then the control, hygiene and the land.
  - 26 — `S0R-3`:
    - It runs on the R6 ledger under `--cap 1 --spare 2`.
    - The predicate is item 5's: GREEN when the run is whole, the teardown words hold, and either the floor held or
      `kept:` names a run.
    - It names each kept screen in closed words, including the folder word (`own-run-dir` · `parent` · `temp` ·
      `none`), plugin yes/no, mode yes/no and the difference it selects.
    - Then it STOPS before the operator pass, and the cap is exhausted.
- **Touchpoints:** three files are added — `src/cmd/verify.rs`, `.gitignore` and `tests/support/verify.rs`. These
  files gain their Revision 3 lines: `ledger.rs`, `test_cases.rs`, `run.rs`, `runs.rs`, `tests/cli_verify.rs`,
  `rig/mod.rs` and `rig_rehearsal.rs`.
- **Test Commands:**
  - The prose names `S0R-3` and step 26.
  - The S0C live entry leaves the block.
  - The viola build/hash entry returns.
  - The live entry is now
    `live S0R-3 --cli 2.1.325 --deadline 1500 --cap 1 --spare 2 --ledger …/evidence/live-starts.ndjson`, with
    timeout 1800. Its atoms are the run-whole and teardown words, `kept:` with `lacks kept: unread`,
    `lacks verify-exit: none` and `lacks refused:`. No floor atom is used: the floor-OR-kept disjunction is read by
    implement.
  - One control is added, `rec-wordfilter` (`--timeout 900`, entry 5): the word filter is removed, so the failed case
    writes a…f.
  - Entries 1–7 are unchanged and keep their baselines (`gate.py baseline`: 2 seen green, 5 new (key) carried over).
- **Acceptance:** four lines for the Revision 3 tests, one control line and one live line. The S0C line gains «As
  read: GREEN, no dialog».
- **Constraints:** no start after `S0R-3`; no first-screen file outside the bound; no `signature_rows` cut; no new
  writer; no row text in a record.
- **Implementation notes:** two expected amendments for the wrap.
  - security-plan `decision/verify-signature-only`, `decision/verify-screen-read-against-literals` and
    `decision/fixture-scrub-and-refusal`, add-to (an except). Pattern «Boundary widening» (`rul-r74hshif`,
    `rul-p2zl2kgl`). Answered by the founder himself, relayed by viola-overseer (inputs#I11 item 1), for the wrap's
    answer card.
  - test-plan `decision/standing-live-rig`, add-to: the V arm's `--record` and `kept:`.
- **Scope rev 6:** a new Revision 3 bullet, `[inferred]`, closed against the code.

## Checks
- Dry-run: `entries 19 · not-run 19`.
- planlint: `hits: 0`. The 12 (d) claims were read and none contradicts a step:
  - «does not change» is `merge_stamp`'s signature.
  - The R6 ledger «not touched» is scoped to Revision 2.
  - «unchanged» is the passing record's file list.
- `inputs.py verify`: 11 entries, 0 drifted. I6 and I9 (earlier P5 words) are uncited, as they stood.
- Research lists: `scope lists: parsed 19 items`.
- List sync: `scope sync: PASS · delta +3 −0`.
- Host:
  - `~/.local/share/mise/installs/claude/2.1.325/claude --version` → `2.1.325 (Claude Code)`.
  - The R6 ledger holds `S0R` and `S0R-2`, so `--spare 2` admits `S0R-3` once.
- Size: the plan is 1 075 lines, past the 400-line guide. Revisions 1–3 stand in it whole, as the stopped runs left
  it.
- Validation-1: aligned with scope rev 6 (the Revision 3 bullet) and the working entry.
- Research: lists synced to the revision's delta (+3 −0), rev 5; not re-derived since the take-up's rev 2.
- No capability is claimed (the chunk has no coverage row). No escalate fork was asked at P4. The widening is the
  founder's own answer, so it rides the expected amendment.

## Beyond the plan
- (standing — `.andromeda/runs/2026-11-18T06-13-37Z-phase/p5-card-1.md`) none, as carried from
  `.andromeda/runs/2026-11-18T00-06-23Z-phase/p5-card-2.md`.
- (standing — `.andromeda/runs/2026-11-18T06-13-37Z-phase/p5-card-1.md`) F2, the product finding (verify in a
  first-run config at 80×24 reads its interactive rows `fail`). Its options are the founder's, and no owner route
  entry is proposed; it is recorded in `founder-forks.md` only. S0C now shows that the dialog is verify's doing, not
  the config's, so F2 stands as a question for whatever S0R-3 names.
- **Ended:** (standing — `.andromeda/runs/2026-11-18T06-13-37Z-phase/p5-card-1.md`) «S0R-3 is left for the founder's
  word (R6's second spare), not spent by this chunk; Revision 2 keeps it unspent». His word came (inputs#I11 item 1):
  this revision spends `S0R-3` as the last start of the cap.
- A new cap, asked of the founder after S0R-3, for any mend and re-read (inputs#I11 item 7). It is no route entry; it
  is named at the stop.

Apply? (yes / review / cancel)
