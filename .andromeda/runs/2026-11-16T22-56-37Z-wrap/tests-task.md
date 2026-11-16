@task_prose part=session
The session added one key to the turn-ended event data: scheduled_wakes, the count of the session's scheduled wakes, one number and no content.
It is the length of the Stop hook payload's session_crons array; null when that key is absent, null or not an array (a non-array writes a drift entry expecting array to the instance detail file); null on an API-error (StopFailure) turn end.
It reaches exactly the hops background_tasks reaches: the turn-ended line in events.ndjson, CLI wait --json and MCP wait structuredContent; not last, list, the snapshot or the replay.
The turn-ended data key set goes from three to four keys (last_assistant_message, background_tasks, scheduled_wakes, cause optional); the event line version is unchanged; wait's wake rule is unchanged.
No new IPC method, endpoint, event kind, env var, crate, schema file, config key, log field or parse-rejected word; nothing retired; no human output changed.
Seven new tests across unit, integration and e2e layers, and a booted hook case that scans every home file for a planted canary.
@task_file n=1 path=crates/viola-core/src/lib.rs
what: the shared event kind table, each event kind's data keys (the KIND_TABLE contract pin), which gains scheduled_wakes on turn-ended
@task_file n=2 path=crates/viola-agent-claude/src/hook.rs
what: the claude hook stdin normaliser, mapping a recorded hook payload to event data; its Stop arm now reads session_crons by length only and its StopFailure arm writes scheduled_wakes null; unit cases for the count and the null cases
@task_file n=3 path=src/run/wait.rs
what: the wrapper's wait wake rule (which records wake a wait), with a unit case that a turn end wakes whatever its scheduled_wakes
@task_file n=4 path=crates/viola-state/tests/state_events.rs
what: the event log reader integration suite, the tolerant reader's unknown-field count; a case that scheduled_wakes is a known key
@task_file n=5 path=crates/viola-mcp/tests/handlers.rs
what: the MCP tool handler suite; a case that MCP wait carries the scheduled-wake count unchanged in structuredContent
@task_file n=6 path=tests/hook_events.rs
what: the e2e hook suite over a booted wrapper fed recorded Stop stdin; a case that a Stop holding scheduled wakes carries their count and no content, and the recorded Stop fixtures' empty session_crons reading 0
@task_file n=7 path=tests/cli_wait_last.rs
what: the e2e suite for viola wait and viola last; a case that wait --json carries the scheduled-wake count of a turn end
@task_file n=8 path=tests/cli_wheel.rs
what: the e2e suite for the wheel and turn-ended records; one line added to its expected turn-ended data
@task_item n=1 part=touches
The integration contract on turn-ended event data keys is stale: turn-ended now names four keys, scheduled_wakes added beside last_assistant_message, background_tasks and the optional cause.
@task_item n=2 part=touches
The e2e contract on turn-ended event data background_tasks owes a sibling: scheduled_wakes, the session_crons array length, null when absent, null or not an array, null on an API-error end, reaching the same hops and no others.
@task_item n=3 part=touches
The unit contract on dialog and Stop event data null details (description, preview and background_tasks) owes the scheduled_wakes sibling: null when the Stop payload key is absent or not an array, and null on a StopFailure turn end.
@task_item n=4 part=touches
Wait wake-kind contracts (the channel wait method's wake kinds, wait wake kinds, viola wait wake on turn-ended, viola wait turn-ended wake): a turn end wakes whatever its background_tasks or scheduled_wakes; the rule is unchanged but a row that enumerates the turn-ended data it returns now reads one more key.
@task_item n=5 part=touches
Unknown hook payload fields tolerated and harness or tolerant event reader rows: session_crons moves from an ignored unknown Stop field to a known one read by length only, and scheduled_wakes is a known key, unknown_fields 0.
@task_item n=6 part=touches
The event line keys and the event line round-trip contracts: turn-ended data gains a key while the line's v stays unchanged, an additive field.
@task_item n=7 part=touches
CLI and MCP return identical payloads: CLI wait --json and MCP wait structuredContent both carry scheduled_wakes unchanged.
@task_item n=8 part=touches
Secrets never in logs, synthetic inputs carry a canary, and no secrets in logs or diagnostics: the new Stop field is read as a count only, no wake element text is kept; a booted case plants canary text in the session_crons elements and scans every home file for it.
@task_item n=9 part=touches
The fake agent held to recorded fixtures and recorded hook payloads walked by one test: the recorded Stop fixtures carry an empty session_crons, which now reads 0; the fixtures themselves are unchanged.
@task_item n=10 part=touches
viola last newest turn message and the last document: last does not carry scheduled_wakes; a row asserting the last document's key set holds.
@task_item n=11 part=touches
Snapshot replay recovers a fixed field set: scheduled_wakes is not on the snapshot or the replay; the field set holds.
@task_item n=12 part=touches
Remove-the-guard controls: four controls (the count, the drift entry, the known key, the absence of wake content) each read failed with the guard removed.
@task_fact n=1 slug=e2e-test-strategy
fact: turn-ended event data carries scheduled_wakes, the Stop payload's session_crons array length, null when the key is absent, null or not an array and on an API-error end; it reaches events.ndjson, CLI wait --json and MCP wait only, never last, list, the snapshot or the replay.
@task_fact n=2 slug=unit-test-strategy
fact: the Stop hook's session_crons is read by length alone, no element read or kept; a non-array value writes a drift entry expecting array to the instance detail file only, as background_tasks does.
@task_item n=1 part=untouched
Dialog decision bodies, answer and send refusal orders, send confirmation windows and local commands: no dialog or send path changed.
@task_item n=2 part=untouched
Wheel ownership and release rules, human input taking the wheel, harness-injected turns keeping the wheel: the session reads no prompt-submit source and changes no wheel rule.
@task_item n=3 part=untouched
Strict modes, socket directory, server identity, peer euid and instance-unreachable cause rows: no boundary check changed.
@task_item n=4 part=untouched
The harness contract, agent-run status, gate, pre-push, coverage, mutation, perf and CI integration rows: no harness, gate or workflow surface changed.
@task_item n=5 part=untouched
viola list, the board, revive, run start events, process-start records and verify stamps: none carries the new key.
@task_item n=6 part=untouched
Log format, process log line fields and log event corr rows: no log field or parse-rejected word changed.
@task_item n=7 part=untouched
Fake agent command line, receipts, scripts and recorded fixture sets per CLI version: no fixture or fake-agent option changed.
