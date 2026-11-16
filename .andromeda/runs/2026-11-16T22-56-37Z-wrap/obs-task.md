@task_prose part=session
The session added one key, scheduled_wakes, to the turn-ended event data: the count of the session's scheduled wakes, a number or null, never their content.
The hook reads the Stop payload's session_crons array as its length alone, no element read and none redacted or kept; null when the key is absent, null or not an array; null on an API-error (StopFailure) turn end.
A non-array session_crons writes a drift entry {path session_crons, expected array} to the instance detail file only, as background_tasks already does.
The count reaches the hops background_tasks reaches and no others: the turn-ended line in the event stream, CLI wait --json and the MCP wait tool's structured result; not last, list, the snapshot or replay. wait's wake rule is unchanged.
No new IPC method, event kind, log field, parse-rejected word, diagnostics schema word, config key, env var, crate or dependency; turn-ended data goes from three keys to four, an additive field with its version unchanged.
Measured: the recorded Stop fixtures carry an empty session_crons and now read 0; that a live session reads 1 with a wake scheduled is unmeasured and carried to the standing live rig.
@task_file n=1 path=crates/viola-core/src/lib.rs
what: the core event-kind table and each kind's known data keys, the contract the state reader's unknown-field check reads
@task_file n=2 path=crates/viola-agent-claude/src/hook.rs
what: the Claude hook stdin normaliser that maps a Stop or StopFailure payload to the turn-ended event data and writes drift entries for a known field of the wrong type
@task_file n=3 path=src/run/wait.rs
what: the wrapper's wait dispatch, which kinds of event wake a parked wait
@task_file n=4 path=crates/viola-state/tests/state_events.rs
what: the state crate's event-log tests, the known-key and unknown-field checks over written events
@task_file n=5 path=crates/viola-mcp/tests/handlers.rs
what: the MCP tool handler tests, the wait tool's structured result
@task_file n=6 path=tests/hook_events.rs
what: the booted hook-to-event integration tests over a recorded or scripted Stop payload, including the content-canary scan of every home file
@task_file n=7 path=tests/cli_wait_last.rs
what: the CLI wait and last integration tests over a parked wait and its JSON document
@task_file n=8 path=tests/cli_wheel.rs
what: the CLI wheel integration tests, their turn-ended data pins
@task_item n=1 part=touches
The drift-report decisions (drift reports go to the detail file only) and the detail-file line's drift_report field gain a second count-only field, session_crons, whose wrong-type drift entry lands in the instance detail file only.
@task_item n=2 part=touches
The rows that keep the assistant message and other turn content out of logs and home-level files: the new key carries a count alone, no wake prompt, schedule or id, and a planted canary in the wake content is scanned absent from every home file.
@task_item n=3 part=touches
The event stream line and its shape: the turn-ended line carries one more data key, scheduled_wakes, additive with the line version unchanged.
@task_item n=4 part=touches
The viola-mcp wait tool contract: its structured result carries the turn end's scheduled_wakes count unchanged beside background_tasks.
@task_item n=5 part=touches
The rows keeping event payloads out of process logs: the new count rides the event stream and wait results only, and no process log line or diagnostics field gains it.
@task_item n=6 part=touches
Type-level redaction of payload types: the new hook stdin field holds ignored elements and its length only, so it carries no redaction marker and keeps no text.
@task_item n=7 part=touches
The wait log line's outcome and the wait path's events: unchanged, since wait wakes on a turn end whatever its scheduled_wakes, as it does whatever its background_tasks.
@task_item n=8 part=touches
Recorded fixtures as no telemetry source: the recorded Stop fixtures were read, not changed, and their empty session_crons now reads 0.
@task_item n=9 part=touches
The content canary rule (never in home-level files, only in detail files): extended by a booted case over the scheduled-wake content.
@task_fact n=1 slug=pii-scrubbing-compliance
fact: A turn end's scheduled-wake count crosses as the length of the hook's session_crons alone, no element read or kept; a non-array is a drift entry in the instance detail file only, and an API-error turn end writes it null.
@task_item n=1 part=untouched
Process-log line fields, the diagnostics line schema, event and subject enums, and parse-rejected detail words: none added or renamed.
@task_item n=2 part=untouched
Span and trace coverage, span names and span attributes: no span added, renamed or given a field.
@task_item n=3 part=untouched
Metric coverage, SLO budgets, hook perf gates and CI gates: no counter, budget, threshold or gate moved.
@task_item n=4 part=untouched
Error capture, panic handling, exit codes, detail codes and refusal words: none changed.
@task_item n=5 part=untouched
Send, dialog, answer, link, wheel, verify, revive and heartbeat paths and their log lines: not reached.
@task_item n=6 part=untouched
The snapshot, replay, list and last outputs: they do not carry the new count.
