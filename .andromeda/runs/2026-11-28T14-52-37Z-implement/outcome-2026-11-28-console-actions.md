# Report — 2026-11-28-console-actions

**Chunk:** the fleet console's actions — every verb by keyboard from a selected row, through the CLI verbs, a send's readback, refusals as `unable` lines (Part 1 of 2: the decoder, the selection, the scroll, the runner, the confirm and five verbs)
**Date:** 2026-11-28
**Run:** 2026-11-28T14-52-37Z-implement

## Changes (structured — detectors read this)
- **Files:** new `src/console/action.rs`; modified `src/console/keys.rs`, `src/console/mod.rs`, `src/console/view.rs`, `src/human/console.rs`, `src/human/fleet.rs`, `src/human.rs` (re-exports, companion), `src/cmd/console.rs`, `crates/viola-core/src/obs.rs`, `schemas/diag-line.v1.json`, `schemas/diag-detail.v1.json`, `tests/contract_diag_schema.rs`, `tests/tui_console.rs` (basis: `gate.py scope` — changed 13 · listed 12 · recorded 1).
- **Symbols / APIs:** (all `pub(crate)` in the root bin; no IPC method, endpoint, socket, port or env var added)
  - `console::action` (new): `Verb` {`Last`, `Pause`, `Release`, `Focus`, `Up`} with `word()` (the CLI verb and the `console-action` word) and `label()` (`revive` for `Up`, else the word); `Action { verb, name, args, stdin }` + `Action::on`; `Outcome { verb, name, exit_code, lines }`; `Runner::new(exe, home)`, `Runner::command` (`<exe> --home <home> <verb> <name> <args…>`, `VIOLA_NAME` / `VIOLA_DIR` removed through `STRIPPED`, stdin null or piped, stdout/stderr piped, no shell), `Runner::spawn` (one thread, outcome on an `mpsc` channel); `lines_of` (LF split, each line through `escape_row`). Each stream read on its own thread by `(&mut stream).take(MAX_FRAME).read_to_end(…)`, the rest past the cap drained to `io::sink`. A spawn failure or a signal end reads exit `-1`.
  - `console::keys`: `Key::of` and the scaffold's one-class-per-read `Key` {Quit, Interrupt, Unbound} retired; new `Key` {`Interrupt`, `Up`, `Down`, `Enter`, `Escape`, `Backspace`, `Char(char)`, `Unbound`}, `Decoder` (`feed(read, now)`, `deadline()`, `flush(now)`), `ESC_WAIT` 50 ms, a CSI read whole to its final byte (given up as `Unbound` at 32 bytes), SS3, one whole UTF-8 scalar across reads (C1 and malformed → `Unbound`). `spawn` now sends each read's bytes (`KeyRead = io::Result<Option<Vec<u8>>>`), `READ_CAP` 64 kept. Sole product caller of `keys::spawn`: `console::run`.
  - `console::view`: `View::of(lines, size, selected, result, keys)` (was `View::of(lines, size)`), `View::keys()`, `Selection` (`name()`, `follow`, `moved`), `wrapped_names`; the above line `... <n> above`; result rows at most a third of the body (`... <n> more` past them), laid in the body's lowest rows. Sole caller: `console::State::view`.
  - `console` (mod.rs): `Sources` gains a fourth parameter `act: A` (`FnMut(Action) -> Receiver<Outcome>`); private `Mode` {List, Confirm(Action)}, `Shown`, `State` (`view`, `press`, `reap`). `run`'s sole product caller stays `src/cmd/console.rs`.
  - `human`: `CONSOLE_KEYS` now `q quit  jk move  l last  p pause  r release  f focus  v revive`; new `console_confirm_prompt` (`<verb> <name>? y/n`), `console_running_line` (`<verb>  <name>  running`), `write_console_frame(out, rows, body, keys)` (gains `keys`); `FleetLine` gains `name: Option<String>` (a wrapped row's board item name); new `gutter`, `SELECTED_GUTTER` `> `, `GUTTER` two spaces; `escape_row` newly re-exported from `human`.
  - `viola_core::obs::ObsEvent::ConsoleAction` (`console-action`), `ALL` 18 → 19.
- **Crates / modules:** added module `src/console/action.rs`; changed `viola` (root bin) and `viola-core`.
- **Dependencies:** none.
- **Schema / config:** `schemas/diag-line.v1.json`: event enum + `console-action` message arm + field arm (`verb` enum `last` · `pause` · `release` · `focus` · `up` · `send` · `switch` · `answer` · `open`, `exit_code` integer, both required); the `console-key` arm's `key` enum grows from `quit` · `interrupt` · `unbound` to those three plus `up` · `down` · `action` · `busy` · `yes` · `no` · `escape` · `enter` · `backspace` · `text` (`enter` and `text` are Part 2's and written by no Part-1 path). `schemas/diag-detail.v1.json`: event enum + `console-action`. No config key.
- **Spec-master edits:** none.
- **Retired:** the scaffold's `Key::of` (one class per read) and its `key_of_reads_one_read_as_one_class` / `key_words_are_the_closed_three` cases, replaced by the sequence decoder and its `decoder_…` cases; the console key line `q quit` alone (now the bindings line). No census row stands for them.
- **Counts / qualifiers moved:** the closed event list 18 → 19 (`ObsEvent::ALL`, `LOG_FORMAT_EVENTS` in `crates/viola-core/src/obs.rs` and in `tests/contract_diag_schema.rs`, both schemas' event enums); the `console-key` word set 3 → 13 (`schemas/diag-line.v1.json`); the console key line's text (above).
- **Dev-tool versions:** none.
- **Harness / gate surface:** none.
- **Cross-project / external claims:** inputs#I1 (the overseer's directive, `../additional/viola-overseer/phase-console-actions-directive.md`, copy), inputs#I2 (the operator's own answers F1–F7 and the overseer's technical word, `../additional/viola-overseer/relay-console-actions-forks.md`, copy), inputs#I3 (the P5 word on `.andromeda/runs/2026-11-28T13-46-51Z-phase/p5-card-1.md`: yes to plan rev 3, by the delegate overseer — PROVISIONAL). That card's `## Beyond the plan` rows, verbatim:
  - Q8 and Q9 (`founder-forks.md`, the P4 block) — his to answer. Part 2 waits on both, and so does v2-15's claim.
  - No route entry is inserted (his F2/F7: one entry).
  - adopt from the console is not owned by any entry. If he wants it, it is a new route entry on his word: its proposed
    title would be «Console adopt», placed after «Console actions».
  The implement run's own directive (the invocation): Part 1 only, stop at Part 1's end, operator away, fakes only, no window, no real claude.
- **Reverted / negative API facts:** none.
- **Insufficient fixes (written, kept, not the remedy):** none.
- **Spec claims disproved by measurement:** plan step 9 names `console_refused_action_logs_the_verbs_exit_code` as «pause on a dead instance gives exit 21»; a dead (gone) instance draws no row to select (`crates/viola-state/src/board.rs:113` «a gone instance has no row»; `tests/tui_console.rs::console_draws_a_booted_wrapper_and_its_end` reads shown 1 → 0 at the stop). The case is built over a wrapper frozen until its heartbeat is stale, which `viola pause` refuses `instance-dead`, exit 21.
- **Expected amendments (from plan):**
  - design-system §Surface: cli (decision/list-is-the-board) — the console acts on the selected strip through the verbs: carried (**Symbols / APIs**, `console::action`), Part 1's five verbs.
  - layout-templates §Surface: cli (decision/board-vs-strip) — the same: carried.
  - security-plan §Input Validation + decision/console-runs-verbs-as-children: carried (`Runner::command`, `drain`, one at a time through `State::running`).
  - security-plan §Data Protection + decision/console-action-child-outlives-quit: carried (no parent-death tie; a running child's outcome is dropped at a quit).
  - design-system §Surface: console (contract/fleet-console-keys/unbound) and the bound keys and confirm prompts: carried for Part 1 (`jk` and the arrows, `l` `p` `f` `r` `v`, `y`/`n`/escape); Part 2's `s` `m` `a` `o` read `unbound`.
  - design-system §Surface: console + contract/fleet-console-keys/answer and /open: not carried — Part 2.
  - design-system §Surface: console (contract/fleet-console-view/more-line) + /above-line: carried (`View::of`, `... <n> above`).
  - a11y-plan §5 (decision/cli-no-interactive-input, decision/cli-not-keyboard-interactive, decision/static-linear-output): carried for the selection redraw and the confirm; the editing line is Part 2's.
  - obs-plan §1 (decision/no-interactive-terminal-interface): the same as the a11y line.
  - layout-templates §Surface: console (decision/console-frame-floor) — the key line keeps `q quit` first, then the bindings: carried (`CONSOLE_KEYS`).
  - layout-templates §Surface: console (decision/console-own-surface) — the editing echo, only on Q9 arm 1: not carried — Part 2, Q9 owed.
  - architecture §Standard Contracts + security-plan §Input Validation, only on Q8 arm 1: not carried — Part 2, Q8 owed.
  - security-plan §Input Validation (decision/console-key-reader-bounded) — the decoder's closed classes and the 50 ms lone-ESC wait, the 64-byte bound kept: carried (`Decoder`, `ESC_WAIT`, `READ_CAP`).
  - obs-plan §6 (contract/diag-line-fields/console-key) + contract/diag-line-fields/console-action, an obs Decisions Log entry: carried (**Schema / config**).
  - test-plan §3 (decision/harness-log-fields-owned-by-test-plan) — the console-action line among the grepped fields: carried (the event and its two fields).
  - matrix#v2-31 note: not this chunk's to write (P5 step 4's).
- **Coverage of new surfaces:**
  - `the console's key reader (stdin, raw)` → validation READ_CAP 64 a read + closed `Key` classes + CSI cap 32✓ · instrumentation `console-key` per key (a closed word)✓ · PII no key byte in any line✓ · tests unit (`decoder_…`) + e2e (`tui_console`) · a11y kbd: every Part-1 mode left by one key (`n`/escape from a confirm), Ctrl-C ends every mode✓ · tokens n/a
  - `an action child's stdout/stderr` → validation `take(MAX_FRAME)` per stream + `escape_row` per line✓ · instrumentation one `console-action{verb, exit_code}` per finished child✓ · PII no name, argument or output in any line✓ · tests unit (`action_…`) + e2e · a11y n/a · tokens n/a
  - `the action child's spawn (own exe, --home, VIOLA_NAME/VIOLA_DIR removed, no shell)` → validation `Runner::command`✓ · instrumentation the child's own `cli-<name>` log, as at a shell✓ · PII n/a · tests unit (`action_child_env_drops_the_instance_names`) + e2e · a11y n/a · tokens n/a
  - `the selection, the scroll, the result rows, the confirm prompt on the key row` → validation n/a · instrumentation `console-frame` shown/hidden✓ · PII n/a · tests unit (`selection_…`, `scroll_…`) + e2e · a11y kbd✓ (no key trap) · tokens n/a (plain ASCII, no colour)

## Deviations from intent
- **Decoder classes:** `q`, `j` and `k` decode as `Key::Char`; the list mode gives them their meaning (quit, down, up). The plan's class list names them as decoder classes; decoded as chars, Part 2's editing line can type them as text (plan step 10: «`q` and the verb letters are text here») with no second decoder.
- **The handover (step 8):** `src/cmd/console.rs` reads the canonical exe and the canonical home once (the home after the log made it, as `plugin.rs` does) and hands the loop an action starter, `Sources.act`, built from `Runner::new(exe, home)`, rather than the two paths themselves, so the loop's unit cases inject a starter and spawn nothing.
- **Revive's word:** the action line logs `up`, the CLI verb run; the running line and the confirm read `revive`, the human's word.
- **The gutter:** every fleet line (the caption, the empty-board line and the separator included) takes the two-column gutter, so the columns stay aligned; the above, more and result lines take none. At 80 columns a row's tail is cut two columns sooner; one existing unit case (`console_run_logs_a_frame_only_when_its_fields_change`) is widened to 100 columns so its LAST tick still changes the frame, and the scaffold's frame literals in `src/console/mod.rs` and `src/human/console.rs` carry the gutter and the bindings line.
- **Busy and no selection:** an action letter while a child runs reads `busy` and opens no confirm (`r` / `v` too); a letter with no selection reads `unbound` and runs nothing.
- **Plan step 9's refused case:** a frozen (stale) wrapper in place of a dead one — **Spec claims disproved**.
- **Plan step 9's scroll case:** run at 80x5, below the floor: a body that scrolls at 80x24 needs 22+ live wrapped rows; below the floor each `console-frame` still counts the rows the body would draw, so three booted wrappers read (1,2) → (2,1) → (1,2).
- **`console_last_draws_the_verbs_reply`:** asserts the console's `last` exit equals the shell's (E2E verdicts never read the screen); the drawn lines are pinned by the unit case `console_run_logs_a_finished_child_and_draws_its_lines`.
- **Unit escape case:** a lone ESC followed at once by a key decodes as one Alt sequence, so the loop case `console_run_release_runs_only_on_yes` leaves its second confirm by `n`, and the escape is pressed on the state itself (`console_state_escape_leaves_the_confirm_and_clears_a_result`); the E2E escape case waits for the `escape` key line before its next key.
- **contract_bounded_readers:** the two child reads pass the census by `take(` in their own statements; no `BOUNDARIES` row was added (the file is outside research's lists).
- **Part 2's entries** (the plan's Part-2 unit filter, Part-2 e2e filter and `binary(cli_list) | binary(cross_list_rows)`) were not fired: Part 1 only, by the invocation's directive.
- Scope record (`gate.py scope`: clean — changed 13 · listed 12 · recorded 1 (companion 1)):
  - companion: `src/human.rs` · serves src/human/fleet.rs · self

## Decisions & corrections
- The invocation's directive: Part 1 only, stop at its end with every layer green; operator away, no dialog; fakes only, no window, no real claude; a red with no failed assertion read against hostwatch first; times from `date -u`; no scheduled wake. Followed: no Part-2 key bound (`s` `m` `a` `o` read `unbound`), no Part-2 entry fired.
- Guard refusals, each re-issued once: the Bash guard refused a `cd` into the skill's references dir (re-issued as Read) and a `cat` heredoc to a scratch file (re-issued through the Write tool).
- Sweep hazard (test authoring): a lone ESC written right before the next key decodes with it as one Alt sequence (`ESC v` → `unbound`); a keystroke case writes its next key only after the `escape` key line lands.
- Measured: with the two-column gutter, an 80-column console row whose CLI cell reads `2.1.332 verified` is cut before its LAST cell (it was cut there already without the gutter: the row is 87 columns, `src/human/fleet.rs::fleet_cut_past_the_width_of_a_drawn_row_keeps_the_columns`).
- Measured: a gone instance draws no row, so no console action reaches it; `viola up <name>`'s revive is reached only from a row selected while the instance still stands (the revive E2E case opens the confirm, then kills the wrapper, then answers `y`).

## Outcome
As of this run's end: **green** (Part 1). Part 2 (steps 10–15) not started.
- Acceptance (security, F1): `action_child_env_drops_the_instance_names` and `contract_bounded_readers` green; the child is the own exe, `--home` passed, no shell, each stream through `MAX_FRAME` — met.
- Acceptance (product, v2-15 Part-1 legs): `l` `p` `f`, `r`·`y`, `v`·`y` from a selected row, each the verb's own record and exit and one `console-action{verb, exit_code}`: the Part-1 e2e filter green (11 cases) — met for Part 1; v2-15 stays unclaimed (its send, answer and open legs are Part 2's).
- Acceptance (product, Part 2: send, switch, open; answer on Q8): not in this run.
- Acceptance (a11y, no key trap): `console_ctrl_c_ends_from_every_mode` (list and confirm; the editing mode is Part 2's) and `console_release_confirm_left_by_escape_runs_nothing` green — met for Part 1's modes.
- Acceptance (security, the decoder): reads of at most 64 bytes, a sequence split across reads one key, no key byte in any line: the Part-1 unit filter green — met.
- Acceptance (obs): `console-action` in `ObsEvent::ALL`, both schemas' enums and both pinned lists; the schema probe reads `verb,exit_code`; `contract_diag_schema` and the core crate green — met.
- Acceptance (layout, F3/F6): `console_selection_keeps_its_name_when_the_fleet_changes` and `console_scroll_follows_the_selection` green — met (the scroll case below the floor, **Deviations**).
- Acceptance (tests): `scripts/agent-run.sh run --unit`, `--integration`, `--e2e` each `"ok":true` — met for Part 1.
- Gates (one block firing, `gate.py run --only 1,2,3,4,5,7,8,10,12,13,14,15,16`):
  - `cargo fmt --all --check` — green · exit 0
  - `cargo clippy --workspace --all-targets --features fake-agent -- -D warnings` — green · exit 0
  - `cargo nextest run -p viola-core --no-fail-fast` — green · exit 0
  - `cargo nextest run -p viola --features fake-agent --bins --no-fail-fast` — green · exit 0
  - the Part-1 unit filter (`console::(keys::tests::decoder_|view::tests::(selection_|scroll_)|action::tests::action_|tests::console_run_takes_one_action_at_a_time$)`) — green · exit 0 · 47 run, 47 passed
  - the Part-2 unit filter (`decoder_editing_|action_(send|switch|open)_`) — not run — Part 2
  - `binary(tui_console)` — green · exit 0 · 22 run, 22 passed
  - the Part-1 e2e filter (`^console_(selection_|scroll_|pause_|release_|last_|focus_|revive_|refused_action_|ctrl_c_ends_)`) — green · exit 0 · 11 run, 11 passed
  - the Part-2 e2e filter (`^console_(send_|editing_line_|switch_|open_|answer_)`) — not run — Part 2
  - `binary(contract_diag_schema) | binary(contract_bounded_readers) | binary(contract_error_bodies) | binary(cross_exit_causes)` — green · exit 0 · 127 run, 127 passed
  - `binary(cli_list) | binary(cross_list_rows)` — not run — Part 2 (step 14)
  - the `jq` console-action required-fields probe — green · exit 0 · last line `verb,exit_code`
  - `hostwatch.py read --last 15` — recorded · QUIET - 5 s stalled on IO (14:55:07Z..15:10:07Z)
  - `bash scripts/agent-run.sh run --unit` — green · `"ok":true` · nextest-unit 3497 passed
  - `bash scripts/agent-run.sh run --integration` — green · `"ok":true` · nextest-integration 704 passed
  - `bash scripts/agent-run.sh run --e2e` — green · `"ok":true` · nextest-e2e 385 passed
  - `bash scripts/agent-run.sh pre-push`, `gate.py hygiene`, the push, `ci.py conclusion` — not run — leg operator (the operator pass; not started, by the directive)
- Remove-the-guard controls (`gate.py control --timeout 900`, `.andromeda/cache/chunks/2026-11-28-console-actions/evidence/control-1.txt`): `env-strip` holds, `one-child` holds, `out-escape` holds, `confirm` holds (its case red on its own assertion, `[("release", 0)]`) — `control: 4 holds · 0 timeout · 0 does not hold · 0 unreproduced · clone removed`.
- Smoke: no `role = 'smoke'` entry and the `viola run` boot path untouched; the changed `viola console` start ran as the built binary under an outer PTY in `binary(tui_console)`.
- Watches: none folded.
- Outcome basis: this run alone (`.andromeda/runs/2026-11-28T14-52-37Z-implement/`).
- Process hygiene (`gate.py census --comm viola --comm viola-fake-agen --comm foot --comm claude --args projects/viola/target`, 6 running): none started by this run survives.

  | process | started by | final state |
  |---|---|---|
  | foot + claude `overseer1` (529738, 529756) | not this run (the host's up.sh) | left running: not this run's — the operator's |
  | foot + claude `viola-overseer` (529781, 529823) | not this run (the host's up.sh) | left running: not this run's — the operator's |
  | `~/.viola/bin/0.1.0-…/viola mcp` (537285) | not this run | left running: not this run's — its session's |
  | `~/.local/viola-product/bin/viola wait viola-builder …` (2711747) | not this run (the overseer's bridge) | left running: not this run's — the overseer's |
  | test wrappers, fake agents, stand-in foot trees of the gate block and the controls | this run (entries 7, 8, 14–16; the control clone) | terminated (none in the census) |
