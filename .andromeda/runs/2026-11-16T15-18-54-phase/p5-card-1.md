# P5 review — revision 1 of 2026-11-16-survey-key-keeps-the-wheel (changed lines only)

**Why:** `/andromeda-implement` stopped before the operator pass. S4 counted and read no survey to key into
(`shown: no · keys-written: 0`, ledger 4 of 4). The operator's word is `inputs#I7`. The founder's cap of 6, and what
the revision may change, are in `inputs#I8`, the overseer's relay.

**The plan, to open in an editor:** `.andromeda/cache/docs/2026-11-16-survey-key-keeps-the-wheel/plan.md` (rev 7).
- No plan stands in the chunk's folder.
- Writing in that file changes no record until a `put`.
- `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/rec.py diff --root . 2026-11-16-survey-key-keeps-the-wheel`
  diffs the chunk's records.

## What changed
- **Scope (rev 5):**
  - "Live starts: 4" is corrected to "at most 6: S1–S4 spent, then S5 and, only if S5 again reads no survey, S6"
    (`[premise-corrected]`).
  - One bullet is added: S4's two likely causes, neither measured.
- **Re-entry:** a block at the head of Implementation Steps names steps 1–9 and S4 as done, with their evidence.
  `/andromeda-implement` re-enters at step 1's rig edits, then:
  1. entry 12 (the rehearsal);
  2. entry 16 (the build);
  3. S5;
  4. S6 if owed;
  5. step 11;
  6. the operator pass.

  The product tree is not touched.
- **Step 1, the rig** (marked *Revision 1*):
  - `live S5|S6` added. A label already ledgered is refused uncounted (`refused: spent`). `live S6` is refused
    uncounted unless the ledger's S5 summary reads `shown: no` (`refused: s6-not-owed`).
  - The cap moves from 4 to 6 (`refused: cap`).
  - A token-window refusal is uncounted. The run says so and stops, with no timer.
  - **S5** is S4's shape (`viola run survey-s5 -- claude`, the prompt by one `viola send`). It keys `0` once, after
    the prompt and options-row literals have stood together on the screen for 1 s, at any point in a watch:
    - the watch opens at the first send's reply and closes 120 s after `turn-ended`, or 210 s after it opened,
      whichever comes first;
    - the screen is pumped throughout, the send's own run included;
    - literals seen only during the send, or for under 1 s, write no key.
  - **S6, the second cause:** no send stands between the survey's appearance and the key. Its shape is
    `viola run survey-s6 -- claude "viola survey probe: reply with the single word ok"`:
    - the turn starts from the child's own argument, as in S1–S3;
    - no `viola send` comes before the key, and `first-send: none`;
    - the one send after the key is its `second-send`;
    - the watch opens at `session-start`.
  - New readings: `survey-seen-at: during-send|mid-turn|after-turn-end|none` · `survey-stood-ms` ·
    `turn-ended-before-key`.
- **Step 2, the rehearsal:** new arms `cap-refused` (at 6) · `spent-refused` · `s6-not-owed` · `s5-shape` (it replaces
  `s4-shape`) · `s6-shape` · `survey-timing`. `survey-timing` is the «survey seen mid-turn» arm: six synthetic
  timelines over the rig's key-due rule, with no byte written.
  - **What it cannot show:** the fake agent draws no survey screen (no recorded `Screen.survey`, fork 3), and it
    ignores a positional prompt (its argument parse's `_ => {}` arm).
  - So no real mid-turn survey, and no argument-started S6 turn, is rehearsed. The rule is proven only on synthetic
    timelines; under the fake agent, only the watch's span and S6's no-send start shape are shown.
- **Steps 3, 10, 10b, 11:**
  - S5 is fired as entry 17. If it reads `wheel-after-key: none` and `second-send: confirmed`, the claim is proved and
    S6 stays unspent; the record says so.
  - If S5 reads `shown: no`, S6 is fired as entry 18, with the same assertions.
  - Any other reading is red and stops the chunk before the operator pass.
  - A seventh start, or any other key, goes back to the founder.
  - S4 is written as it was read.
- **Gate fence:**
  - Entry 12 gains seven arm atoms. Its baseline was run now: **red as predicted**, exit 0 with the old arms passing,
    `contains arm spent-refused: pass` failing.
  - The old entry 17 (S4) is replaced by entry 17 (`live S5`, `ledger: starts 5`) and entry 18 (`live S6`,
    `ledger: starts 6`). Both have timeout 420, and both expect `token-window: ok`, `wheel-after-key: none`,
    `second-send: confirmed` and `folder: absent`.
  - pre-push, hygiene, push and the CI read move to entries 19–22, and the prose references follow.
- **Acceptance:** v2-21 names S5 (entry 17), or S6 (entry 18) only if S5 reads no survey, where it named S4. The
  security line covers each start made, with no seventh start.
- **Constraints:** four rejections added (S6 after an S5 that proved the claim · re-labelling or re-firing S4 · a
  timer for the token · any move of the product code, its tests, the grammar or the claim's wording).
- **Unchanged:** the product code, its tests and controls, the grammar (`0` and `3`), entries 1–11, 13–16, and the
  claim's wording beyond the S5/S6 naming.

## Checks (step 1, re-run after the last edit)
- `gate.py scope --sync`: **PASS**, delta +0 −0 (plan rev 5 → 7; research rev 4, its `## Revision 1` line).
- dry-run: 22 entries parse. baseline: 9 `new` entries; entry 12 is the only one changed, and it was baseline-run (red
  as predicted). The others are carried over unchanged with their baselines.
- planlint: 0 hits; the four 12 (d) «unchanged» listings are the take-up's lines, untouched.
- inputs verify: 8 entries, 0 drifted. I5 stays uncited, as the take-up left it.
- The lists parse (4 items).
- Validation-1: aligned with scope rev 5.

Research: lists synced to the revision's delta (+0 −0), rev 4; not re-derived since the take-up's rev 2.

## Concretization preview — `matrix#v2-21` (claimed by this chunk; re-claimed after the word)
- **Now:** "… in S4 (plan entry 17), under viola run on this chunk's build, the survey key 0 writes no
  wheel{holder:"human",cause:"human-input"} line after its offset …, and the driver's next viola send is confirmed; in
  CI, …"
- **Proposed:** "… in S5 (plan entry 17), or, only if S5 reads no survey to type into, in S6 (plan entry 18), under
  viola run on this chunk's build, …". Every other word is the same, so nothing is weakened (`inputs#I8` item 2).
  A dated note is added naming the revision and the word.

## Beyond the plan
- (standing — `.andromeda/runs/2026-11-16T12-32-18-phase/p5-card-2.md`) An owner for the part left out (`inputs#I3`,
  fork 3). A CARRY on «The 2.1.325 recorded fixture set»:
  - the survey's capability-ledger row;
  - its `viola verify` Run F: a forced survey, signature read, no key;
  - the recorded `Screen.survey`;
  - the fake agent's survey-screen option;
  - an outer-PTY `tui_wheel` survey case.

Apply? (yes / review / cancel)
