# P5 review card 2 — «Model and effort at launch» (2026-11-23-model-and-effort-at-launch)

**What changed since card 1** (`.andromeda/runs/2026-11-23T06-49-18Z-phase/p5-card-1.md`), on your review, which is
snapped as `inputs#I3`:
- **Step 10:** the pair-after-`--` refusal's one fixed stderr line now names the two viola flags to use, `--model` and
  `--effort` given to `viola run` / `viola revive` before `--`. The line holds none of the caller's words. It is one
  literal shared by both verbs, pinned by its own literal case (`run_args_refuse_a_pair_in_the_callers_words`, in gate
  entry 8).
- **The (usage) acceptance criterion** says the same.
- **A new implementation note** records the host launcher: it puts the pair after `--` today, so once a build holding
  this chunk is pinned, its every start is that exit 2.
- **Beyond the plan:** a new row 3.
- Nothing of the steps' work or the gate block moved, so the 10 baselines stand. The full check set was re-run on rev 5.

**The plan:** `doc:2026-11-23-model-and-effort-at-launch/plan` rev 5, rendered to
`.andromeda/cache/docs/2026-11-23-model-and-effort-at-launch/plan.md` to open in an editor. No plan stands in the
chunk's folder; writing in that file changes no record until a `put`. A diff of the chunk's records:
`python -X utf8 ~/.claude/skills/andromeda-tools/scripts/rec.py diff --root . 2026-11-23-model-and-effort-at-launch`.

## Goal
Every `viola run` / `viola revive` start hands its child `--model` and `--effort`:
- the given pair, else `config.json`'s `default_model` / `default_effort`, else the one compiled `DEFAULT_PAIR` (your
  F2);
- one half given takes the other from the default.

The pair is recorded as a new 17th, log-only kind `launch {model, effort}` after `lineage`, mirrored in the snapshot and
recovered by replay.

`viola revive` hands the recorded pair to the resumed child (your F6 head answer: a second ratified crossing beside the
session id), and an override on `revive` is recorded.

`list --json` and MCP `list` gain `model?` / `effort?`. The board is unchanged.

Not added: a ledger row, a live start, a settings write, or an environment read.

## Top acceptance criteria
- A start with no pair hands and records `DEFAULT_PAIR`. A start with a pair hands and records that pair. One half
  takes the other from the default (`path1_run_start_hands_the_default_pair`,
  `path1_run_start_hands_the_given_pair`).
- A bad config value takes that half of the default, with one `parse-rejected{config-json, malformed}` line, and
  disables no control.
- A bad effort, a bad model, or a pair typed after `--` (exact or `=` form) is usage exit 2: nothing created, no
  document. The pair-after-`--` line names `--model` and `--effort` as the flags to give `viola run` before `--`.
- A revive hands the recorded pair. An override is recorded. A recorded half that no longer parses takes the current
  default's half, with no refusal.
- Replay recovers the pair. A revive over a corrupt snapshot still hands it.
- `list --json` / MCP `list` carry the pair. The six columns are unchanged.
- Every counted-literal reader moves (steps 15–18), and the three harness layers print `"ok":true`.
- Three remove-the-guard controls run: `launch-last-wins`, `caller-words`, `revive-pair`.

## Touchpoints
- **New:** `src/run/launch.rs`.
- **Modified, 25 files:**
  - the agent crate's flags, `EFFORT_LEVELS`, `DEFAULT_PAIR` and `launch_args`;
  - core's `is_model_word` and `EventKind::Launch`;
  - state's snapshot, replay and board, with their two test binaries;
  - `src/run/mod.rs`, `src/cmd/run.rs`, `src/cmd/revive.rs`, `src/cmd/list.rs`, `src/obs.rs`, the config schema;
  - the fake agent and its option case;
  - the harness `start_records`;
  - nine test files.

## Test commands
22 entries:
- fmt and clippy;
- four crate units, each green at baseline: 39, 803, 290 and 1224 tests;
- three filtered new-case entries, each red-vacuous at baseline (exit 4, 0 selected), which carry the three controls;
- the ten-binary integration entry, green at baseline with 193 tests;
- the harness boot read, green at baseline with 12 tests;
- the host read, recorded QUIET;
- the three harness layers;
- the operator pass: boot / status / cleanup smoke, pre-push, hygiene, push, and `ci.py` 1800.

## Capability claims and the concretization preview
**None claimed** (your F1). v2-30 stays `planned, unclaimed`, its acceptance unchanged. One dated ledger note is
written after your word:

> 2026-11-23 (2026-11-23-model-and-effort-at-launch, phase P5): not claimed here (the operator's F1, `inputs#I2`).
> This chunk proves clauses 1–3 at the level of the argv handed and the record. The start hands `DEFAULT_PAIR` or the
> given pair, and a revive hands the recorded pair, each witnessed by the fake agent's receipt and the `launch` line.
> The real CLI running on the pair is unmeasured, owed to «The fleet on the founder's host». The open / up / adopt
> clauses wait for those verbs, and the claim waits for the last of them.

## Escalations the wrap will meet (playbook rows read: 4)
- **Your own answers, `inputs#I2`** (rul-bznresq5: one answer card at the wrap):
  - F2: the two `config.json` keys and `DEFAULT_PAIR`.
  - F6 head: the recorded pair onto the revived child's argv. This includes narrowing architecture ·
    decision/revive-resumes-in-place's «replays no launch» (see the letter-wrong list below).
- **A delegate's answers** (rul-r74hshif: the wrap's full escalation, landing provisional):
  - `revive --model` / `--effort` overrides onto the resumed argv (F6, the rest).
  - The pair crossing into `list --json` / MCP `list` (F5 (i)).
- **The MCP `list` crossing under W1** (your standing fork from the last wrap, on logged instance names): this
  chunk's `model?` / `effort?` are not W1's class. They are two closed words that name no instance. They land
  provisional on the delegate's word all the same, beside W1's conservative arm.

## Validation
- **Mechanical checks 1–12:** pass, with one item each for checks 3 and 6:
  - **Check 3:** it found a reached row the P3 block missed. It is now in the plan's expected amendments.
  - **Check 6 (WARN):** the plan prose runs about 417 lines (557 lines in all, the fence's about 140 not counted), just
    past the ~400 band.
  - **Rev 5 re-run:** dry-run 22 entries, exit 0 · planlint `hits: 0` · `scope lists: parsed 26 items` · inputs 3
    entries (I1 and I2 unchanged; I3 `n/a — a message has no live source`, cited 3 times) · baseline listing
    `seen green 5 · not seen 0 · new (key) 10 · leg 7`. Every new entry carries its `baseline`.
- **Check 9:** no runner-only bullet; the CI read is operator-leg.
- **Check 11 (b):** no recorded harm in «Lineage registry»'s report, from which the reused harness comes.
- **Validation-1:** aligned with `doc:…/scope` rev 3 and the working entry. One item is for you:
  - The `revive` override flags and the caller-words refusal go slightly past the scope's text. Both come from the
    answered forks (`inputs#I2`, the delegate's).
  - I read them as intent-incomplete-justified, and the scope is not amended. Say if you want them out.

## The directive's item-12 block (verbatim where a tool printed it)
**The union:**
```
architecture: rows 992 · packs 10 · picked 244 (142 by every draw) · standing 31 · 68206 B
security-plan: rows 637 · packs 7 · picked 186 (130 by every draw) · standing 12 · 53807 B
design-system: rows 326 · packs 4 · picked 146 (91 by every draw) · standing 11 · 34408 B
layout-templates: rows 227 · packs 3 · picked 68 (29 by every draw) · standing 2 · 14975 B
test-plan: rows 1374 · packs 14 · picked 298 (171 by every draw) · standing 56 · 74573 B
obs-plan: rows 1040 · packs 11 · picked 268 (154 by every draw) · standing 31 · 63908 B
a11y-plan: rows 238 · packs 3 · picked 122 (75 by every draw) · standing 2 · 31996 B
rows-union: 7 master(s) · picked 1332 of 4834 · rule any
```
- The rows files sum to 341 873 B, past 250 000 B, so P4 read them by search, never whole.
- **Every cited row:** `cited handles 53 · in a rows file or standing list 44 · in neither 9`:
  - **7 are rows the plan proposes as new (+):** `contract/child-argv/launch-pair`,
    `contract/config-json/default-model`, `contract/event-data/launch`, `contract/instance-snapshot/launch`,
    `contract/list-item/model`, `contract/normalised-event-kinds/launch`, `contract/viola-cli-arguments/run-model`.
  - **2 exist and were looked up small with `rec.py find`:** security-plan ·
    `decision/lineage-parent-authenticates-nothing` (`dec-otvnh2p3`) and a11y-plan ·
    `decision/start-verify-stop-grace-off-the-human` (`dec-pylglj5m`).
- **The round's pre-flight:** none. The plan holds no round or live entry.
- **`[RETYPED]` lines:**
  - `security-pick-p6.d3 · [RETYPED] line 17: contract/channel-faults-sanitised is no row of the pack — read as decision/channel-faults-sanitised, the pack's row 17`
  - `tests-pick-p14.d2 · [RETYPED] line 15: decision/chaos-home-outside-scans is no row of the pack — read as decision/chaos-home-outside-scan, the pack's row 15`
  - `obs-pick-p9.d3 · [RETYPED] line 70: decision/test-homes-kept-until-gates-run is no row of the pack — read as decision/test-homes-kept-until-gates, the pack's row 70`
- **Re-spawns:** `layouts-history · [CITES] UNRESOLVED: 2026-10-16-resume-and-compact-ledger-rows — history:34`. It
  was re-spawned once as `history-r1` and passed (`VERDICT: 1 of 1 pass`). The twin is `.raw-layouts-history.md`.
- **Non-zero exits and refusals:**
  - The three baseline reds are the predicted vacuous ones: `7 … red · exit 0 ✗ (exit 4)`,
    `8 … red · exit 0 ✗ (exit 4)`, `10 … red · exit 0 ✗ (exit 4)`, each `error: no tests to run`.
  - The Bash guard refused:
    - three `cd` calls (`Blocked: cd (…) moves the session cwd for every later call`), each re-issued with absolute
      paths;
    - one pipe into a filter (`Blocked: a pipeline tool's output piped into a filter`), re-issued bare.
  - The Grep tool is not available in this session (`No such tool available: Grep`), so grep went through Bash.
  - No `UNPARSED`, `INDETERMINATE` or `NOT DERIVED` line was printed by any tool.
- **Where a letter was wrong or silent, or I was:**
  - (a) My P3 reached-row sweep missed architecture · decision/revive-resumes-in-place («replays no launch»): its regex
    keyed on `revive-in-place`. P5 check 3's `rec.py find` for `launch` found it. It is added to the expected
    amendments, and its narrowing rests on your F6 answer.
  - (b) `gate.py show --n N` reads only the last `gate.py run` call's firing. For an entry fired in an earlier call it
    prints `not run — excluded … shown: none`, so the test counts for entries 6 and 9–11 were read from their logs under
    the gate's temp folder. The letter says `show` reads the trail and is silent on this.
  - (c) During picker part 1's wait I wrote a scratch script to the session scratchpad and ran `claude --help`. The
    batch-wait letter allows reads only. Both are recorded in the distill checkpoint.
  - (d) At P3's closure I dropped the fake agent's start receipt from the scope's surfaces so the correction would not
    re-run the row read. It stands in research's file list and is recorded in the research checkpoint.

## Beyond the plan
1. **A CARRY on «The fleet on the founder's host»** (F8, `inputs#I2`): the real-CLI reading of the pair. It covers
   whether a session started with `--model` / `--effort` runs on them, and whether a session resumed with them does.
   The resume case and effort are unmeasured today; the model at start is measured once, from a fixture (research M2,
   M3).
2. **v2-30's claim left to the last of «`viola open`», «`viola up`» and «`viola adopt`»** (F1, `inputs#I2`), with the
   dated ledger note above. There is no route insertion.
3. **A CARRY on «Version done-check»**, the entry that owns the next product re-pin (`inputs#I3`, your review of card
   1):
   - The host's launcher, a script outside this repository, starts every wrapped session as
     `viola run <name> -- claude --effort E --model M …`, the pair after `--`.
   - Before a build holding this chunk is pinned, the launcher moves the pair to `viola run --model … --effort …`.
   - Otherwise each such start is this chunk's usage exit 2, whose fixed stderr line names those two flags.
   - No step of this chunk touches the script.

Apply? (yes / review / cancel)
