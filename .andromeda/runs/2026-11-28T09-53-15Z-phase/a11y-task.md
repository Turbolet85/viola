@task_prose part=chunk
Builds the view of the terminal fleet console (`viola console`), past the scaffold's one fixed frame: the instances drawn as their lineage trees in the board's order, each row with the six board cells in the board's words plus a seventh console-only cell captioned `LAST`.
The frame is kept current: the board re-read once a second, a redraw only when the drawn rows change, the `claude agents --json` read at most once every 5 s off the drawing loop.
Every drawn cell is control-escaped; each row is cut at the terminal's width ending `...`, and a fleet past the frame ends in one `... <n> more` line.
The board read model's row gains an unserialised last-activity time; the `console-frame` event gains `shown` and `hidden` counts; the skip-count WARN line is written by the console only when the count changes.
The compositor half (a tiled window at a quarter of the screen) is left to a live reading at the operator's host; this chunk's first pass fires nothing live.
@task_file n=1 path=src/human/console.rs mark=edits
what: the one frame writer of the terminal fleet console, redrawing the whole frame from a cleared top-left in plain ASCII — edits: the frame it writes (rows, captions, the `LAST` cell, the width cut, the `... <n> more` line)
@task_file n=2 path=src/console/mod.rs mark=edits
what: the console loop: the 100 ms size poll, the key read and the redraw — edits: `console::run` (the board source injected), the once-a-second board re-read, the redraw on a change, the 5 s agents read off the loop, the skip-count WARN line on a change only
@task_file n=3 path=src/console/view.rs
what: the console's frame model (new): the rows, cells and hidden count the frame draws, built from the injected board source
@task_file n=4 path=src/cmd/console.rs
what: the `viola console` verb that hands the console loop its terminal and its board source
@task_file n=5 path=crates/viola-state/src/board.rs mark=edits
what: the board read model, `scan` and `read`, read in-process — edits: the board row (gains the last-activity time, the age of the newest events line, never serialised)
@task_file n=6 path=src/human/board/rows.rs
what: the board row builder: the lineage ordering (`forest`) and the six board captions with their words and slot order, reused by the console
@task_item n=1 part=needs
Lineage trees: the console draws instances in the order and indentation the board already derives for `viola list` (its lineage ordering), not a second tree builder.
@task_item n=2 part=needs
Session state in the board's own words only: LIVE live/stale, STATUS idle/busy/unknown, DIALOG none or `DIALOG <kind>`; «stalled» is the board's `stale`; no new state vocabulary.
@task_item n=3 part=needs
Wheel holder shown in the board's words: `driver`, `human`, `unknown`.
@task_item n=4 part=needs
The six board captions reused with their exact words and slot order (NAME, LIVE, STATUS, WHEEL, DIALOG, CLI), with `LAST` added only as a seventh cell after them.
@task_item n=5 part=needs
`LAST`, the age of the newest events line, is taken in the pass the board read already makes, held in the board row but never serialised: it never reaches `list --json`, the MCP list item or `viola list`'s board.
@task_item n=6 part=needs
Control escaping: every disk- or upstream-sourced cell the console draws passes the row escaping as the board's cells do; no raw C0, C1 or DEL byte reaches the screen.
@task_item n=7 part=needs
Width: the frame stays whole at or above the floor; each row is cut at the terminal's width ending `...`, never inside an escape, the cells after CLI cut first and then CLI.
@task_item n=8 part=needs
Overflow: a fleet past the frame draws the board's order until one row is left, then one line `... <n> more`.
@task_item n=9 part=needs
Kept current: the board re-read once a second, a redraw only when the drawn rows change (or the size changes), the `claude agents --json` read at most once every 5 s off the drawing loop so it never stalls the frame or key read; that read failing leaves STATUS `unknown`.
@task_item n=10 part=needs
The console's redraw stays inside its named exceptions to static linear output and to no interactive input, which his answer widens to «…or the fleet it shows changes»: it redraws on a size change, a key or a fleet change, and on nothing else; the row edits land with the wrap.
@task_item n=11 part=needs
The frame stays plain ASCII with no SGR span, redrawn whole from a cleared top-left through the one frame writer.
@task_item n=12 part=needs
No key, action, send or selection beyond the scaffold's (`q`, Ctrl-C, «unbound»); no time bound falls on the human.
@task_item n=13 part=needs
The `console-frame` event gains `shown` and `hidden` counts, and every list and schema that pins its fields gains them; the `console` role file also carries the lines the board read and the agents read write (`parse-rejected`, `state-recovered`, the `agents-probe` pair).
@task_item n=14 part=needs
The skip-count WARN line of the symlink-skip decision is written by the console only when the count changes: a named console exception to that rule.
@task_item n=15 part=needs
Tests reach the frame through the frame model with an injected board source; live rows come from booted fake-agent wrappers, size cases from planted instance dirs, every home test-owned; no real `claude`, no read of the operator's real `~/.claude*` or `~/.viola`, nothing dispatched to the compositor.
@task_item n=16 part=needs
The tiled quarter-screen clause is a live reading at the operator's host that gates the matrix claim but never the build; no live start and no window opened, moved, focused or closed.
@task_standing n=1
convention: each new a11y assertion lives inside the existing test suites and fails as a nextest failure, with no separate a11y driver or suite
home: bootstrap-phases-derive-for-route-setup-project
@task_standing n=2
convention: each fixed string the console prints (`LAST`, `... <n> more`, the captions) is pinned by its literal in a test and comes from the design copy
home: 8-cognitive-accessibility · cognitive-accessibility
@task_standing n=3
convention: each new test is deterministic, with zero flakiness, no retries and no skips; a gate breach fails the build
home: 10-slo-invariants-a11y-budgets
@task_standing n=4
convention: each new a11y case runs inside the coverage suite of the GitHub Actions push/PR workflow, with no a11y artifact of its own
home: 9-ci-integration
@task_item n=1 part=not_needed
The `viola run` TUI passthrough contracts (entry, human keystrokes, focus, mouse and resize sequences, vendor content, vt100 feed panic): the console wraps no child and passes no terminal through.
@task_item n=2 part=not_needed
Screen-reader support rows (the Orca targets for the cli and the tui, the wrapped TUI reading as unwrapped): the console's a11y verification, keyboard reach and SC rows stay with «Fleet a11y verification».
@task_item n=3 part=not_needed
Refusal contracts of other verbs (revive and revive --json refusals, frame verb refusals, hook calls, MCP frame tools, start refusals, link and unlink, plugin install): the chunk adds no refusal path.
@task_item n=4 part=not_needed
The `viola --help` contracts and the CLI exit-code contracts: no verb is added and no exit code changes.
@task_item n=5 part=not_needed
The time bounds of other verbs (the dialog hold, `viola open`'s live wait, a switch's reap wait, the window step's wait, the compositor read deadline, the MCP wait tool's timeout): the chunk reaches none of those waits and dispatches nothing to the compositor.
@task_item n=6 part=not_needed
The absences of a web or graphical frontend, a touch surface, a rendered page and internationalization: the console is a terminal surface and adds none of these.
