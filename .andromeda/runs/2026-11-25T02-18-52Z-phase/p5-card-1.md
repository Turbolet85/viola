# P5 review card 1 — «Placing without moving the view» (`2026-11-25-placing-without-moving-the-view`)

Run `.andromeda/runs/2026-11-25T02-18-52Z-phase/` · the plan's render to open in an editor:
`.andromeda/cache/docs/2026-11-25-placing-without-moving-the-view/plan.md` (457 lines). No plan stands in a chunk's
folder; writing in that file changes no record until a `put`; `python -X utf8
~/.claude/skills/andromeda-tools/scripts/rec.py diff --root . 2026-11-25-placing-without-moving-the-view` is a diff of
the chunk's records.

## Goal
A window placed by `viola open`, `viola up` or `viola revive --origin` reaches its workspace with the operator's
active workspace unchanged (F4). The forks are built on these PROVISIONAL arms:
- **M1**: the window step reads the focused monitor's active workspace before and after its one move, and focuses it
  back only when the move took it. Same eight requests, no argv changed, no fixture re-recorded.
- **V1** (the folded item): verify refuses `untrusted-folder` and writes no stamp when its Run B–E first screen read
  the trust or imports form. A start in an untrusted folder then leaves the version un-attempted instead of locking it
  red.
- **W1**: one implement window.
- v2-07's claim is GATED on R7.

## Top acceptance criteria
- matrix#v2-07, gated on R7: `evidence/r7.md` holds `r7: green placement=placed active=unchanged` (entry 13, also the
  F4 predicate). Over the fakes, the window step's request sequence (entry 4).
- A failed monitors read, after-read or restore leaves the placement word as the move set it. The eight words are
  unchanged.
- The folded item: `verify_window_refuses_an_untrusted_folder_and_stamps_nothing` (with its trusted control) and
  `verify_window_start_from_an_untrusted_folder_leaves_the_version_unattempted` (entry 5).
- `untrusted-folder` is in the exit-cause table, the line schema and the error-body census (entry 6).

## Touchpoints
- New: `scripts/r7-place.sh`
- Modify: `src/run/window.rs`, `tests/cli_open.rs`, `src/cmd/verify.rs`, `tests/cli_verify.rs`,
  `tests/cli_version_gate.rs`, `tests/cross_exit_causes.rs`, `schemas/diag-line.v1.json`

## Test commands
22 entries:
- fmt and clippy;
- the root units (3), the filtered window-step cases (4);
- the new integration cases (5), the readers (6);
- the R7 parse (7) and rehearsal (8);
- the host read (9);
- the three harness layers (10–12);
- the gated reader of `evidence/r7.md` (13);
- the operator pass: boot smoke (14–16), pre-push (17), hygiene (18), push (19), CI (20), then R7's base-absent check
  and build (21) and R7 itself (22), fired once on his word.

Controls `win-restore`, `win-stayed` and `vfy-confound` are implement's one-shots, `--timeout 900` each.

## Concretization preview — matrix#v2-07 (never weakened)
- **Now:** «From the CLI and from an agent's own shell, an open brings up a new terminal window running the named
  wrapped session, placed beside its parent's window on Hyprland and left to the window manager elsewhere; it returns
  only once the session is live, records the window's identity and the open's origin, and names the calling agent as
  the parent when none is given. Over the fakes (tests/cli_open.rs): … Gated on R2 alone, for the real window placed
  beside its parent's on Hyprland: `evidence/r2.md` holds the line `r2: green placement=placed …`; a stand-in never
  verifies it.»
- **Proposed:** every clause above kept, word for word, and these added:
  - «… with the operator's active workspace unchanged;
  - over the fakes, the window step asks the focused monitor's active workspace before and after its one move and
    focuses that workspace back only when the move took it (`src/run/window.rs`
    `window_step_restores_the_active_workspace_the_move_took`, `window_step_restores_nothing_when_the_view_stayed`);
  - gated on R7 for the active-workspace clause: this chunk's `evidence/r7.md` holds
    `r7: green placement=placed active=unchanged` (R2's placement reading stands as cleared).»
- **The gated criterion's block, verbatim:** «R7, ONE viola open placing a window live at the operator host, unlocked,
  the active workspace read before and after, its moment his to name (founder-forks L)». A stand-in, stub or recorded
  artifact never verifies it.

## Forks of the operator's (held in `founder-forks.md`, the away form)
- **M** — the move's form: M1 (taken) · M2 · M3 · M0. Under M1, a failed restore is silent to the human
  (`inputs#I2` T3).
- **V** — the automatic verify: V1 (taken) · V2 · V3 · V0.
- **L** — R7's shape and moment. The granted move-fixture re-record stays unspent.
- **W** — one window (taken) or the folded item split out.

## The directive's item 8 block (verbatim where a tool printed it)
- The union's per-master lines and the `rows-union` line:
  ```
  architecture: rows 1045 · packs 11 · picked 260 (154 by every draw) · standing 20 · 75701 B
  security-plan: rows 665 · packs 7 · picked 148 (103 by every draw) · standing 11 · 46174 B
  design-system: rows 361 · packs 4 · picked 83 (60 by every draw) · standing 32 · 22697 B
  layout-templates: rows 237 · packs 3 · picked 71 (56 by every draw) · standing 10 · 16169 B
  test-plan: rows 1385 · packs 14 · picked 294 (172 by every draw) · standing 173 · 78670 B
  obs-plan: rows 1055 · packs 11 · picked 262 (167 by every draw) · standing 20 · 65566 B
  a11y-plan: rows 244 · packs 3 · picked 101 (62 by every draw) · standing 3 · 27980 B
  rows-union: 7 master(s) · picked 1219 of 4992 · rule any
  ```
- **Every cited row is in the union or its standing list**: yes. All 18 handles the plan cites were read present by
  `grep -F` over `{x}-rows.md` and `{x}-standing.md`. The rows files' sum is 332 957 B, past 250 000 B, so they were
  read by search, never whole.
- planlint's own lines, whole (the last firing, plan rev 3):
  ```
  planlint v1.25 · 1a2cb894
  fence: 22 entries · plan: doc:2026-11-25-placing-without-moving-the-view/plan · research: doc:2026-11-25-placing-withou…
  check 9 · CI-outcome reads: 1 · leg operator exempt: 1 · Platform issues consulted: none · `none fetched (no network r…`
  check 9 · runner-only scope bullet: not read — the letter's
  check 4 (5) · leg = 'live' entries: 0 · Harness rules consulted: filled · ``.claude/rules/verification-harness.md` — r…`
  check 4 (5) · leg entries with a content atom: 5 · note without `atom from`: 0
  check 4 (13) · operator CI-outcome reads: 1 · not ci.py conclusion: 0
  check 4 (13) · job-log reads by pattern: 0
  check 4 (14) · base-less git change reads outside the operator and round legs: 0
  check 4 (15) · grep → wc -l counts asserting exit 0 and last line 0: 0
  check 4 (16) · guards whose exclusions leave a new file in: 0 of 0 with an exclusion
  check 4 (18) · entries reading a master's or a base's path the store now holds: 0
  check 4 (19) · absence probes against the test names their own plan adds: 0 · probes read 0
  check 4 (17) · entries reading a path the wrap writes: 0 of 12 asserting — listed, P5's read
  check 4 (17) · a path the wrap writes, read from inside a program an entry runs (a suite, the ledger's tool): not read
  check 2 · ledger-note entries (matrix#): 0
  check 2 · entries naming no master and no matrix#: 0
  check 5 · placeholders outside code: 0
  check 12 (c) · step references the plan's steps lack: 0 of 6
  check 12 (d) · «unchanged» claims on a name another part of the plan names: 4 of 8 — listed, P5's read
    line 71 · «unchanged» · tests/cli_up.rs (Test Commands) · tests/cli_revive.rs (Test Commands)
    line 81 · «unchanged» · stamps.json (step 13)
    line 176 · «unchanged» · src/run/window.rs (Implementation Steps)
    line 383 · «unchanged» · window_step_moves_on_when_the_monitors_read_fails (step 6)
        window_step_keeps_placed_when_the_restore_fails (step 6)
    a claim on a name no other step, Files to modify or Test Commands names is not listed — P5's read
  check 4 (4) · criterion evidence paths the plan names nowhere else: 0 of 1
  hits: 0 — resolved by its check (validation.md 2 · 5 · 9 · 4 (4), (5), (13)–(16), (18)–(19) · 12 (c)); 0 is not a pass
  ```
  Each 12 (d) claim was read and holds:
  - line 71: those two files are not edited;
  - line 81: step 13 asserts no stamps entry;
  - line 176: without `--origin` the step does not run;
  - line 383: the eight words stay the same.
- **The claim's lines for v2-07** (`matrix.py claim` and its gated file): not yet fired. They are P5 step 4's, after
  the word; the preview above is what they will write.
- **`[RETYPED]` lines and re-spawns**, verbatim:
  - `arch-pick-p1 · [EVERY_ROW] decision/flat-cargo-workspace has no record — a row with no record was not read` →
    re-spawned as `pick-p1-r1`, `VERDICT: 1 of 1 pass`
  - `security-pick-p7.d3 · [RETYPED] line 58: decision/server-verified-before-any-channel-frame is no row of the pack —
    read as decision/server-verified-before-any-frame, the pack's row 58`
  - `tests-pick-p6.d3 · [RETYPED] line 75: decision/hook-fails-open-silently is no row of the pack — read as
    decision/hook-fails-open, the pack's row 75`
  - `tests-pick-p9 · [RETYPED] line 37: decision/synthetic-fixture-text-only is no row of the pack — read as
    decision/synthetic-fixture-text, the pack's row 37`
  - `tests-pick-p12.d2 · [RETYPED] line 57: decision/harness-builds-into-its-own-target is no row of the pack — read as
    decision/harness-builds-in-its-own-target, the pack's row 57`
- **Every non-zero exit and refusal:**
  - The Bash guard refused three calls; each was re-issued once in the form it passes:
    - a `cd` into the skill's references dir;
    - a `cd` into the run dir;
    - a tool's output piped into `grep`.
  - One hand `python3 -c` read failed (`AttributeError: 'list' object has no attribute 'get'`, my probe of the graph
    trace's shape) and was re-read correctly.
  - Baseline reds:
    - entry 5: `exit 4`, `no tests to run`;
    - entry 7: `exit 127`, `bash: scripts/r7-place.sh: No such file or directory`;
    - entry 8: `exit 127`, same.
  - Entry 13's typed control: the red and absent records read `exit 1` (as designed).
  - No `UNPARSED`, `INDETERMINATE` or `NOT DERIVED` line was printed by any tool in this run.
- **Where a letter was wrong or silent:**
  - Validation 4 (2) wants a declared boot-path list, and the test plan declares none (searched, 0 hits). I used the
    project's precedent instead (`viola open` treated the window step as boot-path) and added the
    boot → status → cleanup smoke (entries 14–16).
  - Item 5 of the directive asks for «your window» in the forks block. The window's use is not measurable as a number
    from inside, so the block says so.
  - The P4 letter would ask scope forks through a dialog. Under the directive they went to `founder-forks.md` at P3's
    end (away form), and the technical ones to the overseer (`inputs#I2`).
  - Hyprland 0.56.2's platform docs could not be fetched (no network read). The move forms were measured from the
    compositor's own local artefacts, with no dispatch.
  - The host read CONTENDED at baseline: escher's build stalled IO for 285 s, so each baseline is read against it.

## Beyond the plan
| # | Proposal the word would approve | Owner |
|---|---|---|
| 1 | The operator's answers to forks M, V, L, W (`founder-forks.md`), owed. Until then, each arm built stands PROVISIONAL, and the wrap carries them. | his word, relayed by the overseer |
| 2 | The granted ONE live re-record of the move's fixture is NOT spent by this chunk (M1 changes no form). It stays his to spend, e.g. on M2's measurement. | his grant, unspent |
| 3 | Only if he answers M3: a new route entry, proposed title «A silent move form, measured», placed after this entry and before Epoch 5's boundary. | a new entry, his word |
| 4 | The red stamp already kept for 2.1.331 on his own viola home is not touched by this chunk (this phase reads nothing of `~/.viola`). Clearing it, if he wants it cleared, is his at the desk. | his, at the desk |

Apply? (yes / review / cancel)
