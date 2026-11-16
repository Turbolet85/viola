@task_prose part=session
The session added one data key, scheduled_wakes, to the turn-ended event: the count of the session's scheduled wakes beside background_tasks, one number and no wake content.
The count is the length of the Stop payload's session_crons array; it is null when the key is absent, null or not an array (a non-array writes a drift entry to the instance detail file), and null on an API-error (StopFailure) turn end.
It reaches what background_tasks reaches and nothing else: the turn-ended line in events.ndjson, the CLI viola wait --json document and the MCP wait tool's structuredContent; not viola last, viola list, the snapshot or the replay.
The turn-ended data key set grew from three keys to four (last_assistant_message, background_tasks, scheduled_wakes, cause when present); wait wakes on a turn end whatever its scheduled_wakes; no human output changed, and no verb, flag, exit code, refusal, event kind or MCP tool was added.
Measured: the recorded Stop fixtures carry an empty session_crons and now read 0; a live session reading 1 with a wake scheduled is unmeasured and carried as a provisional item.
@task_file n=1 path=crates/viola-core/src/lib.rs
what: the core event kind table and each event kind's data keys, the turn-ended data contract that wait and the MCP tools hand on - edited
@task_file n=2 path=crates/viola-agent-claude/src/hook.rs
what: the hook normaliser that maps the claude CLI's Stop and StopFailure hook payloads to the turn-ended event data, count-only for background tasks and scheduled wakes - edited
@task_file n=3 path=src/run/wait.rs
what: the viola wait wake rule, which events end a wait (turn-ended, dialog, timeout) - edited, tests only
@task_file n=4 path=crates/viola-state/tests/state_events.rs
what: the state events tests, that a turn-ended data key is a known field - edited
@task_file n=5 path=crates/viola-mcp/tests/handlers.rs
what: the viola mcp tool handler tests, the wait tool's success result and its structuredContent - edited
@task_file n=6 path=tests/hook_events.rs
what: the end-to-end hook tests, a booted wrapper turning hook payloads into events with no content kept - edited
@task_file n=7 path=tests/cli_wait_last.rs
what: the viola wait and viola last CLI tests, the wait --json turn-ended document - edited
@task_file n=8 path=tests/cli_wheel.rs
what: the wheel verb CLI tests, viola pause and viola release output - edited, one line
@task_item n=1 part=touches
contract/cli-verbs/wait: the viola wait --json document for a turn end now carries scheduled_wakes (a count or null) beside background_tasks.
@task_item n=2 part=touches
contract/mcp-tools/wait: the MCP wait tool's turn-ended result carries scheduled_wakes in structuredContent, handed on unchanged.
@task_item n=3 part=touches
contract/mcp-tool-results/success: a wait success result's structuredContent for a turn end gains the scheduled_wakes key.
@task_item n=4 part=touches
contract/cli-json-mode/one-document: the wait --json turn-ended document gains one key; any cell that enumerates the turn-ended data keys (three keys) now reads four.
@task_item n=5 part=touches
contract/viola-wait-output/turn-ended: the human wait output for a turn end is unchanged, but a cell naming the turn-ended data (last_assistant_message, background_tasks, cause) or saying no scheduled-wake count is carried is now stale.
@task_item n=6 part=touches
decision/turn-ended-not-send-outcome: a rule over the turn-ended event, whose data gained scheduled_wakes; the wake rule (wait wakes on any turn end whatever its background_tasks or scheduled_wakes) holds.
@task_item n=7 part=touches
contract/mcp-tools/last and contract/cli-verbs/last and contract/viola-last-output/message: scheduled_wakes deliberately does not reach last; a row saying last carries what wait carries would be wrong.
@task_item n=8 part=touches
contract/viola-list-json/document and contract/mcp-tools/list: scheduled_wakes deliberately does not reach list or the snapshot; a row that lists turn-ended data reaching list would be wrong.
@task_item n=9 part=touches
An API-error turn end: wait's turn-ended data now holds last_assistant_message, background_tasks and scheduled_wakes all null with cause; any row enumerating the StopFailure nulls gains scheduled_wakes.
@task_item n=10 part=touches
decision/human-output-is-contract: the session changed no human output line (wait, last, list, wheel pins unchanged) - a check, not a change.
@task_fact n=1 slug=surface-cli
fact: viola wait --json and the MCP wait tool carry, for a turn end, scheduled_wakes beside background_tasks - the count of the session's scheduled wakes, one number and no wake content, null when the CLI sent none or an unreadable value and null on an API-error turn end; viola last, viola list and the snapshot do not carry it, and the human wait output does not show it.
@task_fact n=2 slug=surface-cli
fact: the name scheduled_wakes is occupied as a turn-ended data key in the --json and MCP surfaces (the CLI's own field session_crons is never surfaced by that name).
@task_item n=1 part=untouched
The board and viola list human output: columns, colours, STATUS, DIALOG and WHEEL cells, truncation and the BAY line are unchanged.
@task_item n=2 part=untouched
viola send, its readback mirror, word column, refusals and --json document are unchanged.
@task_item n=3 part=untouched
viola revive, viola run starts, their refusals and --json error codes are unchanged.
@task_item n=4 part=untouched
viola verify, viola plugin install, viola link and unlink output are unchanged.
@task_item n=5 part=untouched
CLI exit codes, hints, refusal lines, exit-21 details and the CLI token rendering are unchanged; no new verb, flag or exit code.
@task_item n=6 part=untouched
viola answer, the dialog outputs and viola wait's dialog, dialog-expired, timed-out and waiting lines are unchanged.
@task_item n=7 part=untouched
The web surface absence and the deep-link absence stand.
