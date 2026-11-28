@task_prose part=chunk
Builds the view `viola console` draws past the scaffold's one fixed frame: the fleet's lineage trees, each session's state and wheel holder in the board's words, and a console-only seventh cell `LAST` (age of the newest events log line).
Keeps it current: the board re-read once a second in-process, a redraw only when the drawn rows change, the claude agents read at most every 5 s off the drawing loop.
Fits the frame to the terminal: every disk- or upstream-sourced cell control-escaped, each row cut at the width ending `...`, a fleet past the frame ending in one `... <n> more` line.
The board read model's row gains an unserialised last-activity time; the console-frame event gains `shown` and `hidden` counts; the skip-count WARN line is written by the console only when the count changes.
Starts nothing live: no real claude, no compositor dispatch, no window opened; the tiled quarter-screen half is a later live reading at the operator's host.
@task_file n=1 path=src/human/console.rs mark=edits
what: the console's one hand-written frame writer, redrawing the whole frame from a cleared top-left in plain ASCII — edits: the frame it draws (title, captions, rows, `... <n> more`, key line)
@task_file n=2 path=src/console/mod.rs mark=edits
what: the console loop (100 ms size poll, key read, `console::run`) — edits: `console::run`, which takes an injected board source, re-reads the board once a second, redraws on a change and reads the agents off the loop
@task_file n=3 path=src/console/view.rs
what: the new frame model of the console view: the rows, cells, width cut and overflow the frame writer draws
@task_file n=4 path=src/cmd/console.rs mark=edits
what: the `viola console` verb that hands the console loop its terminal — edits: the board source it passes into `console::run`
@task_file n=5 path=crates/viola-state/src/board.rs mark=edits
what: the board read model (`scan` / `read`), sync and Claude-free, behind `viola list` and the MCP list — edits: the board row, which gains the unserialised last-activity time, and the events-log pass of `read` that takes it
@task_file n=6 path=src/human/board/rows.rs
what: the board's row builder: the lineage ordering `forest` and the six board captions in their slot order, reused by the console
@task_file n=7 path=src/human/escape.rs
what: the human-output control escaping, `escape_row`, every board and console cell passes
@task_item n=1 part=needs
Lineage trees: instances drawn as parent/child trees in the order and indentation the board already derives for `viola list` (its lineage forest), never a second tree builder; lineage stays a label only.
@task_item n=2 part=needs
Session state in the board's own words read from the same board read model: LIVE live/stale, STATUS idle/busy/unknown, DIALOG none or `DIALOG <kind>`; «stalled» is the board's `stale` (session liveness), no new vocabulary.
@task_item n=3 part=needs
Wheel holder shown per session in the board's words: driver, human, unknown; the console only shows it and moves no wheel.
@task_item n=4 part=needs
Last activity: the age of the newest events.ndjson line, taken in the pass the board read already makes (no second read of the log), held in the board row but never serialised; the list payload item, `viola list --json` and the MCP list item and `viola list`'s board gain no field.
@task_item n=5 part=needs
The six board captions reused with their words and slot order (the shared fixed captions), `LAST` a seventh caption after them.
@task_item n=6 part=needs
Control escaping: every disk- or upstream-sourced cell passes `escape_row` as the board's cells do; no raw C0, C1 or DEL byte reaches the screen; upstream text stays untrusted content.
@task_item n=7 part=needs
Kept current: the board re-read once a second through the in-process board read (list reads state directly, needs no wrapper or channel), a redraw only when the drawn rows change.
@task_item n=8 part=needs
The claude agents read (`read_here`, called as it is) at most once every 5 s and off the drawing loop; it degrades, never errors, and a failed read leaves status unknown as the board's agents-read-failure rule says.
@task_item n=9 part=needs
Width: the frame stays whole at or above its floor; each row keeps its six cells and `LAST` and is cut at the terminal's width ending `...`, never inside an escape, the cells after CLI cut first and then CLI.
@task_item n=10 part=needs
Overflow: a fleet past the frame's height draws the board's order until one row is left, then one line `... <n> more`.
@task_item n=11 part=needs
The console-frame event of the `console` role file gains `shown` and `hidden` counts, with every list and schema that pins its fields; the role file stays a codes-only process log.
@task_item n=12 part=needs
The lines the board read and the agents read write land in the console's role file: `parse-rejected`, `state-recovered` and the `agents-probe` pair.
@task_item n=13 part=needs
The skip-count WARN line (symlink skips logged as a count) is written by the console only when the count changes: a named console exception to that rule.
@task_item n=14 part=needs
The frame writer stays hand-written and the one human writer: each draw redraws the whole frame from a cleared top-left, plain ASCII with no SGR, no cell grid or diffing; the size polled every 100 ms and re-read on each key, a changed size redrawing whole, no SIGWINCH handler.
@task_item n=15 part=needs
The terminal fleet console is its own surface, the one view, governed by no verb-output rule; its static-output and no-interactive-input exceptions also cover a redraw when the fleet it shows changes.
@task_item n=16 part=needs
Where the view lives: the frame model in `src/console/view.rs`, the board source injected into `console::run` so tests drive the view without a live fleet; the board read model stays sync and Claude-free.
@task_item n=17 part=needs
Tests prove live rows through booted fake-agent wrappers and the size cases through planted instance dirs; every home test-owned; no real claude, no compositor, no read of the operator's real home.
@task_item n=18 part=needs
No key beyond the scaffold's (`q`, Ctrl-C, «unbound»), no action, send or selection is added by the view.
@task_standing n=1
convention: each new test runs in its own test-owned viola home, cleaned by its owner
home: established-decisions · occupied-resources
@task_standing n=2
convention: each new test runs against the fake agent, never a live claude in CI
home: ci-cd-approach
@task_standing n=3
convention: each new log line is emitted through obs_event! with no print macro, a code only, no content
home: build-system · cross-cutting-patterns
@task_standing n=4
convention: each new test wait shares the root tests' one bound, and a killed test leaves its evidence
home: occupied-resources
@task_standing n=5
convention: each new end-to-end run holds zero panic lines and any insta snapshot runs in check mode only
home: project-directory-structure
@task_standing n=6
convention: each new OS-gated test names its reason and no test gate names another OS
home: cross-cutting-patterns
@task_item n=1 part=not_needed
Wrapper channel methods, frames, params and fault codes: the view makes no channel call; it reads the disk through the board.
@task_item n=2 part=not_needed
Send, paste, confirmation and refusal rows: the view sends nothing and types into no session.
@task_item n=3 part=not_needed
Hook tiers, hook-to-kind map, hook exit codes and dialog answering: the view answers no dialog and runs no hook.
@task_item n=4 part=not_needed
Verify, the capability ledger stamps, probes and recorded fixtures: the view verifies no CLI version.
@task_item n=5 part=not_needed
Run, revive, adopt and up start paths, pinned copies, child argv and child environment: the view starts or resumes no session.
@task_item n=6 part=not_needed
Hyprland seam errors, window placement and focus rows: nothing is dispatched to the compositor by this chunk.
@task_item n=7 part=not_needed
Plugin install, the embedded plugin and the marketplace tree: untouched by the view.
@task_item n=8 part=not_needed
Trust writer code words and trust records: the view writes no trust record.
@task_item n=9 part=not_needed
Exit codes and --json documents of other verbs (send, wait, link, revive, verify, open, plugin install): the view adds no verb output.
@task_item n=10 part=not_needed
Fuzz workspace, release tooling, dist and crates.io rows: no release or fuzz target changes.
@task_item n=11 part=not_needed
Wheel take, release and claim mechanics: the view only shows the holder and moves no wheel.
