@task_prose part=session
A turn end now carries the session's scheduled-wake count beside its background-task count: one number, no content.
The turn-ended event data gains the key scheduled_wakes, the length of the Stop hook payload's session_crons array; null when the key is absent, null or not an array; null on an API-error (StopFailure) turn end.
No element of the array is read or kept; a non-array value writes one drift entry to the instance diagnostics detail file only, as background_tasks does.
The count reaches the same hops background_tasks reaches (the turn-ended line in the event log, CLI wait --json, MCP wait structuredContent) and no others; wait wakes on a turn end as before.
No new channel method, endpoint, event kind, socket, port, env var, crate, dependency, config key or schema file; nothing retired.
Measured from recorded fixtures only: a recorded Stop with an empty session_crons now reads 0; a live session reading 1 with a wake scheduled is unmeasured and carried as a PROVISIONAL live-rig reading.
@task_file n=1 path=crates/viola-core/src/lib.rs
what: edited — the core crate's shared contract types: the event kind table and each kind's closed list of known data keys, which the event log reader checks unknown fields against
@task_file n=2 path=crates/viola-agent-claude/src/hook.rs
what: edited — the Claude adapter's hook payload normaliser: the tolerant parse of hook stdin into known fields, the drift entries for a field of the wrong type, and the mapping of Stop and StopFailure payloads to turn-ended event data
@task_file n=3 path=src/run/wait.rs
what: edited — the wrapper's wait verb: which events wake a parked wait and what a turn end's data carries back to the driver
@task_file n=4 path=crates/viola-state/tests/state_events.rs
what: edited — the state crate's event log reader tests: known-key and unknown-field checks over written event lines
@task_file n=5 path=crates/viola-mcp/tests/handlers.rs
what: edited — the MCP server's tool handler tests: the wait tool's structuredContent passed through unchanged
@task_file n=6 path=tests/hook_events.rs
what: edited — the booted end-to-end hook tests: a hook payload through the wrapper into events, with a planted-canary scan of every home file for content that must not be kept
@task_file n=7 path=tests/cli_wait_last.rs
what: edited — the CLI wait and last end-to-end tests: the --json document a driver reads back from a turn end
@task_file n=8 path=tests/cli_wheel.rs
what: edited — the CLI wheel end-to-end tests: the turn-ended data a wheel test asserts on, human output unchanged
@task_item n=1 part=touches
decision/stop-background-tasks-count-only: its sibling is owed — Stop's session_crons now crosses the hook boundary the same way, as a length only, no element read, null when absent, null or not an array, a non-array writing a drift entry.
@task_item n=2 part=touches
decision/api-error-turn-end-text-dropped: stale — an API-error turn end now carries null last_assistant_message, null background_tasks and null scheduled_wakes beside its cause.
@task_item n=3 part=touches
decision/hook-payload-is-content and decision/tolerant-hook-parsing: the hook payload's session_crons elements (wake ids, schedules, prompts) are content; the tolerant parse now reads one more field, its length only, ignoring every element.
@task_item n=4 part=touches
decision/drift-reports-diagnostics-only: a new drift entry path session_crons expected array goes to the instance diagnostics detail file only, never to the event data.
@task_item n=5 part=touches
decision/hook-security-fields-closed and decision/hook-security-fields-closed-enums: a new hook field is read; it is a count, not a security field, and its drift word is array.
@task_item n=6 part=touches
decision/user-content-only-to-diagnostics, decision/never-log-claude-values and decision/upstream-text-is-content: no wake prompt or schedule text reaches events, snapshot or logs; a booted test scans every home file for a planted canary.
@task_item n=7 part=touches
contract/mcp-tools/wait and contract/wrapper-channel-methods/wait: the wait reply's turn-ended data carries one more key, scheduled_wakes; the wake rule is unchanged and MCP structuredContent passes it unchanged.
@task_item n=8 part=touches
decision/shared-validators-in-core and decision/closed-types-for-protocol-fields: the core crate's closed list of turn-ended data keys grows from three to four (last_assistant_message, background_tasks, scheduled_wakes, cause optional).
@task_item n=9 part=touches
decision/api-error-classes-adapter-only: the StopFailure arm of the Claude adapter now also writes scheduled_wakes null; the class words stay inside the adapter.
@task_item n=10 part=touches
decision/question-option-text-crosses-boundary: any row listing what of a hook payload crosses into events now has one more crossing, a bare count.
@task_fact n=1 slug=input-validation
fact: Stop's session_crons crosses the hook boundary as its length alone, scheduled_wakes on turn-ended: no element read or kept, null when absent, null or not an array, a non-array writing one drift entry to the diagnostics detail file; null on an API-error turn end.
@task_fact n=2 slug=data-protection
fact: The scheduled-wake count reaches only the hops the background-task count reaches (the turn-ended event line, CLI wait --json, MCP wait); last, list, the snapshot and the replay do not carry it.
@task_item n=1 part=untouched
Authentication and authorization: the socket directory, peer euid checks, server verification, strict-modes checks and their refusals are unchanged.
@task_item n=2 part=untouched
Dependency security: no crate, dependency, feature, pin, toolchain, workflow or audit gate changed.
@task_item n=3 part=untouched
Secret management: the child environment strip, credential handling and the artifact canary scan classes are unchanged.
@task_item n=4 part=untouched
Paste and dialog text validation, send and answer refusals and their control-character sets: no typed-text path changed.
@task_item n=5 part=untouched
Pinned binary, plugin install, verify and its probes, fixture recording and scrubbing: no recorded fixture changed and no verify path moved.
@task_item n=6 part=untouched
Frame caps, JSON depth limits, MCP stdin line cap and channel frame params: no reader bound or frame shape changed; no new channel method, event kind, listener or env read.
@task_item n=7 part=untouched
Error handling and refusal words, CLI exit codes, human-output escaping: no error writer, refusal code or human output changed.
