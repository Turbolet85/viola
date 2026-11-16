@task_prose part=chunk
The Claude adapter reads the Stop hook payload's session_crons as a count only and carries it as one number beside background_tasks on the normalised turn-ended event.
That count reaches every hop background_tasks reaches today: the turn-ended event data in the event log, CLI wait --json, MCP wait, and any further hop P3 measures (last, list).
A session with no scheduled wake reads none, mapped the way background_tasks maps an absent, null or non-array field; no wake's content crosses out of the adapter.
wait's wake rule does not change; P3 re-measures every hop background_tasks takes and re-reads the recorded Stop payload fixtures for session_crons's shape.
P3 also reads from the code whether the count lets a wake's prompt be told from a typed one, and closes with one placement block; it builds nothing for that.
@task_file n=1 path=crates/viola-agent-claude/src/hook.rs mark=edits
what: the Claude adapter's hook payload parsing, which reads the Stop payload and counts background_tasks into the normalised turn end — edits: the Stop payload read, adding the session_crons count
@task_file n=2 path=crates/viola-core/src/lib.rs mark=edits
what: the normalised event kinds of viola-core and the turn-ended event data — edits: the turn-ended event data, adding the scheduled-wake count beside background_tasks
@task_file n=3 path=src/cmd/run.rs mark=edits
what: the viola run wrapper command that turns a hook's normalised turn end into the logged turn-ended event — edits: the turn-ended event data it writes
@task_file n=4 path=src/run/send/slot.rs mark=edits
what: the wrapper's send slot that carries the turn end's background_tasks — edits: the carried turn-end data, adding the count
@task_file n=5 path=src/run/wait.rs mark=edits
what: the wrapper channel's wait method and its wake on turn-ended — edits: the turn-ended wake's returned event data, adding the count; the wake rule itself unchanged
@task_file n=6 path=tests/cli_wait_last.rs mark=edits
what: CLI tests of viola wait and viola last over the fake agent, asserting the turn-ended wake and its background_tasks — edits: the turn-ended data assertions
@task_file n=7 path=tests/cli_wheel.rs mark=edits
what: CLI tests of the wheel, human input and driver sends over the fake agent that assert background_tasks on a turn end — edits: the turn-ended data assertions
@task_file n=8 path=tests/hook_events.rs mark=edits
what: viola hook tests fed recorded Stop stdin, asserting the logged turn-ended event and its background_tasks — edits: the Stop payload cases
@task_file n=9 path=crates/viola-mcp/tests/handlers.rs mark=edits
what: MCP tool handler tests, including the MCP wait result and its turn-ended data — edits: the MCP wait result assertions
@task_file n=10 path=crates/viola-state/tests/state_events.rs mark=edits
what: event log reader tests over turn-ended lines and their data keys — edits: the turn-ended event data cases
@task_file n=11 path=fixtures/claude/2.1.283/Stop.default.json mark=re-reads
what: the recorded Stop hook payload for CLI 2.1.283, read back for the shape of session_crons
@task_file n=12 path=fixtures/claude/2.1.287/Stop.default.json mark=re-reads
what: the recorded Stop hook payload for CLI 2.1.287, read back for the shape of session_crons
@task_file n=13 path=fixtures/claude/2.1.288/Stop.default.json mark=re-reads
what: the recorded Stop hook payload for CLI 2.1.288, read back for the shape of session_crons
@task_file n=14 path=fixtures/claude/2.1.302/Stop.default.json mark=re-reads
what: the recorded Stop hook payload for CLI 2.1.302, read back for the shape of session_crons
@task_item n=1 part=needs
The turn-ended event data gains one scheduled-wake count beside background_tasks: the event data keys of turn-ended, the turn-ended background_tasks contract, and the Stop event data's null-details rule all apply to the new key.
@task_item n=2 part=needs
The Stop hook payload read in the adapter: session_crons counted with its elements ignored, never kept or typed; unknown hook payload fields stay tolerated and the hook still fails open if the field is malformed.
@task_item n=3 part=needs
A session with no scheduled wake reads none: an absent, null or non-array session_crons maps to the same no-count reading background_tasks has today (null or 0 as that rule says).
@task_item n=4 part=needs
No wake's content (prompt, cron expression, id, schedule, any string) reaches an event, snapshot, log line, diagnostics file, reply or fixture: the no-secrets-in-logs, logs-carry-no-secrets and fixtures-free-of-real-data rules bind the new code and its tests.
@task_item n=5 part=needs
The count reaches CLI wait --json and MCP wait, and last or list where background_tasks reaches them: CLI and MCP return identical payloads, and the CLI --json one-document rule holds for the carried count.
@task_item n=6 part=needs
wait's wake rule is unchanged: the wake kinds, the turn-ended wake, the non-waking activity and wheel records, the timeout and the after-cursor reading stay as they are, and no wake, hold or completion decision reads the count.
@task_item n=7 part=needs
The new data is added only on the normalised event in viola-core under its naming rule: no Claude-specific name crosses out of the adapter.
@task_item n=8 part=needs
Every reader of viola's own formats keeps skipping unknown fields so an older binary reads the new key safely: forward-compatible event log reading, the tolerant event reader, forward-compatible lines skipped, unknown events counted and skipped.
@task_item n=9 part=needs
Whether the count is a dependence on an undocumented CLI behaviour needing a capability-ledger row with a verify probe (and so the version gate's relied rows), or read-only telemetry that fails to null: the plan follows how background_tasks stands on that rule.
@task_item n=10 part=needs
Tests run on the fake agent and the recorded payloads only: no live claude start, real claude never in CI, the fake agent replays recorded bytes, recorded hook payloads walked by one test, every CLI-version fixture set replayed, fixtures named by hook event and per CLI version.
@task_item n=11 part=needs
The Stop payload parser is a parser surface: property and fuzz coverage of parser surfaces and the hook stdin test layer bind the changed read, with bounded readers and no unbounded JSON depth.
@task_item n=12 part=needs
The event line keeps its format and keys with the new data key: events never truncated, event line round-trip, and test waits keyed on logged events.
@task_item n=13 part=needs
How the new tests are proved: no mutation run in this chunk; each control is a bounded remove-the-guard witness on an entry filtered to its own cases with a named timeout, holding when those cases ran and failed with no timeout line.
@task_item n=14 part=needs
Test bounds and homes: a test waiting on a channel or thread takes a bound, a test socket lives under the short /tmp root, a stamped home comes from the shared stamp's copy, and check-guarded cases start stamped.
@task_item n=15 part=needs
The adjacent fact is read, not built: a wake's submitted prompt is logged with origin human, the wheel goes to human, driver sends are then refused human-typing, and no dialog is held for the driver; the human-input-takes-the-wheel and prompt-submitted origin rules are read from the code to close P3's placement block, with no change to them.
@task_standing n=1
convention: Each new test function follows the plan's test function naming
home: 2-test-strategy
@task_standing n=2
convention: Each new root integration test binary is prefixed by the surface it tests, and cases go into the existing binary of that surface
home: 2-test-strategy
@task_standing n=3
convention: Each new unit case is inline in its module, table-driven in rstest where it varies by input
home: 2-test-strategy
@task_standing n=4
convention: Each new test creates its own private home and seeds its own data; on-disk product state is produced by viola, never hand-written
home: 7-test-data-fixtures · test-data-fixtures
@task_standing n=5
convention: Each new fixture or test input is synthetic text carrying the canary, never from real prompts or real sessions
home: 7-test-data-fixtures · test-data-fixtures
@task_standing n=6
convention: Each new assertion is a literal oracle on the contract, never on fixture internals or a Debug string
home: 11-test-anti-patterns-never-do-these
@task_standing n=7
convention: Each new test uses no sleep synchronisation and no retries, waiting on logged events under a bound
home: 11-test-anti-patterns-never-do-these
@task_item n=1 part=not_needed
Dialog hook decision bodies, dialog hold expiry, pending-dialog and question event option details: the Stop hook writes no decision body and the chunk changes no dialog.
@task_item n=2 part=not_needed
Revive verbs: revive exit codes, refusal codes, preflight order and revive human and JSON output; the chunk touches no revive path.
@task_item n=3 part=not_needed
Run and revive start refusals and start events: already-live, squatted name, stale heartbeat, pinned-exe hash, plugin rewrite and the claude-child environment strip.
@task_item n=4 part=not_needed
Link and unlink methods of the wrapper channel, link records and implicit link on send: no link behaviour changes.
@task_item n=5 part=not_needed
Send confirmation windows, paste framing, local-command and /clear or /compact confirmation, and the send ok document: the chunk adds nothing to a send.
@task_item n=6 part=not_needed
PTY passthrough, outer PTY rendering, vt100 screen feed and screen signatures: the count is read from a hook payload, never the screen.
@task_item n=7 part=not_needed
Heartbeat staleness classification and instance liveness: the chunk changes no heartbeat or liveness reading.
@task_item n=8 part=not_needed
The test harness command contracts for boot, cleanup, session records, pre-push stages and the local-live suite: the chunk adds no harness command or field.
@task_item n=9 part=not_needed
Release verb and pause rules: the chunk changes no wheel hand-back.
