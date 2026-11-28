@task_prose part=chunk
Builds the view `viola console` draws past the scaffold's one fixed frame: the fleet's lineage trees in the board's order, each session's state and wheel holder in the board's words, and a seventh LAST cell for last activity.
The view stays current: the board is re-read once a second, a redraw happens only when the drawn rows change, and the claude agents read runs at most once every 5 s off the drawing loop.
Every disk- or upstream-sourced cell is control-escaped; each row is cut at the terminal width ending `...`, and a fleet past the frame ends in one `... <n> more` line.
The console-frame event gains shown and hidden counts, and the console writes the skip-count WARN line only when the count changes.
The compositor half (tiled, quarter-screen on the founder's host) is left to a later live reading; this chunk opens no window and runs nothing live.
@task_file n=1 path=src/human/console.rs mark=edits
what: the console's one frame writer, which redraws the whole frame from a cleared top-left in plain ASCII — edits: the frame it draws (captions, rows, LAST cell, the width cut, the overflow line)
@task_file n=2 path=src/console/mod.rs mark=edits
what: the console loop: the 100 ms size poll, the key read, console::run — edits: console::run (takes an injected board source), the once-a-second board re-read, redraw-on-change, the off-loop agents read
@task_file n=3 path=src/console/view.rs
what: the console's new frame model: the fleet's rows, their cells and the shown/hidden split that the frame writer draws
@task_file n=4 path=src/cmd/console.rs
what: the viola console verb, which hands the console loop its terminal and its board source
@task_file n=5 path=crates/viola-state/src/board.rs mark=edits
what: the board read model (scan / read), called in-process — edits: the board row, which gains an unserialised last-activity time taken in the pass the read already makes over each events log
@task_file n=6 path=src/human/board/rows.rs
what: the board row builder: the lineage ordering (forest) and the six board captions with their words and slot order
@task_item n=1 part=needs
The console role file and its console-frame and console-key events: console-frame gains two additive fields, shown and hidden (rows drawn, rows left out), and every list and schema that pins console-frame's fields must take them.
@task_item n=2 part=needs
The skipped-state-entries count line (a parse-rejected count at WARN, from the board read) goes into the console role file only when the count changes, not on every once-a-second re-read. This is a named console exception to the skip-count rule.
@task_item n=3 part=needs
The lines the board read and the agents read already write go into the console's role file: parse-rejected, state-recovered and the agents-probe spawn/exit pair. A once-a-second board re-read and a 5 s agents read must stay inside the rules for how often those lines may be written, with no info-level line per loop tick.
@task_item n=4 part=needs
The claude agents --json read is called as it is, at most once every 5 s and off the drawing loop. Its output is untrusted, and a parse failure reads as unknown, so a session's STATUS shows unknown, never a guess.
@task_item n=5 part=needs
Session state comes only in the board's own words, from the same board read model: LIVE live or stale (stale by the heartbeat staleness threshold), STATUS idle, busy or unknown, DIALOG none or `DIALOG <kind>`, and WHEEL driver, human or unknown. "Stalled" means the board's stale.
@task_item n=6 part=needs
Last activity is the age of the newest events.ndjson line, read in the pass the board read already makes. It sits in the board row but is never serialised: it stays out of list --json, the MCP list item and viola list's board, and only the console shows it.
@task_item n=7 part=needs
Every cell the view draws from disk or from upstream (instance names, CLI, dialog kind, agents output) passes escape_row, so no raw C0, C1 or DEL byte reaches the terminal. The width cut never splits an escape.
@task_item n=8 part=needs
The frame layout: the six board captions keep their words and slot order with LAST after them, and the lineage trees keep the board's order and indentation. A row is cut at the terminal width ending `...`, with the cells after CLI cut first and then CLI. A fleet larger than the frame shows the board's order down to the last free row, then one `... <n> more` line, and the frame stays whole at or above the size floor.
@task_item n=9 part=needs
The console's stdout carries only the drawn frame, through the one writer. No diagnostics line, telemetry or ANSI/SGR sequence goes to stdout or into the frame.
@task_item n=10 part=needs
Liveness changes the console sees on each re-read (live to stale, busy to idle) are drawn, not logged. Redraws and heartbeat ticks add no log line beyond the console-frame event.
@task_item n=11 part=needs
New console lines carry no instance or lineage names, paths, agents rows or screen content, only closed codes and counts: shown and hidden are counts, and the role file is a home-level file.
@task_item n=12 part=needs
The board source is injected into console::run so tests can drive the view without a live fleet. Live rows are proved with booted fake-agent wrappers, size cases with planted instance directories, and every home is owned by its test: no real claude, and no read of the operator's real ~/.claude* or ~/.viola.
@task_item n=13 part=needs
The new view code runs inside the console role, whose panic outcome stays as the panic-by-role rule sets it: a panic in the frame model or the re-read path is caught and logged as the console's one-line panic record.
@task_standing n=1
convention: each new or extended diagnostics event field is added only, never renamed or removed, is emitted through the one obs_event macro, and joins the diagnostic line schema and its field allow-list
home: 6-log-coverage · log-coverage
@task_standing n=2
convention: each new integration or E2E test's emitted lines are schema-checked, its home lives under the one scanned test-home root and is kept until the panic and schema gates run
home: 9-ci-integration · ci-integration
@task_standing n=3
convention: each new test that needs a live session runs against the fake agent only, never a real claude CLI
home: 4-span-trace-coverage
@task_standing n=4
convention: new product code uses no raw tracing or print macros outside the obs module, no bare instrument attribute, and no info logging in hot loops
home: 11-obs-anti-patterns-never-do-these
@task_item n=1 part=not_needed
MCP server rows (the stdio framing, MAX_FRAME, frame-tool refusals, isError results, the tools' log lines): the chunk changes no MCP code beyond keeping LAST out of the list item.
@task_item n=2 part=not_needed
Hook rows (fail-open, hook stdout and stderr, hook latency and perf budgets, hook-decision detail): the console reaches no hook.
@task_item n=3 part=not_needed
Send, answer, dialog, wait, wheel-transition and link path log events, their span chains and refusal details: the chunk adds no action, key, send or selection, since Console actions owns those.
@task_item n=4 part=not_needed
viola run, revive, adopt and verify rows (start spans, exit-1 and exit-21 detail codes, the confirm window, the version gate, the pinned copy): the chunk starts, revives or verifies nothing.
@task_item n=5 part=not_needed
viola-channel JSON-RPC rows (params, the response line, -32603, conn and srv_conn, peer credentials): the console opens no channel connection.
@task_item n=6 part=not_needed
PTY seam and vt100 rows (spawn, paste, kill-fallback, feed panics): the console drives no PTY and reads no screen.
@task_item n=7 part=not_needed
Future error reporter, Sentry, OTel SDK or exporter, OTLP receiver and network egress rows: the chunk adds no reporter, exporter or network path.
@task_item n=8 part=not_needed
Secret-scan hit report, artifact upload, retention and SHA-pinned action rows: the chunk adds no CI upload or workflow job.
