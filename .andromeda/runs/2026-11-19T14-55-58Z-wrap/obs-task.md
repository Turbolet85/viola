@task_prose part=session
The session landed a sixth recorded claude fixture set, for claude 2.1.328, as a byte copy of the live rig's kept recordings (28 files: hook payloads, screens, the agents probe output, the MCP tools list and a verify record).
The stamped recorded sets the ledger-probe contract suite walks went from 3 to 4, adding 2.1.328; the drift, recorded-exchange, hygiene and ledger-probe suites are green over every set.
The test-only live rig's context arm gained an after-send transcript lookup and a new closed rig summary word, context-transcript-after (read, absent or unread); a transcript first found after the send is counted from its start, tags only, no byte kept.
One live reading on 2.1.328, no verification claimed: the /context output reached the session transcript under the rig's private config as one local-command-stdout tag, and the send reply was not-delivered with no-prompt-submitted.
No product crate, product log line, diagnostics schema, config key, CI gate or dependency moved; the default CLI version and the stamped-home version are unchanged.
@task_file n=1 path=crates/viola-e2e/src/rig/mod.rs
what: the test-only live rig that starts the real claude CLI under a private config and prints closed summary words; its context arm reads the session transcript as a tag count, never keeping bytes
@task_file n=2 path=crates/viola-e2e/tests/rig_rehearsal.rs
what: the live rig's rehearsal tests over the fake agent, asserting the rig's summary words
@task_file n=3 path=tests/contract_ledger_probes.rs
what: the ledger-probe contract suite, holding the list of stamped recorded fixture sets it walks
@task_file n=4 path=fixtures/claude/2.1.328/Agents.recorded-1.json
what: a recorded claude fixture: the claude agents probe output, scrub-required test input, no telemetry source
@task_file n=5 path=fixtures/claude/2.1.328/Mcp.tools-list-1.json
what: a recorded claude fixture: the viola mcp stdio server's tools list exchange as the MCP tap recorded it
@task_file n=6 path=fixtures/claude/2.1.328/PermissionRequest.permission-1.json
what: a recorded claude fixture: a hook stdin payload for a permission dialog
@task_file n=7 path=fixtures/claude/2.1.328/PermissionRequest.plan-1.json
what: a recorded claude fixture: a hook stdin payload for a plan dialog
@task_file n=8 path=fixtures/claude/2.1.328/PostToolUse.parallel-1.json
what: a recorded claude fixture: a hook stdin payload after a tool use
@task_file n=9 path=fixtures/claude/2.1.328/PostToolUse.parallel-2.json
what: a recorded claude fixture: a hook stdin payload after a tool use
@task_file n=10 path=fixtures/claude/2.1.328/PostToolUse.permission-1.json
what: a recorded claude fixture: a hook stdin payload after a tool use
@task_file n=11 path=fixtures/claude/2.1.328/PostToolUse.plan-2.json
what: a recorded claude fixture: a hook stdin payload after a tool use
@task_file n=12 path=fixtures/claude/2.1.328/PostToolUse.questions-1.json
what: a recorded claude fixture: a hook stdin payload after a tool use
@task_file n=13 path=fixtures/claude/2.1.328/PreToolUse.parallel-1.json
what: a recorded claude fixture: a hook stdin payload before a tool use
@task_file n=14 path=fixtures/claude/2.1.328/PreToolUse.parallel-2.json
what: a recorded claude fixture: a hook stdin payload before a tool use
@task_file n=15 path=fixtures/claude/2.1.328/PreToolUse.plan-1.json
what: a recorded claude fixture: a hook stdin payload before a tool use
@task_file n=16 path=fixtures/claude/2.1.328/PreToolUse.plan-2.json
what: a recorded claude fixture: a hook stdin payload before a tool use
@task_file n=17 path=fixtures/claude/2.1.328/PreToolUse.questions-1.json
what: a recorded claude fixture: a hook stdin payload before a tool use
@task_file n=18 path=fixtures/claude/2.1.328/Screen.modal.json
what: a recorded claude fixture: the screen content of a modal, read by the screen parsers in tests
@task_file n=19 path=fixtures/claude/2.1.328/Screen.ready.json
what: a recorded claude fixture: the screen content of a ready prompt, read by the screen parsers in tests
@task_file n=20 path=fixtures/claude/2.1.328/Screen.turn.json
what: a recorded claude fixture: the screen content of a running turn, read by the screen parsers in tests
@task_file n=21 path=fixtures/claude/2.1.328/SessionEnd.clear-1.json
what: a recorded claude fixture: a session-end hook stdin payload
@task_file n=22 path=fixtures/claude/2.1.328/SessionEnd.default.json
what: a recorded claude fixture: a session-end hook stdin payload
@task_file n=23 path=fixtures/claude/2.1.328/SessionStart.clear-1.json
what: a recorded claude fixture: a session-start hook stdin payload
@task_file n=24 path=fixtures/claude/2.1.328/SessionStart.compact-1.json
what: a recorded claude fixture: a session-start hook stdin payload
@task_file n=25 path=fixtures/claude/2.1.328/SessionStart.default.json
what: a recorded claude fixture: a session-start hook stdin payload
@task_file n=26 path=fixtures/claude/2.1.328/SessionStart.resume-1.json
what: a recorded claude fixture: a session-start hook stdin payload on resume
@task_file n=27 path=fixtures/claude/2.1.328/Stop.default.json
what: a recorded claude fixture: a stop hook stdin payload
@task_file n=28 path=fixtures/claude/2.1.328/UserPromptSubmit.default.json
what: a recorded claude fixture: a prompt-submit hook stdin payload
@task_file n=29 path=fixtures/claude/2.1.328/UserPromptSubmit.paste-1.json
what: a recorded claude fixture: a prompt-submit hook stdin payload of a paste
@task_file n=30 path=fixtures/claude/2.1.328/UserPromptSubmit.paste-2.json
what: a recorded claude fixture: a prompt-submit hook stdin payload of a paste
@task_file n=31 path=fixtures/claude/2.1.328/UserPromptSubmit.paste-3.json
what: a recorded claude fixture: a prompt-submit hook stdin payload of a paste
@task_item n=1 part=touches
Recorded claude fixtures are scrub-required: a new set for 2.1.328 landed under that rule, passing the record scrub and the hygiene walk with no hand edit.
@task_item n=2 part=touches
Recorded fixtures are no telemetry source: the new set is test input only and feeds no log line, counter or derived metric.
@task_item n=3 part=touches
Wrapped claude internals and claude transcripts are not a contract: the session pins one more version's hook payloads and screens as fixtures and the live rig reads a claude session transcript, both as test evidence, not as a product contract.
@task_item n=4 part=touches
The transcript echo read stays uninstrumented: the rig's new after-send transcript lookup is test-only and writes no product log line; it counts tags under the MCP stdin MAX_FRAME bound and keeps no byte.
@task_item n=5 part=touches
CI verifies against the fake agent only and a real-CLI verify never runs in CI: the 2.1.328 recording and the context reading were local live rig starts; CI ran only the fixture suites over recorded sets.
@task_item n=6 part=touches
Local run outputs are never uploaded: the rig's kept recordings under the build target directory were copied into the tree as fixtures only after the scrub and hygiene walk; nothing was uploaded.
@task_item n=7 part=touches
Agents probe output untrusted and its parse failure read as unknown: a new recorded claude agents output for 2.1.328 is parsed by the tests green.
@task_item n=8 part=touches
Hook stdin parse rejection (parser hook-stdin, detail unknown-value) and the hook stdout decision body: the 2.1.328 hook payloads pass the drift suites with no new unknown value.
@task_item n=9 part=touches
The viola mcp stdio server tools (list, send, wait, last, answer) and MCP stdout carrying frames only: a new tools-list exchange recorded by the MCP tap for 2.1.328 now stands as a fixture.
@task_item n=10 part=touches
No telemetry from screen content: new recorded screens for 2.1.328 (modal, ready, turn) are test input to the screen parsers only.
@task_item n=11 part=touches
Wrapper send-refused detail words no-prompt-submitted and input-not-ready: the live reading's reply was not-delivered with no-prompt-submitted, as expected; a quoted modal literal in a transcript row was measured refusing a send as input-not-ready, carried as a proposal, not mended.
@task_item n=12 part=touches
Capability ledger stamps never logged and an unreadable ledger reading as unverified: the stamped recorded sets the ledger-probe suite walks went from 3 to 4, a test list only; no ledger path or stamp is logged.
@task_item n=13 part=touches
Secret scan scope and fixtures in the repository: 28 new files under the claude fixtures tree fall in the scanned tree; the pre-push gate and CI were green.
@task_item n=14 part=touches
Claude environment strip and stripped CLAUDE values never logged: the live rig ran claude 2.1.328 under its own private config; the user config was observed changed by others, never by the rig.
@task_item n=1 part=untouched
The process log line, diagnostics line and detail-file line schemas, their field names, event enum, corr and conn: no product log line or field was added or changed.
@task_item n=2 part=untouched
Exit codes and detail codes for exit 1, 20 and 21, process-exit, strict-modes and socket-directory refusals: no product exit path moved.
@task_item n=3 part=untouched
Span and trace coverage, span names, span attributes and span kinds of run, send, wait, wheel, dialog and hook: no span was declared or changed.
@task_item n=4 part=untouched
Panic capture, the panic line, panic outcome by role and the error-capture boundary: no product panic or error path changed.
@task_item n=5 part=untouched
CI gates G1 to G4, perf budgets, hook latency budgets, heartbeat staleness and the zero-panics and error budgets: no gate, threshold or budget moved.
@task_item n=6 part=untouched
The logging stack, subscriber, levels, log targets, file sinks, file modes and home resolution: no obs dependency or init order changed.
@task_item n=7 part=untouched
Telemetry exporters, OTel, error reporters, network egress and the absences of a database, gRPC or hosted deployment: nothing was added.
@task_item n=8 part=untouched
Service identity, the version constant and config.json keys: the default CLI version and the stamped-home version were not moved, and no config key was added.
