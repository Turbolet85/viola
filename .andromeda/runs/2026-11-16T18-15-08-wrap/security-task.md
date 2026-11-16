@task_prose part=session
An API-error turn end (the hook's StopFailure event) now names its cause from a closed list of nine words: rate-limited, overloaded, auth-failed, billing, invalid-request, model-not-found, server-error, output-limit, api-error.
The Claude adapter maps the CLI's 13 StopFailure error words onto those nine. An absent error word reads api-error. Any other string reads api-error and is marked unknown. The error text is still read only to classify and is never copied.
The adapter-internal API-error classes and their classifier function were retired. The classification now crosses the boundary only as the closed cause word, and the classifier still lives only in the adapter.
For a StopFailure whose error string is outside the 13 words, viola hook writes one parse-rejected WARN line (parser hook-stdin, detail unknown-value, count 1) with the code and count only, then delivers the event as before. The diag-line schema's parse-rejected detail enum grows from 8 to 9 values.
The wrapper's hook.event re-check accepts a turn-ended cause that is absent or one of the nine, and refuses anything else. The word reaches events.ndjson, wait --json and MCP wait unchanged. Nothing new was added: no IPC method, endpoint, socket, env var, event kind, crate or dependency.
The 13 CLI error words were read from the installed CLI bundles' source and were not seen in any live run. The nine-word map is therefore unmeasured against real payloads. The founder's ruling A makes the nine words standing. The unknown-value notice rests on the overseer's technical answer, not the founder's word.
@task_file n=1 path=crates/viola-core/src/lib.rs
what: edited — the core crate's shared closed types and validators, where the turn-end cause enum grows from one word to nine and its closed name check follows that list
@task_file n=2 path=crates/viola-agent-claude/src/hook.rs
what: edited — the Claude adapter's hook payload parser: it maps the StopFailure error word to the closed cause word, drops the error text at the parse, marks an outside value as unknown, and no longer holds the API-error classes
@task_file n=3 path=src/cmd/hook.rs
what: edited — the viola hook command, which reads hook stdin, writes the parse-rejected refusal log line and sends the hook.event frame; it now writes a codes-only unknown-value line and still delivers the event
@task_file n=4 path=schemas/diag-line.v1.json
what: edited — the diagnostics log line schema, where the parse-rejected detail enum gains unknown-value
@task_file n=5 path=src/cmd/run.rs
what: edited — the wrapper's hook.event handler, which re-validates the turn-ended cause against the closed set before it appends to the event log; only its test cases were added
@task_file n=6 path=src/run/send/slot.rs
what: edited — the wrapper's send slot, which clears the running turn on a turn end; one test row was added
@task_file n=7 path=crates/viola-state/tests/state_events.rs
what: edited — the state crate's tests for the event log (events.ndjson)
@task_file n=8 path=tests/hook_events.rs
what: edited — the booted hook-event tests: real viola hook processes under a booted wrapper. A new case covers each CLI error word, checks that no planted error text reaches the home, and validates the role log against the schema
@task_file n=9 path=tests/cli_wheel.rs
what: edited — the CLI tests for send and the wheel: a turn end lets the next send through
@task_file n=10 path=tests/cli_list.rs
what: edited — the CLI tests for viola list, which reads STATUS idle after an API-error turn end
@task_file n=11 path=crates/viola-mcp/tests/handlers.rs
what: edited — the MCP server's handler tests: the wait tool carries the turn-end cause word unchanged in structuredContent
@task_item n=1 part=touches
data-protection decision/api-error-classes-adapter-only is changed. The API-error classes and their classifier function are retired. The classification now crosses as one of the nine closed cause words. The classifier stays inside the adapter, and the error text still never crosses. The row's provisional standing ends on the founder's ruling A.
@task_item n=2 part=touches
data-protection decision/api-error-turn-end-text-dropped is changed. The cause word is now one of nine, not api-error alone. The error text is still read only to classify and is never copied. Background tasks and the last assistant message stay null.
@task_item n=3 part=touches
input-validation contract/channel-frames/hook-event-revalidation is changed. Its value "api-error alone" becomes the nine words. The wrapper appends a turn-ended event only when the cause is absent or one of the nine, and refuses any other word, including the CLI's raw words.
@task_item n=4 part=touches
input-validation decision/hook-security-fields-closed and decision/hook-security-fields-closed-enums are touched indirectly: the turn-ended cause stays a closed enum, now with nine members, and its name check is derived from that list.
@task_item n=5 part=touches
Tolerant parsing rows (threat-model decision/tolerant-hook-parsing, decision/child-output-tolerant-unknown, decision/hook-payload-is-content) are touched indirectly. An unknown StopFailure error string now fails to api-error plus a counted, codes-only notice, and is never rejected or copied.
@task_item n=6 part=touches
input-validation hook stdin contracts (contract/hook-stdin/oversize, contract/hook-stdin/not-one-object) gain a sibling. viola hook now writes a parse-rejected line with detail unknown-value for an outside error value. It is a new hook-stdin refusal-log outcome, and the event is still delivered.
@task_item n=7 part=touches
input-validation contract/mcp-stdin/parse-rejected and contract/channel-response-log/detail are touched only if a cell lists the parse-rejected detail values or counts them. That enum moves from 8 to 9 values with unknown-value.
@task_item n=8 part=touches
Logging floor rows (bootstrap decision/user-content-only-to-diagnostics, decision/never-log-claude-values, decision/error-display-no-upstream-text; threat-model decision/upstream-text-is-content) are touched indirectly. The new unknown-value line carries the code and count only, with no planted value in any role or detail line.
@task_item n=9 part=touches
threat-model decision/hook-fails-open and input-validation decision/hooks-fail-open are touched indirectly. A marked StopFailure still delivers its turn-ended event after the notice. The hook stays silent and exits 0.
@task_item n=10 part=touches
input-validation decision/invalid-hook-frames-dropped is touched indirectly. A hook.event whose cause is outside the nine is not appended.
@task_item n=11 part=touches
The wait surfaces (threat-model contract/mcp-tools/wait, contract/wrapper-channel-methods/wait, contract/wrapper-channel-methods/hook-event) are touched where a cell names the turn-ended cause value. The word now reaches wait --json and MCP wait as one of nine. No new method, tool or field was added.
@task_item n=12 part=touches
threat-model decision/client-reads-closed-values-only and input-validation decision/client-reply-code-words-only are touched only where a cell enumerates the turn-end cause. The closed value set clients read grows additively, with no v bump.
@task_item n=13 part=touches
input-validation decision/shared-validators-in-core is touched indirectly. The core crate's closed cause type is the one list that both the adapter's map and the wrapper's re-check follow.
@task_fact n=1 slug=input-validation
fact: For a StopFailure whose error value is outside the CLI's closed word list, viola hook writes exactly one parse-rejected line (parser hook-stdin, detail unknown-value, count 1) carrying the code and count only, never the value, and still delivers the event with cause api-error. This rests on the overseer's technical answer, not the founder's word.
@task_fact n=2 slug=data-protection
fact: The CLI's StopFailure error words were read from the installed CLI source (bundles 2.1.287 to 2.1.325), not from a live run. The nine-word cause map is unmeasured against real payloads, and that live reading stays owed.
@task_item n=1 part=untouched
Authentication and authorization rows are not touched: peer euid checks, socket directory and socket mode, strict-modes checks and refusals, server verification, release-from-driver, and the home owner-only rules.
@task_item n=2 part=untouched
Dependency security is not touched. No crate, dependency, toolchain, workflow, cargo-deny or zizmor setting was added or bumped.
@task_item n=3 part=untouched
Secret management is not touched: credential stripping, the CLAUDE env strip, the canary scan and stored secrets.
@task_item n=4 part=untouched
API security's rmcp feature rows are not touched. No MCP tool, feature or transport was added.
@task_item n=5 part=untouched
The instance-unreachable cause and detail words (endpoint-lost, instance-dead and others) share the token "cause word" but are a different closed list, and they did not move.
@task_item n=6 part=untouched
Send, answer and paste text validation is not touched: the character sets, control-character and empty-text refusals, and bracketed paste.
@task_item n=7 part=untouched
Pinned binary, plugin files, verify probes, fixture scrubbing and Hyprland recorder rows in data protection are not touched.
@task_item n=8 part=untouched
Link, unlink, revive, the board and list home-read rules, and the CLI exit codes are not touched. No exit code or refusal code moved.
@task_item n=9 part=untouched
The hook-decision detail contracts (channel-unreachable, malformed-json, oversize-stdin and others) are not touched. The new notice is a parse-rejected line, not a hook-decision line.
