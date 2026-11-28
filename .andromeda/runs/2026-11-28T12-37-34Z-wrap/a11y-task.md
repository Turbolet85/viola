@task_prose part=session
The session built the fleet console's view: viola console now draws the board's lineage rows in the board's order, its state words, and a seventh LAST cell holding the last activity in one unit (12s, 4m, 2h, 3d), unknown where no log line exists, n/a on an unwrapped row.
Every fleet cell is control-escaped; a line wider than the terminal is cut ending ... and never inside an escape; a fleet taller than the terminal ends in a ... <n> more line.
The console re-reads the board once a second, replays no unchanged log, and redraws and logs console-frame only when the view or the size changes; console-frame gains shown and hidden (3 to 5 fields).
The claude agents read runs on its own thread at most every 5 s, off the loop, so keys still quit; a failed read hands an empty list and STATUS reads unknown.
A home widened mid-run ends the console with exit 1, the home pair and one process-exit strict-modes-failed; an unlistable board directory ends in error: internal error.
viola list, list --json and the MCP list item are unchanged; nothing retired; matrix v2-14 stays GATED on the operator's live tiled quarter-screen reading, which was not fired.
@task_file n=1 path=crates/viola-core/src/obs.rs
what: edited — the shared field names of the diagnostic log lines (console-frame's shown and hidden)
@task_file n=2 path=crates/viola-state/src/board.rs
what: edited — the board read model: the scan of instances, each instance's snapshot-or-replay read, the board's items, now with each item's last activity (never serialised)
@task_file n=3 path=schemas/diag-line.v1.json
what: edited — the diagnostic line schema; its console-frame arm gains shown and hidden
@task_file n=4 path=src/cmd/console.rs
what: edited — the viola console verb: its board, clock and agents sources, and its exit on a refused scan
@task_file n=5 path=src/cmd/list.rs
what: edited — the viola list verb: the board read and its join of the claude agents observations
@task_file n=6 path=src/console/frame.rs
what: edited — the console frame's geometry: the body rows between the title line and the key line
@task_file n=7 path=src/console/mod.rs
what: edited — the fleet console's loop: the once-a-second board re-read, the redraw only on a change, the terminal restore on a refusal, the console-frame line
@task_file n=8 path=src/human.rs
what: edited — the human output module and what it exports
@task_file n=9 path=src/human/board.rs
what: edited — the viola list board renderer and its shared captions, separator and cut for the console
@task_file n=10 path=src/human/board/cells.rs
what: edited — the board's cells: state words, NAME truncation, control-character escaping, n/a
@task_file n=11 path=src/human/board/rows.rs
what: edited — the board's rows: the lineage forest order and indent, the caption row, the unwrapped separator, the empty-board line
@task_file n=12 path=src/human/board/test_support.rs
what: edited — the board tests' fixture item
@task_file n=13 path=src/human/console.rs
what: edited — the console frame writer: the title line, the body lines from row two, the key line
@task_file n=14 path=tests/tui_console.rs
what: edited — the console's end-to-end pty tests, now each started with an empty test-scoped PATH
@task_file n=15 path=src/console/fleet.rs
what: added (new) — the console's board source behind the strict-modes scan, and its agents thread
@task_file n=16 path=src/console/view.rs
what: added (new) — the console's view: the lines cut to the columns, the ... <n> more line, the shown and hidden counts
@task_file n=17 path=src/human/fleet.rs
what: added (new) — the fleet text: the board's rows with the LAST cell, the age form, the line cut ending ...
@task_item n=1 part=touches
decision/static-linear-output gains its exception: the console redraws when the fleet it shows changes (carried amendment, «...or the fleet it shows changes»).
@task_item n=2 part=touches
decision/cli-no-interactive-input gains «...or a change in the fleet it shows»: the console's frame now changes without a keystroke.
@task_item n=3 part=touches
decision/linear-static-output and decision/linear-static-cli-output (the twins of static linear output): the console's redraw on a fleet change reaches them the same way.
@task_item n=4 part=touches
decision/claude-agents-read-fails-open now names the console: viola console runs the claude agents read on its own thread every 5 s; a failed read hands an empty list, STATUS reads unknown and the unwrapped rows go.
@task_item n=5 part=touches
decision/list-captions-fixed, contract/viola-list-board/caption and contract/viola-list-caption/order: the console draws the board's six captions then a seventh, LAST; the list board's own six are unchanged.
@task_item n=6 part=touches
decision/control-character-escaping, decision/c0-c1-escaped, decision/control-bytes-escaped and contract/cli-output/control-characters: every fleet cell is row-escaped, and the line cut never splits an escape.
@task_item n=7 part=touches
decision/unwrapped-name-truncation and decision/name-truncation: in the console an unwrapped NAME is not width-capped; the whole line is cut at the terminal's columns instead, ending ...
@task_item n=8 part=touches
decision/state-carried-by-printed-word: the console shows the board's own state words, and LAST in one unit, unknown or n/a.
@task_item n=9 part=touches
contract/viola-list-board/order, contract/viola-list-board/empty-board and contract/viola-list-board/unwrapped-separator (and contract/viola-list/unwrapped-separator): the console now draws through the same forest order, empty-board line and separator; the list board's lines are unchanged.
@task_item n=10 part=touches
decision/critical-path-cli-board-output: the board's rows are now built on a shared forest-rows helper; viola list's output stays the same literal lines.
@task_item n=11 part=touches
decision/no-sgr-in-plain-modes, decision/colour-second-cue-only and decision/word-beside-every-sgr: the fleet frame is plain ASCII with no colour, so the console's view carries no SGR.
@task_item n=12 part=touches
decision/human-keystrokes-never-blocked: the agents read runs off the console's loop, so a key still quits while it runs.
@task_item n=13 part=touches
decision/time-bounds-fall-on-the-driver, decision/time-bounds-never-on-human and decision/hyprctl-read-deadline (5 s bound): the console's agents read is bounded by the existing 5 s deadline, and an agents child may outlive a quit by up to that bound (accepted as built).
@task_item n=14 part=touches
decision/strict-modes-refusal-before-any-write, decision/unstattable-home-is-strict-modes-refusal and contract/frame-verb-refusal/strict-modes-failed: a home widened while the console runs is refused after frames were drawn, the terminal restored, then exit 1 with the home pair and one process-exit strict-modes-failed.
@task_item n=15 part=touches
decision/no-bare-error-text and contract/json-fault/internal-error: an unlistable board directory mid-run reaches the cli catch site as error: internal error after the restore.
@task_item n=16 part=touches
decision/run-diagnostics-off-terminal and contract/log-identity/process: the console's own log gains the agents-probe pair, state-recovered, parse-rejected state-entry on a change of the scan's counts, and process-exit; none reach the terminal.
@task_item n=17 part=touches
decision/fixed-cli-strings-pinned-by-literal, decision/fixed-cli-strings-pinned and decision/fixed-copy-from-design: new fixed console strings LAST, ... <n> more, unknown, n/a and the age form, pinned by literal cases.
@task_item n=18 part=touches
Section 5's overview (the fleet console's keyboard reach is the route entry Fleet a11y verification's): the console's view still claims no a11y verification; it stays owed by that entry.
@task_item n=19 part=touches
decision/live-readings-never-gate and decision/screen-reader-target-tui: matrix v2-14 stays GATED on the operator's live tiled quarter-screen reading, not fired; the code half is proven over fakes only.
@task_item n=20 part=touches
decision/a11y-inside-existing-suites and decision/a11y-runs-in-existing-test-suites: the escape, cut and redraw cases run inside the existing unit and console e2e suites, the console e2e starts now with an empty test-scoped PATH.
@task_fact n=1 slug=visual-design-verification
fact: In the fleet console an unwrapped NAME is not width-capped as on the board; the whole line is cut at the terminal's columns, ending ..., never inside an escape.
@task_fact n=2 slug=6-visual-design-verification
fact: A fleet taller than the console's body rows ends in a ... <n> more line, so no row is hidden without a word saying so.
@task_fact n=3 slug=visual-design-verification
fact: The console's LAST cell carries the last activity as a word in one unit (12s, 4m, 2h, 3d), unknown where no log line exists, n/a on an unwrapped row.
@task_fact n=4 slug=cognitive-accessibility
fact: The console's claude agents read runs off its loop at most every 5 s; a key never waits on it, and an agents child may outlive a quit by up to its 5 s bound.
@task_fact n=5 slug=cognitive-accessibility
fact: A strict-modes refusal that comes mid-run in the console is the one after frames were drawn: the terminal is restored first, then exit 1 with the home pair.
@task_item n=1 part=untouched
The viola run TUI passthrough contracts (entry, keystrokes, focus/mouse/resize, vendor content, feed panic, own bytes): the console is not the passthrough.
@task_item n=2 part=untouched
The revive, link, unlink, switch, up, verify, wait and plugin install output and refusal contracts, and the CLI exit codes 10 to 21.
@task_item n=3 part=untouched
The MCP frame tools, the hook calls and the instance-unreachable details: no IPC method, endpoint, socket or env var was added.
@task_item n=4 part=untouched
The CLI --json failure documents and refusal layout (unable, hint, code words): no new refusal code or hint.
@task_item n=5 part=untouched
CI integration, the a11y gate, SLO and budgets, and the structured violation JSON: the harness, its status shape and the log format are unchanged.
@task_item n=6 part=untouched
The viola --help rows, the screen-reader test setups and the manual Orca and keyboard passes.
@task_item n=7 part=untouched
The absences (internationalization, touch, web or graphical frontend, rendered page, user time limit): none gains or loses a capability.
