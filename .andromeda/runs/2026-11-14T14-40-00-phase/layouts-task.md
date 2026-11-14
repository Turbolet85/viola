@task_prose part=chunk
Removes the budget gate whole: viola reads no rate limits and refuses or pauses nothing on budget.
Leaves: the wrapper's gate and its start step, the budget-paused send refusal and its exit 11 (retired, never reused), the config thresholds, the budget state file and snapshot fields, the release --budget option, the list board's budget header and cells, the list payload's budget field, the budget hints, the rate-limits reading and ledger row, the Path 6 suite and every budget test case.
Keeps: the driver release guard, the plain release verb, the status-line pass-through, config.json with dialog_hold_s, and readers of old disk state holding budget lines and fields.
Re-measures at P3 how a config.json still carrying a threshold key reads, and that old budget-gate log lines and snapshot fields still read on replay and the board.
@task_file n=1 path=src/run/budget.rs
what: the wrapper's budget gate — thresholds read, decide, the release --budget lift, the budget evaluation span (deleted)
@task_file n=2 path=src/run/send/budget.rs
what: the send path's budget-paused check (deleted)
@task_file n=3 path=src/run/send.rs mark=edits
what: the send verb's refusal order in the wrapper — edits: the budget-paused rung
@task_file n=4 path=src/run/send/slot.rs mark=edits
what: the send slot of the wrapper — edits: the budget-paused rung
@task_file n=5 path=src/run/send/test_support.rs mark=edits
what: the send path's test support — edits: its budget-paused references
@task_file n=6 path=src/run/mod.rs mark=edits
what: the wrapper's run loop and start order — edits: the budget step of the start order, budget_paused and budget_override_until in first()
@task_file n=7 path=src/run/wheel.rs mark=edits
what: the wrapper's wheel handling — edits: its budget references
@task_file n=8 path=src/run/wait.rs mark=edits
what: the wrapper's wait handling — edits: its budget references
@task_file n=9 path=src/run/dialog.rs mark=edits
what: the wrapper's dialog handling — edits: its budget references
@task_file n=10 path=src/run/version_gate.rs mark=edits
what: the wrapper's version gate at start — edits: its budget references
@task_file n=11 path=src/cmd/run.rs mark=edits
what: the viola run verb's start — edits: its budget references
@task_file n=12 path=src/cmd/revive.rs mark=edits
what: the viola revive verb's start — edits: its budget references
@task_file n=13 path=crates/viola-state/src/budget.rs
what: the budget state file reader and writer — write_budget, read_budget, read_budget_stamped (deleted)
@task_file n=14 path=crates/viola-state/src/lib.rs mark=edits
what: the state crate's module list — edits: pub mod budget
@task_file n=15 path=crates/viola-state/src/snapshot.rs mark=edits
what: the instance snapshot — edits: the budget_paused and budget_override_until fields
@task_file n=16 path=crates/viola-state/src/replay.rs mark=edits
what: the event-log replay — edits: its recovery of budget_paused and budget_override_until
@task_file n=17 path=crates/viola-state/src/board.rs mark=edits
what: the board's state reading — edits: the board item's budget_paused and the board's budget reading
@task_file n=18 path=crates/viola-core/src/lib.rs mark=edits
what: the core vocabulary of event kinds, refusal reasons and value types — edits: EventKind::BudgetGate, RefusalReason::BudgetPaused, BudgetWindowKind, Percent, BudgetWindow, BudgetReading and their tests
@task_file n=19 path=src/cmd/release.rs mark=edits
what: the viola release verb — edits: the --budget option and its gate lifted / gate open human lines
@task_file n=20 path=src/cmd/send.rs mark=edits
what: the viola send verb's exit-code mapping — edits: the budget-paused refusal's exit 11
@task_file n=21 path=crates/viola-mcp/src/result.rs mark=edits
what: the MCP tool results — edits: the list payload's budget field
@task_file n=22 path=src/human/board/bay.rs mark=edits
what: the viola list board's BAY line — edits: the budget header
@task_file n=23 path=src/human/board/cells.rs mark=edits
what: the viola list board's cells — edits: the budget cells
@task_file n=24 path=src/human/board/rows.rs mark=edits
what: the viola list board's rows — edits: the budget cells
@task_file n=25 path=src/human/board.rs mark=edits
what: the viola list board — edits: its budget header and cells
@task_file n=26 path=src/human/hints.rs mark=edits
what: the refusal hints — edits: the budget hints
@task_file n=27 path=src/human/verbs.rs mark=edits
what: the human verb output lines — edits: the budget verb lines
@task_file n=28 path=src/human.rs mark=edits
what: the human output module — edits: its budget lines
@task_file n=29 path=src/cmd/mod.rs mark=edits
what: the CLI verb definitions and help — edits: the list verb's help text naming the budget
@task_file n=30 path=crates/viola-mcp/src/tools.rs mark=edits
what: the MCP tool definitions — edits: the list tool's description naming the budget
@task_file n=31 path=crates/viola-agent-claude/src/statusline.rs mark=edits
what: the status-line payload reading — edits: the rate_limits parse in reading
@task_file n=32 path=src/cmd/hook/statusline.rs mark=edits
what: the hook statusline arm — edits: the budget.json write and the budget_written log field
@task_file n=33 path=src/cmd/hook.rs mark=edits
what: the hook verb and its hook-decision log line — edits: the budget_written field
@task_file n=34 path=crates/viola-agent-claude/src/ledger.rs mark=edits
what: the relied ledger rows of the claude CLI — edits: the statusline-rate-limits row, whole_rate_limits, the STATUSLINE_RATE_LIMITS record fixture name, the relied count
@task_file n=35 path=src/bin/viola-fake-agent.rs mark=edits
what: the fake agent — edits: its rate-limits replay after the first Stop
@task_file n=36 path=tests/cli_budget.rs
what: the CLI budget gate tests (deleted)
@task_file n=37 path=crates/viola-e2e/tests/path6_budget_harness.rs
what: the Path 6 budget end-to-end suite (deleted)
@task_file n=38 path=schemas/diag-line.v1.json mark=edits
what: the diagnostic log line schema — edits: its budget fields
@task_file n=39 path=.config/nextest.toml mark=edits
what: the test runner configuration — edits: the Path 6 entry
@task_file n=40 path=tests/cli_list.rs mark=edits
what: the viola list tests — edits: its budget cases
@task_item n=1 part=needs
Exit code 11 is retired: the budget-paused refusal leaves, no other refusal takes 11, and every remaining CLI exit code keeps its number, in human and --json form.
@task_item n=2 part=needs
The send refusal order loses its budget-paused rung; an automated send is never refused on budget, over the CLI or the MCP send tool, and the remaining refusal words, hints and refusal results stay as they are.
@task_item n=3 part=needs
viola release loses its --budget option and its gate lifted / gate open output; the plain release (the wheel handed back) and its wheel verb output stay.
@task_item n=4 part=needs
The driver release guard stays: a release carrying a string from is refused release-from-driver, another type of from is refused with no data; viola release stays a human verb that no hint names.
@task_item n=5 part=needs
The viola list board loses its budget: the BAY line's budget cells, the closed budget gate and passed window states, and the budget cell order and columns; the other board states, widths and plain lines stay.
@task_item n=6 part=needs
The viola list --json document and the MCP list tool result lose their budget field; the MCP list tool description and the list verb's help text no longer name the budget.
@task_item n=7 part=needs
The budget hints and verb lines leave; every refusal cause that remains still carries its hint, last on stderr, naming the next verb.
@task_item n=8 part=needs
The budget-gate event kind and the budget-paused refusal reason leave the closed vocabulary; readers keep reading an old log holding budget-gate lines and an old snapshot holding budget fields, skipping and counting unknown kinds and fields.
@task_item n=9 part=needs
The two config.json threshold keys and their parse-rejected lines leave; config.json and dialog_hold_s stay; a file still carrying a threshold key reads as the config reader treats an unknown key, measured at P3.
@task_item n=10 part=needs
The hook statusline arm writes no budget file and its hook-decision line carries no budget_written field; its pass-through to the user's status-line command stays until the next entry.
@task_item n=11 part=needs
The statusline-rate-limits ledger row, its recorded fixtures and the fake agent's rate-limits replay leave; the relied row count drops from 24 to 23, and the --record fixture names and verify rows follow.
@task_item n=12 part=needs
The diagnostic log line schema and the recorded fixtures lose their budget and rate-limits shapes; the recorded MCP tools-list fixture no longer names the budget.
@task_item n=13 part=needs
A test that used budget-paused only as a sample refusal word keeps its case with another closed refusal word.
@task_item n=14 part=needs
No live claude start, no install or re-pin, no write under the viola home, no mutation run; a guard test's remove-the-guard control runs through the gate tool under a kill deadline and holds when the named case runs and fails.
@task_standing n=1
convention: human output is a contract — each changed or new test of a CLI verb's output asserts its exact lines and words, on the right stream
home: surface-cli
@task_standing n=2
convention: each --json document a test pins is one document with its typed fields, its refusal carrying the closed word and its exit code
home: surface-cli
@task_item n=1 part=not_needed
The web surface: viola renders no page; the chunk adds and removes nothing there.
@task_item n=2 part=not_needed
viola link and unlink output: no budget word or field appears in it.
@task_item n=3 part=not_needed
viola plugin install output and refusals: the chunk does not reach the plugin install verb.
@task_item n=4 part=not_needed
viola last output and the last MCP tool: they carry no budget field.
@task_item n=5 part=not_needed
viola answer output and the answer MCP tool: the dialog answer path holds no budget step.
@task_item n=6 part=not_needed
The frame verb exit-21 hints and the instance-unreachable --json details: reachability causes, untouched by the gate's removal.
@task_item n=7 part=not_needed
viola revive --list rows, its revive refusals and their --json error codes: none is a budget refusal.
@task_item n=8 part=not_needed
The CLI token rendering rows (colours, roles, spacing, radius, motion): the chunk adds no styled element and changes no token.
@task_item n=9 part=not_needed
Deep links or URL coupling in the CLI: nothing of the chunk links anywhere.
@task_item n=10 part=not_needed
The readback mirror line and its layout: a send's issue and outcome lines are unchanged by the removed rung.
