# Report — 2026-11-28-console-actions

**Chunk:** the fleet console's actions — every verb by keyboard from a selected row, through the CLI verbs, a send's readback, refusals as `unable` lines (both parts: Part 1 built by run `2026-11-28T14-52-37Z-implement`, Part 2 by this run)
**Date:** 2026-11-28
**Run:** 2026-11-28T15-45-49Z-implement

## Changes (structured — detectors read this)
- **Files:** new `src/console/action.rs`; modified `src/console/keys.rs`, `src/console/mod.rs`, `src/console/view.rs`, `src/human/console.rs`, `src/human/fleet.rs`, `src/human.rs` (re-exports, companion), `src/cmd/console.rs`, `src/cmd/list.rs` (companion), `src/human/board/test_support.rs` (companion), `crates/viola-core/src/obs.rs`, `crates/viola-state/src/board.rs`, `schemas/diag-line.v1.json`, `schemas/diag-detail.v1.json`, `tests/contract_diag_schema.rs`, `tests/tui_console.rs`, `Cargo.toml`, `Cargo.lock` (basis: `gate.py scope` — changed 18 · listed 16 · recorded 2).
- **Symbols / APIs:** (all `pub(crate)` in the root bin save the board field; no IPC method, endpoint, socket, port or env var added)
  - Part 1 (carried from run `2026-11-28T14-52-37Z-implement`'s outcome, re-read at the tree): `console::action` `Verb`, `Action { verb, name, args, stdin }` + `Action::on`, `Outcome`, `Runner::new` / `command` / `spawn` (`<exe> --home <home> <verb> <name> <args…>`, `VIOLA_NAME` / `VIOLA_DIR` removed through `STRIPPED`, no shell, each stream through `take(MAX_FRAME)`), `lines_of` (each line through `escape_row`); `console::keys` `Key` {`Interrupt`, `Up`, `Down`, `Enter`, `Escape`, `Backspace`, `Char(char)`, `Unbound`}, `Decoder` (`feed`, `deadline`, `flush`), `ESC_WAIT` 50 ms, `READ_CAP` 64 kept; `console::view` `View::of(lines, size, selected, result, keys)`, `Selection`, `wrapped_names`, the `... <n> above` line; `console` `Sources.act`, `Mode`, `Shown`, `State`; `human` `console_confirm_prompt`, `console_running_line`, `write_console_frame(out, rows, body, keys)`, `FleetLine.name`, `gutter`, `SELECTED_GUTTER`, `GUTTER`, `escape_row` re-exported; `viola_core::obs::ObsEvent::ConsoleAction` (`console-action`).
  - Part 2 (this run): `Verb` gains `Send` · `Switch` · `Open` · `Answer` (words `send` · `switch` · `open` · `answer`); new `Action::send(name, text)` (text on stdin unchanged), `Action::switch(name, text)` (space-split words, an `is_effort_level` word `--effort=<word>`, any other `--model=<word>`, one element each), `Action::open(name, typed)` (`open <typed> --parent <name> -- claude`, the typed text one element; `Runner::command` puts no name positional for `Open`), `Action::answer(name, dialog_id, behavior)` (`answer <name> <id>`, `{"behavior":"<word>"}` on stdin); new `Behavior` {`Allow`, `Deny`, `Approve`, `Revise`} with `word()`.
  - `console` (mod.rs): `Mode` gains `Editing { verb, name, text }` and `Answer { name, dialog }`; new private `Fold` (an editing read's `text` / `backspace` / `unbound` counts, one line a class of count ≥ 1); `State` gains `dialogs` (the wrapped rows' pending dialogs by name), `press_read` (one read's keys; `run`'s only key path now), `editing_end`, `answer_key`, `answer_opened`. Keys `s` `m` `o` open the editing line, `a` the answer prompt (permission or plan), or one result line on a question.
  - `console::view`: new `View::with_fitted_keys(keys)` (the editing line takes no second cut). Sole caller: `State::view`.
  - `human`: `CONSOLE_KEYS` now `q quit  jk move  l last  p pause  r release  f focus  v revive  s send  m switch  a answer  o open`; new `console_editing_prompt(verb, name)` (`send <name>: ` · `switch <name>: ` · `open child of <name>: `), `console_answer_prompt(name, kind)` (`answer <name> permission? y allow  n deny` · `answer <name> plan? y approve  n revise` · none), `console_question_line(name)` (`<name>  question - answer it in the session (f)`), `console_editing_line(prompt, text, columns)` (the width rule); `FleetLine` gains `dialog: Option<Pending>`; new `Pending { kind, id }`, re-exported from `human`.
  - `viola_state::board::Item` gains `pub dialog_id: Option<u64>`, `#[serde(skip)]` beside `dialog_kind` (read from the snapshot's `pending_dialog.dialog_id` in `snapshot_item`, `None` in `replayed_item` and `Item::unwrapped`). Other constructors (companions): `src/cmd/list.rs`, `src/human/board/test_support.rs`, `src/human/fleet.rs` and `src/console/mod.rs` test literals. `list`, `list --json` and the MCP list item unchanged (`binary(cli_list) | binary(cross_list_rows)` green; `board_read_item_serialises_to_the_contract_literal` unchanged and green).
- **Crates / modules:** added module `src/console/action.rs` (Part 1); changed `viola` (root bin), `viola-core`, `viola-state`.
- **Dependencies:** added `unicode-width = "=0.2.2"` to `[workspace.dependencies]` and the root bin's `[dependencies]` — the crate and version `vt100 v0.16.2` already brought into the lock; `Cargo.lock` gains only the root package's dependency line, no crate added; `cargo deny check` green.
- **Schema / config:** `schemas/diag-line.v1.json`: Part 1 — event enum + `console-action` message arm + field arm (`verb` enum, `exit_code` integer, both required), the `console-key` `key` enum grown to 13 words; Part 2 — the `console-key` arm gains `count` (`#/$defs/count`), `key` gains `question`, and a nested `if key == "text" then required ["count"]`. `schemas/diag-detail.v1.json`: event enum + `console-action` (Part 1). `crates/viola-core/src/obs.rs`'s event doc names `count`. No config key.
- **Spec-master edits:** none.
- **Retired:** (Part 1) the scaffold's `Key::of` and its `key_of_reads_one_read_as_one_class` / `key_words_are_the_closed_three` cases; the key line `q quit` alone. (Part 2) the per-key `console-key` line in the editing mode — never shipped, replaced before landing by the per-read fold. No census row stands for them.
- **Counts / qualifiers moved:** the closed event list 18 → 19 (`ObsEvent::ALL`, `LOG_FORMAT_EVENTS` in `crates/viola-core/src/obs.rs` and `tests/contract_diag_schema.rs`, both schemas' event enums) (Part 1); the `console-key` word set 3 → 14 (`schemas/diag-line.v1.json`: 13 at Part 1, `question` added at Part 2); the console-key field arm's keys `key` → `count,key` (the plan's probe); `console-action`'s `verb` enum 9 words (unchanged by Part 2; Part 2's four verbs now write it); the console key line's text (above).
- **Dev-tool versions:** none.
- **Harness / gate surface:** none.
- **Cross-project / external claims:** inputs#I1 (the overseer's take-up directive), inputs#I2 (the operator's answers F1–F7), inputs#I3 (the P5 word on `.andromeda/runs/2026-11-28T13-46-51Z-phase/p5-card-1.md`, delegate overseer, PROVISIONAL), inputs#I4 (the stop at the Part 1 seam), inputs#I5 (the operator's own answers to Q8 and Q9 and the revision's six orders), inputs#I6 (the P5 word on `.andromeda/runs/2026-11-28T15-25-27Z-phase/p5-card-1.md`: P1–P5 technical, PROVISIONAL), inputs#I7 (the P5 word on `.andromeda/runs/2026-11-28T15-25-27Z-phase/p5-card-2.md`: yes to rev 6 — the governing card), inputs#I8 (this run's invocation: Part 2, the whole block, operator away, v2-15 GATED, fakes only, then the operator pass). Card 2's `## Beyond the plan` rows, verbatim:
  - (standing — `.andromeda/runs/2026-11-28T13-46-51Z-phase/p5-card-1.md`) No route entry is inserted (his F2/F7: one entry).
  - (standing — `.andromeda/runs/2026-11-28T13-46-51Z-phase/p5-card-1.md`) adopt from the console is not owned by any entry. If he wants it, it is a new route entry on his word: its proposed
    title would be «Console adopt», placed after «Console actions».
  - ENDED — card 1's row «The five product choices P1–P5 of this run's `founder-forks.md` stay his, owed»: answered by
    the delegate overseer as technical on their recommended arms (inputs#I6); their rows land PROVISIONAL for the epoch
    batch, where he sees them at his live reading.
  - After the word: P5 step 4 claims matrix#v2-15 GATED with card 1's note and block; the phase ends; no implement starts
    (inputs#I6).
- **Reverted / negative API facts:** none.
- **Insufficient fixes (written, kept, not the remedy):** none.
- **Spec claims disproved by measurement:** (Part 1, carried) plan step 9 named `console_refused_action_logs_the_verbs_exit_code` as «pause on a dead instance gives exit 21»; a gone instance draws no row (`crates/viola-state/src/board.rs` «a gone instance has no row»), so the case runs over a frozen, stale wrapper — read into plan rev 6's step 9. Part 2: none.
- **Expected amendments (from plan):**
  - design-system §Surface: cli (decision/list-is-the-board): carried (**Symbols / APIs**, `console::action`, all nine verbs).
  - layout-templates §Surface: cli (decision/board-vs-strip): carried.
  - security-plan §Input Validation + decision/console-runs-verbs-as-children: carried (`Runner::command`, `drain`, one child at a time; Part 2's stdin texts through the same runner).
  - security-plan §Data Protection + decision/console-action-child-outlives-quit: carried (no parent-death tie).
  - design-system §Surface: console (contract/fleet-console-keys/unbound) + bound keys and confirm prompts: carried (`jk`, arrows, `l` `p` `f` `r` `v` `s` `m` `a` `o`, `y`/`n`/escape).
  - design-system §Surface: console + contract/fleet-console-keys/answer: carried (`console_answer_prompt`, `console_question_line`, `answer_opened`; `a` with nothing pending reads `unbound`).
  - design-system §Surface: console + /open · /send · /switch: carried (`Action::open`, `Action::send`, `Action::switch`, `console_editing_prompt`).
  - design-system §Surface: console (contract/fleet-console-view/more-line) + /above-line: carried (Part 1).
  - a11y-plan §5 (decision/cli-no-interactive-input, decision/cli-not-keyboard-interactive, decision/static-linear-output): carried — the selection redraw, the confirm, the answer prompt and one editing line.
  - obs-plan §1 (decision/no-interactive-terminal-interface): carried, the same.
  - layout-templates §Surface: console (decision/console-frame-floor): carried (`CONSOLE_KEYS`, `q quit` first; at 80 columns the line is cut after `m swi`, **Decisions & corrections**).
  - layout-templates §Surface: console (decision/console-own-surface): carried (`console_editing_line`: the echo as typed through `escape_row`, `unicode-width` 1/2/0 columns, past the edge prompt + `...` + longest fitting tail, a straddling wide scalar and a width-0 head left out).
  - security-plan §Input Validation (decision/console-key-reader-bounded): carried (`Decoder`, `ESC_WAIT`, `READ_CAP`; the editing text bounded at `MAX_FRAME` bytes, a char past it counted `unbound`; Enter, Escape and Interrupt end the read, the rest dropped).
  - obs-plan §6 (contract/diag-line-fields/console-key) + contract/diag-line-fields/console-action, an obs Decisions Log entry: carried (**Schema / config**; `Fold`).
  - test-plan §3 (decision/harness-log-fields-owned-by-test-plan): carried (the console-action line and the console-key count).
  - matrix#v2-15 is claimed at P5 step 4 (GATED); matrix#v2-31 not claimed by this entry: not this run's to write.
- **Coverage of new surfaces:**
  - `the console's key reader (stdin, raw)` → validation READ_CAP 64 a read + closed `Key` classes + CSI cap 32 + editing text bound `MAX_FRAME`✓ · instrumentation `console-key` per key, at most four a read on the editing line✓ · PII no key byte, typed text or count of anything but keys in any line✓ · tests unit (`decoder_…`, `editing_…`) + e2e (`tui_console`) · a11y kbd: every mode left by one key (`n`/escape from a confirm, escape from the editing line and the answer prompt), Ctrl-C ends every mode✓ · tokens n/a
  - `an action child's stdout/stderr` → validation `take(MAX_FRAME)` per stream + `escape_row` per line✓ · instrumentation one `console-action{verb, exit_code}` per finished child✓ · PII no name, argument or output in any line✓ · tests unit (`action_…`) + e2e · a11y n/a · tokens n/a
  - `the action child's spawn and stdin (own exe, --home, VIOLA_NAME/VIOLA_DIR removed, no shell; send's text, answer's body)` → validation `Runner::command`; switch words as `=` values, open's text one element✓ · instrumentation the child's own `cli-<name>` log✓ · PII n/a · tests unit (`action_child_env_drops_the_instance_names`, `action_switch_words_are_values_never_flags`, `action_open_takes_the_typed_name_as_one_element`, `action_answer_bodies_are_the_four_response_shapes`) + e2e · a11y n/a · tokens n/a
  - `the editing line's echo (UTF-8, the width rule)` → validation decoder admits no control + `escape_row`✓ · instrumentation n/a (no line per char) · PII the text reaches no line✓ (`editing_letters_are_text_and_backspace_removes_one_scalar`, `console_send_hands_the_line_to_the_verb_and_logs_its_exit`) · tests unit (`editing_width_…`, `view_fitted_keys_take_no_second_cut`) + e2e (`console_send_a_cyrillic_line_as_typed`) · a11y kbd✓ · tokens n/a (plain text, no colour)
  - `the answer prompt and the question line` → validation the board's kind word and id alone, no dialog text read✓ · instrumentation `console-key` `question`, `console-action{answer}`✓ · PII no question or option text in any line✓ (`console_answer_question_draws_one_line_and_runs_nothing`) · tests unit + e2e · a11y kbd✓ · tokens n/a
  - `Item.dialog_id (board)` → validation from the strict-checked snapshot's `u64`✓ · instrumentation n/a · PII `#[serde(skip)]`, not in `list --json` or the MCP item✓ · tests unit (`board_read_takes_a_pending_dialog_and_its_kind`) + integration (`cli_list`, `cross_list_rows`) · a11y n/a · tokens n/a

## Deviations from intent
- **Part 1's deviations stand as that run's outcome lists them** (`.andromeda/runs/2026-11-28T14-52-37Z-implement/outcome-2026-11-28-console-actions.md`): `q` `j` `k` decoded as `Key::Char`, the action starter `Sources.act`, revive's word `up`/`revive`, the two-column gutter, busy and no selection, the frozen-wrapper refused case, the 80x5 scroll case — each read into plan rev 6's Part 1 steps (absorbed by `.andromeda/runs/2026-11-28T15-25-27Z-phase`).
- **`FleetLine.dialog` as `Option<Pending>`**: the plan says the line «carries the selected row's kind and id»; every wrapped line carries its own pending dialog (kind and id only when both stand), and `State.dialogs` keeps them by name from the last view, so `a` reads the selected row's.
- **The editing mode's `Up`/`Down` and a backspace on an empty line** count in the read's `unbound` line (a key dropped), so every key of a read lands in one of the fold's three counts or the ending line.
- **`a` on a question while a child runs** writes `question` and leaves the running line standing (no child starts either way); `a` on a permission or plan while a child runs reads `busy`, as the other action letters do.
- **Answer's busy key**: `y`/`n` in the answer prompt while a child runs reads `busy` and leaves the prompt, as the confirm does.
- **`console_answer_permission_by_y`** raises its permission by a test-run `viola hook permission-request` over the recorded `PermissionRequest.permission-1.json`, as `tests/cli_answer.rs`'s plan case raises its plan, rather than through the gated `path4-permission.json` script the plan names; the verdict reads the hook's own decision body (`allow`) and one `dialog-answered` line. The plan case reads `deny` from the body (a revise of a PermissionRequest-raised plan denies it).
- **`console_switch_hands_its_words_to_the_verb`** reads the pair from the wrapper's `launch` line after the switch (`sonnet`, `high`) on a session adopted as `tests/cli_switch.rs` adopts it, not from the fake agent's start receipt.
- **The Ctrl-C case** raises one permission before its four console starts (list, confirm, editing, answer), so the answer prompt has a dialog to open.
- Scope record (`gate.py scope`: clean — changed 18 · listed 16 · recorded 2 (companion 2 · mechanical 0 · in-intent 0 · widening 0)):
  - companion: `src/human.rs` · serves src/human/fleet.rs · self (Part 1; now listed)
  - companion: `src/cmd/list.rs` · serves crates/viola-state/src/board.rs · self (an `Item` literal gains `dialog_id`)
  - companion: `src/human/board/test_support.rs` · serves crates/viola-state/src/board.rs · self (the same)

## Decisions & corrections
- The invocation's directive (inputs#I8): Part 2 from step 10, the whole block, operator away and no dialog, v2-15 stays GATED, fakes only under their own PTY, test home and test-scoped PATH, no window, no real claude, a red with no failed assertion read against hostwatch first, times from `date -u`, no scheduled wake, then the operator pass through the push and its CI read. Followed: no live start; v2-15 given no ref.
- Guard refusal, re-issued once: the Bash guard refused a `cat` heredoc to a scratch script; the edit was re-issued as Edit-tool edits.
- Own slip, measured: a shell call that began `cat > /dev/null;` blocked on stdin and ran to the Bash tool's limit before the script after it ran; stopped by its task id, nothing written (the files read unchanged by `git diff --stat`), re-done with the Edit tool.
- Measured: at the 80-column floor the new key line is cut to `q quit  jk move  l last  p pause  r release  f focus  v revive  s send  m swi...` (98 columns in full), so `a answer` and `o open` are not shown at 80 columns; `q quit` stays first as the plan rules (`console_run_on_q_draws_the_frame_then_quits_restored` pins the cut). Whether the floor's line should show every letter is the layout row's to say (decision/console-frame-floor), for the wrap.
- Measured: a 200-byte paste in one PTY write reached the console in at most four reads (`console_editing_line_logs_one_text_line_per_read`: the `text` counts sum to 200 in 1–4 lines).
- Measured: a CR inside a read ends the editing line and drops the rest of that read, so `ab\rcdq` sends `ab` and the `q` after it is never pressed (`editing_a_cr_ends_the_read_and_drops_the_rest`).

## Outcome
As of this run's end: **green** (both parts; the operator pass follows in this window).
- Acceptance (security, F1): `action_child_env_drops_the_instance_names` and `contract_bounded_readers` green; own exe, `--home`, no shell, `MAX_FRAME` per stream — met, held by Part 2.
- Acceptance (product, Part 1 verbs): the part-1 e2e filter green (11 cases) — met.
- Acceptance (product, `s` · `m` · `o`): the part-2 e2e filter green (12 cases): send's records and exit 0, the empty line exit 13, switch's launch pair, a bad word exit 2, open's spawn naming `run child` and `--parent builder` — met.
- Acceptance (product, his Q8): `console_answer_permission_by_y` (`allow`), `console_answer_plan_by_n` (`deny`), `console_answer_question_draws_one_line_and_runs_nothing` (no child, no `dialog-answered`, no question or option text in `console.ndjson`) green — met.
- Acceptance (layout, his Q9): the seven `editing_width_follows_the_one_rule` cases and `console_send_a_cyrillic_line_as_typed` (the prompt-submitted text `привет мир`, origin `driver`, `send-confirmed`) green — met.
- Acceptance (obs): `editing_a_read_of_text_writes_one_text_line` and `console_editing_line_logs_one_text_line_per_read` green; the console-key probe reads `count,key`; the console-action probe reads `verb,exit_code` — met.
- Acceptance (a11y, no key trap): `console_ctrl_c_ends_from_every_mode` (list, confirm, editing, answer; exit 0, cooked after), `console_release_confirm_left_by_escape_runs_nothing`, `console_editing_line_left_by_escape_runs_nothing`, `console_answer_left_by_escape_answers_nothing` green — met.
- Acceptance (security, the decoder): the part-1 unit filter green — met.
- Acceptance (layout, F3/F6): Part 1's selection and scroll cases green — met.
- Acceptance (tests): `run --unit`, `--integration`, `--e2e` each `"ok":true` — met.
- Acceptance (matrix#v2-15): GATED — proven over the fakes by the part-1 and part-2 e2e filters; its live half waits on his live reading; no ref written (the directive, and a gated cap takes none without clearance).
- Gates (one block firing, `gate.py run`, `.andromeda/runs/2026-11-28T15-45-49Z-implement/gate-2026-11-28-console-actions.json`):
  - `cargo fmt --all --check` — green · exit 0
  - `cargo clippy --workspace --all-targets --features fake-agent -- -D warnings` — green · exit 0
  - `cargo nextest run -p viola-core --no-fail-fast` — green · 41 run, 41 passed
  - `cargo nextest run -p viola-state --no-fail-fast` — green · 334 run, 334 passed
  - `cargo nextest run -p viola --features fake-agent --bins --no-fail-fast` — green · 1561 run, 1561 passed
  - the part-1 unit filter — green · 47 run, 47 passed
  - the part-2 unit filter (`console::(tests::editing_|action::tests::action_(send|switch|open|answer)_)|human::console::tests::editing_width_`) — green · 20 run, 20 passed
  - `binary(tui_console)` — green · 34 run, 34 passed
  - the part-1 e2e filter — green · 11 run, 11 passed
  - the part-2 e2e filter (`^console_(send_|editing_line_|switch_|open_|answer_)`) — green · 12 run, 12 passed
  - `binary(contract_diag_schema) | binary(contract_bounded_readers) | binary(contract_error_bodies) | binary(cross_exit_causes)` — green · 127 run, 127 passed
  - `binary(cli_list) | binary(cross_list_rows)` — green · 31 run, 31 passed
  - the console-action `jq` probe — green · last line `verb,exit_code`
  - the console-key `jq` probe — green · last line `count,key`
  - `cargo deny check` — green · exit 0
  - `hostwatch.py read --last 15` — recorded · QUIET - 0 s stalled on IO (15:44:13Z..15:59:13Z)
  - `bash scripts/agent-run.sh run --unit` — green · `"ok":true` · nextest-unit 3523 passed
  - `bash scripts/agent-run.sh run --integration` — green · `"ok":true` · nextest-integration 704 passed
  - `bash scripts/agent-run.sh run --e2e` — green · `"ok":true` · nextest-e2e 397 passed
  - `bash scripts/agent-run.sh pre-push`, `gate.py hygiene`, the push, `ci.py conclusion` — not run — leg operator (the operator pass)
- Remove-the-guard controls: Part 1's four hold (`evidence/control-1.txt`, not re-run, plan Implementation notes); Part 2's two (`gate.py control --timeout 900`, `.andromeda/cache/chunks/2026-11-28-console-actions/evidence/control-2.txt`): `text-fold` holds, `flag-value` holds — `control: 2 holds · 0 timeout · 0 does not hold · 0 unreproduced · clone removed`.
- Smoke: skipped — no `role = 'smoke'` entry and the `viola run` boot path untouched; the changed `viola console` ran as the built binary under an outer PTY in `binary(tui_console)`.
- Watches: none folded.
- Outcome basis: this run, and run `2026-11-28T14-52-37Z-implement`'s outcome file (Part 1), re-read at the tree.
- Process hygiene (`gate.py census --comm viola --comm viola-fake-agen --comm foot --comm claude --args projects/viola/target`, 6 running): none started by this run survives.

  | process | started by | final state |
  |---|---|---|
  | foot + claude `overseer1` (529738, 529756) | not this run (the host's up.sh) | left running: not this run's — the operator's |
  | foot + claude `viola-overseer` (529781, 529823) | not this run (the host's up.sh) | left running: not this run's — the operator's |
  | `~/.viola/bin/0.1.0-…/viola mcp` (537285) | not this run | left running: not this run's — its session's |
  | `~/.local/viola-product/bin/viola wait viola-builder …` (3031796) | not this run (the overseer's bridge) | left running: not this run's — the overseer's |
  | test wrappers, fake agents, stand-in foot trees of the gate block and the controls | this run (entries 4–12, 17–19; the control clone) | terminated (none in the census) |
