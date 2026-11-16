@task_prose part=session
The session added one machine-read field to a turn end: the turn-ended event now carries scheduled_wakes, the count of the session's scheduled wakes, beside background_tasks.
The count is a number alone, never the wake's content; it is null when the agent's payload holds no list, holds something else, or the turn ended on an API error.
It reaches only the hops background_tasks reaches: the event log line, viola wait --json and the MCP wait tool; viola last, viola list, the snapshot and the replay do not carry it.
No human output changed: no column, word, colour, hint, exit code, refusal or verb was added, and wait wakes on a turn end exactly as before, whatever the count.
Nothing was retired; no new verb, event kind, endpoint, env var or crate.
@task_file n=1 path=crates/viola-core/src/lib.rs
what: the shared event kind table, holding each event kind's known data keys, here the turn-ended event's key set grown from three to four
@task_file n=2 path=crates/viola-agent-claude/src/hook.rs
what: the Claude Code hook normaliser behind the hook verb, mapping the agent's Stop and StopFailure payloads to the turn-ended event and dropping content it does not keep
@task_file n=3 path=src/run/wait.rs
what: viola wait's wake rule, deciding which events end a wait
@task_file n=4 path=crates/viola-state/tests/state_events.rs
what: the state tests that an event's data keys are known keys, so no unknown-field count rises
@task_file n=5 path=crates/viola-mcp/tests/handlers.rs
what: the MCP tool handler tests, here the MCP wait tool passing a turn end's data through unchanged
@task_file n=6 path=tests/hook_events.rs
what: the hook verb's end-to-end tests over recorded agent payloads, here a booted wrapper proving the count crosses and no wake content lands in any home file
@task_file n=7 path=tests/cli_wait_last.rs
what: the CLI tests of viola wait and viola last, their human and --json output, here wait --json carrying the count of a turn end
@task_file n=8 path=tests/cli_wheel.rs
what: the CLI tests of the wheel, who holds control of a session, and its shown words
@task_item n=1 part=touches
viola wait output, event form: the --json document of a turn-ended event now holds scheduled_wakes beside background_tasks; the human event form line is unchanged.
@task_item n=2 part=touches
The --json result as one stdout document: the turn-ended document gains one additive key, still one document, no new stream use.
@task_item n=3 part=touches
CLI callers, LLM driver sessions: a driver now reads a turn end's scheduled-wake count from wait --json and the MCP wait tool, one number and no content.
@task_item n=4 part=touches
API-error turn end shown as any turn end: an API-error end now also carries scheduled_wakes null; its display as an ordinary turn end is unchanged.
@task_item n=5 part=touches
Turn-ended never fills the readback box: still holds, the count is no text and is shown nowhere in human output.
@task_item n=6 part=touches
The hook verb, its exit code and stderr: the hook's mapping of a Stop payload grew one field and a non-list payload value adds a drift entry to the instance detail file; the hook stays silent and fail-open, its exit code and stderr unchanged.
@task_item n=7 part=touches
The wait verb's name and behaviour: wait wakes on a turn end whatever its scheduled_wakes, as before; no new flag or form.
@task_item n=8 part=touches
viola last output, header and message: last does not carry the count; its header and message are unchanged.
@task_item n=9 part=touches
The wheel shown as words and the WHEEL column: one wheel test line was added, no wheel word, column or width moved.
@task_item n=10 part=touches
Nothing announces success before confirmation and no optimistic send confirmation: a scheduled wake is counted, never announced or shown as a pending send.
@task_item n=1 part=untouched
Colour palette, typography, spacing, iconography and glyph rows: no colour, SGR, glyph, separator or width changed.
@task_item n=2 part=untouched
viola list board and table rows: list does not carry the count; columns, status, row order, escaping and skip counts are unchanged.
@task_item n=3 part=untouched
Refusals, hints, exit codes and exit-21 causes: no refusal, hint, code word or exit code was added or moved.
@task_item n=4 part=untouched
viola run, revive, verify, link, unlink, send and pause output rows: none of these verbs' output changed.
@task_item n=5 part=untouched
Brand identity, the flight-progress strip and the readback box contracts, and the web surface absence: no strip field, readback state or surface was added.
@task_item n=6 part=untouched
Self-validation checks and anti-pattern rules: no human-facing pattern was introduced for them to judge.
