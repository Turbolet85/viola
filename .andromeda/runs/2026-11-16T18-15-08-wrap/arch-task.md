@task_prose part=session
The turn-end cause of an API-error turn end grew from the one word api-error to a closed list of nine kebab-case words: rate-limited, overloaded, auth-failed, billing, invalid-request, model-not-found, server-error, output-limit, api-error.
The Claude adapter now classifies the CLI's StopFailure error word (13 known words) into one of the nine; an absent word reads api-error, any other string reads api-error and is marked unknown; the error text itself still crosses nothing.
The adapter-internal API-error class enum and its classifier function were retired; the classification now crosses the adapter boundary only as the turn-end cause word, and the classifier stays in the adapter.
A StopFailure whose error string is outside the 13 makes the hook write one codes-only parse-rejected WARN line (parser hook-stdin, detail unknown-value, count 1) and then deliver the event as before; the diag-line schema's parse-rejected detail enum grew from eight to nine values.
No new IPC method, endpoint, socket, port, env var, event kind, crate or dependency; the turn-ended cause set grew additively with no protocol or record v bump.
The 13 CLI error words were read from the installed CLI bundles' source, not measured in a live run; the founder ruled the nine-word list (ruling A), the unknown-value notice rests on the overseer's technical answer.
@task_file n=1 path=crates/viola-core/src/lib.rs
what: viola-core's shared agent-neutral types, holding the turn-end cause enum (the closed cause set, its wire spelling and its from_name parse)
@task_file n=2 path=crates/viola-agent-claude/src/hook.rs
what: the Claude adapter's hook payload normaliser, the hook-to-kind map that turns a StopFailure into a turn-ended event and its data, now with the StopFailure error-word classifier
@task_file n=3 path=src/cmd/hook.rs
what: the root bin's viola hook handler, which reads hook stdin, writes the hook role's codes-only diagnostics and delivers the event over the channel (fail-open)
@task_file n=4 path=schemas/diag-line.v1.json
what: the diagnostics line schema, holding the parse-rejected line's closed detail enum
@task_file n=5 path=src/cmd/run.rs
what: the wrapper's run loop and its channel hook.event handling, which re-checks the turn-ended cause against the closed set before appending the event line
@task_file n=6 path=src/run/send/slot.rs
what: the wrapper's send slot, the running-turn rung a turn end clears before the next send passes
@task_file n=7 path=crates/viola-state/tests/state_events.rs
what: viola-state's tests of the event log and its replay into the instance snapshot
@task_file n=8 path=tests/hook_events.rs
what: booted end-to-end tests of hook events crossing hook, wrapper and events.ndjson against the fake agent
@task_file n=9 path=tests/cli_wheel.rs
what: end-to-end tests of the wheel and send gating through the CLI
@task_file n=10 path=tests/cli_list.rs
what: end-to-end tests of viola list, the board and its status column
@task_file n=11 path=crates/viola-mcp/tests/handlers.rs
what: viola-mcp's handler tests of the MCP tools, among them wait
@task_item n=1 part=touches
The turn-end cause contract's cause set: one member (api-error) becomes nine words in a fixed order; the row naming the cause set is stale.
@task_item n=2 part=touches
The hook-to-kind map row for StopFailure: cause is now the classified word from the CLI error word, not the literal api-error; last_assistant_message and background_tasks stay null, the error text is still read only to classify.
@task_item n=3 part=touches
The turn-ended cause event-data contract: cause is one of nine words; its provisional-ahead-of-ratification qualifier ends on the founder's ruling.
@task_item n=4 part=touches
The rule that an API-error turn end clears the running turn: still true of every one of the nine words (each clears the turn and reads status idle), names no word.
@task_item n=5 part=touches
The normalised event kind turn-ended and the channel hook.event appended-line contract: the wrapper's closed-set re-check of cause now admits the nine words and still refuses raw CLI words, error text or any outside value.
@task_item n=6 part=touches
The convention that unparseable external values become unknown: an outside StopFailure error string reads api-error and is counted by a codes-only notice; the catch-all word here is api-error, not unknown.
@task_item n=7 part=touches
The diagnostics line contracts and the hook role file: the hook role file can now hold a parse-rejected line with parser hook-stdin and detail unknown-value; the parse-rejected detail enum count moved 8 to 9.
@task_item n=8 part=touches
The codes-only role log and process-log rules: the new unknown-value line carries code and count only, never the outside value or any error text.
@task_item n=9 part=touches
Upstream text is content and untrusted, sanitised error surfaces, tolerant external payloads: the CLI error text is still dropped at the parse; only a closed word crosses.
@task_item n=10 part=touches
Claude shapes in one crate and the adapter-is-pure rule: the CLI error-word vocabulary and its classifier live only in the adapter; viola-core holds only the agent-neutral cause words.
@task_item n=11 part=touches
Event kinds added only in viola-core: no kind added, but the cause enum in viola-core grew; agent-neutral words, no Claude word in core.
@task_item n=12 part=touches
Enum string values on the wire in kebab-case: the nine new wire words follow it.
@task_item n=13 part=touches
The protocol version bump rule and versioned-tolerant own formats: the cause value set grew additively without a v bump on the event line or snapshot.
@task_item n=14 part=touches
MCP wait and channel wait result, CLI wait --json: the cause word reaches them unchanged; any row that names api-error as the only cause a waiter sees is stale.
@task_item n=15 part=touches
Hooks fail open and the hook never writes stderr: the unknown-value notice goes to diagnostics only and the event is delivered as before.
@task_fact n=1 slug=conventions
fact: A StopFailure's CLI error word maps in the Claude adapter onto the nine turn-end causes (rate_limit rate-limited, overloaded overloaded, five auth words auth-failed, billing_error billing, invalid_request invalid-request, model_not_found model-not-found, server_error server-error, max_output_tokens output-limit, unknown api-error); an absent word reads api-error, any other string reads api-error marked unknown.
@task_fact n=2 slug=conventions
fact: A StopFailure error string outside the CLI's known words makes the hook write one codes-only parse-rejected line (parser hook-stdin, detail unknown-value, count 1) and still deliver the event; the outside value is in no line (provisional, delegate overseer).
@task_fact n=3 slug=crate-dependency-direction
fact: The API-error classification stays in the Claude adapter; only the agent-neutral turn-end cause word crosses out of it, never the CLI error word or text.
@task_item n=1 part=untouched
The dialog hold, answer, permission, question and plan flows: no dialog rule moved.
@task_item n=2 part=untouched
The instance-unreachable cause set and frame-verb exit details: a different closed cause list sharing the words cause word, unchanged.
@task_item n=3 part=untouched
Occupied names: no crate, binary, command, MCP tool, environment variable, socket or event kind taken or freed.
@task_item n=4 part=untouched
Stack and build system: no dependency added or bumped, no toolchain, deny, lint, CI, release or deployment change.
@task_item n=5 part=untouched
Revive, verify, the capability ledger and stamps, the pinned copy and its rehash: not reached.
@task_item n=6 part=untouched
Link and unlink and the link set: no rule changed.
@task_item n=7 part=untouched
Human CLI output, colour and hint lines: no human-output change; the cause word reaches only --json and MCP.
@task_item n=8 part=untouched
Channel fault codes, refusal reasons and refusal details: no code or detail added.
