@task_prose part=chunk
Builds the fleet console's view: the frame `viola console` draws past the scaffold's one fixed frame, showing the home's instances as lineage trees with each session's state, wheel holder and last activity in the board's words, every row control-escaped.
Keeps the frame current: the board re-read once a second, a redraw only when the drawn rows change, the agents read at most every 5 s off the drawing loop.
Keeps the frame whole at a quarter-screen tiled window: rows cut at the terminal width ending `...`, a fleet past the frame ending in one `... <n> more` line.
Adds an unserialised last-activity time to the board's row, `shown` and `hidden` counts to the `console-frame` event, and a skip-count WARN line written only on a change.
Lands his reworded rows (the fleet console as its own surface governed by no verb-output rule) and re-runs the row read once on the corrected scope.
@task_file n=1 path=src/human/console.rs mark=edits
what: the console's one frame writer, the hand-written whole-frame redraw from a cleared top-left in plain ASCII — edits: the frame it draws (the view's rows, captions, row cut and overflow line)
@task_file n=2 path=src/console/mod.rs mark=edits
what: the console loop — the 100 ms size poll, the key read and the redraw — edits: `console::run` (the board source injected into it), the once-a-second board re-read, the redraw on a change, the agents read off the loop, the skip-count line on a change
@task_file n=3 path=src/console/view.rs
what: the console's frame model — the fleet's rows, cells and overflow the frame draws, built from the injected board source
@task_file n=4 path=crates/viola-state/src/board.rs mark=edits
what: the board read model, its `scan` and `read` — edits: the board row (gains the last-activity time, the age of the newest events line, never serialised)
@task_file n=5 path=src/human/board/rows.rs
what: the board's row builder — the lineage ordering of `viola list` (the forest) and the six board captions with their slot order
@task_file n=6 path=src/cmd/console.rs
what: the `viola console` verb that hands the console loop its terminal
@task_item n=1 part=needs
Lineage trees: the console draws the home's instances as parent/child trees in the order and indentation the board already derives for `viola list` (its forest), never a second tree builder, and never re-sorting the strips.
@task_item n=2 part=needs
Session state in the board's own words, from the same board read model: LIVE live or stale, STATUS idle, busy or unknown, DIALOG none or `DIALOG <kind>`; «stalled» is the board's `stale` (a dated matrix note); no new vocabulary.
@task_item n=3 part=needs
Wheel holder shown as words, the board's own: driver, human or unknown.
@task_item n=4 part=needs
The last-activity cell: the age of the newest events line, taken in the pass the board read already makes, held in the board's row but never serialised, shown by the console alone as a seventh cell captioned `LAST` after the six; never added to `list --json`, the MCP list item or `viola list`'s board.
@task_item n=5 part=needs
The six board captions reused by the console with their words and slot order unchanged (NAME, LIVE, STATUS, WHEEL, DIALOG, CLI), `LAST` after them (the shared-captions carry).
@task_item n=6 part=needs
Row escaping: every disk- or upstream-sourced cell the view draws passes the board's row escaping as the board's cells already do; no raw C0, C1 or DEL byte reaches the screen.
@task_item n=7 part=needs
Kept current: the board re-read once a second, a redraw only when the drawn rows change, the agents read at most once every 5 s and off the drawing loop; the 100 ms size poll and the whole redraw on a changed size stay as the scaffold built them.
@task_item n=8 part=needs
Row width: the frame stays whole at or above the floor; each row keeps its cells and is cut at the terminal's width ending `...`, never inside an escape, the cells after CLI and then CLI cut first; lines are never wrapped and no width is hardcoded.
@task_item n=9 part=needs
Overflow: a fleet past the frame (22 rows between title and key line at 24 rows) draws the board's order until one row is left, then one line `... <n> more`.
@task_item n=10 part=needs
The `console-frame` event in the console's role file gains `shown` and `hidden` counts, and every list and schema that pins its fields follows; `console-key` is unchanged.
@task_item n=11 part=needs
The skip-count WARN line (symlink skips logged as a count) is written by the console only when the count changes: a named console exception to the skipped-count rule.
@task_item n=12 part=needs
The lines the board read and the agents read write into the console's role file (`parse-rejected`, `state-recovered`, the `agents-probe` pair) keep their form; the agents read is called as it is.
@task_item n=13 part=needs
The frame writer stays hand-written: each draw redraws the whole frame from a cleared top-left, plain ASCII with no SGR, through the one writer, with no cell grid or diffing.
@task_item n=14 part=needs
Fleet console keys unchanged: `q` and Ctrl-C quit, any other key is unbound; no action, send or selection is added.
@task_item n=15 part=needs
His reworded rows: the fleet-console-not-verb-output decision (and its no-prompt-or-pager twin) read «The terminal fleet console (`viola console`), its own surface, is governed by no verb-output rule»; the eight verb-output rows stand as they are, scoped by those two.
@task_item n=16 part=needs
Where the new code lives: the frame model in its own console view module, the board source injected into `console::run` so tests drive it without a real home.
@task_item n=17 part=needs
What the new tests owe: booted fake-agent wrappers for live rows, planted instance dirs for the size cases, every home test-owned; no real `claude`, no read of the operator's `~/.claude*` or `~/.viola`, nothing dispatched to the compositor.
@task_item n=18 part=needs
The console's refusals (no terminal, a home failing its read rule) stay as the scaffold built them while the view reads the board.
@task_standing n=1
convention: every human-facing line the console draws escapes control characters before it reaches the terminal
home: anti-patterns-never-do-these
@task_standing n=2
convention: every line the console draws is ASCII-only in the terminal's own font, no glyph outside ASCII
home: typography
@task_item n=1 part=not_needed
Send readback, readback box and handoff transfer marker contracts: the console sends nothing and confirms nothing in this chunk; those belong to the console actions chunk.
@task_item n=2 part=not_needed
The verb-output contracts and outcome words of other verbs (up, run, revive, verify, wait, link, unlink, focus, last, plugin install): the chunk adds and changes no verb output.
@task_item n=3 part=not_needed
Exit codes, exit-21 causes, revive and focus `--json` refusal codes and the fixed refusal pairs of other verbs: the console's view adds no refusal or exit path.
@task_item n=4 part=not_needed
The mcp and hook verbs, wrapper fault `--json` detail, the wrapper socket and peer reply screening: the chunk reaches no wrapper or MCP path.
@task_item n=5 part=not_needed
The web-spa surface and the rendered-page absence: viola renders no page.
@task_item n=6 part=not_needed
The command-name register and `viola --help` verb groups: the chunk adds no verb.
@task_item n=7 part=not_needed
CLI verification state contracts (verified, unverified-cli): the chunk changes no CLI verification.
