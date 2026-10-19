# Codebase Research — 2026-10-19-overseer-switch-to-viola

## Scope
- **Depth:** moderate · **Reads:** 14 · **Globs/Greps:** 16
- **Harness rules consulted:** `.claude/rules/verification-harness.md` (read in full, 18 Session Additions) — 3 applied: 2026-10-10 (the bare `claude` resolves to a newer CLI than a home is stamped for; a live rig puts the stamped version's install dir first on `PATH` and reads `claude --version` by path), 2026-10-09 (a build's file hash is no proof two builds load the same code), 2026-10-07 (the tmpfs `target/e2e-home`; a stalled-start red is a finding about the backing)
- **Platform issues consulted:** none — no runner-only bullet and no CI-reading entry outside the operator leg
- **External inputs:**
  - `inputs#I1` — the operator's take-up direction: the overseer's scripts planned as its own steps, never files of this repository; three founder forks raised as one dialog.
  - `inputs#I2` / `inputs#I3` — `vb.sh` / `vbr.sh`: the builder's fresh and resumed start, both through `vstart.sh`.
  - `inputs#I4` / `inputs#I5` — `waitm.sh` / `waitlong.sh`: they call the prototype by absolute path. They read its first-line words, its `dialogs/<id>.json` option previews, its log's `hook_event_name: Stop` / `background_tasks`, and its `list` column 3.
  - `inputs#I6` — `scr.sh`: reads the prototype's `out.raw`.
  - `inputs#I7` / `inputs#I8` — `vstart.sh` / `up.sh` (andromedaV3 at `8d053dcd`, pointers): the prototype hard-coded in `VIOLA_HOME`, `--plugin-dir`, `--dialog-timeout 3500`, `list` column 3 and the session-folder archive.
  - `inputs#I9` — `driving-guide.md`. Since the 2026-10-08 gap list it adds the waitlong rule: return on a turn end whose Stop `background_tasks` is empty (V103, V105). It also adds: live starts inside the founder's per-chunk cap; "a reading that needs a typed key or a window is the founder's to grant"; the nested compositor of 2026-10-08 is not repeated.
  - `inputs#I10`, `inputs#I11`, `inputs#I12`, `inputs#I13` — `pc-overseer/vb-pulse.sh`, `vb-conductor.sh`, `escher-overseer/vb.sh`, `andromeda-worker/vw.sh`: each calls the shared `vstart.sh`.
  - `inputs#I14` (P4) — the founder's four answers: 2.1.296 with a cap of 8, the switch mid-implement, his own key and release later (no agent key), both gaps built with a task count only. They settle the three open questions below.

## Files inspected
- `crates/viola-agent-claude/src/dialog.rs` (100–175, 420–445 via `src/run/dialog.rs`) — `question_data` keeps each option's `label` only. An AskUserQuestion option's `description` and `preview` are dropped before the `question` event (`dialog.rs:138-155`).
- `crates/viola-agent-claude/src/hook.rs` (`normalise`, :116-162) — a Stop maps to `{"last_assistant_message"}` alone (`hook.rs:162`). The payload's `background_tasks` is dropped.
- `crates/viola-core/src/lib.rs` (105–135) — `EventKind::data_keys`: `TurnEnded => ["last_assistant_message"]`, `Question => ["dialog_id","questions"]`. A reader counts any other key as unknown (`crates/viola-state/src/events.rs:214`).
- `src/run/dialog.rs` (420–445) — `answer` on `!self.cli_verified` → `refusal(UnverifiedCli)`. `src/cmd/run.rs:869-886` (the unit `methods_route_hook_dialog_and_answer_to_the_dialog_slot`): on an unverified CLI a dialog is answered `null` AT ONCE, not held.
- `src/cmd/run.rs` (36–44) — `RunArgs { name, program: Vec<OsString> }` after `--`. `crates/viola-agent-claude/src/lib.rs:510` (`resolve_program_uses_a_given_path_as_given`): an absolute program path is taken as given, so a builder can run a pinned CLI install.
- `src/obs.rs:43-46` — `ensure_private_dir`: "an existing one is narrowed to 0700 on Unix" (through `viola_state::fs::create_private_dir`).
- `fixtures/claude/2.1.287/Stop.default.json` and `PreToolUse.ask-user-question.json` (grep) — both shapes are recorded today: `background_tasks` on Stop; `description` / `preview` on question options (recorded in the 2.1.283, 2.1.287 and 2.1.288 Stop sets).
- `~/.viola/sessions/viola-builder/events.ndjson` (grep) — the prototype logs `background_tasks` arrays like `[{"agent_type","description","id","status","type"}]` at Stop.
- `viola-0.1.0/chunks/2026-10-08-first-live-test-and-self-drive/evidence/gap-list.md` and `evidence/live-run.md` — the 39-item gap list. In the takeover there, "No `release` and no `pause` was run by anyone"; the wheel was left with the human, on a separate live instance in a nested compositor.
- `scripts/code-graph-cookbook.md` (full) — query shapes 1 and the refs form.

## Graph impact
- **question_data / tool_data / normalise** (`viola-agent-claude`) — 12 caller rows (`tree-query-2026-10-19-overseer-switch-to-viola.json`, query 1). The production callers are `dialog/classify` (`crates/viola-agent-claude/src/dialog.rs:102`), `dialog/tool_data` (`:129`) and `cmd/hook/handle` (`src/cmd/hook.rs:164`). The rest are `hook.rs` unit tests. An additive key inside each question object, or in a Stop's `data`, changes no signature.
- **data_keys** (`viola-core`) — 2 refs (`crates/viola-core/src/lib.rs:638`, `crates/viola-state/src/events.rs:214`). A new `turn-ended` key must be listed there, or every reader counts it unknown.

## Patterns detected
- **Closed per-kind data keys, tolerant readers** (`crates/viola-core/src/lib.rs:113-130`; `crates/viola-state/src/events.rs:214`): an event's `data` keys are a closed list per kind. A key outside it is skipped and counted, never refused.
- **Dialog data is a normalised projection of the tool input** (`crates/viola-agent-claude/src/dialog.rs:119-155`): the event carries a chosen subset (text, header, labels, multi-select), and the decision body is built from the stored raw `input`.
- **Unverified CLI degrades to transport-only** (`src/run/dialog.rs:436`; `src/cmd/run.rs:869-886`): dialogs are answered `null` at once, and `answer` is refused `unverified-cli`.

## Conventions to follow
- **Claude shapes only in `viola-agent-claude`** (CLAUDE.md, Critical Warnings): any reading of `background_tasks` or of option `description` / `preview` lives in that crate's `hook.rs` / `dialog.rs`, never in `src/run/`.
- **Upstream text is content**: an option `description` / `preview` is content, like the question text already carried. It reaches `events.ndjson` only as event data, never a home-level diagnostics line (obs-plan §8 PII Scrubbing).
- **The overseer's scripts are not files of this repository** (`inputs#I1`): every change to `vb.sh` · `vbr.sh` · `vstart.sh` · `up.sh` · `waitm.sh` · `waitlong.sh` · `scr.sh` · `driving-guide.md` is a named overseer step in `plan.md`.

## New files to create
- `viola-0.1.0/chunks/2026-10-19-overseer-switch-to-viola/evidence/switch-round.md` — the live switch record: the verify run, the switched start, the six v1-33 acts and the long-hold reading, as kinds, cursors, exit codes and durations only.
- `viola-0.1.0/chunks/2026-10-19-overseer-switch-to-viola/evidence/switch-readings.ndjson` — one line per live act, with the clock time.

## Files to modify
- `crates/viola-agent-claude/src/dialog.rs` — built (inputs#I14 Q4): each question option's `description` and `preview` carried in the `question` event data, with unit cases.
- `crates/viola-agent-claude/src/hook.rs` — built (inputs#I14 Q4): a Stop's `background_tasks` carried on `turn-ended`, with unit cases.
- `crates/viola-core/src/lib.rs` — `EventKind::TurnEnded`'s data keys gain the new key, with its unit.
- `crates/viola-state/tests/state_events.rs` — the reader's known-key cases.
- `tests/hook_events.rs` — the end-to-end cases that the new keys reach `events.ndjson` under the fake agent.
- `src/run/dialog.rs` — its question-data unit pins (`multi_select` sites).

## Sweeps
- `multi_select` over `src/`, `crates/`, `tests/` (`grep -rln 'multi_select' --include=*.rs src crates tests`): 3 files · up to 3 changed with the dialog half (`src/run/dialog.rs`, `crates/viola-agent-claude/src/dialog.rs`, `crates/viola-state/tests/state_events.rs`) · 0 otherwise.
- `last_assistant_message` over `src/`, `crates/`, `tests/` (`grep -rln 'last_assistant_message' --include=*.rs src crates tests`): 20 files · the turn half changes `hook.rs`, `lib.rs` and `state_events.rs`, plus a case in `tests/hook_events.rs`. The other 16 read the key and stay unchanged (additive key; `last` and `wait` read `last_assistant_message` by name).
- `vstart.sh` over the overseer launchers (`grep -l vstart.sh` over `pc-overseer/*.sh`, `escher-overseer/*.sh`, `andromeda-worker/*.sh`): 5 launchers call it (`inputs#I10`–`I13`, plus `pc-overseer/vbr.sh`). A change to `vstart.sh` reaches every one, so the switch's start line is viola-builder's own.

## Open questions
- Which CLI version the switched builder runs (2.1.296, needing a fresh 24-row verify in its home, or a pinned 2.1.287), and the live start cap → blocks: plan-decision (founder fork).
- The moment of the switch inside this chunk's lifecycle → blocks: plan-decision (founder fork).
- The two gaps found since the 2026-10-08 list (option `description` / `preview`; Stop `background_tasks`): built here, or the overseer adapts → blocks: implementation-scope (the file list above is provisional on it).
