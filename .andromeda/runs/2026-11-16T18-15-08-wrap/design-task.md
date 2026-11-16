@task_prose part=session
An API-error turn end now names its cause as one of nine closed words: rate-limited, overloaded, auth-failed, billing, invalid-request, model-not-found, server-error, output-limit, api-error; before it was the one word api-error.
The Claude adapter maps the CLI's 13 StopFailure error words onto those nine; an absent word reads api-error, and an error word outside the 13 reads api-error and is marked unknown.
The hook verb, for a marked turn end, writes one codes-only parse-rejected log line with detail unknown-value and still delivers the event; the error text itself still crosses nothing and no error text reaches any output.
The cause word reaches the events log, viola wait --json and the MCP wait result only; no human output changes, and every one of the nine words clears the running turn and reads STATUS idle on the board.
The adapter-internal API-error class list is retired; the classification now crosses as the turn-end cause words.
Not measured: the 13 CLI error words are read from the installed CLI bundles, none seen in a live run.
@task_file n=1 path=crates/viola-core/src/lib.rs
what: the shared core types, holding the closed set of turn-end cause words (now nine)
@task_file n=2 path=crates/viola-agent-claude/src/hook.rs
what: the Claude adapter's hook normaliser, mapping each CLI hook event to an event kind and the StopFailure error word to the turn-end cause word
@task_file n=3 path=src/cmd/hook.rs
what: the hook verb, which reads the hook's stdin, logs a rejected or unknown value by its code only, and delivers the event to the wrapper
@task_file n=4 path=schemas/diag-line.v1.json
what: the diagnostic log line schema, whose parse-rejected detail list gains unknown-value
@task_file n=5 path=src/cmd/run.rs
what: the run verb's wrapper, which re-checks a hook event's turn-end cause against the closed word set before appending it (tests added only)
@task_file n=6 path=src/run/send/slot.rs
what: the send slot that holds a running turn and lets the next send past once a turn ends (tests added only)
@task_file n=7 path=crates/viola-state/tests/state_events.rs
what: the state crate's event-log tests, covering a turn-ended event carrying a cause word
@task_file n=8 path=tests/hook_events.rs
what: the booted hook-event tests, now checking each CLI error word's turn-end cause and that no error text is written
@task_file n=9 path=tests/cli_wheel.rs
what: the booted send-wheel tests, now with a rate-limited turn end letting the next send past
@task_file n=10 path=tests/cli_list.rs
what: the booted viola list board tests, STATUS reading idle after an API-error turn end
@task_file n=11 path=crates/viola-mcp/tests/handlers.rs
what: the MCP handler tests, now checking that MCP wait carries a turn-end cause word unchanged
@task_item n=1 part=touches
The decision that an API-error turn end is shown as any turn end names the closed cause word api-error; the cause word is now one of nine closed words, still never shown in human output.
@task_item n=2 part=touches
The viola wait output event form and the --json wait document: a turn-ended event's cause now carries one of the nine words, not only api-error.
@task_item n=3 part=touches
The LLM-driver CLI callers and the MCP verb's wait result: the turn-end cause word reaches them as one of nine words, unchanged.
@task_item n=4 part=touches
The viola list board STATUS and the flight-progress strip STATUS: every one of the nine cause words clears the running turn and reads idle, the same as any turn end.
@task_item n=5 part=touches
The rule that turn-ended never fills the readback box: it now holds for each of the nine cause words, not one.
@task_item n=6 part=touches
The hook verb's stderr and exit-code contract: the hook verb now writes a parse-rejected unknown-value log line, codes only, for an unknown error word, and still delivers the event and exits as before.
@task_item n=7 part=touches
The decisions that no internals reach errors or hints and that hints never quote text: the CLI's error text still crosses nothing; only the closed cause word does.
@task_item n=8 part=touches
The send-wheel and turn-running refusal (refusal hint not-delivered turn-running): a turn end of any of the nine causes releases the running turn so the next send is not refused as turn-running.
@task_item n=1 part=untouched
Brand identity, the readback box, send readback forms and the handoff display: no readback or send-line wording moved.
@task_item n=2 part=untouched
Colour palette, typography, spacing, depth, border radius and iconography: no colour, glyph, column or width changed.
@task_item n=3 part=untouched
CLI exit codes, exit 21 causes and their --json cause words: no exit code or instance-unreachable cause changed.
@task_item n=4 part=untouched
Start, revive and verify refusals, the revive --json codes, wrapper fault details and peer reply screening: none moved.
@task_item n=5 part=untouched
Command names and the help verb groups: no verb was added, renamed or removed.
@task_item n=6 part=untouched
The viola list board's columns, NAME truncation, row escaping and skip counts: unchanged; only STATUS after a turn end was re-checked.
@task_item n=7 part=untouched
The anti-patterns and self-validation protocol: no visual or output-mode rule moved.
