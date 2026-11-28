# P5 review card 1 — «Console actions» (2026-11-28-console-actions)

Written 2026-11-28T14:49:32Z. The plan is `doc:2026-11-28-console-actions/plan` rev 3, rendered for an editor at
`.andromeda/cache/docs/2026-11-28-console-actions/plan.md`. No plan stands in a chunk's folder. Writing in that file
changes no record until a `put`. `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/rec.py diff --root . 2026-11-28-console-actions`
is a diff of the chunk's records.

## The plan in brief
- **Goal.** From a selected wrapped row, by keyboard alone, the console runs v2-15's eight verbs plus switch. Each runs
  as a direct child `viola` of the console's own exe, with `--home` passed and `VIOLA_NAME`/`VIOLA_DIR` removed (his
  F1), its output captured through `MAX_FRAME` and drawn escaped. One child runs at a time, off the key loop.
- **Two parts, one entry** (relay item 2). There is a planned stop between them, and the overseer calls it at the seam.
  - **Part 1, steps 1–9:**
    - the byte-level decoder with a 50 ms lone-ESC wait;
    - the selection over wrapped rows by name, and the scroll with `... <n> above` / `... <n> more`;
    - the child runner and the `console-action` event, `verb` and `exit_code`, every schema and pinned list named;
    - the `y`/`n` confirm for release and revive;
    - `l` last · `p` pause · `r` release · `f` focus · `v` revive (as `viola up <name>`).
  - **Part 2, steps 10–15:** the one editing line, `s` send and its readback, `m` switch, `o` open a child, and `a`
    answer, which is HELD behind Q8.
- **What the tree holds at Part 1's end:**
  - steps 1–9 built;
  - every part-1 entry green (3, 4, 5, 7, 8, 10, 12) and the unit, integration and e2e layers `"ok":true`;
  - no Part-2 key bound (`s` `m` `a` `o` read `unbound`).
  A tree that can be committed with every layer green.
- **Part 2 begins only** when his answers to **Q8** (how the console answers a dialog) and **Q9** (how the editing
  line shows non-ASCII text) are relayed verbatim and snapped (`founder-forks.md`, the P4 block).
- **Touchpoints:**
  - new: `src/console/action.rs`;
  - modify: `src/console/{keys,mod,view}.rs`, `src/human/{console,fleet}.rs`, `src/cmd/console.rs`,
    `crates/viola-core/src/obs.rs`, `schemas/diag-{line,detail}.v1.json`, `tests/contract_diag_schema.rs`,
    `tests/tui_console.rs`;
  - `crates/viola-state/src/board.rs`, Part 2 step 14 only.
- **Test commands:** 20 entries. 16 are fired by implement (fmt, clippy, the core crate, the root units whole, the
  part-1 and part-2 unit filters, `tui_console` whole and its two part filters, the four contract binaries, the list
  binaries for Part 2, the schema probe, the host read, and the harness's three layers). 4 are the operator pass's
  (pre-push, hygiene, the push, the CI read). 4 remove-the-guard controls are implement's one-shot runs: `env-strip`,
  `one-child`, `confirm`, `out-escape`.

## Leans the review may change (each a builder's reading, none his word)
1. `o` opens a child with the name typed on the one editing line, `--parent <selected>`, and everything else at open's
   own defaults. Its cwd is therefore the console's own folder, not the parent's recorded folder.
2. `m` takes one or two words: one of the five effort words is `--effort`, any other word is `--model`.
3. The key line keeps `q quit` first, so a cut never hides it, then the bindings.
4. A finished action's lines take the body's lower third at most, cut with `... <n> more`. Esc or the next action
   clears them.
5. A CR inside a pasted read ends the editing line, and the rest of that read is dropped.
6. A child running at a quit is not tied to the console's death: it runs to its verb's own end, and its output is
   dropped. This follows his F1 answer, which named this.
7. The relay's `d` adopt is **not taken up**. His F2 answer names v2-15's eight verbs plus switch, and adopt is not
   among them; adopt also needs an unwrapped row, which his F3 skips. Adding it would need his own word.
8. No row read re-run. Every surface the closed scope adds (child verbs, switch, answer, the console's lines) was named
   in the pinned scope. The task writers named no file, and new files sit inside the console module they read.

## Capabilities — concretization preview (the never-weaken checkpoint)
- **matrix#v2-15** (e2e) — current acceptance: «Every viola verb (open a child, focus, send, answer, last, pause,
  release, revive) is reachable from a selected session by keyboard alone, each acting through the same CLI verb with
  the same outcome and refusal as when typed at a shell.»
  Proposed concrete form, unweakened: from a selected wrapped row of `viola console` under an outer PTY, the key bytes
  of each verb's letter run that verb as a child of the console's exe. Each case's `console-action{verb, exit_code}`
  equals the exit of the same verb run at a shell on a twin instance, and the instance's records are the verb's own.
  The proof is the part-1 and part-2 e2e filters of `tui_console`.
  - **What the fakes prove:** every clause. The keys are real bytes into a real PTY, which is affordance level. The
    outcome and refusal are the verb's own over the fake agent, the fake foot and the fake hyprctl.
  - **What needs his live reading:** none of v2-15's own clauses. A real window raise or placement stays focus's and
    open's own gated rows (test-plan `decision/window-raise-proven-live`, `decision/window-placement-proven-live`).
  - **Claim:** only once Q8 is answered. **Arm 1** proves the answer leg whole, and v2-15 is claimed. **Arm 2**
    answers permissions and plans only, so claiming would weaken «answer», which is never done: v2-15 stays pooled
    with a dated note, unless he rewords its acceptance himself. **Arm 3:** not claimed.
- **matrix#v2-31** (integration) — **not claimed.** Its «takes effect for its next turn» is the real CLI's, carried to
  «The fleet on the founder's host» (its notes, and its route freight). This entry builds its console leg (`m`, step
  12), proven over the fakes: the start receipt carries the pair. A dated `matrix.py note` records it at step 4.
- **Gated criterion:** none. The entry carries no BLOCKED-ON, and no clause of its own waits on a live reading.

## The union (verbatim, `rec.py rows-union`)
```
architecture: rows 1049 · packs 11 · picked 374 (242 by every draw) · standing 36 · 98325 B
security-plan: rows 671 · packs 7 · picked 297 (211 by every draw) · standing 13 · 81272 B
design-system: rows 371 · packs 4 · picked 245 (184 by every draw) · standing 7 · 54835 B
layout-templates: rows 244 · packs 3 · picked 129 (92 by every draw) · standing 1 · 29052 B
test-plan: rows 1391 · packs 14 · picked 484 (305 by every draw) · standing 80 · 116619 B
obs-plan: rows 1063 · packs 11 · picked 521 (337 by every draw) · standing 20 · 123274 B
a11y-plan: rows 244 · packs 3 · picked 130 (94 by every draw) · standing 17 · 35753 B
rows-union: 7 master(s) · picked 2180 of 5033 · rule any
```
- **The rows files were read by search, not whole:** the union's sum is 539 130 B, past 250 000 B. Every row the plan
  cites was checked against the rows files and standing lists (a script over the plan's handles): **26 of 30 in the
  union**. The 4 outside it are the plan's own proposed rows: `contract/diag-line-fields/console-action`,
  `contract/fleet-console-keys/answer`, `decision/console-runs-verbs-as-children` and
  `decision/console-action-child-outlives-quit`.

## planlint (rev 3, whole)
```
planlint v1.25 · a61c2794
fence: 20 entries · plan: doc:2026-11-28-console-actions/plan · research: doc:2026-11-28-console-actions/research
check 9 · CI-outcome reads: 1 · leg operator exempt: 1 · Platform issues consulted: none · `none — no runner-only bull…`
check 9 · runner-only scope bullet: not read — the letter's
check 4 (5) · leg = 'live' entries: 0 · Harness rules consulted: none · `none — no live leg in this chunk (directive i…`
check 4 (5) · leg entries with a content atom: 3 · note without `atom from`: 0
check 4 (13) · operator CI-outcome reads: 1 · not ci.py conclusion: 0
check 4 (13) · job-log reads by pattern: 0
check 4 (14) · base-less git change reads outside the operator and round legs: 0
check 4 (15) · grep → wc -l counts asserting exit 0 and last line 0: 0
check 4 (16) · guards whose exclusions leave a new file in: 0 of 0 with an exclusion
check 4 (18) · entries reading a master's or a base's path the store now holds: 0
check 4 (19) · absence probes against the test names their own plan adds: 0 · probes read 0
check 4 (17) · entries reading a path the wrap writes: 0 of 15 asserting — listed, P5's read
check 4 (17) · a path the wrap writes, read from inside a program an entry runs (a suite, the ledger's tool): not read
check 2 · ledger-note entries (matrix#): 0
check 2 · entries naming no master and no matrix#: 0
check 5 · placeholders outside code: 0
check 12 (c) · step references the plan's steps lack: 0 of 9
check 12 (d) · «unchanged» claims on a name another part of the plan names: 0 of 5 — listed, P5's read
  a claim on a name no other step, Files to modify or Test Commands names is not listed — P5's read
check 4 (4) · criterion evidence paths the plan names nowhere else: 0 of 0
hits: 0 — resolved by its check (validation.md 2 · 5 · 9 · 4 (4), (5), (13)–(16), (18)–(19) · 12 (c)); 0 is not a pass
```
Rev 1 had 1 HIT (check 2: the amendments as a table; rewritten as lines, rev 2) and one check 12 (d) «unchanged» on
`src/cmd/console.rs` (reworded).

## The baselines (P5, each fired through the tool on the untouched tree)
- 3 green, 41 tests · 4 green, 1487 · 7 green, 11 · 10 green, 127 · 11 green, 31.
- 5, 6, 8 and 9 are red, exit 4 «no tests to run»: vacuous, 0 selected, until their steps write the cases.
- 12 is red: last line empty, no arm yet. Its literal is the plan's own, so it got a must-pass control over a
  `git clone --shared` at `a07542e9`: red unmodified, green with step 7's arm
  (`.andromeda/runs/2026-11-28T13-46-51Z-phase/controls-p5.md`).
- 13 recorded: QUIET.
- 1, 2, 14, 15 and 16 were `seen green` at the previous chunk's wrap.
- `inputs.py verify`: 2 entries, unchanged 2, every one cited.
- `gate.py scope --lists-only`: parsed 13 items (new 1 · modify 12).

## Log volume per hour at a 33-instance fleet (relay)
The plan adds no loop and no timer that writes, so every figure is independent of the fleet's size:
- `console-key`: one line a key read, bounded by the human's typing. A paste of N chars into the editing line writes
  N `text` lines.
- `console-action`: one line a finished child, bounded by one child at a time and the human's keys.
- The decoder's ESC wait, the runner's two reader threads and the child's wait write nothing.
- The child verb writes its own `cli-<name>.ndjson` lines, as at a shell.
- The agents thread's 720 pairs an hour stand unchanged.

## Wrap amendments the plan expects, and whose word each lands on
| Master · row | Direction | Whose word |
|---|---|---|
| design-system `decision/list-is-the-board`; layout-templates `decision/board-vs-strip` | the console acts on the selected strip through the verbs | his own, F1/F2 (inputs#I2) |
| security-plan + `decision/console-runs-verbs-as-children` | the child crossing — a **boundary widening** (`rul-r74hshif`) | his own answer to F1 after it was shown (`rul-bznresq5`: the wrap halts once on his answer card) |
| security-plan + `decision/console-action-child-outlives-quit` | a child may run past a quit | his own, F1 |
| design-system `contract/fleet-console-keys/unbound` + a row a bound key, the confirm prompts | F3 letters, `jk`/arrows, the release/revive confirm | his own, F3/F5 |
| design-system + `contract/fleet-console-keys/answer` · `/open` | `a`, `o` | delegate overseer (relay item 1), PROVISIONAL |
| design-system `contract/fleet-console-view/more-line` + `/above-line` | the body follows the selection | his own, F6 |
| a11y-plan `decision/cli-no-interactive-input`, `decision/cli-not-keyboard-interactive`, `decision/static-linear-output`; obs-plan `decision/no-interactive-terminal-interface` | the console's except: a selection redraw, a confirm, one editing line | his own, F3–F6 |
| layout-templates `decision/console-frame-floor` | `q quit` first in the key line | builder's lean (lean 3), PROVISIONAL |
| layout-templates `decision/console-own-surface` | the editing line's echo as typed | his own, Q9 (owed) |
| architecture `decision/option-text-never-in-human-line`; security-plan `decision/question-option-text-crosses-boundary` | only on Q8 arm 1 — a widening | his own, Q8 (owed) |
| security-plan `decision/console-key-reader-bounded` | the sequence decoder's classes, the 50 ms wait | delegate overseer (technical), PROVISIONAL |
| obs-plan `contract/diag-line-fields/console-key` + `/console-action`, an obs Decisions Log entry | the key words; the new event | delegate overseer (technical), PROVISIONAL |
| test-plan `decision/harness-log-fields-owned-by-test-plan` | console-action among the grepped fields | delegate overseer (technical), PROVISIONAL |

## Every [RETYPED] line and re-spawn
- `security-pick-p7` · `[RETYPED] line 77: decision/server-verified-before-any-channel-frame is no row of the pack — read as decision/server-verified-before-any-frame, the pack's row 77`
- `security-pick-p7.d3` · `[RETYPED] line 58: decision/server-verified-before-any-channel-frame is no row of the pack — read as decision/server-verified-before-any-frame, the pack's row 58`
- `tests-pick-p6.d3` · `[RETYPED] line 72: decision/hook-fails-open-silently is no row of the pack — read as decision/hook-fails-open, the pack's row 72`
- `tests-pick-p13.d3` · `[RETYPED] line 2: contract/harness-run/mutants-exit-<code> is no row of the pack — read as contract/harness-run/mutants-exit-reason, the pack's row 2`
- `tests-pick-p13.d3` · `[RETYPED] line 99: contract/local-live-failure/verify-exit-<n> is no row of the pack — read as contract/local-live-failure/verify-exit-n, the pack's row 99`
- `obs-pick-p8.d3` · `[RETYPED] line 64: decision/run-worker-panic-ends-session is no row of the pack — read as decision/run-worker-panic-ends-run, the pack's row 64`
- Re-spawns, each once, each passed: `layouts-pick-p3.d2` [ABSENT] (pick-p1-r1); `tests-pick-p1` [ABSENT] (pick-p2-r1);
  `obs-history` [CITES] `UNRESOLVED: 2026-11-17-cli-machine-contract` (history-r1; its raw twin `.raw-obs-history.md`).
- a11y-history carries 65 `not a marker` rows: it cites amendment ids (`amd-…`) in `marker=`. The check passed them as
  designed. Read, and no fix owed.

## Every non-zero exit, refusal, UNPARSED / INDETERMINATE / NOT DERIVED line
- No tool printed `UNPARSED`, `INDETERMINATE` or `NOT DERIVED`.
- **Baseline exits:** 4 on entries 5, 6, 8 and 9 (vacuous); entry 12 red with exit 0.
- **The Bash guard refused three calls,** each re-issued once in the passing form: a `cd` into the run dir, a
  `rec.py find` piped into a filter, and a `gate.py run` loop piped into `grep`.
- **One shell error of my own** (exit 2): a backtick inside a double-quoted `grep` pattern. Re-issued in single quotes.
- The `Grep` tool is not in this session, so the rows were searched with `command grep` through Bash.

## Where a letter was wrong or silent
- The relay's item 1 gives `d` for adopt, but his F2 answer («all eight of v2-15 plus switch») does not name adopt.
  The relay is wider than his word there, so it is not taken up (lean 7).
- His F3 option named no letters for answer and open. The relay supplies `a` and `o`, which are recorded as the
  delegate's word.
- v2-31's console leg is this entry's carry, but v2-31's acceptance holds a real-CLI clause that another route entry
  carries. The letters say «carry it here»; the ledger rule says no claim. Leaving it unclaimed with a note is the
  reading.
- The answer action's input form was left to research by the relay (item 3). It needs a widening on one arm, so it
  goes back as Q8, and the phase does not stand on it.

## Scratch left standing (named; removed by no recursive delete)
- `$TMPDIR/claude-1000/-home-turbolet-dev-projects-viola/a5bddab8-dcc1-4c25-a776-a7c56fd5d598/scratchpad/ctl-schema/`
  — the shared clone of the entry-12 control.

## Beyond the plan
- Q8 and Q9 (`founder-forks.md`, the P4 block) — his to answer. Part 2 waits on both, and so does v2-15's claim.
- No route entry is inserted (his F2/F7: one entry).
- adopt from the console is not owned by any entry. If he wants it, it is a new route entry on his word: its proposed
  title would be «Console adopt», placed after «Console actions».

Apply? (yes / review / cancel)
