@task_prose part=chunk
Builds the frame `viola console` draws past the scaffold's one fixed frame: the fleet's lineage trees, each session's state and wheel holder in the board's words, and a console-only LAST cell for last activity.
Every drawn cell is control-escaped; each row is cut to the terminal width and a fleet past the frame ends in a `... <n> more` line.
The frame stays current: the board is re-read once a second, a redraw happens only when the drawn rows change, and the agents read runs at most every 5 s off the drawing loop.
The console-frame event gains shown and hidden counts. The live tiled quarter-screen reading at the operator's host is gated and is not fired.
@task_file n=1 path=src/human/console.rs mark=edits
what: the console's one frame writer, hand-written whole-frame redraw from a cleared top-left — edits: the frame it writes (rows, captions, LAST cell, row cut, overflow line)
@task_file n=2 path=src/console/mod.rs mark=edits
what: the console loop: the 100 ms size poll and the key read — edits: console::run, which takes an injected board source, the once-a-second board re-read and the redraw on a change
@task_file n=3 path=src/console/view.rs mark=edits
what: the console's frame model (new): the drawn rows built from the board source, the row cut and the overflow line
@task_file n=4 path=src/cmd/console.rs mark=edits
what: the viola console verb that hands the loop its terminal and its board source
@task_file n=5 path=crates/viola-state/src/board.rs mark=edits
what: the board read model, scan and read — edits: the board row, which gains the unserialised last-activity time taken in the pass the read already makes
@task_file n=6 path=src/human/board/rows.rs
what: the board row builder: the lineage forest ordering and the six shared board captions in their slot order
@task_item n=1 part=needs
The lineage trees: the fleet's instances drawn as parent/child trees in the order and indentation the board already derives for viola list, through its lineage forest, never a second tree builder.
@task_item n=2 part=needs
Each session's state shown in the board's own words from the same board read model: LIVE live or stale, STATUS idle, busy or unknown, DIALOG none or DIALOG <kind>; stalled is shown as stale, never a new word.
@task_item n=3 part=needs
The wheel holder of each session in the board's words: driver, human or unknown.
@task_item n=4 part=needs
Last activity: the age of the newest events log line, taken in the board read's own pass, held in the board row but never serialised, drawn by the console alone as a seventh cell captioned LAST after the six.
@task_item n=5 part=needs
LAST never reaches list --json, the MCP list item or the viola list board: the board row's new field stays out of every serialised and verb-drawn form.
@task_item n=6 part=needs
The six shared board captions are reused with their words and slot order unchanged (Shared captions fixed), with LAST after them.
@task_item n=7 part=needs
Every disk- or upstream-sourced cell the view draws passes the board's row escaping as the board's cells already do; no raw C0, C1 or DEL byte reaches the screen.
@task_item n=8 part=needs
Kept current: the board re-read once a second, a redraw only when the drawn rows change, the claude agents read at most once every 5 s and off the drawing loop.
@task_item n=9 part=needs
The console frame floor of 80x24: the frame stays whole at or above it; a 24-row frame holds 22 rows between its title and its key line.
@task_item n=10 part=needs
The row cut: each row keeps its six cells and LAST, and is cut at the terminal width ending `...`, never inside an escape; the cells after CLI are cut first, then CLI.
@task_item n=11 part=needs
A fleet past the frame: rows drawn in the board's order until one row is left, then one line `... <n> more`.
@task_item n=12 part=needs
The console-frame event gains shown and hidden counts, and every list and schema that pins its fields follows them; console-key stays as it is.
@task_item n=13 part=needs
The board read's and the agents read's own lines (parse-rejected, state-recovered, the agents-probe pair) land in the console's role file.
@task_item n=14 part=needs
The symlink skip-count WARN line is written by the console only when the count changes: a named console exception to the skip-logged-as-count rule.
@task_item n=15 part=needs
The frame writer stays hand-written: each draw redraws the whole frame from a cleared top-left, in plain ASCII with no SGR, through the one writer; the size is polled every 100 ms and re-read on each key, and a changed size redraws whole.
@task_item n=16 part=needs
The terminal fleet console is its own surface and is governed by no verb-output rule, as the reworded console-own-surface and no-prompt-or-pager rows hold; the rows his answers reword land at the wrap as his.
@task_item n=17 part=needs
No key beyond the scaffold's (q, Ctrl-C, the unbound rest): no action, send or selection is added to the console.
@task_item n=18 part=needs
The test seam: the frame model in its own module with the board source injected into console::run, so the view is proved without a live fleet.
@task_item n=19 part=needs
Proof: booted fake-agent wrappers for live rows and planted instance directories for the size cases, every home test-owned; no real claude, no read of the operator's real claude or viola homes, no window opened or compositor dispatch.
@task_item n=20 part=needs
The compositor half (the window opens tiled at a quarter of the screen) is shown only by a live reading at the operator's host, his to grant and time; the code proves only the frame half.
@task_standing n=1
convention: every state a drawn row carries is said by a word, never by colour or a glyph alone
home: surface-cli
@task_item n=1 part=not_needed
The web surface: viola renders no page and this chunk adds none.
@task_item n=2 part=not_needed
The send verb's rows (readback mirror, send output and refusals, prompt text from stdin or a file, progress patterns): the console sends nothing.
@task_item n=3 part=not_needed
Wheel and handoff verb output (pause, release, spoken phraseology) and the answer verb's output: the console takes no action.
@task_item n=4 part=not_needed
The run, revive, up and open start rows, their refusals, JSON error codes and window placement: the chunk starts no instance.
@task_item n=5 part=not_needed
The verify verb's output and refusals, the plugin install output, and link or unlink output: none of those verbs changes.
@task_item n=6 part=not_needed
The wait, last and switch verbs' output and the MCP tool results for send, wait, last and answer: untouched by the view.
@task_item n=7 part=not_needed
The exit-21 frame-verb hints and instance-unreachable details, and the help and per-verb usage rows: the chunk adds no verb, flag or refusal.
@task_item n=8 part=not_needed
The CLI colour token rendering rows (accent, surface, role and text colours, radius, spacing, motion tokens): the console draws plain ASCII with no colour.
