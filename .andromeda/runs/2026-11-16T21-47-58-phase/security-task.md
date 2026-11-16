@task_prose part=chunk
Reads the Stop hook payload's `session_crons` in the Claude adapter as a count only, never keeping or typing an element, and carries it as one number beside `background_tasks` on the normalised `turn-ended` event.
Carries that count through every hop `background_tasks` reaches today: event data in `events.ndjson`, CLI `wait --json`, MCP `wait`, and any further hop (`last`, `list`) that P3 measures.
A session with no scheduled wake reads none, mapped from an absent, null or non-array shape the same way `background_tasks` is; `wait`'s wake rule does not change.
P3 re-measures every `background_tasks` hop, re-reads the recorded Stop fixtures for the field's shape, and re-reads the human-typing wheel path to close one block on whether the count lets a wake's prompt be told from a typed one.
@task_file n=1 path=crates/viola-agent-claude/src/hook.rs mark=edits
what: the Claude adapter's hook payload parse, the tolerant reading of the Stop payload that keeps background tasks as a count only — edits: the Stop payload's field read (`session_crons` counted beside `background_tasks`) and its mapping to the normalised event
@task_file n=2 path=crates/viola-core/src/lib.rs mark=edits
what: the core crate's normalised event types, the agent-neutral `turn-ended` event and its data, under the core naming rule — edits: the `turn-ended` event's data (a new count field beside `background_tasks`)
@task_file n=3 path=src/cmd/run.rs mark=edits
what: the wrapper's run command, which receives hook events over the channel and appends them to the event log — edits: where `background_tasks` is carried onward
@task_file n=4 path=src/run/send/slot.rs mark=edits
what: the wrapper's send slot, the per-instance turn state the turn end lands in — edits: where `background_tasks` is held for a turn end
@task_file n=5 path=src/run/wait.rs mark=edits
what: the wrapper's wait method, the wake rule and the turn-end reply it gives to CLI and MCP `wait` — edits: the reply's turn-end fields (the count added; the wake rule untouched)
@task_file n=6 path=tests/cli_wait_last.rs mark=edits
what: the CLI `wait` and `last` integration tests over the fake agent, asserting the turn-end fields under `--json`
@task_file n=7 path=tests/cli_wheel.rs mark=edits
what: the CLI wheel tests, human typing taking the wheel and the driver's sends refused while the human holds it
@task_file n=8 path=tests/hook_events.rs mark=edits
what: the hook event tests, a recorded hook payload passed through `viola hook` into the event log as a normalised event
@task_file n=9 path=crates/viola-mcp/tests/handlers.rs mark=edits
what: the MCP tool handler tests, the `wait` tool's structured result
@task_file n=10 path=crates/viola-state/tests/state_events.rs mark=edits
what: the state crate's event log tests, writing and reading `events.ndjson` lines and their data
@task_item n=1 part=needs
The Stop hook payload is untrusted content: `session_crons` is parsed tolerantly and counted only. Each element is read as an ignored value, the way `background_tasks`'s elements are, and no prompt, cron expression, id, schedule or other string from a wake is kept, typed or passed out of the adapter.
@task_item n=2 part=needs
A missing, null, non-array or malformed `session_crons` fails to the same "none" reading that `background_tasks` uses today: the hook still fails open to the human, and a bad field never fails the whole payload or the hook.
@task_item n=3 part=needs
The new count is added only to the normalised `turn-ended` event in the core crate, under an agent-neutral name. No Claude-specific field name or class crosses out of the Claude adapter, matching how API-error classes stay inside the adapter.
@task_item n=4 part=needs
The `hook.event` channel frame carries the count from the hook to the wrapper. The wrapper re-validates that frame against its closed types, and an out-of-shape count is dropped, not trusted.
@task_item n=5 part=needs
`events.ndjson` `turn-ended` lines gain one numeric field. Every reader of viola's own formats keeps skipping unknown fields and tolerating torn lines, so an older binary reading the newer line, or a newer one reading an older line, still works.
@task_item n=6 part=needs
CLI `wait --json` and the MCP `wait` tool's structured result gain the count. The client still screens the wrapper's answer to closed values and code words, and the new field holds only a number.
@task_item n=7 part=needs
No wake content reaches any output: no event, snapshot, process or diagnostics log line, channel reply, MCP result, error message or fixture. This is the never-log floor and the rule that user content goes only to diagnostics, here held tighter: nothing at all.
@task_item n=8 part=needs
The wake rule of the wrapper's `wait` method stays as it is: no wake, hold or completion decision reads the new count.
@task_item n=9 part=needs
The hook stdin read stays bounded by the shared frame cap and JSON depth limit. Counting an array adds no unbounded read or allocation, and the count's integer type cannot overflow or panic under the zero-panics gate.
@task_item n=10 part=needs
Recorded Claude Stop payloads under `fixtures/claude/` are read for the field's shape across versions. Any fixture this chunk adds or re-records is scrubbed, reviewed and walked by fixture hygiene, and holds no real wake content.
@task_item n=11 part=needs
`session_crons` is an undocumented CLI field. Viola's reading of it must fail to "none" with no hard dependence, and it gets a capability-ledger row with a verify probe only if `background_tasks` stands that way. Upstream text in it is never a command.
@task_item n=12 part=needs
The human-typing wheel path is re-read, not changed. A CLI-scheduled wake's prompt is logged `origin: human`, the wheel goes to the human, driver sends are refused with `human-typing`, and the next dialog is not held. P3 reads whether the count lets a wake be told from typed input.
@task_item n=13 part=needs
No live `claude` start and no real CLI in CI: the proof runs on the fake agent and the recorded payloads, and any claim that needs a live reading gets its probe shaped without being run.
@task_item n=14 part=needs
Any fake-agent seam that emits a Stop payload with `session_crons` stays under the test-only fake-agent feature and is compiled out of release builds.
@task_standing n=1
convention: each new test uses its own viola home, puts its socket under the short temp root and never at the temp root itself, takes a bound on any wait on a channel or thread, and removes only directories it created
home: secret-management
@task_standing n=2
convention: each new or re-recorded fixture is scrubbed of paths, usernames and emails, reviewed before commit, and kept under the fixture hygiene walk
home: data-protection
@task_standing n=3
convention: each new test seam lives only under the fake-agent feature and is compiled out of release
home: security-anti-patterns
@task_item n=1 part=not_needed
Strict-modes, socket-directory and server-verification refusal contracts for each verb (CLI, hook, MCP, run, revive, list, verify): the chunk adds no refusal, verb or check order.
@task_item n=2 part=not_needed
`viola revive`: its arguments, its `--json` codes and its session list document. The count is not part of revive.
@task_item n=3 part=not_needed
`viola verify` probes, real-CLI rounds, stamps and verify refusal codes: no live claude run and no verify change happen here.
@task_item n=4 part=not_needed
Pinned binary and plugin placement, re-hashing, marketplace and plugin install rows: nothing here installs or pins.
@task_item n=5 part=not_needed
Paste and send text character sets, dialog free-text and answer behaviour: the chunk types no text and answers no dialog.
@task_item n=6 part=not_needed
Link and unlink methods, their event lines and exit codes: unrelated to turn-end data.
@task_item n=7 part=not_needed
Inherited credential stripping, CLAUDE environment variables and the child's PATH and environment: the child spawn does not change.
@task_item n=8 part=not_needed
Crate-specific version floors, advisory ignores and rmcp feature sets: the chunk adds no dependency.
@task_item n=9 part=not_needed
Workflow permissions, action pinning and release workflow and attestation rows: no workflow or release changes.
@task_item n=10 part=not_needed
Hyprland recording and reply-text rows: the chunk does not reach them.
