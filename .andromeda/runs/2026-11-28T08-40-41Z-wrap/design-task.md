@task_prose part=session
Added the terminal fleet console scaffold: a new verb viola console, the human's own full-screen surface in the alternate screen, which shows a title line and a q quit line and nothing of the fleet yet.
The console refuses with exit 1 and a fixed unable and hint pair when stdin or stdout is not a terminal, or when the home fails its read rule.
It reads keys in raw mode: q quits, Ctrl-C ends with exit 0, every other key is unbound; it redraws when the terminal size changes; below an 80x24 floor it draws one line naming both sizes.
The help's board group now reads list, focus, console; the diagnostics gained a fifth process role console and two events console-frame and console-key; the catch-site role line owners became run, cli and console.
Two internal cleanups rode along with no output change: the relied-row count read from one constant, and viola verify's typed turns pasting through one step function.
@task_file n=1 path=src/cmd/console.rs
what: the viola console verb's entry: the terminal check, the home read-rule check, its two fixed refusal pairs, its own process-start line and the hand-off to the console loop
@task_file n=2 path=src/console/mod.rs
what: the fleet console's loop and its screen guard: enter the alternate screen with the cursor hidden, draw the frame, redraw on a size change, restore the terminal on every end
@task_file n=3 path=src/console/frame.rs
what: the fleet console's size floor, 80x24, and the check whether a terminal size fits it
@task_file n=4 path=src/console/keys.rs
what: the fleet console's key reader: a bounded raw read decoded to a closed key class quit, interrupt or unbound
@task_file n=5 path=src/human/console.rs
what: the fleet console's one human writer: the enter and restore sequences, the frame's title and key lines, the too-small line, plain ASCII with no colour
@task_file n=6 path=tests/tui_console.rs
what: the fleet console's end-to-end checks under its own pseudo-terminal: frame at the floor, below the floor, resize, unbound key, interrupt, restore on quit, the two refusals
@task_file n=7 path=crates/viola-core/src/obs.rs
what: the diagnostics process roles and the closed event catalog, now five roles and eighteen events
@task_file n=8 path=schemas/diag-line.v1.json
what: the diagnostics line schema: the process role enum and the event enum with their fields
@task_file n=9 path=schemas/diag-detail.v1.json
what: the diagnostics detail schema: its process role enum and event enum
@task_file n=10 path=src/obs.rs
what: the root diagnostics glue: the role log file names and the catch-site process-exit internal-error line per role
@task_file n=11 path=src/main.rs
what: the CLI entry and catch site: the verb dispatch and the internal error outcome per role
@task_file n=12 path=src/cmd/mod.rs
what: the CLI verb list and the viola --help verb groups
@task_file n=13 path=src/human.rs
what: the human-output module, declaring the console's writer
@task_file n=14 path=tests/cli_output_plain.rs
what: the plain-output check over every served verb and the help text
@task_file n=15 path=tests/contract_error_bodies.rs
what: the error-body census, with its list of fixed refusal pairs that carry no document
@task_file n=16 path=tests/cross_exit_causes.rs
what: the exit-cause table check across verbs
@task_file n=17 path=tests/contract_diag_schema.rs
what: the diagnostics schema contract check binding both schemas' event enums to one list
@task_file n=18 path=Cargo.toml
what: the root crate manifest, registering the console's end-to-end test binary
@task_file n=19 path=crates/viola-agent-claude/src/ledger.rs
what: the relied ledger rows of the claude agent, with the relied-row count as one constant
@task_file n=20 path=crates/viola-agent-claude/src/ledger/test_cases_stamps.rs
what: the ledger's stamp test cases, reading the relied-row count from the constant
@task_file n=21 path=src/cmd/verify.rs
what: the viola verify verb's checks, reading the relied-row count from the constant
@task_file n=22 path=src/cmd/verify/typed/turns.rs
what: viola verify's typed framing turns, each paste going through one step function, its output unchanged
@task_item n=1 part=touches
The viola --help verb groups contract: the board group's verbs move from list, focus to list, focus, console, in both restatements of the verb-groups row.
@task_item n=2 part=touches
The decision that the fleet console is no verb's output: the console is now reached by its own verb viola console, while what it shows stays its own surface and no other verb's output.
@task_item n=3 part=touches
The no interactive prompt or pager decision: the console reads keys in raw mode, a named exception for the terminal fleet console as its own surface.
@task_item n=4 part=touches
The static expression for the human CLI and the no motion decision: the console clears and redraws its frame when the terminal size changes, an exception for the fleet console surface.
@task_item n=5 part=touches
The command names: the name console is now taken by a verb beside list and focus.
@task_item n=6 part=touches
The TTY detection by std IsTerminal decision: the console checks both stdin and stdout and refuses when either is not a terminal.
@task_item n=7 part=touches
The fixed refusal pairs and the CLI refusal line unable and hint form: two new fixed pairs, the no-terminal pair and the console's home pair, both exit 1.
@task_item n=8 part=touches
The CLI exit codes 0 and 1: the console ends 0 on q, Ctrl-C or input end, and 1 on its refusals and on an internal error.
@task_item n=9 part=touches
The start refusals show no path or pid decision: the console's two refusals name no path, and no key byte or path reaches any log line.
@task_item n=10 part=touches
The silent catch sites decision and the CLI catch site contracts: the catch-site role line owners grow from run and cli to run, cli and console; the console restores the terminal first, then prints error: internal error once, exit 1.
@task_item n=11 part=touches
The ASCII-only human output and no background colour decisions, and the CLI colour amber-and-dim-only rule: the console's writer is plain ASCII with no SGR at all.
@task_item n=12 part=touches
The data on stdout, messages on stderr decision and the CLI streams contracts: the console's stdout is its screen, its refusals go to stderr.
@task_item n=13 part=touches
The COLUMNS is never read decision, the human tables width source, and no hardcoded widths: the console reads the terminal's size from the host terminal and holds a fixed 80x24 floor.
@task_item n=14 part=touches
The CLI callers contract for the human on a TTY: the console is a human-only verb with no --json and no argument but the global --home.
@task_item n=15 part=touches
The no spinner rule and the no spinners in verb output anti-pattern: the console redraws only on a size change, with no spinner or counter.
@task_item n=16 part=touches
The brand identity overview's terminal fleet console that inherits the direction: the console now exists as a scaffold, title and key line only.
@task_fact n=1 slug=new
fact: The terminal fleet console, viola console, is its own surface: it enters the alternate screen with the cursor hidden and restores both on every end, including an error or a panic.
@task_fact n=2 slug=new
fact: The fleet console's frame at or above the 80x24 floor is drawn whole from a cleared top-left, the title viola console on row 1 and q quit on the last row.
@task_fact n=3 slug=new
fact: Below the 80x24 floor the fleet console draws one line, viola console needs 80x24; this terminal is <cols>x<rows>, and an unread size counts as 0x0, not fitting.
@task_fact n=4 slug=new
fact: The fleet console redraws when the terminal size changes, the size re-read on a short poll and on each key.
@task_fact n=5 slug=new
fact: The fleet console's keys: q quits, Ctrl-C ends with exit 0, every other key is unbound and ignored; the end of its input also ends it with exit 0.
@task_fact n=6 slug=new
fact: The fleet console uses no colour and no SGR: plain ASCII text only.
@task_fact n=7 slug=surface-cli
fact: The name console is taken by the CLI verb viola console, in the board group, no arguments beyond --home and no --json.
@task_fact n=8 slug=surface-cli
fact: viola console off a terminal refuses with unable: viola console needs a terminal and hint: run viola console in a terminal window, exit 1, before any home read.
@task_fact n=9 slug=surface-cli
fact: viola console with a home failing its read rule refuses with unable: viola console could not use its home and hint: make the viola home readable by you alone, then run viola console again, exit 1, no line written and the mode left as found.
@task_fact n=10 slug=surface
fact: At the catch site the fleet console restores the terminal first, then prints error: internal error once on stderr, exit 1, as a cli role.
@task_item n=1 part=untouched
The colour palette and its accent, the CLI colour tokens and colour depth: the console uses no colour, so no token or accent rule moves.
@task_item n=2 part=untouched
The flight-progress strip, the readback box, the send readback and the handoff display: the scaffold shows nothing of the fleet, no strip, no readback.
@task_item n=3 part=untouched
The viola list board and table, its columns, NAME cut and skip counts: the board verb's output is unchanged.
@task_item n=4 part=untouched
The outputs and refusals of viola run, revive, up, link, unlink, wait, last, focus and send: none of those verbs' lines moved.
@task_item n=5 part=untouched
The exit 21 causes, the wrapper fault --json detail and the peer reply screening: the console binds no instance and no endpoint.
@task_item n=6 part=untouched
Iconography glyphs and separators, the web-spa and rendered-page absences, and the self-validation protocol: no glyph, page or check of these moved.
