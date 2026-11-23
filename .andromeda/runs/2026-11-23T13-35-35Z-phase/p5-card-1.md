# P5 review — 2026-11-23-seam-requests-for-viola-open «Seam requests for viola open»

The plan to open in an editor: `.andromeda/cache/docs/2026-11-23-seam-requests-for-viola-open/plan.md` (rev 3, rendered
2026-11-23T14:13:05Z). No plan stands in a chunk's folder; writing in that file changes no record until a `put`.
`python -X utf8 ~/.claude/skills/andromeda-tools/scripts/rec.py diff --root . 2026-11-23-seam-requests-for-viola-open`
is a diff of the chunk's records.

## Goal
The Hyprland seam's closed set grows by two requests: a read-only `monitors` query, and ONE move-to-workspace dispatch
on a window address, `hl.dsp.window.move`. The carried stub name `hl.dsp.window.move_to_workspace` was corrected at
P3: it is an event name, not a dispatcher (research M6). Each request gets a closed argv, a parse, a recorded fixture
and a replay through the fake. The 5 s read deadline is settled (F8) with a test that times the public read against it.
The `hyprctl reply` Boundary row gets a witness that drives the drain past its cap. No new closed word, subject, parser,
env read or fake mode is added, and the seam keeps no product caller.

## The one stop
Implement builds and greens steps 1–19, R3 included (step 19, entry 3: `record-monitors`, read-only, no window), and
stops. **R1 is the operator pass's, not implement's** (step 20). In order:
- entry 17 `hyprctl locked`, which must read `false`;
- entry 18 `record-move … --workspace <id>`, where `--id` is the workspace you name. It opens ONE foot running
  `/usr/bin/sleep 30` with no claude in it, finds that window by its pid, moves it once, reads it on the workspace,
  kills and reaps that pid, and writes `MoveToWorkspace.recorded-1.json` alone;
- entry 19, the file stands;
- entry 20, the readers: the crate's walk, the replay contract and the hygiene walk.

**Unbuilt behind R1:**
- the dispatch's recorded fixture;
- its replay case firing on it (the arms are built, but the walk reaches the file only once it stands);
- entries 19–20, which read it.

The chunk's wrap waits for R1. A refused R1 stops the pass, and the chunk comes back as a phase revision plus an
implement re-entry. Reading the raw refused reply would take a second live dispatch, which is a reading beyond R1 and
yours to grant.

## Top acceptance criteria
- The closed set holds 8 requests: the move is ONE Lua element with only the closed address and the i32 in it.
- `monitors` parses into numbers and bools alone, with its canary absent.
- The timed read returns `unreachable` no sooner than 5 s, and the `deadline-5s` control holds.
- The drain keeps exactly `MAX_FRAME + 1` bytes while the child still writes; the `drain-cap` control holds and the
  Boundary row names that case.
- The recorder keeps no `monitors` string, and the `monitors-keep-none` control holds.
- R3's file is recorded, replayed byte for byte and hygiene-walked.
- R1's verb is rehearsed over planted stubs.
- The harness layers, `pre-push` and CI all read green. After R1: the recorded move and its readers.

## Touchpoints (research rev 1, `scope lists: parsed 15 items`)
- New: `fixtures/hyprland/0.56.2/Monitors.recorded-1.json` · `…/MoveToWorkspace.recorded-1.json` ·
  `fuzz/corpus/hyprctl_json/monitors-row`.
- Modify:
  - `crates/viola-hyprland/src/{lib,reply,read}.rs` and `proptest-regressions/reply.txt` (only if a seed is found);
  - `crates/viola-e2e/src/bin/viola-hypr-tap.rs`, `crates/viola-e2e/tests/hypr_tap.rs`;
  - `tests/contract_{bounded_readers,hyprland_seam,fixture_hygiene}.rs`;
  - `schemas/hyprland-fixture.v1.json`, `fuzz/fuzz_targets/hyprctl_json.rs`.

## Test commands (22 entries)
- **The block:** fmt · clippy · R3 (operator, implement) · R3-stands · the seam crate · `ctl-hypr` · the tap ·
  `ctl-tap` · the three contracts · the fuzz replay · the host read · `run --unit` / `--integration` / `--e2e`.
- **The operator pass:** pre-push · hygiene · the lock probe · R1 · R1-stands · the R1 readers · the push · the CI read.
- **Not run:** no round, no live entry, no claude start, no rig start, no mutation run. No boot smoke, because there is
  no boot-path touchpoint.
- **Controls:** 3 one-shot `gate.py control` runs, `--timeout 900`.

**Baselines on the untouched tree:**
- 4: red (file absent);
- 5: green, 84/84;
- 6: red, 0 selected;
- 7: green, 11/11;
- 8: red, 0 selected;
- 9: green, 135/135;
- 10: green, `"ok":true`;
- 11: recorded QUIET.

Entry 17's `last line false` has no must-pass reading at P5: the desktop read `true` here. Its first firing at R1 is
that reading, and a `true` stops R1 as designed.

## Capabilities
None claimed (carry `cry-q3ialp5i`), so there is no concretization preview. v2-07 and v2-08 stay with «`viola open`».

## Carries, each disposed by name
- `cry-ypbg67h6`: `monitors` by R3 · the dispatch by R1.
- `cry-bcosdllb`: by the timed read and the settled doc line (F8).
- `cry-ejfh3wh6`: by the new drain witness.
- `cry-q3ialp5i`: by this stop and R1.

## Expected amendments (wrap)
- architecture `decision/hyprctl-direct-spawn` gains the `hl.dsp.window.move` form. Pattern `rul-bznresq5`, your own
  F9.
- security-plan `decision/hyprctl-read-deadline` moves from provisional to settled (F8).
- security-plan `decision/hyprland-recorder-string-allowlist` covers the two words. Pattern `rul-bznresq5`, F9.

These reach the wrap as ONE answer card citing F9: no new question.

## Directive item 13 — verbatim where a tool printed it
- **Union lines:**
  - `architecture: rows 1008 · packs 11 · picked 57 (41 by every draw) · standing 17 · 21371 B`
  - `security-plan: rows 644 · packs 7 · picked 57 (36 by every draw) · standing 11 · 18081 B`
  - `design-system: rows 326 · packs 4 · picked 2 (1 by every draw) · standing 0 · 3061 B`
  - `layout-templates: rows 228 · packs 3 · picked 0 (0 by every draw) · standing 0 · 411 B`
  - `test-plan: rows 1374 · packs 14 · picked 118 (72 by every draw) · standing 17 · 31614 B`
  - `obs-plan: rows 1041 · packs 11 · picked 78 (37 by every draw) · standing 38 · 20798 B`
  - `a11y-plan: rows 238 · packs 3 · picked 9 (7 by every draw) · standing 11 · 4452 B`
  - `rows-union: 7 master(s) · picked 321 of 4859 · rule any`
- **Cited rows:** every row the plan cites stands in its master's rows file (read whole: ~100 KB, under the 250 000 B
  bound). No row is cited from a standing list alone.
- **Round pre-flight:** none. The plan holds no `round` or `live` entry.
- **`[RETYPED]`:** `tests-pick-p9 · [RETYPED] line 34: decision/synthetic-fixture-text-only is no row of the pack — read
  as decision/synthetic-fixture-text, the pack's row 34`. No re-spawn: 7/7 tasks, 159/159 pickers and 6/6 histories
  passed on the first try (`draws-dead:0`).
- **Non-zero exits and refusals:**
  - `inputs.py snap --dry-run` on the sealed `split-carry.md`, exit 3, `--source: absent`;
  - the Bash guard refused a `cd` into the run dir (re-issued as a subshell), and a `rec.py find` loop piped into a
    filter (re-issued bare);
  - baseline reds: entries 4 (exit 1), 6 and 8 (exit 4, `no tests to run`), each the predicted red.
  - No `UNPARSED`, `INDETERMINATE` or `NOT DERIVED` line was printed.
- **Where a letter was wrong or silent:**
  - (a) No form snaps a sealed run's file. The directive asked for the snap, but `inputs.py snap` reads only an on-disk
    source and refuses one under `.andromeda/`. `split-carry.md` is cited by its seal instead
    (`.andromeda/db/sealed/2026-11-23T12-02-15Z-phase.parquet`, read by `rec.py runs --kind '2026-11-23T12-02-15Z-phase'
    --file split-carry.md --show`, 18232 B).
  - (b) Check 4 (8) cannot order R1's regenerator ahead of entries 5 and 9, which walk the whole set; the prose records
    it.
  - (c) Check 4 (7) has no arm for a regenerator of a committed fixture; the prose records why `--out fixtures/hyprland`
    is no report.
  - (d) The directive names «rec v1.54»; rec prints `v1.56`. Gate `v1.39` and fanout `v52` agree.
  - (e) Two of my own `code-graph.py` calls went through `| tail` against the no-filter rule. The guard did not refuse
    them, and the trace file holds the whole result.

## Beyond the plan
- The a11y time-bound rows (`decision/time-bounds-fall-on-the-driver` / `-never-on-human`) do not list the settled
  `HYPRCTL_DEADLINE`: a bound with no product caller is not listed (a11y history n=5). Owner: «`viola open`», the seam's
  first product caller, which lands the row beside its own bounds (F8: «each new bound lands as a row of its own»).

Apply? (yes / review / cancel)
