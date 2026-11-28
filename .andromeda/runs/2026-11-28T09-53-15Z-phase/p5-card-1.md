# P5 review card 1 — chunk 2026-11-28-fleet-console-view, «Fleet console view»

Written 2026-11-28T11:52:18Z. The operator is away (inputs#I1); this card waits for his word through the overseer.
**The plan:** `doc:2026-11-28-fleet-console-view/plan` rev 5, its render at
`.andromeda/cache/docs/2026-11-28-fleet-console-view/plan.md` to open in an editor. No plan stands in a chunk's folder;
writing in that file changes no record until a `put`. `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/rec.py
diff --root . 2026-11-28-fleet-console-view` is a diff of the chunk's records.

## Goal
`viola console` shows the fleet: the board's lineage forest, each row's state in the board's words, the wheel, CLI,
and a console-only `LAST` cell (the age of the row's newest event-log line); every cell row-escaped; a row cut at the
width ending `...`; a fleet past the frame ending `... <n> more`; the board re-read once a second and redrawn on a
change; the agents read at most every 5 s off the loop. `viola list`, `list --json` and the MCP list item stay
byte-identical. One implement window (his F8).

## Top acceptance criteria (the plan holds 12)
- the six captions in their slots then `LAST`; the forest order and indent (`fleet_caption_row_is_the_six_captions_then_last`, `fleet_rows_keep_the_forest_order_and_indent`)
- the board's words, «stalled» = `stale`, `LAST` in one unit (`fleet_state_words_are_the_boards`, `fleet_last_cell_reads_the_age_in_one_unit`)
- no raw C0/C1/DEL in the frame (`fleet_cells_escape_every_control_character`, control `fleet-escape`)
- the cut and the more line (`fleet_cut_past_the_width_ends_in_dots`, `view_past_the_rows_ends_in_the_more_line`, control `fleet-cut`)
- redraw on a change, not while it stands (`console_run_redraws_when_the_view_changes`, control `view-redraw`; e2e `console_draws_a_booted_wrapper_and_its_end`)
- the agents read off the loop (`console_reads_the_agents_list_off_the_loop`); the skip line on a change only (`fleet_source_logs_the_skip_line_only_when_the_count_changes`)
- a home widened mid-run: exit 1, the home pair, `process-exit` `strict-modes-failed` (`console_ends_one_on_a_home_widened_mid_run`)
- `list --json` unchanged (`board_read_item_never_serialises_last_activity`, `cli_list`, `cross_list_rows`)

## Touchpoints
New: `src/console/view.rs`, `src/console/fleet.rs`, `src/human/fleet.rs`. Modify: `crates/viola-state/src/board.rs`,
`src/cmd/list.rs`, `src/cmd/console.rs`, `src/console/mod.rs`, `src/console/frame.rs`, `src/human/console.rs`,
`src/human.rs`, `src/human/board.rs`, `src/human/board/rows.rs`, `src/human/board/cells.rs`,
`crates/viola-core/src/obs.rs`, `schemas/diag-line.v1.json`, `tests/tui_console.rs`.

## Test commands (21 entries; 9 new, each baselined)
fmt · clippy · state crate (green 328) · its 3 new cases (red, 0 selected) · root bins (green 1446) · the view's new
cases (red, 0 selected) · `tui_console` (green 8) · its 3 new cases (red, 0 selected) · `cli_list | cross_list_rows`
(green 31) · the schema arm's required list (red `cols,rows,fits`; must-pass over a clone green) · the host read
(recorded, QUIET) · `run --unit` · `run --integration` · `run --e2e` · operator: smoke trio, pre-push, hygiene, push,
`ci.py conclusion --sha HEAD --wait 1800`. No round, no live entry, no real `claude`, no compositor call, no mutation
run.

## The concretization preview — matrix#v2-14 (claimed GATED, his word inputs#I3)
- **Now:** «On the founder's Hyprland host the console opens as a tiled window, stays whole and usable at a quarter of
  the screen, and shows the lineage trees, each session's state (working, idle, waiting on a dialog, stalled) in the
  board's words, the wheel holder and the last activity, kept current as the fleet changes.» · method `e2e`.
- **Proposed:** «Over the fake agent, `viola console` draws its home's instances as the board's lineage forest, each
  row's LIVE, STATUS, WHEEL, DIALOG and CLI in the board's words — busy (working), idle, `DIALOG <kind>` (waiting on a
  dialog), `stale` (stalled) — and a `LAST` cell with the age of the row's newest event-log line, every cell
  row-escaped, the board re-read once a second and the frame redrawn when it changes (the plan's `fleet_`, `view_` and
  `console_run_` unit cases and `tui_console`'s three state-read cases). GATED: on the founder's Hyprland host, at a
  moment he names, the console opens as a tiled window, stays whole and usable at a quarter of the screen, and keeps
  the fleet current — his live reading; no stand-in, stub or recorded artifact verifies it.» · method `e2e`.
- **Gated file** (one line, the block verbatim): `BLOCKED-ON: the operator's live reading of viola console on his
  Hyprland host — the window opens tiled, stays whole and usable at a quarter of the screen and keeps the fleet
  current — at a moment he names (inputs#I3)`. Its route half: a `BLOCKED-ON` carry on this chunk's frozen line
  (route-contract §Write row 26), written after the word, before the claim.
- **Dated note on the claim:** `2026-11-28 (2026-11-28-fleet-console-view, phase P5): «stalled» in the acceptance and
  «a stalled turn» in the observed gap are read as the board's stale, a wrapper whose heartbeat is older than 5 s,
  on the operator's own F3 (inputs#I3).`
- **«usable» / «keyboard-driven» — my reading: it does NOT keep v2-14 pooled.** v2-14's acceptance asks the view
  whole and usable at a quarter screen; the console is keyboard-driven as built (raw-mode keys, `q`, Ctrl-C). Reaching
  every verb by keyboard is matrix#v2-15's own requirement and acceptance, «Console actions»'s. If he reads «usable»
  as «drivable», the claim waits for «Console actions» instead — his to say.
- **«in the Omarchy look»** (the requirement, not the acceptance): the console draws plain ASCII in the terminal's own
  font and colours (`decision/console-own-surface`); the look is read at his live reading.

## Every wrap amendment the plan expects, and whose word it lands on
| # | Master · row | Change | Whose word |
|---|---|---|---|
| 1 | design-system · decision/fleet-console-not-verb-output | reworded: «The terminal fleet console (`viola console`), its own surface, is governed by no verb-output rule» | his own, answer (a), inputs#I3 — escalate rul-r74hshif / rul-p2zl2kgl, the wrap halts once on his answer card (rul-bznresq5) |
| 2 | layout-templates · decision/no-prompt-or-pager | reworded the same way | his own, (a) — as 1 |
| 3 | a11y-plan · decision/static-linear-output | console exception gains «…or the fleet it shows changes» | his own, F1 — as 1 |
| 4 | a11y-plan · decision/cli-no-interactive-input | console exception gains «…or a change in the fleet it shows» | his own, F1 — as 1 |
| 5 | obs-plan · decision/symlink-skip-logged-as-count | a named console exception beside `viola up`'s: the line when the count changes | his own, F7 |
| 6 | layout-templates · decision/shared-captions-fixed | «…and the console adds one caption of its own, `LAST`, after them» — **reached**: the row binds the console by name, so it is no verb-output row and (a) does not scope it off | his own, F2 |
| 7 | design-system / layout-templates · Surface: console, new rows | the body lines, the caption row with `LAST`, the age form `12s · 4m · 2h · 3d` (`unknown`, `n/a`), the row cut ending `...` never inside an escape, `... <n> more`, the 1 s re-read and redraw on a change, the 5 s agents read off the loop | his own, F2 · F4 · F5 · F6 |
| 8 | the same area | a mid-run refused scan: restore, `process-exit` `strict-modes-failed`, the home pair, exit 1 | the plan's lean (step 7) on (layout-templates · contract/cli-verbs/console) «1 on a refusal» and (security-plan · contract/strict-modes-refusal/console)'s pair — not his word; his to overturn |
| 9 | obs-plan · contract/diag-line-fields/console-frame | gains `shown` and `hidden` (counts, required) | the overseer's technical T2 — delegate viola-overseer, PROVISIONAL |
| 10 | obs-plan · contract/diag-subject/agents-probe; security-plan · decision/claude-agents-resolved-from-path; a11y-plan · decision/claude-agents-read-fails-open; architecture · decision/agents-join-read-ungated | each names the console's agents read | his own, F6 (he was told it spawns the real CLI every 5 s) — a new subprocess crossing, escalate rul-r74hshif, his answer card (rul-bznresq5) |
| 11 | matrix#v2-14 notes | «stalled» read as `stale` (the dated note above) | his own, F3 — written with the claim at P5 step 4, not by the wrap |

**F4's cut — the reading I take:** no named console exception lands beside the truncation rows. (design-system ·
decision/truncate-only-unwrapped-name), (layout-templates · decision/truncate-only-unwrapped-names), (design-system ·
decision/no-hardcoded-widths-or-wrapping), (a11y-plan · decision/name-truncation) and (a11y-plan ·
decision/unwrapped-name-truncation) govern the human tables of `viola list`, a verb's output; his reworded (a) puts the
console under no verb-output rule, so they are scoped off and the console's cut lands as its own surface row (row 7).
Should he read the a11y pair as surface-wide, the conservative arm is a named console exception on each.

**F4 with F2:** `LAST`, placed after CLI by F2, is the first cell the cut reaches, then CLI. I read his F4 «the CLI
cell cut first» as «the last cells first». His to correct.

## Rows reached and read as not reached (named, his)
- **Log volume — in none of his answers, his to rule, nothing lands:** the 1 s view writes into the never-rotated
  `console.ndjson` one `console-frame` per redraw (each second while any row's `LAST` is under a minute, so about
  3 600 an hour), one `state-recovered` per replayed instance per read (obs-plan ·
  contract/state-recovered-detail/snapshot-replayed «one line per snapshot read call»), and one `agents-probe` pair per
  agents read (about 720 pairs an hour). Each is what its row allows as written; (obs-plan ·
  decision/no-info-in-hot-loops), (obs-plan · decision/no-per-heartbeat-logging) and (obs-plan ·
  decision/heartbeat-file-not-logged) read as not reached (the PTY pump and the heartbeat), but they warn against
  this shape. The plan builds as the rows stand.
- (design-system · decision/no-spinner) «no elapsed-time counter» and (layout-templates · decision/no-spinner): a
  ticking `LAST` reads as one; scoped off by (a)'s reworded rows. Rests on (a).
- (architecture · decision/wait-blocks-nothing-polls) and (architecture · absence/polling-endpoints): read as not
  reached — they govern `wait` and endpoints; the console polls its own disk read, as the scaffold already polls the
  size every 100 ms.
- (architecture · overview/project-intent) «a keyboard-driven terminal fleet console is planned, not built»: left for
  «Console actions», where the console becomes drivable.
- The eight verb-output rows of the wrap's fork (b), «leave them», stay scoped off by (a): a11y
  `decision/linear-static-output`, `decision/linear-static-cli-output`, `decision/no-spinners`; layout-templates
  `decision/no-persistent-footer`, `decision/human-tty-expression-level`, `decision/only-header-bay-line`,
  `decision/plain-output-modes`, `contract/cli-token-rendering/motion-tokens`.

## The union, verbatim (the row read re-run once on the corrected scope, T1)
```
architecture: rows 1049 · packs 11 · picked 129 (72 by every draw) · standing 25 · 38812 B
security-plan: rows 670 · packs 7 · picked 113 (76 by every draw) · standing 19 · 33629 B
design-system: rows 366 · packs 4 · picked 80 (37 by every draw) · standing 10 · 18701 B
layout-templates: rows 241 · packs 3 · picked 36 (21 by every draw) · standing 3 · 7857 B
test-plan: rows 1391 · packs 14 · picked 210 (129 by every draw) · standing 80 · 54904 B
obs-plan: rows 1062 · packs 11 · picked 196 (114 by every draw) · standing 29 · 48089 B
a11y-plan: rows 244 · packs 3 · picked 44 (24 by every draw) · standing 16 · 13969 B
rows-union: 7 master(s) · picked 808 of 5023 · rule any
```
The sum, 215 961 B, is under 250 000: the rows files were read whole. **Every row the plan cites is in the union or
its standing list:** 28 cited, 28 found (a scratch read of the plan's `({doc} · {handle})` citations against the run
dir's rows and standing files; its first pass found 4 written with no master or a short one, re-spelt in rev 5).

## planlint, whole (rev 5)
```
planlint v1.25 · a61c2794
fence: 21 entries · plan: doc:2026-11-28-fleet-console-view/plan · research: doc:2026-11-28-fleet-console-view/research
check 9 · CI-outcome reads: 1 · leg operator exempt: 1 · Platform issues consulted: none · `none — no runner-only bull…`
check 9 · runner-only scope bullet: not read — the letter's
check 4 (5) · leg = 'live' entries: 0 · Harness rules consulted: none · `none — no live leg in this chunk's first pass…`
check 4 (5) · leg entries with a content atom: 4 · note without `atom from`: 0
check 4 (13) · operator CI-outcome reads: 1 · not ci.py conclusion: 0
check 4 (13) · job-log reads by pattern: 0
check 4 (14) · base-less git change reads outside the operator and round legs: 0
check 4 (15) · grep → wc -l counts asserting exit 0 and last line 0: 0
check 4 (16) · guards whose exclusions leave a new file in: 0 of 0 with an exclusion
check 4 (18) · entries reading a master's or a base's path the store now holds: 0
check 4 (19) · absence probes against the test names their own plan adds: 0 · probes read 0
check 4 (17) · entries reading a path the wrap writes: 0 of 13 asserting — listed, P5's read
check 4 (17) · a path the wrap writes, read from inside a program an entry runs (a suite, the ledger's tool): not read
check 2 · ledger-note entries (matrix#): 0
check 2 · entries naming no master and no matrix#: 0
check 5 · placeholders outside code: 0
check 12 (c) · step references the plan's steps lack: 0 of 7
check 12 (d) · «unchanged» claims on a name another part of the plan names: 7 of 9 — listed, P5's read
  line 24 · «byte-identical» · crates/viola-state/src/board.rs (step 1) · forest (step 3) · board_rows (step 3)
      src/human/board/rows.rs (step 3)
  line 58 · «unchanged» · src/human/board/rows.rs (Files to modify) · board_rows (Files to modify)
  line 92 · «unchanged» · count (step 2) · crates/viola-core/src/obs.rs (Files to modify) · obs.rs (Files to modify)
  line 130 · «unchanged» · src/cmd/list.rs (Test Commands) · board::read (step 2)
  line 137 · «unchanged» · src/human/board/rows.rs (step 3) · board_rows (step 3)
  line 153 · «not touched» · src/cmd/list.rs (Files to modify)
  line 356 · «unchanged» · last_activity (step 1)
  a claim on a name no other step, Files to modify or Test Commands names is not listed — P5's read
check 4 (4) · criterion evidence paths the plan names nowhere else: 0 of 0
hits: 0 — resolved by its check (validation.md 2 · 5 · 9 · 4 (4), (5), (13)–(16), (18)–(19) · 12 (c)); 0 is not a pass
exit=0
```
12 (d)'s readings: each «unchanged» or «byte-identical» names an OUTPUT the code change must not move (the board's
lines, `list --json`, the event enums) beside code the plan does change: true on both sides, not a contradiction.

## Re-spawns and RETYPED lines
- Take-up row read: `arch-pick-p6.d2 · [ABSENT]` → re-spawned once, `pick-p1-r1` passed.
- T1 re-run: `design-task · [ABSENT]` → `task-r1` passed; `design-pick-p4.d3 · [ABSENT]` → `pick-p1-r1` passed.
- `[RETYPED]` (take-up run): `tests-pick-p3 line 58: decision/shared-e2e-helpers-never-copied → decision/shared-e2e-helpers`;
  `tests-pick-p10 line 35` and `tests-pick-p10.d2 line 66: decision/external-processes-real-stub-executables →
  decision/external-processes-real-stubs`.
- `[RETYPED]` (T1 re-run): `arch-pick-p9.d2 line 89: decision/verify-probe-dirs-drop-guard → …-drop-guarded`;
  `design-pick-p1.d2 line 18: decision/no-spinner-or-elapsed-time-counter → decision/no-spinner`;
  `tests-pick-p10 line 35: decision/external-processes-real-stub-executables → decision/external-processes-real-stubs`;
  `obs-pick-p8.d3 line 64: decision/run-worker-panic-ends-session → decision/run-worker-panic-ends-run`;
  `obs-pick-p11 line 93: contract/diagnostics-line/corr-on-dialog-and-hook-lines → contract/diagnostics-line/corr-on-dialog-lines`.

## Every non-zero exit, refusal and UNPARSED / INDETERMINATE / NOT DERIVED line
- new-session's health call: `records · INDETERMINATE: rec.py check exited 3 — rec.py check: timed out after 120 s`;
  the same `rec.py check --no-heal` run on its own read `check: clean`.
- Bash guard refusals, each re-issued once in the form it passes: a `cd` into the run dir (re-run in a subshell); a
  gate tool's output piped into `grep` (re-run bare).
- Baselines, red as the plan predicts: entry 4 exit 4 (0 of 328), entry 6 exit 4 (0 of 1446), entry 8 exit 4 (0 of
  8), entry 10 last line `cols,rows,fits`.
- Entry 6's first spelling read GREEN on the untouched tree (`view_` selected
  `run::window::tests::window_step_restores_nothing_when_the_view_stayed`): re-spelt anchored to `tests::`, re-baselined
  red.
- Entry 10's first spelling (`grep -c "\"shown\""`) read `last: '2'` over the must-pass clone (two lines hold the
  word): re-spelt to the jq read of the required list; controls in `.andromeda/runs/2026-11-28T09-53-15Z-phase/controls-p5.md`.
- `grep -c` probes exiting 1 on no match during research (the absence reads: `notify`/`inotify` in `Cargo.lock`, the
  `v2-14` route grep), each read as the count 0. No `UNPARSED`, `INDETERMINATE` or `NOT DERIVED` line from any phase tool.

## Where a letter was wrong or silent
- The scope pinned at the take-up put the console loop in `src/cmd/console.rs`; it is `src/console/mod.rs`. The
  letter then re-runs the row read whole; the directive's sequencing (T1, the overseer's) ran it once, after his
  answers folded.
- T4's «planted instance dirs for the size cases» met (test-plan · decision/product-state-from-real-runs) «never
  hand-written» outside viola-state's own unit tests: the plan keeps the size cases at unit tier over a built `Board`
  and boots fake wrappers end to end (plan, Constraints).
- `fanout.py census` counts both runs' re-spawns together (3); the letter is silent on a re-run's census.
- `rec.py find --text stalled` matches `installed` (a substring search); the board-word absence was read from the hits.
- The relay asked whether «usable» / «keyboard-driven» keeps v2-14 pooled; the letter's P4 lean rule (a requirement
  naming a surface this chunk does not build → no claim) would read «keyboard-driven» against this chunk — my reading
  above, his to decide.
- His F2 and F4 meet at the cut's order (`LAST` after CLI): the plan reads F4 as «the last cells first».
- The phase letter prompts for the word here; under the directive no dialog is raised and the card waits for his
  word through the overseer.

## Beyond the plan
- **The console's log volume** (`console-frame` per redraw, `state-recovered` per replayed instance per read, the
  `agents-probe` pair per agents read, in the never-rotated `console.ndjson`): no owner proposed; his to rule — the
  rows as written allow each.
- **An agents child outliving a quit** (up to its 5 s bound, the thread not joined so no key waits): accepted by the
  plan as built; no owner proposed.

Apply? (yes / review / cancel)
