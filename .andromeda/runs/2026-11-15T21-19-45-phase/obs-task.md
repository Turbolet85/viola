@task_prose part=chunk
Makes a turn the claude CLI ends on an API error end as a turn: a hook for the CLI signal that ends such a turn (a StopFailure-class hook is the hypothesis P3 re-measures) is registered and its payload read in the Claude agent crate.
The turn is written to events.ndjson as turn-ended carrying a cause code from a closed viola set homed neutrally in viola-core; the error text reaches no log line, event or snapshot.
After it the instance reads idle in list and the board, the wheel's turn-running state clears exactly as a Stop clears it, a wait on the turn end returns and the next send is accepted, with no pause and release.
The fake agent learns to emit the API-error end, and a test driven by recorded or synthetic payloads fails on the tree before the fix.
P3 re-reads the defect from the code (which state reads turn-running while the board reads idle) and re-measures the ending mechanism from the code and the recorded fixtures.
@task_file n=1 path=plugin/hooks/hooks.json mark=edits
what: the plugin's hook registration list, the claude hook events viola hook is wired to — edits: the registration for the CLI signal that ends a turn on an API error
@task_file n=2 path=src/run/send/slot.rs mark=edits
what: the run pump's send slot, the wheel's turn-running state a send is refused against with turn-running — edits: the turn-end path
@task_file n=3 path=src/run/wait.rs mark=edits
what: the run's wait dispatch, which returns a waiter on a turn end — edits: the turn-end path
@task_item n=1 part=needs
A new hook event registered for viola hook: the hook's stdout carries only its decision body, it never writes stderr, keeps exit 0 and fails open, and is a silent no-op without a trusted instance, like every hook event.
@task_item n=2 part=needs
The hook-invoked and hook-decision process log lines for the new hook event: their fields, corr by hook fields, and a closed event value on each line.
@task_item n=3 part=needs
The new payload's read in the Claude agent crate: the CLI's error string maps into the closed cause set, a value outside it reads a fixed fallback member, never the CLI's string; a parse failure surfaces as the fallback, not as text.
@task_item n=4 part=needs
The cause set itself: a closed set of code words homed in viola-core with no Claude-specific name, every member named; viola-core emits no telemetry of its own.
@task_item n=5 part=needs
The turn-ended event in events.ndjson gains a cause key: the event line shape and its data keys change additively, readers skip and count unknown keys, and event emission stays deterministic.
@task_item n=6 part=needs
The NEVER-log floor over the error text: last_assistant_message and error_details on this path reach no events.ndjson line, no snapshot and no home-level process log line; no assistant message or result body in logs.
@task_item n=7 part=needs
The wheel's turn-end transition after an API-error end: turn-running cleared as a Stop clears it, the wheel product event as the audit record, and the wheel transition span and its attributes.
@task_item n=8 part=needs
The send path after that turn end: the wrapper's send-refused detail turn-running no longer fires on the next send, which is issued and confirmed with its usual send log lines.
@task_item n=9 part=needs
The wait path on a turn end: a waiter returns, with its wait and last log fields, channel-response outcome and wait span attributes.
@task_item n=10 part=needs
The instance's state as list and the board read it: idle after the API-error turn end, the list liveness and agent-run status state readings unchanged in kind.
@task_item n=11 part=needs
The fake agent's ability to emit an API-error turn end, and the recorded or synthetic payloads that drive it: recorded claude fixtures are scrub-required and are no telemetry source; CI verifies against the fake agent only, a real-CLI verify never runs in CI.
@task_item n=12 part=needs
The new test fails on the tree before the fix; each new guard test carries a one-off remove-the-guard control named by the test's name alone, under a kill deadline; no mutation run of any kind.
@task_item n=13 part=needs
A dependence on undocumented claude behaviour (whether the CLI's API-error signal fires) is a capability-ledger row with a viola verify probe; verify log lines carry no content, and wrapped claude internals and transcripts are not a contract.
@task_item n=14 part=needs
Test sockets under the short tmp root, and any home given to record through its one home helper.
@task_item n=15 part=needs
Metrics derived from the logs grouped by closed enums only: the cause code is a closed value usable for that grouping, never free text.
@task_standing n=1
convention: each new log line is written through the one obs_event macro, with a closed event value, explicit corr and instance, and only allow-listed fields
home: 11-obs-anti-patterns-never-do-these · logging-stack
@task_standing n=2
convention: each new or changed log field is additive only, its name frozen once added
home: 6-log-coverage
@task_standing n=3
convention: each new integration or E2E test has its emitted lines schema-checked, its home under the one scanned test root, kept until the gate steps run
home: ci-integration · 9-ci-integration
@task_standing n=4
convention: each new test waits on a log line or event without sleep-based waits, and adds no retry
home: 10-slo-invariants-telemetry-budgets
@task_standing n=5
convention: each new fixture or recorded payload is scrubbed and synthetic where it can be uploaded, never a telemetry source
home: pii-scrubbing-compliance · 8-pii-scrubbing-compliance
@task_standing n=6
convention: each new detail or cause value is a closed code word carrying no path, pid or text
home: 4-span-trace-coverage · log-coverage
@task_item n=1 part=not_needed
Panic capture and reporting by role, the panic line and its catch sites, and the future error-reporter decisions: the chunk changes no panic path and adds no reporter.
@task_item n=2 part=not_needed
The socket-directory, strict-modes and server-verification refusal codes and their exit 1 and exit 21 lines: the chunk changes no refusal before a connection.
@task_item n=3 part=not_needed
Dialog raise, answer, expiry and their dialog hook lines: an API-error turn end raises no dialog.
@task_item n=4 part=not_needed
The run start sequence: version gate, pinned copy, collision check, spawn span attributes, env strip and the claude child's process-start line.
@task_item n=5 part=not_needed
viola revive, link and unlink and their process-log contracts: the chunk touches no session revival or linking.
@task_item n=6 part=not_needed
The send confirmation window and the readiness gate: send-confirmation whitespace and the readiness-gate race are another chunk's scope.
@task_item n=7 part=not_needed
The wait wake condition for background-task questions: another chunk's scope.
@task_item n=8 part=not_needed
The service identity, config.json keys, home resolution order and diagnostics file paths and modes: the chunk adds no process role and no config key.
@task_item n=9 part=not_needed
OTel, exporters, network egress, trace ids and dashboards: absences the chunk does not revisit.
@task_item n=10 part=not_needed
The heartbeat file, its staleness threshold and liveness flip: an API-error turn end is a turn state, not a liveness change.
@task_item n=11 part=not_needed
The MCP server's stdin framing, oversize lines and its frame-tool refusal causes: the chunk changes no MCP framing.
@task_item n=12 part=not_needed
State recovery by replay, torn-line healing and the replayed snapshot's live fields: the chunk changes no recovery path.
