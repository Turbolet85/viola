@task_prose part=chunk
Builds the fleet console's view: the frame viola console draws now shows every instance of the home as its lineage tree, each row in the six board captions plus a seventh LAST cell.
Each row shows the session's state and wheel holder in the board's own words, and the age of its newest event line, with every disk- or upstream-sourced cell control-escaped.
The view keeps itself current: it re-reads the board once a second, redraws only when the drawn rows change, and reads the agents listing at most every 5 s, off the drawing loop.
The frame stays whole at any terminal size from the floor up: each row is cut at the width, ending with three dots, and a fleet too big for the frame ends with one more-count line.
The console-frame log event gains shown and hidden counts. The board row gains an unserialised last-activity time. The skip-count warning is logged only when the count changes.
@task_file n=1 path=src/console/view.rs
what: the console's frame model, a new file: lineage-ordered rows, the seven cells, the cut at the terminal width and the more-count line, built from an injected board source
@task_file n=2 path=src/console/mod.rs mark=edits
what: the fleet console's loop: the 100 ms size poll, the key read, the board re-read and the redraw — edits: console::run (it takes the injected board source), the once-a-second board re-read, the redraw only on a change, the agents read every 5 s off the loop, the skip-count warning only on a change, and the console-frame event's shown and hidden counts
@task_file n=3 path=src/human/console.rs mark=edits
what: the console's one frame writer, which redraws the whole plain-ASCII frame from a cleared top-left — edits: it now writes the view's rows, captions and more-count line
@task_file n=4 path=src/cmd/console.rs mark=edits
what: the viola console verb, which hands the loop its terminal — edits: it now also hands console::run the board source
@task_file n=5 path=crates/viola-state/src/board.rs mark=edits
what: the board read model (scan and read), read in-process by viola list and the console — edits: the board row gains a last-activity time taken from the newest events.ndjson line in the read's existing pass, never serialised
@task_file n=6 path=src/human/board/rows.rs mark=edits
what: the board's row builder: the lineage forest ordering and the six shared captions NAME, LIVE, STATUS, WHEEL, DIALOG, CLI — the console reuses them, words and slot order unchanged
@task_item n=1 part=needs
Lineage trees: the console draws the home's instances as parent/child trees, in the same order and indentation viola list's board already uses. It reuses the board's forest ordering and builds no second tree.
@task_item n=2 part=needs
Session state in the board's words, from the same board read model and agents listing: LIVE live or stale, STATUS idle, busy or unknown, DIALOG none or DIALOG with its kind. Stalled is shown as the board's stale, with no new word. An agents listing that cannot be read shows unknown.
@task_item n=3 part=needs
Wheel holder in the board's words: driver, human or unknown, as the board's WHEEL cell already shows it.
@task_item n=4 part=needs
The six board captions reused with their words and slot order (shared captions fixed), and a seventh cell, captioned LAST, after them. LAST shows the age of the newest event line.
@task_item n=5 part=needs
The board row's new last-activity time is never serialised: it stays out of the viola list --json items, the MCP list item and viola list's human board columns. The CLI and MCP list payloads stay identical, and the existing list column and JSON item checks still hold.
@task_item n=6 part=needs
Last activity is read in the pass the board read already makes over each events log. It goes through the tolerant event-log reader (complete lines only; torn, unknown and non-JSON lines counted and skipped, never a panic). The age is computed from an instant that is passed in, not read from a clock inside the read.
@task_item n=7 part=needs
Control escaping: every cell sourced from disk or upstream that the view draws goes through the human-output escaping, the same way list rows show control characters as hex. No raw C0, C1 or DEL byte reaches the screen, and a test plants such bytes in a name or a dialog kind.
@task_item n=8 part=needs
Width handling: the frame stays whole at or above the size floor. Each row keeps its seven cells and is cut at the terminal width, ending with three dots and never inside an escape. The cells after CLI are cut first, then CLI. Proved at many widths, including 80 columns, where no wrapped row fits whole.
@task_item n=9 part=needs
Height handling: a fleet taller than the frame draws in the board's order until one row is left, then one line giving the count of rows left out. A 24-row frame holds 22 rows between its title and its key line.
@task_item n=10 part=needs
Keeping current: the board is re-read once a second and the frame is redrawn only when the drawn rows change. The agents listing is read at most every 5 s, off the drawing loop, so a slow or hung listing never stalls a redraw or a key. Tests drive this cadence through the injected clock or a logged event, never through sleeps or retries.
@task_item n=11 part=needs
The console-frame log event gains shown and hidden row counts. Every log-field list and diagnostics schema that pins its fields gains them, the line stays a codes-only JSON line, and the schema conformance check still passes.
@task_item n=12 part=needs
The lines the board read and the agents read write into the console's role file: parse-rejected, state-recovered and the agents-probe pair. Their corr and level fields follow the log-event contracts, and they hold no secrets and no session text.
@task_item n=13 part=needs
The skip-count warning (symlinks and invalid names counted and skipped) is logged by the console only when the count changes, not on every once-a-second re-read. This is a named console exception to the logged-as-a-count rule, and a test reads the role file to assert it.
@task_item n=14 part=needs
The board source is injected into console::run: a product seam through which tests reach the frame model and the loop directly. Any double stays at that seam alone, and the board read model itself is still read from real files.
@task_item n=15 part=needs
Test inputs: rows for live sessions come from fake-agent wrappers booted under a test-owned home. Instance dirs for the size cases are produced by viola, never hand-written as state files. Every home is test-owned and private, and no test reads the host's ~/.claude or ~/.viola or runs the real claude CLI.
@task_item n=16 part=needs
The console's terminal behaviour must not change: plain ASCII with no SGR, a whole redraw from a cleared top-left, the 100 ms size poll, and a size change redrawing whole. Its keys stay q, Ctrl-C and unbound, a key after a resize still waits for the new size, and the terminal restore is still read on the console's own terminal.
@task_item n=17 part=needs
The tiled quarter-screen window clause is proved only by a gated live reading at the operator's host, recorded as a reading. No test opens, moves or focuses a window or sends anything to the compositor, and the chunk's pass fires nothing live.
@task_item n=18 part=needs
Frame oracles are literal: each expected frame, row, cut and more-count line is written as a literal string from the compiled captions and words. No assertion goes beyond those literals, and no test parses the rendered child screen.
@task_standing n=1
convention: each new test creates its own fresh, private home, and its fixtures come from real viola runs, never from real prompts or host data
home: 7-test-data-fixtures · test-data-fixtures
@task_standing n=2
convention: each new test waits on logged events or injected clocks, never on sleeps or retries, runs in parallel with no shared mutable state, and asserts with literal oracles on public behaviour
home: 11-test-anti-patterns-never-do-these
@task_standing n=3
convention: each new test follows the test naming rule, keeps unit tests inline and integration binaries prefixed by surface, uses rstest case tables for table-driven cases, and runs under nextest
home: 2-test-strategy · test-strategy
@task_standing n=4
convention: each new log line is a structured JSON line with codes only and its fields set by the log-line contracts, and passes the diagnostics schema check
home: log-format
@task_standing n=5
convention: each new or changed unit is held to the comprehensive tier's coverage floors, with no ignore parking and no widened coverage exclusions
home: 10-quality-gates-coverage-targets · quality-gates-coverage-targets
@task_standing n=6
convention: each new test that spawns viola or a wrapper ties it to a drop guard under a test-owned home, and drops inherited session and CLAUDE environment variables
home: integration-test-strategy · 5-integration-test-strategy
@task_item n=1 part=not_needed
Send, answer, release, wait and last verbs, their exit codes, refusal orders and JSON documents: the console takes no action and sends nothing. Console actions is a later chunk's.
@task_item n=2 part=not_needed
MCP frame tools, the MCP tools/list set, MCP invalid-params fields and the MCP reply screen: the chunk adds no tool and changes no MCP reply.
@task_item n=3 part=not_needed
Hook exec entry points, dialog decision bodies, hook deadlines and their performance budgets: the chunk changes no hook path.
@task_item n=4 part=not_needed
Wrapper channel JSON-RPC errors, link and unlink methods, the channel's MAX_FRAME and server identity checks: the console opens no channel frame.
@task_item n=5 part=not_needed
viola run and revive start, their refusal codes, exit-1 causes, the child environment strip and PTY passthrough: the chunk starts no instance and changes no start path.
@task_item n=6 part=not_needed
viola verify, ledger stamps, recorded fixtures per CLI version and the unverified-CLI gate: the chunk records and stamps nothing.
@task_item n=7 part=not_needed
Fake-agent command-line options, its receipts and turn scripts: the chunk uses the fake agent as it is and adds no option to it.
@task_item n=8 part=not_needed
Mutation testing triggers, mutation verdicts and kill lines: the chunk's pass runs no mutation.
@task_item n=9 part=not_needed
Fuzz targets, fuzz corpus replay and nightly fuzzing: the chunk adds no parser surface.
@task_item n=10 part=not_needed
CI workflow definitions, Actions pinning, artifact names and retention, and the pre-push gate's stages and reasons: the chunk adds no CI job and edits no workflow.
@task_item n=11 part=not_needed
Live readings of window raise, real adopt and fleet re-open (R3, R4, R5): the fleet on the founder's host is another chunk's.
@task_item n=12 part=not_needed
The harness session record, status document and supervisor exit file: the chunk changes no harness command's shape.
@task_item n=13 part=not_needed
Cross-user peer admission and euid checks: the console serves no peer.
