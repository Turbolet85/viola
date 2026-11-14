@task_prose part=chunk
A REMOVAL chunk: the budget gate leaves viola whole — no rate-limits reading, no budget.json, no send gate, no budget-paused refusal, no budget board or list field, no budget log or diagnostics field, no Path 6 suite.
It removes the budget-gate event kind, the budget-paused refusal word and the budget core types, the release --budget option, the config thresholds five_hour / seven_day, and the statusline-rate-limits ledger row with its recorded fixtures.
Exit code 11 is retired and never renumbered or reused; the driver release guard (release-from-driver) and the plain release stay.
The status-line pass-through, config.json's dialog_hold_s and the reading of old disk state that still holds budget lines and fields stay; P3 re-measures that old event logs and snapshots still read.
@task_file n=1 path=src/run/budget.rs
what: the wrapper's budget gate — thresholds read, pause decision, release --budget lift, budget evaluation over start, send and turn-ended; deleted whole
@task_file n=2 path=src/run/send/budget.rs
what: the send path's budget-paused check; deleted whole
@task_file n=3 path=src/run/send.rs mark=edits
what: the wrapper's send handler and its refusal order — edits: the budget-paused rung of the send refusal order
@task_file n=4 path=src/run/send/slot.rs mark=edits
what: the send slot of the wrapper's send handler — edits: its budget-paused references
@task_file n=5 path=src/run/mod.rs mark=edits
what: the wrapper's run loop and start order — edits: the budget step of the start order (the budget-gate start record), budget_paused and budget_override_until in first(), the turn-ended budget re-evaluation
@task_file n=6 path=src/run/wheel.rs mark=edits
what: the wheel holder of a wrapped instance — edits: its budget references
@task_file n=7 path=src/run/version_gate.rs mark=edits
what: the unverified CLI version gate over the relied ledger rows — edits: its budget references
@task_file n=8 path=src/cmd/run.rs mark=edits
what: the viola run verb — edits: its budget references (thresholds read at start)
@task_file n=9 path=crates/viola-state/src/budget.rs
what: the budget.json state file writer and readers (write_budget, read_budget, read_budget_stamped); deleted whole
@task_file n=10 path=crates/viola-state/src/lib.rs mark=edits
what: the state crate's module list — edits: the pub mod budget entry
@task_file n=11 path=crates/viola-state/src/snapshot.rs mark=edits
what: the instance snapshot — edits: the budget_paused and budget_override_until fields
@task_file n=12 path=crates/viola-state/src/replay.rs mark=edits
what: snapshot replay from the event log — edits: the recovery of budget_paused and budget_override_until
@task_file n=13 path=crates/viola-state/src/board.rs mark=edits
what: the list board's items read through the replay — edits: the item's budget_paused and the board's budget reading
@task_file n=14 path=crates/viola-core/src/lib.rs mark=edits
what: the core event kinds, refusal reasons and shared types — edits: EventKind BudgetGate (budget-gate), RefusalReason BudgetPaused (budget-paused), BudgetWindowKind, Percent, BudgetWindow, BudgetReading and their tests
@task_file n=15 path=src/cmd/release.rs mark=edits
what: the viola release verb and the channel release param — edits: the --budget option, the budget param, the human lines write_budget_lifted and write_budget_open
@task_file n=16 path=src/cmd/send.rs mark=edits
what: the viola send verb's refusal-to-exit-code mapping — edits: the budget-paused exit 11
@task_file n=17 path=crates/viola-mcp/src/result.rs mark=edits
what: the MCP tool results, the list payload mirroring list --json — edits: the list payload's budget field
@task_file n=18 path=crates/viola-mcp/src/tools.rs mark=edits
what: the MCP tools/list tool set and descriptions — edits: the list tool description naming the budget
@task_file n=19 path=src/cmd/mod.rs mark=edits
what: the CLI verb table and help — edits: the list verb's help text naming the budget
@task_file n=20 path=src/human/board.rs mark=edits
what: the viola list human board — edits: the budget header
@task_file n=21 path=src/human/board/cells.rs mark=edits
what: the human board's cells, the WHEEL cell among them — edits: the budget cells
@task_file n=22 path=src/human/board/bay.rs mark=edits
what: the human board's header bay — edits: the budget header
@task_file n=23 path=src/human/hints.rs mark=edits
what: the human hints for each exit cause — edits: the budget hints
@task_file n=24 path=src/human/verbs.rs mark=edits
what: the human verb lines — edits: the budget verb lines
@task_file n=25 path=crates/viola-agent-claude/src/statusline.rs mark=edits
what: the statusline payload parser — edits: the rate_limits parse (reading)
@task_file n=26 path=src/cmd/hook/statusline.rs mark=edits
what: the viola hook statusline arm — edits: its budget.json write, keeping only the pass-through
@task_file n=27 path=src/cmd/hook.rs mark=edits
what: the viola hook entry points and the hook-decision log line — edits: the hook-decision budget_written field
@task_file n=28 path=crates/viola-agent-claude/src/ledger.rs mark=edits
what: the ledger rows (LEDGER_ROWS) with their verify probes and record fixture names — edits: the statusline-rate-limits row, whole_rate_limits, the STATUSLINE_RATE_LIMITS record fixture name, the relied row count 24 to 23
@task_file n=29 path=fixtures/claude/2.1.287/StatusLine.rate-limits-1.json
what: a recorded statusline fixture carrying rate limits, CLI 2.1.287; deleted
@task_file n=30 path=fixtures/claude/2.1.302/StatusLine.rate-limits-1.json
what: a recorded statusline fixture carrying rate limits, CLI 2.1.302; deleted
@task_file n=31 path=src/bin/viola-fake-agent.rs mark=edits
what: the fake agent serving the claude boundary — edits: its rate-limits replay after the first Stop
@task_file n=32 path=tests/cli_budget.rs
what: the budget integration suite (send gate, release --budget, budget.json); deleted whole
@task_file n=33 path=crates/viola-e2e/tests/path6_budget_harness.rs
what: the Path 6 budget E2E suite; deleted whole
@task_file n=34 path=tests/hook_statusline.rs mark=edits
what: the viola hook statusline integration suite — edits: its budget cases
@task_file n=35 path=crates/viola-e2e/src/harness/status.rs mark=edits
what: the test harness status command and its polled fields — edits: its budget references (budget, budget_paused)
@task_file n=36 path=crates/viola-e2e/src/harness/boot.rs mark=edits
what: the test harness boot command — edits: its budget references
@task_file n=37 path=.config/nextest.toml mark=edits
what: the nextest profiles and test-group filtersets — edits: the Path 6 entry
@task_file n=38 path=schemas/diag-line.v1.json mark=edits
what: the diagnostics line schema — edits: its budget fields
@task_file n=39 path=schemas/claude-statusline-fixture.v1.json mark=edits
what: the tolerant schema recorded statusline fixtures are checked against — edits: its rate-limits shape
@task_file n=40 path=fixtures/claude/2.1.302/Mcp.tools-list-1.json mark=edits
what: the recorded MCP tools/list exchange the fake agent and MCP fixtures are compared to — edits: the list tool description naming the budget
@task_item n=1 part=needs
An automated send is never refused on budget: the budget-paused rung leaves the send refusal order; the remaining rungs (empty-text, human-typing, manual-pause, turn-running, input-not-ready) keep their order and words, over the CLI, the wrapper channel and MCP send.
@task_item n=2 part=needs
CLI exit code 11 is retired: no refusal maps to it, it is never renumbered or reused, and every other exit code (10, 12, 13, 14, 20, 21) keeps its number; the closed refusal-word set and the MCP refusal result lose budget-paused.
@task_item n=3 part=needs
The run start records lose the budget-gate record: the first records at viola run and revive start and their order change, and the event log no longer carries a budget-gate kind at start or after turn-ended.
@task_item n=4 part=needs
Old disk state keeps reading: an event log holding budget-gate lines and a snapshot holding budget_paused or budget_override_until are skipped and counted as unknown, never rejected, by the tolerant event reader, replay and the list board (measured at P3; no deny_unknown_fields).
@task_item n=5 part=needs
Snapshot replay recovers a smaller fixed field set: budget_paused and budget_override_until leave the snapshot and its replay; replay still writes no file.
@task_item n=6 part=needs
viola release loses --budget and its JSON document, and the channel release loses its budget param; the driver release guard stays (a string from refused -32602 release-from-driver, a from of another type -32602 with data null, its channel-response detail and log line), the plain release hands the wheel back, and release stays a human verb absent from driver hints.
@task_item n=7 part=needs
list --json and the MCP list tool lose the budget field and each item's budget_paused, keeping CLI and MCP payloads identical; the human board loses its budget header and budget cells and the WHEEL cell no longer shows budget-paused.
@task_item n=8 part=needs
The list verb's help text and the MCP list tool description stop naming the budget; the recorded MCP tools/list fixture follows (whether the MCP tap records without a live claude is measured at P3).
@task_item n=9 part=needs
viola hook statusline writes no budget.json and its hook-decision line carries no budget_written field; its pass-through output and shell-out stay as they are; a budget.json left in a home is read by nothing.
@task_item n=10 part=needs
The statusline-rate-limits ledger row retires with its verify probe and record fixture name: the relied row count goes 24 to 23, the version gate needs every remaining relied row, verify stamps cover the remaining rows; P3 settles whether the row leaves here or with the next entry.
@task_item n=11 part=needs
Recorded fixtures: the rate-limits statusline fixtures leave per CLI version, the fake agent stops replaying rate limits and stays byte-for-byte with the remaining recordings, the tolerant statusline fixture schema drops the rate-limits shape, and the fuzz statusline seed still carrying rate_limits is kept as content the hook still receives.
@task_item n=12 part=needs
config.json loses its five_hour and seven_day threshold keys and their parse-rejected config-json lines; dialog_hold_s stays read once at start with its default fallback; what a config still carrying a threshold key reads is measured at P3 against the config reader's handling of an unknown key.
@task_item n=13 part=needs
The core types BudgetWindowKind, Percent, BudgetWindow and BudgetReading leave with their tests; the closed EventKind and RefusalReason enums lose one member each; Reading<T> stays only if P3 measures a non-budget caller.
@task_item n=14 part=needs
The diagnostics line schema loses its budget fields and every process log line still conforms to it; no budget field remains in any log or diagnostics line.
@task_item n=15 part=needs
The test harness loses the Path 6 suite and its nextest entry, the status command's budget and budget_paused polled fields, boot's budget seeding through the statusline hook, and the profraw census script's mention; the harness gate's required suites follow.
@task_item n=16 part=needs
Tests that used budget-paused only as a sample refusal word (channel client, MCP handlers, machine contract, error bodies, fake-agent drift, bounded readers) keep their case with another closed refusal word; P3 names each.
@task_item n=17 part=needs
Every budget case leaves the remaining suites (list, wheel, instance state, machine contract, controls not disableable, Path 1 run start, chaos snapshot replay, MCP tools, MCP handlers, state events and replay); the controls-not-disableable sweep keeps every remaining control.
@task_item n=18 part=needs
A guard test's one-off remove-the-guard control runs through the bounded witness under a kill deadline, its case the test's name alone, holding only when that case ran and failed; no mutation run, local or dispatched.
@task_item n=19 part=needs
No live claude start, no live desktop step, no install or re-pin and no write under the live home; real claude stays out of CI, and a step needing any of them is a named operator step that waits.
@task_item n=20 part=needs
Every statement about how a tool, a crate or the claude CLI behaves is measured at P3 and cited, or marked unmeasurable.
@task_standing n=1
convention: every new or edited test function follows the test function naming rule
home: 2-test-strategy
@task_standing n=2
convention: every root integration test binary keeps its surface prefix, and the layer filtersets select it by that prefix
home: 2-test-strategy · test-strategy
@task_standing n=3
convention: every test creates its own fresh private home and seeds its own data, product state produced by viola, never hand-written
home: 7-test-data-fixtures · test-data-fixtures
@task_standing n=4
convention: every CLI and human-mode output a test asserts is pinned by a literal, contract tables written as literals
home: 5-integration-test-strategy · integration-test-strategy
@task_standing n=5
convention: every test wait keys on a logged event, never a sleep or a retry
home: 11-test-anti-patterns-never-do-these
@task_standing n=6
convention: every synthetic test input carries no real data and no secret, fixtures scrubbed
home: test-scope-summary · 1-test-scope-summary
@task_item n=1 part=not_needed
Dialog hook decision bodies, the answer verb, dialog hold expiry and the plan, permission and question bodies: the budget gate never covered answer and no dialog surface changes.
@task_item n=2 part=not_needed
Link and unlink methods, their records and implicit link on send: no link rule changes with the budget gate.
@task_item n=3 part=not_needed
Strict-modes refusals, the socket directory check, server identity verification and peer euid admission: no security check of the frame verbs is touched.
@task_item n=4 part=not_needed
PTY passthrough, resize, Ctrl-Z, the vt100 screen feed and the outer PTY driver: no terminal handling changes.
@task_item n=5 part=not_needed
Send confirmation by normalised match, its window, typed paste framing and the /clear and /compact post-conditions: the confirmation half of send is untouched.
@task_item n=6 part=not_needed
The claude child environment strip, the process-start claude-child record and pinned executable hashing: the child spawn is untouched.
@task_item n=7 part=not_needed
Harness session record, pid file, cleanup document and supervisor exit file contracts: the harness lifecycle is untouched beyond the status fields and Path 6.
@task_item n=8 part=not_needed
Secret-scan hit classes, CI artifact names and retention, workflow permissions and SHA pinning of actions: no workflow file changes.
@task_item n=9 part=not_needed
Heartbeat staleness classification and the instance liveness states: liveness reading is untouched.
@task_item n=10 part=not_needed
The pre-push detail, stage and reason codes and the local-live suite failure codes: no local gate script changes.
@task_item n=11 part=not_needed
Windows and macOS support, a database, outbound HTTP, a browser suite and a deploy stage: absences the removal does not reach.
