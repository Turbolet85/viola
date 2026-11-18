# P5 card 1 — 2026-11-18-trust-record-row-read-live, Revision 4 (the changed lines only, inputs#I17 item 7)

Plan: `doc:2026-11-18-trust-record-row-read-live/plan` rev 10, rendered to
`.andromeda/cache/docs/2026-11-18-trust-record-row-read-live/plan.md` to open in an editor. No plan stands in the
chunk's folder; writing in that file changes no record until a `put`. `python -X utf8
~/.claude/skills/andromeda-tools/scripts/rec.py diff --root . 2026-11-18-trust-record-row-read-live` diffs the chunk's
records.

**The stop:** `S0R-3` read the floor broken with `kept: b,c,d,f`. All four kept screens are one CLI modal, a
`browser-tools-prompt`, and it selects the order. The R6 cap is exhausted.

**The word:** the operator, inputs#I16. The founder answered «Риг сейчас, окно — отдельной записью» and «1 + 1
запасной», read 09:00:30Z and relayed as inputs#I17.

## What changed
- **Metadata** names Revision 4.
- **New section «Revision 4»:**
  - **Done, with evidence:** steps 21–25 are built and green, and the control `rec-hiddengate` holds. `S0R-3` was
    read as recorded.
  - **The word:** only the rig is mended. Verify and `run` are not changed for the prompt. The prompt becomes its own
    route entry at the wrap. The new cap is `S0B` plus one spare, `S0B-2`, on a fresh ledger
    `evidence/browser-starts.ndjson` (`--cap 1 --spare 1`).
  - **The bundle read** (item 3; no start):
    - **The dialog** is `ChromeAutoEnableDialog`. Its «keep browser tools off» answer writes
      `claudeInChromeDefaultEnabled: false`.
    - **The offer step** skips («decision already recorded») when that key holds any value.
    - **The arming gate** is `MIs`. It needs that key absent, a detected extension and a GrowthBook flag. The
      detection comes from the config's `cachedChromeExtensionInstalled`. An earlier start's background scan writes
      that key, so the trigger is a detected extension recorded by an earlier start. It is not a startup count and
      not a print-mode run as such.
    - **The seed** is that key, with the value the bundle writes. The bundle has the key, so the environment arm is
      not taken.
  - **Item 4 (which earlier step arms it):**
    - The bundle answers HOW: any earlier start whose scan ran writes the cache. `S0C`'s single start read it empty.
    - It does not answer WHICH: print probe or Run A stays **unmeasured**.
    - The link from `S(…)` to the GrowthBook cache is not traced.
  - **The six markers, one bundle reason each:**
    - Kept: `hasSeenAutoDefaultNotice`, `lastReleaseNotesSeen`, `lastClawdEntranceVersion`, `migrationVersion`.
    - Dropped: `hasSeenAutoModeEntryWarning` (read only when a session enters auto mode, which no verify run does) and
      `lastOnboardingVersion` (written, never read).
- **Step 26** gains its «As read».
- **Step 27, `rig/isolate.rs`:**
  - `Marker::Off` is added, and `USED_MARKERS` becomes five keys.
  - The doc comment gives each key's bundle reason.
  - The unit case is renamed `seed_used_markers_writes_the_bundle_markers_and_keeps_the_seed`. It pins the exact key
    set: the seed's two plus the five, `claudeInChromeDefaultEnabled` false, and the two dropped markers absent.
  - `seed-markers: written` now covers all five keys.
  - The arms stay as they are: V and VC share the seed, and every other arm is unchanged.
- **Step 28:** `Arm::of_label` takes `S0B` and `S0B-2` as `V`, and `S0B-3` maps to none. The label case and the live
  verb's module comment are updated. The ledger needs no change, because `-2` already counts as a spare.
- **Step 29:** everything is green and landed before the start. No control: item 7 names the unit case as the pin.
  Both V rehearsals stay green unchanged.
- **Step 30, `S0B`:**
  - **The start:** ledgered with the clock's time first. `keys: none`, with `stop:` printed beside it. A start refused
    for the token window is uncounted.
  - **RED:** any floor id on `failing:`, `failing:` missing or unread, `rows` below 23, `seed-markers` not written, or
    a failed teardown word. On RED, each kept screen is named in step 26's closed words and implement stops.
    `S0B-2` is spent only when a kept screen shows a further obstacle that is the rig's to mend, after that mend's
    plan revision.
  - **Floor held, `pass`:** the row is VERIFIED and the operator pass runs.
  - **Floor held, `fail`:** implement stops, Run F's word is named, and the fork goes to the founder.
- **Test Commands:**
  - The prose names `S0B`.
  - The controls paragraph says Revision 4 adds none.
  - The spent `S0R-3` entry is replaced by the `S0B` entry: the fresh ledger, `--cap 1 --spare 1`, S0R-2's 21 floor
    `lacks` atoms, and `kept:` / `lacks kept: unread` / `lacks refused:`. Each atom key was checked against the
    S0R-3 summary in `evidence/live-starts.md`.
  - The probe, build and census notes say «before/after S0B».
- **Acceptance:**
  - S0R-3's «As read» is added.
  - Two lines are new: the Revision 4 tests and the live `S0B`.
  - The CI line now reads «held floor with `trust-record: pass`».
  - The Revision 1 seed line is marked superseded.
- **Constraints:** six new lines:
  - no modal literal, ledger row or probe, and no verify or `run` change, for the prompt;
  - no environment hide;
  - no copied value and no key without a reason;
  - no start to measure the order;
  - `S0B-2` only on a rig cause after a revision, and a third start is the founder's;
  - the R6 ledger is not touched.
- **Notes:**
  - Expected amendments: security-plan `decision/live-rig-isolated-child` (the seed holds five keys whose values come
    from the bundle) and test-plan `decision/standing-live-rig` (the `S0B` labels and the key).
  - Leaned no boundary widening: one more key goes into the rig's own private config, which adds no crossing. No
    escalate pattern is met and no fork is asked.
  - The product finding is named.
  - Provenance (Revision 4).
- **Touchpoints:** the descriptions of `rig/isolate.rs`, `rig/mod.rs` and `viola-live-rig.rs` are updated. No file is
  added.
- **Scope** is at rev 8, with a Revision 4 bullet closed against the bundle.

## Checks (step 1, run in full after the last edit)
- Dry-run: `entries 19 · not-run 19`.
- Baseline: `seen green 2 · new (key) 5 · leg 12`, carried over unchanged.
- planlint: `hits: 0`. The 12 (d) claims were read. The one that conflicted with the revision (the Revision 1 seed
  line) is marked superseded.
- `inputs.py verify`: 17 entries, 0 drifted. I6, I9 and I14 are the earlier P5 words, uncited as they stood.
- Research lists: `scope lists: parsed 19 items`.
- List sync: `scope sync: PASS · delta +0 −0`.
- Validation-1: aligned with scope rev 8 and the working entry.
- Research: lists synced to the revision's delta (+0 −0), rev 6; not re-derived since the take-up's rev 2.
- No capability is claimed (coverage: 0 rows for the chunk).

## Beyond the plan
- (standing — `.andromeda/runs/2026-11-18T07-42-29Z-phase/p5-card-2.md`) none, as carried from
  `.andromeda/runs/2026-11-18T00-06-23Z-phase/p5-card-2.md`.
- (standing — `.andromeda/runs/2026-11-18T07-42-29Z-phase/p5-card-2.md`) F2, the product finding: verify in a
  first-run config at 80×24 reads its interactive rows `fail`. Its options are the founder's, recorded in
  `founder-forks.md` only.
  - Now that the screen is named, F2's substance is the browser-tools finding in the next row. It stands until the
    wrap folds it there.
- **Ended:** (standing — `.andromeda/runs/2026-11-18T07-42-29Z-phase/p5-card-2.md`) «A new cap, asked of the founder
  after S0R-3». His word came as inputs#I17 item 2 (`S0B` plus one spare).
- **New — a route insertion, the wrap's (inputs#I17 item 1).**
  - **Proposed title:** «Browser-tools prompt read as a modal».
  - **What it does:** a compiled modal literal for 2.1.325's `ChromeAutoEnableDialog`, with its capability-ledger row
    and `viola verify` probe. Verify then reads it as a modal, and `run`'s pre-send gate never types into it.
  - **Its place:** asked of the overseer at the wrap. Not built here.

Apply? (yes / review / cancel)
