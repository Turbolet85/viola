@task_prose part=session
The session added the terminal fleet console's scaffold: a new verb viola console (no arguments, the global --home only) that shows a quarter-screen frame in the human's own terminal and reads keys in raw mode.
The console enters the alternate screen with the cursor hidden and restores both on every end; at or above the floor 80x24 it draws the whole frame (viola console on row 1, q quit on the last row), below it one line naming the needed and the actual size; it redraws on a resize; plain ASCII, no colour.
Keys: q quits, Ctrl-C (0x03) ends with exit 0, every other key is unbound; end of input ends it with exit 0. Off a terminal, or with a home not private, it refuses with exit 1 and an unable/hint pair on stderr; a caught internal error restores first, then prints error: internal error, exit 1.
The viola --help board row now reads list, focus, console; bare viola is unchanged (help on stderr, exit 2). The console takes no --json and shows nothing of the fleet yet.
The diagnostics gained a fifth process role console and two events console-frame and console-key; viola verify's typed turns were refactored with no output change, and the relied-row count is now read from one constant.
@task_file n=1 path=src/cmd/console.rs
what: the viola console verb's start: the terminal check, the home check, its two refusal pairs, and the run of the console
@task_file n=2 path=src/console/mod.rs
what: the terminal fleet console's loop and its screen guard: enter, draw, redraw on resize, restore on every end
@task_file n=3 path=src/console/frame.rs
what: the console's frame floor, 80x24, and the check whether a terminal size fits it
@task_file n=4 path=src/console/keys.rs
what: the console's key reader: a bounded read decoded to quit, interrupt or unbound
@task_file n=5 path=src/human/console.rs
what: the console's one human writer: the enter and restore sequences, the frame lines and the one too-small line
@task_file n=6 path=tests/tui_console.rs
what: the console's end-to-end tests under its own pseudo-terminal: the frame, the floor, resize, keys, restore and the refusals
@task_file n=7 path=crates/viola-core/src/obs.rs
what: the diagnostics process roles and the closed event list
@task_file n=8 path=schemas/diag-line.v1.json
what: the diagnostics line schema: its process roles and its events
@task_file n=9 path=schemas/diag-detail.v1.json
what: the diagnostics detail schema: its process roles and its events
@task_file n=10 path=src/obs.rs
what: the diagnostics set-up per role, its role file names and the catch-site internal-error exit line
@task_file n=11 path=src/main.rs
what: the CLI's entry and its catch site for an internal error
@task_file n=12 path=src/cmd/mod.rs
what: the CLI's verb list and the viola --help verb group rows
@task_file n=13 path=src/human.rs
what: the human output module that holds every human writer of the CLI
@task_file n=14 path=tests/cli_output_plain.rs
what: the check that the CLI's human output is plain and that the help lists every served verb
@task_file n=15 path=tests/contract_error_bodies.rs
what: the census of the CLI's error bodies and its list of fixed stderr refusal pairs with no document
@task_file n=16 path=tests/cross_exit_causes.rs
what: the table of the CLI's exit causes, each read across the verbs
@task_file n=17 path=tests/contract_diag_schema.rs
what: the check binding the diagnostics schemas to the closed event list
@task_file n=18 path=Cargo.toml
what: the root crate's manifest and its list of test binaries
@task_file n=19 path=crates/viola-agent-claude/src/ledger.rs
what: the claude agent's ledger of relied rows and its count
@task_file n=20 path=crates/viola-agent-claude/src/ledger/test_cases_stamps.rs
what: the ledger's test cases over the relied rows
@task_file n=21 path=src/cmd/verify.rs
what: the viola verify verb's tests over the relied rows
@task_file n=22 path=src/cmd/verify/typed/turns.rs
what: viola verify's typed turns: the paste, the guard and the wait of each turn
@task_item n=1 part=touches
The viola --help verb group rows: the board row moves from list, focus to list, focus, console.
@task_item n=2 part=touches
No interactive prompt or pager: the console reads keys in raw mode and holds the terminal, an exception for the terminal fleet console as its own surface.
@task_item n=3 part=touches
Send lines appended, never redrawn, and any rule of static linear output: the console clears and redraws its frame on a resize, an exception for the console's surface.
@task_item n=4 part=touches
No persistent footer: the console's frame holds q quit on its last row, standing while the frame stands, an exception for the console.
@task_item n=5 part=touches
No banner anywhere and the only header is the BAY line: the console's frame holds the title viola console on its first row.
@task_item n=6 part=touches
The CLI holds every action and flat verbs plus a target: viola console is a new flat verb with no target and no argument.
@task_item n=7 part=touches
--json on every data verb: the console takes no --json; it is no data verb and prints no document.
@task_item n=8 part=touches
TTY detection through std IsTerminal: the console requires both stdin and stdout to be terminals and refuses otherwise.
@task_item n=9 part=touches
Results on stdout, the rest on stderr: the console's screen is its stdout; its refusals and its internal error go to stderr.
@task_item n=10 part=touches
The CLI refusal line form, a hint for every refusal cause, the hint last on stderr and hints that name the next verb: two new refusal pairs, unable: viola console needs a terminal with hint: run viola console in a terminal window, and unable: viola console could not use its home with hint: make the viola home readable by you alone, then run viola console again.
@task_item n=11 part=touches
The CLI exit codes: the console's two refusals exit 1, not one of the coded exits; q, Ctrl-C and end of input exit 0.
@task_item n=12 part=touches
The CLI internal error in human output: the console restores the terminal first, then prints error: internal error once, exit 1.
@task_item n=13 part=touches
The frame verbs' strict-modes refusal and the home-tree refusals: the console applies the home's read rule before any log opens and refuses a home not private with its own pair, no line written.
@task_item n=14 part=touches
Hand-written SGR styling, the human TTY expression level, plain output modes and one token vocabulary on the CLI: the console writes plain ASCII with no SGR and no colour at any depth.
@task_item n=15 part=touches
Human output is a contract: the console's frame lines, its too-small line and its refusal pairs are new human output.
@task_item n=16 part=touches
The per-verb usage of viola verb --help: viola console gains its usage, Show the terminal fleet console in this terminal.
@task_item n=17 part=touches
The MCP server with no human surface and viola mcp hidden: the console is the human's own surface beside the CLI, not served over MCP.
@task_fact n=1 slug=new
fact: A Surface: console area: the terminal fleet console, viola console, is its own surface in the human's terminal, entered on the alternate screen with the cursor hidden and restored on every end, plain ASCII and no colour.
@task_fact n=2 slug=new
fact: The console's frame floor is 80x24: at or above it the whole frame from a cleared top-left, viola console on row 1 and q quit on the last row; below it one line, viola console needs 80x24; this terminal is <cols>x<rows>; an unread size counts as 0x0, not fitting.
@task_fact n=3 slug=new
fact: The console redraws its frame when the terminal size changes, the size re-read on each key and every 100 ms; it draws once while the size stands.
@task_fact n=4 slug=new
fact: The console's keys: q quits and Ctrl-C ends it, each exit 0; every other key is unbound and does nothing; the end of input ends it with exit 0.
@task_fact n=5 slug=new
fact: The console's fleet view columns and its action keys are owed, to Fleet console view and Console actions; the scaffold shows nothing of the fleet.
@task_fact n=6 slug=surface-cli
fact: A CLI verbs contract item for viola console: no argument, the global --home only, no --json, exit 0 on quit, interrupt or end of input, exit 1 on a refusal or an internal error.
@task_fact n=7 slug=surface-cli
fact: The console's refusal pairs: off a terminal, unable: viola console needs a terminal with hint: run viola console in a terminal window, before any home read; a home not private, unable: viola console could not use its home with hint: make the viola home readable by you alone, then run viola console again; each exit 1.
@task_fact n=8 slug=surface-cli
fact: No interactive prompt or pager, static output and no persistent footer each take the exception of the terminal fleet console, its own surface, which reads keys in raw mode and redraws.
@task_item n=1 part=untouched
The web surface absence: viola still renders no page.
@task_item n=2 part=untouched
The viola list board, its BAY line, columns, colour depth and --json document: the console shows no board and the board's output is unchanged.
@task_item n=3 part=untouched
viola send output, its refusals, the readback mirror and its vocabulary: no send line moved.
@task_item n=4 part=untouched
viola run and viola revive starts, their refusals, usage and --json codes: unchanged.
@task_item n=5 part=untouched
viola link and unlink output, viola wait, viola answer, viola last and the wheel verb output: unchanged.
@task_item n=6 part=untouched
The viola mcp tools and tool results: no tool added or changed.
@task_item n=7 part=untouched
The exit-21 --json details and the frame verb exit-21 hints: the console binds no endpoint and prints no exit-21.
@task_item n=8 part=untouched
The CLI token rendering of colours, radius, spacing and motion: the console uses no token.
@task_item n=9 part=untouched
The absence of deep links or URL coupling in the CLI: the console holds no link.
@task_item n=10 part=untouched
viola plugin install, viola open and viola switch output: unchanged.
