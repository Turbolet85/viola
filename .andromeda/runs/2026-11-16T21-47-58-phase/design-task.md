@task_prose part=chunk
The Claude adapter reads the Stop hook payload's `session_crons` as a count only, never keeping or typing its elements, and carries that one number beside `background_tasks` on the normalised `turn-ended` event.
The count reaches every hop `background_tasks` reaches today: the event data in `events.ndjson`, CLI `wait --json`, MCP `wait`, and any further hop P3 measures (for example `last` or `list`); a session with no scheduled wake reads none, as `background_tasks` does.
`wait`'s wake rule does not change, and no wake's content (prompt, cron expression, id, schedule) crosses out of the adapter.
P3 re-measures every hop `background_tasks` takes, re-reads the recorded Stop payloads for the field's shape, and closes one block on the directive's adjacent fact: a CLI-scheduled wake's prompt logged as human, which this entry places but does not build.
@task_file n=1 path=crates/viola-agent-claude/src/hook.rs mark=edits
what: the Claude adapter's hook-payload reader, which normalises the Stop payload into the turn-ended event — edits: the Stop payload read where `background_tasks` is counted
@task_file n=2 path=crates/viola-core/src/lib.rs mark=edits
what: the normalised event types every agent adapter emits, including the turn-ended event's data — edits: the turn-ended event's fields beside `background_tasks`
@task_file n=3 path=src/cmd/run.rs mark=edits
what: the viola run wrapper, which receives hook events and writes them to the event log — edits: where `background_tasks` is carried onward
@task_file n=4 path=src/run/send/slot.rs mark=edits
what: the wrapper's send slot, which holds the turn-ended state a send and a wait read — edits: where `background_tasks` is carried
@task_file n=5 path=src/run/wait.rs mark=edits
what: the wait verb's wake logic and its event-form and --json result — edits: where `background_tasks` is reported on the turn-ended result
@task_file n=6 path=tests/cli_wait_last.rs mark=edits
what: the CLI tests of viola wait and viola last output, human and --json
@task_file n=7 path=tests/cli_wheel.rs mark=edits
what: the CLI tests of the wheel and the turn-end state a driver reads
@task_file n=8 path=tests/hook_events.rs mark=edits
what: the tests of the hook verb normalising recorded Claude hook payloads into events
@task_file n=9 path=crates/viola-mcp/tests/handlers.rs mark=edits
what: the tests of the MCP server's tool handlers, including the wait tool's reply
@task_file n=10 path=crates/viola-state/tests/state_events.rs mark=edits
what: the tests of the event log's read and write of events in `events.ndjson`
@task_file n=11 path=fixtures/claude/2.1.283/Stop.default.json mark=re-reads
what: the recorded Stop hook payload of CLI 2.1.283, read back for the shape of `session_crons`
@task_file n=12 path=fixtures/claude/2.1.287/Stop.default.json mark=re-reads
what: the recorded Stop hook payload of CLI 2.1.287, read back for the shape of `session_crons`
@task_file n=13 path=fixtures/claude/2.1.288/Stop.default.json mark=re-reads
what: the recorded Stop hook payload of CLI 2.1.288, read back for the shape of `session_crons`
@task_file n=14 path=fixtures/claude/2.1.302/Stop.default.json mark=re-reads
what: the recorded Stop hook payload of CLI 2.1.302, read back for the shape of `session_crons`
@task_item n=1 part=needs
The turn-ended event gains one number, the session's scheduled-wake count, beside its background-task count; it carries no content of any wake.
@task_item n=2 part=needs
A session with no scheduled wake reads none: an absent, null or non-array `session_crons` maps exactly as the background-task count's absent shape maps today (null versus zero follows that existing rule).
@task_item n=3 part=needs
The viola wait output in its event form and its --json result carries the scheduled-wake count on a turn end, beside the background-task count.
@task_item n=4 part=needs
The MCP wait tool's reply carries the scheduled-wake count on a turn end, as it carries the background-task count.
@task_item n=5 part=needs
Any further hop that shows the background-task count today (viola last output, the viola list board, or another P3 measures) shows the scheduled-wake count beside it.
@task_item n=6 part=needs
The wait wake rule stays as it is: no wake, hold or completion decision of viola wait reads the new count; a turn end still wakes wait exactly as before.
@task_item n=7 part=needs
No wake's content (prompt, cron expression, id, schedule or any string from it) reaches an event, snapshot, log line, diagnostics file, reply, fixture, human output or --json document.
@task_item n=8 part=needs
The field's normalised name is set under the core's naming rule for normalised events; no Claude-specific name crosses out of the adapter into any output, reply or log.
@task_item n=9 part=needs
Every reader of viola's own formats keeps skipping unknown fields, so an older binary reading an event, reply or --json document with the added field does not fail.
@task_item n=10 part=needs
Indirect, through the --json result rule: the added count joins the one stdout document a --json verb prints; no second document and no field outside it.
@task_item n=11 part=needs
Indirect, through the human-output rules: if a human-mode line shows the count, it is ASCII-only, plain words and numbers, with no glyph, colour or hint added for it, and a turn end is still shown as any turn end, never filling the readback box.
@task_item n=12 part=needs
Whether the count needs a capability-ledger row with a verify probe follows how the background-task count stands on that question; it is undocumented-CLI telemetry that fails to none when the CLI stops sending it.
@task_item n=13 part=needs
The plan closes one block on the directive's adjacent fact (a CLI-scheduled wake's prompt logged as human origin, the wheel going to human, driver sends refused human-typing, the next dialog not held): what this entry's count covers of it, what it leaves, what a further step would be, and the placement options; nothing of it is built here.
@task_item n=14 part=needs
The new tests run on the fake agent and the recorded payloads, with no live CLI start and no scheduled wake set; a test waiting on a channel or thread takes a bound, a test socket stands under the short temporary root, and a stamped home is copied from the shared stamp.
@task_standing n=1
convention: every --json result a verb prints is one document on stdout, its messages on stderr
home: surface-cli
@task_standing n=2
convention: every human-mode line is ASCII-only, with control characters escaped and no glyph or emoji
home: surface-cli · anti-patterns-never-do-these
@task_item n=1 part=not_needed
Colour palette, typography, spacing, depth and border-radius rows: the count adds no colour, weight, span or width of its own.
@task_item n=2 part=not_needed
Send readback rows and the readback box: the chunk changes no send, and a turn end never fills the readback box.
@task_item n=3 part=not_needed
viola revive, viola run start refusals and their --json refusal codes: no start, revive or refusal path changes.
@task_item n=4 part=not_needed
viola verify output and refusals, and viola plugin install output: no verify step, stamp or install changes.
@task_item n=5 part=not_needed
viola link and unlink output and their --json documents: pairing is untouched.
@task_item n=6 part=not_needed
Exit-21 causes, wrapper fault --json detail, peer reply screening and strict-modes refusals: no new failure, cause or refusal is added.
@task_item n=7 part=not_needed
viola --help verb groups and the command names: no verb is added, renamed or regrouped.
@task_item n=8 part=not_needed
The web-page surface and rendered-page absences, and the self-validation token test: viola renders no page.
@task_item n=9 part=not_needed
CLI colour decision order and colour depth: the chunk adds no styled span.
