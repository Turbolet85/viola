@task_prose part=session
The session built the fleet console's view: viola console now draws the board's lineage trees, state words, wheel and a seventh LAST cell (last activity) between its title and its key line, every cell control-escaped.
The view re-reads the board once a second, redraws only when the view changes, cuts a wide line at the terminal's columns ending in ..., and ends a tall fleet with a ... <n> more line.
The console reads the agents list on its own thread at most every 5 s, off the key loop; a failed read hands an empty list, so STATUS reads unknown.
A home widened mid-run ends the console with exit 1, the home pair and one process-exit strict-modes-failed line; an unlistable board directory ends in error: internal error.
The console-frame diagnostic line gains shown and hidden (3 to 5 fields) and is written only on the first draw and on a change of its fields; viola list, list --json and the MCP list item are unchanged.
Nothing retired; no verb, flag, exit code, config key, event, endpoint or dependency added; matrix v2-14 stays GATED on the operator's live reading.
@task_file n=1 path=crates/viola-core/src/obs.rs
what: the shared observability vocabulary — the diagnostic line's fields and event subjects, here the console-frame line's new shown and hidden fields
@task_file n=2 path=crates/viola-state/src/board.rs
what: the board read model — the scan of instances, each instance's snapshot-or-replay read and the board item, now with a last-activity time never serialised
@task_file n=3 path=schemas/diag-line.v1.json
what: the diagnostic line schema — its console-frame arm, now requiring cols, rows, fits, shown and hidden
@task_file n=4 path=src/cmd/console.rs
what: the viola console verb — wiring the console's board, size and clock sources and its agents read every 5 s, and its exit on a refused scan
@task_file n=5 path=src/cmd/list.rs
what: the viola list verb — the board read and the agents observations joined into the STATUS column, shared with the console
@task_file n=6 path=src/console/frame.rs
what: the console frame geometry — the frame floor and the body rows left between the title and the key line
@task_file n=7 path=src/console/mod.rs
what: the terminal fleet console's loop — raw mode and restore, key handling, the once-a-second board re-read, redraw on a change and the console-frame line
@task_file n=8 path=tests/tui_console.rs
what: the console's end-to-end tests under a pty and a test home, each start with a test-scoped PATH so no real claude is read
@task_file n=9 path=src/human.rs
what: the human output module — the console frame writer, now drawing the body lines, and the fleet text module it exports
@task_file n=10 path=src/human/board.rs
what: the viola list board rendering — the board's captions, rows and empty-board line, now shared with the console
@task_file n=11 path=src/human/board/cells.rs
what: the board's cells — the state, wheel, CLI and dialog words of a board row, row-escaped
@task_file n=12 path=src/human/board/rows.rs
what: the board's rows — the lineage forest order and indent, the six shared captions, the unwrapped rack separator
@task_file n=13 path=src/human/board/test_support.rs
what: the board rendering's test fixtures — the board item literal
@task_file n=14 path=src/human/console.rs
what: the console frame's drawing — the title line, the body lines from row two and the key line
@task_file n=15 path=src/console/fleet.rs
what: the console's board source — the strict-modes scan re-read without replaying an unchanged log, the skip line on a change of its count, and the agents thread
@task_file n=16 path=src/console/view.rs
what: the console's view — the fleet lines cut to the columns and the rows, the ... <n> more line, and its shown and hidden counts
@task_file n=17 path=src/human/fleet.rs
what: the console's fleet text — the board's rows with the seventh LAST caption, the age cell in one unit with unknown and n/a, and the line cut ending in ...
@task_item n=1 part=touches
The shared captions decision: the console's caption row is the board's six captions, then a seventh, LAST, added after them; the row saying the captions are fixed is stale and gains that exception.
@task_item n=2 part=touches
The console as its own surface: the console now draws the fleet (lineage trees, state words, wheel, LAST) in the board's words, not a scaffold with no capability; the console surface's rows gain the view.
@task_item n=3 part=touches
The console frame floor of 80x24: the frame's fields grow from cols, rows, fits to five with shown and hidden, written on the first draw and then only on a change of those fields; the floor value itself is unchanged.
@task_item n=4 part=touches
The viola console verb contract: it now re-reads the board once a second, reads the agents list at most every 5 s off its loop, and ends with exit 1 and the home pair on a home widened mid-run, or error: internal error on an unlistable board directory.
@task_item n=5 part=touches
No interactive prompt or pager: the console now redraws when the fleet it shows changes; the reword of that row is the operator's (fork a), not carried by code, and stays his.
@task_item n=6 part=touches
Truncate only unwrapped names, and the list board's NAME truncation: in the console an unwrapped NAME is not cut by the fleet text; the whole line is cut at the terminal's columns instead, ending in ..., never inside an escape.
@task_item n=7 part=touches
Board row escaping: every console cell is row-escaped the same way the board's are (control fleet-escape).
@task_item n=8 part=touches
Words carry every state and the board's state words: the console's STATE, WHEEL, CLI and DIALOG cells are the board's words; LAST reads an age in one unit (12s, 4m, 2h, 3d), unknown or n/a, no glyph and no colour.
@task_item n=9 part=touches
The failed agents read and STATUS join on the board: in the console a failed agents read hands an empty list, so STATUS reads unknown and the unwrapped rows go too, rather than keeping a stale busy.
@task_item n=10 part=touches
Indirect, through the board read model and the board rendering now shared with the console: the viola list board's tree, captions, unwrapped rack separator, empty board and list --json document are rebuilt on shared functions but their lines and document are unchanged; the new last-activity time is never in list --json or the MCP list item.
@task_item n=11 part=touches
No persistent footer and only header is the BAY line, as they bear on the console: the console keeps its own title and key line, with the fleet body between them; no other header or footer added.
@task_item n=12 part=touches
No spinner and no progress glyphs: the console's once-a-second re-read and its 5 s agents read show no spinner or glyph; the frame is redrawn only when the view changes.
@task_item n=13 part=touches
Plain output modes and the uncoloured modes: the console's view is plain ASCII with no colour; colour depth unchanged.
@task_fact n=1 slug=surface-console
fact: The console draws the fleet as body lines between its title line and its key line, the board's rows in the board's forest order and indent.
@task_fact n=2 slug=surface-console
fact: The console's caption row is the board's six captions, then a seventh, LAST, whose cell is the age of the instance's newest log line in one unit (12s, 4m, 2h, 3d), unknown where no line, n/a on an unwrapped row.
@task_fact n=3 slug=surface-console
fact: A console line wider than the terminal is cut at its columns ending in ..., never inside an escape; an unwrapped NAME is not cut on its own.
@task_fact n=4 slug=surface-console
fact: A fleet taller than the body rows ends in a ... <n> more line, the shown and hidden rows counted.
@task_fact n=5 slug=surface-console
fact: The console re-reads the board once a second and redraws only when its view changes, none while size and view stand.
@task_fact n=6 slug=surface-console
fact: The console reads the agents list on its own thread at most every 5 s, off the key loop; a failed read leaves STATUS unknown.
@task_fact n=7 slug=surface-console
fact: A home widened while the console runs ends it with exit 1, the home refusal pair and one process-exit strict-modes-failed line; a board directory that cannot be listed ends in error: internal error.
@task_item n=1 part=untouched
The web surface absence: no page or web surface is added.
@task_item n=2 part=untouched
The send, readback mirror, wait, answer, last and switch output contracts and their refusals and hints: no send-side or dialog-side line changes.
@task_item n=3 part=untouched
The run, revive, adopt, open, up, focus, link and unlink output and refusal contracts: no start, window or link path changes.
@task_item n=4 part=untouched
The viola verify and plugin install outputs and refusals: untouched.
@task_item n=5 part=untouched
The CLI exit codes 10 to 21, the internal error forms, the global flags, the --json one-document rule and the exit-21 details and frame verb hints: no code, flag or detail added or moved.
@task_item n=6 part=untouched
The MCP tool set, its hidden verbs and its tool results: the MCP list item is unchanged and no tool is added.
@task_item n=7 part=untouched
The CLI token rendering rows (colour, spacing, radius, motion, roles) and the amber-means-dialog rule: the view adds no colour or token.
@task_item n=8 part=untouched
The help output, verb groups, flat verbs and clap parsing: no verb or flag is added.
@task_item n=9 part=untouched
The deep-links absence and the no-link-column rule: no link or URL is drawn.
