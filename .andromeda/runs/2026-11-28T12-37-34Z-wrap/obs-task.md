@task_prose part=session
The session built the fleet console's view: viola console now draws the board's lineage rows in the board's order and state words, with a seventh caption LAST (the age of an instance's newest log line), every cell control-escaped, a wide line cut ending in dots, a tall fleet ending in a more line.
The console re-reads the board once a second, replays no unchanged log, and redraws and writes a console-frame line only when the view or the size changes; console-frame gains the required fields shown and hidden (3 fields to 5).
The console runs the claude agents probe on its own thread at most every 5 s; a failed read hands an empty list, so STATUS reads unknown.
The console now writes into its own role file the agents-probe pair, state-recovered per replay, parse-rejected for state entries on a change of the scan's skip counts, and one process-exit exit 1 strict-modes-failed when the home is widened mid-run; an unlistable instances directory mid-run ends as an internal error at the cli catch site.
The board item gains a last-activity time that is never serialised: viola list, list --json and the MCP list item are unchanged. No event, subject, detail word, config key, endpoint or env var was added; nothing was retired; the live tiled reading stays gated.
@task_file n=1 path=crates/viola-core/src/obs.rs
what: the obs module — the obs_event macro and the closed log field names, gaining the console-frame fields shown and hidden
@task_file n=2 path=crates/viola-state/src/board.rs
what: the state crate's board read model — the instance scan behind the strict-modes checks, the snapshot read through replay, the skipped-entry counts and the board item
@task_file n=3 path=schemas/diag-line.v1.json
what: the diag-line v1 schema, its console-frame arm
@task_file n=4 path=src/cmd/console.rs
what: the viola console verb's dispatch — its sources, the claude agents read it hands the console, its exit on a mid-run refusal
@task_file n=5 path=src/cmd/list.rs
what: the viola list verb — the board read and the join of claude agents observations behind list, list --json and the MCP list tool
@task_file n=6 path=src/console/fleet.rs
what: the console's board source and agents thread — re-reads the fleet behind the strict-modes scan, logs the skip and replay lines, runs the agents probe off the loop
@task_file n=7 path=src/console/frame.rs
what: the console frame's geometry — the body rows between the title and the key line
@task_file n=8 path=src/console/mod.rs
what: the console loop — draw, redraw on a change, the console-frame line, the board re-read interval, the end on a refused or failed read
@task_file n=9 path=src/console/view.rs
what: the console's view — the drawn lines cut to the terminal, with the shown and hidden counts
@task_file n=10 path=src/human.rs
what: the human terminal rendering module and the console frame writer it exports
@task_file n=11 path=src/human/board.rs
what: the board's human rendering for viola list — its rows, captions and empty line
@task_file n=12 path=src/human/board/cells.rs
what: the board's cells — the state words and the control-escaped cell text
@task_file n=13 path=src/human/board/rows.rs
what: the board's rows — the lineage forest order and indent, the six captions
@task_file n=14 path=src/human/board/test_support.rs
what: the board rendering's test fixtures
@task_file n=15 path=src/human/console.rs
what: the console frame writer — the title, the body lines and the key line
@task_file n=16 path=src/human/fleet.rs
what: the fleet's lines — the caption row with LAST, the age cell, the line cut that never splits an escape
@task_file n=17 path=tests/tui_console.rs
what: the console's end-to-end tests under a pty and a test home, each console line schema-checked
@task_item n=1 part=touches
contract/diag-line-fields/console-frame: its fields move from 3 to 5 (cols, rows, fits, then shown and hidden, both required), and its cadence moves from once per draw to the first draw, then only when one of its five fields changes.
@task_item n=2 part=touches
decision/symlink-skip-logged-as-count gains the console exception: the console writes the parse-rejected state-entry skip line only on a change of the scan's counts, while viola list still writes it per scan.
@task_item n=3 part=touches
contract/diag-subject/agents-probe now names the console: viola console runs the agents probe on its own thread at most every 5 s and writes its pair into its role file.
@task_item n=4 part=touches
contract/diagnostics-files/console: the console's role file now also holds the agents-probe pair, state-recovered per replay, parse-rejected for state entries and a mid-run process-exit, beside console-frame and console-key.
@task_item n=5 part=touches
decision/strict-modes-failure-lines, contract/process-exit-detail/strict-modes-failed and contract/diag-detail/strict-modes-failed: a home widened while the console runs ends it with exit 1, the home pair on stderr and one process-exit strict-modes-failed line.
@task_item n=6 part=touches
The state-recovered rows (contract/diag-line-fields/state-recovered, contract/state-recovered/*, contract/corr-by-event/state-recovered): the console now writes state-recovered for each replay it runs, and none for a log left unchanged.
@task_item n=7 part=touches
The parse-rejected rows for skipped state entries (contract/diag-line-fields/parse-rejected, contract/parse-rejected-detail/symlink-ignored and malformed, decision/skipped-symlink-never-logged, decision/skipped-counters, decision/per-parser-parse-failure-counts): the console now emits the count line, on a change only, naming no entry.
@task_item n=8 part=touches
The agents probe parse-failure rows (decision/agents-probe-parse-failure-unknown, decision/agents-cli-parse-failure-unknown, decision/agents-cli-instrumented-at-boundary, decision/agents-probe-output-untrusted, decision/agents-cli-output-untrusted, contract/parse-rejected/claude-agents-json): the console's failed read hands an empty list, so STATUS reads unknown.
@task_item n=9 part=touches
decision/claude-agents-rows-never-logged and decision/agent-rows-never-verbatim: the console's agents read logs only the probe pair; session ids and agent rows reach no line.
@task_item n=10 part=touches
The terminal control-escape rows (decision/escape-controls-in-terminal, decision/controls-escaped-in-terminal-output): every fleet cell the console draws is row-escaped, and the line cut never splits an escape.
@task_item n=11 part=touches
decision/no-interactive-terminal-interface: the console now redraws when the fleet it shows changes, not only on a key or a resize.
@task_item n=12 part=touches
The log-volume rows (decision/no-info-in-hot-loops, decision/no-hot-path-instrumentation, decision/no-per-heartbeat-logging, decision/liveness-transitions-not-logged): the console's once-a-second re-read writes no line of its own; only a change writes console-frame, and the agents-probe pair runs up to 720 an hour.
@task_item n=13 part=touches
The additive-schema rows (decision/diag-schema-additive-only, decision/log-schema-additive-only, decision/log-fields-additive-only, decision/additive-field-naming, decision/log-field-names-frozen): the console-frame arm gains two required fields; no event and no field name is removed.
@task_item n=14 part=touches
decision/diag-schema-matches-event-enum, decision/event-values-closed and the diag-subject enum rows: the event enums and the subject list are unchanged by the new console lines.
@task_item n=15 part=touches
The replay-read rows (decision/no-raw-snapshot-reads, decision/no-endpoint-over-replayed-snapshot, decision/replayed-snapshot-unreachable): the console's board source reads the snapshot through the board read and its replay, never raw.
@task_item n=16 part=touches
decision/state-sync-instrumentation and decision/state-crate-sync-instrumentation: the state crate's board read is rebuilt on a per-instance read and a relive that opens no log, keeping its line order; its skip lines are now emitted through separate skip-count helpers.
@task_item n=17 part=touches
contract/viola-mcp-tools/list, contract/list-json/liveness and decision/endpoint-and-pinned-bin-never-logged: the board item gains a never-serialised last-activity time; list --json and the MCP list item carry no new key.
@task_item n=18 part=touches
contract/cli-internal-error/human-line and decision/boundary-errors-end-in-logged-line: an unlistable instances directory mid-run in the console returns an error after the terminal restore and reaches the cli catch site as error: internal error.
@task_item n=19 part=touches
contract/panic-outcome-by-role/console and decision/panic-caught-at-every-role: the console role now runs a second thread, the agents reader, beside its loop.
@task_item n=20 part=touches
decision/diag-lines-schema-checked-in-unit-tests and decision/schema-conformance-gate: every console line in the end-to-end cases is schema-checked with shown and hidden, and every console test start runs with an empty test-scoped PATH so no host claude is read.
@task_fact n=1 slug=log-coverage
fact: The console's agents probe runs on its own thread at most every 5 s and writes its agents-probe pair each read, up to 720 pairs an hour into the console role file; this volume is accepted as built.
@task_fact n=2 slug=obs-scope-summary
fact: The console's agents read fails open: a failed or unparsable read hands an empty list, so STATUS reads unknown and the unwrapped rows drop, never the last good list.
@task_fact n=3 slug=error-capture-reporting
fact: A board read that errs in the console mid-run (an unlistable instances directory, not a strict-modes refusal) restores the terminal and ends as error: internal error at the cli catch site.
@task_item n=1 part=untouched
The hook rows: its stdin, stdout, stderr, fail-open, budgets and decision lines; no hook path changed.
@task_item n=2 part=untouched
The channel, send, dialog, wait, wheel and link path rows: their events, corr, spans and refusal details are unchanged.
@task_item n=3 part=untouched
viola run, its start spans, pty seam, claude-child process lines, env strip and confirm window; viola verify and plugin install.
@task_item n=4 part=untouched
The PII upload and secret-scan rows, CI artifact retention, pinned actions and the fuzz and mutation upload rules.
@task_item n=5 part=untouched
The perf, SLO and budget rows (hook budgets, hyperfine, error budget, panic count gates) and the metric coverage rows.
@task_item n=6 part=untouched
The absences (OTel, Sentry, error reporter, network egress, exporters, database, gRPC, deployment) and the logging-stack choice rows (tracing, subscriber features, no rotation, file sinks, home resolution, service identity, config.json keys).
