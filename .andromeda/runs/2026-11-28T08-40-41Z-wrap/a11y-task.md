@task_prose part=session
The session built the scaffold of the terminal fleet console: a new verb viola console that takes over the human's own terminal on the alternate screen and redraws a two-line frame (viola console on the first row, q quit on the last), plain ASCII with no SGR and no colour.
Below the 80x24 floor the console draws one line naming the floor and the terminal's size; it re-reads the size every 100 ms and on each key, and redraws when the size changes.
It reads keys in raw mode through a bounded 64-byte reader decoded to three closed classes (quit, interrupt, unbound); q and Ctrl-C end it with exit 0, input end ends it too, and each key is logged by its class word, never its bytes.
It refuses with exit 1 and an unable/hint pair, before any log line, when stdin or stdout is not a terminal or when the home fails the strict read rule; on any exit, even an error or a caught panic, the terminal is restored before anything is printed.
It writes its own diagnostics as a fifth process role, console, into console.ndjson, with two new events console-frame and console-key; the help's board group now reads list, focus, console; a keystroke test driver checks the console by its log lines and by reading its terminal's line mode.
It also folded the verify typed turns' paste steps into one step function and the relied-row count into one constant, with verify's output unchanged.
@task_file n=1 path=src/cmd/console.rs
what: the viola console verb: its terminal check, its home read-rule refusal and the two refusal pairs, its own process-start, the console loop's start
@task_file n=2 path=src/console/mod.rs
what: the terminal fleet console's loop and its screen guard: alternate-screen enter, frame redraw on resize, the restore of the terminal before the console returns or unwinds
@task_file n=3 path=src/console/frame.rs
what: the console's 80x24 floor, the check whether a terminal size fits it
@task_file n=4 path=src/console/keys.rs
what: the console's key reader: a bounded raw stdin read decoded to the closed key classes quit, interrupt and unbound
@task_file n=5 path=src/human/console.rs
what: the console's one terminal writer: the alternate-screen enter and restore, the frame's title and key line, the one too-small line, plain ASCII with no SGR
@task_file n=6 path=src/human.rs
what: the human output module, the CLI's human-facing writers, gaining the console writer
@task_file n=7 path=tests/tui_console.rs
what: the console's keystroke test driver and its e2e cases: frame at and below the floor, resize redraw, key classes, interrupt, terminal restore on quit, the two refusals
@task_file n=8 path=src/main.rs
what: the CLI's verb list and dispatch, gaining the console verb and its help line
@task_file n=9 path=src/cmd/mod.rs
what: the CLI's command table and viola --help's group rows, the board row now list, focus, console
@task_file n=10 path=src/obs.rs
what: the root crate's diagnostics init and the catch-site internal-error process-exit line by role, now for run, cli and console
@task_file n=11 path=crates/viola-core/src/obs.rs
what: the closed diagnostics process roles and event catalog, gaining the console role and the console-frame and console-key events
@task_file n=12 path=schemas/diag-line.v1.json
what: the diagnostics line schema: the process enum, the event enum and each event's fields, gaining console, console-frame and console-key
@task_file n=13 path=schemas/diag-detail.v1.json
what: the diagnostics detail schema's process and event enums, gaining console and the two console events
@task_file n=14 path=tests/contract_diag_schema.rs
what: the contract check binding both diagnostics schemas' event enums to the one log-format event list
@task_file n=15 path=tests/cli_output_plain.rs
what: the plain CLI output check over the help text and its served-verb list, now holding console
@task_file n=16 path=tests/contract_error_bodies.rs
what: the error-body writer-and-witness census, its fixed stderr pairs with no document gaining the console's two refusal pairs
@task_file n=17 path=tests/cross_exit_causes.rs
what: the exit-cause table check, gaining the console's no-terminal and strict-modes-failed causes and its off-a-terminal refusal case
@task_file n=18 path=Cargo.toml
what: the root crate manifest, its test binary list gaining the console's e2e binary under the fake-agent feature
@task_file n=19 path=crates/viola-agent-claude/src/ledger.rs
what: the agent ledger's relied rows, gaining a public relied-row count constant
@task_file n=20 path=crates/viola-agent-claude/src/ledger/test_cases_stamps.rs
what: the agent ledger's stamp test cases, reading the relied-row count from the constant
@task_file n=21 path=src/cmd/verify.rs
what: the viola verify run's relied-row checks, reading the relied-row count from the constant
@task_file n=22 path=src/cmd/verify/typed/turns.rs
what: the viola verify typed turns, their pastes folded into one guarded paste-and-wait step, verify's output unchanged
@task_item n=1 part=touches
decision/static-linear-output gains an exception: the terminal fleet console, viola console, is its own surface that redraws a frame on the alternate screen when the terminal resizes; every other verb's output stays static and linear.
@task_item n=2 part=touches
The twin rows decision/linear-static-output and decision/linear-static-cli-output state the same linear static output rule and meet the same console exception.
@task_item n=3 part=touches
decision/cli-not-keyboard-interactive gains the named exception: the console reads keys in raw mode, q to quit and Ctrl-C to interrupt.
@task_item n=4 part=touches
decision/cli-no-interactive-input gains the same named exception: the console takes interactive key input on the human's own terminal.
@task_item n=5 part=touches
contract/cli-help/group-labels and the help's group rows: the board group's verbs move from list, focus to list, focus, console; bare viola still prints help on stderr with exit 2.
@task_item n=6 part=touches
contract/cli-help/served-verbs-only: console is a newly served verb listed in the help.
@task_item n=7 part=touches
contract/cli-refusal/format, contract/cli-refusal/unable-line and contract/cli-refusal/hint-line, and decision/refusal-ends-with-hint: the console adds two unable/hint refusal pairs with exit 1, one for no terminal, one for a home failing the read rule; the console takes no --json.
@task_item n=8 part=touches
decision/strict-modes-refusal-before-any-write and decision/unstattable-home-is-strict-modes-refusal: the console checks the home's read rule before any log opens and writes no line on refusal, an absent home passing.
@task_item n=9 part=touches
decision/error-bodies-have-writer-and-witness and decision/fixed-cli-strings-pinned-by-literal: the console's two refusal pairs are fixed stderr pairs pinned by literal in the error-body census and the exit-cause table.
@task_item n=10 part=touches
decision/no-sgr-in-plain-modes, decision/no-sgr-off-terminal and decision/colour-second-cue-only: the console draws plain ASCII with no SGR and no colour at all, on a terminal too.
@task_item n=11 part=touches
contract/log-identity/process: the process role words grow from four to five, run, hook, mcp, cli, console, in both diagnostics schemas.
@task_item n=12 part=touches
decision/run-diagnostics-off-terminal: the console's diagnostics go to its own role file console.ndjson, never to the terminal it draws on.
@task_item n=13 part=touches
decision/tui-boundary-assertions-only, decision/tui-boundary-only and decision/zero-viola-bytes-on-the-terminal speak of the viola run passthrough, where the terminal belongs to the child; the console is a different terminal surface where viola draws its own bytes, and these rows must not read as covering it.
@task_item n=14 part=touches
decision/tui-terminal-belongs-to-child: under viola console the terminal belongs to viola's own console, not to a child.
@task_item n=15 part=touches
decision/control-character-escaping and decision/c0-c1-escaped: the console prints no key byte at all; a key reaches the log only as its closed class word.
@task_item n=16 part=touches
decision/portable-pty-outer-pty-driver and decision/tui-boundary-driver-pin: the console's tests run it under the outer pseudo-terminal driver and read its line mode through the console's own terminal, since the driver exposes no master fd.
@task_item n=17 part=touches
decision/boundary-tests-sequenced-on-records and decision/no-screen-content-parsing: the console's cases are sequenced on its console-frame and console-key log lines, not by parsing the screen.
@task_item n=18 part=touches
decision/a11y-runs-in-existing-test-suites and decision/a11y-cases-inside-coverage-suite: the console's cases run in the existing e2e suite as a new test binary, with no a11y driver or suite of their own.
@task_item n=19 part=touches
decision/time-bounds-never-on-human and absence/user-time-limit: the console has no time limit on the human; it polls the size every 100 ms and waits on keys until q, Ctrl-C or input end.
@task_item n=20 part=touches
decision/screen-reader-target-tui and decision/manual-orca-pass: no screen-reader or manual keyboard pass has run on the console; its a11y checks are owed to the route entry Fleet a11y verification.
@task_item n=21 part=touches
The section 5 Keyboard Navigation overview already names the console's keyboard reach as owed to Fleet a11y verification; the scaffold's reach is q and Ctrl-C only.
@task_fact n=1 slug=5-keyboard-navigation
fact: The terminal fleet console's only keys are q to quit and Ctrl-C to interrupt, both exit 0; every other key is read as unbound and does nothing, and the end of its input ends it.
@task_fact n=2 slug=visual-design-verification
fact: The console's frame needs at least 80x24; below that floor it draws one line naming the floor and the terminal's own size instead of a frame, and redraws when the size changes.
@task_fact n=3 slug=6-visual-design-verification
fact: The console draws plain ASCII with no colour and no SGR: its title viola console on the first row and its key line q quit on the last row.
@task_fact n=4 slug=keyboard-test-harness
fact: The console's key reader is a bounded 64-byte raw read decoded to the closed classes quit, interrupt and unbound; a key is logged only by its class word, never its bytes.
@task_fact n=5 slug=cognitive-accessibility
fact: The console restores the terminal (leaves the alternate screen, shows the cursor, returns the line mode) before it returns, before any error line is printed and during a panic's unwind; an internal error then prints error: internal error once with exit 1.
@task_fact n=6 slug=cognitive-accessibility
fact: viola console refuses with exit 1 and an unable/hint pair, before any log line, when stdin or stdout is not a terminal, and when the home fails its read rule.
@task_fact n=7 slug=keyboard-test-harness
fact: The console's keystroke test driver reads its state from the console's own diagnostics lines and reads the line mode on the console's own terminal: raw while the frame stands, cooked after the quit.
@task_item n=1 part=untouched
The viola run TUI passthrough contracts (entry, human keystrokes, focus, mouse and resize sequences, vendor content, vt100 feed panic, pump_child error, refused peer): the passthrough itself is unchanged.
@task_item n=2 part=untouched
The viola list board rows (caption, order, colour, stale-row dim, NAME bold and truncation, DIALOG amber, the unwrapped separator): the board output is unchanged and the console shows no board yet.
@task_item n=3 part=untouched
The revive, link and unlink, plugin install, switch, up and wait output contracts and their --json refusal documents: none of those verbs changed.
@task_item n=4 part=untouched
The MCP frame tools, hook calls, frame-verb refusals and instance-unreachable detail rows: no frame, socket or peer path changed.
@task_item n=5 part=untouched
The send and dialog critical paths, send refused while the human types, dialog hold bounds and config dialog_hold_s: unchanged.
@task_item n=6 part=untouched
The absences of a web or graphical frontend, a rendered page, a touch surface and internationalization: the console is a terminal surface and none of them changes.
@task_item n=7 part=untouched
The a11y tier, compliance regime and WCAG conformance rows, the CI gate and SLO rows (fail-fast, no retries, no skips, mutation audit): no threshold or gate moved.
