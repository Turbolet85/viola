@task_prose part=session
An API-error turn end now names its cause from a closed list of nine kebab-case words (rate-limited, overloaded, auth-failed, billing, invalid-request, model-not-found, server-error, output-limit, api-error) instead of the one word api-error.
The adapter maps the 13 CLI StopFailure error words to those nine; an absent word reads api-error, any other string reads api-error and is marked; the CLI's error text is still dropped at the parse and crosses nothing.
A marked StopFailure makes viola hook write one codes-only parse-rejected WARN log line (parser hook-stdin, detail unknown-value, count 1) to its role log, then deliver the event as before; the diag-line schema's parse-rejected detail enum grows from 8 to 9 values.
The cause word reaches wait --json and MCP wait only; no human-output line, caption, refusal, colour or terminal byte changed; every cause word clears the running turn and the board reads STATUS idle.
Retired the adapter-internal ApiErrorClass classifier; added no IPC method, event kind, crate, env var or dependency.
The 13 CLI error words are read from the installed bundles' source, none measured in a live run.
@task_file n=1 path=crates/viola-core/src/lib.rs
what: the core shared types, holding the closed turn-end cause word set the wrapper re-checks events against
@task_file n=2 path=crates/viola-agent-claude/src/hook.rs
what: the claude adapter's hook-to-event map, which reads vendor hook payloads, drops vendor error text and emits closed code words
@task_file n=3 path=src/cmd/hook.rs
what: the viola hook command, the hook event call into the wrapper and its run-diagnostics log lines kept off the terminal
@task_file n=4 path=schemas/diag-line.v1.json
what: the diagnostic log-line schema, holding the parse-rejected detail word list
@task_file n=5 path=src/cmd/run.rs
what: the viola run wrapper, which revalidates hook events to closed code words before appending them
@task_file n=6 path=src/run/send/slot.rs
what: the wrapper's send slot, the turn-running rung a turn end clears for the next confirmed send
@task_file n=7 path=crates/viola-state/tests/state_events.rs
what: the state crate's event-line tests, an existing suite
@task_file n=8 path=tests/hook_events.rs
what: the booted hook-event suite, checking turn-end events and the hook's role log lines against the schema
@task_file n=9 path=tests/cli_wheel.rs
what: the CLI wheel and send suite, the confirmed-send critical path
@task_file n=10 path=tests/cli_list.rs
what: the viola list board suite, the STATUS word a row prints
@task_file n=11 path=crates/viola-mcp/tests/handlers.rs
what: the MCP server tool handler tests, the MCP wait tool's structured result
@task_item n=1 part=touches
The turn-ended cause, a closed code word under wait --json and the MCP wait tool, is now one of nine kebab-case words; any row stating the code-word format or that unlisted code words stay visible must still hold for all nine.
@task_item n=2 part=touches
The single vendor-content boundary and no-bare-error-text rules: the CLI's StopFailure error text is still dropped in the adapter and never crosses; only a closed word does, plus a codes-only log line for an outside word.
@task_item n=3 part=touches
Peer replies screened to code words: the wrapper's hook-event check now admits nine cause words, still refusing raw CLI words and text.
@task_item n=4 part=touches
Run diagnostics never reach the terminal: the hook now writes a new parse-rejected unknown-value WARN line, to its role log only, carrying code and count and no value.
@task_item n=5 part=touches
State carried by its printed word: every one of the nine cause words clears the running turn and the board's STATUS reads idle; no human line names the cause.
@task_item n=6 part=touches
Critical paths for wait/last output and for confirmed send and readback: a turn end with any cause word ends the wait and lets the next send past the turn-running rung, with no pause or release.
@task_item n=7 part=touches
Fixed CLI strings pinned by literal and fixed copy from design: unchanged human output; the cause word appears only in machine output, so no literal pin or caption moves.
@task_item n=8 part=touches
A11y assertions inside existing suites: the new cases (cause map, booted StopFailure word walk, MCP wait cause, list STATUS idle) were added to the existing nextest suites, no new suite or driver.
@task_item n=1 part=untouched
Colour and SGR rows (amber DIALOG, bold NAME, dim stale rows, no SGR off a terminal, word beside every SGR span): no colour or escape sequence changed.
@task_item n=2 part=untouched
Refusal layout rows (unable/hint pair, one hint per code, reason codes, exit codes 2, 10, 11, 13, 14, 20, 21, --json refusal and fault documents): no refusal, exit code or failure document changed; a turn end is not a refusal.
@task_item n=3 part=untouched
Revive, link, unlink, plugin install and verify output contracts: none of their lines, words or exit codes moved.
@task_item n=4 part=untouched
TUI passthrough rows (vendor content bytes, focus/mouse/resize sequences, vt100 feed panic, own terminal bytes): no terminal byte path changed.
@task_item n=5 part=untouched
Screen-reader targets and setup, manual keyboard and Orca passes, and the viola --help rows: nothing they cover changed.
@task_item n=6 part=untouched
CI integration, gate and SLO rows (fail-fast gate, zero flakiness, no skips, coverage and doctest): no gate, threshold or workflow changed.
@task_item n=7 part=untouched
Absences (internationalization, touch surface, graphical frontend, rendered page, user time limit): none is reached.
