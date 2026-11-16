@task_prose part=chunk
Reads the Stop hook payload's `session_crons` in the Claude adapter as a COUNT only, its elements never kept or typed, and carries it as one number beside `background_tasks` on the normalised `turn-ended` event.
The count reaches every hop `background_tasks` reaches today: the event line in events.ndjson, CLI `wait --json`, the MCP `wait` tool, and any further hop P3 measures (e.g. `last` or `list`).
A session with no scheduled wake reads none, mapped as `background_tasks`'s absent / null / non-array shape is today; `wait`'s wake rule does not change.
Re-measures every hop `background_tasks` takes and the existing assertions of it; re-reads the recorded Stop payloads for the field's shape.
Places, without building, one block on the directive's adjacent fact: whether the count lets a scheduled wake's prompt be told from a typed one.
@task_file n=1 path=crates/viola-agent-claude/src/hook.rs mark=edits
what: the Claude adapter's hook payload parsing, normalising Claude hook stdin into viola-core events — edits: the Stop payload read of `background_tasks`, gaining the `session_crons` count
@task_file n=2 path=crates/viola-core/src/lib.rs mark=edits
what: viola-core's normalised event kinds and their data, the agent-neutral boundary no Claude-specific name crosses — edits: the `turn-ended` event's data, gaining the scheduled-wake count beside `background_tasks`
@task_file n=3 path=src/cmd/run.rs mark=edits
what: viola run, the wrapper process that records hook events into events.ndjson and carries `background_tasks` onward — edits: the hop carrying `background_tasks`
@task_file n=4 path=src/run/send/slot.rs mark=edits
what: the wrapper's turn slot, holding the turn-end state a wait or send reads — edits: the hop carrying `background_tasks`
@task_file n=5 path=src/run/wait.rs mark=edits
what: the wrapper's wait handling, the wake rule and the turn-end reply wait returns to CLI and MCP — edits: the reply carrying `background_tasks`; the wake rule itself unchanged
@task_file n=6 path=tests/cli_wait_last.rs mark=edits
what: integration tests of CLI `wait` and `last` output against the fake agent, asserting `background_tasks`
@task_file n=7 path=tests/cli_wheel.rs mark=edits
what: integration tests of the wheel and turn ends against the fake agent, asserting `background_tasks`
@task_file n=8 path=tests/hook_events.rs mark=edits
what: integration tests of hook payloads normalised into events.ndjson lines, asserting `background_tasks`
@task_file n=9 path=crates/viola-mcp/tests/handlers.rs mark=edits
what: tests of the viola-mcp stdio server tool handlers, the `wait` tool's result asserting `background_tasks`
@task_file n=10 path=crates/viola-state/tests/state_events.rs mark=edits
what: tests of viola-state's event log append, replay and snapshot, asserting `background_tasks`
@task_item n=1 part=needs
The Stop hook stdin parse in the Claude adapter: `session_crons` read as a count only, each element skipped unread as `background_tasks`'s elements are, an absent, null or non-array value read as `background_tasks`'s is; no wake prompt, cron expression, id or schedule string is ever held.
@task_item n=2 part=needs
The normalised `turn-ended` event in viola-core gains one numeric field beside `background_tasks`, named by the plan under viola-core's naming rule; no Claude-specific name crosses, and viola-core still emits no telemetry of its own.
@task_item n=3 part=needs
The events.ndjson event line: the `turn-ended` line carries the count as an additive field; the event stream's line shape stays readable by an older reader that skips unknown fields (mixed binary versions), and a snapshot rebuilt by replay keeps working.
@task_item n=4 part=needs
The null versus absent versus zero reading of the count follows `background_tasks`'s existing rule, including null written as key absence where that rule applies; which one is closed at P3.
@task_item n=5 part=needs
CLI `wait --json` output and the viola-mcp `wait` tool result carry the count as they carry `background_tasks`, and so does any further hop P3 measures (`last`, `list`); MCP results stay codes-only on rejection.
@task_item n=6 part=needs
`wait`'s wake rule is unchanged: no wake, hold or completion decision reads the new count; the wait log line's outcome and fields and the wait span attributes gain nothing.
@task_item n=7 part=needs
No scheduled wake's content reaches an event, snapshot, process log line, detail file, reply or fixture: the NEVER-log floor, default-deny log field allow-list and type-level redaction bind the parsed payload type; event payloads stay out of process logs.
@task_item n=8 part=needs
The hook's own diagnostic lines (hook-invoked, hook-decision) gain no wake field and no count; the hook still fails open, writes no stderr, keeps its stdout to the decision body and stays inside its latency budget.
@task_item n=9 part=needs
Parse failures of the Stop payload's new field fall under the hook-stdin parse rules: a malformed value surfaces as unknown or none, never a fault, as `background_tasks`'s does, with any parse-rejected line carrying only its closed detail word.
@task_item n=10 part=needs
`session_crons` is an undocumented field of the wrapped claude CLI: wrapped claude internals are not a contract, and whether the count needs a capability-ledger row with a verify probe or reads as fail-to-null telemetry follows how `background_tasks` stands, read at P3.
@task_item n=11 part=needs
The recorded Claude Stop fixtures are read as the measured shape of `session_crons` across recorded versions; recorded fixtures are no telemetry source and any fixture touched or added is scrub-required, holding no real wake content.
@task_item n=12 part=needs
Proofs run on the fake agent and recorded payloads only: no live claude start, no real-CLI verify in CI; a claim needing a live reading gets its probe shaped and named at P3's end.
@task_item n=13 part=needs
New and changed tests: a test waiting on a channel or thread takes a bound, a test socket lives under the short /tmp root, a stamped home comes from the shared stamp's copy; each control is filtered to its own cases with a named timeout, and no mutation run.
@task_item n=14 part=needs
The directive's adjacent fact is read, not built: a CLI-scheduled wake's prompt logged origin human, the wheel going to human, driver sends refused human-typing, and the next dialog not held for the driver; P3 reads from the code whether the count tells a wake's prompt from a typed one and places one block with the options.
@task_standing n=1
convention: every new or changed integration or E2E test's emitted diagnostic lines are schema-checked, and the schema conformance gate runs on real process output
home: ci-integration · 9-ci-integration
@task_standing n=2
convention: integration-test homes live under the one scanned root and are kept until the gate steps run, gates running even after failed tests
home: 9-ci-integration · ci-integration
@task_standing n=3
convention: zero panic lines across test homes; any panic line fails the build
home: 10-slo-invariants-telemetry-budgets · slo-invariants-telemetry-budgets
@task_standing n=4
convention: tests wait on log or event lines without sleep-based waits and retry no failure
home: 10-slo-invariants-telemetry-budgets
@task_standing n=5
convention: uploads of test diagnostics pass the secret scan first, from synthetic inputs only
home: 8-pii-scrubbing-compliance · pii-scrubbing-compliance
@task_item n=1 part=not_needed
Span declaration, span kinds and the span chains and attributes of run start, confirmed send, dialog, wheel and verify paths: the chunk declares no span and adds no attribute.
@task_item n=2 part=not_needed
Panic capture, panic outcomes by role, the panic line and the detail-file error line: the chunk adds no catch site and no error path.
@task_item n=3 part=not_needed
Exit-1, exit-21 and process-exit detail codes, socket-directory, strict-modes and server-verify refusals and their stderr pairs: no refusal or exit cause changes.
@task_item n=4 part=not_needed
The viola-channel JSON-RPC errors, link and unlink methods and events, pause and release, and channel conn/corr join keys: the count rides no channel parameter of its own.
@task_item n=5 part=not_needed
viola verify, plugin install, revive and its preflight, the claude agents probe and its parse rows: none of those verbs is touched.
@task_item n=6 part=not_needed
Service identity, config.json keys, log level selection, log file location, rotation and the logging stack's init order: no logging setup changes.
@task_item n=7 part=not_needed
Heartbeat ticks, list liveness and the stale threshold, and agent-run status: liveness is not read by the count.
@task_item n=8 part=not_needed
The future error reporter, OTel export, network egress, hosted deployment and other absences: nothing here adds an exporter or a receiver.
@task_item n=9 part=not_needed
MCP stdin framing, MAX_FRAME and oversize-line handling: the chunk adds a result field, not a framing rule.
@task_item n=10 part=not_needed
Send validation refusals (empty text, control character) and the confirm window close rules for ordinary sends, /clear and /compact: no send path changes.
@task_item n=11 part=not_needed
Mutation-test outputs and fuzz artifact rules: the directive allows no mutation run and the chunk adds no fuzz target.
