@task_prose part=session
An API-error turn end now names its cause from a closed list of nine words: rate-limited, overloaded, auth-failed, billing, invalid-request, model-not-found, server-error, output-limit, api-error; before, api-error was the only word.
The Claude adapter maps the CLI's 13 StopFailure error words onto the nine. An absent word reads api-error. Any other string reads api-error and is marked unknown. The error text is read only to classify and is never copied, logged or put in an event.
A StopFailure whose error word is outside the 13 now makes the hook write one WARN parse-rejected line, parser hook-stdin, detail unknown-value, count 1. The line carries the code and the count only, and the event is still delivered.
The diag-line v1 schema's parse-rejected detail enum gains unknown-value, so it now holds nine values. The turn-ended cause value set grows additively with no v bump. The adapter's internal ApiErrorClass and api_error_class are retired.
The cause word reaches events.ndjson, wait --json and MCP wait unchanged. It makes no change to human output. The 13 CLI words are read from the installed CLI bundles and were never measured in a live run.
The unknown-value notice rests on the overseer's technical answer, not the founder's word. If the reconcile reads it as a widening, it lands as PROVISIONAL, delegate overseer. The nine words are the founder's ruling and stand.
@task_file n=1 path=crates/viola-core/src/lib.rs
what: viola-core, the shared core types crate that emits no telemetry; holds the closed turn-end cause set TurnEndCause, now nine words.
@task_file n=2 path=crates/viola-agent-claude/src/hook.rs
what: the Claude adapter's hook stdin parser and hook-to-kind map; it normalises the StopFailure payload, classifies its error word into a cause word, marks an unknown value and drops the error text.
@task_file n=3 path=src/cmd/hook.rs
what: the viola hook role's handler; it writes the hook's diagnostic lines, including parse-rejected through its stdin rejection path, and delivers the hook event to the wrapper.
@task_file n=4 path=schemas/diag-line.v1.json
what: the diag-line v1 schema for every process log line; the parse-rejected detail enum is the list of closed detail words.
@task_file n=5 path=src/cmd/run.rs
what: the viola run wrapper; it re-validates a hook event before appending it to events.ndjson. Only its tests changed, which cover the cause check against the closed set.
@task_file n=6 path=src/run/send/slot.rs
what: the wrapper's send slot and turn-running rung; a turn end clears the running turn. Only its tests changed, adding a rate-limited turn-end case.
@task_file n=7 path=crates/viola-state/tests/state_events.rs
what: the viola-state events-log tests; the turn-ended event cases.
@task_file n=8 path=tests/hook_events.rs
what: the booted end-to-end hook event tests; the StopFailure case for every CLI error word, which validates the hook role file against the diag-line schema.
@task_file n=9 path=tests/cli_wheel.rs
what: the CLI wheel and send integration tests; an API-error turn end clears the turn with no pause or release.
@task_file n=10 path=tests/cli_list.rs
what: the viola list integration tests; STATUS reads idle after an API-error turn end.
@task_file n=11 path=crates/viola-mcp/tests/handlers.rs
what: the viola-mcp stdio server tool handler tests; the MCP wait tool carries a turn-end cause word unchanged.
@task_item n=1 part=touches
The contract for the parse-rejected diagnostic line's additive fields: detail gains unknown-value, which the hook emits under parser hook-stdin.
@task_item n=2 part=touches
The boundary list of parse-rejected detail words held eight words (control-character, malformed, oversize, panicked, strict-modes-failed, symlink-ignored, unknown-keys, unreadable). It now has a ninth, unknown-value, so the enum count moves from 8 to 9.
@task_item n=3 part=touches
The decision that parse failures surface as unknown plus a counter now also covers the StopFailure error word. An outside word reads the api-error cause and adds one parse-rejected count line.
@task_item n=4 part=touches
The decision that parse failures surface as unknown gains a new instance: an unknown StopFailure error value surfaces as the api-error cause word.
@task_item n=5 part=touches
The decision on per-parser parse-failure counts gains a new parser and detail pair, hook-stdin with unknown-value.
@task_item n=6 part=touches
The decision that the diagnostic log schema is additive only, together with its twin on the log schema: the detail enum gains a member additively and no field moves.
@task_item n=7 part=touches
The decision that schema words no build can emit are removed: the new unknown-value word is emitted by the hook, so the decision holds and must not drop it.
@task_item n=8 part=touches
The corr-by-event contract for parse-rejected applies to a new parse-rejected line, written by the hook role.
@task_item n=9 part=touches
The hook's lines, meaning the hook invocation and decision lines and the hook role's diagnostics file: the hook now writes an extra WARN line on an unknown error value. It still uses one write per line, still writes no stderr, and still delivers the event.
@task_item n=10 part=touches
The hook latency budget and the spine hook budget are indirect: one extra log write happens only on an outside error word. The budgets' thresholds do not move.
@task_item n=11 part=touches
The rules that event payloads stay out of process logs and that content stays out of home-level lines: the new line holds only the code and the count, never the planted error text.
@task_item n=12 part=touches
The default-deny log fields and source allow-list decisions: the new line adds no field. It uses the existing parser, detail and count fields only.
@task_item n=13 part=touches
The decisions that last_assistant_message and the assistant message are never logged: the StopFailure arm still writes last_assistant_message and background_tasks as null.
@task_item n=14 part=touches
The decisions on closed protocol codes and on metric grouping by closed enums: the turn-end cause is a closed enum of nine words, so derived counts can group by it.
@task_item n=15 part=touches
The event stream line contract and the events.ndjson line-shape contract: the turn-ended cause value set grows additively with no v bump.
@task_item n=16 part=touches
The viola-mcp stdio server wait tool contract: wait now carries a cause word from the nine in structuredContent, with isError false.
@task_item n=17 part=touches
The decision that wrapped claude internals are not a contract: the 13 StopFailure error words are read from the CLI bundles, are not measured live, and fall back to api-error when unknown.
@task_item n=18 part=touches
The decision that viola-core emits no telemetry is indirect: the core cause set grew, and it still emits nothing.
@task_fact n=1 slug=log-coverage
fact: The parse-rejected detail word unknown-value means a hook stdin field held a value outside its closed set. Its first use is the StopFailure error word.
@task_fact n=2 slug=log-coverage
fact: For a StopFailure error word outside the CLI's 13, the hook writes one WARN parse-rejected line with parser hook-stdin, detail unknown-value and count 1, carrying the code and count only, and still delivers the event as an api-error turn end; a known word writes no line. It rests on the overseer's technical answer, not the founder's word.
@task_fact n=3 slug=pii-scrubbing-compliance
fact: The CLI's StopFailure error text and error_details are read only to classify the cause. Neither reaches any log line, detail line or event, and only the closed cause word crosses out of the adapter.
@task_item n=1 part=untouched
The instance-unreachable and exit-21 cause words (endpoint-lost, instance-dead and the others) and the MCP unreachable result's cause word. They are a separate closed set from the turn-end cause, and they did not move.
@task_item n=2 part=untouched
Span and trace coverage, span names, span attributes and span kinds. No span was added or changed.
@task_item n=3 part=untouched
The CI gates: the schema conformance, zero-panics, secret scan, bare-instrument and perf jobs. No gate, threshold or job changed.
@task_item n=4 part=untouched
Panic capture, panic lines and the panic outcome by role. No change.
@task_item n=5 part=untouched
Process-exit, exit-1, exit-21 and send-refused detail codes, and the human stderr refusal pairs. No code was added, and human output did not change.
@task_item n=6 part=untouched
Service identity, heartbeat and liveness, the diagnostics file paths and modes, log levels and the logging stack. No change.
@task_item n=7 part=untouched
Error reporting, OTel and exporter absences, and network egress. No capability was added.
@task_item n=8 part=untouched
The parse-rejected lines of the other parsers (mcp-stdin, claude-agents-json, hyprctl-json, config-json, ledger-stamps). They still hold.
