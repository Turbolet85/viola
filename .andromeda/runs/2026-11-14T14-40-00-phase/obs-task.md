@task_prose part=chunk
Removes the budget gate from viola whole: viola reads no rate limits, writes no budget file, and refuses no send on budget.
Leaving: the wrapper's gate and its re-evaluation, the budget-gate event, the budget-paused refusal, the release budget option, the config thresholds, the snapshot and board budget state, and the list budget field.
Also leaving: the status-line arm's rate-limits reading and budget file write, the statusline-rate-limits ledger row, its fixtures, the Path 6 suite and every budget test case.
Exit code 11 is retired and never reused. The driver release guard (release-from-driver) and the plain release stay.
Re-measures that old event logs and snapshots still holding budget lines or fields keep reading after the removal, and what a config still carrying a threshold key reads.
@task_file n=1 path=src/run/budget.rs
what: the wrapper's budget governor — the gate, the threshold reading, the decision, the release budget lift and the run.budget_eval span over start, send and turn-ended; deleted whole
@task_file n=2 path=src/run/send/budget.rs
what: the send path's budget-paused check; deleted whole
@task_file n=3 path=src/run/send.rs mark=edits
what: the wrapper's confirmed send and its refusal order — edits: the budget-paused rung of the refusal order
@task_file n=4 path=src/run/send/slot.rs mark=edits
what: the wrapper's send slot and refusal handling — edits: the budget-paused rung
@task_file n=5 path=src/run/mod.rs mark=edits
what: the viola run start order and its start records in the events log — edits: the budget step of the start order, budget_paused and budget_override_until in first()
@task_file n=6 path=src/run/wheel.rs mark=edits
what: the wheel (pause and release methods, wheel transitions) — edits: its budget references
@task_file n=7 path=src/run/wait.rs mark=edits
what: the wrapper's wait dispatch and turn-ended handling — edits: its budget references and the turn-ended budget re-evaluation
@task_file n=8 path=src/cmd/run.rs mark=edits
what: the viola run verb — edits: its budget references
@task_file n=9 path=src/cmd/revive.rs mark=edits
what: the viola revive start verb that logs as run — edits: its budget references
@task_file n=10 path=crates/viola-state/src/budget.rs
what: the state crate's budget file reader and writer (write_budget, read_budget, read_budget_stamped, the state.budget_write span); deleted whole
@task_file n=11 path=crates/viola-state/src/lib.rs mark=edits
what: the state crate's module list — edits: the budget module
@task_file n=12 path=crates/viola-state/src/snapshot.rs mark=edits
what: the instance snapshot.json — edits: the budget_paused and budget_override_until fields
@task_file n=13 path=crates/viola-state/src/replay.rs mark=edits
what: the snapshot rebuilt by replay over the events log — edits: the recovery of budget_paused and budget_override_until
@task_file n=14 path=crates/viola-state/src/board.rs mark=edits
what: the board read model behind viola list — edits: the board item's budget_paused and the board's budget reading
@task_file n=15 path=crates/viola-core/src/lib.rs mark=edits
what: viola-core's closed types (the event enum, the refusal reason words) — edits: the budget-gate event kind, the budget-paused refusal reason, BudgetWindowKind, Percent, BudgetWindow, BudgetReading and their tests
@task_file n=16 path=src/cmd/release.rs mark=edits
what: the viola release verb and the channel release method's params — edits: the budget option and param, the human lines for a lifted and an open budget
@task_file n=17 path=src/cmd/send.rs mark=edits
what: the viola send verb's exit codes — edits: the budget-paused refusal's exit 11 mapping
@task_file n=18 path=src/cmd/mod.rs mark=edits
what: the CLI verb table and help text — edits: the list verb's help text naming the budget
@task_file n=19 path=crates/viola-mcp/src/result.rs mark=edits
what: the viola-mcp tool results — edits: the list payload's budget field
@task_file n=20 path=crates/viola-mcp/src/tools.rs mark=edits
what: the viola-mcp stdio server tools — edits: the list tool description naming the budget
@task_file n=21 path=src/human/board.rs mark=edits
what: the human board rendering of viola list — edits: the budget header
@task_file n=22 path=src/human/board/cells.rs mark=edits
what: the board's cells — edits: the budget cells
@task_file n=23 path=src/human/hints.rs mark=edits
what: the human hint lines on stderr — edits: the budget hints
@task_file n=24 path=crates/viola-agent-claude/src/statusline.rs mark=edits
what: the claude status-line stdin reader — edits: the rate_limits parse (the reading)
@task_file n=25 path=src/cmd/hook/statusline.rs mark=edits
what: the viola hook statusline arm — edits: its budget file write and the hook-decision budget_written field
@task_file n=26 path=src/cmd/hook.rs mark=edits
what: the viola hook dispatch and its hook-decision line — edits: the budget_written field
@task_file n=27 path=crates/viola-agent-claude/src/ledger.rs mark=edits
what: the claude capability ledger rows and their recorded fixture names — edits: the statusline-rate-limits row, its whole_rate_limits check and the STATUSLINE_RATE_LIMITS fixture name
@task_file n=28 path=src/bin/viola-fake-agent.rs mark=edits
what: the fake agent that replays recorded claude behaviour in CI — edits: the rate-limits replay after the first Stop
@task_file n=29 path=fixtures/claude/2.1.302/StatusLine.rate-limits-1.json
what: a recorded claude status-line fixture carrying rate limits; deleted
@task_file n=30 path=fixtures/claude/2.1.287/StatusLine.rate-limits-1.json
what: a recorded claude status-line fixture carrying rate limits; deleted
@task_file n=31 path=schemas/diag-line.v1.json mark=edits
what: the diag-line v1 schema of every diagnostics line — edits: its budget fields
@task_file n=32 path=schemas/claude-statusline-fixture.v1.json mark=edits
what: the schema of the recorded status-line fixtures — edits: its rate-limits shape
@task_file n=33 path=fixtures/claude/2.1.302/Mcp.tools-list-1.json mark=edits
what: a recorded MCP tools-list fixture — edits: the list tool description naming the budget
@task_file n=34 path=tests/cli_budget.rs
what: the budget gate's CLI tests; deleted whole
@task_file n=35 path=crates/viola-e2e/tests/path6_budget_harness.rs
what: the Path 6 budget E2E suite; deleted whole
@task_file n=36 path=tests/hook_statusline.rs mark=edits
what: the viola hook statusline tests — edits: its budget cases
@task_file n=37 path=tests/cli_list.rs mark=edits
what: the viola list tests — edits: its budget cases
@task_file n=38 path=crates/viola-channel/src/client.rs mark=edits
what: the channel client and its screening of peer refusal words — edits: the tests using budget-paused as a sample refusal
@task_file n=39 path=crates/viola-e2e/src/harness/logs.rs mark=edits
what: the e2e harness logs command reading process logs — edits: its budget references
@task_file n=40 path=.config/nextest.toml mark=edits
what: the nextest test profile and suite list — edits: the Path 6 entry
@task_item n=1 part=needs
The budget-gate product event leaves: no budget-gate line is written to the events log, the budget-gate record leaves the run start events order, and the event kind leaves the closed event enum, so the diag schema and the event enum must still match.
@task_item n=2 part=needs
The budget-paused refusal leaves the send refusal order: it leaves the wrapper send-refused detail words and the refusal reason words. The other closed refusal words, and the unknown fallback for a refusal word the client does not know, stay.
@task_item n=3 part=needs
Exit code 11 is retired and never renumbered or reused: no other refusal takes 11, and the remaining CLI exit codes (1, 2, 20, 21) and their detail codes keep their numbers.
@task_item n=4 part=needs
The release budget option leaves the viola release verb and the channel release method. The release-from-driver refusal stays: a string from is refused -32602 release-from-driver, a from of another type -32602 with null data. Its diag line event, its corr and the release refusal lines stay as they are.
@task_item n=5 part=needs
The budget spans and log lines leave: the run.budget_eval and state.budget_write span attributes, the budget governor span chain, the budget path log events, the budget log line window field and the rule that budget metadata is loggable as fields.
@task_item n=6 part=needs
The config.json five_hour and seven_day keys leave the known keys, with their parse-rejected config-json lines. The v and dialog_hold_s keys stay. What a config still carrying either key reads (ignored, or a parse-rejected unknown-keys line) is measured at P3.
@task_item n=7 part=needs
The diag-line v1 schema loses its budget fields, and the hook-decision line loses budget_written. This is a removal the schema's additive-only rule must allow: schema words no build can emit are removed, and default-deny unevaluatedProperties must still reject what no build emits.
@task_item n=8 part=needs
The hook statusline arm keeps only its pass-through: no rate-limits reading, no budget file write. Its hook-invoked, hook-decision, parse-rejected and statusline-shell lines and its statusline decision detail codes stay unchanged until the next entry.
@task_item n=9 part=needs
The list payload loses its budget field in both list --json and the viola-mcp list tool result, and the MCP list tool description stops naming the budget. The other list fields (liveness, agent-run state) are unchanged.
@task_item n=10 part=needs
The snapshot loses budget_paused and budget_override_until, and replay stops recovering them. The rule that a bad budget file reads as no reading leaves with the file. A budget.json left in a home is read by nothing.
@task_item n=11 part=needs
Readers of old disk state keep reading: an event log holding budget-gate lines and a snapshot holding the removed fields are read by skipping and counting the unknown kinds and fields (skipped counters, parse failures as unknown plus a counter, state-recovered lines), never by deny_unknown_fields. Measured at P3 on replay and the board.
@task_item n=12 part=needs
The board and human output lose the budget header, cells, hints and verb lines. Every remaining stderr line stays a fixed string, and control characters stay escaped in terminal output.
@task_item n=13 part=needs
The statusline-rate-limits capability ledger row leaves, so the relied row count goes from 24 to 23, and its recorded StatusLine.rate-limits fixtures leave with it. P3 settles whether the row leaves here or with the next entry; the statusline-override and statusline-shell rows and StatusLine.default-1.json stay.
@task_item n=14 part=needs
The fake agent stops replaying rate limits after the first Stop and keeps its status-line modes. CI still verifies against the fake agent only.
@task_item n=15 part=needs
The tests: the budget CLI suite, the Path 6 suite and its nextest entry leave, and so do the budget cases in the other tests and in the e2e harness's logs, status, run and perf readers. A test that used budget-paused only as a sample refusal word keeps its case with another closed refusal word.
@task_item n=16 part=needs
A guard test's remove-the-guard control is a one-off through the gate's control under a kill deadline, run on the test's name. No mutation run, local or dispatched.
@task_item n=17 part=needs
Every diagnostics line the remaining tests emit still passes the schema conformance gate against the narrowed diag-line schema, and every event value stays inside the closed enum.
@task_item n=18 part=needs
Reading<T> stays only if a non-budget user remains; P3 measures its callers.
@task_standing n=1
convention: every line a new or changed integration or E2E test emits is schema-checked against the diag-line schema, and its test home lives under the one scanned e2e root, kept until the gates run
home: ci-integration · 9-ci-integration
@task_standing n=2
convention: a new test waits on a log line by polling for it, never by sleeping, and asserts its verdict by machine
home: 10-slo-invariants-telemetry-budgets
@task_standing n=3
convention: a recorded claude fixture the chunk adds or rewrites is scrubbed, and fixtures are never a telemetry source
home: 1-obs-scope-summary · obs-scope-summary
@task_item n=1 part=not_needed
Error reporting and its future reporter (Sentry, OTel bridge, ureq transport, opt-in rules). The chunk adds no reporter and changes no error capture.
@task_item n=2 part=not_needed
Panic capture design: the panic hook, catch sites, the panic line's keys and the panic outcome by role. The chunk touches no catch site.
@task_item n=3 part=not_needed
Heartbeat liveness: the heartbeat file, its tick and the stale threshold. The removal leaves liveness as it is.
@task_item n=4 part=not_needed
The PTY seam log events (spawn, paste, kill-fallback) and the hyprctl probe subject. The removal reaches neither seam.
@task_item n=5 part=not_needed
Link and unlink: their methods, their events in the events log and the implicit link on send and answer. Budget never reaches them.
@task_item n=6 part=not_needed
Socket-directory, strict-modes and server-verify refusal detail codes and their exit-1 and exit-21 documents. The removal changes no connect or home-tree check.
@task_item n=7 part=not_needed
Log file locations, owner-only modes, home resolution, service identity and the version constant. No log file or identity changes.
@task_item n=8 part=not_needed
Secret-scan hit reports and the CI artifact retention and upload rules. The chunk adds no artifact.
@task_item n=9 part=not_needed
The absences of a database, gRPC, a hosted deployment, network egress and a telemetry exporter. A removal adds none of them.
