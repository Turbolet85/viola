# Scope — 2026-10-19-overseer-switch-to-viola

**Working entry (verbatim title and intent):** Overseer switch to viola — the overseer drives through viola instead of
the prototype; claims v1-33, its text re-worded to the Linux dev host and a session-terminal key by the founder's ruling
2026-10-19 (inputs#I1 of the 2026-10-19T07-31-13-wrap run).

**v1-33's acceptance as it stands (re-worded 2026-10-19):** With the product binary on the Linux dev host, an overseer
session sends a pipeline skill to a builder session, waits for the turn, reads the final text, answers a review and
clears between skills, and a key typed in the session terminal takes the wheel; afterwards the overseer drives through
viola instead of the prototype.

## What this chunk builds

- The switch itself: the viola overseer stops driving its builder (`viola-builder`, this repository) through the
  prototype (`viola-lab/prototype/target/debug/viola`) and drives it through this repository's `viola` binary: a
  product-wrapped builder started by the overseer's own start line, driven by the product verbs `send` · `wait` ·
  `last` · `answer` · `list` (CLI or the `viola mcp` tools).
- The live proof of v1-33 on the Linux dev host, recorded as chunk evidence: with the product binary, the overseer
  session sends a pipeline skill to the builder, waits for the turn, reads the final text, answers a review dialog,
  clears between skills (`/clear`, confirmed by `session-start{cause:"clear"}`), and a key typed in the session's
  terminal takes the wheel (wheel line `human`), released by the human's `viola release`.
- The claim of v1-33 by this chunk (`verification-matrix.json#v1-33`, method `manual`), verified only on that live
  record.
- Whatever product defect the switch exposes in this repository is this chunk's to fix (a red met during a chunk folds
  into it); a missing capability beyond the five gap-list groups' settled answers is surfaced, not built here.

## What lies outside this repository (the overseer's own steps, named in the plan, never files of this repo)

Per the operator's take-up direction (inputs#I1): the overseer's scripts change as the overseer's own steps, named in
`plan.md` as such, never written as files of this repository. As at the take-up:

- `viola-overseer/vb.sh` (inputs#I2) and `vbr.sh` (inputs#I3): the builder's fresh and resumed start, both through
  `andromedaV3/andromeda-overseer/docs/tools/vstart.sh` (inputs#I7).
- `vstart.sh` (inputs#I7): hard-codes the prototype (`VIOLA_HOME=…/viola-lab/prototype`, its `plugin/` passed by
  `--plugin-dir`, `run <name> --dialog-timeout 3500 -- claude …`, the prototype's `list` column 3, the prototype's
  `~/.viola/sessions/<name>/` archived before a start). It is shared by every overseer's builders
  (`pulse-builder`, `conductor-builder`, `escher-builder`, `andromeda-worker`), so the switch must leave their launch
  unchanged. (Verified at P3: `pc-overseer/vb-pulse.sh`, `vb-conductor.sh`, `vbr.sh`, `escher-overseer/vb.sh` and
  `andromeda-worker/vw.sh` each call it, inputs#I10–I13.)
- `up.sh` (inputs#I8): its `viola-overseer` row raises `viola-builder` through `vb.sh --continue`.
- `viola-overseer/waitm.sh` (inputs#I4) and `waitlong.sh` (inputs#I5): call the prototype by absolute path, read the
  prototype's `wait` first-line words (`turn-end`, `question`, `plan`, `permission`, `exited`, `held`), its
  `~/.viola/sessions/<name>/dialogs/<id>.json`, its `events.ndjson` Claude event names (`hook_event_name: Stop`,
  `background_tasks`) and its `list` column 3.
- `viola-overseer/scr.sh` (inputs#I6): reads the prototype's `out.raw`; the product stores no screen.
- The overseer's procedure, `viola-overseer/driving-guide.md` (inputs#I9), written against the prototype.

## Settled by earlier rulings (folded, not reopened)

- Gap-list (`chunks/2026-10-08-first-live-test-and-self-drive/evidence/gap-list.md`): 39 items, ten stops in five
  groups. The founder's own answer to Q3 of chunk 2026-10-17-mcp-server-for-drivers («Гр. 2: удержание диалога»,
  that chunk's inputs#I2): only group 2 (the dialog hold) must close before the switch; it closed at
  2026-10-19-dialog-hold-for-the-driver (`dialog_hold_s` 60–3600). Groups 1 (`list`) and the terminal-replies stop
  closed earlier. Groups 3 (the screen read: modals, the screen before a dialog, `ctx NN%`) and 4 (`allow <n>`) are
  not built: the overseer adapts its procedure to the product as it stands. Group 5 (the start line and three scripts)
  is the overseer's, outside this repository.
- The takeover key: the founder's ruling of 2026-10-08T05:02Z (chunk 2026-10-08-first-live-test-and-self-drive,
  inputs#I3): he is not at the keyboard for it; the takeover is a real key an agent types into the session's terminal
  window through the compositor. Standing rule: read `hyprctl locked` before any step that focuses a window or types
  a key with `wtype`, and stop on `true`.

## Folded freight (the entry's three blocks)

- CARRY (minted at the 2026-10-17-mcp-server-for-drivers wrap, its card row 2, the founder's Q4 answer): this entry is
  placed after the gap entries and claims v1-33; gap-list group 5 lies outside this repository.
- CARRY (chunk 2026-10-19-dialog-hold-for-the-driver, card row 2): with the overseer's home at `dialog_hold_s` 3600,
  this switch is where a dialog hold near the 3600 s ceiling is first read live; the ceiling stays PROVISIONAL until
  then (671 s measured on claude 2.1.287, that chunk's `evidence/live-round.md`). Folded as: a hold the switched
  builder raises and the overseer answers late is recorded with its measured duration; the ceiling's PROVISIONAL mark
  is lifted only on a reading near 3600 s. (Verified at P3: `dialog_hold_s` is read once at the wrapper's start, so the
  builder's home `config.json` holds 3600 before its start; history 2026-10-19-dialog-hold-for-the-driver.)
- watch: `run::send::budget::tests::send_budget_paused_is_refused_with_the_window_word_and_no_cursor` case_4 red once
  under a plain `cargo test` (0/3; since 2026-10-18-compact-post-condition-ledger-row).

## Facts measured at the take-up that the plan must meet (closed at P3)

- `viola` first on the host's PATH is the prototype (`which -a viola`: `…/viola-lab/prototype/target/debug/viola`).
  The other overseers keep using it, so the switched overseer names the product by absolute path. (Verified.)
- The installed `claude` is 2.1.296 (`claude --version`; 2.1.287, 2.1.288, 2.1.289 also installed under mise), and
  every product ledger stamp so far is on 2.1.287. On a CLI version with no `viola verify` stamp in the builder's home,
  `cli_verified` is false. Then a non-`null` dialog decision is refused `unverified-cli`, and a send is confirmed
  after the fact only. `[premise-corrected: it is worse than "answers refused" — on an unverified CLI every dialog is
  answered null AT ONCE, never held, so the builder's own dialogs (its founder forks, its review card) fall to the CLI's
  menu in its terminal; src/run/dialog.rs:436, src/cmd/run.rs:869-886]`. `run` takes an absolute program path as
  given (`crates/viola-agent-claude/src/lib.rs:510`), so a builder can run a pinned CLI install.
- `~/.viola` (the product's default home) is the prototype's state root, mode 0755, holding `sessions/` and `archive/`
  for five builders. The product's strict-modes check refuses a home that is not 0700, and a product start narrows an
  existing home to 0700 (`src/obs.rs:43-46`, verified). The prototype's processes run as the same user, so the
  narrowing takes nothing from them. Only the default home reads the user's status line (`~/.claude/settings.json`
  holds a `command` status line); a separate `--home` reads none until `statusline-source.json` is planted. (Verified.)
- The builder that implements this chunk is the session being switched. The switch restarts it under the product
  wrapper, so the moment of the switch decides which of this chunk's own pipeline steps (implement, wrap) run under
  the product. Its first product start is `viola run viola-builder -- claude --resume <id>`, the id from the
  prototype's `~/.viola/sessions/viola-builder/session_id`. `viola revive` only resumes an id a product instance log
  holds (history 2026-10-10-viola-revive), so it serves the restarts after the first. (Verified.)
- The product binary that drives the builder must not be the `target/debug/viola` the builder rebuilds while it works.
  `run` pins a hashed copy under `bin/<version>-<hash>/` for the hooks, but the wrapper process and the overseer's CLI
  calls run the path they were started from (history 2026-09-27-instance-state-and-start-order). (Verified.)

## Gaps found at P3 beyond the 2026-10-08 gap list

The founder's Q3 answer named groups from the 2026-10-08 list. These two were not on it; both stop a part of the
overseer's procedure as `inputs#I4`, `inputs#I5` and `inputs#I9` hold it.

- **Option detail.** A `question` event carries each option's `label` only. `question_data` drops `description`
  and `preview` (`crates/viola-agent-claude/src/dialog.rs:138-155`), although the CLI sends both (recorded in
  `fixtures/claude/2.1.287/PreToolUse.ask-user-question.json`). `waitm.sh` prints each option's `preview` from the
  prototype's `dialogs/<id>.json`, and the overseer relays a builder's founder fork with its options' arguments. On
  the product it would relay bare labels.
- **Background tasks at a turn end.** `turn-ended` carries `last_assistant_message` only (`hook.rs:162`;
  `EventKind::TurnEnded` data keys, `crates/viola-core/src/lib.rs:119`). The Stop payload's `background_tasks` is
  dropped (recorded in `fixtures/claude/2.1.287/Stop.default.json`). `waitlong.sh` returns on a turn end whose Stop
  `background_tasks` is empty: the builder then waits for its driver (V103, V105 in `inputs#I9`). On the product it
  cannot tell a progress turn end from one that waits for the driver.

Whether these are built in this chunk or the overseer adapts is a P4 fork.

## The founder's answers at P4 (inputs#I14; added at P5 validation-1, intent-incomplete)

- Start cap: the builder keeps claude 2.1.296, with a cap of 8 live starts. A failed relied row halts for his word;
  there is no fallback start.
- Switch: mid-implement.
- New gaps: both are built. The option `description` and `preview` go into the question event, and Stop's
  `background_tasks` goes onto `turn-ended` as a count only. The option-text widening is ratified by him.
- Takeover: he types the key himself and runs his own `viola release`, later; no agent key.
- gate: the founder at his keyboard in the viola-builder terminal for his own key and his own viola release — he is
  away from the builders' terminals at the take-up, so the takeover reading (v1-33's "a key typed in the session
  terminal takes the wheel") cannot be taken now. Every other act proceeds; the takeover is the chunk's last attended
  step and its gated criterion.

## Forks that are the founder's (inputs#I1: raised as one dialog at P4)

- A live `claude` start cap: the switched builder start(s) and any `viola verify` run against the builder's CLI
  version (each verify starts five interactive PTY children).
- Any boundary widening the switch needs.
- The moment of the switch.

## CI read at the take-up

- f23586fb (the last wrap's flip, HEAD): green, 15/15 checks, wall 635 s, ci#39234318604.
