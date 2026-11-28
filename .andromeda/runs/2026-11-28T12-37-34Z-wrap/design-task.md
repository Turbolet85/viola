@task_prose part=session
The fleet console (viola console) now draws the fleet: the board's rows in its forest order between the title and the key line, in the board's state and wheel words, every cell control-escaped.
The console's caption row is the board's six captions, then a seventh, LAST: the age of an instance's newest log line in one unit (12s, 4m, 2h, 3d), unknown where a wrapped row has no log line, n/a on an unwrapped row.
A line wider than the terminal is cut at its columns ending in three dots, never inside an escape; a fleet taller than the body ends in a line of the form ... and a count, more.
The board is re-read once a second and the screen redrawn only when the view or the size changes; the claude agents list is read on its own thread at most every 5 s, and a failed read gives an empty list so STATUS reads unknown.
A home widened mid-run ends the console with exit 1 and the home-state refusal pair; an unlistable board directory ends it at the catch site as an internal error.
viola list, list --json and the MCP list item are unchanged; the new last-activity value is never serialised; matrix v2-14 stays gated on the operator's live reading.
@task_file n=1 path=crates/viola-core/src/obs.rs
what: the shared log-line vocabulary of the instrumentation, the console-frame line's fields
@task_file n=2 path=crates/viola-state/src/board.rs
what: the board read model, the strict-modes scan of the home and each instance's state read behind viola list and the fleet console
@task_file n=3 path=schemas/diag-line.v1.json
what: the diag-line schema every log line passes, its console-frame arm
@task_file n=4 path=src/cmd/console.rs
what: the viola console verb, its sources, its refusals and its agents read
@task_file n=5 path=src/cmd/list.rs
what: the viola list verb, the board document and the agents observations it joins
@task_file n=6 path=src/console/frame.rs
what: the fleet console's frame geometry, the body rows between the title and the key line
@task_file n=7 path=src/console/mod.rs
what: the fleet console's loop, its keys, its redraw on a change, its re-read interval and its mid-run refusal
@task_file n=8 path=src/console/fleet.rs
what: the fleet console's board source over the strict-modes scan, and its agents thread
@task_file n=9 path=src/console/view.rs
what: the fleet console's view, the drawn lines cut to the columns, the more line, the shown and hidden counts
@task_file n=10 path=src/human.rs
what: the human output module, the console frame writer with its body lines
@task_file n=11 path=src/human/board.rs
what: the viola list board renderer, its captions, empty-board line and unwrapped separator
@task_file n=12 path=src/human/board/cells.rs
what: the viola list board's cells, the STATUS, WHEEL and NAME cells and the row escaping
@task_file n=13 path=src/human/board/rows.rs
what: the viola list board's rows in forest order, the six captions, the NAME cut
@task_file n=14 path=src/human/board/test_support.rs
what: the board renderer's test fixture items
@task_file n=15 path=src/human/console.rs
what: the fleet console's frame drawing, the title, the body lines from row two and the key line
@task_file n=16 path=src/human/fleet.rs
what: the fleet console's fleet text, the seven-caption rows, the LAST age cell and the line cut ending in dots
@task_file n=17 path=tests/tui_console.rs
what: the fleet console's end-to-end checks under a pty, a test home and a test-scoped PATH
@task_item n=1 part=touches
Surface: console gains its fleet view; its rows hold only the keys today, and the body lines, caption row, LAST cell, cut, more line, re-read cadence, agents read and mid-run refusal belong beside them.
@task_item n=2 part=touches
The six fixed columns of viola list (decision/list-six-fixed-columns, the list columns contracts) stand for the list, but the console adds a seventh caption, LAST, after the six.
@task_item n=3 part=touches
Only an unwrapped NAME is truncated (decision/truncate-only-unwrapped-name, the NAME truncation and NAME cut contracts): the console does not cut the NAME in its fleet text, it cuts the whole line at the terminal's columns ending in three dots.
@task_item n=4 part=touches
Lines never wrapped and no hardcoded widths: the console's lines are cut at the terminal's columns, never wrapped; the cut never splits an escape.
@task_item n=5 part=touches
The width source of human tables and COLUMNS never read: the console's width is the terminal's size, as the frame reads it.
@task_item n=6 part=touches
Control characters escaped in human output and the list row escaping: every console cell is escaped with the same row escape as the list board.
@task_item n=7 part=touches
viola console refusals, home failing its read rule: the refusal now also comes mid-run, ending the console with exit 1, the home-state pair and one process-exit strict-modes-failed line, after the terminal is restored.
@task_item n=8 part=touches
CLI exit code 1 and the CLI catch site: the console's mid-run refused scan exits 1, and an unlistable board directory reaches the catch site as an internal error.
@task_item n=9 part=touches
Strict-modes failure refuses the whole board and the list's strict-modes scan: the console reads the board behind the same scan, now on every re-read.
@task_item n=10 part=touches
The list board's failed agents read and STATUS: in the console a failed agents read hands an empty list, so STATUS goes back to unknown and the unwrapped rows go too, rather than keeping a stale busy.
@task_item n=11 part=touches
Skipped count stays visible and the list's skip counts: the console logs its skip line only on a change of the count, while viola list still writes it per scan.
@task_item n=12 part=touches
No spinner or elapsed-time counter, no motion, busy has no animation: the console's LAST cell is an age that changes with time, redrawn when the view changes at a once-a-second re-read; a picker must judge it against these rows.
@task_item n=13 part=touches
No interactive prompt or pager and fleet console is no verb's output: the console now redraws when the fleet it shows changes; the planned rewords of these two rows were not carried (the operator's word).
@task_item n=14 part=touches
Stale rows in SGR dim and CLI colour limited to amber and dim: the console's fleet text is plain ASCII with no colour, so no dim or amber reaches its rows.
@task_item n=15 part=touches
Wheel shown as words and the flight-progress strip fields (STATUS, WHEEL, NAME, field with no reading): the console shows them in the board's own words, with unknown and n/a in LAST.
@task_item n=16 part=touches
Stable session slots and no auto-sorting of strips: the console keeps the board's forest order and indent.
@task_item n=17 part=touches
The list board's empty board line and unwrapped separator: the console reuses both in its fleet text.
@task_item n=18 part=touches
Fleet console keys q and Ctrl-C: they still quit at once while the agents read runs off the loop.
@task_item n=19 part=touches
ASCII-only output: the cut marker is three ASCII dots and the more line is plain text.
@task_item n=20 part=touches
--json result is one stdout document and the list board's json: list --json and the MCP list item carry no last-activity value; the board item gains it unserialised.
@task_fact n=1 slug=surface-console
fact: The console draws the board's rows in the board's forest order, in its state and wheel words, as body lines between the title and the key line.
@task_fact n=2 slug=surface-console
fact: The console's caption row is the board's six captions, then a seventh, LAST.
@task_fact n=3 slug=surface-console
fact: LAST is the age of the instance's newest log line in one unit (12s, 4m, 2h, 3d); unknown on a wrapped row with no log line, n/a on an unwrapped row.
@task_fact n=4 slug=surface-console
fact: A console line wider than the terminal is cut whole at its columns ending in three dots, never inside an escape; an unwrapped NAME is not cut separately in the console.
@task_fact n=5 slug=surface-console
fact: A fleet taller than the console's body rows ends in a line of the form ... n more.
@task_fact n=6 slug=surface-console
fact: The console re-reads the board once a second and redraws only when its view or the terminal size changes.
@task_fact n=7 slug=surface-console
fact: The console reads the claude agents list on its own thread at most every 5 s; a failed read gives an empty list, so STATUS reads unknown.
@task_fact n=8 slug=surface-console
fact: A home widened while the console runs ends it with exit 1 and the home-state refusal pair after the terminal is restored; an unlistable board directory ends it as an internal error.
@task_fact n=9 slug=surface-console
fact: The console's fleet text is plain ASCII with no colour.
@task_fact n=10 slug=surface-console
fact: The last-activity value is the console's alone; viola list, list --json and the MCP list item never carry it.
@task_item n=1 part=untouched
Brand identity's readback box, send readback, handoff and dialog-pending strip rows: no send, readback or dialog display changed.
@task_item n=2 part=untouched
Colour palette accent, typography, spacing, iconography glyph rows and the rendered-page and web-surface absences: no token, glyph or surface added.
@task_item n=3 part=untouched
The run, revive, up, verify, link, unlink, wait, last, focus, adopt, answer, pause and release output and refusal contracts: none of those verbs changed.
@task_item n=4 part=untouched
Exit 21 causes, refusal hints, peer reply screening, wrapper fault detail and wrapper socket rows: no IPC method, refusal word or hint added.
@task_item n=5 part=untouched
The command names, help verb groups, hook and mcp verb rows: no verb added, renamed or hidden.
@task_item n=6 part=untouched
The self-validation protocol rows: no check of the design process changed.
