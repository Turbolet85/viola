@task_prose part=chunk
A turn end counts the session's scheduled wakes: the Claude adapter reads the Stop payload's session_crons as a count only and carries it as one number beside background_tasks on the normalised turn-ended event.
The count reaches every hop background_tasks reaches today: the event line in events.ndjson, viola wait --json, the MCP wait tool, and any further hop P3 measures.
A session with no scheduled wake reads none; no wake's content crosses out of the adapter; wait's wake rule does not change.
P3 re-measures every hop of background_tasks and the recorded Stop payloads, and re-reads from the code whether the count lets a scheduled wake's prompt be told from a typed one (one block, placed, not built).
@task_file n=1 path=crates/viola-agent-claude/src/hook.rs mark=edits
what: the Claude adapter's hook payload read, the single vendor-content boundary — edits: the Stop payload read of background_tasks, joined by a session_crons count
@task_file n=2 path=crates/viola-core/src/lib.rs mark=edits
what: the normalised event kinds of viola-core — edits: the turn-ended event's data, a new count field beside background_tasks
@task_file n=3 path=src/cmd/run.rs mark=edits
what: viola run, the wrapper that writes the event line to events.ndjson — edits: the turn-ended event line's data
@task_file n=4 path=src/run/send/slot.rs mark=edits
what: the send slot that carries a turn end's data to the driver — edits: the turn-end data carried beside background_tasks
@task_file n=5 path=src/run/wait.rs mark=edits
what: viola wait and its --json output document — edits: the turn-end fields the --json document carries
@task_file n=6 path=tests/cli_wait_last.rs mark=edits
what: CLI tests of viola wait and last output — edits: the background_tasks assertions, joined by the count
@task_file n=7 path=tests/cli_wheel.rs mark=edits
what: CLI tests of the wheel and turn ends — edits: the background_tasks assertions, joined by the count
@task_file n=8 path=tests/hook_events.rs mark=edits
what: tests of the hook event calls and their event lines — edits: the turn-ended event assertions
@task_file n=9 path=crates/viola-mcp/tests/handlers.rs mark=edits
what: tests of the MCP server's tool handlers — edits: the MCP wait tool's turn-end assertions
@task_file n=10 path=crates/viola-state/tests/state_events.rs mark=edits
what: tests of the state crate's event line read and write — edits: the turn-ended event assertions
@task_file n=11 path=fixtures/claude/2.1.283/Stop.default.json mark=re-reads
what: a recorded Stop hook payload, read back for the session_crons shape
@task_file n=12 path=fixtures/claude/2.1.287/Stop.default.json mark=re-reads
what: a recorded Stop hook payload, read back for the session_crons shape
@task_file n=13 path=fixtures/claude/2.1.288/Stop.default.json mark=re-reads
what: a recorded Stop hook payload, read back for the session_crons shape
@task_file n=14 path=fixtures/claude/2.1.302/Stop.default.json mark=re-reads
what: a recorded Stop hook payload, read back for the session_crons shape
@task_item n=1 part=needs
The turn-ended event line in events.ndjson gains one number, the count of the session's scheduled wakes, beside background_tasks.
@task_item n=2 part=needs
The viola wait --json output document carries the same count on a turn end; the document stays one clean JSON document.
@task_item n=3 part=needs
The MCP wait tool's reply carries the same count on a turn end.
@task_item n=4 part=needs
A session with no scheduled wake reads none: an absent, null or non-array session_crons is mapped the way background_tasks's is today, the read failing open with no refusal.
@task_item n=5 part=needs
No content of a scheduled wake (its prompt, cron expression, id, schedule or any string) reaches an event line, snapshot, log line, run diagnostics file, reply or fixture: the elements are counted and never kept, at the single vendor-content boundary.
@task_item n=6 part=needs
The new field is named under viola-core's normalised naming: no Claude-specific name crosses into the event, the wait output or the MCP reply.
@task_item n=7 part=needs
Every reader of viola's own formats keeps skipping unknown fields, so an older reader of the event line or the --json document does not break on the added field.
@task_item n=8 part=needs
wait's wake rule stays as it is: no wake, dialog hold or completion decision reads the new count, and time bounds still fall on the driver only.
@task_item n=9 part=needs
P3 re-reads, from the code, a scheduled wake's prompt as the wrapper sees it: whether it moves the wheel to the human, whether a driver send is then refused while the human types, and whether the next dialog is held for the driver; it closes in one block, nothing built.
@task_item n=10 part=needs
The chunk's checks run on the fake agent and the recorded Stop payloads; no live claude reading gates, and a claim needing one gets its probe shaped and named at P3's end.
@task_item n=11 part=needs
Each new assertion pins the count's field name and value by literal inside the existing suites; a test waiting on a channel or thread takes a bound, a test socket lives under the short /tmp root.
@task_standing n=1
convention: new checks are assertions inside the existing test suites, with no separate a11y driver, suite or artifact
home: 1-a11y-scope-summary · bootstrap-phases-derive-for-route-setup-project
@task_standing n=2
convention: a failing check is a nextest failure that fails the build; no skips, no retries, zero flakiness
home: 10-slo-invariants-a11y-budgets
@task_standing n=3
convention: fixed output strings and field names are pinned by literal in tests
home: 8-cognitive-accessibility · cognitive-accessibility
@task_standing n=4
convention: a11y cases run inside the coverage suite of the GitHub Actions push/PR workflow
home: 9-ci-integration
@task_item n=1 part=not_needed
viola link and unlink, viola plugin install and their result lines and exit codes: the chunk changes no linking or plugin surface.
@task_item n=2 part=not_needed
viola revive refusals, revive --list and revive --json documents: the chunk adds no revive path or refusal.
@task_item n=3 part=not_needed
viola verify step lines and the interrupted verify pair: verify is not touched.
@task_item n=4 part=not_needed
viola --help group labels, usage line and served verbs: the chunk adds no verb or flag.
@task_item n=5 part=not_needed
Start refusals for strict modes, socket directory and server verify, on frame verbs, hook calls and MCP frame tools: no new refusal or precondition is added.
@task_item n=6 part=not_needed
Screen-reader targets, manual Orca and keyboard passes and their test setups: the chunk changes no read-aloud surface.
@task_item n=7 part=not_needed
Absences of a web or graphical frontend, a touch surface, internationalization and a rendered page: none is reached.
@task_item n=8 part=not_needed
Structured violation JSON rows for CI reporting: the chunk adds no CI job or report.
