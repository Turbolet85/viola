@task_prose part=session
The fleet console (viola console) now acts: a selected wrapped row takes every verb by one key, each run as a child through the CLI verb itself, its output drawn as result rows and a refusal as the verb's own unable line.
Keys bound: j/k and arrows move the selection, l last, p pause, r release, f focus, v revive, s send, m switch, a answer, o open; release and revive ask a y/n confirm; a opens a y/n answer prompt on a pending permission or plan, and draws one line pointing to the session on a question.
s, m and o open one editing line that echoes the text as typed (UTF-8, wide scalars two columns, past the edge the prompt, an ellipsis and the longest fitting tail); Enter hands it to the verb, Escape cancels.
Every mode is left by one key (n or Escape from a confirm, Escape from the editing line and the answer prompt) and Ctrl-C ends the console from every mode, cooked after; these no-key-trap cases are new e2e cases under an outer PTY.
Measured: at the 80-column floor the new key line is cut after "m swi", so "a answer" and "o open" do not show there; this goes to the operator as his fork, as built while unanswered.
Retired: the scaffold's three-word key classes and its key line of "q quit" alone; the live half (real terminals, real claude dialogs, his input method, his tile width) stays unproven and gated on his live reading.
@task_file n=1 path=src/console/action.rs
what: new — the fleet console's action runner: each verb run as a child of viola's own executable with --home, no shell, each output stream bounded and every output line escaped of control characters
@task_file n=2 path=src/console/keys.rs
what: edited — the fleet console's key reader: raw stdin decoded into a closed set of key classes, a short escape wait, a bounded read
@task_file n=3 path=src/console/mod.rs
what: edited — the fleet console's modes and key handling: the list, the confirm, the editing line and the answer prompt, one action at a time, the console's key log lines
@task_file n=4 path=src/console/view.rs
what: edited — the fleet console's view: the selection over wrapped rows, the scroll window with its above and more lines, the result rows, the fitted key line
@task_file n=5 path=src/human/console.rs
what: edited — the fleet console's fixed copy: the key line, the confirm, editing and answer prompts, the question line, the running line, the editing line's width rule
@task_file n=6 path=src/human/fleet.rs
what: edited — the fleet lines the console draws: a row's name, its pending dialog kind and id, the selection gutter mark
@task_file n=7 path=src/human.rs
what: edited — the human output module's re-exports (the console copy and fleet line types)
@task_file n=8 path=src/cmd/console.rs
what: edited — the viola console verb's entry: it builds the action runner over its own executable and home
@task_file n=9 path=src/cmd/list.rs
what: edited — the viola list verb's board item literal (a companion: the new dialog id field set empty)
@task_file n=10 path=src/human/board/test_support.rs
what: edited — the board rendering's test support item literal (a companion: the new dialog id field)
@task_file n=11 path=crates/viola-core/src/obs.rs
what: edited — the closed list of diagnostic events: a console-action event added, the console-key event's count documented
@task_file n=12 path=crates/viola-state/src/board.rs
what: edited — the board read of the fleet: each item now carries its pending dialog id, kept out of list --json and the MCP list item
@task_file n=13 path=schemas/diag-line.v1.json
what: edited — the run diagnostics line schema: the console-action line (verb, exit_code), the console-key line's key words grown and its count field
@task_file n=14 path=schemas/diag-detail.v1.json
what: edited — the diagnostics detail schema's event list: console-action added
@task_file n=15 path=tests/contract_diag_schema.rs
what: edited — the diagnostics schema contract check: the closed event list now 19
@task_file n=16 path=tests/tui_console.rs
what: edited — the fleet console's boundary tests under an outer PTY: selection, scroll, every verb, send, switch, open, answer, the no-key-trap cases
@task_file n=17 path=Cargo.toml
what: edited — the workspace manifest: unicode-width added as a dependency of the root bin
@task_file n=18 path=Cargo.lock
what: edited — the dependency lock: the root package's unicode-width line (no crate added)
@task_item n=1 part=touches
decision/cli-not-keyboard-interactive is now stale or needs its exception: the fleet console takes keyboard input — a moving selection, single-letter verbs, a y/n confirm, a y/n answer prompt and one editing line.
@task_item n=2 part=touches
decision/cli-no-interactive-input: the console's editing line (send, switch, open) and its prompts are interactive input; the rule needs the console named as its exception or a console row beside it.
@task_item n=3 part=touches
The section 5 Keyboard Navigation overview hands the fleet console's keyboard reach to a later route entry; this session brings that keyboard reach (bound keys, every mode left by one key, Ctrl-C from every mode), so the overview is stale or must point at it.
@task_item n=4 part=touches
decision/static-linear-output, decision/linear-static-output and decision/linear-static-cli-output: the console redraws its frame for the selection, the scroll window, the confirm, the answer prompt and the editing line; a rule of static linear output over the console now has an exception.
@task_item n=5 part=touches
decision/critical-path-send-readback: a confirmed send can now start from the console's editing line, handed to viola send unchanged on stdin, its readback drawn as result rows.
@task_item n=6 part=touches
decision/critical-path-dialog-answer: a pending permission or plan can now be answered from the console by y or n through viola answer; a question is not answered there, one line points to the session.
@task_item n=7 part=touches
decision/critical-path-human-takes-wheel: the console's f key focuses the selected session through viola focus, a new path to the wheel.
@task_item n=8 part=touches
contract/cli-refusal/unable-line, contract/cli-refusal/hint-line and contract/cli-refusal/format: a verb refused from the console is drawn as that verb's own unable and hint lines in the result rows, its exit code logged.
@task_item n=9 part=touches
decision/control-character-escaping, decision/c0-c1-escaped and decision/control-bytes-escaped: every line of an action child's output, and the editing line's echo, pass through the same control-character escape before the console draws them.
@task_item n=10 part=touches
absence/internationalization: the editing line now takes and echoes any UTF-8 text as typed (a Cyrillic send is a case), wide scalars counted two columns and width-0 ones none; still no i18n, RTL or lang convention, but the absence row now stands beside a width rule.
@task_item n=11 part=touches
decision/name-truncation and decision/unwrapped-name-truncation: a new truncation rule beside them — the editing line past the edge shows the prompt, an ellipsis and the longest fitting tail; the key line is cut at the 80-column floor.
@task_item n=12 part=touches
decision/colour-second-cue-only, decision/state-carried-by-printed-word and decision/no-information-by-colour-alone: the console's selection is carried by a gutter mark on the selected row alone and the editing line is plain text with no colour.
@task_item n=13 part=touches
decision/time-bounds-never-on-human, decision/time-bounds-fall-on-the-driver and decision/no-unwarned-timeouts: the console's key reader holds a 50 ms escape wait on the decoder; the confirm, the editing line and the answer prompt carry no time limit on the human.
@task_item n=14 part=touches
decision/fixed-cli-strings-pinned, decision/fixed-cli-strings-pinned-by-literal and decision/fixed-copy-from-design: new fixed console copy (the key line, the confirm, editing and answer prompts, the question line) pinned by literal tests.
@task_item n=15 part=touches
decision/tui-boundary-assertions-only, decision/tui-boundary-only and decision/a11y-inside-existing-suites: the console's no-key-trap checks run as e2e cases in its existing outer-PTY boundary suite, not a separate a11y suite.
@task_item n=16 part=touches
decision/manual-keyboard-pass, decision/live-readings-never-gate and decision/manual-a11y-pass-gates-nothing: the console's live half (real terminals and focus, real dialogs, the editing line under his terminal and input method, the key line at his tile width) is unproven until his live reading; the capability stays gated on it.
@task_item n=17 part=touches
decision/no-spinners: the console draws one static running line while a child runs; a key pressed meanwhile reads busy.
@task_item n=18 part=touches
decision/run-diagnostics-off-terminal: the console's new key and action log lines go to the diagnostics log, never the terminal, and hold no key byte, typed text or dialog text.
@task_item n=19 part=touches
decision/no-screen-content-parsing: the answer prompt reads only the board's dialog kind and id, never the dialog's text.
@task_fact n=1 slug=5-keyboard-navigation
fact: The fleet console is keyboard-driven: j/k or the arrows move a selection over wrapped rows, one letter runs each verb on the selected row (l p r f v s m a o), release and revive ask y/n first, q quits.
@task_fact n=2 slug=5-keyboard-navigation
fact: The fleet console has no key trap: n or Escape leaves a confirm, Escape leaves the editing line and the answer prompt, and Ctrl-C ends the console from every mode with the terminal restored.
@task_fact n=3 slug=5-keyboard-navigation
fact: The fleet console's one editing line (send, switch, open) is the named exception to no interactive CLI input: one line, echoed as typed, Enter hands it to the verb, Escape cancels and runs nothing.
@task_fact n=4 slug=5-keyboard-navigation
fact: The console answers a pending permission (y allow, n deny) or plan (y approve, n revise) through viola answer; a question is never answered in the console, one line sends the human to the session by f.
@task_fact n=5 slug=6-visual-design-verification
fact: The editing line's width rule: each scalar drawn at its terminal width (1, 2 or 0 columns); past the edge the prompt, an ellipsis and the longest fitting tail, a straddling wide scalar left out.
@task_fact n=6 slug=6-visual-design-verification
fact: At the 80-column floor the console key line is cut after m swi, so a answer and o open do not show there; q quit stays first; whether the floor shows every binding is the operator's open fork.
@task_fact n=7 slug=6-visual-design-verification
fact: The console's selection is carried by a gutter mark on the selected row alone, never by colour.
@task_fact n=8 slug=2-a11y-strategy
fact: A verb refused from the console is drawn as that verb's own unable and hint lines in the result rows; the console adds no refusal copy of its own.
@task_fact n=9 slug=keyboard-test-harness
fact: The console's no-key-trap checks (Ctrl-C from every mode, Escape from the confirm, the editing line and the answer prompt running nothing) are e2e cases under an outer PTY, failing as nextest failures.
@task_item n=1 part=untouched
The viola run TUI passthrough contracts (entry, human keystrokes, focus, mouse and resize sequences, vendor content, vt100 feed panic, zero viola bytes): viola run's terminal and its wheel are not changed.
@task_item n=2 part=untouched
The revive --json, run and start refusal documents and their reason codes: no refusal word, document or hint was added or moved.
@task_item n=3 part=untouched
The frame-verb, hook-call and MCP frame-tool refusal contracts and the instance-unreachable details: no IPC method, socket, endpoint or MCP tool was added.
@task_item n=4 part=untouched
The CLI exit-code contracts (2, 10, 11, 13, 14, 20, 21): the console relays each verb's exit code as the verb gives it; no code's meaning moved.
@task_item n=5 part=untouched
CI integration and SLO rows (the GitHub Actions gate, zero flakiness, fail-fast, no performance budget, coverage and doctest): no gate, threshold or harness surface changed.
@task_item n=6 part=untouched
viola --help contracts (usage line, group labels, handoff row, served verbs): no verb was added to or removed from the CLI.
@task_item n=7 part=untouched
The screen-reader targets and setups (Orca with GNOME Terminal, Orca with foot): no screen-reader target or test setup changed.
@task_item n=8 part=untouched
The structured violation JSON contracts and the absences of a web or graphical frontend, a touch surface and a rendered page: none changes; the console is a terminal surface.
@task_item n=9 part=untouched
The amber DIALOG word and bold NAME SGR decisions: no colour or SGR span was added or moved.
